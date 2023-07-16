plain
[00:23:23]    Compiling parking_lot v0.5.5
[00:23:25]    Compiling flate2 v1.0.1
[00:23:25]    Compiling backtrace v0.3.6
[00:23:31]    Compiling rustc-rayon v0.1.0
aph>::Node>,
[00:23:35] 22 | |           Self: for<'graph> GraphSuccessors<'graph, Item=<Self as ControlFlowGraph>::Node>
[00:23:35] 23 | | {
[00:23:35] ...  |
[00:23:35] 31 | |                             -> <Self as GraphSuccessors<'graph>>::Iter;
[00:23:35] 32 | | }
[00:23:35]    | |_^ expected control_flow_graph::ControlFlowGraph::Node, found control_flow_graph::GraphSuccessors::Item
[00:23:35]    |
[00:23:35]    = note: expected type `<Self as control_flow_graph::ControlFlowGraph>::Node`
[00:23:35]               found type `<Self as control_flow_graph::GraphSuccessors<'_>>::Item`
[00:23:35] note: required by `control_flow_graph::GraphSuccessors`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:39:1
[00:23:35]    |
[00:23:35] 39 | pub trait GraphSuccessors<'graph> {
[00:23:35] 
[00:23:35] 
[00:23:35] error[E0271]: type mismatch resolving `for<'graph> <<Self as control_flow_graph::GraphPredecessors<'graph>>::Iter as std::iter::Iterator>::Item == <Self as control_flow_graph::GraphPredecessors<'graph>>::Item`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:26:5
[00:23:35]    |
[00:23:35] 26 |     fn num_nodes(&self) -> usize;
[00:23:35]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected control_flow_graph::ControlFlowGraph::Node, found control_flow_graph::GraphPredecessors::Item
[00:23:35]    |
[00:23:35]    = note: expected type `<Self as control_flow_graph::ControlFlowGraph>::Node`
[00:23:35]               found type `<Self as control_flow_graph::GraphPredecessors<'_>>::Item`
[00:23:35] note: required by `control_flow_graph::GraphPredecessors`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:34:1
[00:23:35]    |
[00:23:35] 34 | pub trait GraphPredecessors<'graph> {
[00:23:35] 
[00:23:35] 
[00:23:35] error[E0271]: type mismatch resolving `for<'graph> <<Self as control_flow_graph::GraphSuccessors<'graph>>::Iter as std::iter::Iterator>::Item == <Self as control_flow_graph::GraphSuccessors<'graph>>::Item`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:26:5
[00:23:35]    |
[00:23:35] 26 |     fn num_nodes(&self) -> usize;
[00:23:35]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected control_flow_graph::ControlFlowGraph::Node, found control_flow_graph::GraphSuccessors::Item
[00:23:35]    |
[00:23:35]    = note: expected type `<Self as control_flow_graph::ControlFlowGraph>::Node`
[00:23:35]               found type `<Self as control_flow_graph::GraphSuccessors<'_>>::Item`
[00:23:35] note: required by `control_flow_graph::GraphSuccessors`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:39:1
[00:23:35]    |
[00:23:35] 39 | pub trait GraphSuccessors<'graph> {
[00:23:35] 
[00:23:35] 
[00:23:35] error[E0271]: type mismatch resolving `for<'graph> <<Self as control_flow_graph::GraphPredecessors<'graph>>::Iter as std::iter::Iterator>::Item == <Self as control_flow_graph::GraphPredecessors<'graph>>::Item`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:27:5
[00:23:35]    |
[00:23:35] 27 |     fn start_node(&self) -> Self::Node;
[00:23:35]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected control_flow_graph::ControlFlowGraph::Node, found control_flow_graph::GraphPredecessors::Item
[00:23:35]    |
[00:23:35]    = note: expected type `<Self as control_flow_graph::ControlFlowGraph>::Node`
[00:23:35]               found type `<Self as control_flow_graph::GraphPredecessors<'_>>::Item`
[00:23:35] note: required by `control_flow_graph::GraphPredecessors`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:34:1
[00:23:35]    |
[00:23:35] 34 | pub trait GraphPredecessors<'graph> {
[00:23:35] 
[00:23:35] 
[00:23:35] error[E0271]: type mismatch resolving `for<'graph> <<Self as control_flo^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:23:35] 
[00:23:35] error[E0271]: type mismatch resolving `for<'graph> <<Self as control_flow_graph::GraphSuccessors<'graph>>::Iter as std::iter::Iterator>::Item == <Self as control_flow_graph::GraphSuccessors<'graph>>::Item`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:28:5
[00:23:35]    |
[00:23:35] 28 | /     fn predecessors<'graph>(&'graph self, node: Self::Node)
[00:23:35] 29 | |                             -> <Self as GraphPredecessors<'graph>>::Iter;
[00:23:35]    | |_________________________________________________________________________^ expected control_flow_graph::ControlFlowGraph::Node, found control_flow_graph::GraphSuccessors::Item
[00:23:35]    |
[00:23:35]    = note: expected type `<Self as control_flow_graph::ControlFlowGraph>::Node`
[00:23:35]               found type `<Self as control_flow_graph::GraphSuccessors<'_>>::Item`
[00:23:35] note: required by `control_flow_graph::GraphSuccessors`
[00:23:35]   --> librustc_data_structures/control_flow_graph/mod.rs:39:1
[00:23:35] 
[00:23:35] For more information about this error, try `rustc --explain E0271`.
[00:23:35] error: Could not compile `rustc_data_structures`.
[00:23:35] 
[00:23:35] 
[00:23:35] Caused by:
[00:23:35]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_data_structures librustc_data_structures/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=ec2ad06f18660d2e -C extra-filename=-ec2ad06f18660d2e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-a8b86095ac289705.so --extern ena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libena-75092fb9537b4804.rlib --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-fee7d6c5c9c3b040.rlib --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-c14546c9879586fd.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c485a7f1a48de680.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-09bdd144093f3a08.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-09bdd144093f3a08.rlib --extern parking_lot_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot_core-1b979a7cb32e7a25.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-9090f533a9ec98a5.rlib --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-b413daa63d1ce253.rlib` (exit code: 101)
[00:23:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:23:35] expected success, got: exit code: 101
[00:23:35] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:23:35] travis_fold:end:stage1-rustc

[00:23:35] travis_time:end:stage1-rustc:start=1526521189383890676,finish=1526521226781895510,duration=37398004834


[00:23:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:35] Build completed unsuccessfully in 0:18:34
[00:23:35] Makefile:28: recipe for target 'all' failed
[00:23:35] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05dd599c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
