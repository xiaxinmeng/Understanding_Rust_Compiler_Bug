plain
[01:23:38] failures:
[01:23:38] 
[01:23:38] ---- [run-pass] run-pass/rustc-rust-log.rs stdout ----
[01:23:38] 
[01:23:38] error: failed to decode compiler output as json: `key must be a string at line 1 column 2`
[01:23:38] line: {std::rt::lang_start::<()>}
[01:23:38] output: DEBUG 2018-11-24T04:28:53Z: rustc_target::spec: Got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", target_endian: "little", target_pointer_width: "64", target_c_int_width: "32", target_os: "linux", target_env: "gnu", target_vendor: "unknown", arch: "x86_64", data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128", linker_flavor: Gcc, options: TargetOptions { is_builtin: true, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-Wl,--as-needed", "-Wl,-z,noexecstack", "-m64"]}, pre_link_args_crt: {}, pre_link_objects_exe: [], pre_link_objects_exe_crt: [], pre_link_objects_dll: [], late_link_args: {}, post_link_objects: [], post_link_objects_crt: [], post_link_args: {}, link_env: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: "pic", code_model: None, tls_model: "global-dynamic", disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", target_family: Some("unix"), abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_android: false, is_like_emscripten: false, is_like_fuchsia: false, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, custom_unwind_resume: false, has_elf_tls: true, obj_is_bitcode: false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Unwind, abi_blacklist: [], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: true, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: false, simd_types_indirect: true, override_export_symbols: None } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc_target::spec: Got builtin target: Target { llvm_target: "arm-linux-androideabi", target_endian: "little", target_pointer_width: "32", target_c_int_width: "32", target_os: "android", target_env: "", target_vendor: "unknown", arch: "arm", data_layout: "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64", linker_flavor: Gcc, options: TargetOptions { is_builtin: true, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-Wl,--as-needed", "-Wl,-z,noexecstack", "-Wl,--allow-multiple-definition"]}, pre_link_args_crt: {}, pre_link_objects_exe: [], pre_link_objects_exe_crt: [], pre_link_objects_dll: [], late_link_args: {}, post_link_objects: [], post_link_objects_crt: [], post_link_args: {}, link_env: [], asm_args: [], cpu: "generic", features: "+strict-align,+v5te", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: "pic", code_model: None, tls_model: "global-dynamic", disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", target_family: Some("unix"), abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_android: true, is_like_emscripten: false, is_like_fuchsia: false, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, custom_unwind_resume: false, has_elf_tls: false, obj_is_bitcode: false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(32), atomic_cas: true, panic_strategy: Unwind, abi_blacklist: [Stdcall, Fastcall, Vectorcall, Thiscall, Win64, SysV64], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: false, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: true, simd_types_indirect: true, override_export_symbols: None } }
[01:23:38]  INFO 2018-11-24T04:28:53Z: jobserver::imp: created a jobserver: Client { read: File { fd: 3, path: "pipe:[265399]", read: true, write: false }, write: File { fd: 4, path: "pipe:[265399]", read: false, write: true } }
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_driver: codegen backend candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_driver: probing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends for a codegen backend
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(0), hi: BytePos(64), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(64), hi: BytePos(65), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(65), hi: BytePos(127), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(127), hi: BytePos(128), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(128), hi: BytePos(162), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(162), hi: BytePos(163), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(163), hi: BytePos(165), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(165), hi: BytePos(166), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(166), hi: BytePos(234), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(234), hi: BytePos(235), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(235), hi: BytePos(300), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(300), hi: BytePos(301), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(301), hi: BytePos(364), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(364), hi: BytePos(365), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(365), hi: BytePos(429), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(429), hi: BytePos(430), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(430), hi: BytePos(465), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(465), hi: BytePos(467), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(467), hi: BytePos(535), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(535), hi: BytePos(536), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(536), hi: BytePos(602), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(602), hi: BytePos(603), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(603), hi: BytePos(671), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(671), hi: BytePos(672), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(672), hi: BytePos(736), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(736), hi: BytePos(737), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(737), hi: BytePos(751), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(751), hi: BytePos(752), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(752), hi: BytePos(754), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(754), hi: BytePos(755), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(755), hi: BytePos(784), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(784), hi: BytePos(785), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(785), hi: BytePos(814), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(814), hi: BytePos(816), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(816), hi: BytePos(843), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(843), hi: BytePos(845), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(847), hi: BytePos(848), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(854), hi: BytePos(855), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(857), hi: BytePos(858), ctxt: #0 } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::source_map: byte pos BytePos(845) is on the line at byte pos BytePos(845)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::source_map: char pos CharPos(845) is on the line at char pos CharPos(845)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::source_map: byte is on line: 22
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::attr: parse_outer_attributes: self.token=Ident(fn#0, false)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::parse::attr: parse_outer_attributes: self.token=Eof
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::lint::context: early context: enter_attrs([])
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(prelude_import)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::lint::context: early context: enter_attrs([Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }])
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(prelude_import)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::lint::context: early context: exit_attrs([Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }])
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(macro_use,9) 9 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(macro_use)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::lint::context: early context: enter_attrs([Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }])
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(macro_use,9) 9 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(macro_use)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::lint::context: early context: exit_attrs([Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }])
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::lint::context: early context: enter_attrs([])
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::lint::context: early context: exit_attrs([])
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::lint::context: early context: exit_attrs([])
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } } <-> DefIndex(0:0)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::Krate}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Krate}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Krate}}"), disambiguator: 0 } } <-> DefIndex(1:0)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::CrateDeps}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CrateDeps}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CrateDeps}}"), disambiguator: 0 } } <-> DefIndex(1:1)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::DylibDependencyFormats}}"), disambiguator: 0 } } <-> DefIndex(1:2)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::LangItems}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItems}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItems}}"), disambiguator: 0 } } <-> DefIndex(1:3)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::LangItemsMissing}}"), disambiguator: 0 } } <-> DefIndex(1:4)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::NativeLibraries}}"), disambiguator: 0 } } <-> DefIndex(1:5)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::SourceMap}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::SourceMap}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::SourceMap}}"), disambiguator: 0 } } <-> DefIndex(1:6)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::Impls}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Impls}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Impls}}"), disambiguator: 0 } } <-> DefIndex(1:7)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967040), data=GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"), disambiguator: 0 } } <-> DefIndex(1:8)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::source_map: byte pos BytePos(845) is on the line at byte pos BytePos(845)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::source_map: char pos CharPos(845) is on the line at char pos CharPos(845)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::source_map: byte is on line: 22
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(prelude_import)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('std')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(std,3) 3 (remaining line space=76)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(std)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=73)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('prelude')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(prelude,7) 7 (remaining line space=71)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(prelude)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=64)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('v1')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(v1,2) 2 (remaining line space=62)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(v1)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(macro_use,9) 9 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(macro_use)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::def_collector: visit_item: Item { ident: #0, attrs: [], id: NodeId(1), node: Mod(Mod { inner: Span { lo: BytePos(845), hi: BytePos(857), ctxt: #0 }, items: [Item { ident: #0, attrs: [Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }], id: NodeId(2), node: Use(UseTree { prefix: path(::std::prelude::v1), kind: Glob, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 }, tokens: None }, Item { ident: std#0, attrs: [Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }], id: NodeId(8), node: ExternCrate(None), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 }, tokens: None }, Item { ident: main#0, attrs: [], id: NodeId(10), node: Fn(FnDecl { inputs: [], output: Default(Span { lo: BytePos(855), hi: BytePos(855), ctxt: #0 }), variadic: false }, FnHeader { unsafety: Normal, asyncness: NotAsync, constness: Spanned { node: NotConst, span: Span { lo: BytePos(845), hi: BytePos(847), ctxt: #0 } }, abi: Rust }, Generics { params: [], where_clause: WhereClause { id: NodeId(11), predicates: [], span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, Block { stmts: [], id: NodeId(12), rules: Default, span: Span { lo: BytePos(855), hi: BytePos(857), ctxt: #0 }, recovered: false }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(845), hi: BytePos(845), ctxt: #0 } }, span: Span { lo: BytePos(845), hi: BytePos(857), ctxt: #0 }, tokens: Some(TokenStream { kind: Stream([TokenStream { kind: Tree(Token(Span { lo: BytePos(845), hi: BytePos(847), ctxt: #0 }, Ident(fn#0, false))) }, TokenStream { kind: Tree(Token(Span { lo: BytePos(848), hi: BytePos(852), ctxt: #0 }, Ident(main#0, false))) }, TokenStream { kind: Tree(Delimited(DelimSpan { open: Span { lo: BytePos(852), hi: BytePos(853), ctxt: #0 }, close: Span { lo: BytePos(853), hi: BytePos(854), ctxt: #0 } }, Delimited { delim: Paren, tts: ThinTokenStream(None) })) }, TokenStream { kind: Tree(Delimited(DelimSpan { open: Span { lo: BytePos(855), hi: BytePos(856), ctxt: #0 }, close: Span { lo: BytePos(856), hi: BytePos(857), ctxt: #0 } }, Delimited { delim: Brace, tts: ThinTokenStream(None) })) }]) }) }], inline: true }), vis: Spanned { node: Public, span: Span { lo: BytePos(845), hi: BytePos(845), ctxt: #0 } }, span: Span { lo: BytePos(845), hi: BytePos(857), ctxt: #0 }, tokens: None }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(prelude_import)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('std')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(std,3) 3 (remaining line space=76)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(std)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=73)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('prelude')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(prelude,7) 7 (remaining line space=71)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(prelude)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=64)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('v1')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(v1,2) 2 (remaining line space=62)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(v1)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::def_collector: visit_item: Item { ident: #0, attrs: [Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }], id: NodeId(2), node: Use(UseTree { prefix: path(::std::prelude::v1), kind: Glob, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 }, tokens: None }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(2), data=Misc, parent_def=DefIndex(0:0))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(2), data=Misc)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } } <-> DefIndex(0:1)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: def_index_to_node[DefIndex(0:1) <-> NodeId(2)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(macro_use,9) 9 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(macro_use)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::def_collector: visit_item: Item { ident: std#0, attrs: [Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }], id: NodeId(8), node: ExternCrate(None), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 }, tokens: None }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(8), data=TypeNs("std"), parent_def=DefIndex(0:0))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(8), data=TypeNs("std"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } } <-> DefIndex(0:2)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: def_index_to_node[DefIndex(0:2) <-> NodeId(8)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::def_collector: visit_item: Item { ident: main#0, attrs: [], id: NodeId(10), node: Fn(FnDecl { inputs: [], output: Default(Span { lo: BytePos(855), hi: BytePos(855), ctxt: #0 }), variadic: false }, FnHeader { unsafety: Normal, asyncness: NotAsync, constness: Spanned { node: NotConst, span: Span { lo: BytePos(845), hi: BytePos(847), ctxt: #0 } }, abi: Rust }, Generics { params: [], where_clause: WhereClause { id: NodeId(11), predicates: [], span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, Block { stmts: [], id: NodeId(12), rules: Default, span: Span { lo: BytePos(855), hi: BytePos(857), ctxt: #0 }, recovered: false }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(845), hi: BytePos(845), ctxt: #0 } }, span: Span { lo: BytePos(845), hi: BytePos(857), ctxt: #0 }, tokens: Some(TokenStream { kind: Stream([TokenStream { kind: Tree(Token(Span { lo: BytePos(845), hi: BytePos(847), ctxt: #0 }, Ident(fn#0, false))) }, TokenStream { kind: Tree(Token(Span { lo: BytePos(848), hi: BytePos(852), ctxt: #0 }, Ident(main#0, false))) }, TokenStream { kind: Tree(Delimited(DelimSpan { open: Span { lo: BytePos(852), hi: BytePos(853), ctxt: #0 }, close: Span { lo: BytePos(853), hi: BytePos(854), ctxt: #0 } }, Delimited { delim: Paren, tts: ThinTokenStream(None) })) }, TokenStream { kind: Tree(Delimited(DelimSpan { open: Span { lo: BytePos(855), hi: BytePos(856), ctxt: #0 }, close: Span { lo: BytePos(856), hi: BytePos(857), ctxt: #0 } }, Delimited { delim: Brace, tts: ThinTokenStream(None) })) }]) }) }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(10), data=ValueNs("main"), parent_def=DefIndex(0:0))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(10), data=ValueNs("main"))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 } }
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 } } <-> DefIndex(0:3)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::hir::map::definitions: create_def_with_parent: def_index_to_node[DefIndex(0:3) <-> NodeId(10)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('std')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(std,3) 3 (remaining line space=76)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(std)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=73)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('prelude')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(prelude,7) 7 (remaining line space=71)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(prelude)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=64)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(::)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('v1')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(v1,2) 2 (remaining line space=62)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(v1)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc_resolve::build_reduced_graph: build_reduced_graph_for_use_tree(parent_prefix=[], use_tree=UseTree { prefix: path(::std::prelude::v1), kind: Glob, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }, nested=false)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc_resolve::build_reduced_graph: build_reduced_graph_for_use_tree: prefix=[Segment { ident: {{root}}#0, id: Some(NodeId(4)) }, Segment { ident: std#0, id: Some(NodeId(5)) }, Segment { ident: prelude#0, id: Some(NodeId(6)) }, Segment { ident: v1#0, id: Some(NodeId(7)) }]
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: mk_printer 78
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: []
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: print String(prelude_import)
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: pp Vec<0,0>
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::print::pp: INDENT 0
[01:23:38] DEBUG 2018-11-24T04:28:53Z: syntax::attr: Marking Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } } as used.
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc_resolve::resolve_imports: add_import_directive(ImportDirective { id: NodeId(2), root_id: NodeId(2), span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 }, root_span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 }, parent_scope: ParentScope { module: Some(Mod(DefId(0/0:0))), expansion: Mark(0), legacy: Empty, derives: [] }, module_path: [Segment { ident: {{root}}#0, id: Some(NodeId(4)) }, Segment { ident: std#0, id: Some(NodeId(5)) }, Segment { ident: prelude#0, id: Some(NodeId(6)) }, Segment { ident: v1#0, id: Some(NodeId(7)) }], imported_module: Cell { value: None }, subclass: GlobImport { is_prelude: true, max_vis: Cell { value: Invisible } }, vis: Cell { value: Restricted(DefId(0/0:0)) }, used: Cell { value: false } })
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving extern crate stmt. ident: std orig_name: None
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving crate `extern crate std as std`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: falling back to a load
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = arm-linux-androideabi
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log/auxiliary
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: filesearch: searching lib path
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.so
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: reading "libstd-58ef9b88070440c1.rlib" => 131.22µs
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: register crate `extern crate std as std`
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving deps of external crate
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving dep crate core hash: `5a8fdb9002f13ec8` extra filename: `-5099b14b912fd91d`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving crate `extern crate core as core`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: falling back to a load
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = arm-linux-androideabi
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log/auxiliary
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: filesearch: searching lib path
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.so
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: reading "libcore-5099b14b912fd91d.rlib" => 39.598µs
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: register crate `extern crate core as core`
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving deps of external crate
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving dep crate compiler_builtins hash: `61c8ee34aac2b341` extra filename: `-8bf7d2f0baab06f7`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving crate `extern crate compiler_builtins as compiler_builtins`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: falling back to a load
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = arm-linux-androideabi
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log/auxiliary
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: filesearch: searching lib path
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.so
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.so
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: reading "libcompiler_builtins-8bf7d2f0baab06f7.rlib" => 127.819µs
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: register crate `extern crate compiler_builtins as compiler_builtins`
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving deps of external crate
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving dep crate core hash: `5a8fdb9002f13ec8` extra filename: `-5099b14b912fd91d`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving crate `extern crate core as core`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving dep crate alloc hash: `b775058310ab68eb` extra filename: `-c0a6a180cc46432e`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving crate `extern crate alloc as alloc`
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: falling back to a load
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = arm-linux-androideabi
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log/auxiliary
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: filesearch: searching lib path
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:38] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.so
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.so
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: reading "liballoc-c0a6a180cc46432e.rlib" => 43.39µs
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: register crate `extern crate alloc as alloc`
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving deps of external crate
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving dep crate core hash: `5a8fdb9002f13ec8` extra filename: `-5099b14b912fd91d`
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving crate `extern crate core as core`
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving dep crate compiler_builtins hash: `61c8ee34aac2b341` extra filename: `-8bf7d2f0baab06f7`
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving crate `extern crate compiler_builtins as compiler_builtins`
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving dep crate libc hash: `27d3fbddc1ed251f` extra filename: `-cba975f55ee8cde5`
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: resolving crate `extern crate libc as libc`
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::creader: falling back to a load
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = arm-linux-androideabi
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log/auxiliary
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: filesearch: searching lib path
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libterm-2f242eeb17e9987f.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libgetopts-06aa843e0c3e3e68.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liballoc-c0a6a180cc46432e.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_unwind-5f74681417baec17.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:39]  INFO 2018-11-24T04:28:53Z: rustc_metadata::locator: lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/liblibc-cba975f55ee8cde5.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-8bf7d2f0baab06f7.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcore-5099b14b912fd91d.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libpanic_abort-34648e5933d70d10.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libunwind-11272fcfb3abefd9.rlib
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-58ef9b88070440c1.so
[01:23:39] DEBUG 2018-11-24T04:28:53Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-7c18c6e3a7535b86.so
---
[01:23:44] test result: FAILED. 2861 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[01:23:44] 
[01:23:44] 
[01:23:44] 
[01:23:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "run-pass" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:23:44] 
[01:23:44] 
[01:23:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:23:44] Build completed unsuccessfully in 1:12:47
---
travis_time:end:05c53f68:start=1543033916139253793,finish=1543033916155532368,duration=16278575
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07ac97be
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:118aebcd
travis_time:start:118aebcd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d0498ed
$ dmesg | grep -i kill
