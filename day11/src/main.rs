use sscanf::sscanf;
use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;

#[derive(Debug, Hash)]
struct Monkey {
    items: VecDeque<u32>,
    operation: (char, Option<u32>),
    test: (u32, usize, usize),
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        lines.next();
        let items: VecDeque<u32> = lines.next().unwrap()[18..].split(", ").map(|n| n.parse().unwrap()).collect();
        
        let op_line = lines.next().unwrap();
        let operator = op_line.chars().nth(23).unwrap();
        let operand_chars = &op_line[op_line.rfind(' ').unwrap()+1..];
        let operand = operand_chars.parse().ok();
        
        let divisor = sscanf!(lines.next().unwrap(), "  Test: divisible by {}", u32).unwrap();
        let true_target = sscanf!(lines.next().unwrap(), "    If true: throw to monkey {}", usize).unwrap();
        let false_target = sscanf!(lines.next().unwrap(), "    If false: throw to monkey {}", usize).unwrap();

        Self {
            items,
            operation: (operator, operand),
            test: (divisor, true_target, false_target),
        }
    }
}

fn main() {

    let input = std::fs::read_to_string("./input.txt").expect("Input file should exist");

    let monkies = input.trim().split("\n\n").map(|ms| RefCell::new(Monkey::from(ms))).collect::<Vec<_>>();

    let mut inspections: Vec<u32> = Vec::new();
    for _ in 0..monkies.len() {
        inspections.push(0);
    }

    for round in 1..=20 {
        for monkey_index in 0..monkies.len() {
            let m = &mut monkies[monkey_index].borrow_mut();
            while !m.items.is_empty() {
                // Calculate the new worry value
                let old = m.items.pop_front().expect("Should be able to pop because loop condition requires non-empty queue");
                let new = match m.operation {
                    ('+', x) => (old + x.unwrap_or(old)) / 3,
                    ('*', x) => (old * x.unwrap_or(old)) / 3,
                    (op, _) => panic!("Encountered invalid operator {}, expected + or *", op),
                };

                // Record the inspection
                inspections[monkey_index] += 1;
                
                // Throw the item
                if new % m.test.0 == 0 {
                    monkies[m.test.1].borrow_mut().items.push_back(new);
                }
                else {
                    monkies[m.test.2].borrow_mut().items.push_back(new);
                }
            }
        }

        // This entire loop is just for printing the output
        // println!("\nAfter round {}:", round);
        // for monkey_index in 0..monkies.len() {
        //     let m = &mut monkies[monkey_index].borrow();
        //     println!("Monkey {monkey_index}: {:?}", m.items);
        // }
    }

    // Find the two monkies with the most inspections
    inspections.sort();
    let product_of_two_busiest: u32 = inspections.iter().rev().take(2).product();
    println!("Product of two busiest: {:?}", product_of_two_busiest);
}
