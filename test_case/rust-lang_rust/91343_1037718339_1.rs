
error[E0529]
 --> src/lib.rs:3:14
  |
2 |     match v {
  |           - help: consider slicing here: `v[..]`
3 |         Some([]) => {}
  |              ^^ pattern cannot match with input type `Vec<i32>`
