use std::collections::{BTreeSet, HashMap};

#[derive(Debug, PartialEq)]
struct Line {
    signal: Vec<BTreeSet<char>>,
    output: Vec<BTreeSet<char>>,
}

impl From<String> for Line {
    fn from(input: String) -> Line {
        let (signal, output) = input.split_once(" | ").unwrap();

        Line {
            signal: signal
                .split_whitespace()
                .map(|s| s.chars().collect())
                .collect(),
            output: output
                .split_whitespace()
                .map(|s| s.chars().collect())
                .collect(),
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

    fn decrypt_signal(&self) -> u32 {
        let one = self.signal.iter().filter(|s| s.len() == 2).next().unwrap();
        let four = self.signal.iter().filter(|s| s.len() == 4).next().unwrap();
        let seven = self.signal.iter().filter(|s| s.len() == 3).next().unwrap();
        let eight = self.signal.iter().filter(|s| s.len() == 7).next().unwrap();

        let three = self
            .signal
            .iter()
            .filter(|s| s.len() == 5 && s.intersection(one).count() == 2)
            .next()
            .unwrap();
        let nine = self
            .signal
            .iter()
            .filter(|s| {
                s.len() == 6
                    && s.intersection(one).count() == 2
                    && s.intersection(four).count() == 4
            })
            .next()
            .unwrap();

        let six = self
            .signal
            .iter()
            .filter(|s| s.len() == 6 && s.intersection(one).count() == 1 && s != &nine)
            .next()
            .unwrap();

        let five = self
            .signal
            .iter()
            .filter(|s| s.len() == 5 && s != &three && s.intersection(six).count() == 5)
            .next()
            .unwrap();

        let two = self
            .signal
            .iter()
            .filter(|s| s.len() == 5 && s != &three && s != &five)
            .next()
            .unwrap();
        let zero = self
            .signal
            .iter()
            .filter(|s| s.len() == 6 && s != &six && s != &nine)
            .next()
            .unwrap();

        let mut mapping = HashMap::new();
        mapping.insert(zero, 0);
        mapping.insert(one, 1);
        mapping.insert(two, 2);
        mapping.insert(three, 3);
        mapping.insert(four, 4);
        mapping.insert(five, 5);
        mapping.insert(six, 6);
        mapping.insert(seven, 7);
        mapping.insert(eight, 8);
        mapping.insert(nine, 9);
        let mapping = mapping;

        mapping[&self.output[0]] * 1000
            + mapping[&self.output[1]] * 100
            + mapping[&self.output[2]] * 10
            + mapping[&self.output[3]]
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day8.txt").unwrap();

    let lines: Vec<Line> = data.split('\n').map(String::from).map(Line::from).collect();

    println!(
        "{} 1478s",
        lines.iter().map(Line::count_1478).sum::<usize>()
    );

    println!(
        "{} sum of outputs",
        lines.iter().map(Line::decrypt_signal).sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_mapping() {
        assert_eq!(Line::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string()).decrypt_signal(), 5353);
    }
}
