
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 |   fn foo(bar: usize) -> usize {
  |                         ----- expected `usize` because of return type
2 | /     for i in 0..1 {
3 | |         return 1;
4 | |     }
  | |_____^ expected usize, found ()
  |
  = note: expected type `usize`
             found type `()`
