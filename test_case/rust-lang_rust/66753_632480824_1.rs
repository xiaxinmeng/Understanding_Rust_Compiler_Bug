
error[E0493]: destructors cannot be evaluated at compile-time
 --> src/lib.rs:6:20
  |
6 | const fn unwrap<T>(opt:Option<T>)->T{
  |                    ^^^ constant functions cannot evaluate destructors
