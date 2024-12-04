use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Steep
}

fn is_safe(nums: &Vec<i32>) -> bool { 
    let directions = nums.windows(2)
        .map(|x| 
            match x[0] - x[1] {
                1..=3 => Direction::Down, 
                -3..=-1 => Direction::Up, 
                _ => Direction::Steep
            })
        .collect::<Vec<Direction>>();

    !directions.contains(&Direction::Steep) && directions.iter().all_equal()
}

fn test_less_one(seq: String) -> bool {
    let nums = seq.split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if is_safe(&nums) {
        return true;
    }
    for i in 0..nums.len() {
        let mut shorter_seq = nums.to_vec();
        shorter_seq.remove(i);
        if is_safe(&shorter_seq) {
            return true;
        }
    }
    return false;
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let result = contents.split('\n').filter(|x| test_less_one(x.to_string())).count();
    println!("{result}");
}