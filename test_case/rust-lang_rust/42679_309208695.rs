rust
match (a, b) {
    (box Test::Foo(x), box Test::Bar(y)) => println!("Foo {}, Bar {}", x, y),
    _ => println!("Some other things"),
}
