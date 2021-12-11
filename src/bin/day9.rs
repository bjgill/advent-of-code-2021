use std::collections::HashSet;

type Point = (usize, usize, u8);

#[derive(Debug, PartialEq)]
struct HeightMap {
    heights: Vec<Vec<u8>>,
}

impl HeightMap {
    fn max_x(&self) -> usize {
        self.heights.len()
    }

    fn max_y(&self) -> usize {
        self.heights[0].len()
    }

    fn get_adjacent_points(&self, x: usize, y: usize) -> HashSet<Point> {
        let mut adjacents = HashSet::new();

        if x >= 1 {
            adjacents.insert((x - 1, y, self.heights[x - 1][y]));
        }
        if x < self.max_x() - 1 {
            adjacents.insert((x + 1, y, self.heights[x + 1][y]));
        }

        if y >= 1 {
            adjacents.insert((x, y - 1, self.heights[x][y - 1]));
        }
        if y < self.max_y() - 1 {
            adjacents.insert((x, y + 1, self.heights[x][y + 1]));
        }

        adjacents
    }

    fn is_low_point(&self, x: usize, y: usize) -> Option<Point> {
        let height = self.heights[x][y];

        for (_, _, adjacent_height) in self.get_adjacent_points(x, y) {
            if height >= adjacent_height {
                return None;
            }
        }

        Some((x, y, height))
    }

    fn find_low_points(&self) -> Vec<Point> {
        (0..self.max_x())
            .flat_map(|x| (0..self.max_y()).filter_map(move |y| self.is_low_point(x, y)))
            .collect()
    }

    fn get_risk_level_sum(&self) -> u32 {
        self.find_low_points()
            .into_iter()
            .map(|p| 1 + p.2 as u32)
            .sum()
    }

    fn get_basin_for_low_point(&self, low_point: Point) -> HashSet<Point> {
        let mut basin = HashSet::new();
        basin.insert(low_point);

        for height in (low_point.2 + 1)..9 {
            for basin_point in basin.clone() {
                for adjacent_point in self.get_adjacent_points(basin_point.0, basin_point.1) {
                    if adjacent_point.2 == height && !basin.contains(&adjacent_point) {
                        basin.insert(adjacent_point);
                    }
                }
            }
        }

        basin
    }

    fn get_three_largest_basin_sizes(&self) -> usize {
        let mut basins: Vec<_> = self
            .find_low_points()
            .into_iter()
            .map(|l| self.get_basin_for_low_point(l))
            .map(|basin| basin.len())
            .collect();

        basins.sort();
        basins.reverse();

        basins[0] * basins[1] * basins[2]
    }
}

impl From<String> for HeightMap {
    fn from(input: String) -> HeightMap {
        HeightMap {
            heights: input
                .split('\n')
                .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
                .collect(),
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day9.txt").unwrap();
    let map = HeightMap::from(data);

    println!("Sum of risk levels: {}", map.get_risk_level_sum());

    println!(
        "Product of three largest basins: {}",
        map.get_three_largest_basin_sizes()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            HeightMap::from("12\n34".to_string()),
            HeightMap {
                heights: vec![vec![1, 2], vec![3, 4]]
            }
        )
    }

    #[test]
    fn test_is_low_point() {
        let height_map = HeightMap::from("12\n34".to_string());

        assert_eq!(height_map.is_low_point(0, 0), Some((0, 0, 1)));
        assert_eq!(height_map.is_low_point(1, 0), None);
        assert_eq!(height_map.is_low_point(0, 1), None);
        assert_eq!(height_map.is_low_point(1, 1), None);
    }

    #[test]
    fn test_find_low_points() {
        assert_eq!(
            HeightMap::from("12\n34".to_string()).find_low_points(),
            vec![(0, 0, 1)]
        );
    }

    #[test]
    fn test_example_map() {
        let map = HeightMap::from(
            "2199943210
3987894921
9856789892
8767896789
9899965678"
                .to_string(),
        );

        assert_eq!(map.get_risk_level_sum(), 15);
    }

    #[test]
    fn test_example_map2() {
        let map = HeightMap::from(
            "2199943210
3987894921
0856789890
8767896789
1899965674"
                .to_string(),
        );

        assert_eq!(map.get_risk_level_sum(), 24);
    }

    #[test]
    fn test_get_basin() {
        let map = HeightMap::from(
            "2199943210
3987894921
9856789892
8767896789
9899965678"
                .to_string(),
        );

        assert_eq!(map.get_basin_for_low_point((0, 9, 0)).len(), 9);
    }
}
