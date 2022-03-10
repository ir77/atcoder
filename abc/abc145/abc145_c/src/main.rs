#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use std::cmp::min;
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    // setup
    let n: usize = get_vec_input()[0];
    let mut pattern = (0..n).collect::<Vec<usize>>();
    let mut patterns: Vec<Vec<usize>> = Vec::new();
    loop {
        patterns.push(pattern.clone());
        if !pattern.next_permutation() {
            break;
        }
    }
    let cities = (0..n).map(|_| get_vec_input()).collect::<Vec<Vec<i64>>>();

    // patternごとに街間の距離を計算する
    let mut distance_sum = 0f64;
    for pattern in &patterns {
        let distance = (1..n)
            .map(|x| (pattern[x - 1], pattern[x]))
            .map(|(pre, next)| (cities[pre].clone(), cities[next].clone()))
            .map(|(pre, next)| (next[0] - pre[0], next[1] - pre[1]))
            .map(|(x_diff, y_diff)| (x_diff.abs(), y_diff.abs()))
            .map(|(x_diff, y_diff)| (x_diff.pow(2), y_diff.pow(2)))
            .map(|(x_diff, y_diff)| x_diff as f64 + y_diff as f64)
            .map(|x| x.sqrt())
            .fold(0.0, |a, b| a + b);
        distance_sum += distance;
        // for i in 1..n {
        //     let pre_index = pattern[i - 1];
        //     let next_index = pattern[i];
        //     let pre_city = cities[pre_index].clone();
        //     let next_city = cities[next_index].clone();
        //     let x_diff = (next_city[0] - pre_city[0]).abs();
        //     let y_diff = (next_city[1] - pre_city[1]).abs();
        //     let distance = (x_diff.pow(2) as f64 + y_diff.pow(2) as f64).sqrt();
        //     distance_sum += distance;
        // }
    }
    let answer = distance_sum / patterns.len() as f64;
    println!("{}", answer);
}

#[allow(dead_code)]
fn get_binary_string(digit: usize) -> Vec<String> {
    (0..2i64.pow(digit as u32))
        .map(|x| format!("{:0>1$b}", x, digit))
        .collect::<Vec<String>>()
}

#[allow(dead_code)]
fn check_prime(value: i64) -> bool {
    if value == 2 {
        return true;
    }
    if value % 2 == 0 {
        return false;
    }
    for i in 3..(value as f64).sqrt().floor() as i64 {
        if value % i == 0 {
            return false;
        }
    }
    return true;
}

#[allow(dead_code)]
fn get_digit(value: i64) -> i64 {
    let mut digit = 0i64;
    let mut value = value;
    while value != 0 {
        value /= 10;
        digit += 1;
    }
    digit
}

#[allow(dead_code)]
fn sum_digit2(value: String) -> u32 {
    return value.chars().fold(0u32, |a, b| a + b.to_digit(10).unwrap());
}

#[allow(dead_code)]
fn sum_digit(value: i64) -> i64 {
    if value < 10 {
        return value;
    }
    return sum_digit(value / 10) + value % 10;
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

#[allow(dead_code)]
fn get_string_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim().to_string()
}

/// https://github.com/bluss/permutohedron/blob/master/src/lexical.rs
pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i - 1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }
}
