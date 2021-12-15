use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^(.)(.) -> (.)$").unwrap();
}

#[derive(Debug, PartialEq)]
struct PolymerisationRules(HashMap<(char, char), char>);

impl From<String> for PolymerisationRules {
    fn from(input: String) -> PolymerisationRules {
        PolymerisationRules(
            input
                .split('\n')
                .map(|s| {
                    let mut chars = s.chars();

                    let a = chars.next().unwrap();
                    let b = chars.next().unwrap();

                    ((a, b), chars.last().unwrap())
                })
                .collect(),
        )
    }
}

impl PolymerisationRules {
    fn apply(&self, a: char, b: char) -> char {
        self.0.get(&(a, b)).unwrap().clone()
    }
}

#[derive(Debug, PartialEq)]
struct Polymer(Vec<char>);

impl Polymer {
    fn step(&mut self, rules: &PolymerisationRules) {
        for i in 0..self.0.len() - 1 {
            let a = self.0[2 * i];
            let b = self.0[2 * i + 1];

            self.0.insert(2 * i + 1, rules.apply(a, b));
        }
    }

    fn step_times(&mut self, rules: &PolymerisationRules, times: u32) {
        for _ in 0..times {
            self.step(&rules);
        }
    }

    /// It's easiest to do this manually by inspection.
    fn count_elements(&self) {
        let mut counter = HashMap::new();

        self.0.iter().for_each(|c| {*counter.entry(c).or_insert(0) += 1;});

        println!("Element counts: {:?}", counter);
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day14.txt").unwrap();
    let (start, rules) = data.split_once("\n\n").unwrap();

    let mut polymer = Polymer(start.chars().collect());
    let rules = PolymerisationRules::from(rules.to_string());

    polymer.step_times(&rules, 10);

    polymer.count_elements();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules() {
        let mut expected_rules = HashMap::new();
        expected_rules.insert(('a', 'b'), 'c');
        expected_rules.insert(('c', 'a'), 'b');

        assert_eq!(
            PolymerisationRules::from("ab -> c\nca -> b".to_string()),
            PolymerisationRules(expected_rules)
        );
    }

    #[test]
    fn test_example() {
        let rules = PolymerisationRules::from(
            "CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
                .to_string(),
        );
        let mut polymer = Polymer("NNCB".chars().collect());

        polymer.step(&rules);
        assert_eq!(polymer, Polymer("NCNBCHB".chars().collect()));

        polymer.step(&rules);
        assert_eq!(polymer, Polymer("NBCCNBBBCBHCB".chars().collect()));
    }
}
