 rust
fn assert_send<T: Send>(_t: T) {}

fn main() {
    let line = String::new();
    match [line.as_slice()] {
        [ word ] => { assert_send(word); }
    }
}

