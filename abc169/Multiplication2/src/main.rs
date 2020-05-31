#[allow(unused_imports)]
use std::cmp::max;
use std::io;

fn main() {
    let input = input_number();
    let _n = input[0] as usize;
    let ai = input_number();

    let check = ai.iter().find(|x| **x == 0);
    if let Some(_) = check {
        println!("0");
        return;
    }

    let mut result = 1u64;
    let checker = 1000000000000000000u64;
    for value in ai {
        if result > checker {
            println!("-1");
            return;
        }
        if checker / result < value {
            println!("-1");
            return;
        }
        result = result * value;
    }
    println!("{}", result);
}

#[allow(dead_code)]
fn update_max<T>(left: &mut T, right: T)
where
    T: std::cmp::Ord + std::clone::Clone,
{
    if *left < right {
        *left = right.clone();
    }
}

#[allow(dead_code)]
fn update_min<T>(left: &mut T, right: T)
where
    T: std::cmp::Ord + std::clone::Clone,
{
    if *left > right {
        *left = right.clone();
    }
}

#[allow(dead_code)]
fn input_number() -> Vec<u64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let vec: Vec<u64> = input
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
