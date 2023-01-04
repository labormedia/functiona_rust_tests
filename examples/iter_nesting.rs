fn hourglassSum(arr: &[Vec<i32>]) -> i32 {
    let arr_size = arr.len();
    let hg_last_index = 2;
    let hg = |cursor, data: &[Vec<i32>]| data[cursor..=cursor+hg_last_index]
        .iter()
        .enumerate()
        .map( |(i,x)| 
            x[0..=hg_last_index]
                .iter()
                .enumerate()
                .filter( |(j,_y)| {
                    !( i == 1_usize && (j == &0_usize || j == &2_usize) )
                })
                .map( |(_j,y)| *y)
                .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();
    let first_row_hg = (0..arr_size-hg_last_index)
        .map( |x| {
            hg(x, arr)
        } )
        .collect::<Vec<Vec<Vec<i32>>>>();
    println!("{:?}", first_row_hg);
    0
}