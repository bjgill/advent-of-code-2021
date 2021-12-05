use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

/// In this system, lines can only ever be horizontal, vertical, or at 45 degrees.
#[derive(Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_forward_slope(&self) -> bool {
        (self.start.x > self.end.x && self.start.y > self.end.y)
            || (self.start.x < self.end.x && self.start.y < self.end.y)
    }

    fn points_on_line(&self) -> Vec<Point> {
        if self.is_vertical() {
            let min = usize::min(self.start.y, self.end.y);
            let max = usize::max(self.start.y, self.end.y);

            (min..=max).map(|y| Point { x: self.start.x, y }).collect()
        } else if self.is_horizontal() {
            let min = usize::min(self.start.x, self.end.x);
            let max = usize::max(self.start.x, self.end.x);

            (min..=max).map(|x| Point { x, y: self.start.y }).collect()
        } else if self.is_forward_slope() {
            if self.end.x > self.start.x {
                (self.start.x..=self.end.x)
                    .zip(self.start.y..=self.end.y)
                    .map(|(x, y)| Point { x, y })
                    .collect()
            } else {
                (self.end.x..=self.start.x)
                    .zip(self.end.y..=self.start.y)
                    .map(|(x, y)| Point { x, y })
                    .collect()
            }
        } else {
            if self.end.x > self.start.x {
                (self.start.x..=self.end.x)
                    .zip((self.end.y..=self.start.y).rev())
                    .map(|(x, y)| Point { x, y })
                    .collect()
            } else {
                (self.end.x..=self.start.x)
                    .zip((self.start.y..=self.end.y).rev())
                    .map(|(x, y)| Point { x, y })
                    .collect()
            }
        }
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
}

fn parse_line(line: &str) -> Line {
    let captures = LINE_REGEX.captures(line).unwrap();

    Line {
        start: Point {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        },
        end: Point {
            x: captures[3].parse().unwrap(),
            y: captures[4].parse().unwrap(),
        },
    }
}

fn parse_lines(input: &str) -> Vec<Line> {
    input.split('\n').map(parse_line).collect()
}

fn get_furthest_corner(lines: &[Line]) -> Point {
    Point {
        x: lines
            .iter()
            .map(|l| usize::max(l.start.x, l.end.x))
            .max()
            .unwrap(),
        y: lines
            .iter()
            .map(|l| usize::max(l.start.y, l.end.y))
            .max()
            .unwrap(),
    }
}

/// Horizontal and vertical lines only
fn get_overlapping_orthogonal_lines(lines: Vec<Line>) -> usize {
    get_overlapping_lines(
        lines
            .into_iter()
            .filter(|l| l.is_horizontal() || l.is_vertical())
            .collect(),
    )
}

fn get_overlapping_lines(lines: Vec<Line>) -> usize {
    let furthest_corner = get_furthest_corner(&lines);

    let mut sea_bed = vec![vec![0; furthest_corner.y + 1]; furthest_corner.x + 1];

    for line in lines {
        for point in line.points_on_line() {
            sea_bed[point.x][point.y] += 1
        }
    }

    sea_bed
        .into_iter()
        .flat_map(|column| column.into_iter())
        .filter(|&intersecting_lines| intersecting_lines > 1)
        .count()
}

fn main() {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();

    println!("{}", get_overlapping_orthogonal_lines(parse_lines(&input)));
    println!("{}", get_overlapping_lines(parse_lines(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
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
    fn test_parse_line() {
        assert_eq!(
            parse_line("8,0 -> 0,8"),
            Line {
                start: Point { x: 8, y: 0 },
                end: Point { x: 0, y: 8 }
            }
        )
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines("8,0 -> 0,8\n8,0 -> 0,8"),
            vec![
                Line {
                    start: Point { x: 8, y: 0 },
                    end: Point { x: 0, y: 8 }
                },
                Line {
                    start: Point { x: 8, y: 0 },
                    end: Point { x: 0, y: 8 }
                }
            ]
        );
    }

    #[test]
    fn test_furthest_corner() {
        assert_eq!(
            get_furthest_corner(&parse_lines(TEST_INPUT)),
            Point { x: 9, y: 9 }
        );
    }

    #[test]
    fn test_expected_overlaps() {
        assert_eq!(get_overlapping_orthogonal_lines(parse_lines(TEST_INPUT)), 5);
        assert_eq!(get_overlapping_lines(parse_lines(TEST_INPUT)), 12);
    }
}
