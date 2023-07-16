rust
macro_rules! make_fn {
    () => {
        fn foo() {}
    }
}

fn main() {
    #[allow(dead_code)] make_fn!();
}
