#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::config::BindingType;
    use crate::imposition::calculate_page_order;

    #[test]
    fn test_page_ordering_4_pages_2up() {
        let ordering = calculate_page_order(4, 2, BindingType::SaddleStitch, 1);
        // For 2-up booklets, pages alternate: outside pages on front, inside on back
        assert_eq!(ordering, vec![vec![4, 1], vec![2, 3]]);
    }

    #[test]
    fn test_page_ordering_8_pages_2up() {
        let ordering = calculate_page_order(8, 2, BindingType::SaddleStitch, 1);
        // Sheet 1 front: [8, 1], back: [2, 7]
        // Sheet 2 front: [6, 3], back: [4, 5]
        assert_eq!(ordering, vec![vec![8, 1], vec![2, 7], vec![6, 3], vec![4, 5]]);
    }

    #[test]
    fn test_page_ordering_5_pages_2up() {
        // 5 pages requires 3 sides min, rounded to 4 sides (even) = 8 page slots
        // 3 blanks: 3/4=0, use saddle-stitch nesting
        let ordering = calculate_page_order(5, 2, BindingType::SaddleStitch, 1);
        assert_eq!(ordering.len(), 4); // 4 sides (2 sheets)
        assert_eq!(ordering[0], vec![0, 1]);
        assert_eq!(ordering[1], vec![2, 0]);
        assert_eq!(ordering[2], vec![0, 3]);
        assert_eq!(ordering[3], vec![4, 5]);
    }

    #[test]
    fn test_page_ordering_1_page_2up() {
        // 1 page padded to 2 = 1 pair on front, back is blank
        let ordering = calculate_page_order(1, 2, BindingType::SaddleStitch, 1);
        assert_eq!(ordering.len(), 2); // Front and back
        assert_eq!(ordering[0], vec![0, 1]);
        assert_eq!(ordering[1], vec![0, 0]);
    }

    #[test]
    fn test_page_ordering_8_pages_4up() {
        let ordering = calculate_page_order(8, 4, BindingType::SaddleStitch, 1);
        // 4-up zigzag: pairs 0,2 on front; pairs 1,3 on back (left-to-right)
        assert_eq!(ordering, vec![vec![8, 1, 6, 3], vec![2, 7, 4, 5]]);
    }

    #[test]
    fn test_perfect_bound_4_pages_2up() {
        let ordering = calculate_page_order(4, 2, BindingType::PerfectBound, 1);
        // Perfect bound uses saddle stitch ordering: front [4, 1], back [2, 3]
        assert_eq!(ordering, vec![vec![4, 1], vec![2, 3]]);
    }

    #[test]
    fn test_perfect_bound_8_pages_2up() {
        let ordering = calculate_page_order(8, 2, BindingType::PerfectBound, 1);
        // Perfect bound with 1 sheet/signature: 2 signatures using saddle stitch ordering
        assert_eq!(ordering, vec![vec![4, 1], vec![2, 3], vec![8, 5], vec![6, 7]]);
    }

    #[test]
    fn test_perfect_bound_8_pages_4up() {
        let ordering = calculate_page_order(8, 4, BindingType::PerfectBound, 1);
        // Perfect bound 4-up with 1 sheet per signature
        assert_eq!(ordering.len(), 2);
        // All pages should appear exactly once
        let flat: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();
        for page in 1..=8 {
            assert_eq!(flat.iter().filter(|&&p| p == page).count(), 1);
        }
    }

    #[rstest]
    #[case(1, 2)] // 1 page, 2-up
    #[case(5, 2)] // 5 pages, 2-up
    #[case(9, 2)] // 9 pages, 2-up
    #[case(109, 2)] // 109 pages, 2-up
    #[case(7, 4)] // 7 pages, 4-up
    #[case(15, 4)] // 15 pages, 4-up
    #[case(111, 8)] // 111 pages, 8-up
    fn test_no_duplicate_pages_with_odd_input(#[case] num_pages: usize, #[case] pages_per_sheet: usize) {
        let ordering = calculate_page_order(num_pages, pages_per_sheet, BindingType::SaddleStitch, 1);
        let flattened: Vec<usize> = ordering.iter().flat_map(|s| s.iter().copied()).collect();

        // Check that no page (except blank 0) appears more than once
        for page in 1..=num_pages {
            let count = flattened.iter().filter(|&&p| p == page).count();
            assert_eq!(
                count, 1,
                "With {num_pages} pages and {pages_per_sheet}-up: Page {page} appears {count} times (should be 1)"
            );
        }
    }
}
