
error[E0207]: the const parameter `N` is not constrained by the impl trait, self type, or predicates
 --> src/main.rs:5:13
  |
5 | impl <const N: usize> Collatz<{Some(N)}> {
  |             ^ unconstrained const parameter
note: const parameters can be constrained by using them in a trivial expression
