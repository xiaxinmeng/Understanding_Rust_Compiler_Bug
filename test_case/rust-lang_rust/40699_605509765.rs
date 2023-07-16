text
error[E0621]: explicit lifetime required in the type of `p`
 --> src/main.rs:7:5
  |
4 | fn bar(p: &Foo) -> Box<Fn() -> ()> {
  |           ---- help: add explicit lifetime `'static` to the type of `p`: `&'static Foo`
...
7 |     Box::new(f)
  |     ^^^^^^^^^^^ lifetime `'static` required

error: aborting due to previous error
