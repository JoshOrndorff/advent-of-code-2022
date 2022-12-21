use std::collections::HashMap;
use sscanf::sscanf;

use Expr::*;
enum Expr {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Lit(f64),
}

// TODO if we need or want caching, move this method to a separate Monkey struct which also has a name
impl Expr {
    fn eval(&self, monkies: &HashMap<String, Self>/*, cache: &mut HashMap<String, f64>*/) -> f64 {
        match self {
            Lit(x) => *x,
            Add(p1, p2) => monkies[p1].eval(monkies) + monkies[p2].eval(monkies),
            Sub(p1, p2) => monkies[p1].eval(monkies) - monkies[p2].eval(monkies),
            Mul(p1, p2) => monkies[p1].eval(monkies) * monkies[p2].eval(monkies),
            Div(p1, p2) => monkies[p1].eval(monkies) / monkies[p2].eval(monkies),
        }

        // cache.insert()
    }
}


fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("read input file");
    let monkies = input.trim().lines().map(|l| {
            if l.len() < 15 {
                let (name, val) = sscanf!(l, "{}: {}", String, f64).unwrap();
                (name, Lit(val))
            } else {
                let (name, p1, op, p2) = sscanf!(l, "{}: {} {} {}", String, String, char, String).unwrap();
                (name, 
                    match op {
                        '+' => Add(p1, p2),
                        '-' => Sub(p1, p2),
                        '*' => Mul(p1, p2),
                        '/' => Div(p1, p2),
                        invalid => panic!("Invalid operation character {}", invalid),
                    }
                )
            }
            
        })
        .collect::<HashMap<_, _>>();

    // Initial data analysis SEEMS to show each monkey is used only once
    // So no cache / memoization should be necessary. Nonetheless, I'll create
    // it for debugging purposes
    // let mut cache: HashMap<String, f64> = HashMap::new();


    

    let part_1 = monkies["root".into()].eval(&monkies);
    println!("Part 1: {}", part_1);
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