//! Let's assume the path only goes right and downwards.
use std::cmp::min;

struct Cavern(Vec<Vec<u32>>);

impl From<String> for Cavern {
    fn from(input: String) -> Cavern {
        Cavern(
            input
                .split('\n')
                .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        )
    }
}

impl Cavern {
    fn max_x(&self) -> usize {
        self.0.len()
    }

    fn max_y(&self) -> usize {
        self.0[0].len()
    }

    fn get_quickest_path_length(&self) -> u32 {
        let mut cave_path = vec![];

        for x in 0..self.max_x() {
            cave_path.push(vec![]);

            for y in 0..self.max_y() {
                let path_length = if x > 0 && y > 0 {
                    self.0[x][y] + min(cave_path[x - 1][y], cave_path[x][y - 1])
                } else if x > 0 {
                    self.0[x][y] + cave_path[x - 1][y]
                } else if y > 0 {
                    self.0[x][y] + cave_path[x][y - 1]
                } else {
                    // Costs 0 to enter the starting position.
                    0
                };

                cave_path[x].push(path_length);
            }
        }

        cave_path[self.max_x() - 1][self.max_y() - 1]
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day15.txt").unwrap();
    let cavern = Cavern::from(data);

    println!(
        "Shortest path length: {}",
        cavern.get_quickest_path_length()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let cavern = Cavern::from(
            "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
                .to_string(),
        );

        assert_eq!(cavern.get_quickest_path_length(), 40);
    }
}
