plain
Updating files:  98% (32355/33015)
Updating files:  99% (32685/33015)
Updating files: 100% (33015/33015)
Updating files: 100% (33015/33015), done.
branch 'try' set up to track 'origin/try'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'083263b570c1aa26f2f148f0e329beb40695f37e'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler",
 "SCRIPT": "python src/ci/pgo-windows.py",
}
##[endgroup]
adding extra environment variable DIST_REQUIRE_ALL_TOOLS
adding extra environment variable RUST_CONFIGURE_ARGS
---
file:.git/config remote.origin.url=https://github.com/rust-lang-ci/rust
file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
file:.git/config gc.auto=0
file:.git/config http.https://github.com/.extraheader=AUTHORIZATION: basic ***
file:.git/config branch.try.remote=origin
file:.git/config branch.try.merge=refs/heads/try
file:.git/config submodule.library/backtrace.url=https://github.com/rust-lang/backtrace-rs.git
file:.git/config submodule.library/stdarch.active=true
file:.git/config submodule.library/stdarch.url=https://github.com/rust-lang/stdarch.git
file:.git/config submodule.src/doc/edition-guide.active=true
---
[RUSTC-TIMING] rustc_typeck test:false 161.885
   Compiling rustc_driver v0.0.0 (D:\a\rust\rust\compiler\rustc_driver)
error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30133\\bin\\HostX64\\x64\\link.exe" "/DEF:C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcVk34Dx\\lib.def" "/NOLOGO" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.0.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.1.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.10.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.11.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.12.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.13.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.14.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.15.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.2.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.3.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.4.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.5.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.6.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.7.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.8.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.rustc_driver.d4bb309f-cgu.9.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.36yig9mxpsd2n02q.rcgu.rmeta" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\release\\deps" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\build\\stacker-eb25d0e821e54145\\out" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\build\\psm-04f4a1a54dd952d3\\out" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\build\\rustc_llvm-8cb36a75b8a717ad\\out" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\lib" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_error_codes-41cdc4cddb1b3995.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_save_analysis-2c1c6be29e7c9d19.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libserde_json-8a89def5127dcc30.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libryu-cc72fb2e73bfcf66.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libitoa-b5acef27c1cdf50f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librls_data-23c22270b3540e83.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librls_span-2f2ecdbe2828e1f1.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_log-f237c110a2320b6c.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_interface-22a755eefb5e3a05.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_codegen_llvm-53741711ae948e07.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_llvm-55a6c88215a479e5.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_privacy-3dd122882539342f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_monomorphize-28f99590afdd612e.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_mir_transform-6e79e4ab1aa8a2eb.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_builtin_macros-4ebb054592742652.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_typeck-90ade2d63ddcad2a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_ty_utils-4555cfc9854efe0a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_resolve-43ab252ed040a0e8.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_ast_lowering-42b3a39f15ab24ec.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_passes-34ca9e993fe3c711.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_mir_build-39ad449e857ec7c1.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_borrowck-896d0534e391840b.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_traits-ccbdd9c9cc2f66e2.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libchalk_engine-873ddc107ba9e441.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libchalk_solve-7e49020ef8595fd0.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtracing_tree-4adda6da6bb82b64.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtracing_subscriber-9201bd2bc31d404d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libthread_local-48a1d44d1f833bc4.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libsharded_slab-eb623218ae50aa9d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libansi_term-b0d9cf2f4c6e59f9.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libmatchers-7d0c0ba25521f7f7.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libregex_automata-e852af7e65c73198.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtracing_log-002fe51d3b26d2a3.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libpetgraph-ce634cad8cbe3667.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libfixedbitset-b32b700ee96305bd.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_const_eval-bc93d8a83f00c57f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_mir_dataflow-991d834959ef119a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_query_impl-2637db778e303b0d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_codegen_ssa-2468d5a5ed1b9405.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libthorin-4f70f0634ae47dda.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libgimli-90a8c0dbf75ec9ca.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libfallible_iterator-e0af099474e151cc.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_symbol_mangling-f17041d12d8dc33b.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_demangle-148a15d692f4ff27.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libpunycode-b8a802241f142c5a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_incremental-d3898115e21d24aa.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libpathdiff-06eec329439daac0.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libobject-75d7373f7378552a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libflate2-2b84d1a931034baf.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libminiz_oxide-93ff31e584a6d97d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libadler-0da2af2842c10d78.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libcrc32fast-b42a8b94b08c61eb.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libregex-2242c12c4d3a68c2.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libaho_corasick-1c85b9a1c4bd19c1.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libmemchr-91865a40f8f5fb73.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libregex_syntax-884d8129a405a162.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libcc-06080fe483a97410.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_plugin_impl-9b7f378346087948.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_metadata-9d5276cda6545f92.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_hir_pretty-ae7cd7abb8e8d9b7.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libsnap-78a1bd656e01a57e.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_expand-18a5914934c3bc00.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_ast_passes-1df4ec2f14a08d57.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libitertools-6589890aa97a86c0.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_parse-92ac0964d1bebadf.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libproc_macro-0bed512bd5ccfb10.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\liblibloading-ba691a2f8a9e1ba8.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_lint-17c04a6344dd2cb8.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunicode_security-e3a830ebfa30a15a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunicode_script-bc0a68766002a826.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunicode_normalization-df184a44b4c1f52e.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtinyvec-e75fb9b194b0f7d5.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_trait_selection-c65bf8ee46bb9c0a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_parse_format-2ebac8253148f554.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_infer-38008c06feb0e479.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_middle-5bb804ecee41a3dd.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libchalk_ir-087186b405043642.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librand_xoshiro-23bac5f13e34a98a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_type_ir-1e1d5ccdfec7ff6f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_apfloat-ba359a805b487d9e.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libgsgdt-e6afe4c3eb60f567.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libserde-3a95edd0cb66d91f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libeither-170038d695fa0e75.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libpolonius_engine-3b75650f4b8881ac.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libdatafrog-464e70a987a6eaaa.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_attr-2286b79aa31bc908.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_ast_pretty-200795016b884071.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_query_system-ead8c7c68be81cb1.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_session-16ae0ffd1b8a7e0d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libgetopts-c19fc360c98257e9.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_fs_util-29c6d1ee2060f041.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_feature-5b28f1cf88ee2e55.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_errors-f1d6b4bafd364949.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtermize-40e8ea7940d4d37d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libatty-ee28800075653480.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libannotate_snippets-ddcb34ec9d26b972.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtermcolor-60f94bd9bb6491a4.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi_util-1bcf540a8112ca00.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_lint_defs-55649265d67fe12f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_hir-14d4d38991899cef.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libodht-a30cf9899304c41d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_target-f38f5b72da366039.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_error_messages-210f15d1ef483034.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libfluent_bundle-32621674c48077ad.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libfluent_langneg-4c41688031b57d2b.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libintl_pluralrules-3d0c119cd9655897.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libself_cell-aac22582c9cce4f3.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libintl_memoizer-5831097f3b8cb3c6.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtype_map-48f302d876edc734.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunic_langid-666af6315a236556.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunic_langid_macros-580b08d10747a57a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunic_langid_impl-6cca7b97c0f7f251.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtinystr-e955f947a499558a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libfluent_syntax-6c5ba145daa946f9.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libthiserror-e92365be4a57533e.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_ast-9b188739a4c04da2.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_lexer-a7d2078420d8a5aa.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunic_emoji_char-2cd663b1f7c61b89.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunic_ucd_version-2aeeb601e982057c.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunic_common-57791a2c1d8632aa.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunic_char_property-2f823ae7376d04b8.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunic_char_range-8eb772ac73dd68b7.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunicode_xid-0a11ef049bbdafa4.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_span-3adac6bbe811b8a9.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libscoped_tls-3138ab99353e0c70.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libsha2-03ed4174104f2b31.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libsha1-eb46452427565700.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libcpufeatures-f6b7bc6c6dad3c2b.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libmd5-435e0f0e0f4c16e6.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libdigest-0905248ce8e68f14.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libblock_buffer-902746609877b979.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libcrypto_common-797126a30a5b5921.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libgeneric_array-a8e1a6124bcd1de7.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtypenum-1eb588c5ef590c2d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libunicode_width-a6a0db6edc87ca17.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_arena-3808aa39501851fb.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_data_structures-8eba8501f37cef28.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libstacker-3779e97dffd1ffaf.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libpsm-2ef462b2058447f8.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\liblibc-ea81b5c78cef5a6a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libmemmap2-90283226b3222cc7.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtempfile-f7bb574cb6db6da7.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librand-9ebc24f271847cf0.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librand_chacha-1fd387769c21f41a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libppv_lite86-409f695ac0d035b1.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librand_core-747f641c238d9371.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libremove_dir_all-202184ad41585a3f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libmeasureme-27845090f2d28b69.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libparking_lot-f7a5b522cca220be.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libparking_lot_core-9f15f148731f6bf1.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-9250fc4db850af09.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\liblock_api-92a28f079e8331a0.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libscopeguard-df559294b0760a6c.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libinstant-5cbaa3d8ccf569ab.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libena-7351d3dadd3ab10c.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\liblog-58734aafd6bc9843.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libstable_deref_trait-c9f50994d03fa227.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_graphviz-802ec0bdea3e2718.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libjobserver-83df5e51b9748c71.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_hash-2b96c00051647a5b.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_index-8ea4f521c57d1d1d.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\librustc_serialize-06bd018544af4695.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libindexmap-23f14708dd6aee03.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libhashbrown-5abd7dcdbf9348b0.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libahash-904ea9fae55cfbf1.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libgetrandom-d2ceb57ef693fa77.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libonce_cell-266d760174534186.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libsmallvec-de27dd75b7e78c2f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libarrayvec-4733eeb11f086210.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libcfg_if-2c1cc1b49d0e1fd7.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libbitflags-c9bd86acc944cb9b.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtracing-5e73e329bed65f3b.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libpin_project_lite-9a691d70bd33faf9.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libtracing_core-6f2437e76581ac79.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\liblazy_static-d328636a98fb3e83.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\libcfg_if-e2118195388192ea.rlib" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "std-169b70f5db17574a.dll.lib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-0129814d9ad6fce1.rlib" "psapi.lib" "shell32.lib" "ole32.lib" "uuid.lib" "advapi32.lib" "advapi32.lib" "ole32.lib" "oleaut32.lib" "advapi32.lib" "cfgmgr32.lib" "gdi32.lib" "kernel32.lib" "msimg32.lib" "opengl32.lib" "psapi.lib" "synchronization.lib" "user32.lib" "winspool.lib" "advapi32.lib" "kernel32.lib" "ws2_32.lib" "bcrypt.lib" "advapi32.lib" "userenv.lib" "kernel32.lib" "libcmt.lib" "/NXCOMPAT" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/OUT:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.dll" "/OPT:REF,ICF" "/DLL" "/IMPLIB:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\deps\\rustc_driver-bb3210f75397afc6.dll.lib" "/DEBUG"
  = note: clang_rt.profile-x86_64.lib(InstrProfilingValue.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingValue.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingPlatformWindows.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingPlatformWindows.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingFile.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingFile.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(WindowsMMap.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(WindowsMMap.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingBuffer.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingBuffer.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingMerge.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingMerge.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfiling.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfiling.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingWriter.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingWriter.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingMergeFile.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingMergeFile.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingInternal.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingInternal.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingUtil.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingUtil.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

          clang_rt.profile-x86_64.lib(InstrProfilingBiasVar.c.obj) : warning LNK4099: PDB 'clang_rt.profile-x86_64.pdb' was not found with 'clang_rt.profile-x86_64.lib(InstrProfilingBiasVar.c.obj)' or at 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\clang_rt.profile-x86_64.pdb'; linking object as if no debug info

             Creating library D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\rustc_driver-bb3210f75397afc6.dll.lib and object D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\x86_64-pc-windows-msvc\release\deps\rustc_driver-bb3210f75397afc6.dll.exp

          librustc_llvm-55a6c88215a479e5.rlib(RemarkParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(TypeIndexDiscovery.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(YAMLRemarkParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(BitstreamRemarkParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LegalizeVectorTypes.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RecordName.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(CodeViewError.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RecordSerialization.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Platform.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(COFF.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SelectionDAGPrinter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ResourcePriorityQueue.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(InterfaceFile.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Architecture.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(TextStubCommon.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ArchitectureSet.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SystemZConstantPoolValue.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonISelLoweringHVX.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ItaniumDemangle.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RemarkStringTable.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Mips16FrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MipsSEFrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Mips16ISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MipsSEISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ContinuationRecordBuilder.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SymbolRecordMapping.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(CodeViewRecordIO.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(TypeRecordMapping.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DbgEntityHistoryCalculator.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DwarfExpression.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(GlobalTypeTableBuilder.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(CVTypeVisitor.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DwarfCompileUnit.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AddressPool.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AccelTable.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DwarfStringPool.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ScheduleDAGSDNodes.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(StatepointLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DwarfFile.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DwarfUnit.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ScheduleDAGFast.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LegalizeTypes.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LegalizeVectorOps.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LegalizeDAG.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LegalityPredicates.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RegisterBank.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ScheduleDAGRRList.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ScheduleDAGVLIW.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ELFObjectWriter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MachObjectWriter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(XCOFFObjectWriter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LegacyLegalizerInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(IntrinsicLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86InterleavedAccess.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WinCOFFObjectWriter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WasmObjectWriter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFTypeUnit.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFAbbreviationDeclaration.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFAddressRange.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86ShuffleDecode.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RelocationResolver.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugRnglists.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugInfoEntry.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFCompileUnit.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFAcceleratorTable.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFVerifier.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugAranges.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Decompressor.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFGdbIndex.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFListTable.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugPubTable.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDataExtractor.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugLine.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFUnitIndex.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugAddr.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugRangeList.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugMacro.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugAbbrev.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugLoc.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugArangeSet.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMAttributeParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RISCVAttributeParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Wasm.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDebugFrame.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(BinaryStreamError.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ScopedPrinter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ELF.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ELFAttributeParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(TextStub.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(TapiFile.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(BinaryStreamRef.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(BinaryStreamReader.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LLParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LLLexer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(YAMLRemarkSerializer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(BitstreamRemarkSerializer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DataExtractor.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(VPlanVerifier.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(OptimizedStructLayout.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ProvenanceAnalysisEvaluator.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DebugHandlerBase.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ItaniumManglingCanonicalizer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SymbolRemappingReader.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MachineStableHash.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WebAssemblyISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(BPFRegisterInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(BPFISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SelectionDAGDumper.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonMCCompound.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonHazardRecognizer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonShuffler.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WebAssemblyRegisterInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonSelectionDAGInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonISelDAGToDAGHVX.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonBlockRanges.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonMCShuffler.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(BitTracker.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RDFCopy.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RDFDeadCode.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SparcFrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SparcRegisterInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LoopUnrollRuntime.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(HexagonBitTracker.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RISCVRegisterInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RISCVFrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DAGCombiner.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SparcISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SystemZHazardRecognizer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MSP430ISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MSP430RegisterInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RISCVInstrInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MIRPrinter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SystemZISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SystemZFrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SystemZSelectionDAGInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(PPCRegisterInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(PPCInstrInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(PPCISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(PPCCallLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MipsCallLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MipsRegisterBankInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MipsAnalyzeImmediate.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MipsELFObjectWriter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(M68kFrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(M68kCallLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MipsInstrInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MipsISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RegAllocPBQP.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AArch64PBQPRegAlloc.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AVRISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AVRRegisterInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AArch64ISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AArch64CallLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AArch64RegisterBankInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AArch64InstructionSelector.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WasmAsmParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AArch64InstrInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Combiner.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(CombinerHelper.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ConstantPools.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(AsmLexer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DarwinAsmParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ELFAsmParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMRegisterBankInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMSelectionDAGInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMUnwindOpAsm.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMMachObjectWriter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Thumb1InstrInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMCallLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMLegalizerInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(TargetLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Thumb2InstrInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Thumb1FrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMFrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MCPseudoProbe.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86ShuffleDecodeConstantPool.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DIE.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ARMBaseInstrInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WinException.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(EHStreamer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WinCFGuard.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(PseudoProbePrinter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86InstPrinterCommon.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86InstComments.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DwarfDebug.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(CodeViewDebug.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MCWin64EH.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MCObjectStreamer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MachO.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MCAssembler.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(GISelKnownBits.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(CodeGenCoverage.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ScoreboardHazardRecognizer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MCWinCOFFStreamer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LegalizerHelper.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LostDebugLocObserver.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LegalizerInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RegisterBankInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(InlineAsmLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(CallLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(FunctionLoweringInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(GISelChangeObserver.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(Utils.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MachineIRBuilder.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SwiftErrorValueTracking.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SwitchLoweringUtils.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SelectionDAGISel.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SelectionDAG.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(SelectionDAGBuilder.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86InstrFoldTables.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MIRPrintingPass.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86CallLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86RegisterBankInfo.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(LoopTraversal.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RDFRegisters.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MCAsmStreamer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MCWasmStreamer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MCXCOFFStreamer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86FrameLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MachineSSAUpdater.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RDFGraph.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(RDFLiveness.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFDie.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFFormValue.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFExpression.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(X86ISelLowering.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(XCOFFObjectFile.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WasmObjectFile.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFContext.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(DWARFUnit.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(WindowsResource.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(ELFObjectFile.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

          librustc_llvm-55a6c88215a479e5.rlib(MachOObjectFile.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop
