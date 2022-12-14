use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Clone)]
struct ExplorationState {
    location: (char, char),
    elephant: (char, char),
    open_valves: BTreeSet<(char, char)>,
    score: u64,
}

impl PartialEq for ExplorationState {
    fn eq(&self, other: &Self) -> bool {
        self.open_valves == other.open_valves
            && self.score == other.score
            && ((self.location == other.location && self.elephant == other.elephant)
                || (self.location == other.elephant && self.elephant == other.location))
    }
}

impl Hash for ExplorationState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let joined_location_elephant = if self.location < self.elephant {
            (self.location, self.elephant)
        } else {
            (self.elephant, self.location)
        };
        joined_location_elephant.hash(state);
        self.open_valves.hash(state);
        self.score.hash(state);
    }
}

impl ExplorationState {
    fn local_valve_closed(&self) -> bool {
        !self.open_valves.contains(&self.location)
    }

    fn elephant_valve_closed(&self) -> bool {
        !self.open_valves.contains(&self.elephant)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Status {
    score: u64,
    time_left: u64,
}

impl Status {
    fn dominates(&self, other: &Self) -> bool {
        self.score >= other.score && self.time_left >= other.time_left
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Reads input file");

    // We model the cave as a mapping from each node name to its flow rate and its neighbors
    let mut cave: HashMap<(char, char), (u64, Vec<(char, char)>)> = HashMap::new();
    input.trim().lines().for_each(|l| {
        let equal_index = l.find('=').unwrap();
        let semi_index = l.find(';').unwrap();
        let comma_index = l.find(',').unwrap_or(l.len());

        let location = (l.chars().nth(6).unwrap(), l.chars().nth(7).unwrap());
        let flow = l[equal_index + 1..semi_index].parse().unwrap();

        let neighbors: Vec<(char, char)> = l[comma_index - 2..]
            .trim()
            .split(", ")
            .map(|s| (s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap()))
            .collect();

        cave.insert(location, (flow, neighbors));
    });

    let starting_state = ExplorationState {
        location: ('A', 'A'),
        elephant: ('A', 'A'),
        open_valves: BTreeSet::new(),
        score: 0,
    };

    let part_1 = explore_valves(
        &cave,
        starting_state.clone(),
        Status {
            score: 0,
            time_left: 30,
        },
    );
    println!("part 1: {:?}", part_1);

    let part_2 = explore_valves_with_elephant(&cave, starting_state);
    println!("part 2: {:?}", part_2);
}

/// My homegrown attempt at prioritizing states by statuses that dominate one another.
/// This was "good enough" for part one, but still blew up too fast for part 2.
fn explore_valves(
    cave: &HashMap<(char, char), (u64, Vec<(char, char)>)>,
    starting_state: ExplorationState,
    starting_status: Status,
) -> u64 {
    // The set of states to explore. mapped to the time remaining when exploring them.
    // If at some point we find a faster way to get to the same state, we don't explore the slower.
    let mut to_explore = HashMap::<ExplorationState, Vec<Status>>::new();
    to_explore.insert(starting_state, vec![starting_status]);

    let mut explored = HashMap::<ExplorationState, Vec<Status>>::new();

    while !to_explore.is_empty() {
        // Kinda convoluted way to get ownership of a single kv from the map
        let current_state = to_explore
            .iter()
            .next()
            .map(|(k, _)| k)
            .expect("to_explore is not empty; we checked in the loop condition")
            .clone();
        let (current_state, statuses) = to_explore
            .remove_entry(&current_state)
            .expect("Item should exist,we just got the reference from the map itself.");
        let (flow_rate, neighbor_locations) = &cave[&current_state.location];

        for Status { time_left, score } in statuses {
            // println!("{:?}, time_left {:?}", current_state, time_left);
            // println!(" Curious state is in to_explore: {:?}", to_explore.contains_key(&dd));

            explored
                .entry(current_state.clone())
                .and_modify(|v| v.push(Status { time_left, score }))
                .or_insert(vec![Status { time_left, score }]);

            if time_left == 0 {
                continue;
            }

            let time_left = time_left - 1;

            // Figure out all the neighboring states.
            let mut neighbors = Vec::new();

            if current_state.local_valve_closed() && *flow_rate > 0 {
                let mut open_valved_neighbor = current_state.clone();
                open_valved_neighbor
                    .open_valves
                    .insert(current_state.location.clone());

                neighbors.push((
                    open_valved_neighbor,
                    Status {
                        score: score + flow_rate * time_left,
                        time_left,
                    },
                ));
            }

            for neighbor_location in neighbor_locations {
                // println!("  neighbor location: {}", neighbor_location);
                let mut neighbor = current_state.clone();
                neighbor.location = neighbor_location.clone();

                neighbors.push((neighbor, Status { score, time_left }));
            }

            // Add the neighbors to the states to explore, unless better ones are already
            // there or it is already explored
            for (neighbor, status) in neighbors {
                if explored
                    .get(&neighbor)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|exp| exp.dominates(&status))
                {
                    continue;
                }

                if to_explore
                    .get(&neighbor)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|exp| exp.dominates(&status))
                {
                    continue;
                }

                // None of the explored or to-be-explored entries dominate this one, so we will plan to explore it
                // Schedule its exploration, and cull the exploration of any statuses it dominates
                to_explore
                    .entry(neighbor)
                    .and_modify(|v| {
                        v.retain(|s| !status.dominates(s));
                        v.push(status.clone());
                    })
                    .or_insert(vec![status]);
            }
        }
    }

    // Get the highest score state out of the explored set.
    let mut result = 0;
    for (_, statuses) in explored {
        for status in statuses {
            if status.score > result {
                result = status.score;
            }
        }
    }

    result
}

/// For part two, my status idea was no longer killing enough states. So here I take a different approach.
/// After each step of the problem, we only keep the 1000 highest scoring states. TBH, I don't have a good
/// sense of why 1000 was the right number to use. I know empirically that going to 100 was too low, but
/// going up to 10k or even 100k also worked and ran in reasonable time.
fn explore_valves_with_elephant(
    cave: &HashMap<(char, char), (u64, Vec<(char, char)>)>,
    starting_state: ExplorationState,
) -> u64 {
    let mut current_generation = HashSet::from([starting_state]);
    let mut time_left = 26;

    while time_left > 0 {
        let mut next_generation = HashSet::new();
        time_left = time_left - 1;

        for current_state in current_generation {
            let (flow_rate, neighbor_locations) = &cave[&current_state.location];
            let (elephant_flow, elephant_neighbors) = &cave[&current_state.elephant];

            // Figure out all the neighboring states.
            let mut neighbors = Vec::new();

            // If we both open valves (Be careful! You can't both open the _same_ valve.)
            if current_state.local_valve_closed()
                && *flow_rate > 0
                && current_state.elephant_valve_closed()
                && *elephant_flow > 0
                && current_state.location != current_state.elephant
            {
                let mut neighbor = current_state.clone();
                neighbor.open_valves.insert(current_state.location);
                neighbor.open_valves.insert(current_state.elephant);
                neighbor.score = current_state.score + (flow_rate + elephant_flow) * time_left;
                neighbors.push(neighbor);
            }

            // If just I open a valve
            if current_state.local_valve_closed() && *flow_rate > 0 {
                for elephant_neighbor in elephant_neighbors {
                    let mut neighbor = current_state.clone();
                    neighbor.open_valves.insert(current_state.location);
                    neighbor.elephant = *elephant_neighbor;
                    neighbor.score = current_state.score + flow_rate * time_left;

                    neighbors.push(neighbor);
                }
            }

            // If just the elephant opens a valve
            if current_state.elephant_valve_closed() && *elephant_flow > 0 {
                for neighbor_location in neighbor_locations {
                    let mut neighbor = current_state.clone();
                    neighbor.open_valves.insert(current_state.elephant);
                    neighbor.location = *neighbor_location;
                    neighbor.score = current_state.score + elephant_flow * time_left;

                    neighbors.push(neighbor);
                }
            }

            // If nobody opens a valve
            for neighbor_location in neighbor_locations {
                for elephant_neighbor in elephant_neighbors {
                    let mut neighbor = current_state.clone();
                    neighbor.location = *neighbor_location;
                    neighbor.elephant = *elephant_neighbor;

                    neighbors.push(neighbor);
                }
            }

            next_generation.extend(neighbors);
        }

        // Optimization: Only explore the N highest scoring items in the next generation
        current_generation = next_generation
            .into_iter()
            .sorted_by(|a, b| b.score.cmp(&a.score))
            .take(1000)
            .collect();
    }

    current_generation
        .iter()
        .fold(0, |acc, s| if s.score > acc { s.score } else { acc })
}

#[allow(dead_code)] //for some reason feature = "test" made the tests not compile
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn same_hash() {
    let s1 = ExplorationState {
        location: ('A', 'A'),
        elephant: ('B', 'B'),
        open_valves: BTreeSet::new(),
        score: 0,
    };
    let h1 = hash(&s1);

    let s2 = ExplorationState {
        location: ('B', 'B'),
        elephant: ('A', 'A'),
        open_valves: BTreeSet::new(),
        score: 0,
    };
    let h2 = hash(&s2);

    assert_eq!(h1, h2);
}

#[test]
fn equal() {
    let s1 = ExplorationState {
        location: ('A', 'A'),
        elephant: ('B', 'B'),
        open_valves: BTreeSet::new(),
        score: 0,
    };

    let s2 = ExplorationState {
        location: ('B', 'B'),
        elephant: ('A', 'A'),
        open_valves: BTreeSet::new(),
        score: 0,
    };

    assert_eq!(s1, s2);
}

#[test]
fn set_key_equiv() {
    let s1 = ExplorationState {
        location: ('A', 'A'),
        elephant: ('B', 'B'),
        open_valves: BTreeSet::new(),
        score: 0,
    };

    let s2 = ExplorationState {
        location: ('B', 'B'),
        elephant: ('A', 'A'),
        open_valves: BTreeSet::new(),
        score: 0,
    };

    let mut m = HashSet::<ExplorationState>::new();

    m.insert(s1);
    assert!(m.contains(&s2));

    m.insert(s2);
    assert_eq!(m.len(), 1);
}

#[test]
fn set_equality() {
    let mut a = BTreeSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);

    let mut b = BTreeSet::new();
    b.insert(2);
    b.insert(3);
    b.insert(1);
    b.insert(3);

    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));
}
