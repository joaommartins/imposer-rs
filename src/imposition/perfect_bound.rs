use super::grid::calculate_saddle_stitch_grid;
use super::saddle_stitch::calculate_saddle_stitch_order;

/// Calculate page ordering for perfect bound booklets with signature support
///
/// Perfect binding arranges pages into signatures, where each signature is bound
/// independently using saddle-stitch-like nesting. Unlike traditional saddle stitch
/// where all pages nest together, perfect binding groups pages into signatures
/// that are then stacked behind each other.
///
/// # Key Difference from Saddle Stitch
///
/// - **Saddle Stitch**: All pages nest together (outer sheet wraps all inner sheets)
/// - **Perfect Bound**: Pages are grouped into signatures, each signature nests
///   internally, then signatures are stacked
///
/// # Sheets Per Signature Semantics
///
/// The `sheets_per_signature` parameter refers to the number of **physical sheets after cutting**.
/// For different n-up values:
/// - 2-up: 1 printing sheet → 1 physical sheet (no cutting needed)
/// - 4-up: 1 printing sheet → 2 physical sheets (cut horizontally, stacking 2 × 2-up sheets)
/// - 8-up: 1 printing sheet → 4 physical sheets (cut horizontally and vertically)
///
/// When calculating pages per signature, the algorithm converts the requested physical sheets
/// to the number of printing sheets needed, then calculates pages accordingly.
/// This ensures that requesting "2 sheets/sig" with 4-up produces the same page grouping as
/// requesting "2 sheets/sig" with 2-up, since both require 1 printing sheet per signature.
///
/// # Algorithm
///
/// 1. Calculate how many physical sheets one printing sheet produces (based on grid layout)
/// 2. Determine the number of printing sheets needed to achieve the requested physical sheets
/// 3. Calculate pages per signature (`pages_per_sheet` × 2 × printing sheets per signature)
/// 4. For each signature, apply saddle stitch ordering to that signature's page range
/// 5. Stack the signatures sequentially (not nested as in pure saddle stitch)
///
/// # Examples
///
/// For 16 pages, 4-up, 2 sheets/signature:
/// - 4-up produces 2 physical sheets per printing sheet
/// - 2 physical sheets ÷ 2 = 1 printing sheet per signature = 8 pages/sig
/// - Signature 1 (pages 1-8): Apply saddle stitch → [8,1,6,3]/[2,7,4,5]
/// - Signature 2 (pages 9-16): Apply saddle stitch → [16,9,14,11]/[10,15,12,13]
///
/// For 8 pages, 4-up, 2 sheets/signature:
/// - 4-up produces 2 physical sheets per printing sheet
/// - 2 physical sheets ÷ 2 = 1 printing sheet per signature = 8 pages/sig
/// - Single signature with all 8 pages nested like traditional saddle stitch
///
/// This is the same page grouping as 8 pages, 2-up, 2 sheets/signature:
/// - 2-up produces 1 physical sheet per printing sheet
/// - 2 physical sheets ÷ 1 = 2 printing sheets per signature = 8 pages/sig
/// - Same signature structure but pages laid out differently (2-up vs 4-up grid)
#[must_use]
pub fn calculate_perfect_bound_order(
    num_pages: usize,
    pages_per_sheet: usize,
    sheets_per_signature: usize,
) -> Vec<Vec<usize>> {
    if pages_per_sheet < 2 || !pages_per_sheet.is_power_of_two() || sheets_per_signature < 1 {
        return Vec::new();
    }

    if num_pages == 0 {
        return Vec::new();
    }

    // Calculate how many physical sheets are produced from one printing sheet
    let (grid_rows, grid_cols) = calculate_saddle_stitch_grid(pages_per_sheet);
    let physical_sheets_per_printing_sheet = (grid_rows * grid_cols) / 2;

    // Calculate printing sheets per signature needed to achieve the requested physical sheets
    let printing_sheets_per_signature = sheets_per_signature.div_ceil(physical_sheets_per_printing_sheet);

    // Calculate pages per signature (pages_per_sheet × 2 for front/back × printing sheets in signature)
    let pages_per_signature = pages_per_sheet * 2 * printing_sheets_per_signature;

    // Calculate total signatures needed
    let total_signatures = num_pages.div_ceil(pages_per_signature);

    let mut result = Vec::new();

    // Process each signature independently
    for sig_idx in 0..total_signatures {
        let signature_start = sig_idx * pages_per_signature + 1;
        let signature_end = ((sig_idx + 1) * pages_per_signature).min(num_pages);
        let signature_page_count = signature_end - signature_start + 1;

        // Apply saddle stitch to this signature's page range
        let signature_ordering = calculate_saddle_stitch_order(signature_page_count, pages_per_sheet);

        // Remap page numbers from signature-local (1, 2, 3...) to global page numbers
        let remapped_ordering: Vec<Vec<usize>> = signature_ordering
            .into_iter()
            .map(|side| {
                side.into_iter()
                    .map(|page| {
                        if page == 0 {
                            0 // Keep blanks as 0
                        } else {
                            signature_start + page - 1 // Remap to global page number
                        }
                    })
                    .collect()
            })
            .collect();

        result.extend(remapped_ordering);
    }

    // For perfect binding, do NOT reorder blanks.
    // The saddle-stitch algorithm within each signature already places blanks
    // in the correct positions for proper folding. Just return as-is.
    result
}

#[cfg(test)]
#[path = "./test_perfect_bound.rs"]
mod test_perfect_bound;

/// Calculate page ordering for perfect bound booklets with a specified number of signatures.
///
/// This function distributes pages as evenly as possible across the requested number
/// of signatures. Each signature internally uses saddle-stitch ordering (as in the
/// standard perfect binding), but the total number of signatures is determined by
/// the input rather than by `sheets_per_signature`.
///
/// Pages are distributed to minimise variance: when there are extra pages (remainder),
/// they are spread throughout the signatures rather than concentrated at the beginning.
/// This results in more balanced signature sizes.
///
/// # Algorithm
///
/// 1. Calculate base pages per signature and remainder
/// 2. Distribute remainder pages evenly across signatures
/// 3. For each signature, apply saddle stitch ordering
/// 4. Stack signatures sequentially
/// 5. Move all blanks to the very end
///
/// # Example
///
/// For 10 pages across 3 signatures:
/// - `base_pages` = 10 / 3 = 3, remainder = 1
/// - Distribute remainder evenly: signatures get [3, 4, 3] pages
/// - Each signature is ordered with saddle-stitch internally
///
/// For 100 pages across 5 signatures:
/// - Each signature gets exactly 20 pages
#[must_use]
pub fn calculate_perfect_bound_order_with_signature_count(
    num_pages: usize,
    pages_per_sheet: usize,
    num_signatures: usize,
) -> Vec<Vec<usize>> {
    if pages_per_sheet < 2 || !pages_per_sheet.is_power_of_two() || num_signatures < 1 || num_pages == 0 {
        return Vec::new();
    }

    // Calculate pages per signature, distributing evenly
    let base_pages_per_sig = num_pages / num_signatures;
    let remainder = num_pages % num_signatures;

    // Create a vector of signature sizes, distributing remainder pages evenly
    // from the centre outward. For example:
    // - 10 pages, 3 signatures -> [3, 4, 3] (centre-weighted)
    // - 13 pages, 5 signatures -> [2, 3, 3, 3, 2] (centre-weighted)
    let mut sig_sizes = vec![base_pages_per_sig; num_signatures];

    // Distribute remainder pages by incrementing from centre outward
    // This creates a more balanced appearance with smaller signatures at edges
    for i in 0..remainder {
        // For i=0, place at centre; i=1 symmetric; i=2 next inner pair, etc.
        let centre = num_signatures / 2;
        let offset = i.div_ceil(2);
        let is_left = i % 2 == 0;

        let idx = if is_left { centre - offset } else { centre + offset };

        // Ensure index is within bounds
        if idx < num_signatures {
            sig_sizes[idx] += 1;
        }
    }

    let mut result = Vec::new();
    let mut current_page = 1usize;

    // Process each signature
    for sig_page_count in sig_sizes {
        if sig_page_count == 0 {
            continue;
        }

        let signature_start = current_page;
        let signature_end = current_page + sig_page_count - 1;

        // Apply saddle stitch to this signature's page range
        let signature_ordering = calculate_saddle_stitch_order(sig_page_count, pages_per_sheet);

        // Remap page numbers from signature-local (1, 2, 3...) to global page numbers
        let remapped_ordering: Vec<Vec<usize>> = signature_ordering
            .into_iter()
            .map(|side| {
                side.into_iter()
                    .map(|page| {
                        if page == 0 {
                            0 // Keep blanks as 0
                        } else if page <= sig_page_count {
                            signature_start + page - 1 // Remap to global page number
                        } else {
                            0 // Page beyond signature range becomes blank
                        }
                    })
                    .collect()
            })
            .collect();

        result.extend(remapped_ordering);
        current_page = signature_end + 1;
    }

    // Post-process: For perfect binding, do NOT reorder blanks.
    // The saddle-stitch algorithm already places blanks correctly. Just return as-is.
    result
}
