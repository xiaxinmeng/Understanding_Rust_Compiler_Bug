 rust
#[feature(macro_rules)];

macro_rules! macro {
    () => { println!("hello"); };
    ($y:tt $($x:tt)*) => {
        macro!($($x)*)
    }
}

fn main() {
    trace_macros!(true);
    macro!(1 2 3 4)
}
