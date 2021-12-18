use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    error::Error,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Element {
    Pair {
        left: Box<Element>,
        right: Box<Element>,
    },
    Number(u32),
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
}
