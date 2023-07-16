 rust
#[deriving(Clone)]
enum Test<'a> {
    Slice(&'a int)
}

fn main() {}
