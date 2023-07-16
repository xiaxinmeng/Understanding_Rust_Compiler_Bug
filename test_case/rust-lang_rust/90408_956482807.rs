plain
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.28
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0369]: binary operation `==` cannot be applied to type `<G as graph::DirectedGraph>::Node`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                      --------- ^^ ---------- <G as graph::DirectedGraph>::Node
   | |                      |
   | |                      <G as graph::DirectedGraph>::Node
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: compiler/rustc_data_structures/src/graph/dominators/mod.rs:24:5
   |
24 |       assert_eq!(rpo[0], start_node);
   |
   |
   = note: the trait `std::cmp::PartialEq` is not implemented for `<G as graph::DirectedGraph>::Node`

error[E0369]: binary operation `!=` cannot be applied to type `Option<<G as graph::DirectedGraph>::Node>`
  --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:52:25
   |
52 |             if new_idom != immediate_dominators[node] {
   |                -------- ^^ -------------------------- Option<<G as graph::DirectedGraph>::Node>
   |                |
   |                Option<<G as graph::DirectedGraph>::Node>
help: consider further restricting this bound
   |
   |
22 | fn dominators_given_rpo<G: ControlFlowGraph + std::cmp::PartialEq>(graph: G, rpo: &[G::Node]) -> Dominators<G::Node> {


error[E0369]: binary operation `!=` cannot be applied to type `Node`
  --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:68:17
68 |     while node1 != node2 {
68 |     while node1 != node2 {
   |           ----- ^^ ----- Node
   |           Node
   |
help: consider further restricting this bound
   |
   |
62 | fn intersect<Node: Idx + std::cmp::PartialEq>(


error[E0369]: binary operation `==` cannot be applied to type `Node`
   --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:108:41
    |
108 |         self.dominators(node).any(|n| n == dom)
    |                                       - ^^ --- Node
    |                                       Node
    |
help: consider further restricting this bound
    |
    |
87  | impl<Node: Idx + std::cmp::PartialEq> Dominators<Node> {


error[E0369]: binary operation `==` cannot be applied to type `Node`
   --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:131:20
    |
131 |             if dom == node {
    |                --- ^^ ---- Node
    |                Node
    |
help: consider further restricting this bound
    |
    |
125 | impl<'dom, Node: Idx + std::cmp::PartialEq> Iterator for Iter<'dom, Node> {


error[E0369]: binary operation `!=` cannot be applied to type `<G as graph::DirectedGraph>::Node`
   --> compiler/rustc_data_structures/src/graph/scc/mod.rs:314:34
    |
314 |                     assert!(node != parent, "Node can not be in cycle with itself");
    |                             ---- ^^ ------ <G as graph::DirectedGraph>::Node
    |                             |
    |                             <G as graph::DirectedGraph>::Node
    |
    = note: the trait `std::cmp::PartialEq` is not implemented for `<G as graph::DirectedGraph>::Node`

error[E0369]: binary operation `==` cannot be applied to type `<G as graph::DirectedGraph>::Node`
   --> compiler/rustc_data_structures/src/graph/scc/mod.rs:362:30
    |
362 |             if previous_node == node {
    |                ------------- ^^ ---- <G as graph::DirectedGraph>::Node
    |                |
    |                <G as graph::DirectedGraph>::Node
    |
    = note: the trait `std::cmp::PartialEq` is not implemented for `<G as graph::DirectedGraph>::Node`

error[E0277]: the trait bound `N: Ord` is not satisfied
  --> compiler/rustc_data_structures/src/graph/vec_graph/mod.rs:23:20
   |
23 |         edge_pairs.sort();
   |                    ^^^^ the trait `Ord` is not implemented for `N`
   |
   = note: required because of the requirements on the impl of `Ord` for `(N, N)`
   |
   |
20 | impl<N: Idx + std::cmp::Ord> VecGraph<N> {


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
Some errors have detailed explanations: E0277, E0369, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_data_structures` due to 11 previous errors
warning: build failed, waiting for other jobs to finish...
