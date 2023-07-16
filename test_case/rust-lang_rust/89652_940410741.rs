plain
.................................................................................................... 100/182
....................................................................F.............
failures:

---- spec::tests::x86_64_unknown_linux_gnu stdout ----
thread 'spec::tests::x86_64_unknown_linux_gnu' panicked at 'assertion failed: `(left == right)`
  left: `Err("unknown sanitizer cfi")`,
 right: `Ok(Target { llvm_target: "x86_64-unknown-linux-gnu", pointer_width: 64, arch: "x86_64", data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128", options: TargetOptions { is_builtin: false, endian: little, c_int_width: "32", os: "linux", env: "gnu", abi: "", vendor: "unknown", linker_flavor: Gcc, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-m64"]}, pre_link_objects: {}, post_link_objects: {}, pre_link_objects_fallback: {}, post_link_objects_fallback: {}, crt_objects_fallback: None, late_link_args: {}, late_link_args_dynamic: {}, late_link_args_static: {}, post_link_args: {}, link_script: None, link_env: [], link_env_remove: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: Pic, code_model: None, tls_model: GeneralDynamic, disable_redzone: false, frame_pointer: MayOmit, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", families: ["unix"], abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_emscripten: false, is_like_fuchsia: false, is_like_wasm: false, dwarf_version: None, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, static_position_independent_executables: false, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, main_needs_argc_argv: true, has_elf_tls: true, obj_is_bitcode: false, forces_embed_bitcode: false, bitcode_llvm_cmdline: "", min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Unwind, crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: true, stack_probes: Call, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, default_hidden_visibility: false, emit_debug_gdb_scripts: true, requires_uwtable: false, default_uwtable: false, simd_types_indirect: true, limit_rdylib_exports: true, override_export_symbols: None, merge_functions: Aliases, mcount: "mcount", llvm_abiname: "", relax_elf_relocations: false, llvm_args: [], use_ctors_section: false, eh_frame_header: true, has_thumb_interworking: false, split_debuginfo: Off, supported_sanitizers: ADDRESS | LEAK | MEMORY | THREAD | CFI, default_adjusted_cabi: None, c_enum_min_bits: 32 } })`', compiler/rustc_target/src/spec/tests/tests_impl.rs:6:5
error: test failed, to rerun pass '-p rustc_target --lib'


failures:
    spec::tests::x86_64_unknown_linux_gnu
    spec::tests::x86_64_unknown_linux_gnu

test result: FAILED. 181 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_target" "--" "--quiet"


Build completed unsuccessfully in 0:27:22
