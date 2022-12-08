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

    fn split_reverse(mid: usize, row_or_col: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
        let (to_be_reversed, in_order_other_half) = row_or_col.split_at(mid);
        let reversed: Vec<usize> = to_be_reversed.iter().rev().map(|a| *a).collect();
        let (_, in_order_other_half) = in_order_other_half.split_at(1);
        let in_order_other_half: Vec<usize> = in_order_other_half.iter().map(|a| *a).collect();
        (reversed, in_order_other_half)
    }

    let mut scores: BTreeSet<usize> = BTreeSet::new();

    // consider only inner tress, outer trees always have scenic scores of 0
    for tree_y in 1..(map.len() - 1) {
        for tree_x in 1..(map[0].len() - 1) {
            let row = &map[tree_y];
            let col: Vec<usize> = map.iter().map(|row| row[tree_x]).collect();
            let height = row[tree_x];

            let (row_before, row_after) = split_reverse(tree_x, row);
            let (col_above, col_below) = split_reverse(tree_y, &col);

            let scenic_score: usize = [&row_before, &row_after, &col_above, &col_below]
                .iter()
                .map(|row_or_col| {
                    row_or_col
                        .iter()
                        .enumerate()
                        .map(|a| (a.0 + 1, a.1))
                        .find(|(_, other_height)| *other_height >= &height)
                        .unwrap_or((row_or_col.len(), &0))
                        .0
                })
                .product();
            scores.insert(scenic_score);
        }
    }

    *scores.iter().max().unwrap()
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

    #[test]
    fn ex_part2_works() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 330786);
    }
}
