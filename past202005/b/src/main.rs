#[allow(unused_imports)]
use std::cmp::max;
use std::io;

fn main() {
    let number = input_number();
    let nCount = number[0] as usize; // 参加者
    let mCount = number[1] as usize; // 問題数→何度解かれたか管理する

    let mut mVec = vec![nCount; mCount];
    let mut nVec = vec![vec![0; mCount]; nCount];

    let q = number[2] as usize; // クエリ数

    for _ in 0..q {
        let query = input_number();
        let queryType = query[0];
        let n = query[1] as usize;

        if queryType == 1 {
            //出力が必要
            let mut sum = 0;
            for (index, value) in nVec[n - 1].iter().enumerate() {
                if *value != 0 {
                    sum += mVec[index];
                }
            }
            println!("{}", sum);
        } else {
            // arrayの更新が必要
            let m = query[2] as usize;
            nVec[n - 1][m - 1] += 1;
            mVec[m - 1] -= 1;
        }
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
