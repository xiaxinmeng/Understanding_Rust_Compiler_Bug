 rust
#[feature(struct_variant)];

enum Foo {
    Bar { x: int }
}

fn main() {
    println!("{:?}", Bar { x: 5 });
}
