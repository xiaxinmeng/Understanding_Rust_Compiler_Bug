 rust
macro_rules! my_fmt {
    ($s:expr, $($e:expr),*) => {
        fmt!($s, $($e),*)
    }
}

fn main() {
    let a = ~"";
    my_fmt!("foo: %s %s", a);
}
