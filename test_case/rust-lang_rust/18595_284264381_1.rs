
error[E0308]: mismatched types
 --> <anon>:4:16
  |
4 |         foo(|| { true; });
  |                ^^^^^^^^^ expected bool, found ()
  |
  = note: expected type `bool`
             found type `()`
help: consider removing this semicolon:
 --> <anon>:4:22
  |
4 |         foo(|| { true; });
  |                      ^

error: aborting due to previous error
