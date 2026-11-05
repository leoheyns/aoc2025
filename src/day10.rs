use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    joltages: Vec<u32>,
}

fn bfs(m: Machine) -> u32 {
    let mut presses: u32 = 0;
    let mut states: HashSet<Vec<bool>> = HashSet::new();
    states.insert(m.lights);
    while states.iter().all(|state| state.iter().any(|x| *x)) {
        presses += 1;
        states = states
            .iter()
            .flat_map(|state| {
                m.buttons.iter().map(|b| {
                    state
                        .iter()
                        .enumerate()
                        .map(|(i, statebool)| statebool ^ b.contains(&(i as u32)))
                        .collect::<Vec<bool>>()
                })
            })
            .collect();
    }
    presses
}

fn find_increments(m: Machine) -> Vec<(Vec<u32>, u32)> {
    let mut result: HashMap<Vec<u32>, u32> = HashMap::new();
    for i in 0..((1 as u32) << m.buttons.len()) {
        let perm = (0..(m.buttons.len()))
            .map(|j| (i >> j) % 2 == 1)
            .collect::<Vec<bool>>();
        let presses: u32 = perm.iter().map(|b| *b as u32).sum();
        let increments = (0..(m.joltages.len()))
            .map(|i| {
                m.buttons
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| perm[*j])
                    .flat_map(|(_, b)| b.iter())
                    .filter(|x| **x == (i as u32))
                    .count()
            })
            .map(|c| c as u32)
            .collect::<Vec<u32>>();
        if !result.contains_key(&increments) {
            result.insert(increments, presses);
        } else if *result.get(&increments).unwrap() > presses {
            result.insert(increments, presses);
        }
    }

    return result.iter().map(|(k,v)| (k.clone(), v.clone())).collect::<Vec<(Vec<u32>, u32)>>();
}

fn part2(m: Machine) -> u32 {
    let increments_map = find_increments(m.clone());

    let mut cache: HashMap<Vec<u32>, u32> = HashMap::new();
    fn recursive(joltage_goal: Vec<u32>, increments_map: &Vec<(Vec<u32>, u32)>, cache: &mut HashMap<Vec<u32>, u32>) -> u32 {

        if joltage_goal.iter().all(|j| *j == 0) {
            return 0;
        }
        if cache.contains_key(&joltage_goal){
            return *cache.get(&joltage_goal).unwrap();
        }

        let result = increments_map
            .iter()
            .filter(|(inc, _)| {
                (0..(joltage_goal.len()))
                    .all(|i| (joltage_goal[i] % 2) == (inc[i] % 2))// && (joltage_goal[i] >= inc[i]))
            })
            .filter(|(inc, _)| {
                (0..(joltage_goal.len()))
                    .all(|i| joltage_goal[i] >= inc[i])
            })
            .map(|(inc, presses)| {
                (
                    recursive(
                        (0..(joltage_goal.len()))
                            .map(|i| (joltage_goal[i] - inc[i]) / 2)
                            .collect(), increments_map, cache
                    ),
                    presses,
                )
            })
            .filter(|(rec, _)| *rec < u32::MAX)
            .map(|(rec, presses)| rec * 2 + presses)
            .min()
            .unwrap_or(u32::MAX);
        
        cache.insert(joltage_goal, result);
        return result;
    }
    let result = recursive(m.joltages, &increments_map, &mut cache);
    return result;
}

pub fn _day10() {
    let input = include_str!("../inputs/day10")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| &s[1..(s.len() - 1)])
                .collect::<Vec<&str>>()
        })
        .map(|mlist| Machine {
            lights: mlist[0].chars().map(|c| c == '#').collect::<Vec<bool>>(),
            buttons: mlist[1..(mlist.len() - 1)]
                .iter()
                .map(|b| {
                    b.split(",")
                        .map(|numstring| numstring.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect(),
            joltages: mlist[mlist.len() - 1]
                .split(",")
                .map(|numstring| numstring.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        })
        .collect::<Vec<Machine>>();
    let part1: u32 = input.iter().map(|m| bfs(m.clone())).sum();
    println!("{part1:?}");

    let part2_result: Vec<u32> = input.iter().map(|m| part2(m.clone())).collect();

    let part2_result_forreal: u32= part2_result.iter().sum();
    println!("{part2_result_forreal:?}");

}
