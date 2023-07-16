
error[E0207]: the type parameter `V` is not constrained by the impl trait, self type, or predicates
 --> src/lib.rs:9:6
  |
9 | impl<V: Vector> BaseFieldPrinter<V::MyField> {
  |      ^ unconstrained type parameter
