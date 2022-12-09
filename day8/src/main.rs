use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
    nodes: Vec<Node>,
}

impl Grid {
    fn new() -> Self {
        Grid { nodes: Vec::new() }
    }
    fn from_input(input: String) -> Self {
        let mut grid = Grid::new();
        for (y, row) in input.lines().enumerate() {
            for (x, c) in row
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .enumerate()
            {
                grid.add_node(x as i32, y as i32, c)
            }
        }
        grid
    }
    fn add_node(&mut self, x: i32, y: i32, value: i32) {
        self.nodes.push(Node { x, y, value })
    }
    fn get_node(&self, x: i32, y: i32) -> Option<Node> {
        for node in self.nodes.iter() {
            if node.x == x && node.y == y {
                return Some(*node);
            }
        }
        None
    }

    fn _get_neighbor(&self, node: Node, direction: &Direction) -> Option<Node> {
        match direction {
            Direction::Up => self.get_node(node.x, node.y - 1),
            Direction::Down => self.get_node(node.x, node.y + 1),
            Direction::Left => self.get_node(node.x - 1, node.y),
            Direction::Right => self.get_node(node.x + 1, node.y),
        }
    }

    fn _get_neighbors(&self, node: Node, direction: &Direction) -> Vec<Node> {
        let mut current_node = node;
        let mut neighbors = Vec::new();
        while let Some(next_neighbor) = self._get_neighbor(current_node, direction) {
            neighbors.push(next_neighbor);
            current_node = next_neighbor;
        }
        neighbors
    }

    fn node_is_visible(&self, node: Node) -> bool {
        for direction in Direction::iter() {
            let neighbors = self._get_neighbors(node, &direction);
            if neighbors.is_empty() {
                // no neighbors means an edge node which is always visible
                return true;
            }
            // check through all the neighbors, starting at the closest
            // neighbor to the node. if all neighbor node values are lower
            // than the node value, than node is visible
            // yes, this is inefficient because it has to iterate through
            // all the elements rather than breaking early
            if neighbors.iter().all(|neighbor| neighbor.value < node.value) {
                return true;
            }
        }
        false
    }
    fn visible_count(&self) -> i32 {
        self.nodes
            .iter()
            .filter(|&n| self.node_is_visible(*n))
            .count() as i32
    }
    fn scenic_score(&self, node: Node) -> i32 {
        let mut direction_scores: Vec<i32> = Vec::new();
        for direction in Direction::iter() {
            // each direction registers a direction_score
            let neighbors = self._get_neighbors(node, &direction);

            // we can just continue to the next direction
            // edges are worth 0
            if neighbors.is_empty() {
                direction_scores.push(0);
                continue;
            }

            let mut count = 0;
            for neighbor in neighbors.iter() {
                // we can see past this tree,
                // so add it to count and keep going
                count += 1;
                if neighbor.value >= node.value {
                    break;
                }
            }
            direction_scores.push(count);
        }

        let mut scenic_score = 1;
        for score in direction_scores.iter() {
            scenic_score *= score
        }
        scenic_score
    }
    fn highest_scenic_score(&self) -> i32 {
        self.nodes
            .iter()
            .map(|&n| self.scenic_score(n))
            .max()
            .unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
struct Node {
    x: i32,
    y: i32,
    value: i32,
}

fn part1() {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let grid = Grid::from_input(input);
    println!("{}", grid.visible_count());
}

fn part2() {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let grid = Grid::from_input(input);
    println!("{}", grid.highest_scenic_score())
}

fn main() {
    part1();
    part2();
}
