#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::io;

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let s = input[0] as usize;
    let an = input_number();
    let mod_number = 998244353i64;
    let mut dp = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..s + 1 {
            dp[i + 1][j] += 2 * dp[i][j];
            dp[i + 1][j] %= mod_number;
            if j + an[i] as usize <= s {
                dp[i + 1][j + an[i] as usize] += dp[i][j];
                dp[i + 1][j + an[i] as usize] %= mod_number;
            }
        }
    }
    print_vecvec(dp);
    // println!("{}", dp[n][s]);
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
