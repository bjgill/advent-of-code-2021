use nom::{
    bytes::complete::{tag, take},
    combinator::{cond, eof},
    multi::{many1, many_m_n, many_till},
    sequence::preceded,
    sequence::tuple,
    IResult,
};

fn convert_to_bits<S: Into<String>>(input: S) -> String {
    input
        .into()
        .chars()
        .map(|c| format!("{:0>4b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Debug, PartialEq, Clone)]
struct Message {
    version: u8,
    package_type: MessageType,
    contents: MessageContents,
}

impl Message {
    fn get_as_literal(&self) -> Option<u64> {
        match self.contents {
            MessageContents::Literal(l) => Some(l),
            _ => None,
        }
    }

    fn get_subpackets(&self) -> Option<Vec<Message>> {
        match &self.contents {
            MessageContents::SubPackets(s) => Some(s.clone()),
            _ => None,
        }
    }

    fn get_version_sum(&self) -> u64 {
        self.version as u64
            + match &self.contents {
                MessageContents::Literal(_) => 0,
                MessageContents::SubPackets(subpackets) => {
                    subpackets.iter().map(|s| s.get_version_sum()).sum()
                }
            }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MessageType {
    Literal,
    Operator(u8),
}

#[derive(Debug, PartialEq, Clone)]
enum MessageContents {
    Literal(u64),
    SubPackets(Vec<Message>),
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

fn take_literal(s: &str) -> IResult<&str, u64> {
    let (input, (start, end)) = many_till(take_literal_start, take_literal_end)(s)?;

    let literal = u64::from_str_radix(&(start.join("") + end), 2).unwrap();

    Ok((input, literal))
}

fn take_subpackets(s: &str) -> IResult<&str, Vec<Message>> {
    let (input, length_type) = take(1usize)(s)?;

    let (input, length) = cond(length_type == "0", take(15usize))(input)?;

    match length {
        Some(l) => {
            let length = usize::from_str_radix(l, 2).unwrap();

            let (input, subpackets) = take(length)(input)?;

            let (remainder, subpackets) = many1(parse_message)(subpackets)?;

            let (_, _) = eof(remainder)?;

            Ok((input, subpackets))
        }
        None => {
            let (input, subpacket_count) = take(11usize)(input)?;

            let subpacket_count = usize::from_str_radix(subpacket_count, 2).unwrap();

            let (input, subpackets) = take_n_subpackets(subpacket_count, input)?;

            Ok((input, subpackets))
        }
    }
}

fn take_n_subpackets(n: usize, s: &str) -> IResult<&str, Vec<Message>> {
    many_m_n(n, n, parse_message)(s)
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
            let (input, length) = take_subpackets(input)?;

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

fn main() {
    let data = std::fs::read_to_string("data/day16.txt").unwrap();
    let message_string = convert_to_bits(data);

    println!(
        "Version sum: {}",
        parse_message(&message_string).unwrap().1.get_version_sum()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_bits() {
        assert_eq!(convert_to_bits("9A"), "10011010");
        assert_eq!(
            convert_to_bits("EE00D40C823060"),
            "11101110000000001101010000001100100000100011000001100000"
        );
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
        let message = parse_message("00111000000000000110111101000101001010010001001000000000")
            .unwrap()
            .1;

        assert_eq!(message.version, 1);
        assert_eq!(message.package_type, MessageType::Operator(6));
        assert_eq!(
            message.get_subpackets().unwrap()[0].get_as_literal(),
            Some(10)
        );
        assert_eq!(
            message.get_subpackets().unwrap()[1].get_as_literal(),
            Some(20)
        );
        assert_eq!(message.get_subpackets().unwrap().len(), 2);
    }

    #[test]
    fn test_parse_operator_subpacket_count() {
        let message = parse_message("11101110000000001101010000001100100000100011000001100000")
            .unwrap()
            .1;

        assert_eq!(message.version, 7);
        assert_eq!(message.package_type, MessageType::Operator(3));
        assert_eq!(
            message.get_subpackets().unwrap()[0].get_as_literal(),
            Some(1)
        );
        assert_eq!(
            message.get_subpackets().unwrap()[1].get_as_literal(),
            Some(2)
        );
        assert_eq!(
            message.get_subpackets().unwrap()[2].get_as_literal(),
            Some(3)
        );
        assert_eq!(message.get_subpackets().unwrap().len(), 3);
    }

    #[test]
    fn test_parse_nested_message() {
        let message = parse_message(&convert_to_bits("8A004A801A8002F478"))
            .unwrap()
            .1;

        assert_eq!(message.version, 4);
        assert_eq!(message.get_subpackets().unwrap()[0].version, 1);
        assert_eq!(
            message.get_subpackets().unwrap()[0]
                .get_subpackets()
                .unwrap()[0]
                .version,
            5
        );
        assert_eq!(
            message.get_subpackets().unwrap()[0]
                .get_subpackets()
                .unwrap()[0]
                .get_subpackets()
                .unwrap()[0]
                .version,
            6
        );
        assert_eq!(
            message.get_subpackets().unwrap()[0]
                .get_subpackets()
                .unwrap()[0]
                .get_subpackets()
                .unwrap()[0]
                .get_as_literal()
                .is_some(),
            true
        );
        assert_eq!(message.get_version_sum(), 16);
    }
}
