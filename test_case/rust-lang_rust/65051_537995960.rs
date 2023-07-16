

    Finished release [optimized] target(s) in 8m 14s
Assembling stage1 compiler (x86_64-unknown-linux-musl)
Building stage1 std artifacts (x86_64-unknown-linux-musl -> x86_64-unknown-linux-musl)
warning: rustc-1.37.0-src/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
error: process didn't exit successfully: `rustc-1.37.0-src/build/bootstrap/debug/rustc -vV` (exit code: 127)
--- stderr
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_driver-297236e98e3b11ce.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_Backtrace: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_DeleteException: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_RaiseException: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_GetDataRelBase: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_GetIPInfo: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_GetLanguageSpecificData: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_GetRegionStart: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_GetTextRelBase: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_SetGR: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_SetIP: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_GetIP: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libstd-7ee15cbd53b7b254.so: _Unwind_FindEnclosingFunction: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_interface-f6372f68ca4c37fc.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_borrowck-7373c9341a309f71.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_mir-5c1c1819075ebd26.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_save_analysis-846e375bc77c7d8e.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_codegen_utils-65d9103bc435188d.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_metadata-64d587906dd5eba7.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc-cb39e474659e15c8.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc-cb39e474659e15c8.so: _Unwind_FindEnclosingFunction: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc-cb39e474659e15c8.so: _Unwind_GetIP: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc-cb39e474659e15c8.so: _Unwind_Backtrace: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libtest-dc489fd1ade7cf99.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libsyntax-29c4688773e43a24.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_target-1fdfcbe13b408e9a.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_errors-bcce3650c698371e.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libsyntax_pos-91929928d8d8df07.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libarena-40fb647db2c6c16f.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_data_structures-b3d1b53deed1cc21.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libgraphviz-c924c23f873259cc.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_cratesio_shim-451cc189886af978.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libserialize-ded14d9c4374ae5b.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_lint-cb9cab5a661e0fc4.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_traits-768f999b81f7c152.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_resolve-24af22f041ddbc99.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_privacy-d66f878948e250ee.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_typeck-0c55c94c2fe00bc3.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_plugin-aaf21d589d6f9a96.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_passes-230e54df788c1f06.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_codegen_ssa-ffbc4d353d0a6eba.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_incremental-93604673bdae5cca.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_allocator-61b49b916bd03a67.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libsyntax_ext-894cd6a808d68b68.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libfmt_macros-2f4f0c96b22175f2.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/librustc_fs_util-eae473008a93e789.so: _Unwind_Resume: symbol not found
Error relocating rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage1/lib/libterm-143f9562b76da2bf.so: _Unwind_Resume: symbol not found
