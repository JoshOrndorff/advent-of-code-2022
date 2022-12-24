// 101054 is too high
// 46226 is too high

use Motion::*;
#[derive(Debug)]
enum Motion {
    Walk(u32),
    Right,
    Left,
}

use Direction::*;
#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Into<isize> for Direction {
    fn into(self) -> isize {
        match self {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        }
    }
}

fn route_from_string(s: &str) -> Vec<Motion> {
    let mut route = Vec::<Motion>::new();

    let mut digit_buffer = String::new();
    for c in s.chars() {
        match c {
            'R' => {
                let steps = digit_buffer.parse().unwrap();
                route.push(Walk(steps));
                route.push(Right);
                digit_buffer = String::new();
            }
            'L' => {
                let steps = digit_buffer.parse().unwrap();
                route.push(Walk(steps));
                route.push(Left);
                digit_buffer = String::new();
            }
            digit => {
                digit_buffer.push(digit);
            }
        }
    }

    let steps = digit_buffer.parse().unwrap();
    route.push(Walk(steps));

    route
}
fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("read input file");
    let mut input = input.split("\n\n");

    let grove_part = input.next().unwrap();
    let grove: Vec<Vec<char>> = grove_part.lines().map(|l| l.chars().collect()).collect();

    let route = route_from_string(input.next().unwrap().trim());

    drop(input);

    // Calculate the maximum width of the grove
    // Not all the rows are full width in the input
    let mut max_width = 0;
    for row in &grove {
        if row.len() > max_width {
            max_width = row.len();
        }
    }
    let max_width = max_width;

    // Find the bounds of each row
    let mut row_bounds = Vec::<(usize, usize)>::new();
    for i in 0..grove.len() {
        let mut min = None;
        let mut max = None;

        for j in 0..=max_width {
            let c = *grove[i].get(j).unwrap_or(&' ');

            if min.is_none() && c == ' ' {
                continue;
            } else if min.is_none() && c != ' ' {
                min = Some(j);
            } else if min.is_some() && c != ' ' {
                continue;
            } else {
                max = Some(j - 1);
                break;
            }
        }
        row_bounds.push((min.unwrap(), max.unwrap()));
    }
    let row_bounds = row_bounds; // so it is no longer mutable

    // Find the bounds for each column
    let mut col_bounds = Vec::<(usize, usize)>::new();
    for j in 0..max_width {
        let mut min = None;
        let mut max = None;

        for i in 0..grove.len() + 1 {
            let c = *grove
                .get(i)
                .map(|row| row.get(j).unwrap_or(&' '))
                .unwrap_or(&' ');

            if min.is_none() && c == ' ' {
                continue;
            } else if min.is_none() && c != ' ' {
                min = Some(i);
            } else if min.is_some() && c != ' ' {
                continue;
            } else {
                max = Some(i - 1);
                break;
            }
        }
        col_bounds.push((min.unwrap(), max.unwrap()));
    }
    let col_bounds = col_bounds; // so it is no longer mutable

    println!("{:?}", row_bounds);
    println!("{:?}", col_bounds);

    
    let mut heading = East;
    let mut position = (0isize, row_bounds[0].0 as isize);

    ///////////////// Start actually wandering
    for motion in route {
        match (motion, heading) {
            (Right, East) => heading = South,
            (Right, South) => heading = West,
            (Right, West) => heading = North,
            (Right, North) => heading = East,
            (Left, East) => heading = North,
            (Left, North) => heading = West,
            (Left, West) => heading = South,
            (Left, South) => heading = East,
            (Walk(steps), East) => {
                for _ in 0..steps {
                    let i = position.0;
                    let mut j = position.1 + 1;
                    if j > row_bounds[i as usize].1 as isize {
                        j = row_bounds[i as usize].0 as isize;
                    } else if grove[i as usize][j as usize] == '#' {
                        break;
                    }
                    position = (i, j);
                }
            },
            (Walk(steps), West) => {
                for _ in 0..steps {
                    let i = position.0;
                    let mut j = position.1 - 1;
                    if j < row_bounds[i as usize].0 as isize {
                        j = row_bounds[i as usize].1 as isize;
                    } else if grove[i as usize][j as usize] == '#' {
                        break;
                    }
                    position = (i, j);
                }
            },
            (Walk(steps), North) => {
                for _ in 0..steps {
                    let mut i = position.0 - 1;
                    let j = position.1;
                    if i < col_bounds[j as usize].0 as isize {
                        i = col_bounds[j as usize].1 as isize;
                    } else if grove[i as usize][j as usize] == '#' {
                        break;
                    }
                    position = (i, j);
                }
            },
            (Walk(steps), South) => {
                for _ in 0..steps {
                    let mut i = position.0 + 1;
                    let j = position.1;
                    if i > col_bounds[j as usize].1 as isize {
                        i = col_bounds[j as usize].0 as isize;
                    } else if grove[i as usize][j as usize] == '#' {
                        break;
                    }
                    position = (i, j);
                }
            },
        }
    }

    // The problem uses 1-based indexing and I've solved this using 0-based
    let final_row = position.0 + 1;
    let final_column = position.1 + 1;
    let final_heading: isize = heading.into();
    println!("row: {final_row}, column: {final_column}, heading: {final_heading}");
    let password = 1000 * final_row + 4 * final_column + final_heading;
    println!("password in {password}");
}
