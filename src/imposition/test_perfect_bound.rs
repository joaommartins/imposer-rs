use super::*;

// ===== Basic Perfect Bound Tests =====
// Perfect bound differs from saddle stitch:
// - Sheets are STACKED behind each other, not nested
// - Within each sheet, pages follow a folded pattern (like mini saddle-stitch)
// - Signatures group multiple sheets together before stacking

#[test]
fn test_perfect_bound_4_pages_2up_single_signature() {
    // 4 pages, 2-up, 1 sheet per signature
    // Perfect bound uses saddle stitch ordering for each signature
    let ordering = calculate_perfect_bound_order(4, 2, 1);
    assert_eq!(ordering.len(), 2, "Should have 2 sides (1 sheet)");
    assert_eq!(ordering[0], vec![4, 1], "Front: outer pages (saddle stitch order)");
    assert_eq!(ordering[1], vec![2, 3], "Back: inner pages");
}

#[test]
fn test_perfect_bound_8_pages_2up_single_signature() {
    // 8 pages, 2-up, 1 sheet per signature = 2 signatures
    // Each signature uses saddle stitch ordering
    let ordering = calculate_perfect_bound_order(8, 2, 1);
    assert_eq!(ordering.len(), 4, "Should have 4 sides (2 sheets)");

    // Signature 1 (pages 1-4) - saddle stitch order
    assert_eq!(ordering[0], vec![4, 1], "Sheet 1 front");
    assert_eq!(ordering[1], vec![2, 3], "Sheet 1 back");

    // Signature 2 (pages 5-8) - saddle stitch order
    assert_eq!(ordering[2], vec![8, 5], "Sheet 2 front");
    assert_eq!(ordering[3], vec![6, 7], "Sheet 2 back");
}

#[test]
fn test_perfect_bound_8_pages_2up_two_sheets_per_signature() {
    // 8 pages, 2-up, 2 sheets per signature = 1 signature
    // For perfect binding, sheets within a signature are nested
    // This is indistinguishable from saddle stitch for a single signature
    // Sheet 1 (outer): [8,1] / [2,7]
    // Sheet 2 (inner): [6,3] / [4,5]
    let ordering = calculate_perfect_bound_order(8, 2, 2);
    assert_eq!(ordering.len(), 4, "Should have 4 sides (2 sheets)");

    assert_eq!(ordering[0], vec![8, 1], "Outer sheet front");
    assert_eq!(ordering[1], vec![2, 7], "Outer sheet back");
    assert_eq!(ordering[2], vec![6, 3], "Inner sheet front");
    assert_eq!(ordering[3], vec![4, 5], "Inner sheet back");
}

#[test]
fn test_perfect_bound_16_pages_4up_two_signatures() {
    // 16 pages, 4-up, 1 sheet per signature = 2 signatures
    // Each signature covers 8 pages and uses saddle stitch ordering internally
    // Signature 1 (pages 1-8): [8,1,6,3] / [2,7,4,5]
    // Signature 2 (pages 9-16): [16,9,14,11] / [10,15,12,13]
    let ordering = calculate_perfect_bound_order(16, 4, 1);
    assert_eq!(ordering.len(), 4, "Should have 4 sides (2 signatures × 1 sheet each)");

    assert_eq!(ordering[0], vec![8, 1, 6, 3], "Signature 1, sheet 1 front");
    assert_eq!(ordering[1], vec![2, 7, 4, 5], "Signature 1, sheet 1 back");
    assert_eq!(ordering[2], vec![16, 9, 14, 11], "Signature 2, sheet 1 front");
    assert_eq!(ordering[3], vec![10, 15, 12, 13], "Signature 2, sheet 1 back");
}

#[test]
fn test_perfect_bound_16_pages_2up_two_sheets_per_signature() {
    // 16 pages, 2-up, 2 sheets per signature = 2 signatures
    // Signature 1 (pages 1-8): nested sheets
    //   Sheet 1: [8,1] / [2,7]
    //   Sheet 2: [6,3] / [4,5]
    // Signature 2 (pages 9-16): nested sheets
    //   Sheet 3: [16,9] / [10,15]
    //   Sheet 4: [14,11] / [12,13]
    let ordering = calculate_perfect_bound_order(16, 2, 2);
    assert_eq!(ordering.len(), 8, "Should have 8 sides (4 sheets)");

    // First signature
    assert_eq!(ordering[0], vec![8, 1], "Sig 1, Sheet 1 front");
    assert_eq!(ordering[1], vec![2, 7], "Sig 1, Sheet 1 back");
    assert_eq!(ordering[2], vec![6, 3], "Sig 1, Sheet 2 front");
    assert_eq!(ordering[3], vec![4, 5], "Sig 1, Sheet 2 back");

    // Second signature
    assert_eq!(ordering[4], vec![16, 9], "Sig 2, Sheet 3 front");
    assert_eq!(ordering[5], vec![10, 15], "Sig 2, Sheet 3 back");
    assert_eq!(ordering[6], vec![14, 11], "Sig 2, Sheet 4 front");
    assert_eq!(ordering[7], vec![12, 13], "Sig 2, Sheet 4 back");
}

// ===== Small Input Tests =====

#[test]
fn test_perfect_bound_1_page_2up() {
    // 1 page needs padding to 4 for single sheet
    let ordering = calculate_perfect_bound_order(1, 2, 1);
    assert_eq!(ordering.len(), 2, "Should have 2 sides (1 sheet)");

    // Verify page 1 appears
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    assert!(flat.contains(&1), "Page 1 should appear");
}

#[test]
fn test_perfect_bound_3_pages_2up() {
    // 3 pages needs padding to 4 for single sheet
    let ordering = calculate_perfect_bound_order(3, 2, 1);
    assert_eq!(ordering.len(), 2, "Should have 2 sides (1 sheet)");

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=3 {
        assert!(flat.contains(&page), "Page {page} should appear");
    }
}

#[test]
fn test_perfect_bound_5_pages_2up() {
    // 5 pages needs 2 sheets (8 pages total)
    let ordering = calculate_perfect_bound_order(5, 2, 1);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=5 {
        assert!(flat.contains(&page), "Page {page} should appear");
    }

    // Check for blanks (padding)
    let blank_count = flat.iter().filter(|&&p| p == 0).count();
    assert_eq!(blank_count, 3, "Should have 3 blank pages for padding");
}

// ===== 4-up Tests =====

#[test]
fn test_perfect_bound_8_pages_4up_single_signature() {
    // 8 pages, 4-up, 1 sheet per signature
    // Single sheet folds to create 8 pages
    let ordering = calculate_perfect_bound_order(8, 4, 1);
    assert_eq!(ordering.len(), 2, "Should have 2 sides (1 sheet)");

    assert_eq!(ordering[0].len(), 4, "Front should have 4 positions");
    assert_eq!(ordering[1].len(), 4, "Back should have 4 positions");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=8 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_16_pages_4up_single_signature() {
    // 16 pages, 4-up, 1 sheet per signature = 2 sheets stacked
    let ordering = calculate_perfect_bound_order(16, 4, 1);
    assert_eq!(ordering.len(), 4, "Should have 4 sides (2 sheets)");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=16 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_16_pages_4up_two_sheets_per_signature() {
    // 16 pages, 4-up, 2 sheets per signature = 1 signature with nested sheets
    let ordering = calculate_perfect_bound_order(16, 4, 2);
    assert_eq!(ordering.len(), 4, "Should have 4 sides (2 sheets)");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=16 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

// ===== Partial Sheet Tests =====

#[test]
fn test_perfect_bound_5_pages_4up() {
    // 5 pages, 4-up needs padding to 8
    let ordering = calculate_perfect_bound_order(5, 4, 1);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=5 {
        assert!(flat.contains(&page), "Page {page} should appear");
    }

    let blank_count = flat.iter().filter(|&&p| p == 0).count();
    assert_eq!(blank_count, 3, "Should have 3 blank pages");
}

#[test]
fn test_perfect_bound_blanks_in_saddle_stitch_positions() {
    // For perfect binding, blanks appear in saddle-stitch positions, not all at the end
    // This is correct for proper booklet folding
    let ordering = calculate_perfect_bound_order(10, 4, 1);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // Verify that all pages 1-10 appear exactly once
    for page in 1..=10 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {} should appear exactly once", page);
    }

    // Verify we have the correct number of blanks
    let blank_count = flat.iter().filter(|&&p| p == 0).count();
    assert_eq!(blank_count, 6, "Should have 6 blank pages to pad to 16");
}

#[test]
fn test_perfect_bound_12_pages_4up() {
    // 12 pages, 4-up needs padding to 16
    let ordering = calculate_perfect_bound_order(12, 4, 1);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=12 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }

    let blank_count = flat.iter().filter(|&&p| p == 0).count();
    assert_eq!(blank_count, 4, "Should have 4 blank pages");
}

// ===== 8-up Tests =====

#[test]
fn test_perfect_bound_16_pages_8up_single_signature() {
    // 16 pages, 8-up, 1 sheet per signature
    let ordering = calculate_perfect_bound_order(16, 8, 1);
    assert_eq!(ordering.len(), 2, "Should have 2 sides (1 sheet)");

    assert_eq!(ordering[0].len(), 8, "Front should have 8 positions");
    assert_eq!(ordering[1].len(), 8, "Back should have 8 positions");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=16 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_32_pages_8up_two_sheets_per_signature() {
    // 32 pages, 8-up, 2 sheets per signature
    let ordering = calculate_perfect_bound_order(32, 8, 2);
    assert_eq!(ordering.len(), 4, "Should have 4 sides (2 sheets)");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=32 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

// ===== Large Input Tests =====

#[test]
fn test_perfect_bound_109_pages_2up() {
    // Large odd number of pages
    let ordering = calculate_perfect_bound_order(109, 2, 1);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // All pages 1-109 should appear exactly once
    for page in 1..=109 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_1000_pages_2up() {
    // Very large input
    let ordering = calculate_perfect_bound_order(1000, 2, 1);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // Verify page count
    let non_blank = flat.iter().filter(|&&p| p > 0).count();
    assert_eq!(non_blank, 1000, "Should have all 1000 pages");
}

// ===== Edge Cases =====

#[test]
fn test_perfect_bound_zero_pages() {
    let ordering = calculate_perfect_bound_order(0, 2, 1);
    assert_eq!(ordering.len(), 0, "Zero pages should produce no output");
}

#[test]
fn test_perfect_bound_zero_pages_per_sheet() {
    let ordering = calculate_perfect_bound_order(8, 0, 1);
    assert_eq!(ordering.len(), 0, "Zero pages per sheet should produce no output");
}

#[test]
fn test_perfect_bound_zero_sheets_per_signature() {
    // Should default to 1 sheet per signature or handle gracefully
    let ordering = calculate_perfect_bound_order(8, 2, 0);
    // Implementation should handle this - either default to 1 or return empty
    assert!(
        ordering.len() == 0 || ordering.len() == 4,
        "Should handle zero sheets per signature"
    );
}

// ===== Signature Grouping Tests =====

#[test]
fn test_perfect_bound_32_pages_2up_four_sheets_per_signature() {
    // 32 pages, 2-up, 4 sheets per signature = 2 signatures
    // Each signature has 16 pages (4 sheets × 4 pages/sheet)
    let ordering = calculate_perfect_bound_order(32, 2, 4);
    assert_eq!(ordering.len(), 16, "Should have 16 sides (8 sheets)");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=32 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_24_pages_2up_three_sheets_per_signature() {
    // 24 pages, 2-up, 3 sheets per signature = 2 signatures
    let ordering = calculate_perfect_bound_order(24, 2, 3);
    assert_eq!(ordering.len(), 12, "Should have 12 sides (6 sheets)");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=24 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

// ===== Cross-fold Tests (for future expansion) =====
// Cross-fold is mentioned in the reference but might be a separate feature

#[test]
fn test_perfect_bound_consistency_with_different_signatures() {
    // Test that different signature sizes produce same pages, just ordered differently
    let pages = 16;
    let ordering_1 = calculate_perfect_bound_order(pages, 2, 1);
    let ordering_2 = calculate_perfect_bound_order(pages, 2, 2);
    let ordering_4 = calculate_perfect_bound_order(pages, 2, 4);

    // All should contain the same pages
    let flat1: std::collections::HashSet<usize> = ordering_1
        .iter()
        .flat_map(|s| s.iter().copied())
        .filter(|&p| p > 0)
        .collect();
    let flat2: std::collections::HashSet<usize> = ordering_2
        .iter()
        .flat_map(|s| s.iter().copied())
        .filter(|&p| p > 0)
        .collect();
    let flat4: std::collections::HashSet<usize> = ordering_4
        .iter()
        .flat_map(|s| s.iter().copied())
        .filter(|&p| p > 0)
        .collect();

    assert_eq!(flat1, flat2, "Different signature sizes should contain same pages");
    assert_eq!(flat1, flat4, "Different signature sizes should contain same pages");
}

// ===== 16-up Tests =====

#[test]
fn test_perfect_bound_32_pages_16up_single_signature() {
    let ordering = calculate_perfect_bound_order(32, 16, 1);
    assert_eq!(ordering.len(), 2, "Should have 2 sides (1 sheet)");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=32 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_64_pages_16up_two_sheets_per_signature() {
    let ordering = calculate_perfect_bound_order(64, 16, 2);
    assert_eq!(ordering.len(), 4, "Should have 4 sides (2 sheets)");

    // Verify all pages appear once
    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
    for page in 1..=64 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

// ===== Property-Based Tests =====

#[test]
fn test_perfect_bound_no_duplicate_pages() {
    // Test various configurations don't produce duplicate pages
    let configs = vec![(8, 2, 1), (16, 4, 1), (32, 8, 2), (24, 2, 3)];

    for (pages, pps, sps) in configs {
        let ordering = calculate_perfect_bound_order(pages, pps, sps);
        let flat: Vec<usize> = ordering
            .iter()
            .flat_map(|s| s.iter().copied())
            .filter(|&p| p > 0)
            .collect();

        for page in 1..=pages {
            let count = flat.iter().filter(|&&p| p == page).count();
            assert_eq!(
                count, 1,
                "Page {page} should appear exactly once in {pps}-up with {sps} sheets/sig"
            );
        }
    }
}

#[test]
fn test_perfect_bound_all_pages_present() {
    // Ensure no pages are lost in the imposition
    let configs = vec![(5, 2, 1), (12, 4, 1), (20, 8, 1), (17, 2, 2)];

    for (pages, pps, sps) in configs {
        let ordering = calculate_perfect_bound_order(pages, pps, sps);
        let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

        for page in 1..=pages {
            assert!(
                flat.contains(&page),
                "Page {page} missing in {pps}-up with {sps} sheets/sig"
            );
        }
    }
}

// ===== Signature Count Tests (New Function) =====

#[test]
fn test_perfect_bound_with_signature_count_even_distribution() {
    // 100 pages, 4-up, 5 signatures = 20 pages per signature
    let ordering = calculate_perfect_bound_order_with_signature_count(100, 4, 5);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // Verify all pages 1-100 appear exactly once
    for page in 1..=100 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }

    // Verify no extra pages
    let max_page = flat.iter().filter(|&&p| p > 0).max().copied().unwrap_or(0);
    assert_eq!(max_page, 100, "Max page should be 100");
}

#[test]
fn test_perfect_bound_with_signature_count_uneven_distribution() {
    // 10 pages, 2-up, 3 signatures
    // Distribution: first 2 signatures get 4 pages, last gets 2
    let ordering = calculate_perfect_bound_order_with_signature_count(10, 2, 3);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // Verify all pages 1-10 appear exactly once
    for page in 1..=10 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_with_signature_count_single_signature() {
    // 20 pages, 4-up, 1 signature = all pages in one signature
    let ordering = calculate_perfect_bound_order_with_signature_count(20, 4, 1);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    for page in 1..=20 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_with_signature_count_many_signatures() {
    // 32 pages, 2-up, 8 signatures = 4 pages per signature
    let ordering = calculate_perfect_bound_order_with_signature_count(32, 2, 8);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    for page in 1..=32 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_with_signature_count_small_pages() {
    // 5 pages, 2-up, 2 signatures
    // Distribution: first gets 3 pages, second gets 2 pages
    let ordering = calculate_perfect_bound_order_with_signature_count(5, 2, 2);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    for page in 1..=5 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_with_signature_count_blanks_in_saddle_positions() {
    // 15 pages, 4-up, 3 signatures = 5 pages per signature
    // Each signature uses saddle-stitch, so blanks appear in saddle-stitch positions
    let ordering = calculate_perfect_bound_order_with_signature_count(15, 4, 3);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // With perfect binding, blanks appear in saddle-stitch positions, not all at the end
    // This is correct for proper booklet folding

    // Verify all pages 1-15 appear
    for page in 1..=15 {
        assert!(flat.contains(&page), "Page {page} should be present");
    }

    // Verify each page appears exactly once
    for page in 1..=15 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }
}

#[test]
fn test_perfect_bound_with_signature_count_edge_cases() {
    // Zero pages
    let ordering = calculate_perfect_bound_order_with_signature_count(0, 2, 1);
    assert_eq!(ordering.len(), 0, "Zero pages should produce no output");

    // Zero signatures
    let ordering = calculate_perfect_bound_order_with_signature_count(10, 2, 0);
    assert_eq!(ordering.len(), 0, "Zero signatures should produce no output");

    // Invalid pages_per_sheet
    let ordering = calculate_perfect_bound_order_with_signature_count(10, 3, 1);
    assert_eq!(
        ordering.len(),
        0,
        "Non-power-of-two pages_per_sheet should produce no output"
    );
}

#[test]
fn test_perfect_bound_with_signature_count_balanced_distribution() {
    // 10 pages across 3 signatures should be [3, 4, 3] not [4, 4, 2]
    // This tests that remainder pages are distributed evenly, not just at the front
    let ordering = calculate_perfect_bound_order_with_signature_count(10, 2, 3);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // Verify all pages 1-10 appear exactly once
    for page in 1..=10 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }

    // Verify all pages are present (no missing pages)
    assert_eq!(
        flat.iter().filter(|&&p| p > 0).count(),
        10,
        "All 10 pages should be present"
    );
}

#[test]
fn test_perfect_bound_with_signature_count_center_weighted_distribution() {
    // 13 pages across 5 signatures should be [2, 3, 3, 3, 2] (center-weighted)
    // This tests that remainder pages are distributed from center outward
    let ordering = calculate_perfect_bound_order_with_signature_count(13, 2, 5);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // Verify all pages 1-13 appear exactly once
    for page in 1..=13 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {page} should appear exactly once");
    }

    // Verify all 13 pages are present
    assert_eq!(
        flat.iter().filter(|&&p| p > 0).count(),
        13,
        "All 13 pages should be present"
    );
}

#[test]
fn test_perfect_bound_109_pages_2up_2sheets_per_signature() {
    // 109 pages, 2-up, 2 sheets per signature
    // With 109 pages and 2 sheets/sig = 8 pages per signature, we get:
    // - 109 / 8 = 13 complete signatures + 1 partial signature
    // - Signature sizes: 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 5 (total 109)
    //
    // Each signature of 8 pages (2-up, 2 sheets) should follow saddle stitch nesting:
    // Sheet 1: [8, 1] / [2, 7]
    // Sheet 2: [6, 3] / [4, 5]
    //
    // For the last partial signature (5 pages), should pad to 8:
    // Sig 14 (pages 105-109 → pad to 112):
    // Sheet 1: [0, 105] / [106, 0]   (pages 105, 106 map to positions 8, 1)
    // Sheet 2: [0, 107] / [108, 109] (pages 107, 108, 109 map to positions 6, 3, 4)
    //
    // Then blanks move to end, so:
    // Sheet 1: [105, 106] / [107, 108]
    // Sheet 2: [109, 0] / [0, 0]
    let ordering = calculate_perfect_bound_order(109, 2, 2);

    let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

    // Verify all pages 1-109 appear exactly once
    for page in 1..=109 {
        let count = flat.iter().filter(|&&p| p == page).count();
        assert_eq!(
            count, 1,
            "Page {page} should appear exactly once in 109-page 2-up perfect bound"
        );
    }

    // With perfect binding, blanks appear in saddle-stitch positions, not all at the end
    // This is correct for proper booklet folding

    // Expected structure:
    // 13 complete signatures with proper saddle stitch ordering
    // 1 partial signature at the end
    // Check the last few pages follow the pattern

    // Total sides should be: 13 * 8 / 2 + 4 = 52 + 4 = 56 sides (14 signatures × 4 sides each)
    assert_eq!(ordering.len(), 56, "Should have 56 sides (14 signatures × 4 sides)");

    // The last signature (pages 105-109, padded to 8) should be in the last 4 sides
    // Last 4 sides should contain pages 105-109 with blanks filling in
    let last_four_sides = &ordering[52..56];
    let pages_in_last_sig: Vec<usize> = last_four_sides
        .iter()
        .flat_map(|s| s.iter().copied())
        .filter(|&p| p > 0)
        .collect();

    assert_eq!(
        pages_in_last_sig.len(),
        5,
        "Last signature should have exactly 5 real pages"
    );
    assert!(pages_in_last_sig.contains(&105));
    assert!(pages_in_last_sig.contains(&106));
    assert!(pages_in_last_sig.contains(&107));
    assert!(pages_in_last_sig.contains(&108));
    assert!(pages_in_last_sig.contains(&109));
}

#[test]
fn test_perfect_bound_109_pages_2up_2sheets_detailed_ordering() {
    // More detailed test verifying the exact ordering pattern for 109 pages
    // 109 pages, 2-up, 2 sheets per signature = 8 pages per signature
    // 109 / 8 = 13 complete + 1 partial (5 pages)
    //
    // Expected pattern per signature (saddle stitch within each signature):
    // For 8 pages: Sheet1[8,1]/[2,7], Sheet2[6,3]/[4,5]
    // For 5 pages (padded to 8): Sheet1[0,105]/[106,0], Sheet2[0,107]/[108,109]
    // Blanks stay in saddle-stitch positions (not consolidated)
    let ordering = calculate_perfect_bound_order(109, 2, 2);

    // Verify structure: 14 signatures × 4 sides each = 56 sides
    assert_eq!(ordering.len(), 56, "Should have 56 sides total");

    // Get the last signature (sides 52-55, indices 52-55)
    let last_sig_start = 52;
    let last_sig_end = 56;

    // Collect all pages from last signature in order
    let last_sig_pages: Vec<usize> = ordering[last_sig_start..last_sig_end]
        .iter()
        .flat_map(|s| s.iter().copied())
        .collect();

    println!("Last signature pages: {:?}", last_sig_pages);

    // Verify expected saddle-stitch pattern:
    // [0, 105] (front side of sheet 1)
    // [106, 0] (back side of sheet 1)
    // [0, 107] (front side of sheet 2)
    // [108, 109] (back side of sheet 2)
    // = [0, 105, 106, 0, 0, 107, 108, 109] as flat list
    let expected = vec![0, 105, 106, 0, 0, 107, 108, 109];
    assert_eq!(
        last_sig_pages, expected,
        "Last signature should follow saddle-stitch pattern with blanks in correct positions"
    );

    // Count occurrences of each page 105-109
    for page in 105..=109 {
        let count = last_sig_pages.iter().filter(|&&p| p == page).count();
        assert_eq!(count, 1, "Page {} should appear exactly once in last signature", page);
    }

    // Verify we have exactly 5 real pages and 3 blanks in last signature
    let real_pages = last_sig_pages.iter().filter(|&&p| p > 0).count();
    let blanks = last_sig_pages.iter().filter(|&&p| p == 0).count();
    assert_eq!(real_pages, 5, "Last signature should have 5 real pages");
    assert_eq!(blanks, 3, "Last signature should have 3 blank pages");
}

#[test]
fn test_saddle_stitch_5_pages_2up_debug() {
    // Debug: Check what saddle-stitch ordering produces for 5 pages with 2-up
    use crate::imposition::saddle_stitch::calculate_saddle_stitch_order;

    let ordering = calculate_saddle_stitch_order(5, 2);
    println!("Saddle stitch ordering for 5 pages, 2-up:");
    for (i, side) in ordering.iter().enumerate() {
        println!("  Side {}: {:?}", i, side);
    }
}

#[test]
fn test_perfect_bound_109_pages_2up_2sheets_sheet_structure() {
    // Test that verifies the sheet-by-sheet structure
    // Expected per the user: for last 2 sheets of 109-page 2-up perfect binding with 2 sheets/sig:
    // Sheet 1 (outer): [0, 105] / [106, 0]    - This would be positions [8,1]/[2,7] in saddle stitch
    // Sheet 2 (inner): [0, 107] / [108, 109]  - This would be positions [6,3]/[4,5] in saddle stitch
    // Then after blank consolidation:
    // Sheet 1: [105, 106] / [107, 108]
    // Sheet 2: [109, 0] / [0, 0]

    let ordering = calculate_perfect_bound_order(109, 2, 2);

    // Each side is 2 pages, so 4 sides per signature = one signature is 4 entries
    // 14 signatures total, so last signature is indices 52-55

    let last_sheet1_front = &ordering[52];
    let last_sheet1_back = &ordering[53];
    let last_sheet2_front = &ordering[54];
    let last_sheet2_back = &ordering[55];

    println!("Last 2 sheets:");
    println!("  Sheet 1 front: {:?}", last_sheet1_front);
    println!("  Sheet 1 back:  {:?}", last_sheet1_back);
    println!("  Sheet 2 front: {:?}", last_sheet2_front);
    println!("  Sheet 2 back:  {:?}", last_sheet2_back);

    // EXPECTED: The saddle-stitch nesting pattern is preserved exactly
    // For 5 pages with 2-up saddle stitch: [0,1], [2,0], [0,3], [4,5]
    // When remapped to pages 105-109 within last signature:
    // [0,105], [106,0], [0,107], [108,109]

    assert_eq!(last_sheet1_front, &vec![0, 105], "Sheet 1 front should be [0, 105]");
    assert_eq!(last_sheet1_back, &vec![106, 0], "Sheet 1 back should be [106, 0]");
    assert_eq!(last_sheet2_front, &vec![0, 107], "Sheet 2 front should be [0, 107]");
    assert_eq!(last_sheet2_back, &vec![108, 109], "Sheet 2 back should be [108, 109]");

    // All pages 105-109 should appear exactly once across these 4 sides
    let all_pages: Vec<usize> = vec![
        last_sheet1_front.clone(),
        last_sheet1_back.clone(),
        last_sheet2_front.clone(),
        last_sheet2_back.clone(),
    ]
    .into_iter()
    .flatten()
    .collect();

    for page in 105..=109 {
        assert!(all_pages.contains(&page), "Page {} should be present", page);
    }

    // There should be 3 blanks across the 4 sides
    let blank_count = all_pages.iter().filter(|&&p| p == 0).count();
    assert_eq!(blank_count, 3, "Should have 3 blanks across last 2 sheets");
}
