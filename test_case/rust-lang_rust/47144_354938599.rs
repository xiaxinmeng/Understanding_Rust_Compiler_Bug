
error: borrowed data cannot be moved outside of its closure
 --> file.rs:7:27
  |
6 |     let mut x = None;
  |         ----- move destination (outside closure)
7 |     give_any(|y| x = Some(y));
  |              ---          ^ borrowed data which cannot escape this closure
  |              |
  |              closure
