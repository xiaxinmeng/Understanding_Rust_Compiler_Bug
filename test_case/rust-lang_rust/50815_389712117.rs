plain
[00:26:45]    Compiling rls-data v0.16.0
[00:26:46]    Compiling backtrace v0.3.6
[00:26:49]    Compiling rustc-rayon v0.1.0
[00:26:55]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:26:56] error[E0271]: type mismatch resolving `for<'graph> <<Self as control_flow_graph::GraphPredecessors<'graph>>::Iter as std::iter::Iterator>::Item == <Self as control_flow_graph::GraphPredecessors<'graph>>::Item`
[00:26:56]   --> librustc_data_structures/control_flow_graph/mod.rs:20:1
[00:26:56]    |
[00:26:56] 20 | / pub trait ControlFlowGraph
[00:26:56] 21 | |     where Self: for<'graph> GraphPredecessors<'graph, Item=<Self as ControlFlowGraph>::Node>,
[00:26:56] 22 | |           Self: for<'graph> GraphSuccessors<'graph, Item=<Self as ControlFlowGraph>::Node>
[00:26:56] 23 | | {
[00:26:56] ...  |
[00:26:56] 31 | |                             -> <Self as GraphSuccessors<'graph>>::Iter;
[00:26:56] 32 | | }
[00:26:56]    | |_^ expected contraph>::Node>,
[00:26:56] 22 | |           Self: for<'graph> GraphSuccessors<'graph, Item=<Self as ControlFlowGraph>::Node>
[00:26:56] 23 | | {
[00:26:56] ...  |
[00:26:56] 31 | |                             -> <Self as GraphSuccessors<'graph>>::Iter;
[00:26:56] 32 | | }
[00:26:56]    | |_^ expected control_flow_graph::ControlFlowGraph::Node, found control_flow_graph::GraphSuccessors::Item
[00:26:56]    |
[00:26:56]    = note: expected type `<Self as control_flow_graph::ControlFlowGraph>::Node`
[00:26:56]               found type `<Self as control_flow_graph::GraphSuccessors<'_>>::Item`
[00:26:56] note: required by `control_flow_graph::GraphSuccessors`
[00:26:56]   --> librustc_data_structures/control_flow_graph/mod.rs:39:1
[00:26:56]    |
[00:26:56] 39 | pub trait GraphSuccessors<'graph> {
[00:26:56] 
[00:26:56] 
ssors<'graph>>::Item`
[00:26:56]   --> librustc_data_structures/control_flow_graph/mod.rs:28:5
[00:26:56]    |
[00:26:56] 28 | /     fn predecessors<'graph>(&'graph self, node: Self::Node)
[00:26:56] 29 | |                             -> <Self as GraphPredecessors<'graph>>::Iter;
[00:26:56]    | |_________________________________________________________________________^ expected control_flow_graph::ControlFlowGraph::Node, found control_flow_graph::GraphPredecessors::Item
[00:26:56]    |
[00:26:56]    = note: expected type `<Self as control_flow_graph::ControlFlowGraph>::Node`
[00:26:56]               found type `<Self as control_flow_graph::GraphPredecessors<'_>>::Item`
[00:26:56] note: required by `control_flow_graph::GraphPredecessors`
[00:26:56]   --> librustc_data_structures/control_flow_graph/mod.rs:34:1
[00:26:56]    |
[00:26:56] 34 | pub trait GraphPredecessors<'graph> {
[00:26:56] 
[00:26:56] 
[00:26:56] color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:26:56] expected success, got: exit code: 101
[00:26:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:26:56] travis_fold:end:stage1-rustc

[00:26:56] travis_time:end:stage1-rustc:start=1526519020161639596,finish=1526519062654287402,duration=42492647806


[00:26:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:56] Build completed unsuccessfully in 0:21:37
[00:26:56] make: *** [all] Error 1
[00:26:56] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3acb07c5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
