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
        // 5 pages requires 3 sides min, rounded to 4 sides (even) = 8 page slots
        // 3 blanks: 3/4=0, use saddle-stitch nesting
        let ordering = calculate_page_order(5, 2, BindingType::SaddleStitch);
        assert_eq!(ordering.len(), 4); // 4 sides (2 sheets)
        assert_eq!(ordering[0], vec![0, 1]);
        assert_eq!(ordering[1], vec![2, 0]);
        assert_eq!(ordering[2], vec![0, 3]);
        assert_eq!(ordering[3], vec![4, 5]);
    }

    #[test]
    fn test_page_ordering_1_page_2up() {
        // 1 page padded to 2 = 1 pair on front, back is blank
        let ordering = calculate_page_order(1, 2, BindingType::SaddleStitch);
        assert_eq!(ordering.len(), 2); // Front and back
        assert_eq!(ordering[0], vec![0, 1]);
        assert_eq!(ordering[1], vec![0, 0]);
    }

    #[test]
    fn test_page_ordering_8_pages_4up() {
        let ordering = calculate_page_order(8, 4, BindingType::SaddleStitch);
        // 4-up zigzag: pairs 0,2 on front; pairs 1,3 on back (left-to-right)
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
