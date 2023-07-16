
error[E0178]: expected a path on the left-hand side of `+`, not `&Copy`
 --> <anon>:2:12
  |
2 |     let _: &Copy + 'static;
  |            ^^^^^ expected a path
  |
help: try adding parentheses (per RFC 438):
  |     let _: &(Copy + 'static);
