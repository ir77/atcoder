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
    let input = get_vec_input();
    let n: usize = input[0];
    let k: usize = input[1];
    let mut lights: Vec<i64> = get_vec_input();

    for _ in 0..k {
        // let mut tmp_lights: Vec<i64> = vec![0i64; n];
        // for i in 0..n {
        //     let light = lights[i];
        //     let start = i as i64 - light;
        //     let end = i as i64 + light;
        //     let range = start..end + 1;

        //     range
        //         .filter(|x| *x >= 0 && *x < n as i64)
        //         .for_each(|x| tmp_lights[x as usize] += 1);
        // }

        // 累積和
        let mut tmp_lights: Vec<i64> = vec![0i64; n + 1];
        for i in 0..n {
            let start = max(0, i as i64 - lights[i]) as usize;
            let end = min(n as i64 - 1, i as i64 + lights[i]) as usize;
            tmp_lights[start] += 1;
            tmp_lights[end + 1] -= 1;
        }
        for i in 0..n - 1 {
            tmp_lights[i + 1] += tmp_lights[i];
        }
        tmp_lights.pop();

        lights = tmp_lights.clone();
        if lights
            .iter()
            .filter(|&x| *x == n as i64)
            .collect::<Vec<_>>()
            .len()
            == n
        {
            break;
        }
    }
    lights.iter().for_each(|x| print!("{} ", x));
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
