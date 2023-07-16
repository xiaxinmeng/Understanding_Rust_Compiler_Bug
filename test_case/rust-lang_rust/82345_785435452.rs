plain
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error: associated function is never used: `register_dep_node_debug_str`
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:395:19
    |
395 |     pub(crate) fn register_dep_node_debug_str<F>(&self, dep_node: DepNode<K>, debug_str_gen: F)
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_query_system`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:08:32
