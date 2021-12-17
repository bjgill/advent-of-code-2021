use nom::{
    bytes::complete::{tag, take},
    multi::many_till,
    sequence::preceded,
    sequence::tuple,
    IResult,
};

fn convert_to_bits<S: Into<String>>(input: S) -> String {
    input
        .into()
        .chars()
        .map(|c| format!("{:b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Debug, PartialEq)]
struct Message {
    version: u8,
    package_type: u8,
    literal: u32,
}

fn take_3(s: &str) -> IResult<&str, &str> {
    take(3usize)(s)
}

fn take_4(s: &str) -> IResult<&str, &str> {
    take(4usize)(s)
}

fn take_literal_start(s: &str) -> IResult<&str, &str> {
    preceded(tag("1"), take_4)(s)
}

fn take_literal_end(s: &str) -> IResult<&str, &str> {
    preceded(tag("0"), take_4)(s)
}

fn take_literal(s: &str) -> IResult<&str, String> {
    let (input, (start, end)) = many_till(take_literal_start, take_literal_end)(s)?;

    let literal = start.join("") + end;

    Ok((input, literal))
}

fn parse_message(input: &str) -> IResult<&str, Message> {
    let (input, (version, package_type, literal)) = tuple((take_3, take_3, take_literal))(input)?;

    Ok((
        input,
        Message {
            version: u8::from_str_radix(version, 2).unwrap(),
            package_type: u8::from_str_radix(package_type, 2).unwrap(),
            literal: u32::from_str_radix(&literal, 2).unwrap(),
        },
    ))
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_bits() {
        assert_eq!(convert_to_bits("9A"), "10011010");
    }

    #[test]
    fn test_get_version() {
        assert_eq!(take_3(&convert_to_bits("9A")), Ok(("11010", "100")));
    }

    #[test]
    fn test_parse_message() {
        assert_eq!(
            parse_message("110100101111111000101000"),
            Ok((
                "000",
                Message {
                    version: 6,
                    package_type: 4,
                    literal: 2021,
                }
            ))
        )
    }
}
