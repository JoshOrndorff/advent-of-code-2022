fn main() {
    let input = std::fs::read_to_string("./input.txt")
    .expect("input file should exist");
    
    let mut cals_per_elf = input
    .split("\n\n")
    .map(|es|
        es
            .lines()
            .map(|s| u32::from_str_radix(s, 10).expect("valid numbers"))
            .sum()
    ).collect::<Vec<u32>>();

    cals_per_elf.sort();

    let top_one: u32 = cals_per_elf.iter().rev().take(1).sum();
    let top_three: u32 = cals_per_elf.iter().rev().take(3).sum();

    println!("Top elf    : {}", top_one);
    println!("Top 3 elves: {}", top_three);
}
