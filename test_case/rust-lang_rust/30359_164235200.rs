 rust
fn main() {
    match Some(0) {
        Some(_) => 'a: loop { break },
        None    => 'a: loop { break }
    }
}
