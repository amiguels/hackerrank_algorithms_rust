use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    
    let dim = arr.len();
    
    println!("{:?}", arr);
    println!("{}", dim);

    let mut diag1_sum = 0;
    let mut diag1_idx = 0;
    
    let mut diag2_sum = 0;
    let mut diag2_idx = dim - 1usize;

    for line in arr  {
        diag1_sum += line[diag1_idx];
        diag2_sum += line[diag2_idx];

        diag1_idx += 1;
        diag2_idx = diag2_idx.overflowing_sub(1).0;
    }
    
    (diag1_sum-diag2_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    //let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    //writeln!(&mut fptr, "{}", result).ok();
    println!("{:?}", result);
}
