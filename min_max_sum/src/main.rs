use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut sorted_array = [0; 5];
    sorted_array.copy_from_slice(arr);
    sorted_array.sort();
    
    let sum_closure = |acc: i64, x: &i32| acc + *x as i64;

    let min: i64 = sorted_array[0..4].iter().fold(0i64, sum_closure);
    let max: i64 = sorted_array[1..5].iter().fold(0i64, sum_closure);

    println!("{} {}", min, max);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}

