use crate::config::BindingType;

// Module declarations
mod grid;
mod perfect_bound;
mod saddle_stitch;

// Re-export for internal use
#[expect(deprecated)]
use perfect_bound::calculate_perfect_bound_order;
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
///
/// # Example
///
/// For a 4-page document with 2-up saddle stitch layout:
/// - Sheet 1 front: [4, 1]
/// - Sheet 1 back: [2, 3]
///
/// For a 4-page document with 2-up perfect bound layout:
/// - Sheet 1 front: [1, 2]
/// - Sheet 1 back: [3, 4]
pub fn calculate_page_order(
    num_pages: usize,
    pages_per_sheet: usize,
    binding_type: BindingType,
) -> Vec<Vec<usize>> {
    match binding_type {
        BindingType::SaddleStitch => calculate_saddle_stitch_order(num_pages, pages_per_sheet),
        #[expect(deprecated)]
        BindingType::PerfectBound => calculate_perfect_bound_order(num_pages, pages_per_sheet),
    }
}

#[cfg(test)]
mod tests;
