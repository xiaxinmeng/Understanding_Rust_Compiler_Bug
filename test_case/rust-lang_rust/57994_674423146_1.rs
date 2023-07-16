
error[E0599]: no method named `x` found for struct `S` in the current scope
  --> src/main.rs:10:7
   |
2  | struct S;
   | --------- method `x` not found for this
...
10 |     S.x();
   |       ^ method not found in `S`

error: aborting due to previous error
