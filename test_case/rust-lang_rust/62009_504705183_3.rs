
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:4:5
  |
3 | fn main() {
  |    ---- this is not `async`
4 |     async {}.await
  |     ^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks

error: aborting due to previous error
