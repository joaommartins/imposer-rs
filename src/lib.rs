//! A library for creating n-up booklet impositions from PDF files
//!
//! This library takes a PDF file and rearranges its pages into booklet order,
//! placing multiple pages side-by-side on each output page. The output is designed
//! to be printed and folded into a booklet.
//!
//! By default, it creates 2-up booklets (2 pages per side), but can be configured
//! for any n-up layout (3-up, 4-up, etc.).
//!
//! # Example
//!
//! ```no_run
//! use std::fs;
//! use imposer::{generate_booklet, PageSize, BookletConfig};
//!
//! let input = fs::read("input.pdf").unwrap();
//! let output = generate_booklet(&input).unwrap();
//! fs::write("booklet.pdf", output).unwrap();
//! ```

mod config;
mod error;
/// Page ordering and booklet imposition algorithms
pub mod imposition;
mod pdf;
#[cfg(feature = "python")]
pub mod python;

// Re-export public API
pub use config::{BindingType, BookletConfig, PageSize, SaddleStitchPages};
pub use error::{BookletError, Result};

/// Generate a booklet from a PDF byte array
///
/// Takes a PDF as input and produces a new PDF with pages arranged in booklet order,
/// with multiple pages side-by-side on each output page. Uses A4 page size and 2-up
/// layout by default.
///
/// # Arguments
///
/// * `input_pdf` - The input PDF file as a byte slice
///
/// # Returns
///
/// A `Vec<u8>` containing the generated booklet PDF
///
/// # Errors
///
/// Returns `BookletError::ParseError` if the input PDF cannot be parsed.
/// Returns `BookletError::GenerationError` if the output PDF cannot be generated.
///
/// # Example
///
/// ```no_run
/// use std::fs;
///
/// let input = fs::read("input.pdf").unwrap();
/// let output = imposer::generate_booklet(&input).unwrap();
/// fs::write("booklet.pdf", output).unwrap();
/// ```
pub fn generate_booklet(input_pdf: &[u8]) -> Result<Vec<u8>> {
    generate_booklet_with_config(input_pdf, &BookletConfig::default())
}

/// Generate a booklet with custom configuration
///
/// Takes a PDF as input and produces a new PDF with pages arranged in booklet order,
/// with n pages side-by-side on each output page (configurable).
///
/// # Arguments
///
/// * `input_pdf` - The input PDF file as a byte slice
/// * `config` - Configuration options for the booklet
///
/// # Returns
///
/// A `Vec<u8>` containing the generated booklet PDF
///
/// # Errors
///
/// Returns `BookletError::ParseError` if the input PDF cannot be parsed.
/// Returns `BookletError::GenerationError` if the output PDF cannot be generated.
///
/// # Example
///
/// ```no_run
/// use std::fs;
/// use imposer::{BookletConfig, PageSize};
///
/// let input = fs::read("input.pdf").unwrap();
/// let config = BookletConfig::new(PageSize::A4).with_pages_per_sheet(3); // 3-up layout
/// let output = imposer::generate_booklet_with_config(&input, &config).unwrap();
/// fs::write("booklet.pdf", output).unwrap();
/// ```
pub fn generate_booklet_with_config(input_pdf: &[u8], config: &BookletConfig) -> Result<Vec<u8>> {
    pdf::generate_booklet_with_config(input_pdf, config)
}
