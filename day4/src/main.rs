fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read file");

    let parsed_input = input.lines().map(|s| {
        let dash1 = s.find('-').unwrap();
        let comma = s.find(',').unwrap();
        let dash2 = s.rfind('-').unwrap();

        let start1: u32 = s[0..dash1].parse().unwrap();
        let end1: u32 = s[dash1 + 1..comma].parse().unwrap();
        let start2: u32 = s[comma + 1..dash2].parse().unwrap();
        let end2: u32 = s[dash2 + 1..].parse().unwrap();

        (start1, end1, start2, end2)
    });

    // --------------

    let part_1 = parsed_input
        .clone()
        .filter(|(s1, e1, s2, e2)| (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1))
        .count();

    println!("{:?}", part_1);

    // --------------

    let part_2 = parsed_input
        .filter(|(s1, e1, s2, e2)| {
            (s1 <= s2 && e1 >= e2)
                || (s2 <= s1 && e2 >= e1)
                || (s1 <= s2 && e1 >= s2)
                || (s1 >= s2 && e2 >= s1)
        })
        .count();

    println!("{:?}", part_2);
}
