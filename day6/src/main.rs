fn no_dups(s: &[char]) -> bool {
    let mut s = Vec::from(s);
    s.sort();

    let mut deduped = s.clone();
    deduped.dedup();

    s == deduped
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read file");
    let chars = input.chars().collect::<Vec<_>>();

    // Solve part 1 using fancy methods
    let part_1 = 4 + chars
        .windows(4)
        .position(no_dups)
        .expect("Some start-of-packet marker should exist");

    println!("First start-of-packet: {}", part_1);

    // Solve part 2 using original imperative technique
    // Could, of course, be reworked to use the same technique as above
    for i in 0..input.len() - 14 {
        if no_dups(&chars[i..i + 14]) {
            println!("First message {}", i + 14);
            break;
        }
    }
}
