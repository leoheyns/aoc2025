pub fn _day01(){
    let lines = include_str!("../inputs/day01").lines();
    let mut state: i32 = 50;
    let mut count = 0;

    for line in lines{
        let state_0 = state == 0;
        let tics = line[1..].parse::<i32>().unwrap();
        let mut rotations = tics / 100;
        let real_tics = tics % 100;
        state += real_tics * (if line.chars().nth(0).unwrap() == 'L' {-1} else {1});
        if (state < 1 && !state_0) || state > 99{
            rotations += 1;
        }
        state = (state + 100000) % 100;
        println!("{state}");
        count += rotations;
    }
    println!("");

    println!("{count}")
}