use strum::IntoEnumIterator; 
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

impl Direction {
    fn new_points(&self, old_point: Point) -> Point {
        match *self {
            Direction::Up => Point {x: old_point.x, y: old_point.y - 1},
            Direction::UpRight => Point {x: old_point.x + 1, y: old_point.y - 1},
            Direction::Right => Point {x: old_point.x + 1, y: old_point.y},
            Direction::DownRight => Point {x: old_point.x + 1, y: old_point.y + 1},
            Direction::Down => Point {x: old_point.x, y: old_point.y + 1},
            Direction::DownLeft => Point {x: old_point.x - 1, y: old_point.y + 1},
            Direction::Left => Point {x: old_point.x - 1, y: old_point.y},
            Direction::UpLeft => Point {x: old_point.x - 1, y: old_point.y - 1}
        }
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let input = include_str!("../input.txt");
    let table: Vec<Vec<char>> = input.split('\n').map(|x| x.to_string().chars().collect()).collect();
    let total: i32 = table.iter().enumerate()
        .map(
            |(y, row )| 
            row.iter().enumerate()
                .map(
                    |(x, _)| 
                    find_xmas(&table, Point{x: x.try_into().unwrap(), y: y.try_into().unwrap()})
                )
                .sum::<i32>()
            )
        .sum();
    println!("{total}")
}

fn get_coord(table: &Vec<Vec<char>>, point: Point) -> Option<&char> {
    let x: usize = match point.x.try_into() {
        Ok(i) => i,
        Err(_) => {return None}
    };
    let y: usize = match point.y.try_into() {
        Ok(i) => i,
        Err(_) => {return None}
    };
    let row = match table.get(y) {
        None => {return None},
        Some(i) => i
    };
    row.get(x)
}

fn find_xmas(table: &Vec<Vec<char>>, cur_point: Point) -> i32 {
    if *get_coord(table, cur_point).expect("out of bounds") == 'X' {
        Direction::iter().map(|x| find_next(table, cur_point, x, 'M')).sum()
    } else {
        0
    }
}

fn find_next(table: &Vec<Vec<char>>, cur_point: Point, dir: Direction, target_letter: char) -> i32 {
    let new_point = dir.new_points(cur_point);
    let char = match get_coord(table, new_point) {
        Some(c) => c,
        None => {return 0;}
    };
    if *char == target_letter {
        return match target_letter {
            'M' => find_next(table, new_point, dir, 'A'),
            'A' => find_next(table, new_point, dir, 'S'),
            'S' => {return 1;},
            _ => panic!("Looking for an unknown letter")
        }
    }
    return 0;
}
