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

fn main() {}

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
}
