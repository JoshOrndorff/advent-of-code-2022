use sscanf::sscanf;
use std::collections::HashSet;

fn tail_positions(head_positions: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut tail = (0, 0);
    let mut tail_positions = Vec::new();

    for head in head_positions {
        let horizontal_displacement = head.0 - tail.0;
        let vertical_displacement = head.1 - tail.1;

        tail = match (horizontal_displacement, vertical_displacement, tail) {
            ( 2,  1, (x, y)) => (x + 1, y + 1),
            ( 2,  2, (x, y)) => (x + 1, y + 1),
            ( 1,  2, (x, y)) => (x + 1, y + 1),

            ( 0,  2, (x, y)) => (x    , y + 1),

            (-1,  2, (x, y)) => (x - 1, y + 1),
            (-2,  2, (x, y)) => (x - 1, y + 1),
            (-2,  1, (x, y)) => (x - 1, y + 1),

            (-2,  0, (x, y)) => (x - 1, y    ),

            (-2, -1, (x, y)) => (x - 1, y - 1),
            (-2, -2, (x, y)) => (x - 1, y - 1),
            (-1, -2, (x, y)) => (x - 1, y - 1),

            ( 0, -2, (x, y)) => (x    , y - 1),

            ( 1, -2, (x, y)) => (x + 1, y - 1),
            ( 2, -2, (x, y)) => (x + 1, y - 1),
            ( 2, -1, (x, y)) => (x + 1, y - 1),

            ( 2,  0, (x, y)) => (x + 1, y    ),
            

            (_, _, (x, y)) => (x, y),
        };

        tail_positions.push(tail);
    }

    tail_positions
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let motions = input
        .trim()
        .lines()
        .map(|l| sscanf!(l, "{} {}", char, i32))
        // .map(|r| {println!("{:?}", r); r})
        .map(|r| r.unwrap());

    let mut head = (0, 0);
    let mut head_positions = Vec::new();
    for (direction, steps) in motions {
        for _ in 0..steps {
            head = match (head, direction) {
                ((row, col), 'U') => (row + 1, col),
                ((row, col), 'D') => (row - 1, col),
                ((row, col), 'L') => (row, col - 1),
                ((row, col), 'R') => (row, col + 1),
                _ => panic!("Encountered invalid direction"),
            };
            head_positions.push(head)
        }
    }

    // Part 1
    let unique_tail_positions: HashSet<(i32, i32)> =
        tail_positions(head_positions.clone()).into_iter().collect();
    println!("part 1: {:?}", unique_tail_positions.len());

    // Part 2
    let mut positions = head_positions;
    for _ in 1..10 {
        // println!("after {} generations: {:?}", gen, positions);
        positions = tail_positions(positions);
    }

    let unique_tail_positions: HashSet<(i32, i32)> =positions.into_iter().collect();
    println!("part 2: {:?}", unique_tail_positions.len());


    // Debug printing for part 2
    for i in -13i32..13 {
        for j in -13i32..13 {
            if unique_tail_positions.contains(&(-i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
