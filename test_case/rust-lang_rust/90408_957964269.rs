plain
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.28
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0369]: binary operation `==` cannot be applied to type `IndexVec<I, (K, V)>`
   --> compiler/rustc_data_structures/src/sorted_map/index_map.rs:100:20
    |
100 |         self.items == other.items
    |         ---------- ^^ ----------- IndexVec<I, (K, V)>
    |         |
    |         IndexVec<I, (K, V)>

error[E0369]: binary operation `==` cannot be applied to type `Option<<G as graph::DirectedGraph>::Node>`
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                      --------- ^^ ---------- Option<<G as graph::DirectedGraph>::Node>
    | |                      |
    | |                      Option<<G as graph::DirectedGraph>::Node>
61  | |     });
62  | | }
    | |_- in this expansion of `$crate::assert_eq!` (#2)
...
...
239 | / macro_rules! debug_assert_eq {
240 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
241 | | }
    | |_- in this expansion of `debug_assert_eq!` (#1)
    |
   ::: compiler/rustc_data_structures/src/graph/scc/mod.rs:527:13
   ::: compiler/rustc_data_structures/src/graph/scc/mod.rs:527:13
    |
527 |               debug_assert_eq!(r, Some(node));
    |
help: consider further restricting this bound
    |
    |
205 |     G: DirectedGraph + WithNumNodes + WithSuccessors + std::cmp::PartialEq,


error[E0599]: the method `insert` exists for mutable reference `&mut HashSet<S, BuildHasherDefault<FxHasher>>`, but its trait bounds were not satisfied
   --> compiler/rustc_data_structures/src/graph/scc/mod.rs:546:57
    |
546 |                         .filter(move |&i| duplicate_set.insert(i))
    |                                                         ^^^^^^ method cannot be called on `&mut HashSet<S, BuildHasherDefault<FxHasher>>` due to unsatisfied trait bounds
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `S: std::cmp::Eq`
Some errors have detailed explanations: E0369, E0599.
For more information about an error, try `rustc --explain E0369`.
error: could not compile `rustc_data_structures` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
