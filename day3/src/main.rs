use std::{collections::HashMap, fs};

fn main() {
    //let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();

    let mut values = HashMap::new();
    values.insert('a', 1);
    values.insert('b', 2);
    values.insert('c', 3);
    values.insert('d', 5);
    values.insert('e', 5);
    values.insert('f', 6);
    values.insert('g', 7);
    values.insert('h', 8);
    values.insert('i', 9);
    values.insert('j', 10);
    values.insert('k', 11);
    values.insert('l', 12);
    values.insert('m', 13);
    values.insert('n', 14);
    values.insert('o', 15);
    values.insert('p', 16);
    values.insert('q', 17);
    values.insert('r', 18);
    values.insert('s', 19);
    values.insert('t', 20);
    values.insert('u', 21);
    values.insert('v', 22);
    values.insert('w', 23);
    values.insert('x', 24);
    values.insert('y', 25);
    values.insert('z', 26);

    values.insert('A', 27);
    values.insert('B', 28);
    values.insert('C', 29);
    values.insert('D', 30);
    values.insert('E', 31);
    values.insert('F', 32);
    values.insert('G', 33);
    values.insert('H', 34);
    values.insert('I', 35);
    values.insert('J', 36);
    values.insert('K', 37);
    values.insert('L', 38);
    values.insert('M', 39);
    values.insert('N', 40);
    values.insert('O', 41);
    values.insert('P', 42);
    values.insert('Q', 43);
    values.insert('R', 44);
    values.insert('S', 45);
    values.insert('T', 46);
    values.insert('U', 47);
    values.insert('V', 48);
    values.insert('W', 49);
    values.insert('X', 50);
    values.insert('Y', 51);
    values.insert('Z', 52);

    let mut sames: Vec<char> = Vec::new();
    for line in input.lines().into_iter() {
        let container1: Vec<char> = line.chars().take(line.len() / 2).collect();
        let container2: Vec<char> = line.chars().rev().take(line.len() / 2).collect();
        let mut same: Vec<char> = Vec::new();
        //println!("{:?}", container1);
        //println!("{:?}", container2);
        for c1 in container1.iter() {
            for c2 in container2.iter() {
                if c1 == c2 && !same.contains(c1) {
                    same.push(c1.to_owned());
                }
            }
        }
        sames.push(same[0]);
    }
    let points: i32 = sames
        .iter()
        .map(|c| values
            .get(c)
            .unwrap()
            .to_owned())
        .sum();
    
    println!("{:?}", points);
}
