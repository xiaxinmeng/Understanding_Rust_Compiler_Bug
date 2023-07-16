rust
// Uncomment the following to trigger a compile error.
//#[rustfmt::skip]
mod my_macro {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            "Hello world"
        };
    }
}

fn main() {
    println!(crate::my_macro!());
}
