use sscanf::sscanf;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("read input file");

    let occupied: HashSet<(u32, u32, u32)> = input
        .trim()
        .lines()
        .map(|l| sscanf!(l, "{},{},{}", u32, u32, u32).unwrap())
        // Move the entire thing up, right, and away by one cell so it has no 0 coordinates
        .map(|(x, y, z)| (x + 1, y + 1, z + 1))
        .collect();

    let mut total_surface_area = 0;
    let mut max = (0, 0, 0);

    for (x, y, z) in &occupied {
        // Input analysis - Check just how big this droplet is
        if *x > max.0 {
            max.0 = *x;
        }
        if *y > max.1 {
            max.1 = *y;
        }
        if *z > max.2 {
            max.2 = *z;
        }

        // Solve part 1, calculating total surface area including internal
        for neighbor in neighbors((*x, *y, *z), max) {
            if !occupied.contains(&neighbor) {
                total_surface_area += 1;
            }
        }
    }

    println!("Maximum dimensions of bounding box around droplet: {:?}", max);
    println!("Total surface area! {}", total_surface_area);

    // Queue of cells that we know are filled with steam, but have not yet expanded
    // Start with steam just in the minimum corner of the bounding box
    let mut to_explore = VecDeque::from([(0u32, 0u32, 0u32)]);
    let mut steam = HashSet::from([(0u32, 0u32, 0u32)]);

    let mut exterior_surface_area = 0;

    while !to_explore.is_empty() {
        let current_steam = to_explore.pop_front().expect("Just checked it isn't empty");
        
        // The cell we are currently exploring should always be already known to be steam
        assert!(steam.contains(&current_steam));

        for neighbor in neighbors(current_steam, max) {
            if occupied.contains(&neighbor) {
                exterior_surface_area += 1;
            } else if !steam.contains(&neighbor) {
                steam.insert(neighbor);
                to_explore.push_back(neighbor);
            }
        }
    }

    println!("Exterior surface area: {}", exterior_surface_area);
}

/// Given a cell's coordinates, calculates the 6 neighboring cells
/// Considers boundaries to the area of (0,0,0), (xmax, ymax, zmax).
fn neighbors((x, y, z): (u32, u32, u32), (xmax, ymax, zmax): (u32, u32, u32)) -> impl Iterator<Item= (u32, u32, u32)> {
    let mut v = Vec::new();
    
    if x > 0 {
        v.push((x - 1, y, z));
    }
    if x < xmax {
        v.push((x + 1, y, z));
    }
    if y > 0 {
        v.push((x, y - 1, z));
    }
    if y < ymax {
        v.push((x, y + 1, z));
    }
    if z > 0 {
        v.push((x, y, z - 1));
    }
    if z < zmax {
        v.push((x, y, z + 1));
    }
    
    v.into_iter()
}