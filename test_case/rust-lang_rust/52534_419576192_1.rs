
error[E0597]: `y` does not live long enough
 --> src/main.rs:3:6
  |
3 |     &y
  |      ^ borrowed value does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 1:1...
 --> src/main.rs:1:1
  |
1 | / fn foo(x: &u32) -> &u32 {
2 | |     let y = 22;
3 | |     &y
4 | | }
  | |_^
