use std::collections::{HashSet, HashMap};

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
    fn occupied_cells(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        match self {
            Hline => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Plus => vec![(x, y + 1), (x + 1, y), (x + 1, y + 1), (x + 1, y + 2), (x + 2, y+ 1)],
            L => vec![(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)],
            Vline => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Square => vec![(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)],
        }
    }

    /// The width of the given shape
    fn width(&self) -> i64 {
        match self {
            Hline => 4,
            Plus => 3,
            L => 3,
            Vline => 1,
            Square => 2,
        }
    }

    /// The height of the given shape
    fn height(&self) -> i64 {
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

    let mut fingerprints = HashMap::new();
    let mut historical_heights = Vec::new();
    let mut cycle_length = 0;
    let mut height_diff = 0;

    let mut upcoming_shapes = vec![Hline, Plus, L, Vline, Square].into_iter().cycle();
    let mut upcoming_wind = input.trim().chars().cycle();

    let mut occupied_cells = HashSet::<(i64, i64)>::new();
    let mut highest_occupied = 0;

    for i in 0..1000_000_000i64 {
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

            // If the shape is not against a wall, and none of the would-be-occupied
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
        historical_heights.push(highest_occupied);
        let fingerprint =  fingerprint_top(&occupied_cells, highest_occupied);
        if fingerprints.contains_key(&fingerprint) {
            let prev = fingerprints[&fingerprint];
            cycle_length = i - prev;
            height_diff = historical_heights[i as usize] - historical_heights[prev as usize];
            println!("cycle detected between {} and {}. Shapes dropped: {}, Height added: {}", prev, i, cycle_length, height_diff);
            break;
        }
        fingerprints.insert(fingerprint, i);
    }

    // It isn't always necessary (for example, it wasn't with the example input) but it is more reliable to pull the offset
    // height from after the start of the first detected cycle. Sometimes the very beginning does not follow the cyclical
    // nature perfectly due to the floor boundary.
    let part_1_cycles = 2022 / cycle_length;
    let part_1_offset = 2022 % cycle_length;
    let part_1 = (part_1_cycles - 1) * height_diff + historical_heights[(part_1_offset + cycle_length - 1) as usize];
    println!("part 1:{}", part_1);

    let part_2_cycles = 1_000_000_000_000i64 / cycle_length;
    let part_2_offset = 1_000_000_000_000i64 % cycle_length;
    let part_2 = (part_2_cycles - 1) * height_diff + historical_heights[(part_2_offset + cycle_length - 1) as usize];
    println!("part 2:{}", part_2);
}

#[allow(dead_code)]
fn display_tower(occupied_cells: &HashSet<(i64, i64)>, top: i64, cur_shape: Shape, cur_x: i64, cur_y: i64) {
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

fn fingerprint_top(occupied_cells: &HashSet<(i64, i64)>, top: i64) -> impl std::hash::Hash + Eq{
    let mut v = Vec::new();

    for y in (top - 60)..=top {
        for x in 0..7 {
            if occupied_cells.contains(&(x, y)) {
                v.push((x, top - y));
            }
        }
    }

    v
}