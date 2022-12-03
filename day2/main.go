package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func part1() {
	input, _ := ioutil.ReadFile("input.txt")

	point_values := make(map[string]int)
	point_values["rock"] = 1
	point_values["paper"] = 2
	point_values["scissors"] = 3

	opponent := make(map[string]string)
	opponent["A"] = "rock"
	opponent["B"] = "paper"
	opponent["C"] = "scissors"

	player := make(map[string]string)
	player["X"] = "rock"
	player["Y"] = "paper"
	player["Z"] = "scissors"

	score := 0

	for _, line := range strings.Split(string(input), "\n") {
		if line == "" {
			continue
		}

		o := ""
		p := ""

		for _, char := range line {
			if string(char) == " " {
				continue
			}
			if o == "" {
				o = string(char)
				continue
			}
			if p == "" {
				p = string(char)
				continue
			}
		}
		// fmt.Printf("score:%v\n", score)
		// fmt.Printf("opponent:%s=>%s player:%s=>%s\n", o, opponent[o], p, player[p])

		playscore := point_values[player[p]]

		switch o {
		case "A": // opponent is rock
			switch p {
			case "X": // player is rock
				playscore += 3
			case "Y": // player is paper
				playscore += 6
			case "Z": // player is scissors
				playscore += 0
			}
		case "B": // opponent is paper
			switch p {
			case "X": // player is rock
				playscore += 0
			case "Y": // player is paper
				playscore += 3
			case "Z": // player is scissors
				playscore += 6
			}
		case "C": // opponent is scissors
			switch p {
			case "X": // player is rock
				playscore += 6
			case "Y": // player is paper
				playscore += 0
			case "Z": // player is scissors
				playscore += 3
			}
		}

		score += playscore
	}

	fmt.Println(score)
}

func part2() {
	input, _ := ioutil.ReadFile("input.txt")

	point_values := make(map[string]int)
	point_values["rock"] = 1
	point_values["paper"] = 2
	point_values["scissors"] = 3

	opponent := make(map[string]string)
	opponent["A"] = "rock"
	opponent["B"] = "paper"
	opponent["C"] = "scissors"

	strategy := make(map[string]string)
	strategy["X"] = "lose"
	strategy["Y"] = "draw"
	strategy["Z"] = "win"

	score := 0

	for _, line := range strings.Split(string(input), "\n") {
		if line == "" {
			continue
		}

		o := ""
		s := ""

		for _, char := range line {
			if string(char) == " " {
				continue
			}
			if o == "" {
				o = string(char)
				continue
			}
			if s == "" {
				s = string(char)
				continue
			}
		}
		// fmt.Printf("score:%v\n", score)
		// fmt.Printf("opponent:%s=>%s strategy:%s=>%s\n", o, opponent[o], s, strategy[s])

		playscore := 0
		switch strategy[s] {
		case "lose":
			playscore = 0
		case "draw":
			playscore = 3
		case "win":
			playscore = 6
		}

		switch o {
		case "A": //opponent is rock
			switch s {
			case "X": // strategy is lose
				playscore += point_values["scissors"]
			case "Y": // strategy is draw
				playscore += point_values["rock"]
			case "Z": // strategy is win
				playscore += point_values["paper"]
			}
		case "B": //opponent is paper
			switch s {
			case "X": // strategy is lose
				playscore += point_values["rock"]
			case "Y": // strategy is draw
				playscore += point_values["paper"]
			case "Z": // strategy is win
				playscore += point_values["scissors"]
			}
		case "C": //opponent is scissors
			switch s {
			case "X": // strategy is lose
				playscore += point_values["paper"]
			case "Y": // strategy is draw
				playscore += point_values["scissors"]
			case "Z": // strategy is win
				playscore += point_values["rock"]
			}
		}
		score += playscore
	}

	fmt.Println(score)
}

func main() {
	// part1()
	part2()
}

// use std::{collections::HashMap, fs};
//
// fn main() {
//     let input = fs::read_to_string("example.txt").unwrap();
//
//     let mut point_values = HashMap::new();
//     point_values.insert("rock", 1);
//     point_values.insert("paper", 2);
//     point_values.insert("scissors", 3);
//
//     let mut opponent = HashMap::new();
//     opponent.insert("A", "rock");
//     opponent.insert("B", "paper");
//     opponent.insert("C", "scissors");
//
//     let mut player = HashMap::new();
//     player.insert("X", "rock");
//     player.insert("Y", "paper");
//     player.insert("Z", "scissors");
//
//     let mut score = 0;
//
//     for line in input.lines().into_iter() {
//         let mut o = "".to_owned();
//         let mut p = "".to_owned();
//         for char in line.chars().into_iter() {
//             if char.to_string() == " " {
//                 continue;
//             }
//             if o == "".to_string() {
//                 o = char.to_string();
//                 continue;
//             }
//             if p == "".to_string() {
//                 p = char.to_string();
//                 continue;
//             }
//         }
//         println!("opponent:{} player:{}", o, p);
//
//         let playscore = point_values[&p];
//
//         match o.to_string() {
//             "A" => match p.to_string() {
//                 "X" => score
//             }
//         }
//     }
// }
//
