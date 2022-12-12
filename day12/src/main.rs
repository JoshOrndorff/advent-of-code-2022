// 1838 is too high

use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./example.txt").expect("Input file should exist");
    let input = std::fs::read_to_string("./input.txt").expect("Input file should exist");

    let mut grid:Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();
    println!("Width: {width},height: {height}");

    let mut start = (0, 0);
    let mut end = (0,0);

    for row in 0..height {
        for col in 0..width {
            let c = grid[row][col];
            match c {
                'S' => {
                    grid[row][col] = 'a';
                    start = (row, col);
                }
                'E' => {
                    grid[row][col] = 'z';
                    end = (row, col);
                }
                _ => ()
            }
            
        }
    }

    println!("start{:?}, end{:?}", start, end);

    let mut explored : HashSet<(usize, usize)> = HashSet::new();
    let mut to_explore: VecDeque<((usize, usize), usize)> = VecDeque::new();
    to_explore.push_back((start, 0usize));

    while let Some(((row, col), steps)) = to_explore.pop_front() {

        println!("Exploring ({row}, {col}) after {steps} steps");

        if (row, col) == end {
            println!("Made it in {steps} steps");
            break;
        }

        let mut neighbors = Vec::new();
        // Move up
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        // Move down
        if row < height - 1 {
            neighbors.push((row + 1, col));
        }
        // Move left
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        // Move right
        if col < width - 1 {
            neighbors.push((row, col + 1));
        }

        let viable_neighbors: Vec<_> = neighbors.into_iter().filter(|(n_row, n_col)| {
            // A neighbor is viable as long as the step height is at most one and we haven't already explored it or planned to explore it.
            grid[*n_row][*n_col] as i32 - grid[row][col] as i32 <= 1 && !explored.contains(&(*n_row, *n_col)) && !to_explore.iter().any(|((row, col), _)| row == n_row && col == n_col)
        }).collect();

        // TODO Maybe this could be a for_Each on the previous chain
        for neighbor in viable_neighbors {
            if explored.contains(&neighbor) {
                panic!("")
            }
            println!("found viable neighbor {:?}", neighbor);
            to_explore.push_back((neighbor, steps + 1));
        }

        // drop(viable_neighbors);

        println!("To explore: {:?}", to_explore.len());

        explored.insert((row, col));

        println!("Explored: {:?}\n", explored.len());
    }

}
