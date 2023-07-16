 rust
macro_rules! my_fmt {
    ($s:expr, $($e:expr),*) => {
        fmt!($s, $($e),*) // error: not enough arguments to fmt! ...
    }
}

fn main() {
    let a = ~"";
    my_fmt!("foo: %s %s", a); // note: macro invoked here
}
