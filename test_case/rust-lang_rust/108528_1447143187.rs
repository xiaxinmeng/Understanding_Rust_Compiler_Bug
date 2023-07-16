
error[[E0308]](https://doc.rust-lang.org/nightly/error_codes/E0308.html): mismatched types
  --> src/lib.rs:22:9
   |
3  |     ErrorType(Details),
   |     --------- `ErrorType` defines an enum variant constructor here, which should be called
...
22 |         assert_eq!(err, MyError::ErrorType);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         expected `MyError`, found enum constructor
   |         expected because this is `MyError`
   |
   = note:          expected enum `MyError`
           found enum constructor `fn(Details) -> MyError {MyError::ErrorType}`
