 rust
fn main() {
    for _ in range(0, large_number) {...}
}
// Gets desugared to something like this:
// (also similar to what many iterator adapters do internally)
fn main() {
    match range(0, large_number) {
        ref mut iter => loop {
            match iter.next() {
                Some(_) => {...}
                #[cold] // now we can mark this arm "cold".
                None => break
            }
        }
    }
}
