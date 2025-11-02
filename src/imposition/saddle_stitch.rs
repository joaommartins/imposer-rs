use crate::imposition::grid::calculate_saddle_stitch_grid;

/// Calculate page ordering for saddle stitch booklets (n-up, n ≥ 2).
///
/// This implementation produces a traditional saddle-stitch nesting while adding
/// a couple of practical optimisations for blank-padding and small inputs.
///
/// High-level algorithm summary:
/// - Ensure an EVEN number of sides (front/back pairs). The input page count is
///   padded to the next multiple of `pages_per_sheet`, then the number of sides
///   (pages / `pages_per_sheet`) is rounded up to an even value.
/// - Compute `initial_adjusted = total_sides * pages_per_sheet` and the number of
///   blank pages required = `initial_adjusted - num_pages`.
/// - Remove any "complete blank pairs" (groups of 4 blank pages) from the
///   placement calculation. Each complete blank pair (4 pages) can be kept
///   together at the end of the output and does not change the internal
///   saddle-stitch nesting; we therefore reduce the working page count by
///   multiples of 4. The remaining (0–3) blanks are handled by the nesting so
///   they pair with the low-numbered pages as usual.
/// - Generate traditional outer-to-inner saddle-stitch pairs over the
///   adjusted page count: pair i (0-based) is `(adjusted - i, i+1)` when i is
///   even, and `(i+1, adjusted - i)` when i is odd. Pages greater than
///   `num_pages` are rendered as blanks (0).
/// - Place pairs into the n-up grid row-by-row. Pairs are consumed in sequence
///   and alternately assigned to front and back slots within each row, creating
///   the horizontal zigzag pattern (front left-to-right, back right-to-left).
/// - For layouts larger than 4-up (i.e. `pages_per_sheet > 4`, typically 8-up
///   and above), the back-side row ordering is reversed in certain grid shapes
///   to account for duplex flipping; for 4-up and smaller, the back rows are
///   placed without row reversal.
/// - If a sheet's front and back pages are identical (which can happen for
///   very small inputs), the back side is not emitted (deduplication). The
///   function guarantees that every non-zero page number appears exactly once.
///
/// Implementation notes and invariants:
/// - The function returns a Vec of sides; each side is a Vec<usize> with
///   exactly `pages_per_sheet` slots. Zero denotes a blank/padding slot.
/// - The algorithm avoids creating duplicate non-blank pages; any page > 0
///   appears at most once in the returned structure.
/// - The pairing strategy uses a reduced `adjusted_num_pages` after removing
///   complete blank pairs. This preserves traditional saddle-stitch nesting
///   while grouping full blank pairs at the end of the output.
///
/// Example (4-up, 5 pages): produces front `[0,1,0,3]` and back `[2,0,4,5]`.
#[must_use]
pub fn calculate_saddle_stitch_order(num_pages: usize, pages_per_sheet: usize) -> Vec<Vec<usize>> {
    if pages_per_sheet == 0 {
        return Vec::new();
    }

    // For saddle stitch, we must have an EVEN number of sides (front/back pairs)
    // Calculate minimum sides needed, then round up to even
    let min_sides = num_pages.div_ceil(pages_per_sheet);
    let total_sides = if min_sides.is_multiple_of(2) {
        min_sides
    } else {
        min_sides + 1
    };

    // Calculate blank optimisation: complete blank pairs can be removed from placement
    let initial_adjusted = total_sides * pages_per_sheet;
    let num_blanks = initial_adjusted - num_pages;
    let complete_blank_pairs = num_blanks / 4;

    // Adjusted pages after removing complete blank pairs
    let adjusted_num_pages = initial_adjusted - (complete_blank_pairs * 4);

    // Calculate grid dimensions
    let (grid_rows, grid_cols) = calculate_saddle_stitch_grid(pages_per_sheet);
    let pairs_per_row = grid_cols / 2;

    // Total number of pairs and sheets
    let total_pairs = adjusted_num_pages / 2;
    let total_sheets = total_sides / 2;

    // Create page pairs for saddle stitch nesting
    // The nesting pattern alternates: outer-left with outer-right, then inner-left with inner-right
    // Pair 0 (front): (last, first) = (n, 1)
    // Pair 1 (back): (second, second-last) = (2, n-1)
    // Pair 2: (third-last, third) = (n-2, 3)
    // Pair 3: (fourth, fourth-last) = (4, n-3)
    //
    // Note: complete blank pairs have already been removed from adjusted_num_pages
    let mut page_pairs = Vec::new();

    // Generate pairs using traditional saddle-stitch nesting based on adjusted_num_pages
    for i in 0..total_pairs {
        let (left_page, right_page) = if i % 2 == 0 {
            // Even pairs: outer pair (high, low)
            let left = adjusted_num_pages - i;
            let right = i + 1;
            (left, right)
        } else {
            // Odd pairs: inner pair (low, high)
            let left = i + 1;
            let right = adjusted_num_pages - i;
            (left, right)
        };

        // Convert to actual page numbers (0 for blank pages beyond num_pages)
        let left = if left_page <= num_pages { left_page } else { 0 };
        let right = if right_page <= num_pages {
            right_page
        } else {
            0
        };

        page_pairs.push((left, right));
    }

    let mut ordering = Vec::new();
    let mut pair_idx = 0;

    for _sheet_idx in 0..total_sheets {
        let mut front_pages = vec![0usize; pages_per_sheet];
        let mut back_pages = vec![0usize; pages_per_sheet];

        // Place pairs row by row, alternating between front and back
        // Front fills rows top-to-bottom, back fills rows bottom-to-top
        // We need to place pairs_per_row * 2 pairs per row (half on front, half on back)
        for row in 0..grid_rows {
            let mut front_pairs_in_row = 0;
            let mut back_pairs_in_row = 0;

            // Process pairs_per_row * 2 pairs (alternating front/back)
            for pair_position in 0..(pairs_per_row * 2) {
                if pair_idx >= page_pairs.len() {
                    break;
                }

                let (left, right) = page_pairs[pair_idx];
                pair_idx += 1;

                // Alternate between front and back
                if pair_position % 2 == 0 {
                    // Even positions (0, 2, 4, ...) go on front, left-to-right
                    let front_base = row * grid_cols + front_pairs_in_row * 2;
                    front_pages[front_base] = left;
                    front_pages[front_base + 1] = right;
                    front_pairs_in_row += 1;
                } else {
                    // Odd positions (1, 3, 5, ...) go on back
                    // Within each row: right-to-left
                    // For 8-up and higher (n > 4), reverse row order on back
                    let should_reverse_rows = pages_per_sheet > 4
                        && pairs_per_row == 1
                        && page_pairs.len() >= pairs_per_row * 2 * grid_rows;
                    let back_row = if should_reverse_rows {
                        grid_rows - 1 - row
                    } else {
                        row
                    };
                    let back_base =
                        back_row * grid_cols + (pairs_per_row - 1 - back_pairs_in_row) * 2;
                    back_pages[back_base] = left;
                    back_pages[back_base + 1] = right;
                    back_pairs_in_row += 1;
                }
            }
        }

        ordering.push(front_pages.clone());

        // Don't duplicate if front and back are identical (happens with small inputs)
        if back_pages != front_pages {
            ordering.push(back_pages);
        }
    }

    ordering
}

#[cfg(test)]
#[path = "./test_saddle_stitch.rs"]
mod test_saddle_stitch;
