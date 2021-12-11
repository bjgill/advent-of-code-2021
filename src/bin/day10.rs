#[derive(Debug, PartialEq)]
enum Bracket {
    LeftRound,
    RightRound,
    LeftSquare,
    RightSquare,
    LeftBrace,
    RightBrace,
    LeftAngle,
    RightAngle,
}

impl Bracket {
    fn is_left(&self) -> bool {
        self == &Bracket::LeftRound
            || self == &Bracket::LeftSquare
            || self == &Bracket::LeftBrace
            || self == &Bracket::LeftAngle
    }

    fn closed_by(&self, close_candidate: &Bracket) -> bool {
        match (self, close_candidate) {
            (Bracket::LeftRound, Bracket::RightRound)
            | (Bracket::LeftSquare, Bracket::RightSquare)
            | (Bracket::LeftBrace, Bracket::RightBrace)
            | (Bracket::LeftAngle, Bracket::RightAngle) => true,
            _ => false,
        }
    }
}

impl From<char> for Bracket {
    fn from(input: char) -> Bracket {
        match input {
            '(' => Bracket::LeftRound,
            '[' => Bracket::LeftSquare,
            '{' => Bracket::LeftBrace,
            '<' => Bracket::LeftAngle,

            ')' => Bracket::RightRound,
            ']' => Bracket::RightSquare,
            '}' => Bracket::RightBrace,
            '>' => Bracket::RightAngle,

            i => panic!("Invalid char: {}", i),
        }
    }
}

fn find_syntax_error(line: &str) -> Option<Bracket> {
    let mut stack = Vec::new();

    for c in line.chars() {
        let bracket = Bracket::from(c);

        if bracket.is_left() {
            stack.push(bracket);
        } else {
            if !stack.pop().unwrap().closed_by(&bracket) {
                return Some(bracket);
            }
        }
    }

    None
}

fn get_syntax_error_cost(line: &str) -> u32 {
    match find_syntax_error(line) {
        Some(Bracket::RightRound) => 3,
        Some(Bracket::RightSquare) => 57,
        Some(Bracket::RightBrace) => 1197,
        Some(Bracket::RightAngle) => 25137,
        _ => 0,
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day10.txt").unwrap();

    println!(
        "Syntax error cost: {}",
        data.split('\n')
            .map(|line| get_syntax_error_cost(line))
            .sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            find_syntax_error("{([(<{}[<>[]}>{[]{[(<()>"),
            Some(Bracket::RightBrace)
        );

        assert_eq!(find_syntax_error("{}()<<>>"), None);
    }
}
