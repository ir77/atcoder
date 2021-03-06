#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::io;

// fn main() {
//     let input = input_number();
//     let n = input[0] as usize;
//     let mut current_n = n;
//     let mut hash_set: HashSet<usize> = HashSet::new();
//
//     for i in 2..(n as f64).sqrt() as usize + 2 {
//         if current_n % i == 0 {
//             current_n /= i;
//             hash_set.insert(i);
//         }
//     }
//     if n > 1 && hash_set.len() == 0 {
//         hash_set.insert(n);
//     }
//     println!("{}", hash_set.len());
// }

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let mut current_n = n;
    let mut counter = 0usize;

    for i in 2..(n as f64).sqrt() as usize + 2 {
        if current_n % i == 0 {
            current_n /= i;
            counter += 1;

            let mut j = 2;
            while current_n % i.pow(j) == 0 {
                current_n /= i.pow(j);
                counter += 1;
                j += 1;
            }

            while current_n % i == 0 {
                current_n /= i;
            }
        }
    }
    if current_n > 1 {
        counter += 1;
    }
    println!("{}", counter);
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
fn input() -> (u64, f32) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    let mut iter = s.trim().split_whitespace();
    let c: u64 = iter.next().unwrap().parse().unwrap();
    let n: f32 = iter.next().unwrap().parse().unwrap();
    (c, n)
}
