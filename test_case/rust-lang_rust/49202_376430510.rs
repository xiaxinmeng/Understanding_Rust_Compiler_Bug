
[00:03:49] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 67.446
[00:05:01] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 71.658
[00:05:04] [TIMING] Tidy -- 2.874
[00:05:06] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.595
[00:05:18] [TIMING] Test { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 12.772
[00:19:34] [TIMING] Rustc { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 855.453
[00:24:54] [TIMING] Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false } -- 320.624
[00:26:04] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 69.884
[00:31:20] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 315.508
[00:31:49] [TIMING] Test { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 29.295
[01:08:27] [TIMING] Rustc { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 2197.821
[01:11:01] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 153.982
[01:14:44] [TIMING] Rustdoc { host: "x86_64-unknown-linux-gnu" } -- 223.521
[01:14:46] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.540
[01:14:47] [TIMING] Test { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.510
[01:14:48] [TIMING] Rustc { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.553
[01:14:48] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.543
[01:14:48] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.419
[01:14:49] [TIMING] Test { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.337
[01:14:49] [TIMING] Rustc { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.381
[01:14:50] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.394
[01:16:02] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 72.478
[01:16:03] [TIMING] UnstableBookGen { target: "x86_64-unknown-linux-gnu" } -- 0.511
[01:18:12] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 129.785
[01:18:15] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "unstable-book", src: "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc" } -- 2.690
[01:18:16] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/first-edition", src: "/checkout/src/doc" } -- 0.592
[01:18:17] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/second-edition", src: "/checkout/src/doc" } -- 1.102
[01:18:17] [TIMING] Rustdoc { host: "x86_64-unknown-linux-gnu" } -- 0.373
[01:18:18] [TIMING] Standalone { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.461
[01:18:19] [TIMING] TheBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", name: "book" } -- 1.148
[01:23:03] [TIMING] Std { stage: 2, target: "x86_64-unknown-linux-gnu" } -- 283.958
[01:25:20] [TIMING] WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" } -- 137.121
[01:25:26] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 6.457
[01:25:26] [TIMING] ErrorIndex { target: "x86_64-unknown-linux-gnu" } -- 0.118
[01:25:27] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "nomicon", src: "/checkout/src/doc" } -- 0.366
[01:25:27] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "reference", src: "/checkout/src/doc" } -- 0.480
[01:25:29] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rust-by-example", src: "/checkout/src/doc" } -- 1.612
[01:25:29] [TIMING] CargoBook { target: "x86_64-unknown-linux-gnu", name: "cargo" } -- 0.161
[01:25:31] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.553
[01:25:37] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 6.013
[01:25:40] [TIMING] Tidy -- 2.749
[01:25:48] [TIMING] Bootstrap -- 8.602
[01:25:49] [TIMING] Test { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.563
[01:25:50] [TIMING] Rustc { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.614
[01:25:50] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.573
[01:25:51] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.432
[01:25:51] [TIMING] Test { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.343
[01:25:51] [TIMING] Rustc { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.391
[01:25:52] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.399
[01:26:13] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "compiletest", path: "src/tools/compiletest", mode: Libtest, is_ext_tool: false, extra_features: [] } -- 21.438
[01:28:01] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "ui", suite: "ui" } -- 107.989
[01:46:59] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-pass", suite: "run-pass" } -- 1138.145
[01:51:35] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "compile-fail", suite: "compile-fail" } -- 275.421
[01:51:41] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "parse-fail", suite: "parse-fail" } -- 6.342
[01:52:05] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-fail", suite: "run-fail" } -- 24.049
[01:52:10] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-pass-valgrind", suite: "run-pass-valgrind" } -- 5.299
[01:52:39] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "mir-opt", suite: "mir-opt" } -- 28.143
[01:52:47] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "codegen", suite: "codegen" } -- 8.242
[01:52:51] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "codegen-units", suite: "codegen-units" } -- 4.561
[01:53:27] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "incremental", suite: "incremental" } -- 36.072
[01:53:47] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "debuginfo-gdb", suite: "debuginfo" } -- 19.985
[01:54:20] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "ui", suite: "ui-fulldeps" } -- 32.440
[02:04:15] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-pass", suite: "run-pass-fulldeps" } -- 595.359
[02:04:22] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-fail", suite: "run-fail-fulldeps" } -- 6.565
[02:06:31] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "compile-fail", suite: "compile-fail-fulldeps" } -- 129.629
[02:06:36] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "incremental", suite: "incremental-fulldeps" } -- 5.004
[02:06:37] [TIMING] Rustdoc { host: "x86_64-unknown-linux-gnu" } -- 0.367
[02:11:53] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "rustdoc", suite: "rustdoc" } -- 316.133
[02:12:01] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest, test_kind: Test, krate: "term" } -- 8.029
[02:12:21] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest, test_kind: Test, krate: "test" } -- 19.788
[02:17:25] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "alloc" } -- 304.094
[02:17:25] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "alloc_system" } -- 0.541
[02:26:36] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "core" } -- 550.298
[02:26:36] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "panic_abort" } -- 0.548
[02:34:52] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "std" } -- 495.874
[02:35:07] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "std_unicode" } -- 15.048
[02:35:08] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "unwind" } -- 0.506
[02:35:12] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "arena" } -- 4.688
[02:35:16] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "fmt_macros" } -- 3.466
[02:35:21] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "graphviz" } -- 5.013
[02:35:25] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "proc_macro" } -- 3.774
[02:43:47] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc" } -- 502.705
[02:43:48] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_allocator" } -- 0.392
[02:44:28] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_apfloat" } -- 40.105
[02:44:57] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_back" } -- 29.124
[02:44:57] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_borrowck" } -- 0.371
[02:45:01] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_const_math" } -- 3.180
[02:45:41] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_data_structures" } -- 40.342
[02:47:15] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_driver" } -- 93.595
[02:47:22] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_errors" } -- 7.718
[02:47:34] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_incremental" } -- 12.062
[02:47:35] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_lint" } -- 0.350
[02:47:55] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_metadata" } -- 20.675
[02:50:27] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_mir" } -- 151.767
[02:50:33] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_passes" } -- 5.492
[02:50:47] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_platform_intrinsics" } -- 14.652
[02:50:50] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_plugin" } -- 2.503
[02:50:55] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_privacy" } -- 4.753
[02:50:55] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_resolve" } -- 0.362
[02:51:05] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_save_analysis" } -- 10.618
[02:51:09] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_traits" } -- 4.002
[02:51:10] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_trans_utils" } -- 0.355
[02:51:10] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_typeck" } -- 0.357
[02:52:02] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "serialize" } -- 51.681
[02:54:45] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "syntax" } -- 163.207
[02:55:01] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "syntax_ext" } -- 16.216
[02:55:11] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "syntax_pos" } -- 9.192
[02:55:12] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc-main" } -- 1.786
[02:56:20] [TIMING] CrateRustdoc { host: "x86_64-unknown-linux-gnu", test_kind: Test } -- 67.556
[02:56:20] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 0.527
[02:56:21] [TIMING] UnstableBookGen { target: "x86_64-unknown-linux-gnu" } -- 0.534
[02:56:21] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 0.547
[02:56:24] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "unstable-book", src: "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc" } -- 2.678
[02:56:25] [TIMING] TheBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", name: "book" } -- 1.144
[02:56:26] [TIMING] Std { stage: 2, target: "x86_64-unknown-linux-gnu" } -- 1.133
[02:56:27] [TIMING] WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" } -- 0.645
[02:56:27] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 0.363
[02:56:28] [TIMING] ErrorIndex { target: "x86_64-unknown-linux-gnu" } -- 0.114
[02:56:28] [TIMING] CargoBook { target: "x86_64-unknown-linux-gnu", name: "cargo" } -- 0.155
[02:56:30] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "linkchecker", path: "src/tools/linkchecker", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 2.734
[02:56:52] [TIMING] Linkcheck { host: "x86_64-unknown-linux-gnu" } -- 22.048
[02:57:40] [TIMING] ErrorIndex { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 47.446
