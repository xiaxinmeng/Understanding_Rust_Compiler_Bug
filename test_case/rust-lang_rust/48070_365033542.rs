rust
fn main() {
    let mut foo = Foo { x: 0 };
    match 22 {
        22 => &mut foo, // borrow B1
        _ => foo.twaddle(), // borrow B2
    }.emit();
}
