fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let trees = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible = 0u32;

    // Loop through all the trees checking if they are visible from outside
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
            for below in row + 1..trees.len() {
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
            for right in col + 1..trees[0].len() {
                if trees[row][right] >= my_height {
                    visible_right = false;
                }
            }

            if visible_above || visible_below || visible_left || visible_right {
                visible += 1;
            }
        }
    }

    println!("part 1: {:?}", visible);

    //-----------------------

    let mut best_scenic_score = 0usize;

    // Loop through all the trees checking if they are visible from outside
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            let my_height = trees[row][col];
            let mut scenic_score = 1usize; // multiplicative identity

            // Check from the top
            let mut viewing_distance = 0;
            for above in (0..row).rev() {
                viewing_distance += 1;
                if trees[above][col] >= my_height || above == 0 {
                    break;
                }
            }
            scenic_score *= viewing_distance;

            // Check from bottom
            let mut viewing_distance = 0;
            for below in row + 1..trees.len() {
                viewing_distance += 1;
                if trees[below][col] >= my_height || below == trees.len() - 1 {
                    break;
                }
            }
            scenic_score *= viewing_distance;

            // Check from left
            let mut viewing_distance = 0;
            for left in (0..col).rev() {
                viewing_distance += 1;
                if trees[row][left] >= my_height || left == 0 {
                    break;
                }
            }
            scenic_score *= viewing_distance;

            // Check from right
            let mut viewing_distance = 0;
            for right in col + 1..trees[0].len() {
                viewing_distance += 1;
                if trees[row][right] >= my_height || right == trees[0].len() - 1 {
                    break;
                }
            }
            scenic_score *= viewing_distance;

            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }

    println!("part 2: {:?}", best_scenic_score);
}
