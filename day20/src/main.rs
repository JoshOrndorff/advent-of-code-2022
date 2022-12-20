// use itertools::Itertools;
// use sscanf::sscanf;
// use std::collections::{HashSet, HashMap, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("read input file");

    // The data is stored as a vec of pairs. The pair is the original position then the value
    let mut data: Vec<(usize, i64)> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .enumerate() /*.map(|(n, v)| (n as i64, v))*/
        .collect();
    let length = data.iter().count();

    for i in 0..length {
        let mut starting_position = 0usize;
        let mut value = 0;
        for search_loc in 0..length {
            if data[search_loc].0 == i {
                starting_position = search_loc;
                value = data[search_loc].1;
                break;
            }
        }

        let ending_position = ((starting_position as i64 + 2 * (length as i64 - 1) + value)
            % (length as i64 - 1)) as usize;
        let in_transit = data.remove(starting_position);

        // When values are at the seam, the example appends them to the end, not the beginning
        if ending_position == 0 {
            data.push(in_transit);
        } else {
            data.insert(ending_position, in_transit);
        }

        // println!("Data after round {}: {:?}", i, data.iter().map(|(_,v)| v).collect::<Vec<_>>());
    }

    let mut zero_index = 0;
    for search_loc in 0..length {
        if data[search_loc].1 == 0 {
            zero_index = search_loc;
            break;
        }
    }

    let part_1 = data[(zero_index + 1000) % length].1
        + data[(zero_index + 2000) % length].1
        + data[(zero_index + 3000) % length].1;

    println!("Part 1 {:?}", part_1);
}
