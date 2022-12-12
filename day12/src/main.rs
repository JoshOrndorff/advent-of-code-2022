// 1838 is too high

use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Input file should exist");

    let mut grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut start = (0, 0);
    let mut a_elevations = Vec::new();
    let mut end = (0, 0);

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
                'a' => {
                    a_elevations.push((row, col));
                }
                _ => (),
            }
        }
    }

    // Helper function to do the path finding
    let shortest_path_from = |start: (usize, usize)| -> Option<usize> {
        let mut explored: HashSet<(usize, usize)> = HashSet::new();
        let mut to_explore: VecDeque<((usize, usize), usize)> = VecDeque::new();
        to_explore.push_back((start, 0usize));

        while let Some(((row, col), steps)) = to_explore.pop_front() {
            if (row, col) == end {
                return Some(steps);
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

            let viable_neighbors: Vec<_> = neighbors
                .into_iter()
                .filter(|(n_row, n_col)| {
                    // A neighbor is viable as long as the step height is at most one and we haven't already explored it or planned to explore it.
                    grid[*n_row][*n_col] as i32 - grid[row][col] as i32 <= 1
                        && !explored.contains(&(*n_row, *n_col))
                        && !to_explore
                            .iter()
                            .any(|((row, col), _)| row == n_row && col == n_col)
                })
                .collect();

            for neighbor in viable_neighbors {
                to_explore.push_back((neighbor, steps + 1));
            }

            explored.insert((row, col));
        }

        None
    };

    let part_1 = shortest_path_from(start).unwrap();
    println!("Part 1: {part_1}");

    // The performance here could be improved. I've started a brand new search
    // from scratch for each starting point. A better idea would be to search
    // backwards from the end until the first time I encounter an a-height.
    // Although, wow, running in --release makes even this approach take under 1 second.
    let part_2 = a_elevations
        .iter()
        .filter_map(|s| shortest_path_from(*s).map(|d| (s, d)))
        .fold((start, part_1), |(old_start, old_dist), (start, dist)| {
            if dist < old_dist {
                (*start, dist)
            } else {
                (old_start, old_dist)
            }
        });

    println!("Part 2: {:?}", part_2);
}
