use std::{fs::File, io::Read};
use regex::Regex;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Invalid regex");
    let result: i32 = re.captures_iter(&contents)
        .map(
            |c| 
            c.extract::<2>().1
            .map(|x| x.parse::<i32>().unwrap())
            .iter()
            .product::<i32>()
        ).sum();
    println!("{result}")
}
