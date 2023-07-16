rust
error[[E0596]](https://doc.rust-lang.org/stable/error-index.html#E0596): cannot borrow data in a `&` reference as mutable
 --> src/lib.rs:4:31
  |
4 |     str::make_ascii_lowercase(&mut x); // bad error message
  |                               ^^^^^^ cannot borrow as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `playground` due to previous error
