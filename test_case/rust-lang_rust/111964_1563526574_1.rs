
error[E0366]: Negative impls cannot be specialized
   --> compiler/rustc_data_structures/src/stable_hasher.rs:615:1
    |
615 | impl<V, HCX> !HashStable<HCX> for std::collections::HashSet<V> {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `RandomState` is not a generic parameter
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
   --> /home/gh-spastorino/rust3/library/std/src/collections/hash/set.rs:106:1
    |
106 | pub struct HashSet<T, S = RandomState> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: negative impls should be always applicable
   --> compiler/rustc_data_structures/src/stable_hasher.rs:615:15
    |
615 | impl<V, HCX> !HashStable<HCX> for std::collections::HashSet<V> {}
    |               ^^^^^^^^^^^^^^^ negative impls
