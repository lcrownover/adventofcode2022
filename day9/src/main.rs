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
    None,
    Up,
    Down,
    Right,
    Left,
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
    previous_directions: Vec<Direction>,
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
                    previous_directions: vec![Direction::None],
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
    fn calculate_directions(&self, to: Position, from: Position) -> Vec<Direction> {
        // calculate which direction to return based on the coordinate change
        let mut directions: Vec<Direction> = Vec::new();
        // check vertically
        if (to.y - from.y).abs() > 1 {
            // it moved, check if up or down
            match to.y - from.y > 0 {
                // positive, so it moved up
                true => directions.push(Direction::Up),
                // negative, so it moved down
                false => directions.push(Direction::Down),
            }
        }
        // check horizontally
        if (to.x - from.x).abs() > 1 {
            // it moved, check if left or right
            match to.x - from.x > 0 {
                // positive, so it moved right
                true => directions.push(Direction::Right),
                // negative, so it moved left
                false => directions.push(Direction::Left),
            }
        }
        directions
    }
    fn move_direction(&self, position: Position, direction: Direction) -> Position {
        let mut new_pos = position;
        match direction {
            Direction::Up => {
                new_pos.y += 1;
            }
            Direction::Down => {
                new_pos.y -= 1;
            }
            Direction::Left => {
                new_pos.x -= 1;
            }
            Direction::Right => {
                new_pos.x += 1;
            }
            Direction::None => {}
        }
        new_pos
    }
    fn move_rope(&mut self, direction: Direction) {
        // set the previous position to the current position
        let current_head_position = self.head().current_position;
        self.head_mut().previous_position = current_head_position;
        // we don't really need this, but it's nice to have i suppose
        self.head_mut().previous_directions = vec![direction];
        // move the head, then move the tail
        let head_position = self.head_mut().current_position;
        self.head_mut().current_position = self.move_direction(head_position, direction);
        self.move_tail()
    }
    fn move_tail(&mut self) {
        if (self.head().current_position.x - self.knot(1).current_position.x).abs() > 1
            || (self.head().current_position.y - self.knot(1).current_position.y).abs() > 1
        {
            // head has moved away from tail, recalculate all the other knots
            for i in 0..self.knots.len() {
                let i = i as u32;
                // skip the first knot, that's the head
                if i == 0 {
                    continue;
                }
                // the head of the tail (knots[1]) governs the movement of
                // all the rest of the knots in the tail. we can calculate
                // and track the last movement action and replay that on
                // all downstream knots that need to move
                if i == 1 {
                    // this is the head of the tail, it moves unique to the other knots
                    // it always follows the previous location of the head.
                    let to = self.head().previous_position;
                    let from = self.knot(i).current_position;
                    self.knot_mut(i).current_position = self.head().previous_position;
                    self.knot_mut(i).previous_directions = self.calculate_directions(to, from);
                    continue;
                }
                // all other knots move equal to the previous direction on i-1
                let mut knot = self.knot_mut(i);
                let directions = self.knot(i-1).previous_directions;
                for i in 0..directions.len() {
                    let pos = self.knot(i as u32).current_position;
                    knot.current_position = self.move_direction(pos, directions[i])
                }

                // update the visited positions with the new current position
                let current_position = self.knot(i).current_position;
                self.knot_mut(i).visited_positions.push(current_position)
            }
        }
    }
    fn count_visited_tail_positions(&mut self) -> u32 {
        let mut seen: Vec<Position> = Vec::new();
        for (i, knot) in self.knots.iter().enumerate() {
            if i == 0 {
                continue;
            }
            for pos in knot.visited_positions.iter() {
                if !seen.contains(pos) {
                    // println!("pushing {:?}", pos);
                    seen.push(*pos)
                }
            }
        }
        seen.len() as u32
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut knot_string_vec: Vec<String> = Vec::new();
        for (i, knot) in self.knots.iter().enumerate() {
            knot_string_vec.push(format!(
                "Knot{}[{},{}] ",
                i, knot.current_position.x, knot.current_position.y
            ))
        }
        let knot_string: String = knot_string_vec.into_iter().collect();
        write!(f, "<{}>", knot_string)
    }
}

fn part1() {
    let instructions = parse_instructions("example.txt");
    // let instructions = parse_instructions("input.txt");
    let mut rope = Rope::new(2);
    for instruction in instructions.iter() {
        for _ in 1..=instruction.amount {
            rope.move_rope(instruction.direction)
        }
    }
    let visited_count = rope.count_visited_tail_positions();
    println!("{visited_count}");
}
fn part2() {
    let instructions = parse_instructions("example2.txt");
    // let instructions = parse_instructions("input.txt");
    let mut rope = Rope::new(10);
    for instruction in instructions.iter() {
        for _ in 1..=instruction.amount {
            println!("{}", rope);
            rope.move_rope(instruction.direction)
        }
    }
    let visited_count = rope.count_visited_tail_positions();
    println!("{visited_count}");
}
