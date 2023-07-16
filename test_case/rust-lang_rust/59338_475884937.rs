plain
travis_time:end:02bd4eb2:start=1553352799943099964,finish=1553352871945501151,duration=72002401187
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:49:03] failures:
[01:49:03] 
[01:49:03] ---- [run-pass] run-pass/rustc-rust-log.rs stdout ----
[01:49:03] 
[01:49:03] error: test compilation failed although it shouldn't!
[01:49:03] status: exit code: 1
[01:49:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rustc-rust-log.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "human" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log/auxiliary"
[01:49:03] ------------------------------------------
[01:49:03] 
[01:49:03] ------------------------------------------
[01:49:03] stderr:
[01:49:03] stderr:
[01:49:03] ------------------------------------------
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc_target::spec: Got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", target_endian: "little", target_pointer_width: "64", target_c_int_width: "32", target_os: "linux", target_env: "gnu", target_vendor: "unknown", arch: "x86_64", data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128", linker_flavor: Gcc, options: TargetOptions { is_builtin: true, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-Wl,--as-needed", "-Wl,-z,noexecstack", "-m64"]}, pre_link_args_crt: {}, pre_link_objects_exe: [], pre_link_objects_exe_crt: [], pre_link_objects_dll: [], late_link_args: {}, post_link_objects: [], post_link_objects_crt: [], post_link_args: {}, link_env: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: "pic", code_model: None, tls_model: "global-dynamic", disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", target_family: Some("unix"), abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_android: false, is_like_emscripten: false, is_like_fuchsia: false, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, custom_unwind_resume: false, has_elf_tls: true, obj_is_bitcode: false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Unwind, abi_blacklist: [], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: true, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: false, simd_types_indirect: true, override_export_symbols: None, merge_functions: Aliases } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc_target::spec: Got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", target_endian: "little", target_pointer_width: "64", target_c_int_width: "32", target_os: "linux", target_env: "gnu", target_vendor: "unknown", arch: "x86_64", data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128", linker_flavor: Gcc, options: TargetOptions { is_builtin: true, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-Wl,--as-needed", "-Wl,-z,noexecstack", "-m64"]}, pre_link_args_crt: {}, pre_link_objects_exe: [], pre_link_objects_exe_crt: [], pre_link_objects_dll: [], late_link_args: {}, post_link_objects: [], post_link_objects_crt: [], post_link_args: {}, link_env: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: "pic", code_model: None, tls_model: "global-dynamic", disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", target_family: Some("unix"), abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_android: false, is_like_emscripten: false, is_like_fuchsia: false, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, custom_unwind_resume: false, has_elf_tls: true, obj_is_bitcode: false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Unwind, abi_blacklist: [], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: true, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: false, simd_types_indirect: true, override_export_symbols: None, merge_functions: Aliases } }
[01:49:03]  INFO 2019-03-23T16:42:06Z: jobserver::imp: one of 3 or 4 isn't a pipe
[01:49:03]  INFO 2019-03-23T16:42:06Z: jobserver::imp: created a jobserver: Client { read: File { fd: 3, path: "pipe:[167607]", read: true, write: false }, write: File { fd: 4, path: "pipe:[167607]", read: false, write: true } }
[01:49:03]  INFO 2019-03-23T16:42:06Z: rustc_interface::util: codegen backend candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
[01:49:03]  INFO 2019-03-23T16:42:06Z: rustc_interface::util: probing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends for a codegen backend
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<parse>(key=LocalCrate, span=no-location)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(68) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(68) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:69 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(68) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(68) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(69) is on the line at byte pos BytePos(69)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(69) is on the line at char pos CharPos(69)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 2
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:1:69: 2:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(69) is on the line at byte pos BytePos(69)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(69) is on the line at char pos CharPos(69)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 2
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(135) is on the line at byte pos BytePos(69)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(135) is on the line at char pos CharPos(69)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 2
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:2:1: 2:67 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(135) is on the line at byte pos BytePos(69)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(135) is on the line at char pos CharPos(69)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 2
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(136) is on the line at byte pos BytePos(136)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(136) is on the line at char pos CharPos(136)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 3
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:2:67: 3:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(136) is on the line at byte pos BytePos(136)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(136) is on the line at char pos CharPos(136)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 3
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(204) is on the line at byte pos BytePos(136)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(204) is on the line at char pos CharPos(136)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 3
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:3:1: 3:69 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(204) is on the line at byte pos BytePos(136)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(204) is on the line at char pos CharPos(136)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 3
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(205) is on the line at byte pos BytePos(205)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(205) is on the line at char pos CharPos(205)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 4
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:3:69: 4:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(205) is on the line at byte pos BytePos(205)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(205) is on the line at char pos CharPos(205)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 4
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(269) is on the line at byte pos BytePos(205)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(269) is on the line at char pos CharPos(205)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 4
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:4:1: 4:65 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(269) is on the line at byte pos BytePos(205)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(269) is on the line at char pos CharPos(205)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 4
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(270) is on the line at byte pos BytePos(270)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(270) is on the line at char pos CharPos(270)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 5
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:4:65: 5:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(270) is on the line at byte pos BytePos(270)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(270) is on the line at char pos CharPos(270)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 5
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(284) is on the line at byte pos BytePos(270)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(284) is on the line at char pos CharPos(270)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 5
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:5:1: 5:15 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(284) is on the line at byte pos BytePos(270)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(284) is on the line at char pos CharPos(270)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 5
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(285) is on the line at byte pos BytePos(285)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(285) is on the line at char pos CharPos(285)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 6
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:5:15: 6:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(285) is on the line at byte pos BytePos(285)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(285) is on the line at char pos CharPos(285)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 6
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(287) is on the line at byte pos BytePos(285)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(287) is on the line at char pos CharPos(285)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 6
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:6:1: 6:3 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(287) is on the line at byte pos BytePos(285)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(287) is on the line at char pos CharPos(285)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 6
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(288) is on the line at byte pos BytePos(288)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(288) is on the line at char pos CharPos(288)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 7
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:6:3: 7:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(288) is on the line at byte pos BytePos(288)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(288) is on the line at char pos CharPos(288)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 7
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(317) is on the line at byte pos BytePos(288)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(317) is on the line at char pos CharPos(288)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 7
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:7:1: 7:30 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(317) is on the line at byte pos BytePos(288)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(317) is on the line at char pos CharPos(288)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 7
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(318) is on the line at byte pos BytePos(318)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(318) is on the line at char pos CharPos(318)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 8
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:7:30: 8:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(318) is on the line at byte pos BytePos(318)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(318) is on the line at char pos CharPos(318)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 8
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(347) is on the line at byte pos BytePos(318)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(347) is on the line at char pos CharPos(318)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 8
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:8:1: 8:30 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(347) is on the line at byte pos BytePos(318)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(347) is on the line at char pos CharPos(318)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 8
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(348) is on the line at byte pos BytePos(348)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(348) is on the line at char pos CharPos(348)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 9
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:8:30: 9:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(348) is on the line at byte pos BytePos(348)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(348) is on the line at char pos CharPos(348)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 9
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(386) is on the line at byte pos BytePos(348)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(386) is on the line at char pos CharPos(348)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 9
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:9:1: 9:39 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(386) is on the line at byte pos BytePos(348)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(386) is on the line at char pos CharPos(348)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 9
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(388) is on the line at byte pos BytePos(388)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(388) is on the line at char pos CharPos(388)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 11
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:9:39: 11:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(388) is on the line at byte pos BytePos(388)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(388) is on the line at char pos CharPos(388)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 11
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(415) is on the line at byte pos BytePos(388)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(415) is on the line at char pos CharPos(388)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 11
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:11:1: 11:28 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(415) is on the line at byte pos BytePos(388)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(415) is on the line at char pos CharPos(388)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 11
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:11:28: 13:1 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(419) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(419) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(420) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(420) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:13:3: 13:4 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(426) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(426) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(427) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(427) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:13:10: 13:11 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(429) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(429) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(430) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(430) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: /checkout/src/test/run-pass/rustc-rust-log.rs:13:13: 13:14 })
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::attr: parse_outer_attributes: self.token=Ident(fn#0, false)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::parse::attr: parse_outer_attributes: self.token=Eof
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<register_plugins>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<early_crate_name>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<parse>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<parse>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<dep_graph_future>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<prepare_outputs>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<early_crate_name>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<expand_macros>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<register_plugins>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::ty::query::plumbing: ty::query::get_query<early_crate_name>(key=LocalCrate, span=/checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::lint::context: early context: enter_attrs([])
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: mk_printer 78
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(prelude_import)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: INDENT 0
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::lint::context: early context: enter_attrs([Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream(None), is_sugared_doc: false, span: /checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1 }])
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: mk_printer 78
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(prelude_import)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: INDENT 0
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::lint::context: early context: exit_attrs([Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream(None), is_sugared_doc: false, span: /checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1 }])
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: mk_printer 78
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(macro_use)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: INDENT 0
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::lint::context: early context: enter_attrs([Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream(None), is_sugared_doc: false, span: /checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1 }])
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: mk_printer 78
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(macro_use)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: INDENT 0
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::lint::context: early context: exit_attrs([Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream(None), is_sugared_doc: false, span: /checkout/src/test/run-pass/rustc-rust-log.rs:1:1: 1:1 }])
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::lint::context: early context: enter_attrs([])
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::lint::context: early context: exit_attrs([])
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::lint::context: early context: exit_attrs([])
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } } <-> DefIndex(0:0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::Krate}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Krate}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Krate}}"), disambiguator: 0 } } <-> DefIndex(1:0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::CrateDeps}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CrateDeps}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CrateDeps}}"), disambiguator: 0 } } <-> DefIndex(1:1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"), disambiguator: 0 } } <-> DefIndex(1:2)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::LangItems}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItems}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItems}}"), disambiguator: 0 } } <-> DefIndex(1:3)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"), disambiguator: 0 } } <-> DefIndex(1:4)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"), disambiguator: 0 } } <-> DefIndex(1:5)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::SourceMap}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::SourceMap}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::SourceMap}}"), disambiguator: 0 } } <-> DefIndex(1:6)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::Impls}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Impls}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Impls}}"), disambiguator: 0 } } <-> DefIndex(1:7)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"), disambiguator: 0 } }
[01:49:03] DEBUG 2019-03-23T16:42:06Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"), disambiguator: 0 } } <-> DefIndex(1:8)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(429) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(429) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: mk_printer 78
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(prelude_import)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: INDENT 0
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: mk_printer 78
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(::)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('std')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(std)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(::)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('prelude')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(prelude)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(::)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('v1')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(v1)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: INDENT 0
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: mk_printer 78
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: print String(macro_use)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::print::pp: INDENT 0
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(427) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(427) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(427) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(427) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(419) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(419) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(419) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(419) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(0) is on the line at byte pos BytePos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(0) is on the line at char pos CharPos(0)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 1
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(427) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(427) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(429) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(429) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(429) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(429) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(417) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(419) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(419) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(420) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(420) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(424) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(424) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(424) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(424) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(425) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(425) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(425) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(425) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(426) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(426) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(427) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(427) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(428) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(428) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(428) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(428) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(429) is on the line at byte pos BytePos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: char pos CharPos(429) is on the line at char pos CharPos(417)
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte is on line: 13
[01:49:03] DEBUG 2019-03-23T16:42:06Z: syntax::source_map: byte pos BytePos(417) is on the line at byte pos BytePos(417)
---
[01:49:03] test result: FAILED. 2950 passed; 1 failed; 8 ignored; 0 measured; 0 filtered out
[01:49:03] 
[01:49:03] 
[01:49:03] 
[01:49:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:49:03] 
[01:49:03] 
[01:49:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:49:03] Build completed unsuccessfully in 0:10:24
[01:49:03] Build completed unsuccessfully in 0:10:24
[01:49:03] make: *** [check] Error 1
[01:49:03] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16719612
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar 23 16:43:44 UTC 2019
