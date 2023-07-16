
error[E0594]: cannot assign to `&`-reference *fancy_ref
  --> src/test/compile-fail/E0389.rs:18:5
   |
18 |     fancy_ref.num = 6; //~ ERROR E0389
   |     ^^^^^^^^^^^^^^^^^ cannot assign through `&`-reference

error: aborting due to previous error
