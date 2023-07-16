
error[E0308]: mismatched types
  --> <anon>:5:17
   |
5  | fn foo() -> i32 {
   |                 ^ expected i32, found ()
   |
   = note: expected type `i32`
              found type `()`
help: consider removing this semicolon:
  --> <anon>:46:6
   |
46 |     }; // <- here is the culprit!
   |      ^

error: aborting due to previous error
