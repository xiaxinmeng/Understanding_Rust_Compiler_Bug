 Rust
fn foo<'a>(){}
fn main() {
    foo::<'static>() //~ ERROR too many lifetime parameters provided
}
