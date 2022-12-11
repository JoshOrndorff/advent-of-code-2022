use sscanf::sscanf;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, Hash)]
struct Monkey {
    items: VecDeque<u64>,
    operation: (char, Option<u64>),
    test: (u64, usize, usize),
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        lines.next();
        let items: VecDeque<u64> = lines.next().unwrap()[18..]
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let op_line = lines.next().unwrap();
        let operator = op_line.chars().nth(23).unwrap();
        let operand_chars = &op_line[op_line.rfind(' ').unwrap() + 1..];
        let operand = operand_chars.parse().ok();

        let divisor = sscanf!(lines.next().unwrap(), "  Test: divisible by {}", u64).unwrap();
        let true_target = sscanf!(
            lines.next().unwrap(),
            "    If true: throw to monkey {}",
            usize
        )
        .unwrap();
        let false_target = sscanf!(
            lines.next().unwrap(),
            "    If false: throw to monkey {}",
            usize
        )
        .unwrap();

        Self {
            items,
            operation: (operator, operand),
            test: (divisor, true_target, false_target),
        }
    }
}

fn apply_many_rounds<F: Fn(u64) -> u64>(monkies: &[RefCell<Monkey>], n: u64, f: F) -> u64 {
    let mut inspections: Vec<u64> = Vec::new();
    for _ in 0..monkies.len() {
        inspections.push(0);
    }

    for _round in 1..=n {
        for monkey_index in 0..monkies.len() {
            let m = &mut monkies[monkey_index].borrow_mut();
            while !m.items.is_empty() {
                // Calculate the new worry value
                let old = m.items.pop_front().expect(
                    "Should be able to pop because loop condition requires non-empty queue",
                );
                let new = match m.operation {
                    ('+', x) => f(old + x.unwrap_or(old)),
                    ('*', x) => f(old * x.unwrap_or(old)),
                    (op, _) => panic!("Encountered invalid operator {}, expected + or *", op),
                };

                // Record the inspection
                inspections[monkey_index] += 1;

                // Throw the item
                if new % m.test.0 == 0 {
                    monkies[m.test.1].borrow_mut().items.push_back(new);
                } else {
                    monkies[m.test.2].borrow_mut().items.push_back(new);
                }
            }
        }
    }

    // Find the two monkies with the most inspections
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Input file should exist");

    let monkies = input
        .trim()
        .split("\n\n")
        .map(|ms| RefCell::new(Monkey::from(ms)))
        .collect::<Vec<_>>();

    // Part 1
    let part_1 = apply_many_rounds(&monkies, 20, |n| n / 3);
    println!("Part 1: {part_1}");

    // Part 2
    let lcm: u64 = monkies.iter().map(|m| m.borrow().test.0).product();

    let part_2 = apply_many_rounds(&monkies, 10_000, |n: u64| n % lcm);

    println!("Part 2: {part_2}");
}
