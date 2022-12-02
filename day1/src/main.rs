use std::{fs, collections::HashMap};

fn main() {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut map = HashMap::new();
    let mut elfno = 1;
    let mut cals = 0;
    for line in input.lines() {
        if line.is_empty() {
            map.insert(elfno.to_owned(), cals.to_owned());
            elfno += 1;
            cals = 0;
            continue
        }
        cals += line.trim().parse::<i32>().unwrap();
    }

    let mut highest = 0;
    let mut elf = 0;

    for (e,c) in &map {
        if c > &highest {
            highest = *c;
            elf = *e;
        }
    }

    println!("part1: elf:{} calories:{}", elf, highest);

    // part two

    let mut container = vec![0,0,0];
    for (_,c) in map {
        if c > container[0] {
            container[0] = c;
            container.sort();
        }
    }
    println!("part2: top 3 sum: {}", container.into_iter().sum::<i32>())


}
