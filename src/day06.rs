pub fn _day06() {
    let input = include_str!("../inputs/day06")
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let numbers: Vec<Vec<u64>> = input[..(input.len() - 1)]
        .iter()
        .map(|l| {
            l.iter()
                .map(|numstring| numstring.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let operations = input[input.len() - 1].clone();
    let mut part1 = 0;
    for i in 0..numbers[0].len() {
        let calc = numbers.iter().map(|nline| nline[i]).fold(
            match operations[i] {
                "*" => 1,
                _ => 0,
            },
            match operations[i] {
                "*" => |a, b| a * b,
                _ => |a, b| a + b,
            },
        );
        part1 += calc;
    }
    println!("{part1}");
    let input_chars = include_str!("../inputs/day06")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let numbers = (0..input_chars[0].len())
        .map(|i| {
            (0..(input_chars.len() - 1))
                .map(|j| input_chars[j][i])
                .filter(|c| *c != ' ')
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("-")
        .split("--")
        .map(|numline| {
            numline
                .split("-")
                .map(|ns| ns.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let part2: u64 = numbers
        .iter()
        .enumerate()
        .map(|(i, nline)| {
            nline.iter().fold(
                match operations[i] {
                    "*" => 1,
                    _ => 0,
                },
                match operations[i] {
                    "*" => |a, b| a * b,
                    _ => |a, b| a + b,
                },
            )
        })
        .sum();
    println!("{part2}")
}
