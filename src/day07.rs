use crate::utils::{self, conv2d};

pub fn _day07() {
    let input: Vec<Vec<char>> = include_str!("../inputs/day07")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut state = input.clone();
    for _ in 0..input.len() {
        state = utils::conv2d(
            &state,
            |kernel| {
                if kernel[1][1] == '.' {
                    if (kernel[0][1] == 'S' || kernel[0][1] == '|')
                        || (kernel[0][0] == '|' && kernel[1][0] == '^')
                        || (kernel[0][2] == '|' && kernel[1][2] == '^')
                    {
                        '|'
                    } else {
                        '.'
                    }
                } else {
                    kernel[1][1]
                }
            },
            '.',
            3,
        )
    }
    let part1: usize = conv2d(
        &state,
        |kernel| kernel[1][1] == '^' && kernel[0][1] == '|',
        '.',
        3,
    )
    .iter()
    .map(|l| l.iter().filter(|b| **b).count())
    .sum();

    println!("{part1}");

    let input: Vec<Vec<i64>> = include_str!("../inputs/day07")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => 1,
                    '^' => -1,
                    _ => 0,
                })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let mut state = input.clone();
    for _ in 0..input.len() {
        state = utils::conv2d(
            &state,
            |kernel| {
                if kernel[1][1] == 0 {
                    let mut candidate = 0;
                    if kernel[0][1] > 0 {
                        candidate += kernel[0][1]
                    }
                    if kernel[1][0] == -1 && kernel[0][0] > 0 {
                        candidate += kernel[0][0]
                    }
                    if kernel[0][2] > 0 && kernel[1][2] == -1 {
                        candidate += kernel[0][2]
                    }
                    candidate
                } else {
                    kernel[1][1]
                }
            },
            0,
            3,
        )
    }
    let part2: i64 = state[state.len() - 1].iter().sum();
    println!("{part2}")
}
