
   Compiling playground v0.0.1 (/playground)
error[E0599]: no method named `extend` found for struct `HashSet` in the current scope
 --> src/main.rs:8:9
  |
8 |     set.extend(std::iter::once(Data));
  |         ^^^^^^ method not found in `HashSet<Data>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
