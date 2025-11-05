/// Calculate grid layout for saddle stitch - prefers most square layout
/// Returns (rows, cols) aiming for rows ≈ cols when possible
pub fn calculate_saddle_stitch_grid(pages_per_sheet: usize) -> (usize, usize) {
    if pages_per_sheet == 0 {
        return (1, 1);
    }

    // Find the factor closest to sqrt(n) to get the most square grid
    // Note: usize to f64 precision loss is acceptable for grid calculations
    // since page grid sizes are always reasonable (≤ 1024 in practice)
    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let sqrt = (pages_per_sheet as f64).sqrt().floor() as usize;

    // Try factors starting from sqrt going down
    for r in (1..=sqrt).rev() {
        if pages_per_sheet.is_multiple_of(r) {
            let cols = pages_per_sheet / r;
            return (r, cols);
        }
    }

    // Fallback (shouldn't reach here for perfect squares/valid n-up)
    (1, pages_per_sheet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_2up() {
        let (rows, cols) = calculate_saddle_stitch_grid(2);
        assert_eq!((rows, cols), (1, 2)); // 1x2 landscape
    }

    #[test]
    fn test_grid_4up() {
        let (rows, cols) = calculate_saddle_stitch_grid(4);
        assert_eq!((rows, cols), (2, 2)); // 2x2 square
    }

    #[test]
    fn test_grid_8up() {
        let (rows, cols) = calculate_saddle_stitch_grid(8);
        assert_eq!((rows, cols), (2, 4)); // 2x4 landscape
    }

    #[test]
    fn test_grid_16up() {
        let (rows, cols) = calculate_saddle_stitch_grid(16);
        assert_eq!((rows, cols), (4, 4)); // 4x4 square
    }

    #[test]
    fn test_grid_32up() {
        let (rows, cols) = calculate_saddle_stitch_grid(32);
        assert_eq!((rows, cols), (4, 8)); // 4x8 landscape
    }

    #[test]
    fn test_grid_64up() {
        let (rows, cols) = calculate_saddle_stitch_grid(64);
        assert_eq!((rows, cols), (8, 8)); // 8x8 square
    }

    #[test]
    fn test_grid_128up() {
        let (rows, cols) = calculate_saddle_stitch_grid(128);
        assert_eq!((rows, cols), (8, 16)); // 8x16 landscape
    }

    #[test]
    fn test_grid_256up() {
        let (rows, cols) = calculate_saddle_stitch_grid(256);
        assert_eq!((rows, cols), (16, 16)); // 16x16 square
    }

    #[test]
    fn test_grid_512up() {
        let (rows, cols) = calculate_saddle_stitch_grid(512);
        assert_eq!((rows, cols), (16, 32)); // 16x32 landscape
    }

    #[test]
    fn test_grid_1024up() {
        let (rows, cols) = calculate_saddle_stitch_grid(1024);
        assert_eq!((rows, cols), (32, 32)); // 32x32 square
    }

    #[test]
    fn test_grid_zero() {
        let (rows, cols) = calculate_saddle_stitch_grid(0);
        assert_eq!((rows, cols), (1, 1)); // Edge case: fallback to 1x1
    }

    #[test]
    fn test_grid_one() {
        let (rows, cols) = calculate_saddle_stitch_grid(1);
        assert_eq!((rows, cols), (1, 1)); // Edge case: 1x1
    }

    // ===== Property Tests =====

    #[test]
    fn test_grid_produces_correct_total() {
        // Test that rows × cols = pages_per_sheet for all powers of 2
        let powers_of_2 = vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

        for pages in powers_of_2 {
            let (rows, cols) = calculate_saddle_stitch_grid(pages);
            assert_eq!(rows * cols, pages, "Grid {rows}×{cols} should produce {pages} pages");
        }
    }

    #[test]
    fn test_grid_prefers_square_or_landscape() {
        // Test that the algorithm produces square grids for perfect squares
        // or landscape (cols >= rows) for non-squares
        let test_cases = vec![
            (4, true),    // 2x2 is square
            (16, true),   // 4x4 is square
            (64, true),   // 8x8 is square
            (256, true),  // 16x16 is square
            (1024, true), // 32x32 is square
            (2, false),   // 1x2 is landscape
            (8, false),   // 2x4 is landscape
            (32, false),  // 4x8 is landscape
            (128, false), // 8x16 is landscape
            (512, false), // 16x32 is landscape
        ];

        for (pages, is_square) in test_cases {
            let (rows, cols) = calculate_saddle_stitch_grid(pages);

            if is_square {
                assert_eq!(rows, cols, "Grid for {pages} pages should be square, got {rows}×{cols}");
            } else {
                assert!(
                    cols >= rows,
                    "Grid for {pages} pages should be landscape (cols >= rows), got {rows}×{cols}"
                );
            }
        }
    }

    #[test]
    fn test_grid_rows_close_to_cols() {
        // Test that the algorithm produces grids where rows ≈ cols (most square)
        // For powers of 2, the ratio should be at most 2:1
        let powers_of_2 = vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

        for pages in powers_of_2 {
            let (rows, cols) = calculate_saddle_stitch_grid(pages);
            let ratio = cols as f64 / rows as f64;

            assert!(
                ratio <= 2.0,
                "Grid {rows}×{cols} for {pages} pages has ratio {ratio}, should be ≤ 2.0"
            );
        }
    }

    #[test]
    fn test_grid_non_power_of_2_fallbacks() {
        // Test non-power-of-2 values (should still work, finding best factors)
        let test_cases = vec![
            (3, (1, 3)),  // Prime: 1x3
            (5, (1, 5)),  // Prime: 1x5
            (6, (2, 3)),  // 2x3
            (9, (3, 3)),  // 3x3 square
            (10, (2, 5)), // 2x5
            (12, (3, 4)), // 3x4
            (15, (3, 5)), // 3x5
            (18, (3, 6)), // 3x6
            (20, (4, 5)), // 4x5
            (24, (4, 6)), // 4x6
            (25, (5, 5)), // 5x5 square
            (36, (6, 6)), // 6x6 square
        ];

        for (pages, expected) in test_cases {
            let grid = calculate_saddle_stitch_grid(pages);
            assert_eq!(
                grid, expected,
                "Grid for {pages} pages should be {expected:?}, got {grid:?}"
            );
        }
    }

    #[test]
    fn test_grid_very_large_values() {
        // Test very large values to ensure no overflow/panic
        let large_values = vec![2048, 4096, 8192, 16384];

        for pages in large_values {
            let (rows, cols) = calculate_saddle_stitch_grid(pages);
            assert_eq!(
                rows * cols,
                pages,
                "Grid for {pages} pages should produce correct total"
            );

            // Should still produce reasonable grids (not 1×n)
            assert!(
                rows > 1,
                "Grid for {pages} pages should have more than 1 row, got {rows}×{cols}"
            );
        }
    }
}
