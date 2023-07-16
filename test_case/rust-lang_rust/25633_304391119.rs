
error[E0282]: type annotations needed
 --> <anon>:2:13
  |
2 |     let a = Vec::new();
  |         -   ^^^^^^^^ cannot infer complete type: `Vec<_>`
  |         |
  |         consider giving `a` a type

error[E0282]: type annotations needed
 --> <anon>:2:13
  |
2 |     let a = HashMap::new();
  |         -   ^^^^^^^^^^^^ cannot infer complete type: `HashMap<.., _>`
  |         |
  |         consider giving `a` a type
  | note: could not infer `_` // <-- this is probably wrong looking but something like it

error: aborting due to previous error
