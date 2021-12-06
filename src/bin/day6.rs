/// Represents the number of each age of fish present
#[derive(PartialEq, Debug, Clone)]
struct Shoal([u64; 9]);

impl Shoal {
    fn count(&self) -> u64 {
        self.0.iter().sum()
    }

    /// Inverting the 10-day look-up table
    fn advance_10_days(self) -> Self {
        Shoal([
            self.0[1] + self.0[3],
            self.0[2] + self.0[4],
            self.0[3] + self.0[5],
            self.0[4] + self.0[6],
            self.0[0] + self.0[5] + self.0[7],
            self.0[1] + self.0[6] + self.0[8],
            self.0[0] * 2 + self.0[2] + self.0[7],
            self.0[1] + self.0[8],
            self.0[0] + self.0[2],
        ])
    }

    fn advance_16_days(self) -> Self {
        Shoal([
            self.0[0] * 2 + self.0[2] + self.0[7],
            self.0[1] * 2 + self.0[3] + self.0[8],
            self.0[0] + self.0[2] * 2 + self.0[4],
            self.0[1] + self.0[3] * 2 + self.0[5],
            self.0[2] + self.0[4] * 2 + self.0[6],
            self.0[0] + self.0[3] + self.0[5] * 2 + self.0[7],
            self.0[1] + self.0[4] + self.0[6] * 2 + self.0[8],
            self.0[0] + self.0[5] + self.0[7],
            self.0[1] + self.0[6] + self.0[8],
        ])
    }

    fn advance_80_days(self) -> Self {
        let mut shoal = self;
        for _ in 0..8 {
            shoal = shoal.advance_10_days();
        }
        shoal
    }

    fn advance_256_days(self) -> Self {
        let mut shoal = self;
        for _ in 0..16 {
            shoal = shoal.advance_16_days();
        }
        shoal
    }
}

impl From<String> for Shoal {
    fn from(s: String) -> Shoal {
        let mut new = Shoal([0; 9]);
        s.split(',')
            .map(|f| f.parse().unwrap())
            .for_each(|f: usize| new.0[f] += 1);
        new
    }
}

impl From<Vec<u64>> for Shoal {
    fn from(fish: Vec<u64>) -> Shoal {
        let mut new = Shoal([0; 9]);
        fish.into_iter().for_each(|f: u64| new.0[f as usize] += 1);
        new
    }
}

/// Look-up table manually calculated
fn advance_10_days(fish: u64) -> Vec<u64> {
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

/// Look-up table manually calculated
fn advance_16_days(fish: u64) -> Vec<u64> {
    match fish {
        8 => vec![1, 6, 8],
        7 => vec![0, 5, 7],
        6 => vec![4, 6, 6, 8],
        5 => vec![3, 5, 5, 7],
        4 => vec![2, 4, 4, 6],
        3 => vec![1, 3, 3, 5],
        2 => vec![0, 2, 2, 4],
        1 => vec![1, 1, 3, 6, 8],
        0 => vec![0, 0, 2, 5, 7],
        n => panic!("Fish with invalid number: {}", n),
    }
}

fn advance_double_days(fish: u64, advance_fn: fn(u64) -> Vec<u64>) -> Vec<u64> {
    let mut fish: Vec<_> = advance_fn(fish)
        .into_iter()
        .map(advance_fn)
        .flat_map(|x| x.into_iter())
        .collect();
    fish.sort();
    fish
}

fn advance_80_days(fish: u64) -> Vec<u64> {
    advance_double_days(fish, |fish| {
        advance_double_days(fish, |fish| advance_double_days(fish, advance_10_days))
    })
}

fn parse_fish(fish_input: &str) -> Vec<u64> {
    fish_input.split(',').map(|f| f.parse().unwrap()).collect()
}

fn main() {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    let starting_shoal = Shoal::from(input);

    println!("{} starting fish", starting_shoal.count());
    println!(
        "{} fish after 80 days",
        starting_shoal.clone().advance_80_days().count()
    );
    println!(
        "{} fish after 256 days",
        starting_shoal.clone().advance_256_days().count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn advance_20_days(fish: u64) -> Vec<u64> {
        advance_double_days(fish, advance_10_days)
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_fish("3,4,3,1,2"), vec![3, 4, 3, 1, 2])
    }

    #[test]
    fn test_parse_shoal() {
        assert_eq!(
            Shoal::from("3,4,3,1,2".to_string()),
            Shoal([0, 1, 1, 2, 1, 0, 0, 0, 0])
        )
    }

    #[test]
    /// Compare to hand-calcuated values.
    fn test_advance_20_days() {
        assert_eq!(advance_20_days(8), vec![2, 4, 4, 6]);
        assert_eq!(advance_20_days(4), vec![0, 0, 2, 5, 7]);
        assert_eq!(advance_20_days(2), vec![0, 3, 5, 5, 5, 7, 7]);
    }

    #[test]
    fn test_advance_shoal_10_days() {
        for i in 0..=8 {
            let input = i.to_string();

            assert_eq!(
                Shoal::from(input.clone()).advance_10_days(),
                Shoal::from(
                    parse_fish(&input)
                        .into_iter()
                        .map(advance_10_days)
                        .flat_map(|x| x.into_iter())
                        .collect::<Vec<_>>()
                )
            );
        }
    }

    #[test]
    fn test_advance_shoal_20_days() {
        for i in 0..=8 {
            let input = i.to_string();

            assert_eq!(
                Shoal::from(input.clone())
                    .advance_10_days()
                    .advance_10_days(),
                Shoal::from(
                    parse_fish(&input)
                        .into_iter()
                        .map(advance_20_days)
                        .flat_map(|x| x.into_iter())
                        .collect::<Vec<_>>()
                )
            );
        }
    }

    #[test]
    fn test_advance_shoal_16_days() {
        for i in 0..=8 {
            let input = i.to_string();

            assert_eq!(
                Shoal::from(input.clone()).advance_16_days(),
                Shoal::from(
                    parse_fish(&input)
                        .into_iter()
                        .map(advance_16_days)
                        .flat_map(|x| x.into_iter())
                        .collect::<Vec<_>>()
                ),
                "Failed with input: {}",
                i
            );
        }
    }

    #[test]
    fn test_fill_ocean() {
        assert_eq!(
            Shoal::from("3,4,3,1,2".to_string())
                .advance_256_days()
                .count(),
            26984457539
        );
    }
}
