 rust
fn main() {
    let mut lines = io::stdin().lines();
    for line in lines {
        handle_line(line);
    }
    match lines.err {
        Ok(()) => (),
        Err(e) => handle_error(e)
    }
}
