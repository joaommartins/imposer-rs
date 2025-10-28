/// Calculate grid layout for saddle stitch - prefers most square layout
/// Returns (rows, cols) aiming for rows ≈ cols when possible
pub fn calculate_saddle_stitch_grid(pages_per_sheet: usize) -> (usize, usize) {
    if pages_per_sheet == 0 {
        return (1, 1);
    }

    // Find the factor closest to sqrt(n) to get the most square grid
    let sqrt = (pages_per_sheet as f64).sqrt();
    let sqrt_floor = sqrt.floor() as usize;

    // Try factors starting from sqrt going down
    for r in (1..=sqrt_floor).rev() {
        if pages_per_sheet % r == 0 {
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
}
