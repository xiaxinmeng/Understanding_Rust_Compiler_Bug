
   Compiling playground v0.0.1 (/playground)
warning: unreachable statement
 --> src/lib.rs:9:5
  |
8 |     let iv: BigUint = todo!();
  |                       ------- any code following this expression is unreachable
9 |     let len: usize = todo!();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default

error[E0282]: type annotations needed
  --> src/lib.rs:12:23
   |
11 |     let iv = iv ^ (BigUint::from(1_u8) << (len * 8));
   |         -- consider giving `iv` a type
12 |     let _iv: &[u8] = &iv.to_bytes_be()[1..len + 1];
   |                       ^^ cannot infer type
   |
   = note: type must be known at this point

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0282`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
