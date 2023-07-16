
error[[E0507]](https://doc.rust-lang.org/stable/error-index.html#E0507): cannot move out of `*ref_result` which is behind a shared reference
 --> src/main.rs:4:13
  |
4 |     let x = ref_result.unwrap();
  |             ^^^^^^^^^^^^^^^^^^^ move occurs because `*ref_result` has type `Result<String, String>`, which does not implement the `Copy` trait
  |
help: consider borrowing the `Result`'s content
  |
4 |     let x = ref_result.unwrap().as_ref();
  |                                +++++++++
  