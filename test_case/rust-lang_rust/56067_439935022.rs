plain
travis_time:end:0257a024:start=1542636252705167678,finish=1542636255002626963,duration=2297459285
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:06] .................................................................................................... 100/5036
[00:50:09] .................................................................................................... 200/5036
[00:50:12] .............................ii............................................ii...................ii.. 300/5036
[00:50:14] ..............................................................................................iii... 400/5036
[00:50:17] .....iiiiiiii.iii............................iii...........................................i........ 500/5036
[00:50:24] .................................................................................................... 700/5036
[00:50:30] ..................................................................................i...........i..... 800/5036
[00:50:34] .................................................................................................... 900/5036
[00:50:37] .iiiii..................ii.iiii..................................................................... 1000/5036
---
[00:51:12] .................................................................................................... 2200/5036
[00:51:16] .................................................................................................... 2300/5036
[00:51:20] .................................................................................................... 2400/5036
[00:51:24] .................................................................................................... 2500/5036
[00:51:27] .....................................................................................iiiiiiiii...... 2600/5036
[00:51:34] ...................................................ii............................................... 2800/5036
[00:51:37] .................................................................................................... 2900/5036
[00:51:40] .................................................................................................... 3000/5036
[00:51:44] ..............................................i..................................................... 3100/5036
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:08] 
[01:05:08] running 116 tests
[01:05:11] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:05:12] i.i....iiii.....
[01:05:12] 
[01:05:12]  finished in 3.504
[01:05:12] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:26] 
[01:05:26] running 118 tests
[01:05:52] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:05:56] ......iii.i.....ii
[01:05:56] 
[01:05:56]  finished in 29.423
[01:05:56] travis_fold:end:test_debuginfo

---
[01:32:09]     Finished release [optimized] target(s) in 12.78s
[01:32:09]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_target-b8bb72a2a6360a41
[01:32:09] 
[01:32:09] running 105 tests
[01:32:09] ...........................................................................................F........ 100/105
[01:32:09] failures:
[01:32:09] 
[01:32:09] 
[01:32:09] ---- spec::test_json_encode_decode::x86_64_fortanix_unknown_sgx stdout ----
[01:32:09] thread 'spec::test_json_encode_decode::x86_64_fortanix_unknown_sgx' panicked at 'assertion failed: `(left == right)`
[01:32:09]   left: `Target { llvm_target: "x86_64-unknown-linux-gnu", target_endian: "little", target_pointer_width: "64", target_c_int_width: "32", target_os: "unknown", target_env: "sgx", target_vendor: "fortanix", arch: "x86_64", data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128", linker_flavor: Gcc, options: TargetOptions { is_builtin: false, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-Wl,--as-needed", "-Wl,-z,noexecstack", "-m64", "-fuse-ld=gold", "-nostdlib", "-shared", "-Wl,-e,sgx_entry", "-Wl,-Bstatic", "-Wl,--gc-sections", "-Wl,-z,text", "-Wl,-z,norelro", "-Wl,--rosegment", "-Wl,--no-undefined", "-Wl,--error-unresolved-symbols", "-Wl,--no-undefined-version", "-Wl,-Bsymbolic", "-Wl,--export-dynamic"]}, pre_link_args_crt: {}, pre_link_objects_exe: [], pre_link_objects_exe_crt: [], pre_link_objects_dll: [], late_link_args: {}, post_link_objects: ["libm.a"], post_link_objects_crt: [], post_link_args: {}, link_env: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: false, only_cdylib: false, executables: true, relocation_model: "pic", code_model: None, tls_model: "global-dynamic", disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", target_family: None, abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_android: false, is_like_emscripten: false, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: false, no_default_libraries: true, position_independent_executables: true, needs_plt: false, relro_level: None, archive_format: "gnu", allow_asm: true, custom_unwind_resume: false, has_elf_tls: false, obj_is_bitcode: false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Abort, abi_blacklist: [], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: false, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: false, simd_types_indirect: true, override_export_symbols: Some(["sgx_entry", "HEAP_BASE", "HEAP_SIZE", "RELA", "RELACOUNT", "ENCLAVE_SIZE", "CFGDATA_BASE", "DEBUG"]) } }`,
[01:32:09]  right: `Target { llvm_target: "x86_64-unknown-linux-gnu", target_endian: "little", target_pointer_width: "64", target_c_int_width: "32", target_os: "unknown", target_env: "sgx", target_vendor: "fortanix", arch: "x86_64", data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128", linker_flavor: Gcc, options: TargetOptions { is_builtin: false, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-Wl,--as-needed", "-Wl,-z,noexecstack", "-m64", "-fuse-ld=gold", "-nostdlib", "-shared", "-Wl,-e,sgx_entry", "-Wl,-Bstatic", "-Wl,--gc-sections", "-Wl,-z,text", "-Wl,-z,norelro", "-Wl,--rosegment", "-Wl,--no-undefined", "-Wl,--error-unresolved-symbols", "-Wl,--no-undefined-version", "-Wl,-Bsymbolic", "-Wl,--export-dynamic"]}, pre_link_args_crt: {}, pre_link_objects_exe: [], pre_link_objects_exe_crt: [], pre_link_objects_dll: [], late_link_args: {}, post_link_objects: ["libm.a"], post_link_objects_crt: [], post_link_args: {}, link_env: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: false, only_cdylib: false, executables: true, relocation_model: "pic", code_model: None, tls_model: "global-dynamic", disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", target_family: None, abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_android: false, is_like_emscripten: false, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: false, no_default_libraries: true, position_independent_executables: true, needs_plt: false, relro_level: None, archive_format: "gnu", allow_asm: true, custom_unwind_resume: false, has_elf_tls: false, obj_is_bitcode: false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Abort, abi_blacklist: [], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: false, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: false, simd_types_indirect: true, override_export_symbols: None } }`', librustc_target/spec/mod.rs:289:1
[01:32:09] 
[01:32:09] 
[01:32:09] failures:
[01:32:09] failures:
[01:32:09]     spec::test_json_encode_decode::x86_64_fortanix_unknown_sgx
[01:32:09] test result: FAILED. 104 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:32:09] 
[01:32:09] error: test failed, to rerun pass '--lib'
[01:32:09] 
[01:32:09] 
[01:32:09] 
[01:32:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_target" "--" "--quiet"
[01:32:09] 
[01:32:09] 
[01:32:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:09] Build completed unsuccessfully in 0:45:43
[01:32:09] Build completed unsuccessfully in 0:45:43
[01:32:09] make: *** [check] Error 1
[01:32:09] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e1e28c1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 19 15:36:33 UTC 2018
---
travis_time:end:0f63f2e0:start=1542641796705156203,finish=1542641796711232853,duration=6076650
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07491e99
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkou
