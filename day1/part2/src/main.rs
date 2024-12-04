use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

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
    let second = input.iter().map(|x| x.1).collect::<Vec<i32>>();
    let mut second_map = HashMap::new();
    for item in second {
        if second_map.contains_key(&item) {
            let new = second_map.get(&item).unwrap() + 1;
            second_map.insert(item,new);
        } else {
            second_map.insert(item, 1);
        }
    }
    let zero = 0;
    let result: i32 = first.iter().map(|x| second_map.get(&x).unwrap_or(&zero) * x).sum();
    println!("{result}")
}
