 rust
struct Foo { x: int }

impl Foo {
    fn y(&self) {}
}
trait Z { fn z(&self); }
impl Z for Foo {
    fn z(&self) {}
}

fn main() {
    let a = Foo { x: 1 };
    a.y;
    a.z;
}
