 rust
struct Foo {
    bytes: [u8],
}

fn bar(&Foo { ref bytes }: &Foo) {}
