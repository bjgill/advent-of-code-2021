#[derive(Debug, PartialEq)]
struct Line {
    signal: Vec<String>,
    output: Vec<String>,
}

impl From<String> for Line {
    fn from(input: String) -> Line {
        let (signal, output) = input.split_once(" | ").unwrap();

        Line {
            signal: signal.split_whitespace().map(String::from).collect(),
            output: output.split_whitespace().map(String::from).collect(),
        }
    }
}

impl Line {
    fn count_1478(&self) -> usize {
        self.output
            .iter()
            .filter(|o| [2, 4, 3, 7].contains(&o.len()))
            .count()
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day8.txt").unwrap();

    let lines: Vec<Line> = data.split('\n').map(String::from).map(Line::from).collect();

    println!(
        "{} 1478s",
        lines.iter().map(Line::count_1478).sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_from_string() {
        assert_eq!(
            Line::from("be cfbegad | fdgacbe cefdb".to_string()),
            Line {
                signal: vec!["be".to_string(), "cfbegad".to_string()],
                output: vec!["fdgacbe".to_string(), "cefdb".to_string()]
            },
        )
    }

    #[test]
    fn test_counting_1478() {
        assert_eq!(Line::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string()).count_1478(), 2);
    }

    #[test]
    fn test_count_input_1478() {
        let data = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |        fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |        fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |        cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |        efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |        gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |        gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |        cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |        ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |        gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |        fgae cfgab fg bagce";

        let lines: Vec<Line> = data
            .split('\n')
            .map(String::from)
            .map(Line::from)
            .inspect(|l| eprintln!("{:?}", l))
            .collect();

        assert_eq!(lines.iter().map(Line::count_1478).sum::<usize>(), 26);
    }
}
