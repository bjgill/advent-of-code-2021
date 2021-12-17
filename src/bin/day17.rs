struct TargetRegion {
    max_x: i32,
    min_x: i32,
    max_y: i32,
    min_y: i32,
}

impl TargetRegion {
    fn max_y_initial_velocity(&self) -> i32 {
        -self.min_y - 1
    }

    fn min_y_initial_velocity(&self) -> i32 {
        self.min_y
    }

    fn max_x_initial_velocity(&self) -> i32 {
        self.max_x
    }

    fn min_x_initial_velocity(&self) -> i32 {
        ((self.min_x * 2) as f64).sqrt() as i32
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y
    }

    fn trajectory_intersects(&self, mut x_velocity: i32, mut y_velocity: i32) -> bool {
        let mut x = 0;
        let mut y = 0;

        while y >= self.min_y {
            x += x_velocity;
            y += y_velocity;

            if self.contains(x, y) {
                return true;
            }

            if x_velocity > 0 {
                x_velocity -= 1;
            }
            y_velocity -= 1;
        }

        false
    }

    fn get_count_of_intersecting_trajectories(&self) -> usize {
        (self.min_x_initial_velocity()..=self.max_x_initial_velocity())
            .map(|x_velocity| {
                (self.min_y_initial_velocity()..=self.max_y_initial_velocity())
                    .filter(|y_velocity| self.trajectory_intersects(x_velocity, y_velocity.clone()))
                    .count()
            })
            .sum()
    }
}

fn main() {
    let target_region = TargetRegion {
        max_x: 161,
        min_x: 111,
        max_y: -101,
        min_y: -154,
    };

    // The region is below the y axis, so the solution is for y to be such that on the return
    // it jumps straight from 0 to -154.
    println!(
        "Analytic solution for part 1: {}",
        (target_region.max_y_initial_velocity() * (target_region.max_y_initial_velocity() + 1)) / 2
    );

    println!("Total intersecting trajectories: {}", target_region.get_count_of_intersecting_trajectories());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: TargetRegion = TargetRegion {
        max_x: 30,
        min_x: 20,
        max_y: -5,
        min_y: -10,
    };

    #[test]
    fn test_trajectory() {
        assert_eq!(EXAMPLE.trajectory_intersects(6, 3), true);
        assert_eq!(EXAMPLE.trajectory_intersects(23, -4), false);
    }

    #[test]
    fn test_all_trajectories() {
        assert_eq!(EXAMPLE.get_count_of_intersecting_trajectories(), 112);
    }
}
