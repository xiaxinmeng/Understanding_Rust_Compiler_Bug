
error[E0618]: expected function, found enum variant `foo()`
 --> src/main.rs:6:5
  |
1 | / fn foo() -> Result<i32, i32> {
2 | |     Ok(42)
3 | | }
  | |_- `foo()` defined here
...
6 |       foo()();
  |       ^^^^^--
  |       |
  |       call expression requires function
help: `foo()` is a unit variant, you need to write it without the parenthesis
  |
6 |     foo();
  |     ^^^^^
