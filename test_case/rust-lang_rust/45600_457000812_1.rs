
error[E0009]: cannot bind by-move and by-ref in the same pattern
 --> src/lib.rs:3:29
  |
3 |         (&Some(ref x), Some(y)) => println!("{} {}", x, y), 
  |                -----        ^ by-move pattern here
  |                |
  |                both by-ref and by-move used
