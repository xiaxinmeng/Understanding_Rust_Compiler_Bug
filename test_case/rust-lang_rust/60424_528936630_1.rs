
error[E0515]: cannot return value referencing temporary value
 --> src/lib.rs:2:5
  |
2 |     &[s] as &[&str]
  |     ^---^^^^^^^^^^^
  |     ||
  |     |temporary value created here
  |     returns a value referencing data owned by the current function
