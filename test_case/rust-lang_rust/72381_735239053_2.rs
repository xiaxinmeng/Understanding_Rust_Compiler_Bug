
./x.py doc src/librustdoc 
Updating only changed submodules
Submodules updated in 1.04 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 14.93s
Documenting stage0 rustdoc (x86_64-unknown-linux-gnu)
Documenting stage0 compiler (x86_64-unknown-linux-gnu)
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 1.03s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 6.04s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting coverage_test_macros v0.0.0 (/home/nixon/upstreams/rust/rust/compiler/rustc_mir/src/transform/coverage/test_macros)
    Checking measureme v9.0.0
    Checking tracing v0.1.19
    Checking sha-1 v0.9.1
    Checking sha2 v0.9.1
    Checking md-5 v0.9.1
    Checking chrono v0.4.15
    Checking synstructure v0.12.4
thread 'rustc' panicked at 'Unable to resolve external crate proc_macro2', src/librustdoc/core.rs:457:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

error: Unrecognized option: 'crate-version'

error: could not document `coverage_test_macros`

Caused by:
  process didn't exit successfully: `/home/nixon/upstreams/rust/rust/build/bootstrap/debug/rustdoc --edition=2018 --crate-type proc-macro --crate-name coverage_test_macros compiler/rustc_mir/src/transform/coverage/test_macros/src/lib.rs -o /home/nixon/upstreams/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/nixon/upstreams/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern proc_macro2=/home/nixon/upstreams/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/libproc_macro2-f29f2674397b203b.rmeta --extern proc_macro --crate-version 0.0.0` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/home/nixon/upstreams/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--features" " llvm" "--manifest-path" "/home/nixon/upstreams/rust/rust/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_fs_util" "-p" "rustc_codegen_llvm" "-p" "rustc_codegen_ssa" "-p" "rustc_expand" "-p" "rustc_plugin_impl" "-p" "rustc_trait_selection" "-p" "rustc_llvm" "-p" "rustc_apfloat" "-p" "rustc_feature" "-p" "rustc_lint_defs" "-p" "rustc_driver" "-p" "rustc_privacy" "-p" "rustc_mir_build" "-p" "rustc_incremental" "-p" "rustc_ast_pretty" "-p" "rustc_infer" "-p" "rustc_passes" "-p" "rustc_ty_utils" "-p" "rustc_lexer" "-p" "rustc_graphviz" "-p" "rustc_error_codes" "-p" "rustc_metadata" "-p" "rustc_typeck" "-p" "rustc_index" "-p" "rustc_ast" "-p" "rustc_query_system" "-p" "rustc_builtin_macros" "-p" "rustc_lint" "-p" "rustc_span" "-p" "rustc_parse_format" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_serialize" "-p" "rustc_ast_passes" "-p" "rustc_save_analysis" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_resolve" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_target" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_symbol_mangling" "-p" "rustc_session" "-p" "rustc_arena" "-p" "rustc_errors" "-p" "rustc_ast_lowering" "-p" "rustc_attr"
expected success, got: exit code: 101


failed to run: /home/nixon/upstreams/rust/rust/build/bootstrap/debug/bootstrap doc src/librustdoc
Build completed unsuccessfully in 0:00:51
