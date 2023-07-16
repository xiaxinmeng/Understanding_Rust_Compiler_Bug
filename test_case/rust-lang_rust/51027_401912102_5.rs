
error: borrowed data cannot be stored outside of its closure
  --> issue-45983.rs:17:27
   |
16 |     let mut x = None;
   |         ----- borrowed data cannot be stored into here...
17 |     give_any(|y| x = Some(y));
   |              --- ^^^^^^^^^^^ cannot be stored outside of its closure
   |              |
   |              ...because it cannot outlive this closure
