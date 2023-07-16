
error[E0308]: mismatched types
 --> test:2:5
  |
1 |   fn foo() -> bool {
  |               ---- expected `bool` because of return type
2 | /     & 
3 | |     mut
4 | |     if true { true } else { false }
  | |___________________________________^ expected `bool`, found `&mut bool`
  |
help: consider removing the borrow
  |
2 -     & 
2 +     if true { true } else { false }
  |
