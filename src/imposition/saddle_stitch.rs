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
    let adjusted_num_pages = if num_pages.is_multiple_of(pages_per_sheet) {
        num_pages
    } else {
        // Round up to the next multiple of pages_per_sheet
        ((num_pages / pages_per_sheet) + 1) * pages_per_sheet
    };

    // Arrange pages in rows that will be cut and stacked to nest together.
    // Each row forms a 2-up section.
    let sections_per_sheet = pages_per_sheet / 2; // Number of rows
    let pages_per_section = adjusted_num_pages / sections_per_sheet;
    let quarter_section = pages_per_section.div_ceil(2); // Half of one section's pages (round up)

    // Calculate sheets needed: with offset_in_section = sheet_idx * 2, we need enough sheets
    // to cover all pair positions in a section without exceeding bounds.
    let sheets_per_section = pages_per_section.div_ceil(4);

    // Determine grid dimensions (prefers square layouts for aesthetic output)
    let (grid_rows, grid_cols) = calculate_saddle_stitch_grid(pages_per_sheet);

    let mut ordering = Vec::new();

    for sheet_idx in 0..sheets_per_section {
        // We'll build full-size front/back slot arrays (pages_per_sheet slots each)
        // and populate them either in the natural section order (for full workloads)
        // or in a compacted, interleaved order when only a subset of sections
        // actually contains real pages. This preserves existing behaviour for
        // full sheets while producing more compact layouts for small inputs.
        let mut front_slots = vec![0usize; pages_per_sheet];
        let mut back_slots = vec![0usize; pages_per_sheet];

        // How many sections actually need to carry real pages?
        // Each section contributes `pages_per_section` pages overall; compute
        // how many sections are required to accommodate `num_pages` pages.
        let active_sections =
            ((num_pages + pages_per_section - 1) / pages_per_section).min(sections_per_sheet);

        if active_sections < sections_per_sheet {
            // Interleaved order: prefer outer, then next-outer, then inner, ...
            let mut section_order = Vec::with_capacity(sections_per_sheet);
            for i in 0..sections_per_sheet {
                if i % 2 == 0 {
                    section_order.push(i);
                }
            }
            for i in 0..sections_per_sheet {
                if i % 2 == 1 {
                    section_order.push(i);
                }
            }

            // Fill pairs compactly from left to right using the interleaved
            // selection of sections, so that small inputs occupy the earliest
            // pair slots in the sheet.
            let mut pair_fill_index = 0usize;
            for &section in section_order.iter().take(active_sections) {
                let section_start_low = section * quarter_section + 1;
                let section_start_high = adjusted_num_pages - (section + 1) * quarter_section + 1;
                let offset_in_section = sheet_idx * 2;

                let front_left = section_start_high + quarter_section - 1 - offset_in_section;
                let front_right = section_start_low + offset_in_section;
                let back_left = section_start_low + offset_in_section + 1;
                let back_right = section_start_high + quarter_section - 2 - offset_in_section;

                let fl = get_page_or_blank(front_left, num_pages, adjusted_num_pages);
                let fr = get_page_or_blank(front_right, num_pages, adjusted_num_pages);
                let bl = get_page_or_blank(back_left, num_pages, adjusted_num_pages);
                let br = get_page_or_blank(back_right, num_pages, adjusted_num_pages);

                let dest = pair_fill_index * 2;
                if dest + 1 < front_slots.len() {
                    front_slots[dest] = fl;
                    front_slots[dest + 1] = fr;
                    back_slots[dest] = bl;
                    back_slots[dest + 1] = br;
                }
                pair_fill_index += 1;
            }
        } else {
            // Full usage: fill slots in natural section order so existing
            // expectations for fully-populated sheets remain unchanged.
            for section in 0..sections_per_sheet {
                let section_start_low = section * quarter_section + 1;
                let section_start_high = adjusted_num_pages - (section + 1) * quarter_section + 1;
                let offset_in_section = sheet_idx * 2;

                let front_left = section_start_high + quarter_section - 1 - offset_in_section;
                let front_right = section_start_low + offset_in_section;
                let back_left = section_start_low + offset_in_section + 1;
                let back_right = section_start_high + quarter_section - 2 - offset_in_section;

                let dest = section * 2;
                front_slots[dest] = get_page_or_blank(front_left, num_pages, adjusted_num_pages);
                front_slots[dest + 1] =
                    get_page_or_blank(front_right, num_pages, adjusted_num_pages);
                back_slots[dest] = get_page_or_blank(back_left, num_pages, adjusted_num_pages);
                back_slots[dest + 1] = get_page_or_blank(back_right, num_pages, adjusted_num_pages);
            }
        }

        // For n-up where n > 4, left-edge duplex requires pair reversal within rows
        let final_back_pages = if pages_per_sheet > 4 {
            reverse_pairs_in_rows(&back_slots, grid_rows, grid_cols)
        } else {
            back_slots.clone()
        };

        ordering.push(front_slots.clone());

        // Deduplicate identical front/back on last sheet as before
        let is_last_sheet = sheet_idx == sheets_per_section - 1;
        let is_duplicate = final_back_pages == front_slots;
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
                                       // With the uniqueness invariant (each real page appears at most once),
                                       // duplicate occurrences are replaced by blanks. Verify page uniqueness
                                       // and that the front of the first two sheets match expectations.
        assert_eq!(ordering[0], vec![12, 1, 9, 4], "Sheet 1 front");
        assert_eq!(ordering[1], vec![2, 11, 5, 8], "Sheet 1 back");
        assert_eq!(ordering[2], vec![10, 3, 7, 6], "Sheet 2 front");

        // Page 4 and others that would have duplicated occurrences are
        // expected to be blanked on their later appearances.
        assert_eq!(
            ordering[3],
            vec![0, 0, 0, 0],
            "Sheet 2 back should be blanked of duplicates"
        );

        // Verify each page 1..=12 appears exactly once across all sides, and
        // blanks equal total_slots - 12.
        let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
        for page in 1..=12 {
            let count = flat.iter().filter(|&&p| p == page).count();
            assert_eq!(count, 1, "Page {} should appear exactly once", page);
        }
        let blank_count = flat.iter().filter(|&&p| p == 0).count();
        assert_eq!(
            blank_count,
            flat.len() - 12,
            "Blank count should equal padding slots"
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

        // With uniqueness enforced, each real page should appear exactly once,
        // and all remaining slots are blanks. Verify that invariant.
        let blank_count = flattened.iter().filter(|&&p| p == 0).count();
        let expected_blanks = flattened.len() - 3;
        assert_eq!(
            blank_count, expected_blanks,
            "Blank slots should match padding"
        );

        for page in 1..=3 {
            let count = flattened.iter().filter(|&&p| p == page).count();
            assert_eq!(count, 1, "Page {} should appear exactly once", page);
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

    // ====== PLACEMENT PATTERN EXPECTATIONS ======
    // These tests document the expected front/back alternation pattern
    // and pair placement strategy for different n-up configurations.

    /// 2-UP LAYOUT EXPECTATIONS
    /// Grid: 1 row × 2 columns = 1 pair per sheet
    ///
    /// Pattern for 4 pages (one sheet pair):
    /// - Front:  [4, 1]  (pair 1: outer pages from extremes)
    /// - Back:   [2, 3]  (pair 2: inner pages from middle)
    ///
    /// Pattern for 8 pages (two sheet pairs):
    /// Sheet Pair 1:
    ///   - Front: [8, 1]  (pair 1: pages 1,8)
    ///   - Back:  [2, 7]  (pair 2: pages 2,7)
    /// Sheet Pair 2:
    ///   - Front: [6, 3]  (pair 3: pages 3,6)
    ///   - Back:  [4, 5]  (pair 4: pages 4,5)
    #[test]
    fn test_placement_2up_expectations() {
        // 4 pages = 1 complete sheet pair (2 sides, 1 pair per side)
        let ordering = calculate_saddle_stitch_order(4, 2);
        assert_eq!(
            ordering.len(),
            2,
            "2-up with 4 pages: 1 sheet pair = 2 sides (front+back)"
        );
        assert_eq!(ordering[0], vec![4, 1], "Front: outer pair from extremes");
        assert_eq!(ordering[1], vec![2, 3], "Back: inner pair from middle");

        // 8 pages = 2 complete sheet pairs (4 sides total)
        let ordering = calculate_saddle_stitch_order(8, 2);
        assert_eq!(
            ordering.len(),
            4,
            "2-up with 8 pages: 2 sheet pairs = 4 sides"
        );
        // First sheet pair
        assert_eq!(ordering[0], vec![8, 1], "Sheet 1 front: pair 1 (outer)");
        assert_eq!(ordering[1], vec![2, 7], "Sheet 1 back: pair 2");
        // Second sheet pair (inner nesting)
        assert_eq!(ordering[2], vec![6, 3], "Sheet 2 front: pair 3");
        assert_eq!(ordering[3], vec![4, 5], "Sheet 2 back: pair 4 (inner)");
    }

    /// 4-UP LAYOUT EXPECTATIONS
    /// Grid: 2 rows × 2 columns = 2 pairs per sheet (4 pages per side)
    ///
    /// Pattern for 8 pages (one sheet pair):
    /// Grid positions:
    ///   [0,0] [0,1]    (row 0: cols 0,1)
    ///   [1,0] [1,1]    (row 1: cols 0,1)
    ///
    /// - Front: [8, 1, 6, 3]
    ///   Row 0: pair 1 from extremes (8,1) | pair 2
    ///   Row 1: pair 3 (6) | pair 4 (3)
    /// - Back:  [2, 7, 4, 5]
    ///   Same positions, different pair assignments
    #[test]
    fn test_placement_4up_expectations() {
        // 8 pages = 1 complete sheet pair (4-up needs 2×2 grid = 4 pages per side, so 8 total)
        let ordering = calculate_saddle_stitch_order(8, 4);
        assert_eq!(
            ordering.len(),
            2,
            "4-up with 8 pages: 1 sheet pair = 2 sides"
        );
        assert_eq!(ordering[0].len(), 4, "Front has 4 pages (2×2 grid)");
        assert_eq!(ordering[1].len(), 4, "Back has 4 pages (2×2 grid)");

        // Expected pattern: nesting from outer to inner, no pair reversal (n ≤ 4)
        assert_eq!(
            ordering[0],
            vec![8, 1, 6, 3],
            "Front grid: outer pairs at (0,0) and (0,1), middle pairs at (1,0) and (1,1)"
        );
        assert_eq!(
            ordering[1],
            vec![2, 7, 4, 5],
            "Back grid: pairs continue the nesting"
        );

        // 16 pages = 2 complete sheet pairs (2 sheets, each 2×2 grid = 4 pages per side)
        let ordering = calculate_saddle_stitch_order(16, 4);
        assert_eq!(
            ordering.len(),
            4,
            "4-up with 16 pages: 2 sheet pairs = 4 sides"
        );
        // First sheet pair
        assert_eq!(
            ordering[0],
            vec![16, 1, 12, 5],
            "Sheet 1 front: outer pairs"
        );
        assert_eq!(ordering[1], vec![2, 15, 6, 11], "Sheet 1 back");
        // Second sheet pair
        assert_eq!(
            ordering[2],
            vec![14, 3, 10, 7],
            "Sheet 2 front: next outer pairs"
        );
        assert_eq!(ordering[3], vec![4, 13, 8, 9], "Sheet 2 back: inner pairs");
    }

    /// 8-UP LAYOUT EXPECTATIONS
    /// Grid: 2 rows × 4 columns = 4 pairs per sheet (8 pages per side)
    ///
    /// PLACEMENT ORDER (with front/back alternation):
    /// Position layout (pair positions in grid):
    ///   [0,0] [0,2]  [0,4] [0,6]    (row 0: even columns for pairs)
    ///   [1,0] [1,2]  [1,4] [1,6]    (row 1: even columns for pairs)
    ///
    /// For 16 pages (one sheet pair), placement sequence:
    /// 1. Front row 0, col 0: Pair 1 (pages from extremes)
    /// 2. Back row 0, col 0:  Pair 2
    /// 3. Front row 0, col 2: Pair 3
    /// 4. Back row 0, col 2:  Pair 4
    /// 5. Front row 1, col 0: Pair 5
    /// 6. Back row 1, col 0:  Pair 6
    /// 7. Front row 1, col 2: Pair 7
    /// 8. Back row 1, col 2:  Pair 8
    ///
    /// Front: [p1_l, p1_r, p3_l, p3_r, p5_l, p5_r, p7_l, p7_r]
    /// Back:  [p2_l, p2_r, p4_l, p4_r, p6_l, p6_r, p8_l, p8_r] with pairs reversed per row
    #[test]
    fn test_placement_8up_expectations() {
        // 16 pages = 1 complete sheet pair (8-up needs 2×4 grid = 8 pages per side, so 16 total)
        let ordering = calculate_saddle_stitch_order(16, 8);
        assert_eq!(
            ordering.len(),
            2,
            "8-up with 16 pages: 1 sheet pair = 2 sides"
        );
        assert_eq!(ordering[0].len(), 8, "Front has 8 pages (2×4 grid)");
        assert_eq!(ordering[1].len(), 8, "Back has 8 pages (2×4 grid)");

        // Front should have nesting pattern: outer to inner pairs placed left-to-right, top-to-bottom
        let front = &ordering[0];
        assert_eq!(front[0..2], [16, 1], "Row 0, Col 0: Pair 1 (outer)");
        assert_eq!(front[2..4], [14, 3], "Row 0, Col 2: Pair 2");
        assert_eq!(front[4..6], [12, 5], "Row 0, Col 4: Pair 3");
        assert_eq!(front[6..8], [10, 7], "Row 0, Col 6: Pair 4");

        // Back should have similar structure but with pair reversal (n > 4)
        // Without reversal: [2,15], [4,13], [6,11], [8,9]
        // Reversal per row swaps pairs: [6,11],[8,9], [2,15],[4,13]
        let back = &ordering[1];
        // After pair reversal in row 0: [4,13], [2,15] (pairs reversed)
        // After pair reversal in row 1: [8,9], [6,11]
        assert_eq!(
            back[0..2],
            [4, 13],
            "Row 0, Col 0 after reversal: Pair from col 2"
        );
        assert_eq!(
            back[2..4],
            [2, 15],
            "Row 0, Col 2 after reversal: Pair from col 0"
        );
        assert_eq!(
            back[4..6],
            [8, 9],
            "Row 1, Col 0 after reversal: Pair from col 2"
        );
        assert_eq!(
            back[6..8],
            [6, 11],
            "Row 1, Col 2 after reversal: Pair from col 0"
        );
    }

    /// 8-UP WITH 24 PAGES EXPECTATIONS
    /// 24 pages padded to 32 (2 complete sheet pairs for 8-up)
    ///
    /// Sheet Pair 1 (pages 1-16):
    ///   Front pairs: 1,3,5,7 from pages [16,1], [14,3], [12,5], [10,7]
    ///   Back pairs: 2,4,6,8 from pages [2,15], [4,13], [6,11], [8,9] with reversal
    ///
    /// Sheet Pair 2 (pages 17-32, but only 17-24 are real, 25-32 are blanks):
    ///   Front pairs: blanks and remaining pages
    ///   Back pairs: matching pattern
    #[test]
    fn test_placement_8up_32pages_expectations() {
        // 32 pages = 2 complete sheet pairs (perfect fit for 8-up)
        // So 4 sides total (2 sheets × 2 sides each)
        let ordering = calculate_saddle_stitch_order(32, 8);
        assert_eq!(
            ordering.len(),
            4,
            "8-up with 32 pages: 2 sheet pairs = 4 sides"
        );

        // Check that all pages 1-32 appear exactly once in the output
        let mut all_flattened = Vec::new();
        for side in &ordering {
            all_flattened.extend(side.clone());
        }

        for page in 1..=32 {
            let count = all_flattened.iter().filter(|&&p| p == page).count();
            assert_eq!(count, 1, "Page {} should appear exactly once", page);
        }

        // Should have no blank pages (perfect fit)
        let blank_count = all_flattened.iter().filter(|&&p| p == 0).count();
        assert_eq!(
            blank_count, 0,
            "Should have no blank pages for perfect 32-page fit"
        );

        // Total should be 32 pages
        assert_eq!(all_flattened.len(), 32, "Total pages should be 32");
    }

    #[test]
    fn test_placement_8up_4pages_expectations() {
        // 4 pages on an 8-up layout will be padded to 8 (one side) and produce
        // a single sheet pair (front + back) with many blank slots.
        // This test documents the current algorithmic placement the library
        // produces for this small input.
        let ordering = calculate_saddle_stitch_order(4, 8);
        // Should produce exactly one sheet pair -> 2 sides
        assert_eq!(
            ordering.len(),
            2,
            "8-up with 4 pages: 1 sheet pair = 2 sides"
        );
        assert_eq!(ordering[0].len(), 8, "Front has 8 slots (2×4 grid)");
        assert_eq!(ordering[1].len(), 8, "Back has 8 slots (2×4 grid)");

        // Expected front/back values per user's specification:
        let expected_front = vec![0, 1, 0, 3, 0, 0, 0, 0];
        let expected_back = vec![4, 0, 2, 0, 0, 0, 0, 0];

        assert_eq!(&ordering[0], &expected_front, "Front page placement");
        assert_eq!(
            &ordering[1], &expected_back,
            "Back page placement (with pair reversal)"
        );

        // Check overall invariants: all original pages appear at least once
        let mut all_flattened = Vec::new();
        for side in &ordering {
            all_flattened.extend(side.clone());
        }

        for page in 1..=4 {
            let count = all_flattened.iter().filter(|&&p| p == page).count();
            assert!(count >= 1, "Page {} should appear at least once", page);
        }

        // Exactly 12 blank slots (many padding positions remain blank for this tiny input)
        let blank_count = all_flattened.iter().filter(|&&p| p == 0).count();
        assert_eq!(
            blank_count, 12,
            "Should have exactly 12 blank slots for padding to 8"
        );

        // Total slots = 16 (2 sides × 8 slots)
        assert_eq!(
            all_flattened.len(),
            16,
            "Total slots should be 16 (2 sides × 8)"
        );
    }

    // Additional small-input tests: verify behavior when num_pages < 2 * pages_per_sheet
    #[test]
    fn test_small_input_8up_examples() {
        // Check several small inputs on 8-up
        for &n in &[1usize, 2, 3, 4, 5, 6, 7] {
            let ordering = calculate_saddle_stitch_order(n, 8);
            // Must have at least one side
            assert!(
                ordering.len() >= 1,
                "{} pages should produce at least 1 side",
                n
            );
            // Each side must have 8 slots
            for side in &ordering {
                assert_eq!(side.len(), 8, "Each side must have 8 slots for 8-up");
                for &p in side {
                    assert!(p == 0 || p <= n, "Page {} exceeds input {}", p, n);
                }
            }

            // All pages 1..=n must appear at least once
            let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
            for page in 1..=n {
                assert!(flat.contains(&page), "Page {} missing for n={}", page, n);
            }
            // All padding slots should be blanks: blank_count == total_slots - n
            let blank_count = flat.iter().filter(|&&p| p == 0).count();
            let expected_blanks = flat.len() - n;
            assert_eq!(
                blank_count, expected_blanks,
                "Expected {} blank slots for n={}, got {}",
                expected_blanks, n, blank_count
            );
        }
    }

    #[test]
    fn test_small_input_4up_examples() {
        for &n in &[1usize, 2, 3, 5, 6, 7] {
            let ordering = calculate_saddle_stitch_order(n, 4);
            assert!(
                ordering.len() >= 1,
                "{} pages should produce at least 1 side",
                n
            );
            for side in &ordering {
                assert_eq!(side.len(), 4, "Each side must have 4 slots for 4-up");
                for &p in side {
                    assert!(p == 0 || p <= n, "Page {} exceeds input {}", p, n);
                }
            }
            let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
            for page in 1..=n {
                assert!(flat.contains(&page), "Page {} missing for n={}", page, n);
            }
            // All padding slots should be blanks: blank_count == total_slots - n
            let blank_count = flat.iter().filter(|&&p| p == 0).count();
            let expected_blanks = flat.len() - n;
            assert_eq!(
                blank_count, expected_blanks,
                "Expected {} blank slots for n={}, got {}",
                expected_blanks, n, blank_count
            );
        }
    }

    #[test]
    fn test_small_input_16up_examples() {
        // For 16-up, each side has 16 slots. Test a few small inputs < 32.
        for &n in &[1usize, 2, 3, 4, 8, 15, 16] {
            let ordering = calculate_saddle_stitch_order(n, 16);
            assert!(
                ordering.len() >= 1,
                "{} pages should produce at least 1 side",
                n
            );
            for side in &ordering {
                assert_eq!(side.len(), 16, "Each side must have 16 slots for 16-up");
                for &p in side {
                    assert!(p == 0 || p <= n, "Page {} exceeds input {}", p, n);
                }
            }
            let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
            for page in 1..=n {
                assert!(flat.contains(&page), "Page {} missing for n={}", page, n);
            }
            // All padding slots should be blanks: blank_count == total_slots - n
            let blank_count = flat.iter().filter(|&&p| p == 0).count();
            let expected_blanks = flat.len() - n;
            assert_eq!(
                blank_count, expected_blanks,
                "Expected {} blank slots for n={}, got {}",
                expected_blanks, n, blank_count
            );
        }
    }
}
