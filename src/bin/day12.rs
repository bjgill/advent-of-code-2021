use std::collections::{HashMap, HashSet};

struct CaveNetwork(HashMap<String, Node>);

impl CaveNetwork {
    fn new() -> Self {
        CaveNetwork(HashMap::new())
    }

    fn add_node_pair(&mut self, node_a: &str, node_b: &str) {
        self.0
            .entry(node_a.to_string())
            .or_insert_with(|| Node::from(node_a))
            .add_adjacent(node_b);
        self.0
            .entry(node_b.to_string())
            .or_insert_with(|| Node::from(node_b))
            .add_adjacent(node_a);
    }

    fn extend_paths(&self, input_path: Vec<String>) -> Vec<Vec<String>> {
        let current_node = &self.0[input_path.last().unwrap()];

        match current_node {
            Node::Start { adjacents }
            | Node::BigCave { adjacents }
            | Node::SmallCave { adjacents } => adjacents
                .iter()
                .filter(|next_node| match &self.0[*next_node] {
                    Node::BigCave { .. } | Node::End => true,
                    Node::Start { .. } => false,
                    Node::SmallCave { .. } => !input_path.contains(next_node),
                })
                .flat_map(|a| {
                    let mut path = input_path.clone();
                    path.push(a.to_string());
                    self.extend_paths(path)
                })
                .collect(),
            Node::End => vec![input_path],
        }
    }

    fn get_path_count(&self) -> usize {
        self.extend_paths(vec!["start".to_string()]).len()
    }

    fn path_has_small_cave_repeat(&self, input_path: &[String]) -> bool {
        for i in 0..input_path.len() {
            let element = &self.0[&input_path[i]];

            match element {
                Node::BigCave { .. } | Node::End | Node::Start { .. } => continue,
                Node::SmallCave { .. } => {
                    if input_path[i + 1..].contains(&input_path[i]) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn extend_paths_with_one_repeat(&self, input_path: Vec<String>) -> Vec<Vec<String>> {
        let current_node = &self.0[input_path.last().unwrap()];
        let path_repeats = self.path_has_small_cave_repeat(&input_path);

        match current_node {
            Node::Start { adjacents }
            | Node::BigCave { adjacents }
            | Node::SmallCave { adjacents } => adjacents
                .iter()
                .filter(|next_node| match &self.0[*next_node] {
                    Node::BigCave { .. } | Node::End => true,
                    Node::Start { .. } => false,
                    Node::SmallCave { .. } => !(input_path.contains(next_node) && path_repeats),
                })
                .flat_map(|a| {
                    let mut path = input_path.clone();
                    path.push(a.to_string());
                    self.extend_paths_with_one_repeat(path)
                })
                .collect(),
            Node::End => vec![input_path],
        }
    }

    fn get_path_with_one_repeat_count(&self) -> usize {
        self.extend_paths_with_one_repeat(vec!["start".to_string()])
            .len()
    }
}

impl From<&str> for CaveNetwork {
    fn from(input: &str) -> CaveNetwork {
        let mut cave_network = CaveNetwork::new();

        for connection in input.split('\n') {
            let mut nodes = connection.split('-');
            let node_a = nodes.next().unwrap();
            let node_b = nodes.next().unwrap();

            cave_network.add_node_pair(node_a, node_b);
        }

        cave_network
    }
}

#[derive(PartialEq, Debug)]
enum Node {
    Start { adjacents: HashSet<String> },
    End,
    BigCave { adjacents: HashSet<String> },
    SmallCave { adjacents: HashSet<String> },
}

impl Node {
    fn add_adjacent(&mut self, adjacent: &str) {
        match self {
            Node::Start { adjacents }
            | Node::BigCave { adjacents }
            | Node::SmallCave { adjacents } => {
                adjacents.insert(adjacent.to_string());
            }
            Node::End => {}
        }
    }
}

impl From<&str> for Node {
    fn from(input: &str) -> Node {
        match input {
            "start" => Node::Start {
                adjacents: HashSet::new(),
            },
            "end" => Node::End,
            s if s.chars().all(|c| c.is_ascii_uppercase()) => Node::BigCave {
                adjacents: HashSet::new(),
            },
            s if s.chars().all(|c| c.is_ascii_lowercase()) => Node::SmallCave {
                adjacents: HashSet::new(),
            },
            e => panic!("Invalid node: {}", e),
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day12.txt").unwrap();
    let cave_network = CaveNetwork::from(data.as_str());

    println!("{} possible paths", cave_network.get_path_count());
    println!("{} possible paths with one repeated small cave", cave_network.get_path_with_one_repeat_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_end() {
        let mut cave_network = CaveNetwork::new();

        cave_network.add_node_pair("start", "end");

        assert_eq!(
            cave_network.extend_paths(vec!["start".to_string()]).len(),
            1,
        );
    }

    #[test]
    fn test_start_big_end() {
        let mut cave_network = CaveNetwork::new();

        cave_network.add_node_pair("start", "MIDDLE");
        cave_network.add_node_pair("MIDDLE", "end");

        assert_eq!(cave_network.get_path_count(), 1);
    }

    #[test]
    fn test_start_small_end() {
        let mut cave_network = CaveNetwork::new();

        cave_network.add_node_pair("start", "middle");
        cave_network.add_node_pair("middle", "end");

        assert_eq!(cave_network.get_path_count(), 1);
    }

    #[test]
    fn test_small_example() {
        let cave_network = CaveNetwork::from(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        );

        assert_eq!(cave_network.get_path_count(), 10);
        assert_eq!(cave_network.get_path_with_one_repeat_count(), 36);
    }

    #[test]
    fn test_big_example() {
        let cave_network = CaveNetwork::from(
            "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
        );

        assert_eq!(cave_network.get_path_count(), 226);
        assert_eq!(cave_network.get_path_with_one_repeat_count(), 3509);
    }
}
