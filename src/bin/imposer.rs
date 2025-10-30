//! PDF booklet imposition tool
//!
//! This tool takes a PDF file and arranges its pages into a booklet format,
//! allowing for various configuration options such as page size, binding type,
//! and scaling behaviour.
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

    /// Use perfect binding instead of saddle stitch (sequential page order)
    /// **WARNING: This feature is work in progress and not yet ready for use!**
    #[arg(long)]
    perfect_binding: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut f = File::open(&args.input)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    let page_size = PageSize::from_str(&args.page_size)
        .ok_or_else(|| format!("Invalid page size: {}", args.page_size))?;

    let binding_type = if args.perfect_binding {
        BindingType::PerfectBound
    } else {
        BindingType::SaddleStitch
    };

    // Perfect binding is still work in progress - reject attempts to use it
    if binding_type == BindingType::PerfectBound {
        return Err(
            "Perfect binding is currently a work in progress and not ready for use. \
             Please use saddle stitch binding instead (the default)."
                .into(),
        );
    }

    // Validate pages_per_sheet for saddle stitch binding
    if binding_type == BindingType::SaddleStitch
        && SaddleStitchPages::from_usize(args.pages_per_sheet).is_none()
    {
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
        .with_binding_type(binding_type);

    let out = imposer::generate_booklet_with_config(&buf, &config)?;

    std::fs::write(&args.output, out)?;

    println!(
        "Wrote {}-up booklet to {} ({}){}",
        args.pages_per_sheet,
        args.output.display(),
        args.page_size,
        if args.draw_guides {
            " with guide lines"
        } else {
            ""
        }
    );
    Ok(())
}
