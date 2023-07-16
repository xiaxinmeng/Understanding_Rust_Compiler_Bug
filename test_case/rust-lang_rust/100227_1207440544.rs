rust
error[E0599]: no function or associated item named `contains` found for struct `Vec<_, _>` in the current scope
 --> src/lib.rs:4:10
  |
4 |     Vec::contains(&vec, &0); // doesn't work
  |          ^^^^^^^^ function or associated item not found in `Vec<_, _>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `playground` due to previous error
