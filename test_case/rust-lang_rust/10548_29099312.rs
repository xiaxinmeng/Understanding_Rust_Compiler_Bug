 rust
mod a {
    struct S;
    impl S { }
}

fn foo(_: a::S) {
}

fn main() {}
