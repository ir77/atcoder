use proconio::input;
#[allow(unused_imports)]
use proconio::marker::Chars;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::f64::consts::PI;
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    // setup
    input! {
        n: usize,
        sn: [i64; n]
    }

    // exercise
    for i in 0..n {
        if i == 0 {
            print!("{}", sn[i]);
        } else {
            print!("{}", sn[i] - sn[i - 1]);
        }
        if i == n - 1 {
            print!(" \n");
        } else {
            print!(" ");
        }
    }
}

#[allow(dead_code)]
trait Transpose<'a, Elem, Iter, T>
    where
        Elem: 'a,
        Iter: IntoIterator<Item = &'a Elem>,
        T: IntoIterator<Item = Iter>,
{
    fn transpose(self) -> Transposed<'a, Elem, Iter>;
}

#[allow(dead_code)]
impl<'a, Elem, Iter, T> Transpose<'a, Elem, Iter, T> for T
    where
        Elem: 'a,
        Iter: IntoIterator<Item = &'a Elem>,
        T: IntoIterator<Item = Iter>,
{
    fn transpose(self) -> Transposed<'a, Elem, Iter> {
        Transposed {
            iters: self.into_iter().map(IntoIterator::into_iter).collect(),
        }
    }
}

#[allow(dead_code)]
struct Transposed<'a, Elem, Iter>
    where
        Elem: 'a,
        Iter: IntoIterator<Item = &'a Elem>,
{
    iters: Vec<Iter::IntoIter>,
}

#[allow(dead_code)]
impl<'a, Elem, Iter> Iterator for Transposed<'a, Elem, Iter>
    where
        Elem: 'a,
        Iter: IntoIterator<Item = &'a Elem>,
{
    type Item = Vec<&'a Elem>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iters.iter_mut().map(Iterator::next).collect()
    }
}

#[allow(dead_code)]
fn get_radian_from(degree: f64) -> f64 {
    (degree * PI) / 180.0
}

#[allow(dead_code)]
fn get_alphabet() -> Vec<char> {
    (0..26).map(|x| (x + b'a') as char).collect::<Vec<char>>()
}

#[allow(dead_code)]
fn get_binary_string(digit: usize) -> Vec<Vec<String>> {
    (0..2i64.pow(digit as u32))
        .map(|x| format!("{:0>1$b}", x, digit))
        .map(|x| x.chars().map(|x| x.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<_>>>()
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

/// https://github.com/bluss/permutohedron/blob/master/src/lexical.rs
pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i - 1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }
}
