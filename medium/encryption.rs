use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'encryption' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn encryption(s: &str) -> String {
    let s = s.replace(" ", "");
    let len = s.len();
    
    // Calculate rows and columns
    let sqrt_len = (len as f64).sqrt();
    let rows = sqrt_len.floor() as usize;
    let cols = sqrt_len.ceil() as usize;

    let mut encrypted = Vec::new();

    for i in 0..cols {
        let mut word = String::new();
        for row in (i..len).step_by(cols) {
            word.push(s.chars().nth(row).unwrap());
        }
        encrypted.push(word);
    }

    return encrypted.join(" ")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = encryption(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
