// 776 is too low

use sscanf::sscanf;
use std::collections::{HashMap, HashSet, VecDeque};

type Quad = (u64, u64, u64, u64);

fn add_quad((a, b, c, d): Quad, (w, x, y, z): Quad) -> Quad {
    (a + w, b + x, c + y, d + z)
}

/// Warning, this one panics. Call quad_covers first to check
fn sub_quad((a, b, c, d): Quad, (w, x, y, z): Quad) -> Quad {
    (a - w, b - x, c - y, d - z)
}

fn quad_covers((a, b, c, d): Quad, (w, x, y, z): Quad) -> bool {
    a >= w && b >= x && c >= y && d >= z
}

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore_bot_cost: Quad,
    clay_bot_cost: Quad,
    obsidian_bot_cost: Quad,
    geode_bot_cost: Quad,
}

impl From<&str> for Blueprint {
    fn from(s: &str) -> Self {
        let (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = sscanf!(s, "Blueprint {u64}: Each ore robot costs {u64} ore. Each clay robot costs {u64} ore. Each obsidian robot costs {u64} ore and {u64} clay. Each geode robot costs {u64} ore and {u64} obsidian.").unwrap();

        Self {
            id,
            ore_bot_cost: (ore_ore, 0, 0, 0),
            clay_bot_cost: (clay_ore, 0, 0, 0),
            obsidian_bot_cost: (obsidian_ore, obsidian_clay, 0, 0),
            geode_bot_cost: (geode_ore, 0, geode_obsidian, 0),
        }
    }
}

impl Blueprint {
    fn quality(&self) -> u64 {
        self.id * self.find_max_geodes()
    }

    fn find_max_geodes(&self) -> u64 {
        println!("Blueprint is {:?}", self);

        let starting_state = State {
            bots: (1, 0, 0, 0),
            stuff: (0, 0, 0, 0),
        };

        let mut current_generation = HashSet::from([starting_state]);
        let mut time_left = 24;

        while time_left > 0 {
            println!("Time left: {}", time_left);
            let mut next_generation = HashSet::new();

            for current_state in &current_generation {
                // println!("  Current state: {:?}", current_state);

                let children = current_state.get_next_states(self);
                for child in children {
                    // Check if this one is dominated by any already-scheduled. If so, ditch it, otherwise, cull and add
                    if next_generation.iter().any(|ns: &State| ns.dominates(&child)) {
                        continue;
                    } else {
                        next_generation.retain(|ns| !child.dominates(ns));
                        next_generation.insert(child);
                    }
                }
            }

            current_generation = next_generation;
            time_left -= 1;
        }

        current_generation.into_iter().fold(0, |acc, s| if s.stuff.3 > acc { s.stuff.3 }else { acc })
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct State {
    bots: Quad,
    stuff: Quad,
}

impl State {

    /// This is the key definition that determines which states are worthy of being searched.
    /// If we are aggressive about saying a state dominates another, the algorithm will terminate
    /// faster, but may miss some solutions. Let's start out by not being very aggressive
    /// to guarantee that we do't miss the best solution. Then only get more aggressive if
    /// the algorithm doesn't terminate fast enough.
    fn dominates(&self, other: &Self) -> bool {

        // Attempt 2: More geodes is always better.
        if self.stuff.3 > other.stuff.3 {
            return true;
        } else if other.stuff.3 > self.stuff.3 {
            return false;
        }

        // Having compared geodes, let's now compare geode bots
        if self.bots.3 > other.bots.3 {
            return true;
        } else if other.bots.3 > self.bots.3 {
            return false;
        }

        // I had considered doing a similar comparison for the obsidian, but it fails
        // to find the best solutions for my real input.
        // if self.stuff.2 > other.stuff.2 {
        //     return true;
        // } else if other.stuff.2 > self.stuff.2 {
        //     return false;
        // }

        // if self.bots.2 > other.bots.2 {
        //     return true;
        // } else if other.bots.2 > self.bots.2 {
        //     return false;
        // }

        // This was my simple first attempt. Let's retain it as the ultimate fallback
        quad_covers(self.stuff, other.stuff) && quad_covers(self.bots, other.bots)
    }

    fn get_next_states(&self, blueprint: &Blueprint) -> impl Iterator<Item = Self> {

        // There is always a simple child where nothing gets built. We will construct
        // this one first, put it in the vec, and use it as a template.
        let template_child = State {
            bots: self.bots,
            stuff: add_quad(self.stuff, self.bots),
        };
        let mut children = Vec::from([template_child]);

        // See if we can build another ore bot
        if quad_covers(self.stuff, blueprint.ore_bot_cost) {
            let ore_child = State {
              bots: add_quad(template_child.bots, (1, 0, 0, 0)),
              stuff: sub_quad(template_child.stuff, blueprint.ore_bot_cost),
            };
            children.push(ore_child);
        }

        // See if we can build another clay bot
        if quad_covers(self.stuff, blueprint.clay_bot_cost) {
            let clay_child = State {
              bots: add_quad(template_child.bots, (0, 1, 0, 0)),
              stuff: sub_quad(template_child.stuff, blueprint.clay_bot_cost),
            };
            children.push(clay_child);
        }

        // See if we can build another Obsidian bot
        if quad_covers(self.stuff, blueprint.obsidian_bot_cost) {
            let obsidian_child = State {
              bots: add_quad(template_child.bots, (0, 0, 1, 0)),
              stuff: sub_quad(template_child.stuff, blueprint.obsidian_bot_cost),
            };
            children.push(obsidian_child);
        }

        // See if we can build another geode bot
        if quad_covers(self.stuff, blueprint.geode_bot_cost) {
            let geode_child = State {
              bots: add_quad(template_child.bots, (1, 0, 0, 1)),
              stuff: sub_quad(template_child.stuff, blueprint.geode_bot_cost),
            };
            children.push(geode_child);
        }

        children.into_iter()
    }
}

fn main() {

    // example_1_geodes();
    let input = std::fs::read_to_string("./input.txt").expect("read input file");
    
    let part_1: u64 = input
        .trim()
        .lines()
        .map(Blueprint::from)
        .map(|b| {
            let q = b.quality();
            println!("Quality: {}", q);
            q
        })
        .sum();

    println!("Part 1: {:?}", part_1);
}

#[test]
fn example_1_geodes() {

    let s = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    let b = Blueprint::from(s);

    assert_eq!(9, b.find_max_geodes());
    assert_eq!(9, b.quality());
}

#[test]
fn example_2_geodes() {

    let s = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let b = Blueprint::from(s);

    assert_eq!(12, b.find_max_geodes());
    assert_eq!(24, b.quality());
}

