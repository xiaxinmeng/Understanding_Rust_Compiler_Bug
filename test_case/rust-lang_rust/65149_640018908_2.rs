
error[E0277]: the trait bound `B: Bounded` is not satisfied
 --> src/lib.rs:5:40
  |
2 |     const COUNT : usize;
  |     -------------------- required by `Bounded::COUNT`
...
5 | fn make_array<B : Bounded>() -> [bool; <B as Bounded>::COUNT] {
  |                                        ^^^^^^^^^^^^^^^^^^^^^ the trait `Bounded` is not implemented for `B`
  |
help: consider further restricting this bound
  |
5 | fn make_array<B : Bounded + Bounded>() -> [bool; <B as Bounded>::COUNT] {
  |                           ^^^^^^^^^
