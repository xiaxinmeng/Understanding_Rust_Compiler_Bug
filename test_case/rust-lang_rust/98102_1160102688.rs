rust
fn main() {
    let s = 123;
    let union = Bar { v: Foo { s: &s } };
}
