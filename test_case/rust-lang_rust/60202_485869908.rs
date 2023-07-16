rust
macro_rules! foo {
    ($($bar_lit:tt,)*) => {
        macro_rules! bar {
            $(
              ($bar_lit) => { println!("ok") };
            )*
        }
    }
}

fn main() {
    foo!("baz", "bazz",);
    bar!("baz");
    bar!("bazz");
}
