#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use std::cmp::min;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::f64::consts::PI;
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    let input = get_vec_input();
    let n = input[0];
    let m = input[1];
    let roads = (0..m).map(|_| get_vec_input()).collect::<Vec<Vec<usize>>>();

    // 幅優先探索
    let mut answer: Vec<usize> = Vec::new();
    'out: for target in 2..=n {
        let mut roads = roads.clone();
        let mut queue: Vec<usize> = vec![1; 1];

        'ok: loop {
            // println!("target => {}, {:?}, {:?}", target, queue, roads);
            let mut next_queue: Vec<usize> = Vec::new();
            let mut remove_index: Vec<usize> = Vec::new();

            for (i, road) in roads.iter().enumerate() {
                if queue.contains(&road[0]) {
                    if road[1] == target {
                        answer.push(road[0]);
                        // println!("1: {}, {}, {}, {}", i, road[0], road[1], target);
                        break 'ok;
                    }
                    next_queue.push(road[1]);
                    remove_index.push(i);
                } else if queue.contains(&road[1]) {
                    if road[0] == target {
                        answer.push(road[1]);
                        // println!("2: {}, {}, {}, {}", i, road[0], road[1], target);
                        break 'ok;
                    }
                    next_queue.push(road[0]);
                    remove_index.push(i);
                }
            }
            if remove_index.len() == 0 {
                break 'out;
            }
            for i in remove_index.iter().rev() {
                roads.remove(*i);
            }
            queue = next_queue;
        }
    }
    if answer.len() != n - 1 {
        println!("No");
    } else {
        println!("Yes");
        answer.iter().for_each(|x| println!("{}", x));
    }
}

#[allow(dead_code)]
fn get_radian_from(degree: f64) -> f64 {
    (degree * PI) / 180.0
}

#[allow(dead_code)]
fn get_alphabet() -> Vec<char> {
    (0..26).map(|x| (x + b'a') as char).collect::<Vec<char>>()
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