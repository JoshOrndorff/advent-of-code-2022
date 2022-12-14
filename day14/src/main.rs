use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read input file");

    // Parse rock structures
    let rock_structures = input.trim().lines().map(|l| {
        l.split(" -> ").map(|s| {
            let comma_index = s.find(',').unwrap();
            let x: usize = s[0..comma_index].parse().unwrap();
            let y: usize = s[comma_index + 1..].parse().unwrap();
            (x, y)
        })
    });

    // Render rock structures in cave, detecting boundaries as we go
    let mut lowest = 0;
    let mut rightmost = 0;
    let mut leftmost = usize::max_value();

    let mut cave: HashMap<(usize, usize), char> = HashMap::new();

    for structure in rock_structures {
        // println!("Processing new structure");

        for (x, y) in points_in_path(&structure.collect::<Vec<_>>()) {
            if y > lowest {
                lowest = y;
            }
            if x > rightmost {
                rightmost = x;
            }
            if x < leftmost {
                leftmost = x;
            }
            // println!("  inserting point {:?}", (x, y));
            cave.insert((x, y), '#');
        }
    }

    // Simulate falling sand
    let floor = lowest + 2;
    const SAND_SPAWN: (usize, usize) = (500, 0);
    let mut current_sand = SAND_SPAWN;

    let mut sand_counter = 0;
    let mut part_1 = None;

    for i in 0..100000 {
        if current_sand.1 > lowest && part_1.is_none() {
            println!("updating");
            part_1 = Some(sand_counter);
        }

        let (x, y) = current_sand;

        // Check immediately below
        if !cave.contains_key(&(x, y + 1)) && y + 1 != floor {
            current_sand = (x, y + 1);
        }
        // Check diagonal left
        else if !cave.contains_key(&(x - 1, y + 1)) && y + 1 != floor {
            current_sand = (x - 1, y + 1);
        }
        // Check diagonal right
        else if !cave.contains_key(&(x + 1, y + 1)) && y + 1 != floor {
            current_sand = (x + 1, y + 1);
        } else {
            // Commit resting place to cave and respawn
            cave.insert(current_sand, 'o');
            current_sand = SAND_SPAWN;
            sand_counter += 1;
            // visualize_cave(&cave, lowest, rightmost, leftmost);
        }
    }

    println!("part 1: {}", part_1.unwrap());
}

fn points_in_path(points: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut full_path = Vec::new();
    for i in 0..points.len() - 1 {
        let start = points[i];
        let end = points[i + 1];
        let segment = points_in_segment(start, end);
        full_path.extend(segment.into_iter());
    }

    full_path
}

fn points_in_segment((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Vec<(usize, usize)> {
    if x1 == x2 {
        if y2 > y1 { y1..=y2 } else { y2..=y1 }
            .map(|y| (x1, y))
            .collect()
    } else if y1 == y2 {
        if x2 > x1 { x1..=x2 } else { x2..=x1 }
            .map(|x| (x, y1))
            .collect()
    } else {
        panic!("Encountered segment that is neither horizontal nor vertical")
    }
}

fn visualize_cave(
    cave: &HashMap<(usize, usize), char>,
    lowest: usize,
    rightmost: usize,
    leftmost: usize,
) {
    for row_index in 0..=lowest {
        for col_index in leftmost..=rightmost {
            print!("{}", cave.get(&(col_index, row_index)).unwrap_or(&'.'));
        }
        println!();
    }
}
