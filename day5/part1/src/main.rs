fn main() {
    let input = include_str!("input.txt");
    let (rules_str, pages_str) = input.split_once("\n\n").unwrap();
    let rules: Vec<Vec<u32>> = rules_str.split('\n').map(|x| x.splitn(2, '|').map(|i| i.parse::<u32>().unwrap()).collect()).collect();
}
