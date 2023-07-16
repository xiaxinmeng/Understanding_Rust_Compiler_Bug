
[00:06:24] [TIMING] DocTest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, path: "src/doc/unstable-book", name: "unstable-book", is_ext_doc: false } -- 0.107
[00:07:29] [TIMING] Std { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } } -- 65.520
[00:08:50] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", tool: "tidy", path: "src/tools/tidy", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 80.837
[00:09:00] [TIMING] Tidy -- 9.722
[00:10:22] [TIMING] Bootstrap -- 82.171
[00:10:42] [TIMING] Test { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } } -- 19.363
[00:34:59] [TIMING] Rustc { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } } -- 1457.031
[00:47:19] [TIMING] Llvm { target: "x86_64-pc-windows-msvc", emscripten: false } -- 740.515
[00:50:22] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", backend: "llvm" } -- 183.240
[00:52:23] [TIMING] Std { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 1, host: "x86_64-pc-windows-msvc" } } -- 120.481
[00:52:47] [TIMING] Test { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 1, host: "x86_64-pc-windows-msvc" } } -- 24.185
[01:26:35] [TIMING] Rustc { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 1, host: "x86_64-pc-windows-msvc" } } -- 2027.705
[01:30:15] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", backend: "llvm" } -- 220.489
[01:30:16] [TIMING] TestHelpers { target: "x86_64-pc-windows-msvc" } -- 0.515
[01:33:00] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", tool: "compiletest", path: "src/tools/compiletest", mode: Libtest, is_ext_tool: false, extra_features: [] } -- 164.413
[01:35:03] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "ui", suite: "ui" } -- 122.648
[01:58:34] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "run-pass", suite: "run-pass" } -- 1410.876
[02:03:26] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "compile-fail", suite: "compile-fail" } -- 291.957
[02:03:33] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "parse-fail", suite: "parse-fail" } -- 6.987
[02:04:09] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "run-fail", suite: "run-fail" } -- 36.055
[02:04:14] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "run-pass-valgrind", suite: "run-pass-valgrind" } -- 4.991
[02:04:50] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "mir-opt", suite: "mir-opt" } -- 35.768
[02:04:57] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "codegen", suite: "codegen" } -- 7.507
[02:05:03] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "codegen-units", suite: "codegen-units" } -- 6.195
[02:05:40] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "incremental", suite: "incremental" } -- 36.709
[02:05:40] [TIMING] RustcLink { compiler: Compiler { stage: 1, host: "x86_64-pc-windows-msvc" }, target_compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" } -- 0.110
[02:06:12] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "ui", suite: "ui-fulldeps" } -- 31.750
[02:13:16] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "run-pass", suite: "run-pass-fulldeps" } -- 424.214
[02:13:22] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "run-fail", suite: "run-fail-fulldeps" } -- 6.015
[02:15:48] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "compile-fail", suite: "compile-fail-fulldeps" } -- 146.048
[02:15:52] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "incremental", suite: "incremental-fulldeps" } -- 4.004
[02:19:57] [TIMING] Rustdoc { host: "x86_64-pc-windows-msvc" } -- 244.232
[02:24:02] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: "rustdoc", suite: "rustdoc" } -- 245.393
[02:24:09] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libtest, test_kind: Test, krate: "term" } -- 6.668
[02:24:26] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libtest, test_kind: Test, krate: "test" } -- 17.869
[02:28:24] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd, test_kind: Test, krate: "alloc" } -- 237.669
[02:28:25] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd, test_kind: Test, krate: "alloc_system" } -- 0.570
[02:35:23] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd, test_kind: Test, krate: "core" } -- 417.928
[02:35:23] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd, test_kind: Test, krate: "panic_abort" } -- 0.534
[02:40:57] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd, test_kind: Test, krate: "std" } -- 334.237
[02:40:58] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd, test_kind: Test, krate: "std_unicode" } -- 0.469
[02:40:58] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd, test_kind: Test, krate: "unwind" } -- 0.468
[02:41:02] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "arena" } -- 3.901
[02:41:05] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "fmt_macros" } -- 2.283
[02:41:08] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "graphviz" } -- 3.655
[02:41:11] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "proc_macro" } -- 2.454
[02:44:10] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc" } -- 179.182
[02:44:10] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_allocator" } -- 0.396
[02:44:40] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_apfloat" } -- 29.980
[02:45:00] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_back" } -- 19.489
[02:45:00] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_borrowck" } -- 0.410
[02:45:02] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_const_math" } -- 1.514
[02:45:28] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_data_structures" } -- 26.158
[02:47:06] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_driver" } -- 98.058
[02:47:09] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_errors" } -- 2.869
[02:47:15] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_incremental" } -- 6.482
[02:47:16] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_lint" } -- 0.395
[02:47:22] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_metadata" } -- 6.410
[02:48:03] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_mir" } -- 41.145
[02:48:06] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_passes" } -- 2.534
[02:48:10] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_platform_intrinsics" } -- 4.519
[02:48:12] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_plugin" } -- 1.909
[02:48:14] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_privacy" } -- 2.269
[02:48:15] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_resolve" } -- 0.393
[02:48:18] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_save_analysis" } -- 3.625
[02:48:20] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_traits" } -- 2.056
[02:48:21] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_trans_utils" } -- 0.387
[02:48:21] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc_typeck" } -- 0.426
[02:48:57] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "serialize" } -- 35.807
[02:50:51] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "syntax" } -- 113.561
[02:50:58] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "syntax_ext" } -- 7.342
[02:51:02] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "syntax_pos" } -- 4.231
[02:51:04] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Librustc, test_kind: Test, krate: "rustc-main" } -- 2.060
[02:51:48] [TIMING] CrateRustdoc { host: "x86_64-pc-windows-msvc", test_kind: Test } -- 43.588
[02:52:18] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 30.355
[02:52:20] [TIMING] UnstableBookGen { target: "x86_64-pc-windows-msvc" } -- 1.314
[02:56:44] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 264.779
[02:56:47] [TIMING] RustbookSrc { target: "x86_64-pc-windows-msvc", name: "unstable-book", src: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\md-doc" } -- 2.799
[02:56:49] [TIMING] RustbookSrc { target: "x86_64-pc-windows-msvc", name: "book/first-edition", src: "C:\\projects\\rust\\src/doc" } -- 1.578
[02:56:52] [TIMING] RustbookSrc { target: "x86_64-pc-windows-msvc", name: "book/second-edition", src: "C:\\projects\\rust\\src/doc" } -- 3.407
[02:56:53] [TIMING] Standalone { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" } -- 1.008
[02:56:56] [TIMING] TheBook { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", name: "book" } -- 2.417
[02:58:45] [TIMING] Std { stage: 2, target: "x86_64-pc-windows-msvc" } -- 109.304
[02:59:44] [TIMING] WhitelistedRustc { stage: 2, target: "x86_64-pc-windows-msvc" } -- 58.824
[02:59:51] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 7.371
[02:59:51] [TIMING] ErrorIndex { target: "x86_64-pc-windows-msvc" } -- 0.130
[02:59:52] [TIMING] RustbookSrc { target: "x86_64-pc-windows-msvc", name: "nomicon", src: "C:\\projects\\rust\\src/doc" } -- 0.785
[02:59:53] [TIMING] RustbookSrc { target: "x86_64-pc-windows-msvc", name: "reference", src: "C:\\projects\\rust\\src/doc" } -- 0.981
[02:59:53] [TIMING] RustbookSrc { target: "x86_64-pc-windows-msvc", name: "rustdoc", src: "C:\\projects\\rust\\src/doc" } -- 0.160
[02:59:55] [TIMING] RustbookSrc { target: "x86_64-pc-windows-msvc", name: "rust-by-example", src: "C:\\projects\\rust\\src/doc" } -- 1.935
[02:59:56] [TIMING] CargoBook { target: "x86_64-pc-windows-msvc", name: "cargo" } -- 0.450
