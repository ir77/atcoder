use std::io;

fn main() {
    // let input = input_number();
    // let n = input[0] as usize;
    // let m = input[1] as usize;
    // let l = input[2] as usize;
    // let nm: Vec<Vec<i64>> = (0..n).map(|_| input_number()).collect();
    // let ml: Vec<Vec<i64>> = (0..m).map(|_| input_number()).collect();

    // for i in 0..n {
    //     for j in 0..l {
    //         let sum = (0..m).fold(0, |sum, k| sum + nm[i][k] * ml[k][j]);
    //         print!("{}{}", sum, if j == l - 1 { '\n' } else { ' ' });
    //     }
    // }
    let input = input_number();
    let r = input[0];

    println!("{}", r * r);
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
