
error[E0438]: const `TEST` is not a member of trait `Test`
  --> main_test.rs:10:5
   |
10 |     const TEST: fn() -> _ = 42; 
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a member of trait `Test`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> main_test.rs:10:25
   |
10 |     const TEST: fn() -> _ = 42; 
   |                         ^
   |                         |
   |                         not allowed in type signatures
   |                         help: use type parameters instead: `T`
