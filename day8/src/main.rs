// not 2646
// not 720
// not 1980

fn main() {
    let input = std::fs::read_to_string("./example.txt").expect("should read file");
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let trees = input.lines().map(|l| l.chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    // let mut visible = 0u32;

    // // Loop through all the trees checking if they are visible from outside
    // for row in 0..trees.len() {
    //     for col in 0..trees[0].len() {
    //         let my_height = trees[row][col];

    //         // Check from the top
    //         let mut visible_above = true;
    //         for above in 0..row {
    //             if trees[above][col] >= my_height {
    //                 visible_above = false;
    //             }
    //         }

    //         // Check from bottom
    //         let mut visible_below = true;
    //         for below in row+1..trees.len() {
    //             if trees[below][col] >= my_height {
    //                 visible_below = false;
    //             }
    //         }

    //         // Check from left
    //         let mut visible_left = true;
    //         for left in 0..col {
    //             if trees[row][left] >= my_height {
    //                 visible_left = false;
    //             }
    //         }

    //         // Check from right
    //         let mut visible_right = true;
    //         for right in col+1..trees[0].len() {
    //             if trees[row][right] >= my_height {
    //                 visible_right = false;
    //             }
    //         }

    //         if visible_above || visible_below || visible_left || visible_right {
    //             visible += 1;
    //         }
            
    //     }
    // }
    
    // println!("part 1: {:?}", visible);

    //-----------------------


    let mut best_scenic_score = 0usize;

    // Loop through all the trees checking if they are visible from outside
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            let my_height = trees[row][col];
            let mut scenic_score = 1usize; // multiplicative identity

            println!("Calculating scenic score for position{},{}", row, col);

            // Check from the top
            let mut highest_so_far = 0;
            let mut visible_so_far = 0;
            for above in (0..row).rev() {
                if trees[above][col] >= highest_so_far {
                    highest_so_far = trees[above][col];
                    visible_so_far += 1;
                }
                if trees[above][col] >= my_height || above == 0 {
                    println!("above: {}", visible_so_far);
                    scenic_score *= visible_so_far;
                    break;
                }
            }

            // Check from bottom
            let mut highest_so_far = 0;
            let mut visible_so_far = 0;
            for below in row+1..trees.len() {
                if trees[below][col] >= highest_so_far {
                    highest_so_far = trees[below][col];
                    visible_so_far += 1;
                }
                if trees[below][col] >= my_height || below == trees.len() - 1 {
                    println!("below: {}", visible_so_far);
                    scenic_score *= visible_so_far;
                    break;
                }
            }

            // Check from left
            let mut highest_so_far = 0;
            let mut visible_so_far = 0;
            for left in (0..col).rev() {
                if trees[row][left] >= highest_so_far {
                    highest_so_far = trees[row][left];
                    visible_so_far += 1;
                }
                if trees[row][left] >= my_height || left == 0 {
                    println!("left: {}", visible_so_far);
                    scenic_score *= visible_so_far;
                    break;
                }
            }

            // Check from right
            let mut highest_so_far = 0;
            let mut visible_so_far = 0;
            for right in col+1..trees[0].len() {
                // println!("--- right is {right}, highest_so_far is {highest_so_far}, visible_so_far is {visible_so_far}");
                if trees[row][right] >= highest_so_far {
                    highest_so_far = trees[row][right];
                    visible_so_far += 1;
                }
                // println!("---- right is {right}, highest_so_far is {highest_so_far}, visible_so_far is {visible_so_far}");
                if trees[row][right] >= my_height || right == trees[0].len() - 1 {
                    println!("right: {}", visible_so_far);
                    scenic_score *= visible_so_far;
                    break;
                }
            }

            println!("scenic score is {}\n\n", scenic_score);

            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
            
        }
    }
    
    println!("part 2: {:?}", best_scenic_score);


}
