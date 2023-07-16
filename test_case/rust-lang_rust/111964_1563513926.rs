plain
   Compiling tracing-subscriber v0.3.3
error[E0366]: Negative impls cannot be specialized
   --> compiler/rustc_data_structures/src/stable_hasher.rs:615:1
    |
615 | impl<V, HCX> !HashStable<HCX> for std::collections::HashSet<V> {}
    |
    = note: `RandomState` is not a generic parameter
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
   --> /checkout/library/std/src/collections/hash/set.rs:106:1
   --> /checkout/library/std/src/collections/hash/set.rs:106:1
    |
106 | pub struct HashSet<T, S = RandomState> {

error: negative impls should be always applicable
   --> compiler/rustc_data_structures/src/stable_hasher.rs:615:15
    |
    |
615 | impl<V, HCX> !HashStable<HCX> for std::collections::HashSet<V> {}
    |               ^^^^^^^^^^^^^^^ negative impls
For more information about this error, try `rustc --explain E0366`.
error: could not compile `rustc_data_structures` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:05:31
