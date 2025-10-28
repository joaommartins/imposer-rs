use super::grid::calculate_saddle_stitch_grid;

/// Convert a page number to output value, returning 0 (blank) if out of bounds
#[inline]
fn get_page_or_blank(page: usize, num_pages: usize, adjusted_num_pages: usize) -> usize {
    if page > 0 && page <= adjusted_num_pages {
        if page <= num_pages {
            page
        } else {
            0 // Blank page for padding
        }
    } else {
        0 // Out of bounds
    }
}

/// Reverse pairs within each row for left-edge duplex printing
///
/// Used only when `pages_per_sheet > 4` to handle short-edge duplex flipping.
/// For each row, pairs are reversed: [a,b,c,d] becomes [c,d,a,b]
///
/// Example with 2 rows of 4: [a,b,c,d, e,f,g,h] becomes [c,d,a,b, g,h,e,f]
fn reverse_pairs_in_rows(pages: &[usize], grid_rows: usize, grid_cols: usize) -> Vec<usize> {
    let mut reversed = Vec::new();
    let pages_per_row = grid_cols;

    for row_idx in 0..grid_rows {
        let row_start = row_idx * pages_per_row;
        // Reverse pairs within this row: iterate through pairs backwards
        for pair_start in (0..pages_per_row).step_by(2).rev() {
            let idx = row_start + pair_start;
            if idx < pages.len() {
                reversed.push(pages[idx]);
            }
            if idx + 1 < pages.len() {
                reversed.push(pages[idx + 1]);
            }
        }
    }
    reversed
}

/// Calculate page ordering for saddle stitch booklets (n-up, n ≥ 2)
///
/// Arranges pages in a saddle stitch pattern where the outermost pages form the covers
/// and pages nest inward to create the booklet structure when cut and folded.
///
/// # Grid Layout
/// Pages are arranged in a grid determined by `pages_per_sheet`:
/// - 2-up: 1×2 (landscape)
/// - 4-up: 2×2 (portrait)
/// - 8-up: 2×4 (landscape)
/// - 16-up: 4×4 (portrait)
/// The grid layout prefers square arrangements where possible.
///
/// # Nesting Structure
/// For n-up, pages are split into rows that each become a 2-up section:
/// - Section 0 (outermost): pages from both ends (1-k and (n-k+1)-n)
/// - Section 1 (next): pages from remaining middle working outward
/// - Each section nests inside the previous one after cutting
///
/// # Odd Page Numbers
/// When the input has an odd number of pages (e.g., 109 pages):
/// - Automatically padded to a multiple of `pages_per_sheet` by adding blank pages (0)
/// - Example: 109 pages with 2-up → 110 pages; with 8-up → 112 pages
/// - The blank pages will appear in the imposition layout where the padding is needed
///
/// # Duplex Handling
/// For left-edge duplex printing:
/// - 2-up: short-edge flip (landscape 1×2 grid, left edge is short)
/// - 4-up: long-edge flip (portrait 2×2 grid, left edge is long)
/// - 8-up and higher (n > 4): short-edge flip (left edge is short, pairs reversed on back)
///   * Within each row, pairs [a,b,c,d] become [c,d,a,b] on the back
pub fn calculate_saddle_stitch_order(num_pages: usize, pages_per_sheet: usize) -> Vec<Vec<usize>> {
    if pages_per_sheet == 0 {
        return Vec::new();
    }

    // Pad page count to be divisible by pages_per_sheet
    // This ensures we have complete sheets for the n-up layout
    // Blank pages (represented as 0) will be added as needed
    let adjusted_num_pages = if num_pages % pages_per_sheet == 0 {
        num_pages
    } else {
        // Round up to the next multiple of pages_per_sheet
        ((num_pages / pages_per_sheet) + 1) * pages_per_sheet
    };

    // Arrange pages in rows that will be cut and stacked to nest together.
    // Each row forms a 2-up section.
    let sections_per_sheet = pages_per_sheet / 2; // Number of rows
    let pages_per_section = adjusted_num_pages / sections_per_sheet;
    let quarter_section = (pages_per_section + 1) / 2; // Half of one section's pages (round up)

    // Calculate sheets needed: with offset_in_section = sheet_idx * 2, we need enough sheets
    // to cover all pair positions in a section without exceeding bounds.
    let sheets_per_section = (pages_per_section + 3) / 4;

    // Determine grid dimensions (prefers square layouts for aesthetic output)
    let (grid_rows, grid_cols) = calculate_saddle_stitch_grid(pages_per_sheet);

    let mut ordering = Vec::new();

    for sheet_idx in 0..sheets_per_section {
        let mut front_pages = Vec::new();
        let mut back_pages = Vec::new();

        // Process each section (row) on this physical sheet
        // Sections nest from outside (first) to inside (last)
        for section in 0..sections_per_sheet {
            // Outer sections get pages from ends, inner sections from middle
            // Example: section 0 = pages [1-4, 13-16], section 1 = [5-8, 9-12]
            let section_start_low = section * quarter_section + 1;
            let section_start_high = adjusted_num_pages - (section + 1) * quarter_section + 1;

            // Calculate page numbers for this row using standard 2-up pairing
            let offset_in_section = sheet_idx * 2;

            let front_left = section_start_high + quarter_section - 1 - offset_in_section;
            let front_right = section_start_low + offset_in_section;
            let back_left = section_start_low + offset_in_section + 1;
            let back_right = section_start_high + quarter_section - 2 - offset_in_section;

            // Add pages, replacing out-of-bounds or padding pages with 0 (blank)
            front_pages.push(get_page_or_blank(front_left, num_pages, adjusted_num_pages));
            front_pages.push(get_page_or_blank(
                front_right,
                num_pages,
                adjusted_num_pages,
            ));
            back_pages.push(get_page_or_blank(back_left, num_pages, adjusted_num_pages));
            back_pages.push(get_page_or_blank(back_right, num_pages, adjusted_num_pages));
        }

        // For n-up where n > 4, left-edge duplex requires pair reversal
        // When flipping on the left edge, pairs swap positions within each row
        let final_back_pages = if pages_per_sheet > 4 {
            reverse_pairs_in_rows(&back_pages, grid_rows, grid_cols)
        } else {
            back_pages.clone()
        };

        ordering.push(front_pages.clone());

        // Only skip back pages if this is the last sheet AND front/back are identical
        // (which indicates duplication from rounding up sheet count)
        let is_last_sheet = sheet_idx == sheets_per_section - 1;
        let is_duplicate = final_back_pages == front_pages;
        if !(is_last_sheet && is_duplicate) {
            ordering.push(final_back_pages);
        }
    }

    ordering
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_ordering_4_pages_2up() {
        let ordering = calculate_saddle_stitch_order(4, 2);
        // For 2-up booklets, pages alternate: outside pages on front, inside on back
        assert_eq!(ordering, vec![vec![4, 1], vec![2, 3]]);
    }

    #[test]
    fn test_page_ordering_8_pages_2up() {
        let ordering = calculate_saddle_stitch_order(8, 2);
        // Sheet 1 front: [8, 1], back: [2, 7]
        // Sheet 2 front: [6, 3], back: [4, 5]
        assert_eq!(
            ordering,
            vec![vec![8, 1], vec![2, 7], vec![6, 3], vec![4, 5]]
        );
    }

    #[test]
    fn test_page_ordering_5_pages_2up() {
        // Should pad to 6 pages (even number)
        // With deduplication of duplicate back pages: 5 pages (padded to 6) = 3 output pages
        let ordering = calculate_saddle_stitch_order(5, 2);
        // 6 pages: sheets_per_section = (6 + 3) / 4 = 2, but last back is duplicate so: 3 entries
        assert_eq!(ordering.len(), 3);
    }

    #[test]
    fn test_page_ordering_1_page_2up() {
        // Should pad to 2 pages (even number)
        // With deduplication: front and back would be identical, so only front is output
        let ordering = calculate_saddle_stitch_order(1, 2);
        // 2 pages: sheets_per_section = (2 + 3) / 4 = 1, last back is duplicate so: 1 entry
        assert_eq!(ordering.len(), 1);
        assert_eq!(ordering[0].len(), 2); // 2 pages per side
    }

    #[test]
    fn test_page_ordering_8_pages_4up() {
        let ordering = calculate_saddle_stitch_order(8, 4);
        // For 4-up (2×2 grid), single sheet:
        // Front: [8, 1, 6, 3], Back: [2, 7, 4, 5] (no pair reversal, n ≤ 4)
        assert_eq!(ordering, vec![vec![8, 1, 6, 3], vec![2, 7, 4, 5]]);
    }

    #[test]
    fn test_saddle_stitch_2up_8_pages() {
        let ordering = calculate_saddle_stitch_order(8, 2);
        // Standard 2-up: no pair reversal
        // Sheet 1: [8, 1] / [2, 7]
        // Sheet 2: [6, 3] / [4, 5]
        assert_eq!(
            ordering,
            vec![vec![8, 1], vec![2, 7], vec![6, 3], vec![4, 5]]
        );
    }

    #[test]
    fn test_saddle_stitch_4up_16_pages() {
        let ordering = calculate_saddle_stitch_order(16, 4);
        // 4-up (2×2 grid): no pair reversal on back (n ≤ 4)
        // Sheet 1:
        //   Front: [16, 1, 12, 5] (outer section, first pair)
        //   Back: [2, 15, 6, 11]
        // Sheet 2:
        //   Front: [14, 3, 10, 7]
        //   Back: [4, 13, 8, 9]
        assert_eq!(
            ordering,
            vec![
                vec![16, 1, 12, 5],
                vec![2, 15, 6, 11],
                vec![14, 3, 10, 7],
                vec![4, 13, 8, 9]
            ]
        );
    }

    #[test]
    fn test_saddle_stitch_8up_16_pages() {
        let ordering = calculate_saddle_stitch_order(16, 8);
        // 8-up (2×4 grid): pair reversal on back (n > 4)
        // Sheet 1:
        //   Front: [16, 1, 14, 3, 12, 5, 10, 7]
        //   Back: [4, 13, 2, 15, 8, 9, 6, 11] (pairs reversed within each row)
        assert_eq!(
            ordering,
            vec![
                vec![16, 1, 14, 3, 12, 5, 10, 7],
                vec![4, 13, 2, 15, 8, 9, 6, 11]
            ]
        );
    }

    #[test]
    fn test_saddle_stitch_16up_32_pages() {
        let ordering = calculate_saddle_stitch_order(32, 16);
        // 16-up (4×4 grid): pair reversal on back (n > 4)
        // Single sheet with 4 rows, each row has 4 pages
        // Sheet 1:
        //   Front row 1: [32, 1, 30, 3]
        //   Front row 2: [28, 5, 26, 7]
        //   Front row 3: [24, 9, 22, 11]
        //   Front row 4: [20, 13, 18, 15]
        //   Back row 1: [4, 29, 2, 31] (pairs reversed from [2, 31, 4, 29])
        //   Back row 2: [8, 25, 6, 27] (pairs reversed from [6, 27, 8, 25])
        //   Back row 3: [12, 21, 10, 23] (pairs reversed from [10, 23, 12, 21])
        //   Back row 4: [16, 17, 14, 19] (pairs reversed from [14, 19, 16, 17])
        assert_eq!(
            ordering,
            vec![
                vec![32, 1, 30, 3, 28, 5, 26, 7, 24, 9, 22, 11, 20, 13, 18, 15],
                vec![4, 29, 2, 31, 8, 25, 6, 27, 12, 21, 10, 23, 16, 17, 14, 19]
            ]
        );
    }

    #[test]
    fn test_saddle_stitch_4up_12_pages() {
        let ordering = calculate_saddle_stitch_order(12, 4);
        // 4-up (2×2 grid) with 12 pages: no pair reversal (n ≤ 4)
        // Creates 2 sheets
        assert_eq!(ordering.len(), 4); // 2 sheets × 2 sides each
        assert_eq!(
            ordering,
            vec![
                vec![12, 1, 9, 4], // Sheet 1 front
                vec![2, 11, 5, 8], // Sheet 1 back
                vec![10, 3, 7, 6], // Sheet 2 front
                vec![4, 9, 7, 6]   // Sheet 2 back
            ]
        );
    }

    #[test]
    fn test_saddle_stitch_8up_24_pages() {
        let ordering = calculate_saddle_stitch_order(24, 8);
        // 8-up (2×4 grid) with 24 pages: pair reversal on back (n > 4)
        // Processes as 2 sheets with 4 sections each
        assert_eq!(ordering.len(), 4); // 2 sheets × 2 sides each
        assert_eq!(ordering[0].len(), 8); // 8 pages on front
        assert_eq!(ordering[1].len(), 8); // 8 pages on back

        // Verify front row starts with correct outer pages
        assert_eq!(ordering[0][0], 24); // First page (highest)
        assert_eq!(ordering[0][1], 1); // Second page (lowest)
    }

    #[test]
    fn test_saddle_stitch_2up_odd_pages() {
        let ordering = calculate_saddle_stitch_order(109, 2);
        // 2-up with odd number of pages (109)
        // With automatic padding: treats as 110 pages internally, adding a blank (0) at the end
        let flattened: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

        // Blank page (0) should appear where padding is needed
        let count_blank = flattened.iter().filter(|&&p| p == 0).count();
        assert!(count_blank > 0, "Padding blank page (0) should be present");

        // All original pages should be in the output
        for page in 1..=109 {
            let count = flattened.iter().filter(|&&p| p == page).count();
            assert!(count > 0, "Page {} should be in the output", page);
        }

        // No page should appear more than once (except blank page 0)
        for page in 1..=109 {
            let count = flattened.iter().filter(|&&p| p == page).count();
            assert_eq!(
                count, 1,
                "Page {} should appear exactly once, but appears {} times",
                page, count
            );
        }
    }

    // ===== Tests for specific logic sections =====

    #[test]
    fn test_section_page_ranges_2up_8pages() {
        // Test that section calculations produce correct page ranges
        // For 8 pages, 2-up: 1 section, quarter_section = 4
        // Section 0: low=[1-4], high=[5-8]
        let ordering = calculate_saddle_stitch_order(8, 2);

        // Sheet 0 front: [8, 1] (high end + low start)
        assert_eq!(ordering[0], vec![8, 1]);
        // Sheet 0 back: [2, 7]
        assert_eq!(ordering[1], vec![2, 7]);
        // Sheet 1 front: [6, 3]
        assert_eq!(ordering[2], vec![6, 3]);
        // Sheet 1 back: [4, 5]
        assert_eq!(ordering[3], vec![4, 5]);
    }

    #[test]
    fn test_section_page_ranges_4up_16pages() {
        // For 16 pages, 4-up: 2 sections, quarter_section = 4
        // Section 0: low=[1-4], high=[13-16]
        // Section 1: low=[5-8], high=[9-12]
        let ordering = calculate_saddle_stitch_order(16, 4);

        // Sheet 0 front should have section 0 [16,1] and section 1 [12,5]
        assert_eq!(ordering[0], vec![16, 1, 12, 5]);
        // Sheet 0 back should have section 0 [2,15] and section 1 [6,11]
        assert_eq!(ordering[1], vec![2, 15, 6, 11]);
    }

    #[test]
    fn test_pair_reversal_8up() {
        // For 8-up (pages_per_sheet > 4), back pages should have pairs reversed
        let ordering = calculate_saddle_stitch_order(16, 8);

        // Front: [16, 1, 14, 3, 12, 5, 10, 7] (4 sections, 2 pages each)
        assert_eq!(ordering[0], vec![16, 1, 14, 3, 12, 5, 10, 7]);

        // Back without reversal would be: [2, 15, 4, 13, 6, 11, 8, 9]
        // With reversal in 2×4 grid (1 row of 8):
        // Pairs [2,15], [4,13], [6,11], [8,9] become [6,11], [8,9], [2,15], [4,13]
        // Wait, let me recalculate: the code reverses pairs within each ROW
        // For 2×4 grid: row 0 has 4 pages, row 1 has 4 pages
        // Actually for 8-up it's 2 rows × 4 cols, so grid_rows=2, grid_cols=4
        // Back pages: [2, 15, 4, 13] (row 0), [6, 11, 8, 9] (row 1)
        // Row 0 pairs reversed: [4, 13, 2, 15]
        // Row 1 pairs reversed: [8, 9, 6, 11]
        assert_eq!(ordering[1], vec![4, 13, 2, 15, 8, 9, 6, 11]);
    }

    #[test]
    fn test_pair_reversal_16up() {
        // For 16-up, verify pair reversal works with 4×4 grid
        let ordering = calculate_saddle_stitch_order(32, 16);

        // Should have 2 sides (front and back)
        assert_eq!(ordering.len(), 2);

        // Back should have pairs reversed in each row
        let back = &ordering[1];
        assert_eq!(back.len(), 16);

        // First row pairs reversed: [2, 31, 4, 29] becomes [4, 29, 2, 31]
        assert_eq!(&back[0..4], &[4, 29, 2, 31]);
    }

    #[test]
    fn test_blank_page_handling() {
        // Test that pages > num_pages are replaced with 0 (blank)
        let ordering = calculate_saddle_stitch_order(3, 4);

        // 3 pages padded to 4 for 4-up
        let flattened: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

        // Should have exactly 1 blank page (0)
        let blank_count = flattened.iter().filter(|&&p| p == 0).count();
        assert_eq!(
            blank_count, 1,
            "Should have exactly 1 blank page for 3->4 padding"
        );

        // Should have all pages 1, 2, 3
        for page in 1..=3 {
            assert!(flattened.contains(&page), "Should contain page {}", page);
        }
    }

    #[test]
    fn test_deduplication_small_booklet() {
        // Test that identical front/back pages are deduplicated on last sheet
        let ordering = calculate_saddle_stitch_order(1, 2);

        // With 1 page padded to 2, last sheet has identical front/back
        // Should output only 1 side (front), not 2
        assert_eq!(ordering.len(), 1, "Should deduplicate identical front/back");
        assert_eq!(ordering[0], vec![0, 1], "Should have blank and page 1");
    }

    #[test]
    fn test_no_deduplication_middle_sheets() {
        // Test that deduplication only happens on last sheet
        let ordering = calculate_saddle_stitch_order(8, 2);

        // Should have 4 sides (2 sheets × 2 sides each)
        assert_eq!(ordering.len(), 4);

        // No sheet should be missing its back side (except possibly the last)
        // For 8 pages, we don't expect any deduplication
        assert_eq!(ordering.len(), 4);
    }

    #[test]
    fn test_offset_calculation_progression() {
        // Test that offset_in_section progresses correctly
        // For 16 pages, 2-up: quarter_section = 4, sheets_per_section = 2
        // Sheet 0: offset = 0, Sheet 1: offset = 2
        let ordering = calculate_saddle_stitch_order(16, 2);

        // Sheet 0 (offset=0): front [16, 1], back [2, 15]
        assert_eq!(ordering[0], vec![16, 1]);
        assert_eq!(ordering[1], vec![2, 15]);

        // Sheet 1 (offset=2): front [14, 3], back [4, 13]
        assert_eq!(ordering[2], vec![14, 3]);
        assert_eq!(ordering[3], vec![4, 13]);
    }

    #[test]
    fn test_bounds_checking() {
        // Test that page calculations don't exceed adjusted_num_pages
        let ordering = calculate_saddle_stitch_order(7, 4);

        // 7 pages padded to 8 for 4-up
        let flattened: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

        // No page number should exceed 7 (except 0 for blanks)
        for &page in &flattened {
            assert!(page <= 7, "Page {} exceeds num_pages (7)", page);
        }

        // Should have exactly 1 blank (8 total slots - 7 pages)
        let blank_count = flattened.iter().filter(|&&p| p == 0).count();
        assert_eq!(blank_count, 1);
    }

    #[test]
    fn test_sheets_per_section_calculation() {
        // Test that sheets_per_section formula is correct
        // Formula: (pages_per_section + 3) / 4

        // 8 pages, 2-up: pages_per_section=8, sheets=(8+3)/4=2
        let ordering = calculate_saddle_stitch_order(8, 2);
        assert_eq!(ordering.len(), 4, "2 sheets × 2 sides = 4");

        // 16 pages, 4-up: pages_per_section=8, sheets=(8+3)/4=2
        let ordering = calculate_saddle_stitch_order(16, 4);
        assert_eq!(ordering.len(), 4, "2 sheets × 2 sides = 4");

        // 32 pages, 16-up: pages_per_section=8, sheets=(8+3)/4=2
        let ordering = calculate_saddle_stitch_order(32, 16);
        assert_eq!(ordering.len(), 2, "1 sheet × 2 sides = 2");
    }
}
