use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Steep
}

fn is_safe(seq: String) -> bool {
    let nums = seq.split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    nums.windows(2).map(|x| match x[0] - x[1] {1..=3 => Direction::Down, -3..=-1 => Direction::Up, _ => Direction::Steep}).all_equal()
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let result = contents.split('\n').filter(|x| is_safe(x.to_string())).count();
    println!("{result}");
}