
error[E0282]: type annotations needed for `&mut (_, _)`
 --> file8.rs:7:37
  |
7 |         self.v.get_mut(i as _).map(|&mut (_, ref mut v2)| {
  |                                     ^^^^^^^^^^^^^^^^^^^^ consider giving this closure parameter the explicit type `&mut (_, _)`, with the type parameters specified
  |
  = note: type must be known at this point
