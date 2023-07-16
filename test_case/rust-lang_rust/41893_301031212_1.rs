
error[E0080]: constant evaluation error
 --> <anon>:7:22
  |
7 |     const I: usize = 0 - 1;
  |                      ^^^^^ attempt to subtract with overflow
  |
note: for pattern here
 --> <anon>:8:12
  |
8 |     if let I = 42 {}
  |   
