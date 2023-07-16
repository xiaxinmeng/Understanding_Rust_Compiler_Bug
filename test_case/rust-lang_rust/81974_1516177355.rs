
error[[E0059]](https://doc.rust-lang.org/nightly/error_codes/E0059.html): type parameter to bare `FnOnce` trait must be a tuple
  --> src/main.rs:21:12
   |
21 | impl<A, B> FnOnce<A> for CachedFun<A, B> where
   |            ^^^^^^^^^ the trait `Tuple` is not implemented for `A`
   |
note: required by a bound in `FnOnce`
  --> /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/core/src/ops/function.rs:242:1
help: consider further restricting this bound
   |
22 |     A: Eq + Hash + Clone + std::marker::Tuple,
   |                          ++++++++++++++++++++

error[[E0059]](https://doc.rust-lang.org/nightly/error_codes/E0059.html): type parameter to bare `FnMut` trait must be a tuple
  --> src/main.rs:32:12
   |
32 | impl<A, B> FnMut<A> for CachedFun<A, B> where
   |            ^^^^^^^^ the trait `Tuple` is not implemented for `A`
   |
note: required by a bound in `FnMut`
  --> /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/core/src/ops/function.rs:163:1
help: consider further restricting this bound
   |
33 |     A: Eq + Hash + Clone + std::marker::Tuple,
   |                          ++++++++++++++++++++

error[[E0277]](https://doc.rust-lang.org/nightly/error_codes/E0277.html): functions with the "rust-call" ABI must take a single non-self tuple argument
  --> src/main.rs:26:5
   |
26 |     extern "rust-call" fn call_once(mut self, a: A) -> Self::Output {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Tuple` is not implemented for `A`
   |
help: consider further restricting this bound
   |
22 |     A: Eq + Hash + Clone + std::marker::Tuple,
   |                          ++++++++++++++++++++

error[[E0277]](https://doc.rust-lang.org/nightly/error_codes/E0277.html): functions with the "rust-call" ABI must take a single non-self tuple argument
  --> src/main.rs:36:5
   |
36 |     extern "rust-call" fn call_mut(&mut self, a: A) -> Self::Output {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Tuple` is not implemented for `A`
   |
help: consider further restricting this bound
   |
33 |     A: Eq + Hash + Clone + std::marker::Tuple,
   |                          ++++++++++++++++++++

Some errors have detailed explanations: E0059, E0277.
For more information about an error, try `rustc --explain E0059`.
