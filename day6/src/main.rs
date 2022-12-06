use std::{collections::VecDeque, fs};

fn main() {
    part1();
    part2();
}

struct SizedBuffer {
    size: usize,
    chars: VecDeque<char>,
}

impl SizedBuffer {
    fn new(s: usize) -> Self {
        SizedBuffer {
            size: s,
            chars: VecDeque::new(),
        }
    }
    fn append(&mut self, c: char) {
        self.chars.push_back(c);
        if self.chars.len() > self.size {
            self.chars.pop_front();
        }
    }
    fn check_unique(&self) -> bool {
        let mut v = self.chars
            .iter()
            .map(|c| c.to_owned())
            .collect::<Vec<char>>();
        v.sort();
        v.dedup();
        v.len() == self.size
    }
}

fn part1() {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sb = SizedBuffer::new(4);
    let mut counter = 0;
    for c in input.chars().into_iter() {
        counter += 1;
        sb.append(c);
        if sb.check_unique() {
            println!("{counter}");
            return;
        }
    }
}
fn part2() {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sb = SizedBuffer::new(14);
    let mut counter = 0;
    for c in input.chars().into_iter() {
        counter += 1;
        sb.append(c);
        if sb.check_unique() {
            println!("{counter}");
            return;
        }
    }
}
