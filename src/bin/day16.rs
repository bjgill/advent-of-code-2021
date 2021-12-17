use nom::{
    bytes::complete::{tag, take},
    combinator::{cond, eof, fail},
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
    contents: MessageContents,
}

impl Message {
    fn get_version_sum(&self) -> u64 {
        self.version as u64
            + match &self.contents {
                MessageContents::Literal(_) => 0,
                MessageContents::Sum(subpackets)
                | MessageContents::Product(subpackets)
                | MessageContents::Minimum(subpackets)
                | MessageContents::Maximum(subpackets) => {
                    subpackets.iter().map(|s| s.get_version_sum()).sum()
                }
                MessageContents::GreaterThan(message1, message2)
                | MessageContents::LessThan(message1, message2)
                | MessageContents::EqualTo(message1, message2) => {
                    message1.get_version_sum() + message2.get_version_sum()
                }
            }
    }

    fn get_value(&self) -> u64 {
        match &self.contents {
            MessageContents::Sum(subpackets) => subpackets.iter().map(|s| s.get_value()).sum(),
            MessageContents::Product(subpackets) => {
                subpackets.iter().map(|s| s.get_value()).product()
            }
            MessageContents::Minimum(subpackets) => {
                subpackets.iter().map(|s| s.get_value()).min().unwrap()
            }
            MessageContents::Maximum(subpackets) => {
                subpackets.iter().map(|s| s.get_value()).max().unwrap()
            }
            MessageContents::Literal(l) => *l,
            MessageContents::GreaterThan(packet1, packet2) => {
                if packet1.get_value() > packet2.get_value() {
                    1
                } else {
                    0
                }
            }
            MessageContents::LessThan(packet1, packet2) => {
                if packet1.get_value() < packet2.get_value() {
                    1
                } else {
                    0
                }
            }
            MessageContents::EqualTo(packet1, packet2) => {
                if packet1.get_value() == packet2.get_value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MessageContents {
    Sum(Vec<Message>),
    Product(Vec<Message>),
    Minimum(Vec<Message>),
    Maximum(Vec<Message>),
    Literal(u64),
    GreaterThan(Box<Message>, Box<Message>),
    LessThan(Box<Message>, Box<Message>),
    EqualTo(Box<Message>, Box<Message>),
}

fn take_version(s: &str) -> IResult<&str, u8> {
    let (input, version) = take(3usize)(s)?;

    Ok((input, u8::from_str_radix(version, 2).unwrap()))
}

fn take_package_type(s: &str) -> IResult<&str, u8> {
    take(3usize)(s).map(|(i, p)| (i, u8::from_str_radix(p, 2).unwrap()))
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

    let (input, contents) = match package_type {
        0 => {
            let (input, subpackets) = take_subpackets(input)?;
            (input, MessageContents::Sum(subpackets))
        }
        1 => {
            let (input, subpackets) = take_subpackets(input)?;
            (input, MessageContents::Product(subpackets))
        }
        2 => {
            let (input, subpackets) = take_subpackets(input)?;
            (input, MessageContents::Minimum(subpackets))
        }
        3 => {
            let (input, subpackets) = take_subpackets(input)?;
            (input, MessageContents::Maximum(subpackets))
        }
        4 => {
            let (input, literal) = take_literal(input)?;
            (input, MessageContents::Literal(literal))
        }
        5 => {
            let (input, subpackets) = take_subpackets(input)?;
            (
                input,
                MessageContents::GreaterThan(
                    Box::new(subpackets[0].clone()),
                    Box::new(subpackets[1].clone()),
                ),
            )
        }
        6 => {
            let (input, subpackets) = take_subpackets(input)?;
            (
                input,
                MessageContents::LessThan(
                    Box::new(subpackets[0].clone()),
                    Box::new(subpackets[1].clone()),
                ),
            )
        }
        7 => {
            let (input, subpackets) = take_subpackets(input)?;
            (
                input,
                MessageContents::EqualTo(
                    Box::new(subpackets[0].clone()),
                    Box::new(subpackets[1].clone()),
                ),
            )
        }
        _ => fail(input)?,
    };

    Ok((input, Message { version, contents }))
}

fn main() {
    let data = std::fs::read_to_string("data/day16.txt").unwrap();
    let message_string = convert_to_bits(data);
    let message = parse_message(&message_string).unwrap().1;

    println!("Version sum: {}", message.get_version_sum());
    println!("Value: {}", message.get_value());
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Message {
        fn get_as_literal(&self) -> Option<u64> {
            match self.contents {
                MessageContents::Literal(l) => Some(l),
                _ => None,
            }
        }
        fn get_message_type(&self) -> u8 {
            match &self.contents {
                MessageContents::Sum(_) => 0,
                MessageContents::Product(_) => 1,
                MessageContents::Minimum(_) => 2,
                MessageContents::Maximum(_) => 3,
                MessageContents::Literal(_) => 4,
                MessageContents::GreaterThan(_, _) => 5,
                MessageContents::LessThan(_, _) => 6,
                MessageContents::EqualTo(_, _) => 7,
            }
        }

        fn get_subpackets(&self) -> Option<Vec<Message>> {
            match &self.contents {
                MessageContents::Sum(subpackets)
                | MessageContents::Product(subpackets)
                | MessageContents::Minimum(subpackets)
                | MessageContents::Maximum(subpackets) => Some(subpackets.clone()),
                MessageContents::GreaterThan(message1, message2)
                | MessageContents::LessThan(message1, message2)
                | MessageContents::EqualTo(message1, message2) => {
                    Some(vec![*message1.clone(), *message2.clone()])
                }
                MessageContents::Literal(_) => None,
            }
        }
    }

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
        assert_eq!(message.get_message_type(), 6);
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
        assert_eq!(message.get_message_type(), 3);
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

    #[test]
    fn test_value() {
        assert_eq!(
            parse_message(&convert_to_bits("C200B40A82"))
                .unwrap()
                .1
                .get_value(),
            3
        );
        assert_eq!(
            parse_message(&convert_to_bits("04005AC33890"))
                .unwrap()
                .1
                .get_value(),
            54
        );
        assert_eq!(
            parse_message(&convert_to_bits("9C0141080250320F1802104A08"))
                .unwrap()
                .1
                .get_value(),
            1
        );
    }
}
