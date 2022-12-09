use sscanf::sscanf;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./example.txt").expect("should read file");
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let motions = input
        .trim()
        .lines()
        .map(|l| sscanf!(l, "{} {}", char, i32))
        // .map(|r| {println!("{:?}", r); r})
        .map(|r| r.unwrap());

    let mut tail_spots = HashSet::<(i32, i32)>::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for (direction, steps) in motions {
        for _ in 0..steps {
            head = match (head, direction) {
                ((row, col), 'U') => (row + 1, col),
                ((row, col), 'D') => (row - 1, col),
                ((row, col), 'L') => (row, col - 1),
                ((row, col), 'R') => (row, col + 1),
                _ => panic!("Encountered invalid direction"),
            };

            let horizontal_displacement = head.0 - tail.0;
            let vertical_displacement = head.1 - tail.1;

            if horizontal_displacement > 1 {
                tail.0 += 1;
                tail.1 = head.1;
            } else if horizontal_displacement < -1 {
                tail.0 -= 1;
                tail.1 = head.1;
            }

            if vertical_displacement > 1 {
                tail.1 += 1;
                tail.0 = head.0;
            } else if vertical_displacement < -1 {
                tail.1 -= 1;
                tail.0 = head.0;
            }

            println!("Head spot: {:?}\nTail spot: {:?}\n", head, tail);
            
            tail_spots.insert(tail);
        }
    }

    // for tail in &tail_spots {
    //     println!("{:?}", tail);
    // }

    println!("part 1: {:?}", tail_spots.len());
}
