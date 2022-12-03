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

fn common_among_all_three_rucksacks(sacks: &[&str]) -> char {
    match sacks {
        [a_sack, b_sack, c_sack] => {
            for a in a_sack.chars() {
                for b in b_sack.chars() {
                    if a == b && c_sack.contains(a) {
                        return a;
                    }
                }
            }
            panic!("Searched 3 sacks, but did not find a common item")
        },
        _ => panic!("Called with wrong number of sacks")
    }
    
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let part_1: u32 = input
        .lines()
        .map(item_in_both_compartments)
        .map(priority)
        .sum();

    println!("sum of priorities: {}", part_1);

    let part_2: u32 = input
        .lines()
        .collect::<Vec<_>>()[..]
        .chunks(3)
        .map(common_among_all_three_rucksacks)
        .map(priority)
        .sum();

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
