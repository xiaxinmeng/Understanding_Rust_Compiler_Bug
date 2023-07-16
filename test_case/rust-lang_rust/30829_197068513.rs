 rust
fn test2<R>() {
    |u: &mut R| {
        (|| u)();
    };
}

fn main() {
}
