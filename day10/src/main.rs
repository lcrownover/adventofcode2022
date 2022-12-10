use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

fn main() {
    part1();
}

#[derive(Debug)]
enum Command {
    Noop { sleep: i32 },
    Addx { sleep: i32, value: i32 },
}

fn parse_commands<P>(path: P) -> VecDeque<Command>
where
    P: Into<PathBuf>,
{
    let data = fs::read_to_string(path.into()).unwrap();
    let mut commands = VecDeque::new();
    for line in data.lines().into_iter() {
        let c = match line.split(" ").nth(0).unwrap() {
            "noop" => Command::Noop { sleep: 0 },
            "addx" => Command::Addx {
                value: line.split(" ").nth(1).unwrap().parse::<i32>().unwrap(),
                sleep: 1,
            },
            _ => panic!("unsupported command"),
        };
        commands.push_back(c);
    }
    commands
}

fn part1() {
    let mut reg_x = 1;
    let mut sleep_count = 0;
    let mut current_cycle = 0;
    let mut deferred_command: Command;
    let mut defer_count = 0;
    let mut commands = parse_commands("example1.txt");
    loop {
        current_cycle += 1;
        println!("starting cycle: {current_cycle}, X:{reg_x}");
        if sleep_count > 0 {
            println!("sleeping 1");
            sleep_count -= 1;
            continue;
        }
        let command = match commands.pop_front() {
            Some(c) => c,
            None => break,
        };
        println!("command:{:?}", command);
        match command {
            Command::Noop { sleep } => sleep_count += sleep,
            Command::Addx { sleep, value } => {
                sleep_count += sleep;
                reg_x += value;
            }
        }
        if current_cycle == 20 {
            println!("X:{reg_x}")
        }
    }
}
