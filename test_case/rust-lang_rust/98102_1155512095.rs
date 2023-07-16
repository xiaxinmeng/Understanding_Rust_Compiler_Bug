rust
fn main() {
    let s = 123;
    let v = Foo { s: &s };
    Bar { v };
}
