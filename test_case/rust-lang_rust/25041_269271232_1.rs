
error[E0207]: the type parameter `O2` is not constrained by the impl trait, self type, or predicates
  --> src/parser.rs:81:13
   |
81 | impl<I, O1, O2> Add for Parser<I, O1> {
   |             ^^ unconstrained type parameter
