use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    error::Error,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
enum Element {
    Pair {
        left: Box<Element>,
        right: Box<Element>,
    },
    Number(u32),
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Pair { left, right } => f.debug_list().entry(left).entry(right).finish(),
            Element::Number(number) => f.write_fmt(format_args!("{}", number)),
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, Element> {
    digit1(input).map(|(i, number)| (i, Element::Number(number.parse().unwrap())))
}

fn parse_pair(input: &str) -> IResult<&str, Element> {
    delimited(
        tag("["),
        separated_pair(parse_element, tag(","), parse_element),
        tag("]"),
    )(input)
    .map(|(i, (left, right))| {
        (
            i,
            Element::Pair {
                left: Box::new(left),
                right: Box::new(right),
            },
        )
    })
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    alt((parse_number, parse_pair))(input)
}

/// Taken from the nom example: https://docs.rs/nom/latest/nom/recipes/index.html#implementing-fromstr
impl FromStr for Element {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_element(s).finish() {
            Ok((_remaining, element)) => Ok(element),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl std::ops::Add for Element {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut element = Element::Pair {
            left: Box::new(self),
            right: Box::new(other),
        };

        loop {
            if element.explode_if_necessary().is_none() && element.split().is_none() {
                break;
            }
        }

        element
    }
}

impl Element {
    fn explode_if_necessary(&mut self) -> Option<()> {
        self.explode_if_nested_four_times(0).map(|_| ())
    }

    fn increase_left_most_value_by(&mut self, increase: u32) {
        match self {
            Element::Pair { left, right: _ } => {
                left.increase_left_most_value_by(increase);
            }
            Element::Number(number) => {
                *number += increase;
            }
        }
    }

    fn increase_right_most_value_by(&mut self, increase: u32) {
        match self {
            Element::Pair { left: _, right } => {
                right.increase_right_most_value_by(increase);
            }
            Element::Number(number) => {
                *number += increase;
            }
        }
    }

    fn get_magnitude(&self) -> u32 {
        match self {
            Element::Pair { left, right } => 3 * left.get_magnitude() + 2 * right.get_magnitude(),
            Element::Number(number) => *number,
        }
    }

    fn explode_if_nested_four_times(&mut self, nesting: u32) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            Element::Pair { left, right } if nesting == 4 => {
                // explode
                if let (Element::Number(left_value), Element::Number(right_value)) =
                    (*left.clone(), *right.clone())
                {
                    *self = Element::Number(0);
                    Some((Some(left_value), Some(right_value)))
                } else {
                    panic!("Invalid: nested more than 4 times")
                }
            }
            Element::Pair { left, right } => {
                if let Some((explode_left, explode_right)) =
                    left.explode_if_nested_four_times(nesting + 1)
                {
                    if let Some(explode_right_value) = explode_right {
                        right.increase_left_most_value_by(explode_right_value);
                    }
                    Some((explode_left, None))
                } else if let Some((explode_left, explode_right)) =
                    right.explode_if_nested_four_times(nesting + 1)
                {
                    if let Some(explode_left_value) = explode_left {
                        left.increase_right_most_value_by(explode_left_value);
                    }
                    Some((None, explode_right))
                } else {
                    None
                }
            }
            Element::Number(_) => None,
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            Element::Pair { left, right } => left.split().or_else(|| right.split()),
            Element::Number(number) if *number >= 10 => {
                *self = Element::Pair {
                    left: Box::new(Element::Number((*number) / 2)),
                    right: Box::new(Element::Number((*number + 1) / 2)),
                };

                Some(())
            }
            Element::Number(_) => None,
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day18.txt").unwrap();
    let mut elements = data.split('\n').map(|s| s.parse::<Element>().unwrap());
    let first_element = elements.next().unwrap();

    println!(
        "Magnitude of sum: {}",
        elements
            .fold(first_element, |acc, e| acc + e)
            .get_magnitude()
    );

    let element_pairs = data
        .split('\n')
        .map(|s| s.parse::<Element>().unwrap())
        .permutations(2);
    println!(
        "Maximum magnitude: {:?}",
        element_pairs
            .map(|v| (v[0].clone() + v[1].clone()).get_magnitude())
            .max()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!("1".parse(), Ok(Element::Number(1)));
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(
            "[1,2]".parse(),
            Ok(Element::Pair {
                left: Box::new(Element::Number(1)),
                right: Box::new(Element::Number(2))
            })
        );
    }

    #[test]
    fn test_parse_nested_pair() {
        assert_eq!(
            "[[1,2],3]".parse(),
            Ok(Element::Pair {
                left: Box::new(Element::Pair {
                    left: Box::new(Element::Number(1)),
                    right: Box::new(Element::Number(2))
                }),
                right: Box::new(Element::Number(3))
            })
        );
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            "[1,2]".parse::<Element>().unwrap() + "3".parse::<Element>().unwrap(),
            Element::Pair {
                left: Box::new(Element::Pair {
                    left: Box::new(Element::Number(1)),
                    right: Box::new(Element::Number(2))
                }),
                right: Box::new(Element::Number(3))
            }
        );
    }

    #[test]
    fn test_exploding() {
        let mut element = "[[[[[9,8],1],2],3],4]".parse::<Element>().unwrap();
        assert_eq!(element.explode_if_necessary(), Some(()));
        assert_eq!(element, "[[[[0,9],2],3],4]".parse().unwrap());

        let mut element = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            .parse::<Element>()
            .unwrap();
        assert_eq!(element.explode_if_necessary(), Some(()));
        assert_eq!(
            element,
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap()
        );
    }

    #[test]
    fn test_split() {
        let mut element = "10".parse::<Element>().unwrap();
        assert_eq!(element.split(), Some(()));
        assert_eq!(element, "[5,5]".parse().unwrap());

        let mut element = "11".parse::<Element>().unwrap();
        assert_eq!(element.split(), Some(()));
        assert_eq!(element, "[5,6]".parse().unwrap());
    }

    #[test]
    fn test_addition_with_post_processing() {
        assert_eq!(
            "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<Element>().unwrap()
                + "[1,1]".parse::<Element>().unwrap(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<Element>()
                .unwrap()
        );

        assert_eq!(
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"
                .parse::<Element>()
                .unwrap()
                + "[[[[4,2],2],6],[8,7]]".parse::<Element>().unwrap(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<Element>()
                .unwrap()
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<Element>()
                .unwrap()
                .get_magnitude(),
            3488
        );
    }
}
