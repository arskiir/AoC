use std::collections::BTreeSet;

pub fn part1(input: &str) -> usize {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut inner_visible_trees: BTreeSet<(usize, usize)> = BTreeSet::new();

    // check horizontally
    (1..(map.len() - 1)).for_each(|inner_col_idx| {
        let row = &map[inner_col_idx];
        row.iter().enumerate().for_each(|(tree_x, height)| {
            if tree_x == 0 || tree_x == map[0].len() - 1 {
                return;
            }
            for range in [(0..tree_x), ((tree_x + 1)..row.len())].into_iter() {
                if let Some(max) = range.map(|i| row[i]).max() {
                    if max < *height {
                        inner_visible_trees.insert((tree_x, inner_col_idx));
                    }
                }
            }
        })
    });

    // check vertically
    (1..(map[0].len() - 1)).for_each(|inner_row_idx| {
        let col: Vec<usize> = map.iter().map(|row| row[inner_row_idx]).collect();
        col.iter().enumerate().for_each(|(tree_y, height)| {
            if tree_y == 0 || tree_y == map.len() - 1 {
                return;
            }
            for range in [(0..tree_y), ((tree_y + 1)..col.len())].into_iter() {
                if let Some(max) = range.map(|i| col[i]).max() {
                    if max < *height {
                        inner_visible_trees.insert((inner_row_idx, tree_y));
                    }
                }
            }
        });
    });

    (map[0].len() + map.len() - 2) * 2 + inner_visible_trees.len()
}

pub fn part2(input: &str) -> usize {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut inner_visible_trees: BTreeSet<(usize, usize)> = BTreeSet::new();

    // check horizontally
    (1..(map.len() - 1)).for_each(|inner_col_idx| {
        let row = &map[inner_col_idx];
        row.iter().enumerate().for_each(|(tree_x, height)| {
            if tree_x == 0 || tree_x == map[0].len() - 1 {
                return;
            }
            for range in [(0..tree_x), ((tree_x + 1)..row.len())].into_iter() {
                if let Some(max) = range.map(|i| row[i]).max() {
                    if max < *height {
                        inner_visible_trees.insert((tree_x, inner_col_idx));
                    }
                }
            }
        })
    });

    // check vertically
    (1..(map[0].len() - 1)).for_each(|inner_row_idx| {
        let col: Vec<usize> = map.iter().map(|row| row[inner_row_idx]).collect();
        col.iter().enumerate().for_each(|(tree_y, height)| {
            if tree_y == 0 || tree_y == map.len() - 1 {
                return;
            }
            for range in [(0..tree_y), ((tree_y + 1)..col.len())].into_iter() {
                if let Some(max) = range.map(|i| col[i]).max() {
                    if max < *height {
                        inner_visible_trees.insert((inner_row_idx, tree_y));
                    }
                }
            }
        });
    });

    (map[0].len() + map.len() - 2) * 2 + inner_visible_trees.len()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn ex_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 21);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 1814);
    }
}
