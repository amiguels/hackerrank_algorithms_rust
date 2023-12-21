use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    let mut line: Vec<char> = vec![' '; n as usize];

    for i in (0..(n as usize)).rev()  {
        line[i] = '#';

        for c in line.iter() {
            print!("{}", c.to_string());
        }

        println!("");        
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
