fn item_in_both_compartments(s: &str) -> char {
    let midpoint = s.len() / 2;
    let compartment_1 = &s[..midpoint];
    let compartment_2 = &s[midpoint..];

    for candidate in compartment_1.chars() {
        if compartment_2.contains(candidate) {
            return candidate;
        }
    }

    panic!("no item found in both compartments")
}

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 38
    } else if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        panic!("invalid character for priority")
    }
}

fn common_among_all_three_rucksacks([a_sack, b_sack, c_sack]: &[&str; 3]) -> char {
    for a in a_sack.chars() {
        for b in b_sack.chars() {
            if a == b && c_sack.contains(a) {
                return a;
            }
        }
    }
    panic!("Searched 3 sacks, but did not find a common item")
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let part_1: u32 = input
        .lines()
        .map(item_in_both_compartments)
        .map(priority)
        .sum();

    println!("sum of priorities: {}", part_1);

    // Chunking was the hardest part for me.
    // I looked at https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.array_chunks but avoided it because I wanted to use the stable compiler
    // Just learned I could have used https://doc.rust-lang.org/std/primitive.slice.html#method.chunks
    let mut lines = input.lines().peekable();
    let mut part_2 = 0u32;
    while lines.peek().is_some() {
        let group = &[
            lines.next().expect("Another item exists; we peeked"),
            lines
                .next()
                .expect("Number of lines should be a multiple of 3 (second)"),
            lines
                .next()
                .expect("Number of lines should be a multiple of 3 (third)"),
        ];

        part_2 += priority(common_among_all_three_rucksacks(group));
    }

    println!("sum of priorities: {}", part_2);
}

#[test]
fn a_ascii_code() {
    assert_eq!(priority('a'), 1);
}

#[test]
fn big_a_ascii_code() {
    assert_eq!(priority('A') as u32, 27);
}
