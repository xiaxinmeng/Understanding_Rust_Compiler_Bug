
error[E0391]: cycle detected when simplifying constant for the type system `foo::NUM`
  --> src/main.rs:11:5
   |
11 |     const NUM: usize = NUM as usize;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: ...which requires simplifying constant for the type system `foo::NUM`...
  --> src/main.rs:11:5
   |
11 |     const NUM: usize = NUM as usize;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `foo::NUM`...
  --> src/main.rs:11:5
   |
11 |     const NUM: usize = NUM as usize;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `foo::NUM`...
   = note: ...which again requires simplifying constant for the type system `foo::NUM`, completing the cycle
   = note: cycle used when running analysis passes on this crate

For more information about this error, try `rustc --explain E0391`.
error: could not compile `tmp` due to previous error
