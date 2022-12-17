// use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct ExplorationState {
    location: String,
    open_valves: BTreeSet<String>,
}

impl ExplorationState {
    fn local_valve_closed(&self) -> bool {
        !self.open_valves.contains(&self.location)
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
    let mut cave: BTreeMap<String, (u64, Vec<String>)> = BTreeMap::new();
    input.trim().lines().for_each(|l| {
        let equal_index = l.find('=').unwrap();
        let semi_index = l.find(';').unwrap();
        let comma_index = l.find(',').unwrap_or(l.len());

        let location = l[6..8].to_string();
        let flow = l[equal_index + 1..semi_index].parse().unwrap();

        let neighbors: Vec<String> = l[comma_index - 2..]
            .trim()
            .split(", ")
            .map(String::from)
            .collect();

        cave.insert(location, (flow, neighbors));
    });

    // The set of states to explore. mapped to the time remaining when exploring them.
    // If at some point we find a faster way to get to the same state, we don't explore the slower.
    let mut to_explore = BTreeMap::<ExplorationState, Vec<Status>>::new();
    let starting_state = ExplorationState {
        location: "AA".into(),
        open_valves: BTreeSet::new(),
    };
    let starting_status = Status {
        score: 0,
        time_left: 30,
    };
    to_explore.insert(starting_state, vec![starting_status]);

    let mut explored = BTreeMap::<ExplorationState, Vec<Status>>::new();

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

        for Status{time_left, score} in statuses {
            // println!("{:?}, time_left {:?}", current_state, time_left);
            // println!(" Curious state is in to_explore: {:?}", to_explore.contains_key(&dd));

            explored
                .entry(current_state.clone())
                .and_modify(|v| v.push(Status{time_left, score}))
                .or_insert(vec![Status{time_left, score}]);

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

                neighbors.push((open_valved_neighbor, Status { score: score + flow_rate * time_left, time_left}));
            }

            for neighbor_location in neighbor_locations {
                // println!("  neighbor location: {}", neighbor_location);
                let mut neighbor = current_state.clone();
                neighbor.location = neighbor_location.clone();

                neighbors.push((neighbor, Status{score, time_left}));
            }



            // Add the neighbors to the states to explore, unless better ones are already
            // there or it is already explored
            for (neighbor, status) in neighbors {
                if explored.get(&neighbor).unwrap_or(&Vec::new()).iter().any(|exp| exp.dominates(&status)) {
                    continue;
                }

                if to_explore.get(&neighbor).unwrap_or(&Vec::new()).iter().any(|exp| exp.dominates(&status)) {
                    continue;
                }

                // None of the explored or to-be-explored entries dominate this one, so we will plan to explore it
                // Schedule its exploration, and cull the exploration of any statuses it dominates
                to_explore.entry(neighbor).and_modify(|v| {
                    v.retain(|s| !status.dominates(s));
                    v.push(status.clone());
                }).or_insert(vec![status]);
            }
        }
    }

    // Get the highest score state out of the explored set.
    let mut part_1 = 0;
    for (_, statuses) in explored {
        for status in statuses {
            if status.score > part_1 {
                part_1 = status.score;
            }
        }
    }

    // let part_1 = find_best_strategy(&cave, 30, 0, starting_state);
    // // todo find the explored state with the highest score

    println!("part 1: {:?}", part_1);
}

// fn find_best_strategy(
//     cave: &BTreeMap<String, (u64, Vec<String>)>,
//     time_left: u64,
//     score: u64,
//     current_state: ExplorationState,
// ) -> u64 {
//     if time_left == 0 {
//         return score;
//     }

//     if time_left > 15 {
//         println!(
//             "Exploring location {}. Time left: {}, score: {}",
//             current_state.location, time_left, score
//         );
//     }

//     let (flow_rate, neighbor_locations) = &cave[&current_state.location];

//     // Accumulate the best scores for all of the neighbors
//     let mut neighbor_scores: Vec<u64> = Vec::new();

//     if current_state.local_valve_closed() && *flow_rate > 0 {
//         let mut open_valved_neighbor = current_state.clone();
//         open_valved_neighbor
//             .open_valves
//             .insert(current_state.location.clone());

//         let open_valved_neighbor_score = find_best_strategy(
//             cave,
//             time_left - 1,
//             score + flow_rate * time_left,
//             open_valved_neighbor,
//         );
//         neighbor_scores.push(open_valved_neighbor_score);
//     }

//     for neighbor_location in neighbor_locations {
//         let mut neighbor = current_state.clone();
//         neighbor.location = neighbor_location.clone();

//         let neighbor_score = find_best_strategy(cave, time_left - 1, score, neighbor);
//         neighbor_scores.push(neighbor_score);
//     }

//     // Return the best score available by all the neighbor states
//     *neighbor_scores.iter().max().unwrap()
// }
