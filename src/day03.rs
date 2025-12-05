pub fn _day03(){
    let input = include_str!("../inputs/day03").lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>()).map(|bank| {
        let mut index = 0;
        let mut bat = 0;
        for i in (0..12).rev(){
            let (ind, maxbat) = bank[index..(bank.len() - i)].iter().enumerate().max_by(|(idx0, val0), (idx1, val1)| if val0 == val1 {idx1.cmp(idx0)} else {val0.cmp(val1)}).unwrap();
            index += ind + 1;
            bat = bat * 10 + maxbat
        }
        bat
    });
    // let output: Vec<u64> = input.collect::<Vec<u64>>();
    let output: u64 = input.sum();
    println!("{output:?}")
}