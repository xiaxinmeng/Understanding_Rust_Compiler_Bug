
error: cannot constrain an associated constant to a value
 --> src/lib.rs:5:35
  |
5 | fn main<T, B: TraitWAssocConst<T, A = { 1 }>>() {}
  |                                   -^^^-----
  |                                   |   |
  |                                   |   ...cannot be constrained to this value
  |                                   this associated constant...

error[E0220]: associated type `A` not found for `TraitWAssocConst<T>`
 --> src/lib.rs:5:35
  |
5 | fn main<T, B: TraitWAssocConst<T, A = { 1 }>>() {}
  |                                   ^ associated type `A` not found
