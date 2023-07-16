
error[E0308]: mismatched types
  --> /Users/ekuber/personal/rust/src/test/compile-fail/issue-20862.rs:12:5
   |
12 |     |y| x + y
   |     ^^^^^^^^^
   |     |
   |     possibly `;` missing here?
   |     expected (), found closure
   |
   = note: expected type `()`
   = note:    found type `[closure@/Users/ekuber/personal/rust/src/test/compile-fail/issue-20862.rs:12:5: 12:14 x:_]`

error: expected function, found `()`
  --> /Users/ekuber/personal/rust/src/test/compile-fail/issue-20862.rs:17:13
   |
17 |     let x = foo(5)(2);
   |             ^^^^^^^^^

error: aborting due to 2 previous errors
