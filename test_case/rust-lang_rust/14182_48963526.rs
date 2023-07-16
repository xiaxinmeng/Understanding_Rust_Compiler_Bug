 rust
struct Foo {
    f: <'b> |&'b int|: 'b -> &'b int
}

pub fn main() {
    let foo = Foo {
        f: |x| x
    };
}
