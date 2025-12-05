pub fn _day02(){
    let input = include_str!("../inputs/day02").split(",").map(|pair| pair.split("-").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    fn checkrange(start: &str, end: &str) -> u64{
        let startnum = start.parse::<u64>().unwrap();
        let endnum = end.parse::<u64>().unwrap();

        let mut total = 0;
        for i in startnum..(endnum + 1){
            for patlen in 1..((i.to_string().len() / 2) + 1){
                let patmul = u64::pow(10, patlen as u32);
                let pattern = i % patmul;
                if pattern < (patmul / 10){
                    continue;
                }
                let mut candidate = pattern;
                    while candidate < i{
                        candidate = candidate * patmul + pattern
                    }
                if candidate == i{
                    total += i;
                    break;
                }
            }
        }
        return total;
    }
    let mut total = 0;
    for pair in input.iter(){
        let count = checkrange(pair[0], pair[1]);
        total += count;
    }
    println!("total {total}")

}