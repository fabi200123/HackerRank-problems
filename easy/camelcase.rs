use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'camelcase' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn camelcase(s: &str) -> i32 {
    let mut sum = 1; // We have at least 1 word
    for i in s.chars() {
        if i.is_ascii_uppercase() {
            sum += 1;
        }
    }
    return sum;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = camelcase(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
