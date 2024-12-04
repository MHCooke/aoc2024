use itertools::Itertools;
use num::abs;
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = contents
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .tuples::<(i32, i32)>()
        .collect::<Vec<(i32, i32)>>();
    let mut first = input.iter().map(|x| x.0).collect::<Vec<i32>>();
    first.sort();
    let mut second = input.iter().map(|x| x.1).collect::<Vec<i32>>();
    second.sort();
    let result: i32 = zip(first.iter(), second.iter())
        .map(|x| abs(x.0 - x.1))
        .sum();
    println!("{result}")
}
