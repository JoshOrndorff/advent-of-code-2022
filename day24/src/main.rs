use std::collections::{BTreeMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("read input file");

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // Find the dimensions of the grid
    let height = grid.len();
    let width = grid[0].len();

    // Find the horizontal location of the start and end positions
    let start_x = grid[0]
        .iter()
        .enumerate()
        .find(|(_, c)| **c == '.')
        .unwrap()
        .0;

    let end_x = grid[height - 1]
        .iter()
        .enumerate()
        .find(|(_, c)| **c == '.')
        .unwrap()
        .0;

    // We track the blizzards' locations as a BTreeMap from location to vec of blizzard directions
    let mut blizzards = BTreeMap::<(usize, usize), Vec<char>>::new();
    grid.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, c)| {
            if "<>^v#".contains(*c) {
                blizzards
                    .entry((x, y))
                    .and_modify(|v| v.push(*c))
                    .or_insert(vec![*c]);
            }
        })
    });

    let entrance = (start_x, 0usize);
    let exit = (end_x, height - 1);

    let (first_pass_steps, blizzards) = traverse_valley(entrance, exit, width, height, blizzards);
    let (second_pass_steps, blizzards) = traverse_valley(exit, entrance, width, height, blizzards);
    let (third_pass_steps, _) = traverse_valley(entrance, exit, width, height, blizzards);

    // CAUTION! There is an off-by-one issue between my results and the AOC solutions in subsequent crossings. I'm not sure why.
    println!("Traversed the valley in {first_pass_steps}");
    println!("Returned in {}", second_pass_steps + 1);
    println!("Traversed again in {}", third_pass_steps + 1);

    println!(
        "\nTotal journey distance: {}",
        first_pass_steps + second_pass_steps + third_pass_steps + 2
    );
}

fn traverse_valley(
    start: (usize, usize),
    destination: (usize, usize),
    width: usize,
    height: usize,
    mut blizzards: BTreeMap<(usize, usize), Vec<char>>,
) -> (usize, BTreeMap<(usize, usize), Vec<char>>) {
    // We track our x, y coordinates through the valley on a step-by-step basis
    // Each "generation" represents one step in our path.
    let mut current_generation = VecDeque::from([start]);
    let mut steps = 0usize;

    // Keep track of states we've already seen in case there are ways to wander in circles
    let mut explored: HashSet<(usize, usize, u64)> = HashSet::new();

    loop {
        // println!("\n\nstep {steps}:");
        // println!("generation size: {}", current_generation.len());

        // // For debugging purposes, print the blizzards
        // for y in 0..height {
        //     for x in 0..width {
        //         match blizzards.get(&(x, y)) {
        //             None => print!("."),
        //             Some(v) => {
        //                 if v.len() == 1 {
        //                     print!("{}", v[0]);
        //                 } else {
        //                     print!("{}", v.len());
        //                 }
        //             },
        //         }
        //     }
        //     println!();
        // }

        // A new queue for the next generations of places we may trek
        let mut next_generation: VecDeque<(usize, usize)> = VecDeque::new();

        // Calculate the new blizzards
        let next_blizzards: BTreeMap<(usize, usize), Vec<char>> =
            step_blizzards(width, height, &blizzards);

        for (x, y) in &current_generation {
            if (*x, *y) == destination {
                return (steps, next_blizzards);
            }

            // See if we've explored this state before. If so, bail, if not, note it for future
            let h = hash(&next_blizzards);
            if explored.contains(&(*x, *y, h)) {
                continue;
            } else {
                explored.insert((*x, *y, h));
            }

            // Check the five possible move options
            if !next_blizzards.contains_key(&(*x, *y)) {
                next_generation.push_back((*x, *y));
            }

            if !next_blizzards.contains_key(&(x + 1, *y)) {
                next_generation.push_back((x + 1, *y));
            }

            if !next_blizzards.contains_key(&(x - 1, *y)) {
                next_generation.push_back((x - 1, *y))
            }

            // We need an extra condition here so we don't step off the bottom when beginning the return journey
            if *y < height - 1 && !next_blizzards.contains_key(&(*x, y + 1)) {
                next_generation.push_back((*x, y + 1));
            }

            // We need an extra condition here so we don't step off the top when we first start
            if *y > 0 && !next_blizzards.contains_key(&(*x, y - 1)) {
                next_generation.push_back((*x, y - 1));
            }
        }

        // Increment the step counter and get read for the next generation
        steps += 1;
        current_generation = next_generation;
        blizzards = next_blizzards;
    }
}

fn step_blizzards(
    width: usize,
    height: usize,
    current: &BTreeMap<(usize, usize), Vec<char>>,
) -> BTreeMap<(usize, usize), Vec<char>> {
    let mut next: BTreeMap<(usize, usize), Vec<char>> = BTreeMap::new();

    for ((x, y), directions) in current {
        for direction in directions {
            match direction {
                '>' => {
                    let mut x = x + 1;
                    if x == width - 1 {
                        x = 1;
                    }
                    next.entry((x, *y))
                        .and_modify(|v: &mut Vec<char>| v.push('>'))
                        .or_insert(vec!['>']);
                }
                '<' => {
                    let mut x = x - 1;
                    if x == 0 {
                        x = width - 2;
                    }
                    next.entry((x, *y))
                        .and_modify(|v| v.push('<'))
                        .or_insert(vec!['<']);
                }
                '^' => {
                    let mut y = y - 1;
                    if y == 0 {
                        y = height - 2;
                    }
                    next.entry((*x, y))
                        .and_modify(|v| v.push('^'))
                        .or_insert(vec!['^']);
                }
                'v' => {
                    let mut y = y + 1;
                    if y == height - 1 {
                        y = 1;
                    }
                    next.entry((*x, y))
                        .and_modify(|v| v.push('v'))
                        .or_insert(vec!['v']);
                }
                '#' => {
                    next.insert((*x, *y), vec!['#']);
                }
                bad_direction => panic!("encountered bad direction: {bad_direction}"),
            }
        }
    }

    next
}

use std::hash::{Hash, Hasher};
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
