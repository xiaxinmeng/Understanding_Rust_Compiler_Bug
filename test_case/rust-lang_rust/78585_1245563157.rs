rust
struct Foo {
    value: usize
}

fn foo(a: Option<&mut Foo>, b: usize) {
    match a {
        Some(a) if a.value == b {
            a.value = 1;
        },
        _ => {}
    }
}
