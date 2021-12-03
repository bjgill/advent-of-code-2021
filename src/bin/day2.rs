use regex::Regex;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

type Position = (i32, i32);

type Instruction = (Direction, i32);

fn parse_route(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"([a-z]+) (\d+)").unwrap();

    let raw_instructions = input.split("\n").map(|s| {
        let captures = re.captures(s).unwrap();

        (
            match &captures[1] {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Invalid direction: {}", s),
            },
            captures[2].parse::<i32>().unwrap(),
        )
    });

    raw_instructions.collect()
}

fn follow_route(instructions: Vec<Instruction>) -> Position {
    let mut position: Position = (0, 0);

    for (direction, distance) in instructions {
        match direction {
            Direction::Forward => position.0 += distance,
            Direction::Down => position.1 += distance,
            Direction::Up => position.1 -= distance,
        }
    }

    position
}

fn follow_aimed_route(instructions: Vec<Instruction>) -> Position {
    let mut position: Position = (0, 0);
    let mut aim = 0;

    for (direction, distance) in instructions {
        match direction {
            Direction::Forward => {
                position.0 += distance;
                position.1 += aim * distance;
            }
            Direction::Down => aim += distance,
            Direction::Up => aim -= distance,
        }
    }

    position
}

fn main() {
    let input = std::fs::read_to_string("data/day2.txt").unwrap();

    let final_position = follow_route(parse_route(&input));
    println!(
        "Final position: {:?} => {}",
        final_position,
        final_position.0 * final_position.1
    );

    let final_aimed_position = follow_aimed_route(parse_route(&input));
    println!(
        "Final aimed position: {:?} => {}",
        final_aimed_position,
        final_aimed_position.0 * final_aimed_position.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_route() {
        assert_eq!(follow_route(vec![]), (0, 0));
        assert_eq!(
            follow_route(vec![(Direction::Forward, 1), (Direction::Down, 2)]),
            (1, 2)
        );
        assert_eq!(
            follow_route(vec![(Direction::Forward, 1), (Direction::Up, 2)]),
            (1, -2)
        );
    }

    #[test]
    fn test_follow_aimed_route() {
        assert_eq!(follow_aimed_route(vec![]), (0, 0));
        assert_eq!(follow_aimed_route(vec![(Direction::Forward, 1)]), (1, 0));
        assert_eq!(follow_aimed_route(vec![(Direction::Up, 1)]), (0, 0));
        assert_eq!(follow_aimed_route(vec![(Direction::Down, 1)]), (0, 0));
        assert_eq!(
            follow_aimed_route(vec![(Direction::Down, 2), (Direction::Forward, 3)]),
            (3, 6)
        );
        assert_eq!(
            follow_aimed_route(vec![(Direction::Up, 2), (Direction::Forward, 3)]),
            (3, -6)
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_route("down 1"), vec![(Direction::Down, 1)]);
    }
}
