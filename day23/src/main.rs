use std::collections::{HashMap, HashSet, VecDeque};

use Direction::*;
#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("read input file");

    // The grid is a set of coordinates occupied by elves
    let mut grid = HashSet::<(i64, i64)>::new();

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                grid.insert((i as i64, j as i64));
            }
        }
    }

    // The order of directions in which elves will try to move. This will be mutated after each round
    let mut move_order = VecDeque::from([North, South, West, East]);

    for round in 1..=10000 {
        // The proposals is a map from proposed destination coordinates to the original coordinates of all
        // elves who proposed to move there. This allows us to put elves back in their proper location
        // when collisions happen.
        let mut proposals = HashMap::<(i64, i64), Vec<(i64, i64)>>::new();

        // Figure out all the proposals
        for (i, j) in &grid {
            let proposal = get_proposal(i, j, &grid, &move_order);
            proposals
                .entry(proposal)
                .and_modify(|v| v.push((*i, *j)))
                .or_insert(vec![(*i, *j)]);
        }

        // Calculate the new grid
        let mut new_grid = HashSet::new();
        for (proposal, originals) in proposals {
            if originals.len() == 1 {
                new_grid.insert(proposal);
            } else {
                for original in originals {
                    new_grid.insert(original);
                }
            }
        }

        // Check if any elves moved, and if not we are done. If so, continue on to the next round.
        if new_grid == grid {
            println!("Done after {round} rounds.");
            break;
        }
        grid = new_grid;

        // Update the move order for the next round
        let fucking_borrow_checker = move_order.pop_front().unwrap();
        move_order.push_back(fucking_borrow_checker);

        // For debugging purposes, visualize the grid
        // let (n, s, w, e) = find_bounds(&grid);
        // for i in n..=s {
        //     for j in w..=e {
        //         if grid.contains(&(i, j)) {
        //             print!("{}", "#");
        //         } else {
        //             print!("{}", ".");
        //         }
        //     }
        //     println!();
        // }
    }

    // Calculate the number of empty cells as the total area minus the occupied cells
    let (north, south, west, east) = find_bounds(&grid);
    let empty = (south - north + 1) * (east - west + 1) - grid.len() as i64;

    println!("There are {empty} cells in the bounding box");
}

fn find_bounds(grid: &HashSet<(i64, i64)>) -> (i64, i64, i64, i64) {
    // Find the bounds of the grid
    let mut north = i64::max_value();
    let mut south = i64::min_value();
    let mut west = i64::max_value();
    let mut east = i64::min_value();
    for (i, j) in grid {
        if *i < north {
            north = *i;
        }
        if *i > south {
            south = *i;
        }
        if *j < west {
            west = *j;
        }
        if *j > east {
            east = *j;
        }
    }

    (north, south, west, east)
}

fn get_proposal(
    i: &i64,
    j: &i64,
    grid: &HashSet<(i64, i64)>,
    move_order: &VecDeque<Direction>,
) -> (i64, i64) {
    // If the elf doesn't have any neighbors, it stay where it is.
    if !(grid.contains(&(i - 1, j - 1))
        || grid.contains(&(i - 1, *j))
        || grid.contains(&(i - 1, j + 1))
        || grid.contains(&(*i, j + 1))
        || grid.contains(&(i + 1, j + 1))
        || grid.contains(&(i + 1, *j))
        || grid.contains(&(i + 1, j - 1))
        || grid.contains(&(*i, j - 1)))
    {
        return (*i, *j);
    }

    // If it has neighbors, it looks in each direction to see where it should move
    for direction in move_order {
        match direction {
            North => {
                if !(grid.contains(&(i - 1, j - 1))
                    || grid.contains(&(i - 1, *j))
                    || grid.contains(&(i - 1, j + 1)))
                {
                    return (i - 1, *j);
                }
            }
            South => {
                if !(grid.contains(&(i + 1, j - 1))
                    || grid.contains(&(i + 1, *j))
                    || grid.contains(&(i + 1, j + 1)))
                {
                    return (i + 1, *j);
                }
            }
            West => {
                if !(grid.contains(&(i - 1, j - 1))
                    || grid.contains(&(*i, j - 1))
                    || grid.contains(&(i + 1, j - 1)))
                {
                    return (*i, j - 1);
                }
            }
            East => {
                if !(grid.contains(&(i - 1, j + 1))
                    || grid.contains(&(*i, j + 1))
                    || grid.contains(&(i + 1, j + 1)))
                {
                    return (*i, j + 1);
                }
            }
        }
    }

    // Not explicitly stated, but I guess if an elf can't move in any direction,
    // it stays where it was???
    // panic!("Elf had neighbors, but could not move in any direction")
    (*i, *j)
}
