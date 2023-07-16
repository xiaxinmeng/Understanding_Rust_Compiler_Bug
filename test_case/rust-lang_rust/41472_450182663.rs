rust
macro_rules! m {
    ($tt:tt) => {
        println!("it is a single token");
    };
}

macro_rules! n {
    ($meta:meta) => {
        m!($meta);
    };
}

fn main() {
    n!(recursion_limit = "128");
}
