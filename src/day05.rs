use std::cmp;

pub fn _day05() {
    let input = include_str!("../inputs/day05")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let ranges = input[0]
        .lines()
        .map(|l| {
            l.split("-")
                .map(|numstring| numstring.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let part1 = input[1]
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|r| *id >= r[0] && *id <= r[1]))
        .count();
    println!("{part1}");

    let mut final_ranges: Vec<Vec<u64>> = vec![];
    for range in ranges.iter() {
        let mut candidate = range.clone();
        for (i, testrange) in final_ranges.clone().iter().enumerate().rev() {
            if (candidate[0] >= testrange[0] && candidate[0] <= testrange[1])
                || (candidate[1] >= testrange[0] && candidate[1] <= testrange[1])
                || (testrange[0] >= candidate[0] && testrange[0] <= candidate[1])
                || (testrange[1] >= candidate[0] && testrange[1] <= candidate[1])
            {
                candidate = vec![
                    cmp::min(candidate[0], testrange[0]),
                    cmp::max(candidate[1], testrange[1]),
                ];
                final_ranges.remove(i);
            }
        }
        final_ranges.push(candidate);
    }
    let part2: u64 = final_ranges.iter().map(|r| r[1] - r[0] + 1).sum();
    println!("{part2}");
}
