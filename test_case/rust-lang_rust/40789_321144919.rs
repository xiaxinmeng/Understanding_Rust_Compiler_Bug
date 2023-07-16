
error[E0119]: conflicting implementations of trait `hyper::client::Connect` for type `TestConnect`:
 --> src/main.rs:8:1
  |
8 | impl client::Connect for TestConnect {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: conflicting implementation in crate `hyper`
