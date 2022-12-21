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

    // Optimize the tree so it is faster to calculate.
    // After optimization, we only re-calculate the arts that depend on the humn value.
    // Actually it turns out that this optimization was not at all necessary.
    // Switching to binary search was the winning strategy.
    optimize_monkies("root", &mut monkies);

    // Do binary search to find the right value
    let mut high = 10000000000000i64;
    let mut low = 0i64;

    while high != low {
        let mid = (high + low) / 2;

        monkies.insert("humn".into(), Lit(mid));

        let left = monkies[&target1].eval(&monkies);
        let right = monkies[&target2].eval(&monkies);

        if right == left {
            println!(
                "Found initial equality with humn value {}. Continuing to linear search",
                mid
            );
            break;
        }
        // BE CAREFUL! The direction of the inequality depends which side of your tree has the "humn" value
        else if right > left {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    // There maybe multiple integers that solve the puzzle. The website seems to want the lowest one.
    // So after we find _some_ solution using binary search, we then do a linear search to find the lowest.
    // TODO would be more reliable to start with the known good value and iterate backwards until mismatch.
    low = low - 1000;

    for i in low..high {
        monkies.insert("humn".into(), Lit(i));

        let left = monkies[&target1].eval(&monkies);
        let right = monkies[&target2].eval(&monkies);

        if left == right {
            println!("Found equality with humn value {}", i);
            break;
        }
    }

    // Let's graph this fucker.
    // Following https://towardsdatascience.com/how-to-create-plot-in-rust-fdc6c024461c
    // use plotters::prelude::*;
    // let root_area = BitMapBackend::new("./test.png", (600, 400)).into_drawing_area();
    // root_area.fill(&WHITE).unwrap();

    // let mut ctx = ChartBuilder::on(&root_area)
    //     .set_label_area_size(LabelAreaPosition::Left, 40.0)
    //     .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
    //     .set_label_area_size(LabelAreaPosition::Right, 40.0)
    //     .set_label_area_size(LabelAreaPosition::Top, 40.0)
    //     .build_cartesian_2d(low..high, 3790998775025..10790998775025i64)
    //     .unwrap();

    //     ctx.draw_series(
    //         left_points.iter().map(|point| Circle::new(*point, 1.0_i64, &BLUE)),
    //     ).unwrap();

    //     ctx.draw_series(
    //         right_points.iter().map(|point| Circle::new(*point, 1.0_i64, &RED)),
    //     ).unwrap();
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
        }
        Sub(p1, p2) => {
            if optimize_monkies(&p1, monkies) | optimize_monkies(&p2, monkies) {
                true
            } else {
                monkies.insert(name.to_string(), Lit(e.eval(monkies)));
                false
            }
        }
        Div(p1, p2) => {
            if optimize_monkies(&p1, monkies) | optimize_monkies(&p2, monkies) {
                true
            } else {
                monkies.insert(name.to_string(), Lit(e.eval(monkies)));
                false
            }
        }
        Mul(p1, p2) => {
            if optimize_monkies(&p1, monkies) | optimize_monkies(&p2, monkies) {
                true
            } else {
                monkies.insert(name.to_string(), Lit(e.eval(monkies)));
                false
            }
        }
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
