
error[E0269]: not all control paths return a value
  --> <anon>:5:1
   |
5  | fn foo() -> i32 {
   | ^
   |
help: consider removing this semicolon:
  --> <anon>:46:6
   |
46 |     }; // <- here is the culprit!
   |      ^

error: aborting due to previous error
