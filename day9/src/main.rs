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
    knots: Vec<Knot>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Knot {
    current_position: Position,
    previous_position: Position,
    visited_positions: Vec<Position>,
}

impl Rope {
    fn new(knots: u32) -> Self {
        let init_position = Position { x: 0, y: 0 };
        Rope {
            knots: (1..=knots)
                .into_iter()
                .map(|_| Knot {
                    current_position: init_position,
                    previous_position: init_position,
                    visited_positions: vec![init_position],
                })
                .collect(),
        }
    }
    fn head(&self) -> &Knot {
        &self.knots[0]
    }
    fn head_mut(&mut self) -> &mut Knot {
        &mut self.knots[0]
    }
    fn knot(&self, number: u32) -> &Knot {
        &self.knots[number as usize]
    }
    fn knot_mut(&mut self, number: u32) -> &mut Knot {
        &mut self.knots[number as usize]
    }
    fn move_direction(&mut self, direction: Direction) {
        self.head_mut().previous_position = self.head().current_position;
        match direction {
            Direction::Up => {
                self.head_mut().current_position.y += 1;
                self.move_tail()
            }
            Direction::Down => {
                self.head_mut().current_position.y -= 1;
                self.move_tail()
            }
            Direction::Left => {
                self.head_mut().current_position.x -= 1;
                self.move_tail()
            }
            Direction::Right => {
                self.head_mut().current_position.x += 1;
                self.move_tail()
            }
        }
    }
    fn move_tail(&mut self) {
        if (self.head().current_position.x - self.knot(1).current_position.x).abs() > 1
            || (self.head().current_position.y - self.knot(1).current_position.y).abs() > 1
        {
            // head has moved away from tail, recalculate all the other knots
            for i in 0..self.knots.len() {
                let i = i as u32;
                if i == 0 {
                    continue;
                }
                // set the previous position of the knot to its current position
                self.knot_mut(i).previous_position = self.knot(i).current_position;
                // get the previous position of the knot ahead of it
                let new_pos = self.knot(i as u32 - 1).previous_position;
                // set the current position of this knot to that previous position
                self.knot_mut(i).current_position = new_pos;
                // update the visited positions with the new position
                self.knot_mut(i).visited_positions.push(new_pos)
            }
        }
    }
    fn count_visited_tail_positions(&mut self) -> u32 {
        let mut seen: Vec<Position> = Vec::new();
        for pos in self.knot(1).visited_positions.iter() {
            if !seen.contains(pos) {
                seen.push(*pos)
            }
        }
        seen.len() as u32
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let head = self.head();
        write!(
            f,
            "<Rope Knot1[{},{}] Knot2[{},{}]>",
            head.current_position.x,
            head.current_position.y,
            self.knots[1].current_position.x,
            self.knots[1].current_position.y
        )
    }
}

fn part1() {
    let instructions = parse_instructions("example.txt");
    // let instructions = parse_instructions("input.txt");
    let mut rope = Rope::new(2);
    for instruction in instructions.iter() {
        for _ in 1..=instruction.amount {
            rope.move_direction(instruction.direction)
        }
    }
    let visited_count = rope.count_visited_tail_positions();
    println!("{visited_count}");
}
fn part2() {
    let instructions = parse_instructions("example.txt");
    // let instructions = parse_instructions("input.txt");
    let mut rope = Rope::new(2);
    for instruction in instructions.iter() {
        for _ in 1..=instruction.amount {
            rope.move_direction(instruction.direction)
        }
    }
    let visited_count = rope.count_visited_tail_positions();
    println!("{visited_count}");
}
