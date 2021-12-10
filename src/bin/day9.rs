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

    fn is_low_point(&self, x: usize, y: usize) -> Option<u8> {
        let height = self.heights[x][y];

        if x >= 1 && self.heights[x - 1][y] <= height {
            return None;
        }
        if x < self.max_x() - 1 && self.heights[x + 1][y] <= height {
            return None;
        }

        if y >= 1 && self.heights[x][y - 1] <= height {
            return None;
        }
        if y < self.max_y() - 1 && self.heights[x][y + 1] <= height {
            return None;
        }

        Some(height)
    }

    fn find_low_points(&self) -> Vec<u8> {
        (0..self.max_x())
            .flat_map(|x| (0..self.max_y()).filter_map(move |y| self.is_low_point(x, y)))
            .collect()
    }

    fn get_risk_level_sum(&self) -> u32 {
        self.find_low_points()
            .into_iter()
            .map(|p| 1 + p as u32)
            .sum()
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

        assert_eq!(height_map.is_low_point(0, 0), Some(1));
        assert_eq!(height_map.is_low_point(1, 0), None);
        assert_eq!(height_map.is_low_point(0, 1), None);
        assert_eq!(height_map.is_low_point(1, 1), None);
    }

    #[test]
    fn test_find_low_points() {
        assert_eq!(
            HeightMap::from("12\n34".to_string()).find_low_points(),
            vec![1]
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
}
