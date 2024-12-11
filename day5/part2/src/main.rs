use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("input.txt");
    let (rules_str, pages_str) = input.split_once("\n\n").unwrap();
    let rules_list: Vec<Vec<u32>> = rules_str.split('\n').map(|x| x.splitn(2, '|').map(|i| i.parse::<u32>().unwrap()).collect()).collect();
    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for order in rules_list {
        let list = rules_map.entry(order[0]).or_default();
        list.push(order[1]);
    }
    let pages_list: Vec<Vec<u32>> = pages_str.split('\n').filter(|l| l.len() > 0).map(|l| l.split(',').map(|i| i.parse::<u32>().expect(format!("`{i}` from `{l}`").as_str())).collect()).collect();
    let result: u32 = pages_list.iter().map(|pages|
        pages.iter().enumerate().all(|(cur_page_index, page)|
            rules_map.get(page).map_or(true, |rules|
                (&pages[cur_page_index + 1..]).iter().all(|page| rules.contains(page)) 
                &&
                (&pages[..cur_page_index]).iter().all(|page| !rules.contains(page))
            )
        ).then(|| false).unwrap_or(true)  // this feels stupid but i don't know how else to invert it...
        .then(|| {
            let mut new_pages = pages.to_vec(); 
            new_pages.sort_by(|a, b| cmp_page(a, b, &rules_map));
            new_pages.get(new_pages.len()/2).expect("wrong centre").clone()
        }).unwrap_or(0)
    ).sum();
    println!("{result}");    
}

fn cmp_page(page1: &u32, page2: &u32, rules_map: &HashMap<u32, Vec<u32>>) -> Ordering { 
    if let Some(rules) = rules_map.get(page1) {
        if rules.contains(page2) {
            return Ordering::Less;
        };
    };
    if let Some(rules) = rules_map.get(page2) {
        if rules.contains(page1) {
            return Ordering::Greater;
        };
    };
    return Ordering::Equal;
}
