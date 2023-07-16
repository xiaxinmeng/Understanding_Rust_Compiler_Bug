rust
struct Bug {
    foo: 0,
}

fn main() {
    let bug = Bug { foo: 5 };
    bug.foo;
}
