/// Look-up table manually calculated
fn advance_10_days(fish: u32) -> Vec<u32> {
    match fish {
        8 => vec![5, 7],
        7 => vec![4, 6],
        6 => vec![3, 5],
        5 => vec![2, 4],
        4 => vec![1, 3],
        3 => vec![0, 2],
        2 => vec![1, 6, 8],
        1 => vec![0, 5, 7],
        0 => vec![4, 6, 6, 8],
        n => panic!("Fish with invalid number: {}", n),
    }
}

fn advance_double_days(fish: u32, advance_fn: fn(u32) -> Vec<u32>) -> Vec<u32> {
    let mut fish: Vec<_> = advance_fn(fish)
        .into_iter()
        .map(advance_fn)
        .flat_map(|x| x.into_iter())
        .collect();
    fish.sort();
    fish
}

fn advance_80_days(fish: u32) -> Vec<u32> {
    advance_double_days(fish, |fish| {
        advance_double_days(fish, |fish| advance_double_days(fish, advance_10_days))
    })
}

fn parse_fish(fish_input: &str) -> Vec<u32> {
    fish_input.split(',').map(|f| f.parse().unwrap()).collect()
}

fn main() {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    let starting_fish = parse_fish(&input);

    println!("{} starting fish", starting_fish.len());
    println!(
        "{} fish at 80 days",
        starting_fish
            .into_iter()
            .map(advance_80_days)
            .flat_map(|x| x.into_iter())
            .collect::<Vec<_>>()
            .len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn advance_20_days(fish: u32) -> Vec<u32> {
        advance_double_days(fish, advance_10_days)
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_fish("3,4,3,1,2"), vec![3, 4, 3, 1, 2])
    }

    #[test]
    /// Compare to hand-calcuated values.
    fn test_advance_20_days() {
        assert_eq!(advance_20_days(8), vec![2, 4, 4, 6]);
        assert_eq!(advance_20_days(4), vec![0, 0, 2, 5, 7]);
        assert_eq!(advance_20_days(2), vec![0, 3, 5, 5, 5, 7, 7]);
    }
}
