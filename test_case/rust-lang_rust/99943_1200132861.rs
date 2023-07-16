plain
---- [run-pass-valgrind] src/test/run-pass-valgrind/unsized-locals/by-value-trait-objects-rust-call.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-valgrind/unsized-locals/by-value-trait-objects-rust-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/unsized-locals/by-value-trait-objects-rust-call/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/unsized-locals/by-value-trait-objects-rust-call/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
  |
1 | #![feature(unsized_locals)]
  |            ^^^^^^^^^^^^^^
  |
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information

error[E0277]: functions with the "rust-call" ABI must take a single non-self tuple argument
  |
  |
6 |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
  |                           ^^^^^^^^^ `Args` is not a tuple
help: consider restricting type parameter `Args`
  |
  |
4 | pub trait FnOnce<Args: std::marker::Tuple> {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [run-pass-valgrind] src/test/run-pass-valgrind/unsized-locals/by-value-trait-objects-rust-call2.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-valgrind/unsized-locals/by-value-trait-objects-rust-call2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/unsized-locals/by-value-trait-objects-rust-call2/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/unsized-locals/by-value-trait-objects-rust-call2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
  |
1 | #![feature(unsized_locals)]
  |            ^^^^^^^^^^^^^^
  |
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information

error[E0277]: functions with the "rust-call" ABI must take a single non-self tuple argument
  |
  |
6 |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
  |                           ^^^^^^^^^ `Args` is not a tuple
help: consider restricting type parameter `Args`
  |
  |
4 | pub trait FnOnce<Args: std::marker::Tuple> {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
