use nom::{
    bytes::complete::{tag, take},
    combinator::cond,
    error::{Error, ErrorKind},
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
    package_type: MessageType,
    contents: MessageContents,
}

#[derive(Debug, PartialEq)]
enum MessageType {
    Literal,
    Operator(u8),
}

#[derive(Debug, PartialEq)]
enum MessageContents {
    Literal(u32),
    SubPackets(Length),
}

fn take_version(s: &str) -> IResult<&str, u8> {
    let (input, version) = take(3usize)(s)?;

    Ok((input, u8::from_str_radix(version, 2).unwrap()))
}

fn take_package_type(s: &str) -> IResult<&str, MessageType> {
    let (input, package_type) = take(3usize)(s)?;

    Ok((
        input,
        match package_type {
            "100" => MessageType::Literal,
            t => MessageType::Operator(u8::from_str_radix(t, 2).unwrap()),
        },
    ))
}

fn take_literal_start(s: &str) -> IResult<&str, &str> {
    preceded(tag("1"), take(4usize))(s)
}

fn take_literal_end(s: &str) -> IResult<&str, &str> {
    preceded(tag("0"), take(4usize))(s)
}

fn take_literal(s: &str) -> IResult<&str, u32> {
    let (input, (start, end)) = many_till(take_literal_start, take_literal_end)(s)?;

    let literal = u32::from_str_radix(&(start.join("") + end), 2).unwrap();

    Ok((input, literal))
}

#[derive(Debug, PartialEq)]
enum Length {
    TotalLength(usize),
    SubPacketCount(usize),
}

fn take_length(s: &str) -> IResult<&str, Length> {
    let (input, length_type) = take(1usize)(s)?;

    let (input, length) = cond(length_type == "0", take(15usize))(input)?;

    match length {
        Some(l) => {
            let length = usize::from_str_radix(l, 2).unwrap();

            let (input, _subpackets) = take(length)(input)?;

            Ok((input, Length::TotalLength(length)))
        }
        None => {
            let (input, subpacket_count) = take(11usize)(input)?;

            Ok((
                input,
                Length::SubPacketCount(usize::from_str_radix(subpacket_count, 2).unwrap()),
            ))
        }
    }
}

fn parse_message(input: &str) -> IResult<&str, Message> {
    let (input, (version, package_type)) = tuple((take_version, take_package_type))(input)?;

    match package_type {
        MessageType::Literal => {
            let (input, literal) = take_literal(input)?;
            Ok((
                input,
                Message {
                    version,
                    package_type,
                    contents: MessageContents::Literal(literal),
                },
            ))
        }
        MessageType::Operator(o) => {
            let (input, length) = take_length(input)?;

            Ok((
                input,
                Message {
                    version,
                    package_type: MessageType::Operator(o),
                    contents: MessageContents::SubPackets(length),
                },
            ))
        }
    }
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
        assert_eq!(take_version(&convert_to_bits("9A")), Ok(("11010", 4)));
    }

    #[test]
    fn test_parse_literal() {
        assert_eq!(
            parse_message("110100101111111000101000"),
            Ok((
                "000",
                Message {
                    version: 6,
                    package_type: MessageType::Literal,
                    contents: MessageContents::Literal(2021),
                }
            ))
        )
    }

    #[test]
    fn test_parse_operator_total_length() {
        assert_eq!(
            parse_message("00111000000000000110111101000101001010010001001000000000"),
            Ok((
                "0000000",
                Message {
                    version: 1,
                    package_type: MessageType::Operator(6),
                    contents: MessageContents::SubPackets(Length::TotalLength(27)),
                }
            ))
        )
    }

    #[test]
    fn test_parse_operator_subpacket_count() {
        assert_eq!(
            parse_message("11101110000000001101010000001100100000100011000001100000"),
            Ok((
                "01010000001100100000100011000001100000",
                Message {
                    version: 7,
                    package_type: MessageType::Operator(3),
                    contents: MessageContents::SubPackets(Length::SubPacketCount(3)),
                }
            ))
        )
    }
}
