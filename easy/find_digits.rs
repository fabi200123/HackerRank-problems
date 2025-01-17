use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'findDigits' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn findDigits(n: i32) -> i32 {
    let mut count = 0;
    let mut aux = n;
    while(aux != 0) {
        let mut digit = aux % 10;
        if digit != 0 && n % digit == 0 {
            count += 1;
        }
        aux = aux / 10;
    }
    return count;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = findDigits(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
