//! PDF booklet imposition tool
//!
//! This tool takes a PDF file and arranges its pages into a booklet format,
//! allowing for various configuration options such as page size, binding type,
//! and scaling behaviour.
//!
//! Note on saddle-stitch behavior:
//! - The CLI uses the library's saddle-stitch implementation which may pad the
//!   document with blank pages so the output is divisible into whole sheets.
//! - When possible, whole 4-page blank groups are kept together (entirely
//!   blank sheets are grouped at the end). Any remaining 0–3 padding pages are
//!   nested in the classical outer→inner saddle-stitch order and will therefore
//!   pair with low-numbered pages.
//! - Small layouts (4-up and smaller) avoid certain back-row reversals that are
//!   applied for larger n-up grid shapes (8-up and above). See README.md and
//!   `src/imposition/saddle_stitch.rs` for details and examples.
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use imposer::{BindingType, BookletConfig, PageSize, SaddleStitchPages};

/// Simple PDF booklet imposition tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[expect(clippy::struct_excessive_bools)]
struct Args {
    /// Input PDF path
    #[arg(short, long)]
    input: PathBuf,

    /// Output PDF path
    #[arg(short, long)]
    output: PathBuf,

    /// Output page size (a4, a3, a5, letter, legal, tabloid)
    #[arg(short = 'p', long, default_value = "a4")]
    page_size: String,

    /// Number of pages per sheet side (e.g., 2 for 2-up, 4 for 4-up)
    /// For saddle stitch binding, must be a power of 2: 2, 4, 8, 16, 32, 64, etc.
    #[arg(short = 'n', long, default_value = "2")]
    pages_per_sheet: usize,

    /// Do not scale source pages to fit output size
    #[arg(long)]
    no_scale_to_fit: bool,

    /// Do not preserve aspect ratio when scaling (stretches pages to fill space)
    #[arg(long)]
    no_preserve_aspect_ratio: bool,

    /// Draw fold and cut guide lines (for debugging)
    #[arg(short = 'g', long)]
    draw_guides: bool,

    /// Show page numbers instead of actual content (useful for visualizing page order)
    #[arg(long)]
    number_pages: bool,

    /// Use perfect binding instead of saddle stitch
    /// Perfect binding stacks sheets behind each other rather than nesting them
    #[arg(long)]
    perfect_binding: bool,

    /// Number of sheets per signature for perfect binding (default: 1)
    /// Only applies to perfect binding. Sheets within a signature are nested together.
    /// Use 1 for simple stacking, higher values for sewn signatures.
    #[arg(long, default_value = "1")]
    sheets_per_signature: usize,

    /// Number of signatures for perfect binding (optional)
    /// If specified, pages are evenly distributed across this many signatures.
    /// Takes precedence over --sheets-per-signature.
    /// Only applies to perfect binding.
    #[arg(long)]
    num_signatures: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut f = File::open(&args.input)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    let page_size =
        PageSize::from_str(&args.page_size).ok_or_else(|| format!("Invalid page size: {}", args.page_size))?;

    let binding_type = if args.perfect_binding {
        BindingType::PerfectBound
    } else {
        BindingType::SaddleStitch
    };

    // Validate pages_per_sheet for saddle stitch binding
    if binding_type == BindingType::SaddleStitch && SaddleStitchPages::from_usize(args.pages_per_sheet).is_none() {
        return Err(format!(
            "Invalid pages per sheet: {}. For saddle stitch binding, must be one of: {}",
            args.pages_per_sheet,
            SaddleStitchPages::valid_values_string()
        )
        .into());
    }

    let config = BookletConfig::new(page_size)
        .with_pages_per_sheet(args.pages_per_sheet)
        .with_scale_to_fit(!args.no_scale_to_fit)
        .with_preserve_aspect_ratio(!args.no_preserve_aspect_ratio)
        .with_draw_guides(args.draw_guides)
        .with_number_pages(args.number_pages)
        .with_binding_type(binding_type)
        .with_sheets_per_signature(args.sheets_per_signature)
        .with_num_signatures(args.num_signatures);

    let out = imposer::generate_booklet_with_config(&buf, &config)?;

    std::fs::write(&args.output, out)?;

    println!(
        "Wrote {}-up booklet to {} ({}){}",
        args.pages_per_sheet,
        args.output.display(),
        args.page_size,
        if args.draw_guides { " with guide lines" } else { "" }
    );
    Ok(())
}
