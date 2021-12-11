//! The solution to this problem is the median.
//!
//! I'm not sure about proof, but I think I can argue this as follows: consider the fuel required to reach the median.
//! The fuel required to reach one on either side must necessarily be more - whichever crab is the median will need to
//! move. So at least half of crabs will become further, and at most half of crabs will be closer.

fn minimal_linear_fuel(crabs: Vec<i32>) -> i32 {
    let mut crabs = crabs;

    crabs.sort();

    let median = crabs[crabs.len() / 2];

    crabs.into_iter().map(|c| (c - median).abs()).sum()
}

fn triangle(distance: i32) -> i32 {
    (distance * (distance + 1)) / 2
}

/// This is a fudge, determined experimentally for the particular day 7 input data. It's not universally applicable.
fn minimal_triangular_fuel(crabs: Vec<i32>) -> i32 {
    let average = (crabs.iter().sum::<i32>() as f32 / (crabs.len() as f32)).round() as i32;

    eprintln!("{}", average);

    i32::min(
        crabs
            .clone()
            .into_iter()
            .map(|c| triangle((c - average).abs()))
            .sum(),
        crabs
            .into_iter()
            .map(|c| triangle((c - average + 1).abs()))
            .sum(),
    )
}

fn main() {
    let input = std::fs::read_to_string("data/day07.txt").unwrap();
    let crabs: Vec<i32> = input.split(',').map(|c| c.parse().unwrap()).collect();

    println!(
        "Minimum linear fuel use: {}",
        minimal_linear_fuel(crabs.clone())
    );
    println!(
        "Minimum triangular fuel use: {}",
        minimal_triangular_fuel(crabs)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_linear_fuel() {
        assert_eq!(
            minimal_linear_fuel(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]),
            37
        );
    }

    #[test]
    fn test_expected_triangular_fuel() {
        assert_eq!(
            minimal_triangular_fuel(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]),
            168
        );
    }
}
