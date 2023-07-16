
error[E0597]: borrowed value does not live long enough
 --> src/main.rs:4:27
  |
4 |     let f: &'static () = &loop {break};
  |                           ^^^^^^^^^^^^ temporary value does not live long enough
5 | }
  | - temporary value only lives until here
  |
  = note: borrowed value must be valid for the static lifetime...

error: aborting due to previous error
