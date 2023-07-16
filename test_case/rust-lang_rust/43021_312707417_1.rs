
cargo check
   Compiling panopticon-core v0.16.0 (file:///home/m4b/projects/panopticon/core)
error[E0564]: only named lifetimes are allowed in `impl Trait`, but `` was found in the type `std::iter::FilterMap<std::iter::Map<std::collections::hash_map::Keys<'_, panopticon_graph_algos::adjacency_list::AdjacencyListVertexDescriptor, function::ControlFlowTarget>, fn(&panopticon_graph_algos::adjacency_list::AdjacencyListVertexDescriptor) -> panopticon_graph_algos::adjacency_list::AdjacencyListVertexDescriptor>, [closure@function.rs:448:31: 448:56 cfg:&&'a panopticon_graph_algos::AdjacencyList<function::ControlFlowTarget, il::Guard>]>`
   --> function.rs:447:44
    |
447 | fn derp<'a> (cfg: &'a ControlFlowGraph) -> impl Iterator<Item = &'a ControlFlowTarget> {
    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0564]: only named lifetimes are allowed in `impl Trait`, but `` was found in the type `std::iter::FilterMap<std::iter::Map<std::collections::hash_map::Keys<'_, panopticon_graph_algos::adjacency_list::AdjacencyListVertexDescriptor, function::ControlFlowTarget>, fn(&panopticon_graph_algos::adjacency_list::AdjacencyListVertexDescriptor) -> panopticon_graph_algos::adjacency_list::AdjacencyListVertexDescriptor>, [closure@function.rs:448:31: 448:56 cfg:&&'a panopticon_graph_algos::AdjacencyList<function::ControlFlowTarget, il::Guard>]>`
   --> function.rs:447:44
    |
447 | fn derp<'a> (cfg: &'a ControlFlowGraph) -> impl Iterator<Item = &'a ControlFlowTarget> {
    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
