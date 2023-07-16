rust
struct Foo {
    x: String,
    y: String,
}

fn init_foo() -> Box<Foo> {
    let mut b = /* create a shallow initialized box */;
    b.x = "a".to_owned();
    b.y = "b".to_owned();
    b
}
