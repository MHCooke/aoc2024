enum Direction {
    UpRight,
    DownRight,
    DownLeft,
    UpLeft
}

impl Direction {
    fn new_points(&self, old_point: Point) -> Point {
        match *self {
            Direction::UpRight => Point {x: old_point.x + 1, y: old_point.y - 1},
            Direction::DownRight => Point {x: old_point.x + 1, y: old_point.y + 1},
            Direction::DownLeft => Point {x: old_point.x - 1, y: old_point.y + 1},
            Direction::UpLeft => Point {x: old_point.x - 1, y: old_point.y - 1}
        }
    }
}


#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

const DIAGS: [(Direction, Direction); 2] = [
    (Direction::UpLeft, Direction::DownRight),
    (Direction::DownLeft, Direction::UpRight)
];

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

fn compare_point(table: &Vec<Vec<char>>, point: Point, letter: char) -> bool {
    match get_coord(table, point) {
        Some(c) => *c == letter,
        None => false
    }
}

fn find_xmas(table: &Vec<Vec<char>>, cur_point: Point) -> i32 {
    if *get_coord(table, cur_point).expect("out of bounds") == 'A' {
        let mut total = 0;
        for (first, second) in DIAGS {
            if (compare_point(table, first.new_points(cur_point), 'M') && compare_point(table, second.new_points(cur_point), 'S')) ||
            (compare_point(table, first.new_points(cur_point), 'S') && compare_point(table, second.new_points(cur_point), 'M')) {
                total += 1;
            } else {
                total += 0;
            }
        }
        return if total == 2 {1} else {0};
    } else {
        0
    }
}
