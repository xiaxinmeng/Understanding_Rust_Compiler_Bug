
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   --> src/main.rs:6:37
    |
4   | / fn main() {
5   | |     let mut file = File::create("foo.txt")?;
6   | |     file.write_all(b"Hello, world!")?;
    | |                                     ^ cannot use the `?` operator in a function that returns `()`
7   | | }
    | |_- help: add a fallible return type: `fn main() -> Result<(), Box<dyn std::error::Error>`
