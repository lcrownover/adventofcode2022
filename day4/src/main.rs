use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    //let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sad_elfs = 0;

    // each elf pair
    for line in input.lines() {
        // println!("elf line: {:?}", line);
        // split lines into two elf strings
        let elf1_s = line.split(",").nth(0).unwrap();
        let elf2_s = line.split(",").nth(1).unwrap();

        // expand each elf
        let elf1_low = elf1_s.split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let elf1_high = elf1_s.split("-").nth(1).unwrap().parse::<i32>().unwrap();
        let elf1_r: Vec<i32> = (elf1_low..elf1_high + 1).collect();
        let elf2_low = elf2_s.split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let elf2_high = elf2_s.split("-").nth(1).unwrap().parse::<i32>().unwrap();
        let elf2_r: Vec<i32> = (elf2_low..elf2_high + 1).collect();

        // println!("elf1: {:?}", &elf1_r);
        // println!("elf2: {:?}", &elf2_r);


        // elf1 values in elf2
        let left_side: bool = {
            // println!("elf1 check");
            let mut keep_going = true;
            let mut ret = true;
            for i1 in elf1_r.iter() {
                if !keep_going {
                    break;
                }
                // println!("checking {i1}");
                if !elf2_r.contains(&i1) {
                    // println!("  {i1} not in {:?}", elf2_r);
                    ret = false;
                    keep_going = false;
                }
            }
            ret
        };

        // elf2 values in elf1
        let right_side: bool = {
            // println!("elf2 check");
            let mut keep_going = true;
            let mut ret = true;
            for i2 in elf2_r.iter() {
                if !keep_going {
                    break;
                }
                // println!("checking {i2}");
                if !elf1_r.contains(&i2) {
                    // println!("  {i2} not in {:?}", elf1_r);
                    ret = false;
                    keep_going = false;
                }
            }
            ret
        };

        if left_side || right_side {
            // println!("lazy elfs here");
            sad_elfs += 1;
        }
    }
    println!("{:?}", sad_elfs);
}

fn part2() {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();

    let mut overlaps = 0;

    // each elf pair
    for line in input.lines() {
        // split lines into two elf strings
        let elf1_s = line.split(",").nth(0).unwrap();
        let elf2_s = line.split(",").nth(1).unwrap();

        // expand each elf
        let elf1_low = elf1_s.split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let elf1_high = elf1_s.split("-").nth(1).unwrap().parse::<i32>().unwrap();
        let elf1_r: Vec<i32> = (elf1_low..elf1_high + 1).collect();
        let elf2_low = elf2_s.split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let elf2_high = elf2_s.split("-").nth(1).unwrap().parse::<i32>().unwrap();
        let elf2_r: Vec<i32> = (elf2_low..elf2_high + 1).collect();

        // println!("elf1: {:?}", &elf1_r);
        // println!("elf2: {:?}", &elf2_r);

        for i1 in elf1_r.iter() {
            // println!("checking {i1}");
            if elf2_r.contains(i1) {
                // println!("its in {:?}", elf2_r);
                overlaps += 1;
                break;
            }
        }

    }
    println!("{:?}", overlaps);
}
