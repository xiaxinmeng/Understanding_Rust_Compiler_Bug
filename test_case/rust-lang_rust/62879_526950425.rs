
error[E0391]: cycle detected when processing `foo::A`
 --> src/main.rs:3:30
  |
3 | fn foo<const N: usize, const A: [u8; N]>() {}
  |                              ^
  |
  = note: ...which again requires processing `foo::A`, completing the cycle
note: cycle used when processing `foo`
 --> src/main.rs:3:1
  |
3 | fn foo<const N: usize, const A: [u8; N]>() {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
