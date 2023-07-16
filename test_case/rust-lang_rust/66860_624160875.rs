
error[E0283]: type annotations needed
 --> tests\fail.rs:4:13
  |
4 |     let _ = collect();
  |         -   ^^^^^^^
  |         |   |
  |         |   cannot infer type for type parameter `T` declared on the function `collect`
  |         |   help: consider specifying the type argument in the function call: `collect::<T>`
  |         consider giving this pattern a type
  |
  = note: cannot resolve `_: error_mismatch::Trait`

