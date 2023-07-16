rust
#[derive(Debug)]
enum Foo {
    Done,
    Nested(Option<&'static Foo>),
}

fn walk(mut value: &Foo) {
    loop {
        println!("{:?}", value);
        &Foo::Nested(Some(value)) = value else { break };
    }
}
