plain
   Compiling rustc_log v0.0.0 (/checkout/compiler/rustc_log)
error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:264:5
    |
264 |     pub fn dummy() -> Self {
    |
note: the lint level is defined here
   --> compiler/rustc_data_structures/src/graph/mod.rs:3:9
    |
---

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:274:5
    |
274 |     pub fn immediate_dominator(&self, node: Node) -> Node {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:286:5
    |
    |
286 |     pub fn is_dominated_by(&self, node: Node, dom: Node) -> bool {

error: missing documentation for a constant
  --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:76:1
   |
   |
76 | pub const INVALID_EDGE_INDEX: EdgeIndex = EdgeIndex(usize::MAX);

error: missing documentation for a constant
  --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:85:1
   |
   |
85 | pub const OUTGOING: Direction = Direction { repr: 0 };

error: missing documentation for a constant
  --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:87:1
   |
   |
87 | pub const INCOMING: Direction = Direction { repr: 1 };

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:136:5
    |
    |
136 |     pub fn next_node_index(&self) -> NodeIndex {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:141:5
    |
    |
141 |     pub fn add_node(&mut self, data: N) -> NodeIndex {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:148:5
    |
    |
148 |     pub fn mut_node_data(&mut self, idx: NodeIndex) -> &mut N {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:153:5
    |
    |
153 |     pub fn node_data(&self, idx: NodeIndex) -> &N {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:158:5
    |
    |
158 |     pub fn node(&self, idx: NodeIndex) -> &Node<N> {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:165:5
    |
    |
165 |     pub fn next_edge_index(&self) -> EdgeIndex {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:170:5
    |
    |
170 |     pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, data: E) -> EdgeIndex {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:191:5
    |
    |
191 |     pub fn edge(&self, idx: EdgeIndex) -> &Edge<E> {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:198:5
    |
    |
198 |     pub fn enumerated_nodes(&self) -> impl Iterator<Item = (NodeIndex, &Node<N>)> {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:203:5
    |
    |
203 |     pub fn enumerated_edges(&self) -> impl Iterator<Item = (EdgeIndex, &Edge<E>)> {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:220:5
    |
    |
220 |     pub fn outgoing_edges(&self, source: NodeIndex) -> AdjacentEdges<'_, N, E> {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:225:5
    |
    |
225 |     pub fn incoming_edges(&self, source: NodeIndex) -> AdjacentEdges<'_, N, E> {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:230:5
    |
    |
230 | /     pub fn adjacent_edges(
231 | |         &self,
232 | |         source: NodeIndex,
233 | |         direction: Direction,
...   |
236 | |         AdjacentEdges { graph: self, direction, next: first_edge }
    | |_____^

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:240:5
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:240:5
    |
240 | /     pub fn successor_nodes<'a>(
241 | |         &'a self,
242 | |         source: NodeIndex,
243 | |     ) -> impl Iterator<Item = NodeIndex> + 'a {
244 | |         self.outgoing_edges(source).targets()
    | |_____^

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:247:5
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:247:5
    |
247 | /     pub fn predecessor_nodes<'a>(
248 | |         &'a self,
249 | |         target: NodeIndex,
250 | |     ) -> impl Iterator<Item = NodeIndex> + 'a {
251 | |         self.incoming_edges(target).sources()
    | |_____^

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:255:5
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:255:5
    |
255 | /     pub fn depth_traverse(
256 | |         &self,
257 | |         start: NodeIndex,
258 | |         direction: Direction,
259 | |     ) -> DepthFirstTraversal<'_, N, E> {
260 | |         DepthFirstTraversal::with_start_node(self, start, direction)
    | |_____^

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:264:5
---

error: missing documentation for a struct
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:304:1
    |
304 | pub struct AdjacentEdges<'g, N, E> {

error: missing documentation for a struct
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:340:1
    |
    |
340 | pub struct DepthFirstTraversal<'g, N, E> {

error: missing documentation for an associated function
   --> compiler/rustc_data_structures/src/graph/implementation/mod.rs:349:5
    |
    |
349 | /     pub fn with_start_node(
350 | |         graph: &'g Graph<N, E>,
351 | |         start_node: NodeIndex,
352 | |         direction: Direction,
...   |
356 | |         DepthFirstTraversal { graph, stack: vec![start_node], visited, direction }
    | |_____^

error: missing documentation for a module
  --> compiler/rustc_data_structures/src/graph/mod.rs:10:1
  --> compiler/rustc_data_structures/src/graph/mod.rs:10:1
   |
10 | pub mod iterate;
   | ^^^^^^^^^^^^^^^^

error: missing documentation for a function
  --> compiler/rustc_data_structures/src/graph/iterate/mod.rs:10:1
   |
10 | / pub fn post_order_from<G: DirectedGraph + WithSuccessors + WithNumNodes>(
11 | |     graph: &G,
12 | |     start_node: G::Node,
13 | | ) -> Vec<G::Node> {
14 | |     post_order_from_to(graph, start_node, None)
   | |_^

error: missing documentation for a function
  --> compiler/rustc_data_structures/src/graph/iterate/mod.rs:18:1
  --> compiler/rustc_data_structures/src/graph/iterate/mod.rs:18:1
   |
18 | / pub fn post_order_from_to<G: DirectedGraph + WithSuccessors + WithNumNodes>(
19 | |     graph: &G,
20 | |     start_node: G::Node,
21 | |     end_node: Option<G::Node>,
29 | |     result
30 | | }
   | |_^


error: missing documentation for a function
  --> compiler/rustc_data_structures/src/graph/iterate/mod.rs:66:1
   |
66 | / pub fn reverse_post_order<G: DirectedGraph + WithSuccessors + WithNumNodes>(
67 | |     graph: &G,
68 | |     start_node: G::Node,
69 | | ) -> Vec<G::Node> {
72 | |     vec
73 | | }
   | |_^


error: missing documentation for a module
  --> compiler/rustc_data_structures/src/graph/mod.rs:14:1
   |
14 | pub mod vec_graph;

error: missing documentation for a struct
  --> compiler/rustc_data_structures/src/graph/vec_graph/mod.rs:10:1
   |
   |
10 | pub struct VecGraph<N: Idx> {

error: missing documentation for a trait
  --> compiler/rustc_data_structures/src/graph/mod.rs:56:1
   |
   |
56 | pub trait GraphSuccessors<'graph> {

error: missing documentation for an associated type
  --> compiler/rustc_data_structures/src/graph/mod.rs:57:5
   |
   |
57 |     type Item;
   |     ^^^^^^^^^^

error: missing documentation for an associated type
  --> compiler/rustc_data_structures/src/graph/mod.rs:58:5
   |
58 |     type Iter: Iterator<Item = Self::Item>;

error: missing documentation for a trait
  --> compiler/rustc_data_structures/src/graph/mod.rs:63:1
   |
   |
63 | / pub trait WithPredecessors: DirectedGraph
64 | | where
65 | |     Self: for<'graph> GraphPredecessors<'graph, Item = <Self as DirectedGraph>::Node>,
66 | | {
67 | |     fn predecessors(&self, node: Self::Node) -> <Self as GraphPredecessors<'_>>::Iter;
   | |_^

error: missing documentation for an associated function
  --> compiler/rustc_data_structures/src/graph/mod.rs:67:5
  --> compiler/rustc_data_structures/src/graph/mod.rs:67:5
   |
67 |     fn predecessors(&self, node: Self::Node) -> <Self as GraphPredecessors<'_>>::Iter;

error: missing documentation for a trait
  --> compiler/rustc_data_structures/src/graph/mod.rs:72:1
   |
   |
72 | pub trait GraphPredecessors<'graph> {

error: missing documentation for an associated type
  --> compiler/rustc_data_structures/src/graph/mod.rs:73:5
   |
   |
73 |     type Item;
   |     ^^^^^^^^^^

error: missing documentation for an associated type
  --> compiler/rustc_data_structures/src/graph/mod.rs:74:5
   |
74 |     type Iter: Iterator<Item = Self::Item>;

error: missing documentation for a trait
  --> compiler/rustc_data_structures/src/graph/mod.rs:78:1
   |
---

error: missing documentation for a trait
  --> compiler/rustc_data_structures/src/graph/mod.rs:83:1
   |
83 | / pub trait ControlFlowGraph:
84 | |     DirectedGraph + WithStartNode + WithPredecessors + WithSuccessors + WithNumNodes
85 | | {
86 | |     // convenient trait
   | |_^

error: could not compile `rustc_data_structures` due to 44 previous errors
warning: build failed, waiting for other jobs to finish...
