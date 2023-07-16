rust
fn greet<'a>(name: &'a str) {
    const HELLO: &'a str = "Hello";

    println!("{} {}", HELLO, name);
}
