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

fn parse_stack_fragment(line: &str) -> Result<Vec<Bracket>, Bracket> {
    let mut stack = Vec::new();

    for c in line.chars() {
        let bracket = Bracket::from(c);

        if bracket.is_left() {
            stack.push(bracket);
        } else {
            if !stack.pop().unwrap().closed_by(&bracket) {
                return Err(bracket);
            }
        }
    }

    Ok(stack)
}

fn get_syntax_error_cost(line: &str) -> u32 {
    match parse_stack_fragment(line) {
        Err(Bracket::RightRound) => 3,
        Err(Bracket::RightSquare) => 57,
        Err(Bracket::RightBrace) => 1197,
        Err(Bracket::RightAngle) => 25137,
        _ => 0,
    }
}

fn get_completion_cost(line: &str) -> Result<u64, Bracket> {
    Ok(parse_stack_fragment(line)?
        .into_iter()
        .rev()
        .fold(0, |acc, bracket| {
            acc * 5
                + match bracket {
                    Bracket::LeftRound => 1,
                    Bracket::LeftSquare => 2,
                    Bracket::LeftBrace => 3,
                    Bracket::LeftAngle => 4,
                    _ => panic!("Unexpected bracket in stack remnant"),
                }
        }))
}

fn main() {
    let data = std::fs::read_to_string("data/day10.txt").unwrap();

    println!(
        "Syntax error cost: {}",
        data.split('\n')
            .map(|line| get_syntax_error_cost(line))
            .sum::<u32>()
    );

    let mut completions: Vec<u64> = data
        .split('\n')
        .map(|line| get_completion_cost(line))
        .filter_map(Result::ok)
        .collect();
    completions.sort();

    println!("{:?}", completions[completions.len() / 2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_stack_fragment("{([(<{}[<>[]}>{[]{[(<()>"),
            Err(Bracket::RightBrace)
        );

        assert_eq!(parse_stack_fragment("{}()<<>>"), Ok(vec![]));
        assert_eq!(
            parse_stack_fragment("{}()<<"),
            Ok(vec![Bracket::LeftAngle, Bracket::LeftAngle])
        );
    }

    #[test]
    fn test_completion_cost() {
        assert_eq!(get_completion_cost("[({(<(())[]>[[{[]{<()<>>"), Ok(288957));

        assert_eq!(get_completion_cost("<{([{{}}[<[[[<>{}]]]>[]]"), Ok(294));
    }
}
