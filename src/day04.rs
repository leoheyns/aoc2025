fn conv2d<I: Copy, O, F>(input: &Vec<Vec<I>>, func: F, filler: I, kernel_size: i32) -> Vec<Vec<O>>
where
    F: Fn(Vec<Vec<I>>) -> O,
{
    let idim = input.len();
    let jdim = input[0].len();
    let get_at_coord = |i: i32, j: i32| if i >= 0 && i < (idim as i32) && j >= 0 && j < (jdim as i32) {input[i as usize][j as usize]} else {filler};
    return (0..idim).map(|i| (0..jdim).map(|j| {
        func(
            (i as i32 - (kernel_size / 2)..=(i as i32 + kernel_size / 2)).map(|ki| (j as i32 - (kernel_size / 2)..=(j as i32 + kernel_size / 2)).map(|kj| get_at_coord(ki, kj)).collect::<Vec<I>>()).collect::<Vec<Vec<I>>>()
        )
    }).collect::<Vec<O>>()).collect::<Vec<Vec<O>>>();
} 



pub fn _day04(){
    let input = include_str!("../inputs/day04").lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let part1: u32 = conv2d(&input, |kernel| if kernel[1][1] == '@' && (kernel.iter().map(|kl: &Vec<char>| kl.iter().map(|c: &char| if *c == '@' {1} else {0}).sum::<u32>()).sum::<u32>() < 5) {1} else {0}, '.', 3).iter().flat_map(|l| l.iter()).sum();
    println!("{part1}");

    let roll_count: u32 = input.iter().flat_map(|l| l.iter().map(|c| if *c == '@' {1} else {0})).sum();
    let mut go = true;
    let mut field = input;
    while go{
        let new_field = conv2d(&field, |kernel| if kernel[1][1] == '@' && (kernel.iter().map(|kl: &Vec<char>| kl.iter().filter(|c| **c == '@').count()).sum::<usize>() > 4) {'@'} else {'.'}, '.', 3);
        go =  new_field != field;
        field = new_field;
    }
    let final_count: u32 = field.iter().flat_map(|l| l.iter().map(|c| if *c == '@' {1} else {0})).sum();
    let part2 = roll_count - final_count;
    println!("{part2}")
    

}