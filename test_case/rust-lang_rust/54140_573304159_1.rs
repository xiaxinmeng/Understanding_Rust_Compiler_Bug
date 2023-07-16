rust
macro_rules! lint {
    ($namespace:expr, $lint:literal) => {
        println!("{}", concat!($namespace, "::", $lint));
    };
    ($lint:literal) => {
        println!("{}", concat!(env!("CARGO_PKG_NAME"), "::", $lint));
    }
}

fn main() {
    lint!("builder", "some_warning");
    lint!("some_warning");
}
