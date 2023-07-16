 rust
#[derive(Debug)]
struct Foo {
    x: i32,
}

struct Bar {
    y: i32,
}

fn main() {
    let bar = Bar{ y: 4 };

    let foo: Foo = unsafe { std::mem::transmute(bar) };
    format!("{:?}", foo);
}
