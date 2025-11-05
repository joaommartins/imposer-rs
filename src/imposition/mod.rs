use crate::config::BindingType;

// Module declarations
mod grid;
mod perfect_bound;
/// Saddle stitch imposition algorithm and helpers.
pub mod saddle_stitch;

// Re-export for internal use
use perfect_bound::{calculate_perfect_bound_order, calculate_perfect_bound_order_with_signature_count};
use saddle_stitch::calculate_saddle_stitch_order;

/// Calculate the page ordering for an n-up booklet
///
/// Returns a vector of vectors, where each inner vector represents one side of a sheet
/// and contains the page numbers to place on that side (left to right).
/// Page numbers are 1-indexed, with 0 representing a blank page.
///
/// # Arguments
///
/// * `num_pages` - The total number of pages in the source document
/// * `pages_per_sheet` - Number of pages to place on each side of a sheet (e.g., 2 for 2-up)
/// * `binding_type` - The binding type (saddle stitch or perfect bound)
/// * `sheets_per_signature` - Number of sheets per signature (only for perfect bound, ignored for saddle stitch)
///
/// # Example
///
/// For a 4-page document with 2-up saddle stitch layout:
/// - Sheet 1 front: [4, 1]
/// - Sheet 1 back: [2, 3]
///
/// For a 4-page document with 2-up perfect bound layout (1 sheet/signature):
/// - Sheet 1 front: [1, 4]
/// - Sheet 1 back: [2, 3]
#[must_use]
pub fn calculate_page_order(
    num_pages: usize,
    pages_per_sheet: usize,
    binding_type: BindingType,
    sheets_per_signature: usize,
) -> Vec<Vec<usize>> {
    match binding_type {
        BindingType::SaddleStitch => calculate_saddle_stitch_order(num_pages, pages_per_sheet),
        BindingType::PerfectBound => calculate_perfect_bound_order(num_pages, pages_per_sheet, sheets_per_signature),
    }
}

/// Calculate the page ordering for a perfect bound booklet with a specified number of signatures
///
/// This function distributes pages as evenly as possible across the requested number of signatures.
/// Each signature internally uses saddle-stitch ordering, but the total count is user-specified.
///
/// Returns a vector of vectors, where each inner vector represents one side of a sheet
/// and contains the page numbers to place on that side (left to right).
/// Page numbers are 1-indexed, with 0 representing a blank page.
///
/// # Arguments
///
/// * `num_pages` - The total number of pages in the source document
/// * `pages_per_sheet` - Number of pages to place on each side of a sheet (e.g., 2 for 2-up)
/// * `num_signatures` - The number of signatures to create (pages distributed evenly)
///
/// # Example
///
/// For a 100-page document with 4-up and 5 signatures:
/// - Each signature gets 20 pages
/// - Pages are arranged in signature order with saddle-stitch per signature
#[must_use]
pub fn calculate_page_order_with_signatures(
    num_pages: usize,
    pages_per_sheet: usize,
    num_signatures: usize,
) -> Vec<Vec<usize>> {
    calculate_perfect_bound_order_with_signature_count(num_pages, pages_per_sheet, num_signatures)
}

#[cfg(test)]
mod test_imposition;
