use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Point {
    pub fn build(data: &str) -> Self {
        let binding = data.to_string();
        let mut number_it = binding.split(',').map(|n| n.parse::<i32>().unwrap());
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

    pub fn points(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            return Self::ascending_range_inclusive(self.start.y, self.end.y)
                .map(|y| Point { x: self.start.x, y })
                .collect();
        }
        if self.start.y == self.end.y {
            return Self::ascending_range_inclusive(self.start.x, self.end.x)
                .map(|x| Point { x, y: self.start.y })
                .collect();
        }
        // diagonal 45 deg
        // let start x always be less than end x
        let line = if self.start.x <= self.end.x {
            Line {
                start: self.start.clone(),
                end: self.end.clone(),
            }
        } else {
            Line {
                start: self.end.clone(),
                end: self.start.clone(),
            }
        };
        // 45 deg or -45 deg
        let points = if line.start.y < line.end.y {
            // 45 deg
            //  /
            // /
            (line.start.x..=line.end.x)
                .enumerate()
                .map(|(i, x)| Point {
                    x,
                    y: line.start.y + i as i32,
                })
                .collect()
        } else {
            // -45 deg
            // \
            //  \
            (line.start.x..=line.end.x)
                .enumerate()
                .map(|(i, x)| Point {
                    x,
                    y: line.start.y - i as i32,
                })
                .collect()
        };
        points
    }

    fn ascending_range_inclusive(a: i32, b: i32) -> std::ops::RangeInclusive<i32> {
        if a <= b {
            a..=b
        } else {
            b..=a
        }
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

fn draw(drawn_coords: &mut HashMap<Point, i32>, point: Point) {
    match drawn_coords.get_mut(&point) {
        Some(overlap) => *overlap += 1,
        None => {
            drawn_coords.insert(point, 1);
        }
    };
}

pub fn part1(input: &str) -> i32 {
    let lines = parse_input(input);
    let horizontal_vertical_lines: Vec<Line> = lines
        .into_iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect();

    let mut drawn_coords: HashMap<Point, i32> = HashMap::new();

    for line in horizontal_vertical_lines {
        for point in line.points() {
            draw(&mut drawn_coords, point);
        }
    }

    drawn_coords
        .values()
        .map(|overlap| if *overlap >= 2 { 1 } else { 0 })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let lines = parse_input(input);

    let mut drawn_coords: HashMap<Point, i32> = HashMap::new();

    for line in lines {
        for point in line.points() {
            draw(&mut drawn_coords, point);
        }
    }

    drawn_coords
        .values()
        .map(|overlap| if *overlap >= 2 { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

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
    fn example_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 5);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 5084);
    }

    #[test]
    fn example_part2_works() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 12);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 17882);
    }
}
