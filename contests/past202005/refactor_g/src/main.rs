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
    const START_X: i64 = 500;
    const START_Y: i64 = 500;
    let input = input_number();
    let n = input[0];
    let x = input[1];
    let y = input[2];
    let mut cost_map: Vec<Vec<i64>> = vec![vec![1000000; 1000]; 1000];

    for _ in 0..n {
        let input = input_number();
        let obstacle_x = input[0];
        let obstacle_y = input[1];
        cost_map[(obstacle_x + START_X) as usize][(obstacle_y + START_Y) as usize] = -100;
    }

    cost_map[START_X as usize][START_Y as usize] = 0;
    let mut checker: Vec<(usize, usize)> = Vec::new();
    checker.push((START_X as usize, START_Y as usize));

    loop {
        let mut new_checker: Vec<(usize, usize)> = Vec::new();
        for (i, j) in &checker {
            update_map(&mut cost_map, &mut new_checker, *i, *j, *i + 1, *j + 1);
            update_map(&mut cost_map, &mut new_checker, *i, *j, *i, *j + 1);
            update_map(&mut cost_map, &mut new_checker, *i, *j, *i - 1, *j + 1);
            update_map(&mut cost_map, &mut new_checker, *i, *j, *i + 1, *j);
            update_map(&mut cost_map, &mut new_checker, *i, *j, *i - 1, *j);
            update_map(&mut cost_map, &mut new_checker, *i, *j, *i, *j - 1);
        }
        let target_position = cost_map[(x + START_X) as usize][(y + START_Y) as usize];
        if target_position != 1000000 {
            println!("{}", target_position);
            break;
        }
        if new_checker.len() == 0 {
            println!("-1");
            break;
        }
        checker = new_checker;
    }
}

fn update_map(
    cost_map: &mut Vec<Vec<i64>>,
    new_checker: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
    next_x: usize,
    next_y: usize,
) {
    if next_x >= 1000 || next_y >= 1000 || next_x as i64 - 1 < 0 || next_y as i64 - 1 < 0 {
        return;
    }

    let next_cost = cost_map[next_x][next_y];
    if next_cost != -100 {
        let current_cost = cost_map[x][y] + 1;
        if next_cost > current_cost {
            cost_map[next_x][next_y] = current_cost;
            new_checker.push((next_x, next_y));
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
