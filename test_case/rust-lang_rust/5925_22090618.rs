 rust
use a::Reader;
use b::Reader;

mod a {
    pub trait Reader {
        fn foo(int);
    }
}

mod b {
    pub trait Reader {
        fn foo(int, int);
    }
}

struct A;

impl Reader for A {
    fn foo(_: int) {}
}

fn main() {
}
