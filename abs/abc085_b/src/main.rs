#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    let n = get_vec_input::<usize>()[0];
    let dn: HashSet<u64> = (0..n).map(|_| get_vec_input::<u64>()[0]).collect();
    println!("{}", dn.len());
}

#[allow(dead_code)]
fn sum_digit(value: i64) -> i64 {
    if value < 10 {
        return value;
    }
    return sum_digit(value / 10) + value % 10;
}

#[allow(dead_code)]
fn print_vecvec<T>(value: Vec<Vec<T>>)
where
    T: fmt::Display,
{
    let values = value.iter().map(|x| x);
    for value in values {
        value.iter().for_each(|x| print!("{} ", x));
        println!();
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
fn get_vec_input<T>() -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let vec: Vec<T> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    vec
}

#[allow(dead_code)]
fn get_tuple_input() -> (u64, f32) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    let mut iter = s.trim().split_whitespace();
    let c: u64 = iter.next().unwrap().parse().unwrap();
    let n: f32 = iter.next().unwrap().parse().unwrap();
    (c, n)
}
