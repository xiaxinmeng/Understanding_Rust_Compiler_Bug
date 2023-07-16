
error[E0521]: borrowed data escapes outside of closure
 --> src/main.rs:3:28
  |
2 |     let mut fields: Vec<&str> = Vec::new();
  |         ---------- `fields` declared here, outside of the closure body
3 |     let pusher = |a: &str| fields.push(a);
  |                   -        ^^^^^^^^^^^^^^ `a` escapes the closure body here
  |                   |
  |                   `a` is a reference that is only valid in the closure body
