
error[E0277]: the trait bound `K: Hash` is not satisfied
  --> compiler/rustc_data_structures/src/sso/map.rs:89:29
   |
89 |             SsoHashMap::Map(FxHashMap::with_capacity_and_hasher(cap, Default::default()))
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Hash` is not implemented for `K`
   |
   = note: required by `AHashMap::<K, V, S>::with_capacity_and_hasher`
