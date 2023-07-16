
   Compiling rustc-ap-arena v659.0.0
error[E0599]: no method named `reserve_in_place` found for struct `alloc::raw_vec::RawVec<T>` in the current scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-arena-659.0.0/lib.rs:228:39
    |
228 |                 if last_chunk.storage.reserve_in_place(currently_used_cap, n) {
    |                                       ^^^^^^^^^^^^^^^^ method not found in `alloc::raw_vec::RawVec<T>`

error[E0599]: no method named `reserve_in_place` found for struct `alloc::raw_vec::RawVec<u8>` in the current scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-arena-659.0.0/lib.rs:356:39
    |
356 |                 if last_chunk.storage.reserve_in_place(used_bytes, needed_bytes) {
    |                                       ^^^^^^^^^^^^^^^^ method not found in `alloc::raw_vec::RawVec<u8>`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] rustc_ap_arena test:false 0.187
error: could not compile `rustc-ap-arena`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] jsonrpc_server_utils test:false 7.894
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
[TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: ["clippy"] } -- 565.495
failed to test rls: could not build

...

error: Tool `rls` has regressed from test-pass to build-fail during beta week.
error: Tool `rustfmt` has regressed from test-pass to build-fail during beta week.
