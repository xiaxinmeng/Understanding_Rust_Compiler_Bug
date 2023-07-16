text
error[E0423]: expected function, tuple struct or tuple variant, found struct `S`
 --> <anon>:6:13
  |
2 |     struct S { x: u32, y: u32 }
  |     --------------------------- `S` defined here
...
6 |     let x = S(a, b);
  |             ^^^^^^^
  |
help: use struct literal syntax instead
  |
6 |     let x = S { x: val, y: val };
  |             ~~~~~~~~~~~~~~~~~~~~
help: a local variable with a similar name exists
  |
6 |     let x = a(a, b);
  |             ~
