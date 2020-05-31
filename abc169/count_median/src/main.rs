#[allow(unused_imports)]
use std::cmp::max;
use std::io;

/*
6
3 10
2 6
10 30
1 3
10 20
8 100
*/

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let mut input = (0..n).map(|_| input_number()).collect::<Vec<Vec<i64>>>();
    input.sort_by(|a, b| a[1].cmp(&b[1]));
    input.sort_by(|a, b| a[0].cmp(&b[0]));

    for i in 0..n {
        for j in 0..1000000000 {}
    }

    for i in 0..n {
        println!("{}, {}", input[i][0], input[i][1]);
    }
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
fn input() -> (u64, f32) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    let mut iter = s.trim().split_whitespace();
    let c: u64 = iter.next().unwrap().parse().unwrap();
    let n: f32 = iter.next().unwrap().parse().unwrap();
    (c, n)
}
