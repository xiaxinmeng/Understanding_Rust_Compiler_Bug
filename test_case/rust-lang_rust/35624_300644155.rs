
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
  --> ./test.rs:19:5
   |
19 |     fn method() { }
   |     ^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime 'b as defined on the body at 19:16...
  --> ./test.rs:19:17
   |
19 |     fn method() { }
   |                 ^^^
note: ...so that types are compatible (expected Trait, found Trait)
  --> ./test.rs:19:5
   |
19 |     fn method() { }
   |     ^^^^^^^^^^^^^^^
note: but, the lifetime must be valid for the lifetime 'a as defined on the body at 19:16...
  --> ./test.rs:19:17
   |
19 |     fn method() { }
   |                 ^^^
note: ...so that types are compatible (expected Trait, found Trait)
  --> ./test.rs:19:5
   |
19 |     fn method() { }
   |     ^^^^^^^^^^^^^^^

error: aborting due to previous error
