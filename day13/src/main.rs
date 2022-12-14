use std::cmp::Ordering;
use Signal::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Eq)]
enum Signal {
    Int(u32),
    List(Vec<Signal>),
}

impl From<&str> for Signal {
    fn from(s: &str) -> Self {
        let s_bytes = s.as_bytes();
        // Every signal is either a list that starts and ends with [ and ] or a int
        match s_bytes {
            [] => panic!("Can't parse Signal from 0-length string slice"),
            [b'[', b']'] => List(vec![]),
            [b'[', .., b']'] => {
                let mut depth = 0;
                let mut last_split_index = 0usize;
                let mut parts: Vec<&str> = Vec::new();
                for i in 1..s.len() {
                    match (s_bytes[i], depth) {
                        (b'[', _) => {
                            depth += 1;
                        }
                        (b']', 0) => {
                            parts.push(&s[last_split_index + 1..i]);
                        }
                        (b',', 0) => {
                            parts.push(&s[last_split_index + 1..i]);
                            last_split_index = i;
                        }
                        (b']', _) => {
                            depth -= 1;
                        }
                        _ => (),
                    }
                }

                List(parts.iter().map(|s| Signal::from(*s)).collect())
            }
            _ => Int(s.parse().expect("Failed to parse integer from {}, s")),
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Int(a), Int(b)) => Some(a.cmp(b)),
            (Int(a), List(_)) => List(vec![Int(*a)]).partial_cmp(other),
            (List(_), Int(b)) => self.partial_cmp(&List(vec![Int(*b)])),
            (List(aa), List(bb)) => {
                let mut i = 0usize;
                loop {
                    if i >= aa.len() && i >= bb.len() {
                        return Some(Ordering::Equal);
                    }
                    if i >= aa.len() {
                        return Some(Ordering::Less);
                    }
                    if i >= bb.len() {
                        return Some(Ordering::Greater);
                    }
                    if aa[i] < bb[i] {
                        return Some(Ordering::Less);
                    }
                    if aa[i] > bb[i] {
                        return Some(Ordering::Greater);
                    }
                    i += 1;
                }
            }
        }
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("All comparisons should return Some")
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read input file");

    // -------- Part 1 ----------------

    let pairs = input.trim().split("\n\n").map(|s| {
        let line_break_index = s
            .find('\n')
            .expect("each pair should have a single line break");
        let signal_1 = Signal::from(&s[..line_break_index]);
        let signal_2 = Signal::from(&s[line_break_index + 1..]);
        (signal_1, signal_2)
    });

    let part_1: usize = pairs
        .enumerate()
        .filter_map(|(i, (s1, s2))| (s1 < s2).then_some(i + 1))
        .sum();

    println!("part 1: {}", part_1);

    // -------- Part 2 ----------------

    let divider_packets = vec![
        List(vec![List(vec![Int(2)])]),
        List(vec![List(vec![Int(6)])]),
    ];

    let part_2: usize = input
        .trim()
        .split_ascii_whitespace()
        .map(Signal::from)
        .chain(divider_packets.iter().cloned())
        .sorted()
        .enumerate()
        .filter_map(|(i, s)| divider_packets.contains(&s).then_some(i + 1))
        .product();

    println!("part 2: {}", part_2);
}

#[test]
fn parse_single_digit_int() {
    assert_eq!(Int(4), Signal::from("4"))
}

#[test]
fn parse_multi_digit_int() {
    assert_eq!(Int(45), Signal::from("45"))
}

#[test]
fn parse_empty_list() {
    assert_eq!(List(vec![]), Signal::from("[]"))
}

#[test]
fn parse_double_empty_list() {
    assert_eq!(List(vec![List(vec![])]), Signal::from("[[]]"))
}

#[test]
fn parse_triple_empty_list() {
    assert_eq!(List(vec![List(vec![List(vec![])])]), Signal::from("[[[]]]"))
}

#[test]
fn parse_list_of_int() {
    assert_eq!(List(vec![Int(3)]), Signal::from("[3]"))
}

#[test]
fn parse_list_of_two_ints() {
    assert_eq!(List(vec![Int(2), Int(3)]), Signal::from("[2,3]"))
}

#[test]
fn parse_list_of_ints_and_lists() {
    assert_eq!(
        List(vec![List(vec![Int(1), Int(2),]), Int(3), List(vec![]),]),
        Signal::from("[[1,2],3,[]]")
    )
}

#[test]
fn compare_ints() {
    assert_eq!(Some(Ordering::Less), Int(4).partial_cmp(&Int(5)))
}

#[test]
fn compare_int_empty_list() {
    assert_eq!(Some(Ordering::Greater), Int(4).partial_cmp(&List(vec![])))
}

#[test]
fn compare_two_ints_vs_one() {
    let a = List(vec![Int(4), Int(5)]);
    let b = List(vec![Int(6)]);
    assert_eq!(Some(Ordering::Less), a.partial_cmp(&b))
}

#[test]
fn compare_two_ints_vs_one_2() {
    let a = List(vec![Int(4), Int(5)]);
    let b = List(vec![Int(4)]);
    assert_eq!(Some(Ordering::Greater), a.partial_cmp(&b))
}

#[test]
fn compare_double_empty_vs_empty() {
    let a = List(vec![List(vec![])]);
    let b = List(vec![]);
    assert_eq!(Some(Ordering::Greater), a.partial_cmp(&b))
}

#[test]
fn compare_list_of_single_int_vs_empty_list() {
    let a = List(vec![Int(4)]);
    let b = List(vec![]);
    assert_eq!(Some(Ordering::Greater), a.partial_cmp(&b))
}

#[test]
fn should_not_be_equal() {
    let a = Signal::from("[7,9]");
    let b = Signal::from("[[7]]");

    assert_eq!(Some(Ordering::Greater), a.partial_cmp(&b))
}
