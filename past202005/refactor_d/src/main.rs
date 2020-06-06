#[allow(unused_imports)]
use std::cmp::max;
use std::io;
use std::str::Chars;

fn main() {
    let number = input_number();
    let n = number[0] as usize;
    let mut lights: Vec<String> = vec!["".to_string(); 5];
    for i in 0..5 {
        lights[i] = input_string();
    }

    for i in 0..n {
        let row0 = &lights[0][i * 4 + 1..=i * 4 + 3];
        let row1 = &lights[1][i * 4 + 1..=i * 4 + 3];
        let row2 = &lights[2][i * 4 + 1..=i * 4 + 3];
        let row3 = &lights[3][i * 4 + 1..=i * 4 + 3];
        let row4 = &lights[4][i * 4 + 1..=i * 4 + 3];
        // 6, 8, 9式で分岐させて書いていくべきだった
        if &row1[1..2] == "." && &row2[1..2] == "." && &row3[1..2] == "." && &row4[1..2] == "." {
            print!("7");
        } else if &row1[1..2] == "." && &row2[1..2] == "." && &row3[1..2] == "." {
            print!("0");
        } else if &row0[2..3] == "." && &row1[2..3] == "." && &row2[2..3] == "." {
            print!("1");
        } else if &row2[2..3] == "#" && &row3[2..3] == "." && &row4[2..3] == "#" {
            print!("2");
        } else if &row1[0..1] == "." && &row2[0..1] == "#" && &row3[0..1] == "." {
            print!("3");
        } else if &row0[1..2] == "." && &row1[1..2] == "." && &row2[1..2] == "#" {
            print!("4");
        } else if &row1[2..3] == "." && &row3[0..1] == "." {
            print!("5");
        } else if row0 == "###" && row1 == "#.." {
            print!("6");
        } else if row1 == "#.#" && row3 == "#.#" {
            print!("8");
        } else if row1 == "#.#" && row3 == "..#" {
            print!("9");
        }
    }
}

#[allow(dead_code)]
fn input_number() -> Vec<i64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let vec: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap_or(0))
        .collect();
    vec
}

#[allow(dead_code)]
fn input() -> (char, usize) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    let mut iter = s.trim().split_whitespace();
    let c: char = iter.next().unwrap().chars().nth(0).unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    (c, n)
}

#[allow(dead_code)]
fn input_string() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    let mut iter = s.trim().split_whitespace();
    let s: String = iter.next().unwrap().to_string();
    s
}
