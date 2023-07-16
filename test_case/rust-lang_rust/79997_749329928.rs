plain
.................................................................................................... 9000/11196
.................................................................................................... 9100/11196
......................................................................................i......i...... 9200/11196
.................................................................................................... 9300/11196
.........................iiiiii..iiiiii.i........................................................... 9400/11196
.................................................................................................... 9600/11196
.................................................................................................... 9700/11196
.................................................................................................... 9800/11196
.................................................................................................... 9900/11196
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.062 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i....ii..ii......ii...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.453 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 100/158
..............................F...........................
failures:

---- spec::tests::wasm32_wasi stdout ----
thread 'spec::tests::wasm32_wasi' panicked at 'assertion failed: `(left == right)`
  left: `Ok(Target { llvm_target: "wasm32-wasi", pointer_width: 32, arch: "wasm32", data_layout: "e-m:e-p:32:32-i64:64-n32:64-S128", options: TargetOptions { is_builtin: false, endian: "little", c_int_width: "32", os: "wasi", env: "", vendor: "unknown", linker_flavor: Lld(Wasm), linker: Some("rust-lld"), lld_flavor: Wasm, pre_link_args: {Gcc: ["-Wl,-z", "-Wl,stack-size=1048576", "-Wl,--stack-first", "-Wl,--allow-undefined", "-Wl,--fatal-warnings", "-Wl,--no-demangle", "-Wl,--export-dynamic", "--target=wasm32-wasi"], Lld(Wasm): ["-z", "stack-size=1048576", "--stack-first", "--allow-undefined", "--fatal-warnings", "--no-demangle", "--export-dynamic"]}, pre_link_objects: {}, post_link_objects: {}, pre_link_objects_fallback: {DynamicNoPicExe: ["crt1.o"], DynamicPicExe: ["crt1.o"], StaticNoPicExe: ["crt1.o"], StaticPicExe: ["crt1.o"], WasiReactorExe: ["crt1-reactor.o"]}, post_link_objects_fallback: {}, crt_objects_fallback: Some(Wasm), late_link_args: {}, late_link_args_dynamic: {}, late_link_args_static: {}, post_link_args: {}, link_script: None, link_env: [], link_env_remove: [], asm_args: [], cpu: "generic", features: "", dynamic_linking: true, only_cdylib: true, executables: true, relocation_model: Static, code_model: None, tls_model: LocalExec, disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "", dll_suffix: ".wasm", exe_suffix: ".wasm", staticlib_prefix: "lib", staticlib_suffix: ".a", os_family: None, abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_emscripten: false, is_like_fuchsia: false, dwarf_version: None, linker_is_gnu: false, allows_weak_linkage: true, has_rpath: false, no_default_libraries: true, position_independent_executables: false, static_position_independent_executables: false, needs_plt: false, relro_level: None, archive_format: "gnu", allow_asm: true, main_needs_argc_argv: false, has_elf_tls: true, obj_is_bitcode: false, forces_embed_bitcode: false, bitcode_llvm_cmdline: "", min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Abort, unsupported_abis: [], crt_static_allows_dylibs: true, crt_static_default: true, crt_static_respected: true, stack_probes: false, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: true, no_builtins: false, default_hidden_visibility: true, emit_debug_gdb_scripts: false, requires_uwtable: false, simd_types_indirect: false, limit_rdylib_exports: false, override_export_symbols: None, merge_functions: Aliases, mcount: "mcount", llvm_abiname: "", relax_elf_relocations: false, llvm_args: [], use_ctors_section: false, eh_frame_header: false, has_thumb_interworking: false, supports_wasi_reactor: false } })`,
 right: `Ok(Target { llvm_target: "wasm32-wasi", pointer_width: 32, arch: "wasm32", data_layout: "e-m:e-p:32:32-i64:64-n32:64-S128", options: TargetOptions { is_builtin: false, endian: "little", c_int_width: "32", os: "wasi", env: "", vendor: "unknown", linker_flavor: Lld(Wasm), linker: Some("rust-lld"), lld_flavor: Wasm, pre_link_args: {Gcc: ["-Wl,-z", "-Wl,stack-size=1048576", "-Wl,--stack-first", "-Wl,--allow-undefined", "-Wl,--fatal-warnings", "-Wl,--no-demangle", "-Wl,--export-dynamic", "--target=wasm32-wasi"], Lld(Wasm): ["-z", "stack-size=1048576", "--stack-first", "--allow-undefined", "--fatal-warnings", "--no-demangle", "--export-dynamic"]}, pre_link_objects: {}, post_link_objects: {}, pre_link_objects_fallback: {DynamicNoPicExe: ["crt1.o"], DynamicPicExe: ["crt1.o"], StaticNoPicExe: ["crt1.o"], StaticPicExe: ["crt1.o"], WasiReactorExe: ["crt1-reactor.o"]}, post_link_objects_fallback: {}, crt_objects_fallback: Some(Wasm), late_link_args: {}, late_link_args_dynamic: {}, late_link_args_static: {}, post_link_args: {}, link_script: None, link_env: [], link_env_remove: [], asm_args: [], cpu: "generic", features: "", dynamic_linking: true, only_cdylib: true, executables: true, relocation_model: Static, code_model: None, tls_model: LocalExec, disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "", dll_suffix: ".wasm", exe_suffix: ".wasm", staticlib_prefix: "lib", staticlib_suffix: ".a", os_family: None, abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_emscripten: false, is_like_fuchsia: false, dwarf_version: None, linker_is_gnu: false, allows_weak_linkage: true, has_rpath: false, no_default_libraries: true, position_independent_executables: false, static_position_independent_executables: false, needs_plt: false, relro_level: None, archive_format: "gnu", allow_asm: true, main_needs_argc_argv: false, has_elf_tls: true, obj_is_bitcode: false, forces_embed_bitcode: false, bitcode_llvm_cmdline: "", min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Abort, unsupported_abis: [], crt_static_allows_dylibs: true, crt_static_default: true, crt_static_respected: true, stack_probes: false, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: true, no_builtins: false, default_hidden_visibility: true, emit_debug_gdb_scripts: false, requires_uwtable: false, simd_types_indirect: false, limit_rdylib_exports: false, override_export_symbols: None, merge_functions: Aliases, mcount: "mcount", llvm_abiname: "", relax_elf_relocations: false, llvm_args: [], use_ctors_section: false, eh_frame_header: false, has_thumb_interworking: false, supports_wasi_reactor: true } })`', compiler/rustc_target/src/spec/tests/tests_impl.rs:6:5


failures:
    spec::tests::wasm32_wasi
    spec::tests::wasm32_wasi

test result: FAILED. 157 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustc_target --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_target" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:28:45
