use crate::utils;

pub fn _day04() {
    let input = include_str!("../inputs/day04")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let part1: u32 = utils::conv2d(
        &input,
        |kernel| {
            if kernel[1][1] == '@'
                && (kernel
                    .iter()
                    .map(|kl: &Vec<char>| {
                        kl.iter()
                            .map(|c: &char| if *c == '@' { 1 } else { 0 })
                            .sum::<u32>()
                    })
                    .sum::<u32>()
                    < 5)
            {
                1
            } else {
                0
            }
        },
        '.',
        3,
    )
    .iter()
    .flat_map(|l| l.iter())
    .sum();
    println!("{part1}");

    let roll_count: u32 = input
        .iter()
        .flat_map(|l| l.iter().map(|c| if *c == '@' { 1 } else { 0 }))
        .sum();
    let mut go = true;
    let mut field = input;
    while go {
        let new_field = utils::conv2d(
            &field,
            |kernel| {
                if kernel[1][1] == '@'
                    && (kernel
                        .iter()
                        .map(|kl: &Vec<char>| kl.iter().filter(|c| **c == '@').count())
                        .sum::<usize>()
                        > 4)
                {
                    '@'
                } else {
                    '.'
                }
            },
            '.',
            3,
        );
        go = new_field != field;
        field = new_field;
    }
    let final_count: u32 = field
        .iter()
        .flat_map(|l| l.iter().map(|c| if *c == '@' { 1 } else { 0 }))
        .sum();
    let part2 = roll_count - final_count;
    println!("{part2}")
}
