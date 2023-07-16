 rust
struct Foo {
   a: int,
   b: int,
}

fn bar(c: uint) {
    match c {
        0 => io::println("0"),
        1 => io::println("1"),
        2 => io::println("2"),
        _ => io::println("many"),
    }
}
