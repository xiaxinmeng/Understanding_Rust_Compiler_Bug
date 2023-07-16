 rust
fn main() {
    macro_rules! foo {
        (...) => { ... }
    }    

    if true { foo!(1, 2) } else { foo!(3, 4); bar() }
}
