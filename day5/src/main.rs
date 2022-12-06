use sscanf::sscanf;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let [starting_string, instr_string] = input.split("\n\n").collect::<Vec<_>>()[..] else { panic!("line 4")};

    let mut starting_lines = starting_string.lines().rev();

    // Determine the total number of stacks we're dealing with
    let num_cols = starting_lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .count();

    // Init 2D grid of chars
    let mut positions = Vec::<Vec<char>>::new();
    for _ in 0..num_cols {
        positions.push(Vec::new());
    }

    // Populate crate stacks with given init data
    let remaining_lines = starting_lines.collect::<Vec<_>>();
    for row_i in 0..remaining_lines.len() {
        let line_chars = remaining_lines[row_i].chars().collect::<Vec<_>>();
        for col in 0..num_cols {
            let char_i = col * 4 + 1;
            let c = line_chars[char_i];
            if c != ' ' {
                positions[col].push(c);
            }
        }
    }

    // Make a copy - one for each part
    let part_2_positions = positions.clone();

    let parsed_instructions = instr_string
        .trim()
        .lines()
        .map(|l| sscanf!(l, "move {} from {} to {}", usize, usize, usize).unwrap())
        // Convert 1-based indices in problem to 0-based for Rust Vectors
        .map(|(stack, from, to)| (stack, from - 1, to - 1));

    // ----------------------

    for (count, from, to) in parsed_instructions.clone() {
        for _ in 0..count {
            let in_transit = positions[from].pop().unwrap();
            positions[to].push(in_transit);
        }
    }

    let mut part_1 = String::new();
    for i in 0..num_cols {
        let top = positions[i].pop().unwrap();
        part_1.push(top);
    }

    println!("{}", part_1);

    // ----------------------

    let mut positions = part_2_positions;

    for (count, from, to) in parsed_instructions.clone() {
        let height = positions[from].len();

        let in_transit = positions[from].split_off(height - count);
        positions[to].extend(in_transit);
    }

    let mut part_1 = String::new();
    for i in 0..num_cols {
        let top = positions[i].pop().unwrap();
        part_1.push(top);
    }

    println!("{}", part_1);
}
