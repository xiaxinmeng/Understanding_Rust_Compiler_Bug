 rust
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = match read_all_lines(stdin.lock()) {
        Err(err) => {
            println!("Failed to read input: {}", err);
            ::std::process::exit(1);
        }
        Ok(result) => result,
    };

    //  continue the program with lines: Vec<String>
}

fn read_all_lines<R: BufRead>(reader: R) -> Result<Vec<String>, io::Error> {
    // magic: .collect() can transform an iterator of Result<T, E> into a Result<Vec<T>, E>!
    reader.lines().collect()
}

fn read_all_lines_2<R: BufRead>(reader: R) -> Result<Vec<String>, io::Error> {
    let mut lines = Vec::new();
    for line_result in reader.lines() {
        // magic? return with an error if there is an error,
        // otherwise push the String to the vector.
        lines.push(line_result?);
    }
    Ok(lines)
}
