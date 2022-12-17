use std::collections::HashSet;

use Shape::*;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Hline,
    Plus,
    L,
    Vline,
    Square,
}

impl Shape {
    /// Returns a list of coordinates that this shape occupies, given a reference coordinate
    /// which is the lower left corner of its bounding box.
    fn occupied_cells(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        match self {
            Hline => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Plus => vec![(x, y + 1), (x + 1, y), (x + 1, y + 1), (x + 1, y + 2), (x + 2, y+ 1)],
            L => vec![(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)],
            Vline => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Square => vec![(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)],
        }
    }

    /// The width of the given shape
    fn width(&self) -> i32 {
        match self {
            Hline => 4,
            Plus => 3,
            L => 3,
            Vline => 1,
            Square => 2,
        }
    }

    /// The height of the given shape
    fn height(&self) -> i32 {
        match self {
            Hline => 1,
            Plus => 3,
            L => 3,
            Vline => 4,
            Square => 2,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("read input file");

    let mut upcoming_shapes = vec![Hline, Plus, L, Vline, Square].into_iter().cycle();
    let mut upcoming_wind = input.trim().chars().cycle();

    let mut occupied_cells = HashSet::<(i32, i32)>::new();
    let mut highest_occupied = 0;

    for _ in 0..2022 {
        let shape = upcoming_shapes.next().unwrap();

        let mut x = 2;
        let mut y = highest_occupied + 4;

        // display_tower(&occupied_cells, 20, shape, x, y);
        // let _ = std::io::stdin().read_line(&mut String::new());

        loop {

            let wind_offset = match upcoming_wind.next().unwrap() {
                '<' => -1,
                '>' => 1,
                _ => panic!("invalid wind character"),
            };

            // If teh shape is not against a wall, and none of the would-be-occupied
            // cells are already occupied, then execute the wind blow
            let blown_x = x + wind_offset;
            if blown_x >= 0
                && blown_x + shape.width() < 8
                && !shape
                    .occupied_cells(blown_x, y)
                    .iter()
                    .any(|p| occupied_cells.contains(p))
            {
                x += wind_offset;
            }

            // If the shape has not landed, then execute the fall
            // Otherwise, update the occupied cells, and move on to the next shape
            let fallen_y = y - 1;
            if fallen_y > 0 && !shape.occupied_cells(x, fallen_y).iter().any(|p| occupied_cells.contains(p)) {
                y -= 1;
            }
            else {
                if y + shape.height() - 1 > highest_occupied {
                    highest_occupied = y + shape.height() - 1;
                }
                occupied_cells.extend(shape.occupied_cells(x, y));
                break;
            }
        }
    }

    println!("part 1:{}", highest_occupied);
}

fn display_tower(occupied_cells: &HashSet<(i32, i32)>, top: i32, cur_shape: Shape, cur_x: i32, cur_y: i32) {
    let falling_cells = cur_shape.occupied_cells(cur_x, cur_y);
    for y in (1..=top).rev() {
        print!("{:4} |", y);
        for x in 0..7 {
            if occupied_cells.contains(&(x, y)) {
                print!("#");
            } else if falling_cells.contains(&(x, y)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("     +-------+\n");
}