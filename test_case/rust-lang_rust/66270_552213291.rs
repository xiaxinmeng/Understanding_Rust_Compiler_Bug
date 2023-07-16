rust
struct Bug {
    foo: 0,
}

fn main() {
    let Bug { foo: 0 } = Bug {};
}
