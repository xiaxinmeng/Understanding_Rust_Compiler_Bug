 rust
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Copy, Clone)]
enum State {
    Outside,
    BeginSlash,
    Inside,
    EndAsterisk,
}

fn count_non_comment_bytes(fname: &str) -> std::io::Result<u32> {
    let mut input = BufReader::new(File::open(fname)?);
    let mut state = State::Outside;
    let mut buffer = [0u8];
    let mut non_comment_count = 0;
    while input.read(&mut buffer)? > 0 {
        match (state, buffer[0]) {
            (State::Outside, b'/') => state = State::BeginSlash,
            (State::Outside, _) => non_comment_count += 1,
            (State::BeginSlash, b'*') => state = State::Inside,
            (State::BeginSlash, _) => {
                state = State::Outside;
                non_comment_count += 2;
            }
            (State::Inside, b'*') => state = State::EndAsterisk,
            (State::Inside, _) => {},
            (State::EndAsterisk, b'/') => state = State::Outside,
            (State::EndAsterisk, _) => state = State::Inside,
        }
    }
    Ok(non_comment_count)
}

fn main() {
    for fname in std::env::args().skip(1) {
        match count_non_comment_bytes(&fname) {
            Ok(n) => println!("{}: {} non-comment bytes", fname, n),
            Err(err) => println!("error while processing {}: {:?}", fname, err),
        }
    }
}
