use core::fmt;
use std::{fs, path::PathBuf};

fn main() {
    // part1();
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

fn move_direction(position: Position, direction: &Direction) -> Position {
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
        Direction::Stay => {}
    }
    new_pos
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Stay,
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
    has_moved: bool,
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
                    previous_directions: vec![Direction::Stay],
                    visited_positions: vec![init_position],
                    has_moved: false,
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
        println!("from:{:?}", from);
        println!("to:{:?}", to);
        // check vertically
        if (to.y - from.y).abs() > 0 {
            println!("it moved vert");
            // it moved, check if up or down
            match to.y - from.y > 0 {
                true => directions.push(Direction::Up), // positive, so it moved up
                false => directions.push(Direction::Down), // negative, so it moved down
            }
        }
        if (to.x - from.x).abs() > 0 {
            println!("it moved horiz");
            // check horizontally
            match to.x - from.x > 0 {
                true => directions.push(Direction::Right), // positive, so it moved right
                false => directions.push(Direction::Left), // negative, so it moved left
            }
        }
        directions
    }
    fn move_rope(&mut self, direction: Direction) {
        println!("moving head: {:?}", direction);
        // set the previous position to the current position
        let current_head_position = self.head().current_position;
        self.head_mut().previous_position = current_head_position;
        // we don't really need this, but it's nice to have i suppose
        self.head_mut().previous_directions = vec![direction];
        // move the head, then move the tail
        let head_position = self.head_mut().current_position;
        self.head_mut().current_position = move_direction(head_position, &direction);
        self.head_mut().has_moved = true;
        self.move_tail()
    }
    fn move_tail(&mut self) {
        println!("starting move tail");
        if (self.head().current_position.x - self.knot(1).current_position.x).abs() > 1
            || (self.head().current_position.y - self.knot(1).current_position.y).abs() > 1
        {
            println!("head moved farther than 1 from second knot, moving rest of tail");
            // head has moved away from tail, recalculate all the other knots
            // skip index 0, thats the head
            // in the beginning, a knot shouldnt move unless the previous has already moved
            let mut current_mover = 0;
            for i in 1..self.knots.len() {
                let i = i as u32;
                println!("moving knot[{}]", i);
                // the head of the tail (knots[1]) governs the movement of
                // all the rest of the knots in the tail. we can calculate
                // and track the last movement action and replay that on
                // all downstream knots that need to move
                //
                // first, we check if the previous knot has moved. if it hasnt,
                // that means that we shouldnt move this knot
                if !self.knot(i - 1).has_moved {
                    println!(
                        "knot[{}] is not moving because knot[{}] either just moved hasnt moved yet",
                        i,
                        i - 1
                    );
                    continue;
                }
                let mut new_pos = self.knot(i).current_position;
                if i == 1 {
                    // this is the head of the tail, it moves unique to the other knots
                    // it always follows the previous location of the head.
                    // new_pos is what we're gonna assign to the current knot
                    // we store in a var outside this scope so we can
                    // append it to the knot's visited positions
                    new_pos = self.head().previous_position;
                    // now we need to calculate the directions that every other node
                    // will take to copy how knot[1] moved.
                    // we set "to" to the new position of knot[1]
                    let to = new_pos;
                    // then "from" should be the current position of the current knot
                    let from = self.knot(i).current_position;
                    self.knot_mut(i).previous_directions = self.calculate_directions(to, from);
                    println!(
                        "previous directions of knot1: {:?}",
                        self.knot(i).previous_directions
                    );
                    // finally, we set the current position of this knot[1]
                    // to the new position
                    self.knot_mut(i).current_position = new_pos;
                } else {
                    // all other knots move equal to the previous direction on i-1
                    let directions: Vec<Direction> = self
                        .knot(i - 1)
                        .previous_directions
                        .iter()
                        .map(|d| d.to_owned())
                        .collect();
                    println!("directions to do:  {:?}", directions);
                    let knot = self.knot_mut(i);
                    for direction in directions.iter() {
                        new_pos = move_direction(new_pos, direction)
                    }
                    knot.current_position = new_pos;
                    knot.previous_directions = directions;
                }

                // update the visited positions with the new current position
                self.knot_mut(i).visited_positions.push(new_pos);

                // now this knot has moved
                current_mover = i;
            }
            self.knot_mut(current_mover).has_moved = true;
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
    println!("{}", rope);
    for instruction in instructions.iter() {
        for _ in 1..=instruction.amount {
            rope.move_rope(instruction.direction);
            println!("{}", rope);
        }
    }
    let visited_count = rope.count_visited_tail_positions();
    println!("{visited_count}");
}
