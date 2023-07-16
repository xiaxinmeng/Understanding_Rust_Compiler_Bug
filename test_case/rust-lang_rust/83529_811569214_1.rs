
   Compiling rustc_interface v0.0.0 (C:\rust\compiler\rustc_interface)
[RUSTC-TIMING] rustc_symbol_mangling test:false 343.675
[RUSTC-TIMING] rustc_plugin_impl test:false 178.420
[RUSTC-TIMING] rustc_ty_utils test:false 412.983
[RUSTC-TIMING] rustc_incremental test:false 598.292
[RUSTC-TIMING] rustc_save_analysis test:false 591.511
[RUSTC-TIMING] rustc_privacy test:false 419.040
[RUSTC-TIMING] rustc_ast_lowering test:false 746.298
[RUSTC-TIMING] rustc_parse test:false 757.701
[RUSTC-TIMING] rustc_codegen_ssa test:false 784.703
[RUSTC-TIMING] rustc_builtin_macros test:false 867.048
[RUSTC-TIMING] rustc_lint test:false 957.121
[RUSTC-TIMING] rustc_mir_build test:false 902.907
[RUSTC-TIMING] rustc_infer test:false 1054.076
[RUSTC-TIMING] rustc_expand test:false 1132.934
[RUSTC-TIMING] rustc_metadata test:false 1270.295
[RUSTC-TIMING] rustc_resolve test:false 1264.349
[RUSTC-TIMING] rustc_trait_selection test:false 1327.004
[RUSTC-TIMING] rustc_traits test:false 1192.673
[RUSTC-TIMING] rustc_codegen_llvm test:false 1168.620
[RUSTC-TIMING] rustc_passes test:false 1412.476
[RUSTC-TIMING] rustc_interface test:false 1223.635

thread 'LTO module rustc_middle.btguyyk8-cgu.4' has overflowed its stack
[RUSTC-TIMING] rustc_middle test:false 1615.740
error: could not compile `rustc_middle`

Caused by:
  process didn't exit successfully: `C:\rust\build\bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 compiler\rustc_middle\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C metadata=49c48b545a6f1e73 -C extra-filename=-49c48b545a6f1e73 --out-dir C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps -L dependency=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\deps --extern bitflags=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libbitflags-e03c9dfd96a4debb.rmeta --extern chalk_ir=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libchalk_ir-966c239cc2df9d90.rmeta --extern measureme=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libmeasureme-90f0eb70636ff75c.rmeta --extern polonius_engine=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libpolonius_engine-0aba9eefa8cd010b.rmeta --extern rustc_rayon_core=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_rayon_core-92b430a86ac5bcd2.rmeta --extern rustc_apfloat=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_apfloat-85ddf52c2524e5f3.rmeta --extern rustc_arena=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_arena-34f3b231bac29114.rmeta --extern rustc_ast=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_ast-79f3363dc4fba861.rmeta --extern rustc_attr=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_attr-44b1e3d9b6995d09.rmeta --extern rustc_data_structures=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_data_structures-c80d9e54d70ff55d.rmeta --extern rustc_errors=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_errors-01878d15631f1aad.rmeta --extern rustc_feature=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_feature-9dfd44ff009497fa.rmeta --extern rustc_hir=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_hir-f1d082e8566d020c.rmeta --extern rustc_index=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_index-6b52f99ef56658e4.rmeta --extern rustc_macros=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\deps\rustc_macros-593d411e4911c3bb.dll --extern rustc_query_system=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_query_system-740d50df20775353.rmeta --extern rustc_serialize=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_serialize-4fd24e600f08514a.rmeta --extern rustc_session=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_session-0b4cb80a8d54fc67.rmeta --extern rustc_span=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_span-9d7a865d019ad1b8.rmeta --extern rustc_target=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_target-37eecd04060ec2a8.rmeta --extern rustc_type_ir=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\librustc_type_ir-384fbfa03a4d895a.rmeta --extern smallvec=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libsmallvec-1f7f64b0734380c5.rmeta --extern tracing=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libtracing-4c93346f6188a83f.rmeta -Zmacro-backtrace -Ctarget-feature=+crt-static -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic -Z binary-dep-depinfo -L native=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\build\stacker-f6e39d316edc7bc1\out -L native=C:\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\build\psm-b6c7f400dd8ba8bf\out` (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_typeck test:false 1683.957
[RUSTC-TIMING] rustc_mir test:false 2105.423
[RUSTC-TIMING] rustc_query_impl test:false 3333.671
error: build failed
command did not execute successfully: "\\\\?\\C:\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "48" "--release" "--locked" "--features" " llvm max_level_info" "--manifest-path" "C:\\rust\\compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: C:\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 1:21:46
