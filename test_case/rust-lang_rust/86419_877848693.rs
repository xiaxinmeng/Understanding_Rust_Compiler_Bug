
8    = note: see issue #58713 <https://github.com/rust-lang/rust/issues/58713> for more information
9 
9 
10 error: multiple definitions of external function `f` from library `foo.dll` have different calling conventions
-   --> $DIR/multiple-definitions.rs:8:5
12    |
12    |
- LL |     fn f(x: i32);
-    |     ^^^^^^^^^^^^^
+ LL |         fn f(x: i32);
15 
16 error: aborting due to previous error; 1 warning emitted
17
