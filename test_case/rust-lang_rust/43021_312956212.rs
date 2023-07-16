
error[E0391]: unsupported cyclic reference between types/traits detected
 --> impl-trait-lifetime-inference.rs:8:32
  |
8 | fn example(input: &Example) -> impl Iterator<Item = &Example> {
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cyclic reference
  |
note: the cycle begins when processing `example::{{impl-Trait}}`...
 --> impl-trait-lifetime-inference.rs:8:32
  |
8 | fn example(input: &Example) -> impl Iterator<Item = &Example> {
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...which then again requires processing `example::{{impl-Trait}}`, completing the cycle.
