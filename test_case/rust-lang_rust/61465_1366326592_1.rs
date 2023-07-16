
error[E0391]: cycle detected when const-evaluating + checking `foo::NUM`
  --> src/main.rs:11:24
   |
11 |     const NUM: usize = NUM as usize;
   |                        ^^^
   |
   = note: ...which immediately requires const-evaluating + checking `foo::NUM` again
note: cycle used when simplifying constant for the type system `foo::NUM`
  --> src/main.rs:11:5
   |
11 |     const NUM: usize = NUM as usize;
   |     ^^^^^^^^^^^^^^^^
