plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0277]: the trait bound `u64: StableHasherResult` is not satisfied
   --> compiler/rustc_query_impl/src/on_disk_cache.rs:674:28
674 |                     hasher.finish()
    |                            ^^^^^^ the trait `StableHasherResult` is not implemented for `u64`
    |
    = help: the following other types implement trait `StableHasherResult`:
    = help: the following other types implement trait `StableHasherResult`:
              Fingerprint
              Hash128
              Hash64
note: required by a bound in `StableHasher::finish`
   --> /checkout/compiler/rustc_data_structures/src/stable_hasher.rs:42:22
    |
42  |     pub fn finish<W: StableHasherResult>(self) -> W {
    |                      ^^^^^^^^^^^^^^^^^^ required by this bound in `StableHasher::finish`
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:40:35
    |
36  | macro_rules! assert_eq {
36  | macro_rules! assert_eq {
    | ---------------------- in this expansion of `$crate::assert_eq!` (#2)
...
40  |                 if !(*left_val == *right_val) {
    |                      ---------    ^^^^^^^^^^ expected `Hash64`, found `u64`
    |                      expected because this is `Hash64`
...
246 | macro_rules! debug_assert_eq {
    | ---------------------------- in this expansion of `debug_assert_eq!` (#1)
    | ---------------------------- in this expansion of `debug_assert_eq!` (#1)
...
249 |             $crate::assert_eq!($($arg)*);
    |             ---------------------------- in this macro invocation (#2)
    |
   ::: compiler/rustc_query_impl/src/on_disk_cache.rs:676:17
    |
676 |                 debug_assert_eq!(hash.local_hash(), local_hash);

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_query_impl` due to 2 previous errors
