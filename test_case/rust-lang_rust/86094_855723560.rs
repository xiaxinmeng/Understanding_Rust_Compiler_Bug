
error[E0308]: mismatched types
 --> src/main.rs:5:9
  |
4 | /     if x {
5 | |         Err(MyError)
  | |         ^^^^^^^^^^^^ expected `()`, found enum `Result`
6 | |     }
  | |_____- expected this to be `()`
  |
  = note: expected unit type `()`
                  found enum `Result<_, MyError>`
help: you might have meant to return this value
  |
5 |         return Err(MyError);
  |         ^^^^^^             ^
