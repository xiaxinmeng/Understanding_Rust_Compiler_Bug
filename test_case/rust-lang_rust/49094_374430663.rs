
[00:03:16] [TIMING] Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:03:16] [TIMING] StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:03:16] [TIMING] Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:03:43] [RUSTC-TIMING] core test:false 26.554
[00:03:45] [RUSTC-TIMING] libc test:false 1.718
[00:03:45] [RUSTC-TIMING] panic_abort test:false 0.228
[00:03:46] [RUSTC-TIMING] unwind test:false 0.284
[00:03:46] [RUSTC-TIMING] std_unicode test:false 2.357
[00:03:47] [RUSTC-TIMING] compiler_builtins test:false 3.071
[00:03:53] [RUSTC-TIMING] alloc test:false 6.890
[00:03:53] [RUSTC-TIMING] alloc_system test:false 0.339
[00:03:53] [RUSTC-TIMING] panic_unwind test:false 0.595
[00:03:54] [RUSTC-TIMING] alloc_jemalloc test:false 0.367
[00:03:54] [RUSTC-TIMING] rustc_asan test:false 0.411
[00:03:54] [RUSTC-TIMING] rustc_tsan test:false 0.439
[00:03:54] [RUSTC-TIMING] rustc_lsan test:false 0.396
[00:03:54] [RUSTC-TIMING] rustc_msan test:false 0.330
[00:04:11] [RUSTC-TIMING] std test:false 17.063
[00:04:11] [TIMING] Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:04:11] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:04:11] [TIMING] StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:04:11] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 54.841
[00:04:12] [RUSTC-TIMING] itoa test:false 0.710
[00:04:12] [RUSTC-TIMING] dtoa test:false 0.707
[00:04:14] [RUSTC-TIMING] num_traits test:false 2.148
[00:04:25] [RUSTC-TIMING] serde test:false 12.938
[00:04:37] [RUSTC-TIMING] serde_json test:false 9.807
[00:05:08] [RUSTC-TIMING] tidy test:false 8.461
[00:05:09] [RUSTC-TIMING] tidy test:false 0.794
[00:05:09] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 57.466
[00:05:09] [TIMING] Tidy { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:05:10] [TIMING] Tidy -- 1.886
[00:05:12] [TIMING] Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:05:12] [TIMING] StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:05:12] [TIMING] Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:05:13] [TIMING] Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:05:13] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:05:13] [TIMING] StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:05:13] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.598
[00:05:16] [RUSTC-TIMING] getopts test:false 3.282
[00:05:18] [RUSTC-TIMING] term test:false 4.848
[00:05:26] [RUSTC-TIMING] test test:false 7.534
[00:05:26] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.008
[00:05:26] [TIMING] TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:05:26] [TIMING] Test { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 13.062
[00:05:27] [RUSTC-TIMING] scoped_tls test:false 0.543
[00:05:27] [RUSTC-TIMING] bitflags test:false 0.442
[00:05:28] [RUSTC-TIMING] ar test:false 0.834
[00:05:28] [RUSTC-TIMING] stable_deref_trait test:false 0.360
[00:05:30] [RUSTC-TIMING] lazy_static test:false 0.367
[00:05:35] [RUSTC-TIMING] termcolor test:false 4.206
[00:05:35] [RUSTC-TIMING] cfg_if test:false 0.211
[00:05:40] [RUSTC-TIMING] rustc_platform_intrinsics test:false 13.364
[00:05:40] [RUSTC-TIMING] unicode_width test:false 0.372
[00:05:42] [RUSTC-TIMING] libc test:false 1.926
[00:05:42] [RUSTC-TIMING] smallvec test:false 0.902
[00:05:43] [RUSTC-TIMING] rustc_serialize test:false 12.727
[00:05:45] [RUSTC-TIMING] graphviz test:false 2.069
[00:05:45] [RUSTC-TIMING] rustc_demangle test:false 3.232
[00:05:45] [RUSTC-TIMING] fmt_macros test:false 1.491
[00:05:45] [RUSTC-TIMING] lazy_static test:false 0.376
[00:05:45] [RUSTC-TIMING] quick_error test:false 0.419
[00:05:46] [RUSTC-TIMING] rustc_cratesio_shim test:false 0.395
[00:05:46] [RUSTC-TIMING] arena test:false 0.734
[00:05:46] [RUSTC-TIMING] byteorder test:false 1.314
[00:05:46] [RUSTC-TIMING] owning_ref test:false 0.595
[00:05:47] [RUSTC-TIMING] log test:false 1.242
[00:05:48] [RUSTC-TIMING] atty test:false 0.421
[00:05:49] [RUSTC-TIMING] rls_span test:false 1.613
[00:05:51] [RUSTC-TIMING] log_settings test:false 0.794
[00:05:52] [RUSTC-TIMING] humantime test:false 2.020
[00:05:53] [RUSTC-TIMING] rand test:false 5.086
[00:05:53] [RUSTC-TIMING] serialize test:false 10.754
[00:05:54] [RUSTC-TIMING] ena test:false 0.764
[00:05:56] [RUSTC-TIMING] rustc_apfloat test:false 3.262
[00:05:57] [RUSTC-TIMING] rls_data test:false 3.145
[00:05:59] [RUSTC-TIMING] jobserver test:false 6.304
[00:06:00] [RUSTC-TIMING] parking_lot_core test:false 2.996
[00:06:00] [RUSTC-TIMING] env_logger test:false 4.058
[00:06:01] [RUSTC-TIMING] backtrace_sys test:false 0.318
[00:06:01] [RUSTC-TIMING] miniz_sys test:false 0.377
[00:06:02] [RUSTC-TIMING] backtrace test:false 1.580
[00:06:02] [RUSTC-TIMING] parking_lot test:false 2.014
[00:06:03] [RUSTC-TIMING] flate2 test:false 2.084
[00:06:05] [RUSTC-TIMING] rustc_data_structures test:false 2.923
[00:06:09] [RUSTC-TIMING] syntax_pos test:false 3.768
[00:06:18] [RUSTC-TIMING] rustc_errors test:false 9.182
[00:07:49] [RUSTC-TIMING] syntax test:false 90.981
[00:07:57] [RUSTC-TIMING] rustc_const_math test:false 7.616
[00:07:58] [RUSTC-TIMING] proc_macro test:false 8.996
[00:08:06] [RUSTC-TIMING] rustc_back test:false 16.989
[00:08:33] [RUSTC-TIMING] syntax_ext test:false 34.992
[00:13:50] [RUSTC-TIMING] rustc test:false 339.541
[00:14:38] [RUSTC-TIMING] rustc_allocator test:false 47.893
[00:15:11] [RUSTC-TIMING] rustc_incremental test:false 80.968
[00:15:33] [RUSTC-TIMING] rustc_traits test:false 22.194
[00:16:07] [RUSTC-TIMING] rustc_resolve test:false 137.419
[00:17:12] [RUSTC-TIMING] rustc_metadata test:false 202.262
[00:17:37] [RUSTC-TIMING] rustc_plugin test:false 15.078
[00:18:03] [RUSTC-TIMING] rustc_typeck test:false 173.242
[00:18:26] [RUSTC-TIMING] rustc_privacy test:false 22.709
[00:19:10] [RUSTC-TIMING] rustc_save_analysis test:false 66.576
[00:19:13] [RUSTC-TIMING] rustc_mir test:false 220.013
[00:19:39] [RUSTC-TIMING] rustc_trans_utils test:false 25.983
[00:19:44] [RUSTC-TIMING] rustc_lint test:false 30.867
[00:19:47] [RUSTC-TIMING] rustc_passes test:false 33.696
[00:19:47] [RUSTC-TIMING] rustc_borrowck test:false 34.026
[00:20:42] [RUSTC-TIMING] rustc_driver test:false 54.998
[00:20:43] [RUSTC-TIMING] rustc test:false 1.329
[00:20:43] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[00:20:43] [TIMING] RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.001
[00:20:43] [TIMING] Rustc { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 917.774
[TIMING] Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false } -- 0.010
[00:20:45] [RUSTC-TIMING] winapi test:false 0.960
[00:20:48] [RUSTC-TIMING] num_cpus test:false 2.623
[00:20:49] [RUSTC-TIMING] kernel32 test:false 0.390
[00:20:50] [RUSTC-TIMING] remove_dir_all test:false 0.422
[00:20:52] [RUSTC-TIMING] tempdir test:false 1.054
[00:20:54] [RUSTC-TIMING] cc test:false 9.443
[00:20:58] [RUSTC-TIMING] rustc_llvm test:false 1.172
[00:21:51] [RUSTC-TIMING] rustc_trans test:false 52.994
[00:21:51] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 67.862
[00:21:51] [TIMING] Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:21:51] [TIMING] Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:21:51] [TIMING] Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.001
[00:21:51] [TIMING] StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:22:58] [RUSTC-TIMING] core test:false 65.825
[00:23:00] [RUSTC-TIMING] libc test:false 1.964
[00:23:00] [RUSTC-TIMING] panic_abort test:false 0.117
[00:23:00] [RUSTC-TIMING] unwind test:false 0.175
[00:23:00] [RUSTC-TIMING] std_unicode test:false 2.336
[00:23:01] [RUSTC-TIMING] compiler_builtins test:false 2.682
[00:23:09] [RUSTC-TIMING] alloc test:false 8.687
[00:23:09] [RUSTC-TIMING] alloc_system test:false 0.199
[00:23:10] [RUSTC-TIMING] panic_unwind test:false 0.420
[00:23:10] [RUSTC-TIMING] alloc_jemalloc test:false 0.227
[00:23:10] [RUSTC-TIMING] rustc_asan test:false 0.264
[00:23:10] [RUSTC-TIMING] rustc_tsan test:false 0.272
[00:23:10] [RUSTC-TIMING] rustc_msan test:false 0.183
[00:23:10] [RUSTC-TIMING] rustc_lsan test:false 0.201
[00:23:28] [RUSTC-TIMING] std test:false 18.145
[00:23:28] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:23:28] [TIMING] StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:23:28] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 96.708
[00:23:30] [RUSTC-TIMING] getopts test:false 1.656
[00:23:31] [RUSTC-TIMING] term test:false 3.089
[00:23:37] [RUSTC-TIMING] test test:false 5.315
[00:23:37] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.000
[00:23:37] [TIMING] TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:23:37] [TIMING] Test { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 8.763
[00:23:37] [RUSTC-TIMING] cfg_if test:false 0.084
[00:23:37] [RUSTC-TIMING] unicode_width test:false 0.218
[00:23:38] [RUSTC-TIMING] smallvec test:false 1.024
[00:23:38] [RUSTC-TIMING] rustc_demangle test:false 1.029
[00:23:39] [RUSTC-TIMING] byteorder test:false 1.559
[00:23:39] [RUSTC-TIMING] scoped_tls test:false 0.301
[00:23:41] [RUSTC-TIMING] termcolor test:false 2.616
[00:23:41] [RUSTC-TIMING] fmt_macros test:false 1.251
[00:23:43] [RUSTC-TIMING] graphviz test:false 1.532
[00:23:43] [RUSTC-TIMING] quick_error test:false 0.235
[00:23:44] [RUSTC-TIMING] arena test:false 0.522
[00:23:45] [RUSTC-TIMING] bitflags test:false 0.338
[00:23:49] [RUSTC-TIMING] stable_deref_trait test:false 0.156
[00:23:50] [RUSTC-TIMING] ar test:false 0.727
[00:23:52] [RUSTC-TIMING] lazy_static test:false 0.212
[00:23:52] [RUSTC-TIMING] lazy_static test:false 0.222
[00:23:52] [RUSTC-TIMING] serialize test:false 10.600
[00:23:53] [RUSTC-TIMING] rustc_serialize test:false 14.521
[00:23:54] [RUSTC-TIMING] log test:false 1.132
[00:23:54] [RUSTC-TIMING] rustc_cratesio_shim test:false 0.150
[00:23:54] [RUSTC-TIMING] libc test:false 2.315
[00:23:54] [RUSTC-TIMING] humantime test:false 1.599
[00:23:55] [RUSTC-TIMING] owning_ref test:false 0.491
[00:23:56] [RUSTC-TIMING] log_settings test:false 0.482
[00:23:57] [RUSTC-TIMING] ena test:false 0.771
[00:23:57] [RUSTC-TIMING] rls_span test:false 1.153
[00:23:58] [RUSTC-TIMING] rustc_platform_intrinsics test:false 13.608
[00:23:59] [RUSTC-TIMING] jobserver test:false 2.216
[00:23:59] [RUSTC-TIMING] atty test:false 0.230
[00:24:00] [RUSTC-TIMING] rustc_apfloat test:false 3.351
[00:24:02] [RUSTC-TIMING] rls_data test:false 2.600
[00:24:03] [RUSTC-TIMING] env_logger test:false 2.928
[00:24:03] [RUSTC-TIMING] miniz_sys test:false 0.180
[00:24:03] [RUSTC-TIMING] rand test:false 4.196
[00:24:05] [RUSTC-TIMING] parking_lot_core test:false 1.426
[00:24:06] [RUSTC-TIMING] flate2 test:false 2.260
[00:24:06] [RUSTC-TIMING] parking_lot test:false 1.642
[00:24:06] [RUSTC-TIMING] backtrace_sys test:false 0.139
[00:24:07] [RUSTC-TIMING] backtrace test:false 1.060
[00:24:10] [RUSTC-TIMING] rustc_data_structures test:false 3.809
[00:24:14] [RUSTC-TIMING] syntax_pos test:false 3.588
[00:24:21] [RUSTC-TIMING] rustc_errors test:false 6.972
[00:25:30] [RUSTC-TIMING] syntax test:false 69.035
[00:25:34] [RUSTC-TIMING] rustc_const_math test:false 3.732
[00:25:36] [RUSTC-TIMING] proc_macro test:false 6.224
[00:25:42] [RUSTC-TIMING] rustc_back test:false 12.089
[00:25:57] [RUSTC-TIMING] syntax_ext test:false 20.908
[00:30:40] [RUSTC-TIMING] rustc test:false 297.873
[00:31:04] [RUSTC-TIMING] rustc_allocator test:false 24.232
[00:31:25] [RUSTC-TIMING] rustc_incremental test:false 45.137
[00:31:59] [RUSTC-TIMING] rustc_metadata test:false 78.753
[00:32:10] [RUSTC-TIMING] rustc_typeck test:false 89.692
[00:32:30] [RUSTC-TIMING] rustc_traits test:false 11.345
[00:32:31] [RUSTC-TIMING] rustc_resolve test:false 32.616
[00:32:34] [RUSTC-TIMING] rustc_plugin test:false 4.683
[00:32:38] [RUSTC-TIMING] rustc_privacy test:false 7.873
[00:32:52] [RUSTC-TIMING] rustc_save_analysis test:false 17.948
[00:34:19] [RUSTC-TIMING] rustc_mir test:false 109.484
[00:34:33] [RUSTC-TIMING] rustc_trans_utils test:false 14.153
[00:34:38] [RUSTC-TIMING] rustc_borrowck test:false 18.354
[00:34:38] [RUSTC-TIMING] rustc_passes test:false 18.495
[00:34:38] [RUSTC-TIMING] rustc_lint test:false 19.151
[00:35:12] [RUSTC-TIMING] rustc_driver test:false 33.346
[00:35:12] [RUSTC-TIMING] rustc test:false 0.731
[00:35:12] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[00:35:12] [TIMING] RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.001
[00:35:12] [TIMING] Rustc { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 695.693
[00:35:14] [RUSTC-TIMING] winapi test:false 0.796
[00:35:15] [RUSTC-TIMING] num_cpus test:false 1.455
[00:35:16] [RUSTC-TIMING] kernel32 test:false 0.135
[00:35:17] [RUSTC-TIMING] remove_dir_all test:false 0.112
[00:35:18] [RUSTC-TIMING] tempdir test:false 0.663
[00:35:18] [RUSTC-TIMING] cc test:false 5.596
[00:35:24] [RUSTC-TIMING] rustc_llvm test:false 0.965
[00:36:02] [RUSTC-TIMING] rustc_trans test:false 38.147
[00:36:02] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 49.256
[00:36:02] [TIMING] Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:36:02] [TIMING] Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:36:02] [TIMING] Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.001
[00:36:02] [TIMING] StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:36:02] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:36:02] [TIMING] StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:36:02] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:36:02] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.000
[00:36:02] [TIMING] TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:36:02] [TIMING] Test { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:36:02] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[00:36:02] [TIMING] RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.001
[00:36:02] [TIMING] Rustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:36:03] [RUSTC-TIMING] bitflags test:false 0.597
[00:36:03] [RUSTC-TIMING] winapi test:false 1.056
[00:36:04] [RUSTC-TIMING] kernel32 test:false 0.187
[00:36:04] [RUSTC-TIMING] remove_dir_all test:false 0.143
[00:36:06] [RUSTC-TIMING] libc test:false 2.989
[00:36:09] [RUSTC-TIMING] pulldown_cmark test:false 6.880
[00:36:11] [RUSTC-TIMING] rand test:false 5.781
[00:36:12] [RUSTC-TIMING] tempdir test:false 0.742
[00:37:45] [RUSTC-TIMING] rustdoc test:false 92.840
[00:37:47] [RUSTC-TIMING] rustdoc_tool_binary test:false 1.790
[00:37:47] [TIMING] Rustdoc { host: "x86_64-unknown-linux-gnu" } -- 105.078
[00:37:48] [TIMING] Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:37:48] [TIMING] StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:48] [TIMING] Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.001
[00:37:49] [TIMING] Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:49] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:37:49] [TIMING] StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:49] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.678
[00:37:50] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.000
[00:37:50] [TIMING] TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:50] [TIMING] Test { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.607
[00:37:50] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[00:37:50] [TIMING] RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.001
[00:37:50] [TIMING] Rustc { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.608
[TIMING] Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false } -- 0.007
[00:37:51] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.639
[00:37:51] [TIMING] Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.001
[00:37:51] [TIMING] Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:51] [TIMING] Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:37:51] [TIMING] StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:51] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:37:51] [TIMING] StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:51] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.419
[00:37:52] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.000
[00:37:52] [TIMING] TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:52] [TIMING] Test { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.316
[00:37:52] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[00:37:52] [TIMING] RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.001
[00:37:52] [TIMING] Rustc { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.362
[00:37:52] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.364
[00:37:52] [TIMING] Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:37:52] [TIMING] Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:52] [TIMING] Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.001
[00:37:52] [TIMING] StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:52] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:37:52] [TIMING] StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:37:52] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:37:54] [RUSTC-TIMING] dtoa test:false 0.696
[00:37:54] [RUSTC-TIMING] itoa test:false 0.757
[00:37:55] [RUSTC-TIMING] num_traits test:false 2.382
[00:38:05] [RUSTC-TIMING] serde test:false 12.041
[00:38:17] [RUSTC-TIMING] serde_json test:false 11.868
[00:38:50] [RUSTC-TIMING] tidy test:false 8.475
[00:38:52] [RUSTC-TIMING] unstable_book_gen test:false 2.550
[00:38:52] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 59.831
[00:38:52] [TIMING] UnstableBookGen { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:38:53] [TIMING] UnstableBookGen { target: "x86_64-unknown-linux-gnu" } -- 0.341
[00:38:53] [RUSTC-TIMING] cfg_if test:false 0.219
[00:38:55] [RUSTC-TIMING] either test:false 0.476
[00:38:55] [RUSTC-TIMING] shlex test:false 1.419
[00:38:55] [RUSTC-TIMING] is_match test:false 0.412
[00:38:55] [RUSTC-TIMING] libc test:false 1.997
[00:38:56] [RUSTC-TIMING] quick_error test:false 0.511
[00:38:56] [RUSTC-TIMING] vec_map test:false 0.717
[00:38:56] [RUSTC-TIMING] rustc_demangle test:false 3.296
[00:38:57] [RUSTC-TIMING] bitflags test:false 0.427
[00:38:57] [RUSTC-TIMING] ansi_term test:false 1.676
[00:38:58] [RUSTC-TIMING] winapi test:false 0.898
[00:39:00] [RUSTC-TIMING] num_traits test:false 2.840
[00:39:00] [RUSTC-TIMING] termcolor test:false 4.395
[00:39:01] [RUSTC-TIMING] bitflags test:false 0.422
[00:39:01] [RUSTC-TIMING] pest test:false 0.575
[00:39:01] [RUSTC-TIMING] lazy_static test:false 0.444
[00:39:02] [RUSTC-TIMING] getopts test:false 4.081
[00:39:03] [RUSTC-TIMING] lazy_static test:false 0.488
[00:39:03] [RUSTC-TIMING] utf8_ranges test:false 1.192
[00:39:04] [RUSTC-TIMING] strsim test:false 2.716
[00:39:04] [RUSTC-TIMING] void test:false 0.268
[00:39:04] [RUSTC-TIMING] open test:false 0.371
[00:39:05] [RUSTC-TIMING] ucd_util test:false 1.424
[00:39:06] [RUSTC-TIMING] unicode_width test:false 0.290
[00:39:07] [RUSTC-TIMING] log test:false 1.348
[00:39:08] [RUSTC-TIMING] memchr test:false 0.975
[00:39:08] [RUSTC-TIMING] atty test:false 0.478
[00:39:08] [RUSTC-TIMING] itertools test:false 2.587
[00:39:10] [RUSTC-TIMING] humantime test:false 2.083
[00:39:12] [RUSTC-TIMING] rand test:false 5.760
[00:39:12] [RUSTC-TIMING] time test:false 4.116
[00:39:12] [RUSTC-TIMING] unreachable test:false 0.221
[00:39:13] [RUSTC-TIMING] num_integer test:false 0.698
[00:39:26] [RUSTC-TIMING] serde_json test:false 13.877
[00:39:28] [RUSTC-TIMING] pulldown_cmark test:false 15.556
[00:39:29] [RUSTC-TIMING] toml test:false 16.673
[00:39:32] [RUSTC-TIMING] textwrap test:false 2.593
[00:39:32] [RUSTC-TIMING] log test:false 1.272
[00:39:32] [RUSTC-TIMING] aho_corasick test:false 1.660
[00:39:37] [RUSTC-TIMING] num_iter test:false 0.469
[00:39:38] [RUSTC-TIMING] regex_syntax test:false 25.153
[00:39:38] [RUSTC-TIMING] thread_local test:false 1.445
[00:39:38] [RUSTC-TIMING] kernel32 test:false 0.404
[00:39:39] [RUSTC-TIMING] num test:false 0.401
[00:39:39] [RUSTC-TIMING] remove_dir_all test:false 0.327
[00:39:41] [RUSTC-TIMING] tempdir test:false 1.503
[00:39:47] [RUSTC-TIMING] chrono test:false 7.761
[00:40:06] [RUSTC-TIMING] backtrace_sys test:false 0.397
[00:40:09] [RUSTC-TIMING] backtrace test:false 2.705
[00:40:15] [RUSTC-TIMING] regex test:false 35.892
[00:40:21] [RUSTC-TIMING] clap test:false 43.240
[00:40:22] [RUSTC-TIMING] error_chain test:false 1.982
[00:40:26] [RUSTC-TIMING] env_logger test:false 5.574
[00:40:30] [RUSTC-TIMING] toml_query test:false 6.369
[00:40:37] [RUSTC-TIMING] handlebars test:false 15.396
[00:41:05] [RUSTC-TIMING] mdbook test:false 28.242
[00:41:09] [RUSTC-TIMING] rustbook test:false 3.771
[00:41:09] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 136.132
[00:41:09] [TIMING] Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:41:10] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "unstable-book", src: "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc" } -- 1.380
[00:41:10] [TIMING] UnstableBook { target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:41:10] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/first-edition", src: "/checkout/src/doc" } -- 0.253
[00:41:10] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/first-edition" } -- 0.000
[00:41:11] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/second-edition", src: "/checkout/src/doc" } -- 0.509
[00:41:11] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/second-edition" } -- 0.000
[00:41:11] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.000
[00:41:11] [TIMING] TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:41:11] [TIMING] Test { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:41:11] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[00:41:11] [TIMING] RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.002
[00:41:11] [TIMING] Rustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:41:11] [TIMING] Rustdoc { host: "x86_64-unknown-linux-gnu" } -- 0.343
[00:41:12] [TIMING] Standalone { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.528
[00:41:13] [TIMING] TheBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", name: "book" } -- 1.357
[00:42:19] [RUSTC-TIMING] core test:false 65.687
[00:42:21] [RUSTC-TIMING] std_unicode test:false 1.260
[00:42:21] [RUSTC-TIMING] compiler_builtins test:false 2.119
[00:42:22] [RUSTC-TIMING] libc test:false 2.393
[00:42:22] [RUSTC-TIMING] unwind test:false 0.241
[00:42:22] [RUSTC-TIMING] panic_abort test:false 0.143
[00:42:31] [RUSTC-TIMING] alloc test:false 10.370
[00:42:31] [RUSTC-TIMING] alloc_system test:false 0.192
[00:42:31] [RUSTC-TIMING] rustc_asan test:false 0.192
[00:42:31] [RUSTC-TIMING] rustc_tsan test:false 0.192
[00:42:31] [RUSTC-TIMING] panic_unwind test:false 0.453
[00:42:31] [RUSTC-TIMING] rustc_msan test:false 0.192
[00:42:32] [RUSTC-TIMING] alloc_jemalloc test:false 0.204
[00:42:32] [RUSTC-TIMING] rustc_lsan test:false 0.180
[00:42:54] [TIMING] Std { stage: 2, target: "x86_64-unknown-linux-gnu" } -- 100.691
[00:42:54] [RUSTC-TIMING] unicode_width test:false 0.124
[00:42:54] [RUSTC-TIMING] bitflags test:false 0.182
[00:42:54] [RUSTC-TIMING] cfg_if test:false 0.057
[00:42:54] [RUSTC-TIMING] stable_deref_trait test:false 0.099
[00:42:55] [RUSTC-TIMING] scoped_tls test:false 0.133
[00:42:55] [RUSTC-TIMING] smallvec test:false 0.644
[00:42:55] [RUSTC-TIMING] termcolor test:false 0.514
[00:42:55] [RUSTC-TIMING] rustc_cratesio_shim test:false 0.073
[00:42:55] [RUSTC-TIMING] owning_ref test:false 0.281
[00:42:55] [RUSTC-TIMING] log test:false 0.359
[00:42:56] [RUSTC-TIMING] ena test:false 0.486
[00:42:56] [RUSTC-TIMING] libc test:false 1.278
[00:42:56] [RUSTC-TIMING] atty test:false 0.125
[00:42:58] [RUSTC-TIMING] rand test:false 1.741
[00:42:58] [RUSTC-TIMING] parking_lot_core test:false 0.459
[00:42:58] [RUSTC-TIMING] serialize test:false 4.061
[00:42:59] [RUSTC-TIMING] parking_lot test:false 0.692
[00:43:02] [RUSTC-TIMING] rustc_data_structures test:false 3.196
[00:43:04] [RUSTC-TIMING] syntax_pos test:false 1.789
[00:43:06] [RUSTC-TIMING] rustc_errors test:false 2.059
[00:43:30] [RUSTC-TIMING] syntax test:false 24.041
[00:43:33] [TIMING] Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" } -- 39.592
[00:43:37] [RUSTC-TIMING] error_index_generator test:false 3.747
[00:43:37] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 4.060
[00:43:37] [TIMING] ErrorIndex { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:37] [TIMING] ErrorIndex { target: "x86_64-unknown-linux-gnu" } -- 0.099
[00:43:38] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "nomicon", src: "/checkout/src/doc" } -- 0.158
[00:43:38] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "nomicon" } -- 0.000
[00:43:38] [TIMING] Nomicon { target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:38] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "reference", src: "/checkout/src/doc" } -- 0.210
[00:43:38] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "reference" } -- 0.000
[00:43:38] [TIMING] Reference { target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:38] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rustdoc", src: "/checkout/src/doc" } -- 0.021
[00:43:38] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "rustdoc" } -- 0.000
[00:43:38] [TIMING] Rustdoc { target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:39] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rust-by-example", src: "/checkout/src/doc" } -- 0.763
[00:43:39] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "rust-by-example" } -- 0.000
[00:43:39] [TIMING] RustByExample { target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:39] [TIMING] CargoBook { target: "x86_64-unknown-linux-gnu", name: "cargo" } -- 0.071
[00:43:40] [TIMING] Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:43:40] [TIMING] StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:40] [TIMING] Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.001
[00:43:41] [TIMING] Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:41] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:43:41] [TIMING] StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:41] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.554
[00:43:42] [RUSTC-TIMING] tidy test:false 0.748
[00:43:42] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 1.305
[00:43:42] [TIMING] Tidy { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:43:44] [TIMING] Tidy -- 1.882
[00:44:13] [TIMING] Bootstrap -- 29.312
[00:44:14] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.000
[00:44:14] [TIMING] TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:14] [TIMING] Test { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.542
[00:44:14] [TIMING] CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[00:44:14] [TIMING] RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.003
[00:44:14] [TIMING] Rustc { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.579
[TIMING] Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false } -- 0.008
[00:44:15] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.582
[00:44:15] [TIMING] Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.003
[00:44:15] [TIMING] Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:15] [TIMING] Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.001
[00:44:15] [TIMING] StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:15] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:44:15] [TIMING] StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:15] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.387
[00:44:16] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.000
[00:44:16] [TIMING] TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:16] [TIMING] Test { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.280
[00:44:16] [TIMING] CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[00:44:16] [TIMING] RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.003
[00:44:16] [TIMING] Rustc { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.314
[00:44:16] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.322
[00:44:16] [TIMING] Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.002
[00:44:16] [TIMING] Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:16] [TIMING] Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.001
[00:44:16] [TIMING] StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:16] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd } -- 0.000
[00:44:16] [TIMING] StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:16] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[00:44:16] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest } -- 0.000
[00:44:16] [TIMING] TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:16] [TIMING] Test { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:16] [TIMING] TestHelpers { target: "x86_64-unknown-linux-gnu" } -- 0.056
[00:44:16] [TIMING] RemoteCopyLibs { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:44:17] [RUSTC-TIMING] filetime test:false 0.585
[00:44:19] [RUSTC-TIMING] env_logger test:false 1.898
[00:44:19] [RUSTC-TIMING] diff test:false 2.123
[00:44:35] [RUSTC-TIMING] compiletest test:false 16.075
[00:44:35] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "compiletest", path: "src/tools/compiletest", mode: Libtest, is_ext_tool: false, extra_features: [] } -- 18.819
[00:44:35] [TIMING] Compiletest { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:45:52] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "ui", suite: "ui" } -- 76.752
[00:45:52] [TIMING] Ui { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[00:58:09] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-pass", suite: "run-pass" } -- 737.388
[00:58:09] [TIMING] RunPass { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:01:51] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "compile-fail", suite: "compile-fail" } -- 222.155
[01:01:51] [TIMING] CompileFail { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:01:56] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "parse-fail", suite: "parse-fail" } -- 4.934
[01:01:56] [TIMING] ParseFail { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:02:13] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-fail", suite: "run-fail" } -- 16.373
[01:02:13] [TIMING] RunFail { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:02:16] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-pass-valgrind", suite: "run-pass-valgrind" } -- 3.727
[01:02:16] [TIMING] RunPassValgrind { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:02:39] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "mir-opt", suite: "mir-opt" } -- 22.934
[01:02:39] [TIMING] MirOpt { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:02:44] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "codegen", suite: "codegen" } -- 4.607
[01:02:44] [TIMING] Codegen { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:02:47] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "codegen-units", suite: "codegen-units" } -- 3.424
[01:02:47] [TIMING] CodegenUnits { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:03:13] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "incremental", suite: "incremental" } -- 25.543
[01:03:13] [TIMING] Incremental { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:03:13] [TIMING] DebuggerScripts { sysroot: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2", host: "x86_64-unknown-linux-gnu" } -- 0.008
[01:03:30] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "debuginfo-gdb", suite: "debuginfo" } -- 17.043
[01:03:30] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "debuginfo-XXX", suite: "debuginfo" } -- 0.000
[01:03:30] [TIMING] Debuginfo { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:03:30] [TIMING] CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc } -- 0.000
[01:03:30] [TIMING] RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.006
[01:03:30] [TIMING] Rustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:03:42] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "ui", suite: "ui-fulldeps" } -- 12.059
[01:03:42] [TIMING] UiFullDeps { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:08:33] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-pass", suite: "run-pass-fulldeps" } -- 290.944
[01:08:33] [TIMING] RunPassFullDeps { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:08:35] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-fail", suite: "run-fail-fulldeps" } -- 2.485
[01:08:35] [TIMING] RunFailFullDeps { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:09:16] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "compile-fail", suite: "compile-fail-fulldeps" } -- 40.790
[01:09:16] [TIMING] CompileFailFullDeps { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:09:19] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "incremental", suite: "incremental-fulldeps" } -- 2.985
[01:09:19] [TIMING] IncrementalFullDeps { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:09:20] [TIMING] Rustdoc { host: "x86_64-unknown-linux-gnu" } -- 0.332
[01:13:10] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "rustdoc", suite: "rustdoc" } -- 230.676
[01:13:10] [TIMING] Rustdoc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:14:53] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "run-make", suite: "run-make" } -- 102.865
[01:14:53] [TIMING] RunMake { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:14:55] [RUSTC-TIMING] term test:true 1.867
[01:14:56] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest, test_kind: Test, krate: "term" } -- 2.651
[01:15:02] [RUSTC-TIMING] test test:true 6.016
[01:15:05] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest, test_kind: Test, krate: "test" } -- 9.754
[01:15:07] [RUSTC-TIMING] libc test:false 1.359
[01:15:10] [RUSTC-TIMING] rand test:false 2.506
[01:15:23] [RUSTC-TIMING] alloc test:true 12.898
[01:15:46] [RUSTC-TIMING] collectionstests test:true 36.236
[01:17:53] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "alloc" } -- 167.925
[01:17:54] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "alloc_system" } -- 0.590
[01:19:04] [RUSTC-TIMING] coretests test:true 69.256
[01:23:05] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "core" } -- 311.357
[01:23:06] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "panic_abort" } -- 0.593
[01:23:07] [RUSTC-TIMING] env test:true 0.778
[01:24:21] [RUSTC-TIMING] std test:true 74.213
[01:27:55] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "std" } -- 288.794
[01:27:56] [RUSTC-TIMING] std_unicode_tests test:true 0.891
[01:28:05] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "std_unicode" } -- 10.668
[01:28:06] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd, test_kind: Test, krate: "unwind" } -- 0.516
[01:28:07] [RUSTC-TIMING] arena test:true 1.172
[01:28:07] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "arena" } -- 1.530
[01:28:07] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "arena" } -- 0.000
[01:28:09] [RUSTC-TIMING] fmt_macros test:true 0.854
[01:28:09] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "fmt_macros" } -- 1.183
[01:28:09] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "fmt_macros" } -- 0.000
[01:28:10] [RUSTC-TIMING] graphviz test:true 1.512
[01:28:11] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "graphviz" } -- 1.850
[01:28:11] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "graphviz" } -- 0.000
[01:28:12] [RUSTC-TIMING] proc_macro test:true 0.939
[01:28:12] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "proc_macro" } -- 1.272
[01:28:12] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "proc_macro" } -- 0.000
[01:30:13] [RUSTC-TIMING] rustc test:true 120.561
[01:30:13] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc" } -- 120.976
[01:30:13] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc" } -- 0.000
[01:30:13] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_allocator" } -- 0.301
[01:30:13] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_allocator" } -- 0.000
[01:30:16] [RUSTC-TIMING] rustc_apfloat test:true 2.674
[01:30:16] [RUSTC-TIMING] ppc test:true 2.788
[01:30:25] [RUSTC-TIMING] ieee test:true 11.672
[01:30:25] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_apfloat" } -- 12.328
[01:30:25] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_apfloat" } -- 0.000
[01:30:33] [RUSTC-TIMING] rustc_back test:true 7.703
[01:30:33] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_back" } -- 8.052
[01:30:33] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_back" } -- 0.000
[01:30:34] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_borrowck" } -- 0.298
[01:30:34] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_borrowck" } -- 0.000
[01:30:35] [RUSTC-TIMING] rustc_const_math test:true 0.595
[01:30:35] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_const_math" } -- 0.905
[01:30:35] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_const_math" } -- 0.000
[01:30:46] [RUSTC-TIMING] rustc_data_structures test:true 10.930
[01:30:46] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_data_structures" } -- 11.274
[01:30:46] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_data_structures" } -- 0.000
[01:31:17] [RUSTC-TIMING] rustc_driver test:true 30.537
[01:31:17] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_driver" } -- 30.901
[01:31:17] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_driver" } -- 0.000
[01:31:19] [RUSTC-TIMING] rustc_errors test:true 1.666
[01:31:19] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_errors" } -- 1.974
[01:31:19] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_errors" } -- 0.000
[01:31:23] [RUSTC-TIMING] rustc_incremental test:true 3.619
[01:31:23] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_incremental" } -- 3.965
[01:31:23] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_incremental" } -- 0.000
[01:31:23] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_lint" } -- 0.306
[01:31:23] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_lint" } -- 0.000
[01:31:29] [RUSTC-TIMING] rustc_metadata test:true 5.187
[01:31:29] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_metadata" } -- 5.553
[01:31:29] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_metadata" } -- 0.000
[01:32:07] [RUSTC-TIMING] rustc_mir test:true 38.331
[01:32:07] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_mir" } -- 38.713
[01:32:07] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_mir" } -- 0.000
[01:32:09] [RUSTC-TIMING] rustc_passes test:true 1.442
[01:32:09] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_passes" } -- 1.808
[01:32:09] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_passes" } -- 0.000
[01:32:13] [RUSTC-TIMING] rustc_platform_intrinsics test:true 3.648
[01:32:13] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_platform_intrinsics" } -- 3.979
[01:32:13] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_platform_intrinsics" } -- 0.000
[01:32:14] [RUSTC-TIMING] rustc_plugin test:true 0.728
[01:32:14] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_plugin" } -- 1.084
[01:32:14] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_plugin" } -- 0.000
[01:32:16] [RUSTC-TIMING] rustc_privacy test:true 1.388
[01:32:16] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_privacy" } -- 1.735
[01:32:16] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_privacy" } -- 0.000
[01:32:16] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_resolve" } -- 0.328
[01:32:16] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_resolve" } -- 0.000
[01:32:19] [RUSTC-TIMING] rustc_save_analysis test:true 2.542
[01:32:19] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_save_analysis" } -- 2.908
[01:32:19] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_save_analysis" } -- 0.000
[01:32:21] [RUSTC-TIMING] rustc_traits test:true 1.138
[01:32:21] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_traits" } -- 1.491
[01:32:21] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_traits" } -- 0.000
[01:32:21] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_trans_utils" } -- 0.348
[01:32:21] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_trans_utils" } -- 0.000
[01:32:21] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc_typeck" } -- 0.351
[01:32:21] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc_typeck" } -- 0.000
[01:32:36] [RUSTC-TIMING] serialize test:true 14.480
[01:32:39] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "serialize" } -- 17.264
[01:32:39] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "serialize" } -- 0.000
[01:33:30] [RUSTC-TIMING] syntax test:true 51.342
[01:33:31] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "syntax" } -- 52.206
[01:33:31] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "syntax" } -- 0.000
[01:33:36] [RUSTC-TIMING] syntax_ext test:true 4.839
[01:33:36] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "syntax_ext" } -- 5.153
[01:33:36] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "syntax_ext" } -- 0.000
[01:33:39] [RUSTC-TIMING] syntax_pos test:true 2.295
[01:33:39] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "syntax_pos" } -- 2.664
[01:33:39] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "syntax_pos" } -- 0.000
[01:33:40] [RUSTC-TIMING] rustc test:true 1.058
[01:33:40] [TIMING] Crate { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc, test_kind: Test, krate: "rustc-main" } -- 1.407
[01:33:40] [TIMING] CrateLibrustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", test_kind: Test, krate: "rustc-main" } -- 0.000
[01:34:22] [RUSTC-TIMING] rustdoc test:true 41.736
[01:34:26] [TIMING] CrateRustdoc { host: "x86_64-unknown-linux-gnu", test_kind: Test } -- 45.804
[01:34:27] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 0.690
[01:34:27] [TIMING] UnstableBookGen { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:27] [TIMING] UnstableBookGen { target: "x86_64-unknown-linux-gnu" } -- 0.399
[01:34:28] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 0.756
[01:34:28] [TIMING] Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:29] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "unstable-book", src: "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc" } -- 1.505
[01:34:29] [TIMING] UnstableBook { target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:29] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/first-edition", src: "/checkout/src/doc" } -- 0.000
[01:34:29] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/first-edition" } -- 0.000
[01:34:29] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/second-edition", src: "/checkout/src/doc" } -- 0.000
[01:34:29] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/second-edition" } -- 0.000
[01:34:29] [TIMING] Standalone { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.001
[01:34:31] [TIMING] TheBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", name: "book" } -- 1.298
[01:34:32] [TIMING] Std { stage: 2, target: "x86_64-unknown-linux-gnu" } -- 1.191
[01:34:32] [TIMING] Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" } -- 0.679
[01:34:33] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 0.380
[01:34:33] [TIMING] ErrorIndex { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:33] [TIMING] ErrorIndex { target: "x86_64-unknown-linux-gnu" } -- 0.132
[01:34:33] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "nomicon", src: "/checkout/src/doc" } -- 0.000
[01:34:33] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "nomicon" } -- 0.000
[01:34:33] [TIMING] Nomicon { target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:33] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "reference", src: "/checkout/src/doc" } -- 0.000
[01:34:33] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "reference" } -- 0.000
[01:34:33] [TIMING] Reference { target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:33] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rustdoc", src: "/checkout/src/doc" } -- 0.000
[01:34:33] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "rustdoc" } -- 0.000
[01:34:33] [TIMING] Rustdoc { target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:33] [TIMING] RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rust-by-example", src: "/checkout/src/doc" } -- 0.001
[01:34:33] [TIMING] Rustbook { target: "x86_64-unknown-linux-gnu", name: "rust-by-example" } -- 0.000
[01:34:33] [TIMING] RustByExample { target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:33] [TIMING] CargoBook { target: "x86_64-unknown-linux-gnu", name: "cargo" } -- 0.087
[01:34:37] [RUSTC-TIMING] linkchecker test:false 3.308
[01:34:37] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "linkchecker", path: "src/tools/linkchecker", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 4.090
[01:34:37] [TIMING] Linkchecker { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:34:54] [TIMING] Linkcheck { host: "x86_64-unknown-linux-gnu" } -- 17.201
[01:35:34] [TIMING] ErrorIndex { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 39.533
[01:35:35] [TIMING] DocTest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, path: "src/doc/rustdoc", name: "rustdoc", is_ext_doc: false } -- 1.300
[01:35:35] [TIMING] RustdocBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[01:35:48] [TIMING] DocTest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, path: "src/doc/unstable-book", name: "unstable-book", is_ext_doc: false } -- 12.836
[01:35:48] [TIMING] UnstableBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.000
[01:35:48] [TIMING] RustdocJS { host: "x86_64-unknown-linux-gnu", target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:35:50] [RUSTC-TIMING] rustdoc_themes test:false 1.078
[01:35:50] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustdoc-themes", path: "src/tools/rustdoc-themes", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 1.676
[01:35:50] [TIMING] RustdocTheme { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 0.000
[01:35:50] [TIMING] RustdocTheme { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.017
