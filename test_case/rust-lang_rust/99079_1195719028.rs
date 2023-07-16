rust
use std::fmt::Display;

fn foo<T: Display + Copy>(t: T, recurse: bool) -> impl Display {
    if recurse {
        (|| {
            let t: T = foo(t, false);
            println!("{t}");
        })();
    }
    t
}

fn main() {
    foo(1i32, true);
}
