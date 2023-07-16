
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
 --> src/main.rs:2:12
  |
2 |     x.push(y);
  |            ^
  |
note: ...the reference is valid for the lifetime 'a as defined on the function body at 1:1...
 --> src/main.rs:1:1
  |
1 | / fn foo<'a, 'b>(x: &mut Vec<&'a u8>, y: &'b u8) {
2 | |     x.push(y);
3 | | }
  | |_^
note: ...but the borrowed content is only valid for the lifetime 'b as defined on the function body at 1:1
 --> src/main.rs:1:1
  |
1 | / fn foo<'a, 'b>(x: &mut Vec<&'a u8>, y: &'b u8) {
2 | |     x.push(y);
3 | | }
  | |_^
