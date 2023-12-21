use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let is_pm = s.contains("PM");
    let am_pm_token = if is_pm {"PM"} else {"AM"};

    let clean_time_str = s.replace(am_pm_token, "");

    let mut tokens: Vec<i32> = clean_time_str.split(":").map(|x| x.parse::<i32>().unwrap()).collect();
    if is_pm {
        if tokens[0] != 12 {
            tokens[0] = (tokens[0] + 12) % 24;
        }        
    }
    else {
        tokens[0] = tokens[0] % 12;
    }

    println!("{:02}:{:02}:{:02}", tokens[0], tokens[1], tokens[2]);

    format!("{:02}:{:02}:{:02}", tokens[0], tokens[1], tokens[2])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    //let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    //writeln!(&mut fptr, "{}", result).ok();
}
