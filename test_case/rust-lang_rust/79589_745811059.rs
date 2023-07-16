plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +028785483b502487002cfcac336956b2881a5b99:refs/remotes/pull/79589/merge
---
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:165:41
    |
165 |                     nodes.push(previous.index_to_node(data.red.node_indices[red_index]));
    |                                         |
    |                                         expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:170:41
    |
170 |                     nodes.push(previous.index_to_node(data.light_green.node_indices[lg_index]));
    |                                         |
    |                                         expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:175:41
    |
175 |                     nodes.push(previous.index_to_node(prev_index));
    |                                         |
    |                                         expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:498:53
    |
498 |             HybridIndex::Red(red_index) => previous.index_to_node(data.red.node_indices[red_index]),
    |                                                     |
    |                                                     expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:500:26
    |
500 |                 previous.index_to_node(data.light_green.node_indices[light_green_index])
    |                          |
    |                          expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:502:60
    |
502 |             HybridIndex::DarkGreen(prev_index) => previous.index_to_node(prev_index),
    |                                                            |
    |                                                            expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:627:41
    |
627 |                     nodes.push(previous.index_to_node(red.node_indices[i]));
    |                                         |
    |                                         expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:634:41
    |
634 |                     nodes.push(previous.index_to_node(lg.node_indices[i]));
    |                                         |
    |                                         expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:640:41
    |
640 |                     nodes.push(previous.index_to_node(prev_index));
    |                                         |
    |                                         expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
    |
36  |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37  |         &self,
    |         -----
    |         -----
38  |         dep_node_index: SerializedDepNodeIndex,
    |         --------------------------------------
39  |         tcx: CTX,

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> compiler/rustc_query_system/src/dep_graph/graph.rs:1476:32
     |
1476 |         let node = &prev_graph.index_to_node(prev_index);
     |                                |
     |                                expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> compiler/rustc_query_system/src/dep_graph/prev.rs:36:12
     |
36   |     pub fn index_to_node<CTX: DepContext<DepKind = K>>(
37   |         &self,
     |         -----
     |         -----
38   |         dep_node_index: SerializedDepNodeIndex,
     |         --------------------------------------
39   |         tcx: CTX,

error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0061`.
For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_query_system`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_span" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_ast" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_target" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_serialize" "-p" "rustc_feature" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_incremental" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_errors" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:13
== clock drift check ==
  local time: Wed Dec 16 07:04:38 UTC 2020
