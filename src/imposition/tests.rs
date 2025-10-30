#[cfg(test)]
mod tests {
    use crate::config::BindingType;
    use crate::imposition::calculate_page_order;
    use rstest::rstest;

    #[test]
    fn test_page_ordering_4_pages_2up() {
        let ordering = calculate_page_order(4, 2, BindingType::SaddleStitch);
        // For 2-up booklets, pages alternate: outside pages on front, inside on back
        assert_eq!(ordering, vec![vec![4, 1], vec![2, 3]]);
    }

    #[test]
    fn test_page_ordering_8_pages_2up() {
        let ordering = calculate_page_order(8, 2, BindingType::SaddleStitch);
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
        let ordering = calculate_page_order(5, 2, BindingType::SaddleStitch);
        // 6 pages: sheets_per_section = (6 + 3) / 4 = 2, but last back is duplicate so: 3 entries
        assert_eq!(ordering.len(), 3);
    }

    #[test]
    fn test_page_ordering_1_page_2up() {
        // Should pad to 2 pages (even number)
        // With deduplication: front and back would be identical, so only front is output
        let ordering = calculate_page_order(1, 2, BindingType::SaddleStitch);
        // 2 pages: sheets_per_section = (2 + 3) / 4 = 1, last back is duplicate so: 1 entry
        assert_eq!(ordering.len(), 1);
        assert_eq!(ordering[0].len(), 2); // 2 pages per side
    }

    #[test]
    fn test_page_ordering_8_pages_4up() {
        let ordering = calculate_page_order(8, 4, BindingType::SaddleStitch);
        // For 4-up (2×2 grid), single sheet:
        // Front: [8, 1, 6, 3], Back: [2, 7, 4, 5] (no pair reversal, n ≤ 4)
        assert_eq!(ordering, vec![vec![8, 1, 6, 3], vec![2, 7, 4, 5]]);
    }

    #[test]
    fn test_perfect_bound_4_pages_2up() {
        let ordering = calculate_page_order(4, 2, BindingType::PerfectBound);
        // Perfect bound: front [1, 4], back [3, 2] (reversed for short-edge duplex)
        assert_eq!(ordering, vec![vec![1, 4], vec![3, 2]]);
    }

    #[test]
    fn test_perfect_bound_8_pages_2up() {
        let ordering = calculate_page_order(8, 2, BindingType::PerfectBound);
        // Perfect bound: [1, 4], [3, 2], [5, 8], [7, 6]
        assert_eq!(
            ordering,
            vec![vec![1, 4], vec![3, 2], vec![5, 8], vec![7, 6]]
        );
    }

    #[test]
    fn test_perfect_bound_8_pages_4up() {
        let ordering = calculate_page_order(8, 4, BindingType::PerfectBound);
        // Perfect bound 4-up with head-to-head layout:
        // Front: [5, 4, 8, 1] (top row: 5, 4(upside down); bottom row: 8, 1)
        // Back: [3, 6, 2, 7] (top row: 3, 6(upside down); bottom row: 2, 7)
        assert_eq!(ordering, vec![vec![5, 4, 8, 1], vec![3, 6, 2, 7]]);
    }

    #[rstest]
    #[case(1, 2)] // 1 page, 2-up
    #[case(5, 2)] // 5 pages, 2-up
    #[case(9, 2)] // 9 pages, 2-up
    #[case(109, 2)] // 109 pages, 2-up
    #[case(7, 4)] // 7 pages, 4-up
    #[case(15, 4)] // 15 pages, 4-up
    #[case(111, 8)] // 111 pages, 8-up
    fn test_no_duplicate_pages_with_odd_input(
        #[case] num_pages: usize,
        #[case] pages_per_sheet: usize,
    ) {
        let ordering = calculate_page_order(num_pages, pages_per_sheet, BindingType::SaddleStitch);
        let flattened: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

        // Check that no page (except blank 0) appears more than once
        for page in 1..=num_pages {
            let count = flattened.iter().filter(|&&p| p == page).count();
            assert_eq!(
                count, 1,
                "With {} pages and {}-up: Page {} appears {} times (should be 1)",
                num_pages, pages_per_sheet, page, count
            );
        }
    }
}
