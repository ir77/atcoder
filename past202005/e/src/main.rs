#[allow(unused_imports)]
use std::cmp::max;
use std::io;
#[allow(unused_imports)]
use std::str::Chars;

fn main() {
    let input = input_number();
    let _n = input[0] as usize;
    let m = input[1] as usize;
    let q = input[2] as usize;

    let mut relation: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        let input = input_number();
        relation.push((input[0] as usize, input[1] as usize));
    }

    let mut colors = input_number();

    for _ in 0..q {
        let query = input_number();
        let query_type = query[0];

        let x = query[1] as usize;
        println!("{}", colors[x - 1]);

        if query_type == 1 {
            for (a, b) in &relation {
                if x == *a {
                    colors[*b - 1] = colors[x - 1];
                }
                if x == *b {
                    colors[*a - 1] = colors[x - 1];
                }
            }
        } else {
            let y = query[2];
            colors[x - 1] = y;
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
