
error[[E0106]](https://doc.rust-lang.org/stable/error-index.html#E0106): missing lifetime specifier
 [--> src/lib.rs:3:18
](https://play.rust-lang.org/#)  |
3 |     const CONST: &str = "Hello, world!";
  |                  ^ expected named lifetime parameter
  |
help: consider using the `'static` lifetime
  |
3 |     const CONST: &'static str = "Hello, world!";
  |                   +++++++
help: consider introducing a named lifetime parameter
  |
2 ~ impl<'a> Test {
3 ~     const CONST: &'a str = "Hello, world!";
  |
