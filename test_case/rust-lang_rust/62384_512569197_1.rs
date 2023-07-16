
error[E0282]: type annotations needed
 --> src/test/ui/suggestions/missing_return_type.rs:3:25
  |
3 | fn main() -> Result<(), impl std::fmt::Debug>{
  |                         ^^^^^^^^^^^^^^^^^^^^ cannot infer type
