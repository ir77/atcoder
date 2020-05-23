#[allow(unused_imports)]
use std::cmp::max;
use std::io;
#[allow(unused_imports)]
use std::str::Chars;

// a = 97, z = 122

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let mut string: String = "".to_string();
    for _ in 0..n {
        string += &input_string();
    }

    let mut raw_alphabet_count: Vec<i64> = vec![0; 26];

    for c in string.chars() {
        let c_number = c as usize;
        raw_alphabet_count[c_number - 97] += 1;
    }

    let mut pre_string: String = String::new();
    let needs_pair_count = n / 2;
    let pair_count: i64 = raw_alphabet_count.iter().map(|x| x / 2).sum();
    let mut alphabet_count: Vec<i64> = raw_alphabet_count.iter().map(|x| x / 2).collect();
    if n % 2 == 0 {
        if needs_pair_count > pair_count as usize {
            println!("-1");
        }

        for _ in 0..(n / 2) {
            for i in 0..26 {
                if alphabet_count[i] > 0 {
                    let c = (i + 97) as u8 as char;
                    print!("{}", c);
                    pre_string += &c.to_string();
                    alphabet_count[i] -= 1;
                    break;
                }
            }
        }
    } else {
        let remainder_count: i64 = alphabet_count.iter().map(|x| x % 2).sum();

        if needs_pair_count > pair_count as usize && remainder_count > 0 {
            println!("-1");
        }

        let mut remainder_alphabet_count: Vec<i64> =
            raw_alphabet_count.iter().map(|x| x % 2).collect();

        for _ in 0..(n / 2) {
            for i in 0..26 {
                if alphabet_count[i] > 0 {
                    let c = (i + 97) as u8 as char;
                    print!("{}", c);
                    pre_string += &c.to_string();
                    alphabet_count[i] -= 1;
                    break;
                }
            }
        }
        for i in 0..26 {
            if remainder_alphabet_count[i] > 0 {
                let c = (i + 97) as u8 as char;
                print!("{}", c);
                remainder_alphabet_count[i] -= 1;
                break;
            }
        }
    }
    print!("{}", pre_string.chars().rev().collect::<String>());
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
