
error[E0308]: `match` arms have incompatible types
 --> test.rs:4:14
  |
2 |       let x = match 1 {
  |  _____________-
3 | |         1 => other(),
  | |              ------- this is found to be of type `impl Future`
4 | |         2 => other().await,
  | |              ^^^^^^^^^^^^^ expected opaque type, found enum `Result`
5 | |     };
  | |_____- `match` arms have incompatible types
  |
  = note: expected type `impl Future`
             found enum `Result<(), ()>`
help: consider `await`ing on the `Future`
  |
4 |         1 => other().await,
  |                     ++++++
