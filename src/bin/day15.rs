//! Let's assume the path only goes right and downwards.
use std::cmp::min;

struct Cavern(Vec<Vec<usize>>);

impl From<String> for Cavern {
    fn from(input: String) -> Cavern {
        Cavern(
            input
                .split('\n')
                .map(|s| {
                    s.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
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

    fn get_5x_cave_position_risk(&self, x: usize, y: usize) -> usize {
        let x_repeats = x / self.max_x();
        let y_repeats = y / self.max_y();

        let x_remainder = x % self.max_x();
        let y_remainder = y % self.max_y();

        // eprintln!(
        //     "{} {} {} {} - {}",
        //     x_repeats, y_repeats, x_remainder, y_remainder, &self.0[x_remainder][y_remainder]
        // );

        let cumulative_risk = &self.0[x_remainder][y_remainder] + x_repeats + y_repeats;

        if cumulative_risk > 9 {
            cumulative_risk % 10 + 1
        } else {
            cumulative_risk
        }
    }

    fn get_quickest_path_length(&self) -> usize {
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

        *cave_path.last().unwrap().last().unwrap()
    }

    fn get_5x_quickest_path_length(&self) -> usize {
        let mut cave_path = vec![];

        for x in 0..self.max_x() * 5 {
            cave_path.push(vec![]);

            for y in 0..self.max_y() * 5 {
                print!("{}", self.get_5x_cave_position_risk(x, y));

                let path_length = if x > 0 && y > 0 {
                    let length = self.get_5x_cave_position_risk(x, y)
                        + min(cave_path[x - 1][y], cave_path[x][y - 1]);

                    // This is an abominable hack. For part 2, the assumption that the path
                    // monotonically increases rightwards and downwards fails. However, it turns
                    // out that it turns back only by one, so we only ned to look backwards one
                    // position in either direction to get the correct answer.
                    if length + self.get_5x_cave_position_risk(x - 1, y) < (cave_path[x - 1][y]) {
                        cave_path[x - 1][y] = length + self.get_5x_cave_position_risk(x - 1, y);
                    }
                    if length + self.get_5x_cave_position_risk(x, y - 1) < (cave_path[x][y - 1]) {
                        cave_path[x][y - 1] = length + self.get_5x_cave_position_risk(x, y - 1);
                    }

                    length
                } else if x > 0 {
                    self.get_5x_cave_position_risk(x, y) + cave_path[x - 1][y]
                } else if y > 0 {
                    self.get_5x_cave_position_risk(x, y) + cave_path[x][y - 1]
                } else {
                    // Costs 0 to enter the starting position.
                    0
                };

                cave_path[x].push(path_length);
            }
            println!("\n-");
        }

        *cave_path.last().unwrap().last().unwrap()
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day15.txt").unwrap();
    let cavern = Cavern::from(data);

    println!(
        "Shortest path length: {}",
        cavern.get_quickest_path_length()
    );

    println!(
        "Shortest path length for large cave: {}",
        cavern.get_5x_quickest_path_length()
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
        assert_eq!(cavern.get_5x_quickest_path_length(), 315);
    }

    #[test]
    fn test_value_large_cavern() {
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

        assert_eq!(cavern.get_5x_cave_position_risk(10, 10), 3);
    }
}
