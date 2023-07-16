rust
macro_rules! foo {
    ($x:ident) => {{ let $x = 42; }}
}

fn main() {
    foo!(y);
}
