plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0433]: failed to resolve: use of undeclared type `FxHashMap`
   --> compiler/rustc_session/src/parse.rs:201:51
    |
201 |             ambiguous_block_expr_parse: Lock::new(FxHashMap::default()),
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-hash-1.1.0/src/lib.rs:47:1
    |
    |
47  | pub type FxHashSet<V> = HashSet<V, BuildHasherDefault<FxHasher>>;
    | ----------------------------------------------------------------- similarly named type alias `FxHashSet` defined here
help: a type alias with a similar name exists
    |
    |
201 |             ambiguous_block_expr_parse: Lock::new(FxHashSet::default()),
help: consider importing this type alias
    |
4   | use rustc_data_structures::stable_map::FxHashMap;
    |
