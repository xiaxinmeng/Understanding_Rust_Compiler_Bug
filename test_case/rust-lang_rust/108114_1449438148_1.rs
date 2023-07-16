
error[[E0308]](https://doc.rust-lang.org/stable/error_codes/E0308.html): mismatched types
  --> src/main.rs:27:5
   |
27 |     session.dispatch_request(test_interpreter).await.unwrap();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a> <for<'a> fn(&'a mut String) -> impl Future<Output = Result<String, anyhow::Error>> {test_interpreter} as FnOnce<(&'a mut String,)>>`
              found trait `for<'a> <for<'a> fn(&'a mut String) -> impl Future<Output = Result<String, anyhow::Error>> {test_interpreter} as FnOnce<(&'a mut String,)>>`
note: the lifetime requirement is introduced here
  --> src/main.rs:9:45
   |
9  |         F: for<'a> FnMut(&'a mut String) -> Fut,
   |                                             ^^^
