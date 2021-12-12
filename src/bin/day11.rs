use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum Octopus {
    Dim(u8),
    Flashing,
    Flashed,
}

impl Octopus {
    fn new(energy: u8) -> Self {
        Octopus::Dim(energy)
    }

    fn energise(&mut self) {
        *self = match self {
            Octopus::Dim(e) if *e < 9 => Octopus::Dim(*e + 1),
            Octopus::Dim(_) | Octopus::Flashing => Octopus::Flashing,
            Octopus::Flashed => Octopus::Flashed,
        };
    }

    fn relax(&mut self) {
        match self {
            Octopus::Flashing => *self = Octopus::Flashed,
            _ => {}
        }
    }

    fn reset_for_new_step(&mut self) -> bool {
        match self {
            Octopus::Dim(_) => false,
            Octopus::Flashing => panic!("Unable to reset falshing octopus"),
            Octopus::Flashed => {
                *self = Octopus::Dim(0);
                true
            }
        }
    }

    fn is_flashing(&self) -> bool {
        self == &Octopus::Flashing
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cavern {
    octopuses: Vec<Vec<Octopus>>,
    flash_count: usize,
}

impl Cavern {
    fn max_x(&self) -> usize {
        self.octopuses.len()
    }

    fn max_y(&self) -> usize {
        self.octopuses[0].len()
    }

    fn get_adjacent_points(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut adjacents = HashSet::new();

        let min_x = if x > 0 { x - 1 } else { x };
        let max_x = if x < self.max_x() - 1 { x + 1 } else { x };

        let min_y = if y > 0 { y - 1 } else { y };
        let max_y = if y < self.max_y() - 1 { y + 1 } else { y };

        for adjacent_x in min_x..=max_x {
            for adjacent_y in min_y..=max_y {
                if adjacent_x != x || adjacent_y != y {
                    adjacents.insert((adjacent_x, adjacent_y));
                }
            }
        }

        adjacents
    }

    fn energise_all(&mut self) {
        self.octopuses
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|o| o.energise()))
    }

    fn reset_all(&mut self) {
        self.flash_count += self
            .octopuses
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|o| o.reset_for_new_step())
                    .filter(|flashed| *flashed)
                    .count()
            })
            .sum::<usize>();
    }

    fn process_flashing(mut self) -> Self {
        let original = self.clone();

        for x in 0..self.max_x() {
            for y in 0..self.max_y() {
                let octopus = &mut self.octopuses[x][y];

                if octopus.is_flashing() {
                    octopus.relax();

                    for (adjacent_x, adjacent_y) in self.get_adjacent_points(x, y) {
                        self.octopuses[adjacent_x][adjacent_y].energise()
                    }
                }
            }
        }

        if self == original {
            self.reset_all();
            self
        } else {
            self.process_flashing()
        }
    }

    fn step(mut self) -> Self {
        self.energise_all();

        self.process_flashing()
    }

    fn step_times(mut self, times: u32) -> Self {
        for _ in 0..times {
            self = self.step()
        }

        self
    }

    fn steps_until_synchronised(mut self) -> u32 {
        let mut previous_flashes = 0;
        let mut step = 0;

        loop {
            if self.flash_count == self.max_x() * self.max_y() + previous_flashes {
                return step;
            }
            previous_flashes = self.flash_count;
            step += 1;
            self = self.step();
        }
    }
}

impl From<String> for Cavern {
    fn from(input: String) -> Cavern {
        Cavern {
            octopuses: input
                .split('\n')
                .map(|s| {
                    s.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .map(Octopus::new)
                        .collect()
                })
                .collect(),
            flash_count: 0,
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day11.txt").unwrap();
    let cavern = Cavern::from(data);

    println!(
        "Flashes after 100 steps: {}",
        cavern.clone().step_times(100).flash_count
    );

    println!(
        "Synchronises after {} steps",
        cavern.steps_until_synchronised()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Cavern::from("12\n34".to_string()),
            Cavern {
                octopuses: vec![
                    vec![Octopus::new(1), Octopus::new(2)],
                    vec![Octopus::new(3), Octopus::new(4)]
                ],
                flash_count: 0,
            }
        )
    }

    #[test]
    fn test_flash_octopus() {
        let mut octopus = Octopus::new(8);
        octopus.energise();
        assert_eq!(octopus, Octopus::new(9));
        octopus.energise();
        assert_eq!(octopus, Octopus::Flashing);
        octopus.energise();
        assert_eq!(octopus, Octopus::Flashing);
    }

    #[test]
    fn test_flashing_step() {
        assert_eq!(
            Cavern::from("12\n34".to_string()).step().octopuses,
            Cavern::from("23\n45".to_string()).octopuses,
        );
    }

    #[test]
    fn test_flashing_overflow() {
        assert_eq!(
            Cavern::from("98\n34".to_string()).step().octopuses,
            Cavern::from("00\n67".to_string()).octopuses,
        );
    }

    #[test]
    fn test_example_step_once() {
        assert_eq!(
            Cavern::from(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
                    .to_string()
            )
            .step(),
            Cavern::from(
                "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_example_step_many() {
        let cavern = Cavern::from(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
                .to_string(),
        )
        .step_times(100);

        assert_eq!(
            cavern.octopuses,
            Cavern::from(
                "0397666866
0749766918
0053976933
0004297822
0004229892
0053222877
0532222966
9322228966
7922286866
6789998766"
                    .to_string()
            )
            .octopuses
        );
        assert_eq!(cavern.flash_count, 1656);
    }
}
