
warning: trait objects without an explicit `dyn` are deprecated
 --> mutant.rs:1:25
  |
1 | fn test_ref(x: &u32) -> std::future::Future<Output = u32> + '_ {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn std::future::Future<Output = u32> + '_`
  |
  = note: `#[warn(bare_trait_objects)]` on by default
  = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
  = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>

error[E0601]: `main` function not found in crate `mutant`
 --> mutant.rs:1:1
  |
1 | / fn test_ref(x: &u32) -> std::future::Future<Output = u32> + '_ {}
2 | | fn u() {
3 | |     test_ref & u
4 | | }
  | |_^ consider adding a `main` function to `mutant.rs`

error[E0746]: return type cannot have an unboxed trait object
 --> mutant.rs:1:25
  |
1 | fn test_ref(x: &u32) -> std::future::Future<Output = u32> + '_ {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
  |
1 | fn test_ref(x: &u32) -> T {}
  |                         ^
help: use `impl std::future::Future<Output = u32> + '_` as the return type if all return paths have the same type but you want to expose only the trait in the signature
  |
1 | fn test_ref(x: &u32) -> impl std::future::Future<Output = u32> + '_ {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use a boxed trait object if all return paths implement trait `std::future::Future<Output = u32> + '_`
  |
1 | fn test_ref(x: &u32) -> Box<dyn std::future::Future<Output = u32> + '_> {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', compiler/rustc_middle/src/ty/sty.rs:968:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (9c25eb7aa 2021-07-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [typeck] type-checking `u`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0601, E0746.
For more information about an error, try `rustc --explain E0601`.
