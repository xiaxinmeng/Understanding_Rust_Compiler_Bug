rust
macro_rules! a {
    () => {
        struct A;
    }
}
mod a {
    pub struct A;
}

mod b {
    use crate::a::*;

    a!();

    fn f(a: A) {}

    mod c {
        use crate::b::*;

        fn x() {
            f(A)
        }
    }
}

fn main() { }
