
error[E0507]: cannot move out of borrowed content
 --> src/main.rs:4:13
  |
4 |     let x = ref_result.unwrap();
  |             ^^^^^^^^^^ -------- `unwrap` consumes `ref_result`
  |             |
  |             cannot move out of borrowed content
