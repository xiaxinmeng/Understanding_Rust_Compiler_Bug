rust
fn main() -> io::InputError {
    let file = io::read_line()?;
    let _ = read_to_string(file)?;  // this ? may need modification since it is io::InputError
}
