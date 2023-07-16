rust
match (a, b) {
    (a, box Test::Bar(y)) => println!("Some other {:?}, Bar {}", a, y),
    (box a, b) => println!("Some unboxed {:?}, some boxed {:?}", a, b),
}
