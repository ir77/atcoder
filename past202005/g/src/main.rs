#[allow(unused_imports)]
use std::cmp::max;
use std::io;
#[allow(unused_imports)]
use std::str::Chars;

/*
 1000000: 初期値
 -100: 障害物
*/
fn main() {
    let input = input_number();
    let n = input[0];
    let x = input[1];
    let y = input[2];
    let mut cost_map: Vec<Vec<i64>> = vec![vec![1000000; 1000]; 1000];
    for _ in 0..n {
        let input = input_number();
        let obstacle_x = input[0];
        let obstacle_y = input[1];
        cost_map[(obstacle_x + 500) as usize][(obstacle_y + 500) as usize] = -100;
    }
    cost_map[500][500] = 0;
    let mut checker: Vec<(usize, usize)> = Vec::new();
    checker.push((500, 500));

    loop {
        if cost_map[(x + 500) as usize][(y + 500) as usize] != 1000000 {
            println!("{}", cost_map[(x + 500) as usize][(y + 500) as usize]);
            break;
        }
        let mut new_checker: Vec<(usize, usize)> = Vec::new();
        for (i, j) in &checker {
            if i + 1 < 1000 && j + 1 < 1000 && cost_map[i + 1][j + 1] != -100 {
                if cost_map[i + 1][j + 1] > cost_map[*i][*j] + 1 {
                    cost_map[i + 1][j + 1] = cost_map[*i][*j] + 1;
                    new_checker.push((i + 1, j + 1));
                }
            }
            if j + 1 < 1000 && cost_map[*i][j + 1] != -100 {
                if cost_map[*i][j + 1] > cost_map[*i][*j] + 1 {
                    cost_map[*i][j + 1] = cost_map[*i][*j] + 1;
                    new_checker.push((*i, j + 1));
                }
            }
            if *i as i64 - 1 >= 0 && j + 1 < 1000 && cost_map[i - 1][j + 1] != -100 {
                if cost_map[i - 1][j + 1] > cost_map[*i][*j] + 1 {
                    cost_map[i - 1][j + 1] = cost_map[*i][*j] + 1;
                    new_checker.push((i - 1, j + 1));
                }
            }
            if i + 1 < 1000 && cost_map[i + 1][*j] != -100 {
                if cost_map[i + 1][*j] > cost_map[*i][*j] + 1 {
                    cost_map[i + 1][*j] = cost_map[*i][*j] + 1;
                    new_checker.push((i + 1, *j));
                }
            }
            if *i as i64 - 1 >= 0 && cost_map[i - 1][*j] != -100 {
                if cost_map[i - 1][*j] > cost_map[*i][*j] + 1 {
                    cost_map[i - 1][*j] = cost_map[*i][*j] + 1;
                    new_checker.push((i - 1, *j));
                }
            }
            if *j as i64 - 1 >= 0 && cost_map[*i][j - 1] != -100 {
                if cost_map[*i][j - 1] > cost_map[*i][*j] + 1 {
                    cost_map[*i][j - 1] = cost_map[*i][*j] + 1;
                    new_checker.push((*i, j - 1));
                }
            }
        }
        if cost_map[(x + 500) as usize][(y + 500) as usize] != 1000000 {
            println!("{}", cost_map[(x + 500) as usize][(y + 500) as usize]);
            break;
        }
        if new_checker.len() == 0 {
            println!("-1");
            break;
        }
        checker = new_checker;
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
