
error: unconstrained generic constant
  --> <source>:15:26
   |
15 |             Some(x) => x.hmm(&res),
   |                          ^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::W]:`

error[E0308]: mismatched types
  --> <source>:15:30
   |
15 |             Some(x) => x.hmm(&res),
   |                              ^^^^ expected `Self::W`, found `Self::W`
   |
   = note: expected type `Self::W`
              found type `Self::W`
