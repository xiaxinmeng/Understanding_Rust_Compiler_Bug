rust
// rustc test.rs --crate-type=lib -Zunstable-options --pretty=expanded | rustc -
macro_rules! a {
    () => {
        $crate::main()
    }
}
fn main() {
}

mod b {
    fn f() {
        a!();
    }
}
