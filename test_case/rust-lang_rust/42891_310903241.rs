
rustc 1.19.0-beta.2 (a175ee509 2017-06-15)
error[E0308]: mismatched types
 --> <anon>:1:28
  |
1 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
2 | |     x + 1;
3 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
help: consider removing this semicolon:
 --> <anon>:2:10
  |
2 |     x + 1;
  |          ^

error: aborting due to previous error(s)
