#[allow(unused_imports)]
use std::cmp::max;
use std::io;

fn main() {
    let number = input_number();
    let n_count = number[0] as usize; // 参加者
    let m_count = number[1] as usize; // 問題数→何度解かれたか管理する

    let mut m_vec = vec![n_count; m_count];
    let mut n_vec = vec![vec![0; m_count]; n_count];

    let q = number[2] as usize; // クエリ数

    for _ in 0..q {
        let query = input_number();
        let query_type = query[0];
        let n = query[1] as usize;

        if query_type == 1 {
            //出力が必要
            let index_list: Vec<usize> = n_vec[n - 1]
                .iter()
                .enumerate()
                .filter(|&(_, value)| *value != 0)
                .map(|(index, _)| index)
                .collect();
            let sum: usize = index_list.iter().map(|&index| m_vec[index]).sum();
            println!("{}", sum);
        } else {
            // arrayの更新が必要
            let m = query[2] as usize;
            n_vec[n - 1][m - 1] += 1;
            m_vec[m - 1] -= 1;
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
