use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
extern crate num;

use num::BigInt;
use num::FromPrimitive;
/*
 * Complete the 'fibonacciModified' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER t1
 *  2. INTEGER t2
 *  3. INTEGER n
 */

fn fibonacciModified(t1: i32, t2: i32, n: i32) -> BigInt {
    let mut prev1 = BigInt::from_i32(t1).unwrap();
    let mut prev2 = BigInt::from_i32(t2).unwrap();

    for _ in 3..=n {
        let current = &prev1 + &prev2 * &prev2;
        prev1 = prev2;
        prev2 = current;
    }

    return prev2;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let t1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let t2 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let n = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let result = fibonacciModified(t1, t2, n);

    writeln!(&mut fptr, "{}", result).ok();
}
