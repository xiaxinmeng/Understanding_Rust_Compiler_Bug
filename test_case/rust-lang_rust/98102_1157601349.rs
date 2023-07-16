rust
fn main() {
    let mut s = 123;
    let v = Foo { s: &mut s };
    let v = v; // Works if I comment this line
}
