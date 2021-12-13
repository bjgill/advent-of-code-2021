use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl From<String> for Point {
    fn from(input: String) -> Point {
        let (x, y) = input.split_once(',').unwrap();

        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Paper(BTreeSet<Point>);

impl Paper {
    fn get_visible_dots(&self) -> usize {
        self.0.len()
    }

    fn fold_with(self, fold: Fold) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|mut point| {
                    match fold {
                        Fold::X(fold_index) => {
                            if point.x > fold_index {
                                point.x = 2 * fold_index - point.x;
                            }
                        }
                        Fold::Y(fold_index) => {
                            if point.y > fold_index {
                                point.y = 2 * fold_index - point.y;
                            }
                        }
                    }

                    point
                })
                .collect(),
        )
    }
}

#[derive(PartialEq, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl From<String> for Fold {
    fn from(input: String) -> Fold {
        let (direction, position) = input
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();

        if direction == "x" {
            Fold::X(position.parse().unwrap())
        } else if direction == "y" {
            Fold::Y(position.parse().unwrap())
        } else {
            panic!("Unable to parse as fold: {}", input)
        }
    }
}

fn parse_input(input: &str) -> (Paper, Vec<Fold>) {
    let (paper, folds) = input.split_once("\n\n").unwrap();

    (
        Paper(
            paper
                .split('\n')
                .map(|s| Point::from(s.to_string()))
                .collect(),
        ),
        folds
            .split('\n')
            .map(|s| Fold::from(s.to_string()))
            .collect(),
    )
}

fn main() {
    let data = std::fs::read_to_string("data/day13.txt").unwrap();
    let (mut paper, mut folds) = parse_input(&data);

    paper = paper.fold_with(folds.remove(0));
    println!("{} visible dots after 1 fold", paper.get_visible_dots());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input("6,10\n\nfold along y=7"),
            (
                Paper(vec![Point { x: 6, y: 10 }].into_iter().collect()),
                vec![Fold::Y(7)]
            )
        );
    }

    #[test]
    fn test_example() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        let (mut paper, mut folds) = parse_input(input);

        assert_eq!(paper.get_visible_dots(), 18);

        paper = paper.fold_with(folds.remove(0));
        assert_eq!(paper.get_visible_dots(), 17);
    }
}
