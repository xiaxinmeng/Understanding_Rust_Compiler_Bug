 rust
mod A {
    struct A {
        d: (),
    }
}

fn main() {
    let _: A::A = A {
        d: (),
    };
}
