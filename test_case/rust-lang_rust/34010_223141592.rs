 rust
macro_rules! mac {
    (#[derive($i:ident)] $it:item) => { println!(stringify!($i)); }
}
fn main() {
    mac! { #[derive(Foo)] struct Bar; } // prints "Foo"
}
