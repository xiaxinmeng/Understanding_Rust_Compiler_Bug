
error[[E0499]](https://doc.rust-lang.org/stable/error_codes/E0499.html): cannot borrow `what` as mutable more than once at a time
 --> src/main.rs:7:14
  |
7 |         func(&mut what);
  |         ---- ^^^^^^^^^ `what` was mutably borrowed here in the previous iteration of the loop
  |         |
  |         first borrow used here, in later iteration of loop

For more information about this error, try `rustc --explain E0499`.
