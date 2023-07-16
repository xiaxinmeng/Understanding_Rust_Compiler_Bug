rust
error[[E0308]](https://doc.rust-lang.org/stable/error_codes/E0308.html): mismatched types
  --> src/main.rs:27:5
   |
27 |     session.dispatch_request(test_interpreter).await.unwrap();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'b> FnOnce<(&'b mut String,)>`
              found trait `for<'a> FnOnce<(&'a mut String,)>`
note: the lifetime requirement is introduced here
  --> src/main.rs:9:45
   |
9  |         F: for<'b> FnMut(&'b mut String) -> Fut,
   |                                             ^^^
