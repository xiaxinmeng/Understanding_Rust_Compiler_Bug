
error[E0499]: cannot borrow `decoder` as mutable more than once at a time
  --> main.rs:15:29
   |
15 |     while let Some(frame) = decoder.read_next_frame().unwrap() {
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^ `decoder` was mutably borrowed here in the previous iteration of the loop
16 |         frames.push(frame);  // this line is the culprit
   |         ------------------ first borrow later used here

error: aborting due to previous error
