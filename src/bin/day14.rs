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
}

type Pair = (char, char);

#[derive(Debug, PartialEq)]
struct BulkPolymer(HashMap<Pair, u64>);

impl From<String> for BulkPolymer {
    fn from(input: String) -> BulkPolymer {
        let mut counter = HashMap::new();

        input
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .for_each(|window| {
                *counter
                    .entry((
                        window.first().unwrap().to_owned(),
                        window.last().unwrap().to_owned(),
                    ))
                    .or_insert(0) += 1
            });

        BulkPolymer(counter)
    }
}

impl BulkPolymer {
    fn step(&mut self, rules: &PolymerisationRules) {
        let mut new_counter = HashMap::new();

        for (pair, count) in &self.0 {
            let middle = rules.apply(pair.0, pair.1);

            *new_counter.entry((pair.0, middle)).or_insert(0) += count;
            *new_counter.entry((middle, pair.1)).or_insert(0) += count;
        }

        self.0 = new_counter;
    }

    fn step_times(&mut self, rules: &PolymerisationRules, times: u32) {
        for _ in 0..times {
            self.step(rules);
        }
    }

    fn count_elements(&self) {
        let mut char_counter = HashMap::new();

        for ((a, b), count) in &self.0 {
            *char_counter.entry(a).or_insert(0) += count;
            *char_counter.entry(b).or_insert(0) += count;
        }

        let min = char_counter.iter().min_by_key(|&(_, count)| count).unwrap();
        let max = char_counter.iter().max_by_key(|&(_, count)| count).unwrap();

        // We're technically double-counting the first and last characters in the string. However, thanks to rounding during the division, it all washes out.
        println!("Min: {} x {}", min.0, min.1 / 2);
        println!("max: {} x {}", max.0, max.1 / 2);
        println!("Difference: {}", max.1 / 2 - min.1 / 2);
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day14.txt").unwrap();
    let (polymer, rules) = data.split_once("\n\n").unwrap();

    let mut polymer = BulkPolymer::from(polymer.to_string());
    let rules = PolymerisationRules::from(rules.to_string());

    polymer.step_times(&rules, 10);
    polymer.count_elements();

    polymer.step_times(&rules, 30);
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

    #[test]
    fn test_parse_bulk_polymer() {
        let mut expected_polymer = HashMap::new();
        expected_polymer.insert(('a', 'b'), 1);
        expected_polymer.insert(('b', 'c'), 1);

        assert_eq!(
            BulkPolymer::from("abc".to_string()),
            BulkPolymer(expected_polymer)
        );
    }
}
