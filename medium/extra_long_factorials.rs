use std::io::{self, BufRead};
extern crate num;

use num::BigInt;
use num::FromPrimitive;
/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn extraLongFactorials(n: i32) {
    let mut result = BigInt::from(1);
    for i in 1..=n {
        result *= i;
    }
    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
