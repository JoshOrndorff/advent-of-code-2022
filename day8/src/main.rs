use sscanf::sscanf;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

fn main() {
    let input = std::fs::read_to_string("./example.txt").expect("should read file");
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let trees = input.lines().map(|l| l.chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut visible = 0u32;

    // Loop through all the trees checking if they are visible
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            let my_height = trees[row][col];

            // Check from the top
            let mut visible_above = true;
            for above in 0..row {
                if trees[above][col] >= my_height {
                    visible_above = false;
                }
            }

            // Check from bottom
            let mut visible_below = true;
            for below in row+1..trees.len() {
                if trees[below][col] >= my_height {
                    visible_below = false;
                }
            }

            // Check from left
            let mut visible_left = true;
            for left in 0..col {
                if trees[row][left] >= my_height {
                    visible_left = false;
                }
            }

            // Check from right
            let mut visible_right = true;
            for right in col+1..trees[0].len() {
                if trees[row][right] >= my_height {
                    visible_right = false;
                }
            }

            if visible_above || visible_below || visible_left || visible_right {
                visible += 1;
            }
            
        }
    }
    // let part_1 = todo!();
    println!("part 1: {:?}", visible);
}
