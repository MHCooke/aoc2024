use std::{fs::File, io::Read};
use regex::Regex;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = "do()".to_string();
    file.read_to_string(&mut contents).unwrap();
    contents += "don't()";

    let do_regex = Regex::new(r"do\(\)\p{any}+?don't\(\)").expect("do regex is invalid"); // using `\p{any}` here since it matches newlines too
    let new_string: String = do_regex.captures_iter(&contents).map(|c| c.extract::<0>().0).collect::<String>();

    let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Invalid regex");
    let result: i32 = mul_regex.captures_iter(&new_string)
        .map(
            |c| 
            c.extract::<2>().1
            .map(|x| x.parse::<i32>().unwrap())
            .iter()
            .product::<i32>()
        ).sum();
    println!("{result}")
}
