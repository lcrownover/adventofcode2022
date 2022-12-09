use core::fmt;
use std::{fs, path::PathBuf};

fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: u32,
}

fn parse_instructions<P>(input: P) -> Vec<Instruction>
where
    P: Into<PathBuf>,
{
    let data = fs::read_to_string(input.into()).unwrap();
    let mut instructions = Vec::new();
    for line in data.lines().into_iter() {
        let direction = match line.split(" ").nth(0).unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("bad instruction"),
        };
        let amount = line.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        instructions.push(Instruction { direction, amount })
    }
    instructions
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Rope {
    head: RopeHead,
    tail: RopeTail,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct RopeHead {
    current_position: Position,
    previous_position: Position,
}

#[derive(Debug)]
struct RopeTail {
    current_position: Position,
    visited_positions: Vec<Position>,
}

impl Rope {
    fn new() -> Self {
        let init_position = Position { x: 0, y: 0 };
        let head = RopeHead {
            current_position: init_position,
            previous_position: init_position,
        };
        let tail = RopeTail {
            current_position: init_position,
            visited_positions: vec![init_position],
        };
        Rope { head, tail }
    }
    fn move_direction(&mut self, direction: Direction) {
        self.head.previous_position = self.head.current_position;
        match direction {
            Direction::Up => {
                self.head.current_position.y += 1;
                self.move_tail()
            }
            Direction::Down => {
                self.head.current_position.y -= 1;
                self.move_tail()
            }
            Direction::Left => {
                self.head.current_position.x -= 1;
                self.move_tail()
            }
            Direction::Right => {
                self.head.current_position.x += 1;
                self.move_tail()
            }
        }
    }
    fn move_tail(&mut self) {
        if (self.head.current_position.x - self.tail.current_position.x).abs() > 1
            || (self.head.current_position.y - self.tail.current_position.y).abs() > 1
        {
            self.tail.current_position = self.head.previous_position;
            self.tail.visited_positions.push(self.tail.current_position)
        }
    }
    fn count_visited_tail_positions(&self) -> u32 {
        let mut seen: Vec<Position> = Vec::new();
        for pos in self.tail.visited_positions.iter() {
            if !seen.contains(pos) {
                seen.push(*pos)
            }
        }
        seen.len() as u32
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Rope Head[{},{}] Tail[{},{}]>",
            self.head.current_position.x,
            self.head.current_position.y,
            self.tail.current_position.x,
            self.tail.current_position.y
        )
    }
}

fn part1() {
    // let instructions = parse_instructions("example.txt");
    let instructions = parse_instructions("input.txt");
    let mut rope = Rope::new();
    for instruction in instructions.iter() {
        for _ in 1..=instruction.amount {
            rope.move_direction(instruction.direction)
        }
    }
    let visited_count = rope.count_visited_tail_positions();
    println!("{visited_count}");
}
fn part2() {}
