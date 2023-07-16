 rust
struct Foo { x: int, y: int, z: int }

impl Foo {
    fn default() -> Foo {
        Foo{x: 0, y: 0, z: 0}
    }
}

fn bar(f: Foo) {
    printf!(f);
}

fn main() {
    bar(Foo{y: 5, ..Foo::default()});
}
