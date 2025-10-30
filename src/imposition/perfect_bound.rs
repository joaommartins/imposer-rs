/// Calculate page ordering for perfect bound booklets
///
/// **⚠️ WARNING: This module is a Work In Progress (WIP) and not ready for production use!**
///
/// Pages are arranged for work-and-turn or work-and-tumble printing
/// For 2-up: simple layout with [1,4] front, [3,2] back
/// For 4-up: head-to-head layout with top row upside down
///
/// # Note
///
/// This implementation is experimental and has not been fully tested or verified.
/// Do not use for production booklet generation. Use saddle stitch binding instead.
#[deprecated(
    since = "0.1.0",
    note = "Perfect binding is work in progress and not yet ready for use"
)]
pub fn calculate_perfect_bound_order(num_pages: usize, pages_per_sheet: usize) -> Vec<Vec<usize>> {
    if pages_per_sheet == 0 {
        return Vec::new();
    }

    if pages_per_sheet == 2 {
        // 2-up: simple short-edge duplex layout
        calculate_perfect_bound_2up(num_pages)
    } else if pages_per_sheet == 4 {
        // 4-up: head-to-head layout (top row upside down)
        calculate_perfect_bound_4up(num_pages)
    } else {
        // For other n-up, use simple sequential layout
        calculate_perfect_bound_simple(num_pages, pages_per_sheet)
    }
}

/// 2-up perfect binding: [1,4] front, [3,2] back
fn calculate_perfect_bound_2up(num_pages: usize) -> Vec<Vec<usize>> {
    let pages_per_physical_sheet = 4;
    let total_pages = num_pages.div_ceil(pages_per_physical_sheet) * pages_per_physical_sheet;

    let mut ordering = Vec::new();
    let num_sheets = total_pages / pages_per_physical_sheet;

    for sheet in 0..num_sheets {
        let base_page = sheet * pages_per_physical_sheet + 1;

        // Front: [1, 4], [5, 8], etc.
        let front_pages = vec![
            if base_page <= num_pages { base_page } else { 0 },
            if base_page + 3 <= num_pages {
                base_page + 3
            } else {
                0
            },
        ];
        ordering.push(front_pages);

        // Back: [3, 2], [7, 6], etc.
        let back_pages = vec![
            if base_page + 2 <= num_pages {
                base_page + 2
            } else {
                0
            },
            if base_page < num_pages {
                base_page + 1
            } else {
                0
            },
        ];
        ordering.push(back_pages);
    }

    ordering
}

/// 4-up perfect binding with head-to-head layout
/// Front: [5, 4(upside down), 8, 1] -> grid: top row [5,4], bottom row [8,1]
/// Back: [3, 6(upside down), 2, 7] -> grid: top row [3,6], bottom row [2,7]
fn calculate_perfect_bound_4up(num_pages: usize) -> Vec<Vec<usize>> {
    let pages_per_physical_sheet = 8;
    let total_pages = num_pages.div_ceil(pages_per_physical_sheet) * pages_per_physical_sheet;

    let mut ordering = Vec::new();
    let num_sheets = total_pages / pages_per_physical_sheet;

    for sheet in 0..num_sheets {
        let base = sheet * pages_per_physical_sheet;

        // Front: [5, 4, 8, 1] (positions: top-left, top-right, bottom-left, bottom-right)
        let front_pages = vec![
            if base + 5 <= num_pages { base + 5 } else { 0 },
            if base + 4 <= num_pages { base + 4 } else { 0 },
            if base + 8 <= num_pages { base + 8 } else { 0 },
            if base < num_pages { base + 1 } else { 0 },
        ];
        ordering.push(front_pages);

        // Back: [3, 6, 2, 7] (positions: top-left, top-right, bottom-left, bottom-right)
        let back_pages = vec![
            if base + 3 <= num_pages { base + 3 } else { 0 },
            if base + 6 <= num_pages { base + 6 } else { 0 },
            if base + 2 <= num_pages { base + 2 } else { 0 },
            if base + 7 <= num_pages { base + 7 } else { 0 },
        ];
        ordering.push(back_pages);
    }

    ordering
}

/// Simple sequential layout for other n-up configurations
fn calculate_perfect_bound_simple(num_pages: usize, pages_per_sheet: usize) -> Vec<Vec<usize>> {
    let pages_per_physical_sheet = pages_per_sheet * 2;
    let total_pages = num_pages.div_ceil(pages_per_physical_sheet) * pages_per_physical_sheet;

    let mut ordering = Vec::new();
    let num_sheets = total_pages / pages_per_physical_sheet;

    for sheet in 0..num_sheets {
        let base_page = sheet * pages_per_physical_sheet + 1;

        // Front: left side has low pages, right side has high pages
        let mut front_pages = Vec::new();
        for i in 0..pages_per_sheet {
            let page_num = if i < pages_per_sheet / 2 {
                base_page + i
            } else {
                base_page + pages_per_physical_sheet - 1 - (i - pages_per_sheet / 2)
            };

            if page_num <= num_pages {
                front_pages.push(page_num);
            } else {
                front_pages.push(0);
            }
        }
        ordering.push(front_pages);

        // Back: middle pages in reverse order
        let mut back_pages = Vec::new();
        for i in (0..pages_per_sheet).rev() {
            let page_num = base_page + pages_per_sheet / 2 + i;

            if page_num <= num_pages {
                back_pages.push(page_num);
            } else {
                back_pages.push(0);
            }
        }
        ordering.push(back_pages);
    }

    ordering
}

#[cfg(test)]
#[expect(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_bound_4_pages_2up() {
        let ordering = calculate_perfect_bound_order(4, 2);
        // Perfect bound: front [1, 4], back [3, 2] (reversed for short-edge duplex)
        assert_eq!(ordering, vec![vec![1, 4], vec![3, 2]]);
    }

    #[test]
    fn test_perfect_bound_8_pages_2up() {
        let ordering = calculate_perfect_bound_order(8, 2);
        // Perfect bound: [1, 4], [3, 2], [5, 8], [7, 6]
        assert_eq!(
            ordering,
            vec![vec![1, 4], vec![3, 2], vec![5, 8], vec![7, 6]]
        );
    }

    #[test]
    fn test_perfect_bound_8_pages_4up() {
        let ordering = calculate_perfect_bound_order(8, 4);
        // Perfect bound 4-up with head-to-head layout:
        // Front: [5, 4, 8, 1] (top row: 5, 4(upside down); bottom row: 8, 1)
        // Back: [3, 6, 2, 7] (top row: 3, 6(upside down); bottom row: 2, 7)
        assert_eq!(ordering, vec![vec![5, 4, 8, 1], vec![3, 6, 2, 7]]);
    }
}
