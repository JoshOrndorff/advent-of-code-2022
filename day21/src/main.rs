use sscanf::sscanf;
use std::collections::HashMap;

use Expr::*;
#[derive(Clone, Debug)]
enum Expr {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Lit(i64),
}

impl Expr {
    fn eval(&self, monkies: &HashMap<String, Self>) -> i64 {
        match self {
            Lit(x) => *x,
            Add(p1, p2) => monkies[p1].eval(monkies) + monkies[p2].eval(monkies),
            Sub(p1, p2) => monkies[p1].eval(monkies) - monkies[p2].eval(monkies),
            Mul(p1, p2) => monkies[p1].eval(monkies) * monkies[p2].eval(monkies),
            Div(p1, p2) => monkies[p1].eval(monkies) / monkies[p2].eval(monkies),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("read input file");
    let mut monkies = input
        .trim()
        .lines()
        .map(|l| {
            if l.len() < 15 {
                let (name, val) = sscanf!(l, "{}: {}", String, i64).unwrap();
                (name, Lit(val))
            } else {
                let (name, p1, op, p2) =
                    sscanf!(l, "{}: {} {} {}", String, String, char, String).unwrap();
                (
                    name,
                    match op {
                        '+' => Add(p1, p2),
                        '-' => Sub(p1, p2),
                        '*' => Mul(p1, p2),
                        '/' => Div(p1, p2),
                        invalid => panic!("Invalid operation character {}", invalid),
                    },
                )
            }
        })
        .collect::<HashMap<_, _>>();

    // Part one is a simple recursive eval. No cache necessary
    let part_1 = monkies["root".into()].eval(&monkies);
    println!("Part 1: {}", part_1);

    // Part 2

    // Extract the monkey identities I'm expected to compare
    let Add(target1, target2) = monkies["root".into()].clone() else {
        panic!("Couldn't extract terms from root entry (maybe it isn't Add)")
    };

    println!("Literals before optimization {}", count_literals("root", &monkies));
    optimize_monkies("root", &mut monkies);
    println!("Literals after  optimization {}", count_literals("root", &monkies));
    
    // Start with a simple upper bound of 4k and see if we find a suitable humn value
    for i in 0..100_000_000 {
        
        monkies.insert(
            "humn".into(),
            Lit(i),
        );

        let left = monkies[&target1].eval(&monkies);
        let right = monkies[&target2].eval(&monkies);

        if i % 1_000_000 == 0 {
            // println!("left: {}", left);
            // println!("right {}", right);
            println!("Iteration {}", i);
        };
        if left == right {
            println!("Found equality with humn value {}", i);
            break;
        }
    }
}

/// Recursive algorithm to count the number of TERMINAL literals in the tree.
/// Not every single literal in the map will be counted because some of them will
/// not be reached by recursion
fn count_literals(name: &str, monkies: &HashMap<String, Expr>) -> u64 {
    let e = monkies[name].clone();
    match e {
        Lit(_) => 1,
        Add(p1, p2) => count_literals(&p1, monkies) + count_literals(&p2, monkies),
        Sub(p1, p2) => count_literals(&p1, monkies) + count_literals(&p2, monkies),
        Mul(p1, p2) => count_literals(&p1, monkies) + count_literals(&p2, monkies),
        Div(p1, p2) => count_literals(&p1, monkies) + count_literals(&p2, monkies),
    }
}

/// Recursive algorithm to optimize the tree. The function returns whether the node
/// passed in has the "humn" node as a descendant. If you start with the root node, this
/// will always return true. If the "humn" node is not in the hot path, the node will be
/// evaluated and replaced with a literal value
fn optimize_monkies(name: &str, monkies: &mut HashMap<String, Expr>) -> bool {
    // Terminating case
    if name == "humn" {
        return true;
    }

    let e = monkies[name].clone();
    match e.clone() {
        Lit(_) => false,
        Add(p1, p2) => {
            if optimize_monkies(&p1, monkies) | optimize_monkies(&p2, monkies) {
                true
            } else {
                monkies.insert(name.to_string(), Lit(e.eval(monkies)));
                false
            }
        },
        Sub(p1, p2) => {
            if optimize_monkies(&p1, monkies) | optimize_monkies(&p2, monkies) {
                true
            } else {
                monkies.insert(name.to_string(), Lit(e.eval(monkies)));
                false
            }
        },
        Div(p1, p2) => {
            if optimize_monkies(&p1, monkies) | optimize_monkies(&p2, monkies) {
                true
            } else {
                monkies.insert(name.to_string(), Lit(e.eval(monkies)));
                false
            }
        },
        Mul(p1, p2) => {
            if optimize_monkies(&p1, monkies) | optimize_monkies(&p2, monkies) {
                true
            } else {
                monkies.insert(name.to_string(), Lit(e.eval(monkies)));
                false
            }
        },
    }
}

// Todo, these could be unit tests. Prolly not worth the time though.
// println!("hmdt should be  32: {}" ,monkies["hmdt".into()].eval(&monkies));
// println!("zczc should be   2: {}" ,monkies["zczc".into()].eval(&monkies));
// println!("drzm should be  30: {}" ,monkies["drzm".into()].eval(&monkies));
// println!("dbpl should be   5: {}" ,monkies["dbpl".into()].eval(&monkies));
// println!("sjmn should be 150: {}" ,monkies["sjmn".into()].eval(&monkies));
// println!("pppw should be   2: {}" ,monkies["pppw".into()].eval(&monkies));
// println!("cczh should be   8: {}" ,monkies["cczh".into()].eval(&monkies));
// println!("lfqf should be   4: {}" ,monkies["lfqf".into()].eval(&monkies));
// println!("sllz should be   4: {}" ,monkies["sllz".into()].eval(&monkies));
// println!("lgvd should be   4: {}" ,monkies["lgvd".into()].eval(&monkies));
// println!("ljgn should be   2: {}" ,monkies["ljgn".into()].eval(&monkies));
// println!("ptdq should be   2: {}" ,monkies["ptdq".into()].eval(&monkies));
// println!("humn should be   5: {}", monkies["humn".into()].eval(&monkies));
// println!("dvpt should be   3: {}" ,monkies["dvpt".into()].eval(&monkies));
