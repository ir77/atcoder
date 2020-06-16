#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use std::cmp::min;
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
    // let n: usize = get_vec_input()[0];
    // let s = get_string_input();

    // let mut answer: HashSet<String> = HashSet::new();
    // for i in 0..n - 2 {
    //     for j in i + 1..n - 1 {
    //         for k in j + 1..n {
    //             if k != i {
    //                 answer.insert(s[i..i + 1].to_string() + &s[j..j + 1] + &s[k..k + 1]);
    //             }
    //         }
    //     }
    // }
    // println!("{}", answer.len());

    let n: usize = get_vec_input()[0];
    let s = get_string_input();

    let mut answer = 0i64;
    for x in 0..999 + 1 {
        let target = [x / 100, (x / 10) % 10, x % 10];

        let mut counter = 0usize;
        for i in 0..n {
            if s[i..i + 1] == target[counter].to_string() {
                counter += 1;
            }
            if counter == 3 {
                answer += 1;
                break;
            }
        }
    }
    println!("{}", answer);
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
