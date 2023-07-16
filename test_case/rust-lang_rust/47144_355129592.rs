
error: borrowed data cannot be moved outside of its closure
 --> file.rs:7:27
  |
6 |     let mut x = None;
  |         ----- borrowed data cannot be moved to here...
7 |     give_any(|y| x = Some(y));
  |              ---          ^ cannot be moved outside of its closure
  |              |
  |              ...because it cannot outlive this closure
