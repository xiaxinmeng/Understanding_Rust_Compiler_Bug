rust
/// Reads a line of input from stdin
pub fn read_line() -> io::Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    // remove trailing newlines from input
    let mut n = 0usize;
    for c in s.as_str().chars().rev() {
        match c {
            '\n' | '\r' => n += 1,
            _ => break,
        };
    }
    input.truncate(input.len() - n);
    input
}
