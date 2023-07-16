rust
macro make_struct() {
    struct S;
}

trait Tr {
    make_struct!(); // ERROR expected one of `const`, `extern`, `fn`, `type`, or `unsafe`, found `struct`
}

fn main() {}
