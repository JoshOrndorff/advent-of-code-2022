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

    
    let mut head = (0, 0);
    let mut head_positions =Vec::new();
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

    let mut tail_spots = HashSet::<(i32, i32)>::new();
    let mut tail = (0, 0);

    for head in head_positions {

        let horizontal_displacement = head.0 - tail.0;
        let vertical_displacement = head.1 - tail.1;

        tail = match(horizontal_displacement, vertical_displacement, tail) {
            (2, 2, (x, y)) => (x + 1, y + 1),
            (1, 2,(x, y)) => (x + 1, y + 1),
            (0, 2, (x, y)) => (x, y + 1),
            (-1, 2, (x, y)) => (x - 1, y +1),
            (-2, 2, (x, y)) => (x - 1, y + 1),
            
            (-2, 1, (x, y)) => (x - 1, y + 1),
            (-2, 0, (x, y)) => (x - 1, y),
            (-2, -1, (x, y)) => (x - 1, y - 1),

            (-2, -2, (x, y)) => (x - 1, y - 1),
            (-1, -2, (x, y)) => (x - 1, y - 1),
            (0, -2, (x, y)) => (x, y - 1),
            (1, -2, (x, y)) => (x + 1, y - 1),
            (2, -2, (x, y)) => (x + 1, y - 1),
            
            (2, -1, (x, y)) => (x + 1, y - 1),
            (2, 0, (x, y)) => (x + 1, y),
            (2, 1, (x, y)) => (x + 1, y + 1),

            (_, _, (x, y)) => (x, y),
        };

        println!("Head spot: {:?}\nTail spot: {:?}\n", head, tail);
        
        tail_spots.insert(tail);
    }

    println!("part 1: {:?}", tail_spots.len());
}
