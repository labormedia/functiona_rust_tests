use functional_tests::apply;

fn hourglass_sum<'a>(arr: &'a Vec<Vec<i32>>) -> i32 {
    let arr_size = arr.len();
    let hg_last_index = 2;
    let hg = |cursor:(usize, usize), data: &'a [Vec<i32>]| data[0+cursor.0..=hg_last_index+cursor.0]
        .iter()
        .enumerate()
        .map( move |(i,x)| 
            x[cursor.1..=hg_last_index+cursor.1]
                .iter()
                .enumerate()
                .filter( |(j,_y)| {
                    !( i == 1_usize && (j == &(0_usize) || j == &(2_usize)) )
                })
                .map( |(_j,y)| *y)
                .collect::<Vec<i32>>()
        );
    let full_subset = (0..arr_size-hg_last_index)
        .map( |x| {
            (0..arr_size-hg_last_index).map( |y| hg((x,y), arr)
            .collect::<Vec<Vec<i32>>>() ).collect::<Vec<Vec<Vec<i32>>>>()
        } )
        .collect::<Vec<Vec<Vec<Vec<i32>>>>>();
    let full_subset_flatten = (0..arr_size-hg_last_index)
    .map( |x| {
        (0..arr_size-hg_last_index).map( |y| 
            hg((x,y), arr)
                .flatten()
                .collect::<Vec<i32>>() ).collect::<Vec<Vec<i32>>>()
    } );
    let full_subset_flatten_sum= (0..arr_size-hg_last_index)
        .map( |x| {
            (0..arr_size-hg_last_index)
                .map( |y| 
                    hg((x,y), arr)
                        .flatten()
                        .sum::<i32>() ).collect::<Vec<i32>>()
                } )
        .collect::<Vec<Vec<i32>>>();
    let full_subset_flatten_sum_flatten= (0..arr_size-hg_last_index)
        .map( |x| {
            (0..arr_size-hg_last_index)
                .map( |y| 
                    hg((x,y), arr)
                        .flatten()
                        .sum::<i32>()
                )
                // .flatten()
                .collect::<Vec<i32>>()
        } )
        .flatten()
        .collect::<Vec<i32>>();
    let max_hg = (0..arr_size-hg_last_index)
        .map( |x| {
            (0..arr_size-hg_last_index)
                .map( |y| 
                    hg((x,y), arr)
                        .flatten()
                        .sum::<i32>()
                )
                // .flatten()
                .collect::<Vec<i32>>()
        } )
        .flatten()
        .max();
    println!("full_subset : {:?}", full_subset);
    println!("full_subset_flatten : {:?}", full_subset_flatten);
    println!("full_subset_flatten_sum : {:?}", full_subset_flatten_sum);
    println!("full_subset_flatten_sum_flatten : {:?}", full_subset_flatten_sum_flatten);
    println!("max_hg : {:?}", max_hg);
    max_hg.unwrap()
}

fn parser<T>(data: &String) -> Vec<Vec<i32>> {
        let a = data
        .split('\n')
        .map( |x| {
            x  
                .split(' ')
                .map( |y|
                    y.parse::<i32>().unwrap()
                )
                
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<i32>>>();
        println!("vectors: {:?}", a);
        a
}

fn main() {
    let _a = apply(&"examples/data/iter_nesting_A.txt", hourglass_sum, parser::<Box<String>>);
}