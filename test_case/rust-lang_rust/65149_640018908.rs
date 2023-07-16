
error[E0599]: no associated item named `COUNT` found for type parameter `B` in the current scope
 --> src/lib.rs:5:43
  |
5 | fn make_array<B : Bounded>() -> [bool; B::COUNT] {
  |                                           ^^^^^ associated item not found in `B`
  |
  = help: items from traits can only be used if the type parameter is bounded by the trait
