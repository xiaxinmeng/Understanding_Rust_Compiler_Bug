rust
macro_rules! outer {
    ($dollar:tt 0) => {
        outer!($dollar 1)
    };
    ($dollar:tt $outer_lit:literal) => {
        macro_rules! inner {
            (1) => {
                println!("constant fragment")
            };
            ($dollar inner_lit:literal) => {
                println!("literal fragment")
            };
        }

        inner!($outer_lit);
        inner!(1);
    };
}

fn main() {
    outer!($ 0);
    inner!(1);
}
