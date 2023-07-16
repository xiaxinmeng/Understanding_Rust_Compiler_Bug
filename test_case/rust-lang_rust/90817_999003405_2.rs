
error[E0391]: cycle detected when computing whether impls specialize one another
  --> C:\path\to\bug\src\lib.rs:18:1
   |
18 | / impl<T, U> Combine<U> for T
19 | | where
20 | |     U: Split,
21 | |     Self: Change<U::First>,
22 | |     <Self as Change<U::First>>::Changed: Combine<U::Rest>,
23 | | {
24 | | }
   | |_^
   |
   = note: ...which requires evaluating trait selection obligation `(): bug::Combine<^1_0>`...
   = note: ...which again requires computing whether impls specialize one another, completing the cycle
note: cycle used when type-checking `main`
  --> src\main.rs:1:1
   |
1  | fn main() {
   | ^^^^^^^^^

For more information about this error, try `rustc --explain E0391`.
