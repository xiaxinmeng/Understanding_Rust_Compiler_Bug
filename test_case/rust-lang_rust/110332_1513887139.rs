rust
fn main() {
    f::<Struct>();
}

struct Struct;
impl Struct { type Item = usize; }

struct Foo { s: Struct::Item }
