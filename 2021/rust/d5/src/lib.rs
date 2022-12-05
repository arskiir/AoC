use std::collections::HashSet;

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Point {
    pub fn build(data: &str) -> Self {
        let binding = data.to_string();
        let mut number_it = binding.split(',').map(|n| n.parse::<u32>().unwrap());
        Self {
            x: number_it.next().unwrap(),
            y: number_it.next().unwrap(),
        }
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

fn parse_row(row: &str) -> Line {
    let mut row_it = row.split(" -> ");
    Line::new(
        Point::build(row_it.next().unwrap()),
        Point::build(row_it.next().unwrap()),
    )
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(parse_row).collect()
}

pub struct Coordinate<'a> {
    point: &'a Point,
    overlap: u32,
}

impl<'a> Coordinate<'a> {
    pub fn new(point: &'a Point) -> Self {
        Self { point, overlap: 0 }
    }
}

pub fn part1(input: &str) -> u32 {
    let lines = parse_input(input);
    let horizontal_vertical_lines: Vec<Line> = lines
        .into_iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect();

    let drawn_coords: HashSet<Coordinate> = HashSet::new();

    dbg!(&horizontal_vertical_lines);
    32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn it_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 4);
    }
}
