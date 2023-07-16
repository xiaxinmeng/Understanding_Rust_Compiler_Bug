rust
macro_rules! foo {
    ($x:expr) => {
        #[export_name = $x] pub fn foo() {}
    }
}

foo!(concat!("ma", "in"));

// Errors about `concat!` not being parseable.
// #[export_name = concat!("ma", "in")] pub fn foo() {}

fn main() {}
