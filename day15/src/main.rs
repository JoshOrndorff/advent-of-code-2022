use sscanf::sscanf;
use std::collections::HashSet;

// For the example
// const KEY_ROW: i32 = 10;
// const BOUND: i32 = 20;
// const INPUT_FILE: &str = "./example.txt";

// For the real input
const KEY_ROW: i32 = 2_000_000;
const BOUND: i32 = 4_000_000;
const INPUT_FILE: &str = "./input.txt";

fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).expect("read input file");

    let circles = input
        .trim()
        .lines()
        .map(|l| {
            sscanf!(
                l,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                i32,
                i32,
                i32,
                i32
            )
            .unwrap()
        });

    let known_beacons_key_row = circles.clone()
        .filter_map(|(_, _, x, y)| (y == KEY_ROW).then_some(x))
        .collect::<HashSet<_>>();

    let centers_and_radii = circles.clone()
        .map(|(sx, sy, bx, by)| ((sx, sy), i32::abs(sx - bx) + i32::abs(sy - by)))
        .collect::<Vec<_>>();

    // Solve part 1 with a one-off method that dynamically detects bounds as we go
    // Determine the sections of row 10 that are blocked off
    let mut blocked_sections = Vec::new();
    let mut x_min = None;
    let mut x_max = None;
    for ((x, y), r) in &centers_and_radii {
        let vertical_offset = i32::abs(y - KEY_ROW);
        if vertical_offset > *r {
            continue;
        }

        let x_breadth = r - vertical_offset;
        let left = x - x_breadth;
        let right = x + x_breadth;

        x_min = match x_min {
            None => Some(left),
            Some(x) if x > left => Some(left),
            Some(x) => Some(x),
        };

        x_max = match x_max {
            None => Some(right),
            Some(x) if x < right => Some(right),
            Some(x) => Some(x),
        };

        blocked_sections.push((left, right));
    }

    let x_min = x_min.unwrap();
    let x_max = x_max.unwrap();
    println!("x bounds: ({x_min}, {x_max})");

    let all_covered = (x_min..=x_max).filter(|x| {
        blocked_sections.iter().any(|(min, max)| x >= min && x <= max )
    })
    .count();

    let part_1 = all_covered - known_beacons_key_row.len();

    println!("Part 1: {}", part_1);

    // In part 2 we are given bounds
    for x in 0..=BOUND {
        println!("outer iteration {x} / {BOUND}");

        for y in 0..=BOUND {
            // if y % 10_000 == 0 {
            //     println!("inner iteration {y} / {BOUND}");
            // }

            let mut not_in_any = true;
            for c_and_r in &centers_and_radii{
                if circle_contains_point(c_and_r, (x, y)) {
                    not_in_any = false;
                    break;
                }
            }

            if not_in_any {
                println!("Found location for beacon at ({x}, {y})");
                println!("Tuning frequency is {}", x * 4_000_000 + y);
                break;
            }
        }
    }


}

fn circle_contains_point(((cx, cy),r): &((i32, i32), i32), (px, py): (i32, i32)) -> bool {
    let dist_from_center = i32::abs(cx - px)+ i32::abs(cy - py);
    dist_from_center <= *r
}