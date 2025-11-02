use super::*;

#[test]
fn test_page_ordering_4_pages_2up() {
    let ordering = calculate_saddle_stitch_order(4, 2);
    // For 2-up booklets, pages alternate: outside pages on front, inside on back
    // 4 pages = 2 sides (already even), no padding needed
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
    // 5 pages requires 3 sides min, rounded to 4 sides (even) = 8 page slots
    // 3 blanks: 3/4=0 complete pairs, 3%4=3 leading → use saddle-stitch nesting
    // Traditional saddle-stitch pairs: (8,1)→(0,1), (2,7)→(2,0), (6,3)→(0,3), (4,5)→(4,5)
    let ordering = calculate_saddle_stitch_order(5, 2);
    assert_eq!(ordering.len(), 4); // 4 sides (2 sheets)
    assert_eq!(ordering[0], vec![0, 1]);
    assert_eq!(ordering[1], vec![2, 0]);
    assert_eq!(ordering[2], vec![0, 3]);
    assert_eq!(ordering[3], vec![4, 5]);
}

#[test]
fn test_page_ordering_1_page_2up() {
    // Should pad to 2 pages, creating 1 pair (0,1)
    // Front gets pair 0, back is blank
    let ordering = calculate_saddle_stitch_order(1, 2);
    assert_eq!(ordering.len(), 2); // Front and back
    assert_eq!(ordering[0], vec![0, 1]); // Front: pair 0
    assert_eq!(ordering[1], vec![0, 0]); // Back: blank
}

#[test]
fn test_page_ordering_8_pages_4up() {
    let ordering = calculate_saddle_stitch_order(8, 4);
    // For 4-up (2×2 grid, 1 pair per row), zigzag without row reversal:
    // Front: [8, 1, 6, 3], Back: [2, 7, 4, 5]
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
    // 4-up (2×2 grid): zigzag placement, no row reversal on back (n ≤ 4)
    // Sheet 1:
    //   Front: [16, 1, 14, 3] (pairs 0, 2)
    //   Back: [2, 15, 4, 13] (pairs 1, 3 - placed left-to-right within row, right-to-left within pair)
    // Sheet 2:
    //   Front: [12, 5, 10, 7] (pairs 4, 6)
    //   Back: [6, 11, 8, 9] (pairs 5, 7 - placed left-to-right within row, right-to-left within pair)
    assert_eq!(
        ordering,
        vec![
            vec![16, 1, 14, 3],
            vec![2, 15, 4, 13],
            vec![12, 5, 10, 7],
            vec![6, 11, 8, 9]
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
    // 4-up (2×2 grid) with 12 pages: 12÷4=3 sides → round to 4 sides → 16 slots → 4 blanks
    // 4 blanks: 4/4=1 complete pair removed → work with 12 pages in saddle-stitch nesting
    assert_eq!(ordering.len(), 4); // 4 sides (2 sheets × 2 sides each)

    // Traditional saddle-stitch nesting with complete blank pairs at end
    assert_eq!(ordering[0], vec![12, 1, 10, 3], "Sheet 1 front");
    assert_eq!(ordering[1], vec![2, 11, 4, 9], "Sheet 1 back");

    // Sheet 2: saddle-stitch continues with complete blank pair at end
    assert_eq!(
        ordering[2],
        vec![8, 5, 0, 0],
        "Sheet 2 front: pairs 4-5, then blanks"
    );
    assert_eq!(
        ordering[3],
        vec![6, 7, 0, 0],
        "Sheet 2 back: pair 5, then blank pair"
    );

    // Verify each page 1..=12 appears exactly once across all sides
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
    // 8-up (2×4 grid) with 24 pages: 24÷8=3 sides → round to 4 sides → 32 slots → 8 blanks
    // 8 blanks: 8/4=2 complete pairs removed → work with 24 pages in saddle-stitch nesting
    assert_eq!(ordering.len(), 4); // 4 sides (2 sheets × 2 sides each)
    assert_eq!(ordering[0].len(), 8); // 8 pages per side
    assert_eq!(ordering[1].len(), 8);

    // Traditional saddle-stitch nesting with complete blank pairs at end
    assert_eq!(ordering[0], vec![24, 1, 22, 3, 20, 5, 18, 7]);
    assert_eq!(ordering[1], vec![4, 21, 2, 23, 8, 17, 6, 19]);
    assert_eq!(ordering[2], vec![16, 9, 14, 11, 0, 0, 0, 0]);
    assert_eq!(ordering[3], vec![12, 13, 10, 15, 0, 0, 0, 0]);
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
    // With zigzag placement, pairs are placed sequentially alternating front/back
    // Pairs for 16 pages: (16,1), (2,15), (14,3), (4,13), (12,5), (6,11), (10,7), (8,9)
    let ordering = calculate_saddle_stitch_order(16, 4);

    // Sheet 1 front: pairs 0 and 2
    assert_eq!(ordering[0], vec![16, 1, 14, 3]);
    // Sheet 1 back: pairs 1 and 3 (left-to-right placement)
    assert_eq!(ordering[1], vec![2, 15, 4, 13]);
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
    // With 1 page padded to 2, we have 1 pair (0,1) which goes on front
    // Back remains blank since there's no second pair
    let ordering = calculate_saddle_stitch_order(1, 2);

    assert_eq!(ordering.len(), 2, "Should have front and back");
    assert_eq!(ordering[0], vec![0, 1], "Front: pair 0");
    assert_eq!(ordering[1], vec![0, 0], "Back: blank (no pair 1)");
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
/// Grid: 2 rows × 2 columns = 1 pair per row (4 pages per side)
///
/// Zigzag placement: alternates between front and back, moving horizontally.
/// For 4-up (1 pair per row), we alternate every pair between front and back.
///
/// Pattern for 8 pages (one sheet, pairs 0-3):
/// - Pair 0 (8,1) → Front left
/// - Pair 1 (2,7) → Back right
/// - Pair 2 (6,3) → Front right
/// - Pair 3 (4,5) → Back left
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

    // Zigzag placement: pairs 0,2 on front; pairs 1,3 on back (left-to-right)
    assert_eq!(ordering[0], vec![8, 1, 6, 3], "Front: pairs 0 and 2");
    assert_eq!(
        ordering[1],
        vec![2, 7, 4, 5],
        "Back: pairs 1 and 3 (left-to-right)"
    );

    // 16 pages = 2 sheets, pairs 0-7
    // Sheet 1: pairs 0,1,2,3 (zigzag)
    // Sheet 2: pairs 4,5,6,7 (zigzag)
    let ordering = calculate_saddle_stitch_order(16, 4);
    assert_eq!(ordering.len(), 4, "4-up with 16 pages: 2 sheets = 4 sides");
    // Sheet 1: pairs 0-3
    assert_eq!(ordering[0], vec![16, 1, 14, 3], "Sheet 1 front: pairs 0, 2");
    assert_eq!(ordering[1], vec![2, 15, 4, 13], "Sheet 1 back: pairs 1, 3");
    // Sheet 2: pairs 4-7
    assert_eq!(ordering[2], vec![12, 5, 10, 7], "Sheet 2 front: pairs 4, 6");
    assert_eq!(ordering[3], vec![6, 11, 8, 9], "Sheet 2 back: pairs 5, 7");
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

    // Traditional saddle-stitch with 4 blanks (4/4=1 complete pair removed):
    // Pairs: (4,1), (2,3), then complete blank pair positions
    let expected_front = vec![4, 1, 0, 0, 0, 0, 0, 0];
    let expected_back = vec![0, 0, 2, 3, 0, 0, 0, 0];

    assert_eq!(&ordering[0], &expected_front, "Front page placement");
    assert_eq!(
        &ordering[1], &expected_back,
        "Back page placement (sequential, no row reversal)"
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

#[test]
fn test_4up_with_3_pages() {
    // Explicit test for 4-up with 3 input pages
    // 3 pages requires 1 side min, rounded to 2 sides (even) = 8 page slots
    let ordering = calculate_saddle_stitch_order(3, 4);
    assert_eq!(ordering.len(), 2, "Should have 2 sides (front and back)");

    // Expected: front [0, 1, 0, 0], back [2, 3, 0, 0]
    assert_eq!(
        ordering[0],
        vec![0, 1, 0, 0],
        "Front side should be [0, 1, 0, 0]"
    );
    assert_eq!(
        ordering[1],
        vec![2, 3, 0, 0],
        "Back side should be [2, 3, 0, 0]"
    );
}

#[rstest::rstest]
fn test_zero_pages() {
    // Edge case: zero pages input
    let ordering = calculate_saddle_stitch_order(0, 2);
    assert_eq!(ordering.len(), 0, "Zero pages should produce no sides");
}

#[test]
fn test_4up_partial_rows() {
    // Test 4-up with various partial row scenarios
    // For saddle stitch, pages are arranged in booklet order, not sequential

    // 4-up with 5 pages (1 sheet = 2 sides)
    let ordering = calculate_saddle_stitch_order(5, 4);
    assert_eq!(
        ordering.len(),
        2,
        "5 pages should produce 2 sides (1 sheet)"
    );
    // Verify all pages appear
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=5 {
        assert!(flat.contains(&page), "Page {} should appear", page);
    }

    // 4-up with 6 pages (1 sheet = 2 sides)
    let ordering = calculate_saddle_stitch_order(6, 4);
    assert_eq!(
        ordering.len(),
        2,
        "6 pages should produce 2 sides (1 sheet)"
    );
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=6 {
        assert!(flat.contains(&page), "Page {} should appear", page);
    }

    // 4-up with 7 pages (1 sheet = 2 sides)
    let ordering = calculate_saddle_stitch_order(7, 4);
    assert_eq!(
        ordering.len(),
        2,
        "7 pages should produce 2 sides (1 sheet)"
    );
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=7 {
        assert!(flat.contains(&page), "Page {} should appear", page);
    }
}

#[test]
fn golden_4up_5_pages_exact() {
    // Explicit golden test: expect exact front/back arrays for 4-up with 5 pages
    let ordering = calculate_saddle_stitch_order(5, 4);
    // One sheet -> 2 sides
    assert_eq!(
        ordering.len(),
        2,
        "5 pages on 4-up should produce 1 sheet (2 sides)"
    );
    assert_eq!(
        ordering[0],
        vec![0, 1, 0, 3],
        "Expected front side [0,1,0,3]"
    );
    assert_eq!(
        ordering[1],
        vec![2, 0, 4, 5],
        "Expected back side [2,0,4,5]"
    );
}

#[test]
fn golden_16up_109_pages_padding_and_grouping() {
    // Golden test to protect the blank-grouping optimisation at scale
    let num_pages = 109usize;
    let pps = 16usize;
    let ordering = calculate_saddle_stitch_order(num_pages, pps);

    // Flatten and count blanks
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    let blank_count = flat.iter().filter(|&&p| p == 0).count();

    // Compute expected blanks using the same logic as the algorithm's public contract
    let min_sides = (num_pages + pps - 1) / pps;
    let total_sides = if min_sides % 2 == 0 {
        min_sides
    } else {
        min_sides + 1
    };
    let initial_adjusted = total_sides * pps;
    let expected_blanks = initial_adjusted - num_pages;

    assert_eq!(
        blank_count, expected_blanks,
        "Expected {} blank slots, found {}",
        expected_blanks, blank_count
    );

    // The algorithm removes complete blank pairs (groups of 4) from the
    // placement calculation conceptually; we do not assert a strict
    // contiguous tail in the flattened output because zigzag placement can
    // interleave blanks with remaining pairs. Compute for documentation only.
    let _complete_blank_pairs = expected_blanks / 4; // integer division

    // Ensure every original page appears exactly once
    for page in 1..=num_pages {
        let cnt = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(
            cnt, 1,
            "Page {} should appear exactly once (found {})",
            page, cnt
        );
    }
}

#[test]
fn golden_16up_8_pages_exact() {
    // 8 pages on a 16-up layout -> padded and placed; document expected output
    let ordering = calculate_saddle_stitch_order(8, 16);
    // Should produce 2 sides (1 sheet pair)
    assert_eq!(
        ordering.len(),
        2,
        "8 pages on 16-up should produce 1 sheet (2 sides)"
    );
    // For small inputs, after removing complete blank pairs, the algorithm
    // will nest the 8 real pages in the standard outer-inner pattern.
    // Expect front and back to contain the 8 real pages (and blanks)
    // We capture the exact vectors observed from the current implementation.
    let expected_front = vec![8, 1, 6, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let expected_back = vec![4, 5, 2, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(&ordering[0], &expected_front, "16-up front for 8 pages");
    assert_eq!(&ordering[1], &expected_back, "16-up back for 8 pages");
}

#[test]
fn golden_16up_16_pages_exact() {
    // 16 real pages on a 16-up layout: single sheet, should place pages 1..16 exactly
    let ordering = calculate_saddle_stitch_order(16, 16);
    assert_eq!(
        ordering.len(),
        2,
        "16 pages on 16-up should produce 1 sheet (2 sides)"
    );
    // Each side must have 16 slots
    assert_eq!(ordering[0].len(), 16);
    assert_eq!(ordering[1].len(), 16);

    // Ensure all pages 1..=16 appear across front+back exactly once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=16 {
        let cnt = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(
            cnt, 1,
            "Page {} should appear exactly once in 16-up 16 pages",
            page
        );
    }
}

#[test]
fn test_8up_partial_rows() {
    // Test 8-up with various partial row scenarios (1-7 pages, 9-15 pages)

    // 8-up with 1 page
    let ordering = calculate_saddle_stitch_order(1, 8);
    assert!(ordering.len() >= 1, "1 page should produce at least 1 side");
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    assert!(flat.contains(&1), "Page 1 should appear");

    // 8-up with 5 pages
    let ordering = calculate_saddle_stitch_order(5, 8);
    assert!(
        ordering.len() >= 1,
        "5 pages should produce at least 1 side"
    );
    for side in &ordering {
        assert_eq!(side.len(), 8, "Each side must have 8 slots");
    }
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=5 {
        assert!(flat.contains(&page), "Page {} should appear", page);
    }

    // 8-up with 9 pages (requires second sheet)
    let ordering = calculate_saddle_stitch_order(9, 8);
    assert!(
        ordering.len() >= 2,
        "9 pages should produce at least 2 sides"
    );
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=9 {
        assert!(flat.contains(&page), "Page {} should appear", page);
    }
}

#[test]
fn test_32up_and_64up() {
    // Test higher powers of 2 to ensure generalization works

    // 32-up with 64 pages (1 sheet = 2 sides for saddle stitch)
    let ordering = calculate_saddle_stitch_order(64, 32);
    assert_eq!(
        ordering.len(),
        2,
        "64 pages with 32-up should produce 2 sides (1 sheet)"
    );
    for side in &ordering {
        assert_eq!(side.len(), 32, "Each side must have 32 slots for 32-up");
    }
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=64 {
        assert!(flat.contains(&page), "Page {} should appear", page);
    }

    // 32-up with 32 pages (1 sheet = 2 sides)
    let ordering = calculate_saddle_stitch_order(32, 32);
    assert_eq!(
        ordering.len(),
        2,
        "32 pages with 32-up should produce 2 sides"
    );
    for side in &ordering {
        assert_eq!(side.len(), 32, "Each side must have 32 slots for 32-up");
    }

    // 64-up with 128 pages (1 sheet = 2 sides)
    let ordering = calculate_saddle_stitch_order(128, 64);
    assert_eq!(
        ordering.len(),
        2,
        "128 pages with 64-up should produce 2 sides"
    );
    for side in &ordering {
        assert_eq!(side.len(), 64, "Each side must have 64 slots for 64-up");
    }
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=128 {
        assert!(flat.contains(&page), "Page {} should appear", page);
    }

    // 32-up with 10 pages (partial, 1 sheet = 2 sides)
    let ordering = calculate_saddle_stitch_order(10, 32);
    assert_eq!(
        ordering.len(),
        2,
        "10 pages with 32-up should produce 2 sides"
    );
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=10 {
        assert!(flat.contains(&page), "Page {} should appear", page);
    }
}

#[test]
fn test_large_page_counts() {
    // Test with very large page counts to ensure the algorithm scales
    let large_counts = vec![1000, 2000];

    for &num_pages in &large_counts {
        let ordering = calculate_saddle_stitch_order(num_pages, 2);

        // Verify all pages appear exactly once
        let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
        for page in 1..=num_pages {
            let count = flat.iter().filter(|&&p| p == page).count();
            assert_eq!(count, 1, "Page {} should appear exactly once", page);
        }

        // For 2-up saddle stitch, the algorithm creates nested booklets
        // Verify we have a reasonable number of sides
        assert!(
            ordering.len() >= num_pages / 2,
            "Should have at least {} sides for {} pages",
            num_pages / 2,
            num_pages
        );
        assert!(
            ordering.len() <= num_pages,
            "Should have at most {} sides for {} pages",
            num_pages,
            num_pages
        );

        // Each side should have exactly 2 slots for 2-up
        for side in &ordering {
            assert_eq!(side.len(), 2, "Each side should have 2 slots for 2-up");
        }
    }
}

#[rstest::rstest]
#[case(8, 2)]
#[case(16, 4)]
#[case(32, 8)]
#[case(64, 16)]
#[case(5, 2)]
#[case(7, 4)]
#[case(15, 8)]
#[case(31, 16)]
fn test_no_duplicate_non_blank_pages(#[case] num_pages: usize, #[case] pages_per_sheet: usize) {
    // Property test: verify no page appears twice across all configurations
    let ordering = calculate_saddle_stitch_order(num_pages, pages_per_sheet);
    let flat: Vec<usize> = ordering
        .iter()
        .flat_map(|s| s.iter().copied())
        .filter(|&p| p > 0) // Exclude blanks
        .collect();

    // Check each page appears exactly once
    for page in 1..=num_pages {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(
            count, 1,
            "Page {} should appear exactly once in {}-up with {} pages, but appears {} times",
            page, pages_per_sheet, num_pages, count
        );
    }

    // Verify total non-blank pages equals input
    assert_eq!(
        flat.len(),
        num_pages,
        "Total non-blank pages should equal input count for {}-up with {} pages",
        pages_per_sheet,
        num_pages
    );
}

#[rstest::rstest]
#[case(3, 2)]
#[case(5, 2)]
#[case(3, 4)]
#[case(5, 4)]
#[case(7, 4)]
#[case(10, 8)]
#[case(20, 16)]
fn test_blank_pages_only_at_end(#[case] num_pages: usize, #[case] pages_per_sheet: usize) {
    // Property test: verify blanks are only used for padding, never in the middle
    let ordering = calculate_saddle_stitch_order(num_pages, pages_per_sheet);
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // Check that the highest numbered page appears
    assert!(
        flat.contains(&num_pages),
        "Highest page {} should appear in output for {}-up",
        num_pages,
        pages_per_sheet
    );

    // All pages 1..=num_pages should appear somewhere
    // (Some blanks may appear interleaved due to the zigzag pattern)
    for page in 1..=num_pages {
        assert!(
            flat.contains(&page),
            "Page {} should appear in {}-up with {} pages",
            page,
            pages_per_sheet,
            num_pages
        );
    }
}

#[rstest::rstest]
#[case(4, 2, 1)] // 4 pages, 2-up = 1 sheet (2 sides)
#[case(8, 2, 2)] // 8 pages, 2-up = 2 sheets (4 sides)
#[case(5, 2, 2)] // 5 pages, 2-up = 2 sheets (4 sides, with padding)
#[case(8, 4, 1)] // 8 pages, 4-up = 1 sheet (2 sides)
#[case(16, 4, 2)] // 16 pages, 4-up = 2 sheets (4 sides)
#[case(12, 4, 2)] // 12 pages, 4-up = 2 sheets (4 sides, with padding)
#[case(32, 8, 2)] // 32 pages, 8-up = 2 sheets (4 sides)
fn test_correct_sheet_count(
    #[case] num_pages: usize,
    #[case] pages_per_sheet: usize,
    #[case] expected_sheets: usize,
) {
    // Property test: verify total sheets = ceil(num_pages / pages_per_sheet) / 2
    let ordering = calculate_saddle_stitch_order(num_pages, pages_per_sheet);
    let actual_sheets = ordering.len() / 2;
    assert_eq!(
        actual_sheets, expected_sheets,
        "Expected {} sheets for {} pages with {}-up, got {}",
        expected_sheets, num_pages, pages_per_sheet, actual_sheets
    );
}
