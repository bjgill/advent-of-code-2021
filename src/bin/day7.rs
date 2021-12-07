//! The solution to this problem is the median.
//!
//! I'm not sure about proof, but I think I can argue this as follows: consider the fuel required to reach the median.
//! The fuel required to reach one on either side must necessarily be more - whichever crab is the median will need to
//! move. So at least half of crabs will become further, and at most half of crabs will be closer.

fn fuel_use(crabs: Vec<i32>) -> i32 {
    let mut crabs = crabs;

    crabs.sort();

    let median = crabs[crabs.len()/2];

    crabs.into_iter().map(|c| (c - median).abs()).sum()
}

fn main() {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();
    let crabs = input.split(',').map(|c| c.parse().unwrap()).collect();

    println!("Minimum fuel use: {}", fuel_use(crabs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_fuel() {
        assert_eq!(fuel_use(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
    }
}
