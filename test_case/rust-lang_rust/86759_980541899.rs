
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
 --> foo.rs:2:27
  |
2 |     let a: &'static str = val;
  |                           ^^^
  |
  = note: ...the reference is valid for the static lifetime...
note: ...but the borrowed content is only valid for the anonymous lifetime defined here
 --> foo.rs:1:13
  |
1 | fn bar(val: &'_ str) {
  |             ^^^^^^^
