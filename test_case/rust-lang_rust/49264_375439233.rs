
[00:07:30] [TIMING] StartupObjects { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 0.569
[00:10:40] [TIMING] Std { target: "i686-pc-windows-gnu", compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" } } -- 190.281
[00:12:01] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", tool: "tidy", path: "src/tools/tidy", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 80.332
[00:12:01] [TIMING] Tidy { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 0.140
[00:14:04] [TIMING] Tidy -- 123.503
[00:15:05] [TIMING] Bootstrap -- 60.268
[00:15:24] [TIMING] Test { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 19.631
[00:36:58] [TIMING] Rustc { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 1293.790
[00:52:50] [TIMING] Llvm { target: "i686-pc-windows-gnu", emscripten: false } -- 951.358
[00:55:53] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", backend: "llvm" } -- 183.629
[00:55:54] [TIMING] Assemble { target_compiler: Compiler { stage: 1, host: "i686-pc-windows-gnu" } } -- 0.333
[00:55:54] [TIMING] StartupObjects { compiler: Compiler { stage: 1, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 0.141
[00:58:20] [TIMING] Std { target: "i686-pc-windows-gnu", compiler: Compiler { stage: 1, host: "i686-pc-windows-gnu" } } -- 145.682
[00:58:44] [TIMING] Test { compiler: Compiler { stage: 1, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 24.514
[01:31:10] [TIMING] Rustc { compiler: Compiler { stage: 1, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 1946.125
[01:34:39] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", backend: "llvm" } -- 209.135
[01:34:40] [TIMING] TestHelpers { target: "i686-pc-windows-gnu" } -- 0.368
[01:37:24] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", tool: "compiletest", path: "src/tools/compiletest", mode: Libtest, is_ext_tool: false, extra_features: [] } -- 164.324
[01:42:04] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "ui", suite: "ui" } -- 279.832
[01:42:40] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "parse-fail", suite: "parse-fail" } -- 35.715
[01:43:22] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "run-fail", suite: "run-fail" } -- 42.572
[01:43:29] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "run-pass-valgrind", suite: "run-pass-valgrind" } -- 7.081
[01:44:20] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "mir-opt", suite: "mir-opt" } -- 50.585
[01:44:31] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "codegen", suite: "codegen" } -- 11.027
[01:44:39] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "codegen-units", suite: "codegen-units" } -- 8.449
[01:45:32] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "incremental", suite: "incremental" } -- 52.041
[01:46:04] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "debuginfo-gdb", suite: "debuginfo" } -- 32.197
[01:46:04] [TIMING] RustcLink { compiler: Compiler { stage: 1, host: "i686-pc-windows-gnu" }, target_compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 0.103
[01:46:49] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "ui", suite: "ui-fulldeps" } -- 45.058
[01:55:22] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "run-pass", suite: "run-pass-fulldeps" } -- 512.538
[01:55:29] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "run-fail", suite: "run-fail-fulldeps" } -- 7.014
[01:58:12] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "compile-fail", suite: "compile-fail-fulldeps" } -- 163.293
[01:58:17] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "incremental", suite: "incremental-fulldeps" } -- 4.843
[02:03:22] [TIMING] Rustdoc { host: "i686-pc-windows-gnu" } -- 305.601
[02:08:18] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: "rustdoc", suite: "rustdoc" } -- 295.567
[02:08:27] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libtest, test_kind: Test, krate: "term" } -- 8.871
[02:08:46] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libtest, test_kind: Test, krate: "test" } -- 18.740
[02:14:02] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libstd, test_kind: Test, krate: "alloc" } -- 316.828
[02:14:03] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libstd, test_kind: Test, krate: "alloc_system" } -- 1.041
[02:22:42] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libstd, test_kind: Test, krate: "core" } -- 518.282
[02:22:44] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libstd, test_kind: Test, krate: "panic_abort" } -- 2.167
[02:30:27] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libstd, test_kind: Test, krate: "std" } -- 463.380
[02:30:47] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libstd, test_kind: Test, krate: "std_unicode" } -- 19.988
[02:30:48] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Libstd, test_kind: Test, krate: "unwind" } -- 1.181
[02:30:54] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "arena" } -- 5.299
[02:30:57] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "fmt_macros" } -- 3.517
[02:31:02] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "graphviz" } -- 4.618
[02:31:06] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "proc_macro" } -- 3.791
[02:34:00] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc" } -- 174.336
[02:34:01] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_allocator" } -- 0.868
[02:34:47] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_apfloat" } -- 45.902
[02:35:10] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_back" } -- 23.366
[02:35:11] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_borrowck" } -- 0.826
[02:35:13] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_const_math" } -- 2.403
[02:35:44] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_data_structures" } -- 30.449
[02:37:42] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_driver" } -- 118.383
[02:37:46] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_errors" } -- 3.769
[02:37:55] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_incremental" } -- 9.392
[02:37:56] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_lint" } -- 1.048
[02:38:05] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_metadata" } -- 8.950
[02:38:55] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_mir" } -- 49.624
[02:39:01] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_passes" } -- 5.793
[02:39:06] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_platform_intrinsics" } -- 5.399
[02:39:11] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_plugin" } -- 5.060
[02:39:16] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_privacy" } -- 5.086
[02:39:17] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_resolve" } -- 0.861
[02:39:24] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_save_analysis" } -- 6.727
[02:39:28] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_traits" } -- 3.641
[02:39:29] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_trans_utils" } -- 0.847
[02:39:29] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc_typeck" } -- 0.873
[02:40:14] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "serialize" } -- 44.750
[02:42:13] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "syntax" } -- 118.561
[02:42:22] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "syntax_ext" } -- 9.367
[02:42:28] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "syntax_pos" } -- 6.028
[02:42:45] [TIMING] Crate { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", mode: Librustc, test_kind: Test, krate: "rustc-main" } -- 16.629
[02:44:08] [TIMING] CrateRustdoc { host: "i686-pc-windows-gnu", test_kind: Test } -- 82.987
[02:44:26] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 18.210
[02:44:28] [TIMING] UnstableBookGen { target: "i686-pc-windows-gnu" } -- 1.690
[02:47:50] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 202.304
[02:47:53] [TIMING] RustbookSrc { target: "i686-pc-windows-gnu", name: "unstable-book", src: "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\md-doc" } -- 3.012
[02:47:54] [TIMING] RustbookSrc { target: "i686-pc-windows-gnu", name: "book/first-edition", src: "C:\\projects\\rust\\src/doc" } -- 0.641
[02:47:55] [TIMING] RustbookSrc { target: "i686-pc-windows-gnu", name: "book/second-edition", src: "C:\\projects\\rust\\src/doc" } -- 1.250
[02:47:57] [TIMING] Standalone { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu" } -- 1.908
[02:48:01] [TIMING] TheBook { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", name: "book" } -- 4.283
[02:50:28] [TIMING] Std { stage: 2, target: "i686-pc-windows-gnu" } -- 146.590
[02:52:04] [TIMING] Rustc { stage: 2, target: "i686-pc-windows-gnu" } -- 96.619
[02:52:35] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 30.277
[02:52:35] [TIMING] ErrorIndex { target: "i686-pc-windows-gnu" } -- 0.364
[02:52:36] [TIMING] RustbookSrc { target: "i686-pc-windows-gnu", name: "nomicon", src: "C:\\projects\\rust\\src/doc" } -- 0.861
[02:52:37] [TIMING] RustbookSrc { target: "i686-pc-windows-gnu", name: "reference", src: "C:\\projects\\rust\\src/doc" } -- 0.821
[02:52:37] [TIMING] RustbookSrc { target: "i686-pc-windows-gnu", name: "rustdoc", src: "C:\\projects\\rust\\src/doc" } -- 0.279
[02:52:42] [TIMING] RustbookSrc { target: "i686-pc-windows-gnu", name: "rust-by-example", src: "C:\\projects\\rust\\src/doc" } -- 4.418
[02:52:42] [TIMING] CargoBook { target: "i686-pc-windows-gnu", name: "cargo" } -- 0.658
[02:53:03] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", tool: "linkchecker", path: "src/tools/linkchecker", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 20.454
[02:55:11] [TIMING] Linkcheck { host: "i686-pc-windows-gnu" } -- 129.028
[02:56:23] [TIMING] ErrorIndex { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" } } -- 71.835
[02:56:26] [TIMING] DocTest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, path: "src/doc/rustdoc", name: "rustdoc", is_ext_doc: false } -- 2.660
[02:56:50] [TIMING] DocTest { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" }, path: "src/doc/unstable-book", name: "unstable-book", is_ext_doc: false } -- 24.223
[02:56:52] [TIMING] RustdocJS { host: "i686-pc-windows-gnu", target: "i686-pc-windows-gnu" } -- 1.489
[02:56:54] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-pc-windows-gnu" }, target: "i686-pc-windows-gnu", tool: "rustdoc-themes", path: "src/tools/rustdoc-themes", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 2.535
[02:56:54] [TIMING] RustdocTheme { compiler: Compiler { stage: 2, host: "i686-pc-windows-gnu" } } -- 0.126
