pub fn conv2d<I: Copy, O, F>(
    input: &Vec<Vec<I>>,
    func: F,
    filler: I,
    kernel_size: i32,
) -> Vec<Vec<O>>
where
    F: Fn(Vec<Vec<I>>) -> O,
{
    let idim = input.len();
    let jdim = input[0].len();
    let get_at_coord = |i: i32, j: i32| {
        if i >= 0 && i < (idim as i32) && j >= 0 && j < (jdim as i32) {
            input[i as usize][j as usize]
        } else {
            filler
        }
    };
    return (0..idim)
        .map(|i| {
            (0..jdim)
                .map(|j| {
                    func(
                        (i as i32 - (kernel_size / 2)..=(i as i32 + kernel_size / 2))
                            .map(|ki| {
                                (j as i32 - (kernel_size / 2)..=(j as i32 + kernel_size / 2))
                                    .map(|kj| get_at_coord(ki, kj))
                                    .collect::<Vec<I>>()
                            })
                            .collect::<Vec<Vec<I>>>(),
                    )
                })
                .collect::<Vec<O>>()
        })
        .collect::<Vec<Vec<O>>>();
}
