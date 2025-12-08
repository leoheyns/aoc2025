use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u32>().unwrap();
    match day {
        1 => day01::_day01(),
        2 => day02::_day02(),
        3 => day03::_day03(),
        4 => day04::_day04(),
        5 => day05::_day05(),
        6 => day06::_day06(),
        7 => day07::_day07(),
        _ => {}
    }
}
