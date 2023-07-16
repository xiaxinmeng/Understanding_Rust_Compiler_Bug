 rust
macro_rules! foo {
    (...) => {{
        ()
    }};
    ($e:expr, $($tok:tt)+) => {{
        ($e, foo!($($tok)+))
    }}
}

fn main() {
    let tpl = foo!(1,2,3,4,...);
    println!("{:?}", tpl);
}
