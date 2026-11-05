use std::collections::HashSet;

fn rotate(shape: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    return [
        [shape[2][0], shape[1][0], shape[0][0]].to_vec(),
        [shape[2][1], shape[1][1], shape[0][1]].to_vec(),
        [shape[2][2], shape[1][2], shape[0][2]].to_vec(),
    ]
    .to_vec();
}

fn mirror(shape: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    return [
        [shape[0][2], shape[0][1], shape[0][0]].to_vec(),
        [shape[1][2], shape[1][1], shape[1][0]].to_vec(),
        [shape[2][2], shape[2][1], shape[2][0]].to_vec(),
    ]
    .to_vec();
}

fn add_shape(region: &Vec<Vec<bool>>, shape: &Vec<Vec<bool>>, coord: (usize, usize)) -> Option<Vec<Vec<bool>>>{
    let mut newregion = region.clone();
    for i in 0..3{
        for j in 0..3{
            let newcoord = (coord.0 + i, coord.1 + j);
            if shape[i][j]{
                if region[newcoord.0][newcoord.1] {
                    return None;
                } else {
                    newregion[newcoord.0][newcoord.1] = true;
                }
            }
        }
    }
    return Some(newregion);
}

fn attempt(region: Vec<Vec<bool>>, remaining: Vec<u64>, shapes: &Vec<Vec<Vec<Vec<bool>>>>) -> bool{
    if remaining.iter().all(|x| *x == 0){
        return true;
    }
    let (shape_choice, _) = remaining.iter().enumerate().filter(|(_, c)| **c != 0).next().unwrap();
    let mut newremaining = remaining.clone();
    newremaining[shape_choice] -= 1;
    for i in 0..(region.len() - 2){
        for j in 0..(region[0].len() - 2){
            for perm in &shapes[shape_choice]{
                if match add_shape(&region, &perm, (i,j)){
                    Some(newregion) => attempt(newregion, newremaining.clone(), shapes),
                    None => false
                } {
                    return true
                }
            }
        }
    }
    return false
}

pub fn _day12() {
    let present_shapes = include_str!("../inputs/day12")
        .split("\r\n\r\n")
        .map(|s| {
            s.lines()
                .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>()
        })
        .collect::<Vec<Vec<Vec<bool>>>>();
    // println!("{present_shapes:?}");

    let perm_shapes: Vec<Vec<Vec<Vec<bool>>>> = present_shapes.iter().map(|s| {
        let mut rot = s.clone();
        let mut perms: HashSet<Vec<Vec<bool>>> = HashSet::new();
        for _ in 0..4 {
            perms.insert(rot.clone());
            perms.insert(mirror(&rot).clone());
            rot = rotate(&rot);
        }
        perms.iter().map(|a| a.clone()).collect::<Vec<Vec<Vec<bool>>>>()
    }).collect();

    let regions = include_str!("../inputs/day12_2")
        .lines()
        .map(|l| l.split(": ").collect::<Vec<&str>>())
        .map(|splt| {
            (
                splt[0].split("x").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>(),
                splt[1].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>(),
            )
        }).collect::<Vec<(Vec<u64>, Vec<u64>)>>();
    let present_sizes = present_shapes.iter().map(|s| s.iter().map(|l| l.iter().map(|b| *b as u64).sum::<u64>()).sum::<u64>()).collect::<Vec<u64>>();
    println!("{present_sizes:?}");
    let mut valid = 0;
    let mut invalid = 0;
    for region in regions{
        let shape_spots = (region.0[0] / 3) * (region.0[1] / 3);
        let total_space = region.0[0] * region.0[1];
        if region.1.iter().sum::<u64>() <= shape_spots{
            valid += 1;
        } else if (0..present_sizes.len()).map(|i| region.1[i] * present_sizes[i]).sum::<u64>() > total_space{
            invalid += 1;
        }
    }

    println!("valid: {valid:?}");
    println!("invalid: {invalid:?}");

    // for region in regions{
    //     let empty_region: Vec<Vec<bool>> = (0..(region.0[0])).map(|_| (0..(region.0[1])).map(|_| false).collect::<Vec<bool>>()).collect();
    //     let part1_test = attempt(empty_region, region.1, &perm_shapes);
    //     println!{"{part1_test:?}"}
    // }
    // let region0 = regions[0].clone();


    
}
