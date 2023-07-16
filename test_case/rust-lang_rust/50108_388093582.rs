plain
  Downloading https://files.pythonhosted.org/packages/b7/31/05c8d001f7f87f0f07289a5fc0fc3832e9a57f2dbd4d3b0fee70e0d51365/jmespath-0.9.3-py2.py3-none-any.whl
Collecting python-dateutil<3.0.0,>=2.1; python_version >= "2.7" (from botocore==1.10.17->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/cf/f5/af2b09c957ace60dcfac112b669c45c8c97e32f94aa8b56da4c6d1682825/python_dateutil-2.7.3-py2.py3-none-any.whl (211kB)
    4% |█▌                              | 10kB 44.9MB/s eta 0:00:01
    9% |███                             | 20kB 45.9MB/s eta 0:00:01
    14% |████▋                           | 30kB 50.2MB/s eta 0:00:01
    19% |██████▏                         | 40kB 52.5MB/s eta 0:00:01
---
[00:56:34] ...............i....................................................................................
[00:56:40] ..........test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:57:08] ..........................................................................................
[00:57:29] .................................................................................ii.................
[00:58:20] ......F......................................i....................................................i.
[00:58:30] ii.......................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:59:26] ......iiiiiii.......................................................................................
[00:59:47] ....................................................................................................
[01:00:07] ....................................................................................................
[01:00:26] ...............................................................................
[01:00:26] ...............................................................................
[01:00:26] failures:
[01:00:26] 
[01:00:26] ---- [run-pass] run-pass/rustc-rust-log.rs stdout ----
[01:00:26]  
[01:00:26] error: compilation failed!
[01:00:26] status: exit code: 101
[01:00:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rustc-rust-log.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log.stage2-x86_64-unknown-linux-gnu.aux"
[01:00:26] ------------------------------------------
[01:00:26] Pre-trans
[01:00:26] Pre-trans
[01:00:26] Ty interner             total           ty region  both
[01:00:26]     TyAdt             :    317 89.5%,  0.0%   0.0%  0.0%
[01:00:26]     TyArray           :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TySlice           :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TyRawPtr          :      4  1.1%,  0.0%   0.0%  0.0%
[01:00:26]     TyRef             :     10  2.8%,  0.0%   0.0%  0.0%
[01:00:26]     TyFnDef           :      1  0.3%,  0.0%   0.0%  0.0%
[01:00:26]     TyFnPtr           :      1  0.3%,  0.0%   0.0%  0.0%
[01:00:26]     TyGenerator       :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TyGeneratorWitness:      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TyDynamic         :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TyClosure         :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TyTuple           :      1  0.3%,  0.0%   0.0%  0.0%
[01:00:26]     TyParam           :     20  5.6%,  0.0%   0.0%  0.0%
[01:00:26]     TyInfer           :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TyProjection      :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TyAnon            :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]     TyForeign         :      0  0.0%,  0.0%   0.0%  0.0%
[01:00:26]                   total    354         0.0%   0.0%  0.0%
[01:00:26] Substs interner: #35
[01:00:26] Region interner: #27
[01:00:26] Stability interner: #0
[01:00:26] Interpret interner: #0
[01:00:2false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(64), panic_strategy: Unwind, abi_blacklist: [], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: true, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: false } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc_target::spec: Got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", target_endian: "little", target_pointer_width: "64", target_c_int_width: "32", target_os: "linux", target_env: "gnu", target_vendor: "unknown", arch: "x86_64", data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128", linker_flavor: Gcc, options: TargetOptions { is_builtin: true, linker: None, pre_link_args: {Gcc: ["-Wl,--as-needed", "-Wl,-z,noexecstack", "-m64"]}, pre_link_objects_exe: [], pre_link_objects_dll: [], late_link_args: {}, post_link_objects: [], post_link_args: {}, link_env: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: "pic", code_model: None, tls_model: "global-dynamic", disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", target_family: Some("unix"), abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_android: false, is_like_emscripten: false, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, relro_level: Full, archive_format: "gnu", allow_asm: true, custom_unwind_resume: false, exe_allocation_crate: Some("alloc_jemalloc"), has_elf_tls: true, obj_is_bitcode: false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(64), panic_strategy: Unwind, abi_blacklist: [], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: true, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: false } }
[01:00:26]  INFO 2018-05-10T15:40:21Z: jobserver::imp: one of 3 or 4 isn't a pipe
[01:00:26]  INFO 2018-05-10T15:40:21Z: jobserver::imp: created a jobserver: Client { read: File { fd: 3, path: "pipe:[130529]", read: true, write: false }, write: File { fd: 4, path: "pipe:[130529]", read: false, write: true } }
[01:00:26]  INFO 2018-05-10T15:40:21Z: rustc_driver: codegen backend candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
[01:00:26]  INFO 2018-05-10T15:40:21Z: rustc_driver: probing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends for a codegen backend
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(0), hi: BytePos(64), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(64), hi: BytePos(65), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(65), hi: BytePos(127), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(127), hi: BytePos(128), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(128), hi: BytePos(162), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(162), hi: BytePos(163), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(163), hi: BytePos(165), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(165), hi: BytePos(166), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(166), hi: BytePos(234), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(234), hi: BytePos(235), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(235), hi: BytePos(300), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(300), hi: BytePos(301), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(301), hi: BytePos(364), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(364), hi: BytePos(365), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(365), hi: BytePos(429), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(429), hi: BytePos(430), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(430), hi: BytePos(465), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(465), hi: BytePos(467), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(467), hi: BytePos(494), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(494), hi: BytePos(496), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(498), hi: BytePos(499), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(505), hi: BytePos(506), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(508), hi: BytePos(509), ctxt: #0 } })
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::codemap: byte pos BytePos(496) is on the line at byte pos BytePos(496)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::codemap: char pos CharPos(496) is on the line at char pos CharPos(496)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::codemap: byte is on line: 13
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::attr: parse_outer_attributes: self.token=Ident(fn#0, false)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::parse::attr: parse_outer_attributes: self.token=Eof
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } } <-> DefIndex(0:0)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::Krate}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Krate}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Krate}}"), disambiguator: 0 } } <-> DefIndex(1:0)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::CrateDeps}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CrateDeps}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CrateDeps}}"), disambiguator: 0 } } <-> DefIndex(1:1)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"), disambiguator: 0 } } <-> DefIndex(1:2)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::LangItems}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItems}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItems}}"), disambiguator: 0 } } <-> DefIndex(1:3)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"), disambiguator: 0 } } <-> DefIndex(1:4)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"), disambiguator: 0 } } <-> DefIndex(1:5)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::CodeMap}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CodeMap}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CodeMap}}"), disambiguator: 0 } } <-> DefIndex(1:6)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::Impls}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Impls}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Impls}}"), disambiguator: 0 } } <-> DefIndex(1:7)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"), disambiguator: 0 } } <-> DefIndex(1:8)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::codemap: byte pos BytePos(496) is on the line at byte pos BytePos(496)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::codemap: char pos CharPos(496) is on the line at char pos CharPos(496)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::codemap: byte is on line: 13
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: mk_printer 78
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(prelude_import)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: INDENT 0
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: mk_printer 78
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('std')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(std,3) 3 (remaining line space=78)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(std)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=75)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(::)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('prelude')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(prelude,7) 7 (remaining line space=73)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(prelude)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=66)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(::)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('v1')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(v1,2) 2 (remaining line space=64)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(v1)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: INDENT 0
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: mk_printer 78
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(macro_use,9) 9 (remaining line space=78)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(macro_use)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: INDENT 0
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::def_collector: visit_item: Item { ident: #0, attrs: [], id: NodeId(1), node: Mod(Mod { inner: Span { lo: BytePos(496), hi: BytePos(508), ctxt: #0 }, items: [Item { ident: #0, attrs: [Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }], id: NodeId(2), node: Use(UseTree { prefix: path(std::prelude::v1), kind: Glob, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 }, tokens: None }, Item { ident: std#0, attrs: [Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }], id: NodeId(3), node: ExternCrate(None), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 }, tokens: None }, Item { ident: main#0, attrs: [], id: NodeId(4), node: Fn(FnDecl { inputs: [], output: Default(Span { lo: BytePos(506), hi: BytePos(506), ctxt: #0 }), variadic: false }, Normal, Spanned { node: NotConst, span: Span { lo: BytePos(496), hi: BytePos(498), ctxt: #0 } }, Rust, Generics { params: [], where_clause: WhereClause { id: NodeId(5), predicates: [], span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, Block { stmts: [], id: NodeId(6), rules: Default, span: Span { lo: BytePos(506), hi: BytePos(508), ctxt: #0 }, recovered: false }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(496), hi: BytePos(508), ctxt: #0 }, tokens: Some(TokenStream { kind: Stream([TokenStream { kind: Tree(Token(Span { lo: BytePos(496), hi: BytePos(498), ctxt: #0 }, Ident(fn#0, false))) }, TokenStream { kind: Tree(Token(Span { lo: BytePos(499), hi: BytePos(503), ctxt: #0 }, Ident(main#0, false))) }, TokenStream { kind: Tree(Delimited(Span { lo: BytePos(503), hi: BytePos(505), ctxt: #0 }, Delimited { delim: Paren, tts: ThinTokenStream(None) })) }, TokenStream { kind: Tree(Delimited(Span { lo: BytePos(506), hi: BytePos(508), ctxt: #0 }, Delimited { delim: Brace, tts: ThinTokenStream(None) })) }]) }) }] }), vis: Spanned { node: Public, span: Span { lo: BytePos(496), hi: BytePos(496), ctxt: #0 } }, span: Span { lo: BytePos(496), hi: BytePos(508), ctxt: #0 }, tokens: None }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: mk_printer 78
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(prelude_import)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: INDENT 0
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: mk_printer 78
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('std')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(std,3) 3 (remaining line space=78)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(std)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=75)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(::)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('prelude')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(prelude,7) 7 (remaining line space=73)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(prelude)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=66)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(::)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('v1')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(v1,2) 2 (remaining line space=64)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(v1)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: INDENT 0
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::def_collector: visit_item: Item { ident: #0, attrs: [Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }], id: NodeId(2), node: Use(UseTree { prefix: path(std::prelude::v1), kind: Glob, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 }, tokens: None }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(2), data=Misc, parent_def=DefIndex(0:0))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(2), data=Misc)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } } <-> DefIndex(0:1)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: def_index_to_node[DefIndex(0:1) <-> NodeId(2)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: mk_printer 78
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(macro_use,9) 9 (remaining line space=78)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(macro_use)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: INDENT 0
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::def_collector: visit_item: Item { ident: std#0, attrs: [Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }], id: NodeId(3), node: ExternCrate(None), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 }, tokens: None }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(3), data=TypeNs("std"), parent_def=DefIndex(0:0))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(3), data=TypeNs("std"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } } <-> DefIndex(0:2)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: def_index_to_node[DefIndex(0:2) <-> NodeId(3)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::def_collector: visit_item: Item { ident: main#0, attrs: [], id: NodeId(4), node: Fn(FnDecl { inputs: [], output: Default(Span { lo: BytePos(506), hi: BytePos(506), ctxt: #0 }), variadic: false }, Normal, Spanned { node: NotConst, span: Span { lo: BytePos(496), hi: BytePos(498), ctxt: #0 } }, Rust, Generics { params: [], where_clause: WhereClause { id: NodeId(5), predicates: [], span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, Block { stmts: [], id: NodeId(6), rules: Default, span: Span { lo: BytePos(506), hi: BytePos(508), ctxt: #0 }, recovered: false }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(496), hi: BytePos(508), ctxt: #0 }, tokens: Some(TokenStream { kind: Stream([TokenStream { kind: Tree(Token(Span { lo: BytePos(496), hi: BytePos(498), ctxt: #0 }, Ident(fn#0, false))) }, TokenStream { kind: Tree(Token(Span { lo: BytePos(499), hi: BytePos(503), ctxt: #0 }, Ident(main#0, false))) }, TokenStream { kind: Tree(Delimited(Span { lo: BytePos(503), hi: BytePos(505), ctxt: #0 }, Delimited { delim: Paren, tts: ThinTokenStream(None) })) }, TokenStream { kind: Tree(Delimited(Span { lo: BytePos(506), hi: BytePos(508), ctxt: #0 }, Delimited { delim: Brace, tts: ThinTokenStream(None) })) }]) }) }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(4), data=ValueNs("main"), parent_def=DefIndex(0:0))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4), data=ValueNs("main"))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 } }
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 } } <-> DefIndex(0:3)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::hir::map::definitions: create_def_with_parent: def_index_to_node[DefIndex(0:3) <-> NodeId(4)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: mk_printer 78
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: []
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: print String(prelude_import)
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: pp Vec<0,0>
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::print::pp: INDENT 0
[01:00:26] DEBUG 2018-05-10T15:40:21Z: syntax::attr: Marking Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } } as used.
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc_metadata::creader: resolving extern crate stmt. ident: std orig_name: None
[01:00:26]  INFO 2018-05-10T15:40:21Z: rustc_metadata::creader: resolving crate `extern crate std as std`
[01:00:26]  INFO 2018-05-10T15:40:21Z: rustc_metadata::creader: falling back to a load
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dropck-eyepatch.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dropck-eyepatch.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-call-deep2.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-call-deep2.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-20371.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-20371.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-enum-byref.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-enum-byref.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-infer-borrow-scope-within-loop-ok.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-infer-borrow-scope-within-loop-ok.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-9129.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-9129.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/move-3-unique.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/move-3-unique.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24434.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24434.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23833.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23833.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/blind-item-local-shadow.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/blind-item-local-shadow.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/anon-extern-mod-cross-crate-2.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/anon-extern-mod-cross-crate-2.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-ivec-leak.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-ivec-leak.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2748-a.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mod_file.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-const-marks-live-code.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-const-marks-live-code.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fun-indirect-call.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fun-indirect-call.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-39089.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-39089.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cross-crate-newtype-struct-pat.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cross-crate-newtype-struct-pat.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesj/build/x86_64-unknown-linux-gnu/test/run-pass/monomorphized-callees-with-ty-params-3314.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/closure-bounds-can-capture-chan.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/closure-bounds-can-capture-chan.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/intrinsic-atomics-cc.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/intrinsic-atomics-cc.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24779.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24779.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/monomorphize-abi-alignment.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/monomorphize-abi-alignment.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-4252.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-4252.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2735-3.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2735-3.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lint-expr-stmt-attrs-for-early-lints.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lint-expr-stmt-attrs-for-early-lints.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-return-TwoU8s.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-return-TwoU8s.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/default-method-supertrait-vtable.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/default-method-supertrait-vtable.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-0.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-0.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/once-move-out-on-heap.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/once-move-out-on-heap.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-vectorcall.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-vectorcall.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/linear-for-loop.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/linear-for-loop.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/owned-implies-static.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/owned-implies-static.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16671.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16671.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-26709.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-26709.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/one-tuple.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/one-tuple.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-44247.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-44247.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-use-both.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-use-both.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-20174.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-20174.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-seq-followed-by-seq.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-seq-followed-by-seq.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-12860.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-12860.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11820.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11820.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-39823.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-39823.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-unique-bind.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-unique-bind.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-stream.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-stream.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-47673.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-47673.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-infer-borrow-scope-view.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-infer-borrow-scope-view.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-enum-tuple.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-enum-tuple.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-20427.stage2-x86_64-unknown-linux-gnu.aux
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-20427.stage2-x86_64-unknown-linux-gnu.aux
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-13655.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-13655.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-41888.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-41888.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-8898.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-8898.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign2.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign2.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/blind-item-local-shadow.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/blind-item-local-shadow.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_trans_switchint.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_trans_switchint.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/new-unsafe-pointers.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/new-unsafe-pointers.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/instantiable.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/instantiable.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-3211.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-3211.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33687.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33687.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-at-most-once-rep.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-at-most-once-rep.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/autoderef-and-borrow-method-receiver.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/autoderef-and-borrow-method-receiver.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/crate-method-reexport-grrrrrrr.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/crate-method-reexport-grrrrrrr.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/path.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/path.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-48159.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-48159.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cci_impl_exe.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cci_impl_exe.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-15793.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-15793.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/private-class-field.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/private-class-field.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22463.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22463.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/iter-sum-overflow-overflow-checks.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/iter-sum-overflow-overflow-checks.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lexer-crlf-line-endings-string-literal-doc-comment.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lexer-crlf-line-endings-string-literal-doc-comment.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/new-box.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/new-box.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-20220.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-20220.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11382.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11382.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cleanup-rvalue-for-scope.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cleanup-rvalue-for-scope.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-crate-def-only.stage2-x86_64-unknown-linux-gnu.aux
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-crate-def-only.stage2-x86_64-unknown-linux-gnu.aux
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/estr-slice.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/estr-slice.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/running-with-no-runtime.running_with_no_runtime9.rcgu.o
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/running-with-no-runtime.running_with_no_runtime9.rcgu.o
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/borrowed-ptr-pattern-option.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/borrowed-ptr-pattern-option.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/infer-fn-tail-expr.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/infer-fn-tail-expr.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-40847.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-40847.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-47703-1.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-47703-1.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1460.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1460.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-6449.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-6449.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign-fn-with-byval.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign-fn-with-byval.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-12133-3.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-12133-3.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-12909.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-12909.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23992.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23992.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/discrim-explicit-23030.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/discrim-explicit-23030.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-inherent-prefer-over-trait.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-inherent-prefer-over-trait.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16597.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16597.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fsu-moves-and-copies.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fsu-moves-and-copies.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/guards-not-exhaustive.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/guards-not-exhaustive.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/class-poly-methods-cross-crate.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/class-poly-methods-cross-crate.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-42463.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-42463.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/method-self-arg-aux2.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/method-self-arg-aux2.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-tag-corruption.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-tag-corruption.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-10767.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-10767.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-9249.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-9249.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24954.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24954.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-3979-generics.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-3979-generics.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-29844.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-29844.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deref-on-ref.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deref-on-ref.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-escape-into-other-fn.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-escape-into-other-fn.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fat-lto.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fat-lto.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/for-loop-macro.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/for-loop-macro.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/exhaustive-bool-match-sanity.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/exhaustive-bool-match-sanity.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-project-from-type-param-via-bound-in-where.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-project-from-type-param-via-bound-in-where.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-var-hygiene.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-var-hygiene.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18232.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18232.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/let-destruct-ref.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/let-destruct-ref.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22426.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22426.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-struct-offsets.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-struct-offsets.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18110.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18110.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-31597.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-31597.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/export-non-interference3.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/export-non-interference3.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-14959.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-14959.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-autoderef.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-autoderef.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-relate-bound-regions-on-closures-to-inference-variables.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-relate-bound-regions-on-closures-to-inference-variables.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-17718-static-unsafe-interior.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-17718-static-unsafe-interior.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_fat_ptr.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_fat_ptr.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cfgs-on-items.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cfgs-on-items.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-bound-lists-feature-gate-2.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-bound-lists-feature-gate-2.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/inner-attrs-on-impl.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/inner-attrs-on-impl.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving-show-2.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving-show-2.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/autobind.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/autobind.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-27054-primitive-binary-ops.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-27054-primitive-binary-ops.rs-stage2-x86_64-unknown-linux-gnu.stamp
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/reexport-star.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/reexport-star.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-21909.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-21909.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/hygienic-labels-in-let.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/hygienic-labels-in-let.out
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-pass-TwoU64s.err
[01:00:26] DEBUG 2018-05-10T15:40:21Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-pass-TwoU64s.err
---
[01:00:26] test result: FAILED. 2959 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out
[01:00:26] 
[01:00:26] 
[01:00:26] 
[01:00:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:26] 
[01:00:26] 
[01:00:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:26] Build completed unsuccessfully in 0:14:14
[01:00:26] Build completed unsuccessfully in 0:14:14
[01:00:26] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06a029ae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
