
error[E0283]: type annotations needed
 --> tests\fail.rs:4:13
4 |     let _ = collect();
  |         -   ^^^^^^^ cannot infer type for type parameter `T` declared on the function `collect`
  |         |
  |         consider giving this pattern a type
 ::: C:\Users\mibac\IdeaProjects\test-error-mismatch\src\lib.rs:9:19
  |
9 | pub fn collect<T: Trait>() -> T {
  |                   ----- required by this bound in `error_mismatch::collect`
  = note: cannot satisfy `_: error_mismatch::Trait`
help: consider specifying the type argument in the function call
  |
4 |     let _ = collect::<T>();
  |                    ^^^^^
