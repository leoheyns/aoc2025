use std::collections::HashMap;

pub fn _day11() {
    let input: HashMap<&str, Vec<&str>> = include_str!("../inputs/day11")
        .lines()
        .map(|l| l.split(": ").collect::<Vec<&str>>())
        .map(|splt| (splt[0], splt[1].split_whitespace().collect::<Vec<&str>>()))
        .collect::<HashMap<&str, Vec<&str>>>();
    let mut incoming_edges: HashMap<&str, u64> = HashMap::new();
    for (source, outgoing) in input.iter() {
        for target in outgoing.iter() {
            incoming_edges
                .entry(target)
                .and_modify(|c| *c += 1)
                .or_insert(1 as u64);
        }
        if !incoming_edges.contains_key(source) {
            incoming_edges.insert(source, 0);
        }
    }

    let mut frontier = incoming_edges
        .iter()
        .filter(|(_, c)| **c == 0)
        .map(|(s, _)| *s)
        .collect::<Vec<&str>>();
    println!("{frontier:?}");

    let mut sorted_input: Vec<&str> = Vec::new();

    for _ in 0..(input.len() + 1) {
        let current = frontier.pop().unwrap();
        if input.contains_key(current) {
            for target in input.get(current).unwrap() {
                incoming_edges.entry(target).and_modify(|c| *c -= 1);
                if *incoming_edges.get(target).unwrap() == 0 {
                    frontier.push(target);
                }
            }
        }
        sorted_input.push(current);
    }
    // println!("{sorted_input:?}");

    let mut parents: HashMap<&str, Vec<&str>> = HashMap::new();

    for s in sorted_input.iter() {
        parents.insert(s, Vec::new());
    }

    for (parent, children) in input.iter() {
        for child in children.iter() {
            parents.entry(child).and_modify(|v| v.push(*parent));
        }
    }

    let mut result: HashMap<&str, u64> = HashMap::new();
    let start_state: &str = "you";

    for s in sorted_input.iter() {
        if *s == start_state {
            result.insert(s, 1);
        } else {
            result.insert(s, 0);
        }
    }
    // println!("{parents:?}");

    for s in sorted_input.iter() {
        if *s != start_state {
            result.insert(
                s,
                parents
                    .get(s)
                    .unwrap()
                    .iter()
                    .map(|p| result.get(p).unwrap())
                    .sum(),
            );
        }
        // println!("{result:?}");
    }
    let endstate: &str = "out";
    let part1 = result.get(endstate).unwrap();
    println!("{part1:?}");

    let mut result_2: HashMap<&str, u64> = HashMap::new();
    let start_state_2: &str = "svr";

    for s in sorted_input.iter() {
        if *s == start_state_2 {
            result_2.insert(s, 1);
        } else {
            result_2.insert(s, 0);
        }
    }

    for s in sorted_input.iter() {
        if *s != start_state_2 {
            result_2.insert(
                s,
                parents
                    .get(s)
                    .unwrap()
                    .iter()
                    .map(|p| result_2.get(p).unwrap())
                    .sum(),
            );

            if *s == "dac" || *s == "fft" {
                for s2 in sorted_input.iter() {
                    if !(*s2 == "dac" || *s2 == "fft") {
                        result_2.insert(s2, 0);
                    }
                }
            }
        }
    }
    let part2 = result_2.get(endstate).unwrap();
    println!("{part2:?}");
}
