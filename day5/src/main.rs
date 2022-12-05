use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn parse_stacks(input: String) -> HashMap<i32, VecDeque<char>> {
    let mut map: HashMap<i32, VecDeque<char>> = HashMap::new();
    let stack_amount = input
        .lines()
        .find(|l| l.trim().starts_with("1"))
        .unwrap()
        .replace(" ", "")
        .len();
    // println!("number of stacks: {stack_amount}");
    for n in 0..stack_amount {
        map.insert(n as i32, VecDeque::new());
    }

    let stack_lines: Vec<String> = input
        .lines()
        .filter(|l| l.trim().starts_with("["))
        .map(|l| l.to_owned())
        .collect();

    // println!("{:?}", stack_lines);

    let xmax = stack_amount * 4;
    for line in stack_lines {
        for x in 0..=xmax {
            let c = match line.chars().nth(x) {
                Some(c) => c,
                None => {
                    continue;
                }
            };
            if c == '[' || c == ']' || c == ' ' {
                continue;
            }
            let mut n = 0;
            if x > 4 {
                n = x / 4;
            }
            // println!("inserting {c} into stack {n}");
            map.get_mut(&(n as i32)).unwrap().push_back(c);
        }
    }
    map
}

fn get_instruction_list(input: String) -> Vec<String> {
    input
        .lines()
        .filter(|l| l.starts_with("move"))
        .map(|l| l.to_owned())
        .collect()
}

fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct Instruction {
    amt: i32,
    src: i32,
    dst: i32,
}

impl Instruction {
    fn from_string(s: String) -> Self {
        let amt = s.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        let src = s.split(" ").nth(3).unwrap().parse::<i32>().unwrap() - 1;
        let dst = s.split(" ").nth(5).unwrap().parse::<i32>().unwrap() - 1;
        Instruction { amt, src, dst }
    }
}

fn part1() {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut stackmap = parse_stacks(input.clone());
    // println!("{:?}", stackmap);
    let instruction_list: Vec<Instruction> = get_instruction_list(input.clone())
        .into_iter()
        .map(|s| Instruction::from_string(s))
        .collect();
    // println!("{:?}", instruction_list);
    for instruction in instruction_list {
        // println!("{:?}", instruction);
        let mut src_vec = stackmap.get(&instruction.src).unwrap().clone();
        let mut dst_vec = stackmap.get(&instruction.dst).unwrap().clone();
        // println!("before:");
        // println!("src_vec: {:?}", src_vec);
        // println!("dst_vec: {:?}", dst_vec);
        for _ in 0..instruction.amt {
            let b = src_vec.pop_front().unwrap();
            dst_vec.push_front(b);
        }
        // println!("after:");
        // println!("src_vec: {:?}", src_vec);
        // println!("dst_vec: {:?}", dst_vec);
        stackmap.insert(instruction.dst, dst_vec);
        stackmap.insert(instruction.src, src_vec);
    }

    // println!("{:?}", stackmap);

    let mut ms: Vec<char> = Vec::new();
    for i in 0..stackmap.len() {
        let c = stackmap
            .get(&(i as i32))
            .unwrap()
            .into_iter()
            .nth(0)
            .unwrap();
        ms.push(*c)
    }
    // println!("{:?}", ms);
    let message: String = ms.iter().collect();
    println!("{message}");
}

fn part2() {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut stackmap = parse_stacks(input.clone());
    // println!("{:?}", stackmap);
    let instruction_list: Vec<Instruction> = get_instruction_list(input.clone())
        .into_iter()
        .map(|s| Instruction::from_string(s))
        .collect();
    // println!("{:?}", instruction_list);
    for instruction in instruction_list {
        // println!("{:?}", instruction);
        let mut src_vec = stackmap.get(&instruction.src).unwrap().clone();
        let mut dst_vec = stackmap.get(&instruction.dst).unwrap().clone();
        // println!("before:");
        // println!("src_vec: {:?}", src_vec);
        // println!("dst_vec: {:?}", dst_vec);

        let mut buffer: VecDeque<char> = VecDeque::new();
        for _ in 0..instruction.amt {
            let b = src_vec.pop_front().unwrap();
            buffer.push_front(b);
        }
        while buffer.len() > 0 {
            dst_vec.push_front(buffer.pop_front().unwrap());
        }
        // println!("after:");
        // println!("src_vec: {:?}", src_vec);
        // println!("dst_vec: {:?}", dst_vec);
        stackmap.insert(instruction.dst, dst_vec);
        stackmap.insert(instruction.src, src_vec);
    }

    // println!("{:?}", stackmap);

    let mut ms: Vec<char> = Vec::new();
    for i in 0..stackmap.len() {
        let c = stackmap
            .get(&(i as i32))
            .unwrap()
            .into_iter()
            .nth(0)
            .unwrap();
        ms.push(*c)
    }
    // println!("{:?}", ms);
    let message: String = ms.iter().collect();
    println!("{message}");
}
