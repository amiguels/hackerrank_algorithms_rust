use std::{io::{self, BufRead}, ops::Div};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {

    // Option one: Clean but slower
    // let n_positives = arr.iter().filter(|x| x.is_positive()).count() as f32;
    // let n_negatives = arr.iter().filter(|x| x.is_negative()).count()  as f32;
    // let n_zeros = (arr.len() as f32) - n_positives - n_negatives;

    // println!("{:.6?}", n_positives/(arr.len() as f32));
    // println!("{:.6?}", n_negatives/(arr.len() as f32) );
    // println!("{:.6?}", n_zeros/(arr.len() as f32) );

    // Option two: Less elegant, but faster
    // let mut results = [0f32, 0f32, 0f32];

    // arr.iter()
    //     .map(|x| (x.signum() + 1) as usize )
    //     .for_each(|x| results[x] += 1f32/(arr.len() as f32));

    // static NEGATIVE_IDX: usize = 0;
    // static ZEROS_IDX: usize = 1;
    // static POSITIVE_IDX: usize = 2;
    
    // println!("{:.6?}", results[POSITIVE_IDX]);
    // println!("{:.6?}", results[NEGATIVE_IDX]);
    // println!("{:.6?}", results[ZEROS_IDX]);

    // Option 3: Fastest
    let mut n_positive = 0;
    let mut n_negative = 0;
    let mut n_zero = 0;

    for x in arr {
        if x.is_positive() {
            n_positive += 1;
        }
        else if x.is_negative() {
            n_negative += 1;
        }
        else {
            n_zero += 1;
        }
    }

    println!("{:.6?}", (n_positive as f32) / (arr.len() as f32));
    println!("{:.6?}", (n_negative as f32) / (arr.len() as f32));
    println!("{:.6?}", (n_zero as f32) / (arr.len() as f32));

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
