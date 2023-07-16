plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/18/61/4e0f977cfe063188d73622a91cab8b8b409b662f422303fc687f362d941f/awscli-1.15.18-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 13.3MB/s eta 0:00:01
    1% |▌                               | 20kB 1.6MB/s eta 0:00:01
    2% |▉                               | 30kB 1.9MB/s eta 0:00:01
    3% |█                               | 40kB 1.8MB/s eta 0:00:01
---
[00:51:55] ...............i....................................................................................
[00:52:04] .................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:52:26] ...................................................................................
[00:52:47] .................................................................................ii.................
[00:53:33] .....F.......................................i....................................................i.
[00:53:47] ii.....................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:54:32] ......iiiiiii.......................................................................................
[00:54:52] ....................................................................................................
[00:55:11] ....................................................................................................
[00:55:29] ...............................................................................
[00:55:29] ...............................................................................
[00:55:29] failures:
[00:55:29] 
[00:55:29] ---- [run-pass] run-pass/rustc-rust-log.rs stdout ----
[00:55:29]  
[00:55:29] error: compilation failed!
[00:55:29] status: exit code: 101
[00:55:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rustc-rust-log.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/false, no_integrated_as: false, min_atomic_width: None, max_atomic_width: Some(64), panic_strategy: Unwind, abi_blacklist: [], crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: false, stack_probes: true, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, i128_lowering: false, codegen_backend: "llvm", default_hidden_visibility: false, embed_bitcode: false, emit_debug_gdb_scripts: true, requires_uwtable: false } }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc_target::spec: Got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", target_endian: "little", target_pointer_width: "64", target_c_int_width: "32", target_os: "linux", target_env: "gnu", target_vendor: "unknown", arch: "x86_64", data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128", linker_flavor: Gcc, options: TargetOptions { is_builtin: true, linker: None, pre_link_args: {Gcc: ["-Wl,--as-needed", "-Wl,-z,noexecstack", "-m64"]}, pre_link_objects_exe: [], pre_link_objects_dll: [], late_link_args: {}, post_link_objects: [], post_link_args: {}, link_env: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: "pic", code_model: None, tls_model: "global-dynamic", disable_redzone: false, eliminate_frame_pointer: true, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", target_family: Some("unix"), abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_lespace, sp: Span { lo: BytePos(234), hi: BytePos(235), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(235), hi: BytePos(300), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(300), hi: BytePos(301), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(301), hi: BytePos(364), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(364), hi: BytePos(365), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(365), hi: BytePos(429), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(429), hi: BytePos(430), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(430), hi: BytePos(465), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning whitespace: Some(TokenAndSpan { tok: Whitespace, sp: Span { lo: BytePos(465), hi: BytePos(467), ctxt: #0 } })
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::parse::lexer: scanning a comment Some(TokenAndSpan { tok: Comment, sp: Span { lo: BytePos(467), hi: BytePos(Data("{{GlobalMetaData::Krate}}"))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Krate}}"), disambiguator: 0 } }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Krate}}"), disambiguator: 0 } } <-> DefIndex(1:0)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::CrateDeps}}"))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CrateDeps}}"), disambiguator: 0 } }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::CrateDeps}}"), disambiguator:r::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Impls}}"), disambiguator: 0 } }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::Impls}}"), disambiguator: 0 } } <-> DefIndex(1:7)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(4294967295), data=GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"), disambiguator: 0 } }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: GlobalMetaData("{{GlobalMetaData::ExportedSymbols}}"), disambiguator: 0 } } <-> DefIndex(1:8)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: sline space=75)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(::)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp String('prelude')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(prelude,7) 7 (remaining line space=73)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(prelude)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=66)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(::)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp String('v1')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(v1,2) 2 (remaining line space=64)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(v1)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: INDENT 0
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: mk_printer 78
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(macro_use,9) 9 (remaining line space=78)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(macro_use)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: INDENT 0
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::def_collector: visit_item: Item { ident: #0, attrs: [], id: NodeId(1), node: Mod(Mod { inner: Span { lo: BytePos(496), hi: BytePos(508), ctxt: #0 }, items: [Item { ident: #0, attrs: [Attribute { id: AttrId(1), style: Outer, path: path(prelude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }], id: NodeId(2), node: Use(UseTree { prefix: path(std::prelude::v1), kind: Glob, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 }, tokens: None }, Item { ident: std#0, attrs: [Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }], id: NodeId(3), node: ExternCrate(None), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 }, tokens: None }, Item { id18-05-10T22:55:35Z: syntax::print::pp: pp String('prelude_import')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(prelude_import,14) 14 (remaining line space=78)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(prelude_import)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: INDENT 0
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: mk_printer 78
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp String('std')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(std,3) 3 (remaining line space=78)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(std)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp String('::')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(::,2) 2 (remaining line space=75)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(::)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp String('prelude')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(prelude,7) 7 (remaining line space=73)
[00:55:29]: #1 }, tokens: None }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(2), data=Misc, parent_def=DefIndex(0:0))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(2), data=Misc)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } } <-> DefIndex(0:1)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent: def_index_to_node[DefIndex(0:1) <-> NodeId(2)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: mk_printer 78
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp String('macro_use')/print Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print STR(macro_use,9) 9 (remaining line space=78)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: []
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: print String(macro_use)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: pp Vec<0,0>
[00:55:29] DEBUG 2018-05-10T22:55:35Z: syntax::print::pp: INDENT 0
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::def_collector: visit_item: Item { ident: std#0, attrs: [Attribute { id: AttrId(0), style: Outer, path: path(macro_use), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }], id: NodeId(3), node: ExternCrate(None), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 }, tokens: None }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(3), data=TypeNs("std"), parent_def=DefIndex(0:0))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent(parent=DefIndex(0:0), node_id=NodeId(3), data=TypeNs("std"))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: def_path_hash(DefIndex(0:0)) = DefPathHash(Fingerprint(11814548745822987508, 17045812851297804246))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent: after disambiguation, key = DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: DefPathTable::insert() - DefKey { parent: Some(DefIndex(0:0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } } <-> DefIndex(0:2)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: create_def_with_parent: def_index_to_node[DefIndex(0:2) <-> NodeId(3)
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::def_collector: visit_item: Item { ident: main#0, attrs: [], id: NodeId(4), node: Fn(FnDecl { inputs: [], output: Default(Span { lo: BytePos(506), hi: BytePos(506), ctxt: #0 }), variadic: false }, Normal, Spanned { node: NotConst, span: Span { lo: BytePos(496), hi: BytePos(498), ctxt: #0 } }, Rust, Generics { params: [], where_clause: WhereClause { id: NodeId(5), predicates: [], span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, Block { stmts: [], id: NodeId(6), rules: Default, span: Span { lo: BytePos(506), hi: BytePos(508), ctxt: #0 }, recovered: false }), vis: Spanned { node: Inherited, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } }, span: Span { lo: BytePos(496), hi: BytePos(508), ctxt: #0 }, tokens: Some(TokenStream { kind: Stream([TokenStream { kind: Tree(Token(Span { lo: BytePos(496), hi: BytePos(498), ctxt: #0 }, Ident(fn#0, false))) }, TokenStream { kind: Tree(Token(Span { lo: BytePos(499), hi: BytePos(503), ctxt: #0 }, Ident(main#0, false))) }, TokenStream { kind: Tree(Delimited(Span { lo: BytePos(503), hi: BytePos(505), ctxt: #0 }, Delimited { delim: Paren, tts: ThinTokenStream(None) })) }, TokenStream { kind: Tree(Delimited(Span { lo: BytePos(506), hi: BytePos(508), ctxt: #0 }, Delimited { delim: Brace, tts: ThinTokenStream(None) })) }]) }) }
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::def_collector: create_def(node_id=NodeId(4), data=ValueNs("main"), parent_def=DefIndex(0:0))
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::hir::map::definitions: createude_import), tokens: TokenStream { kind: Empty }, is_sugared_doc: false, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #1 } } as used.
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc_metadata::creader: resolving extern crate stmt. ident: std orig_name: None
[00:55:29]  INFO 2018-05-10T22:55:35Z: rustc_metadata::creader: resolving crate `extern crate std as std`
[00:55:29]  INFO 2018-05-10T22:55:35Z: rustc_metadata::creader: falling back to a load
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dropck-eyepatch.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dropck-eyepatch.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-call-deep2.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-call-deep2.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-20371.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-20371.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-enum-byref.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-enum-byref.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-infer-borrow-scope-within-loop-ok.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-infer-borrow-scope-within-loop-ok.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-9129.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-9129.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/move-3-unique.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/move-3-unique.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24434.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24434.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23833.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23833.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/blind-item-local-shadow.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/blind-item-local-shadow.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/anon-extern-mod-cross-crate-2.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/anon-extern-mod-cross-crate-2.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-ivec-leak.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-ivec-leak.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2748-a.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2748-a.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binary-minus-without-space.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binary-minus-without-space.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/class-separate-impl.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/class-separate-impl.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23699.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23699.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_boxing.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_boxing.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mod_file.rs-stage2-x86: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-4252.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-4252.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2735-3.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2735-3.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lint-expr-stmt-attrs-for-early-lints.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lint-expr-stmt-attrs-for-early-lints.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-return-TwoU8s.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-return-TwoU8s.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/default-method-supertrait-vtable.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/default-method-supertrait-vtable.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-0.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-0.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/once-move-out-on-heap.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/once-move-out-on-heap.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-vectorcall.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-vectorcall.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/linear-for-loop.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/linear-for-loop.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/owned-implies-static.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/owned-implies-static.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16671.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16671.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-26709.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-26709.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/one-tuple.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/one-tuple.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-44247.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-44247.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-p testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign2.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign2.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/blind-item-local-shadow.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/blind-item-local-shadow.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_trans_switchint.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_trans_switchint.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/new-unsafe-pointers.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/new-unsafe-pointers.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/instantiable.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/instantiable.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /chlesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/crate-method-reexport-grrrrrrr.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/path.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/path.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-48159.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-48159.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cci_impl_exe.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cci_impl_exe.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-15793.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-15793.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-borrow-evec-uniq.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-borrow-evec-uniq.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-lifetime.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-lifetime.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cfg-macros-notfoo.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cfg-macros-notfoo.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-38556.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-38556.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/liveness-move-in-loop.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/liveness-move-in-loop.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/private-class-field.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/private-class-field.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22463.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22463.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/iter-sum-overflow-overflow-checks.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/iter-sum-overflow-overflow-checks.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lexer-crlf-line-endings-string-literal-doc-comment.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lexer-crlf-line-endings-string-literal-doc-comment.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/new-box.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/new-box.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-20220.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-20220.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11382.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11382.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cleanup-rvalue-for-scope.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cleanup-rvalue-for-scope.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-crate-def-only.stage2-x86_64-unknown-linux-gnu.aux
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-crate-def-only.stage2-x86_64-unk/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-40847.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-40847.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-47703-1.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-47703-1.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1460.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1460.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-6449.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-6449.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign-fn-with-byval.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign-fn-with-byval.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-12133-3.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16597.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fsu-moves-and-copies.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fsu-moves-and-copies.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/guards-not-exhaustive.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/guards-not-exhaustive.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/class-poly-methods-cross-crate.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/class-poly-methods-cross-crate.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-42463.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-42463.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/method-self-arg-aux2.err
[00:55:29] DEBUG 20heckout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/for-loop-macro.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/for-loop-macro.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/exhaustive-bool-match-sanity.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/exhaustive-bool-match-sanity.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-project-from-type-param-via-bound-in-where.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-project-from-type-param-via-bound-in-where.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-var-hygiene.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-var-hygiene.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18232.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18232.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/let-destruct-ref.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/let-destruct-ref.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22426.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22426.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-struct-offsets.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-struct-offsets.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18110.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18110.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-31597.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-31597.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/export-non-interference3.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/export-non-interference3.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-14959.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-14959.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-autoderef.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-autoderef.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-relate-bound-regions-on-closures-to-inference-variables.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-relate-bound-regions-on-closures-to-inference-variables.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-17718-static-unsafe-interior.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-17718-static-unsafe-interior.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_fat_ptr.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_fat_ptr.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cfgs-on-items.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cfgs-on-items.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-bound-lists-feature-gate-2.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-bound-lists-feature-gate-2.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/inner-attrs-on-impl.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejectenu/test/run-pass/newtype-struct-xc.stage2-x86_64-unknown-linux-gnu.aux
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-9951.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-9951.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/coherence-bigint-vecint.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/coherence-bigint-vecint.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fn-item-type-coerce.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fn-item-type-coerce.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-block-cross-crate-fn.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-block-cross-crate-fn.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-26205.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-26205.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-simple.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-simple.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-21212.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-issue-21212.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-static-methods.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-static-methods.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/multiple-trait-bounds.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/multiple-trait-bounds.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-41298.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-41298.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11225-1.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11225-1.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/i128.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/i128.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18652.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18652.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rec.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rec.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24085.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-24085.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-type-synonym.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generic-type-synonym.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/intrinsic-atomics.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/intrinsic-atomics.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/hrtb-resolve-lifetime.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/hrtb-resolve-lifetime.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-19811-escape-unicode.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-19811-escape-unicode.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-20414.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-20414.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-1.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-1.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22864-1.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22864-1.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2633.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2633.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/paths-containing-nul.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/paths-containing-nul.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testheckout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-22814.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/enum-discrim-autosizing.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/enum-discrim-autosizing.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2633.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2633.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1701.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1701.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/overlap-permitted-for-marker-traits.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/overlap-permitted-for-marker-traits.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-lifetime-used-with-labels.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_adt_construction.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_adt_construction.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-self-in-enums.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-self-in-enums.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/running-with-no-runtime.crate.allocator.rcgu.o
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/running-with-no-runtime.crate.allocator.rcgu.o
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/enum-non-c-like-repr-c-and-int.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/enum-non-c-like: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33387.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-30615.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-30615.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-infer-call-2.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-infer-call-2.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fn-bare-assign.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fn-bare-assign.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-4865-2.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-4865-2.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-21891.err
[sion::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-tuple-struct.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-delimiter-significance.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-delimiter-significance.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2611-3.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2611-3.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-use-all.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-use-all.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fn-pattern-expected-type.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fn-pattern-expected-type.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-21909.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rbreak.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/break.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-30615.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-30615.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/overloaded-autoderef-count.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/overloaded-autoderef-count.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-export-inner-module.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-export-inner-module.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-46095.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-46095.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-19129-1.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-19129-1.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-46959.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-46959.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1857-drop-order.rfc1857_drop_order10.rcgu.o
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1857-drop-order.rfc1857_drop_order10.rcgu.o
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/overloaded_deref_with_ref_pattern_issue15609.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/overloaded_deref_with_ref_pattern_issue15609.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-enum-byref.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-enum-byref.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-no-variance-from-fn-generics.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-no-variance-from-fn-generics.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lazy-and-or.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lazy-and-or.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-36260.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-36260.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-reassign-let-bound-pointer.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-reassign-let-bound-pointer.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arith-unsigned.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arith-unsigned.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11709.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11709.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-ref-binding.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-ref-binding.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/ivec-tag.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/ivec-tag.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/env-funky-keys.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/env-funky-keys.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-32008.err
[00:55:29] DEBUG 2sion::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/project-cache-issue-37154.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/project-cache-issue-37154.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-meta-items.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-meta-items.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-tuple-struct.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-tuple-struct.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-43910.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-43910.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/coerce-overloaded-autoderef.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/coerce-overloaded-autoderef.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-42552.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-42552.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-vectorcall.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-vectorcall.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-simple.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-simple.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-borrow-uniq.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-borrow-uniq.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-4759.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-4759.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-nt-list.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-nt-list.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/float-literal-inference.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/float-literal-inference.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lint-non-camel-case-types-non-uppercase-statics-unicode.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lint-non-camel-case-types-non-uppercase-statics-unicode.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/into-iterator-type-inference-shift.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/into-iterator-type-inference-shift.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-free-region-outlives-static-outlives-free-region.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-free-region-outlives-static-outlives-free-region.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/last-use-in-block.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/last-use-in-block.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_calls_to_shims.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir_calls_to_shims.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16783.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-16783.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-21384.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-21384.rs-stage2-x86_64x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-5280.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-5280.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dst-raw.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dst-raw.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-20389.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-20389.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-struct.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-struct.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-34053.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-34053.rs-stageow-scope-addr-of.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11958.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-11958.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-6117.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-6117.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro_with_super_2.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro_with_super_2.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2063.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2063.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-28279.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-28279.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-27105.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-27105.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/option-unwrap.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/option-unwrap.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-19358.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-19358.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/nested_item_main.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/nested_item_main.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cci_capture_clause.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cci_capture_clause.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23968-const-not-overflow.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-23968-const-not-overflow.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-13872.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-13872.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-path.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-path.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign-int-types.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/foreign-int-types.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/import5.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/import5.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-resolve-lifetime.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types-resolve-lifetime.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-take-value.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-take-value.out
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-5518.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-5518.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cfg-attr-crate.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cfg-attr-crate.rs-stage2-x86_64-unknown-linux-gnu.stamp
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/crt-static-on-works.out
[00:55:29] DEBUG 2heckout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-2414-c.err
[00:55:29] DEBUG 2018-05-10T22:55:35Z: rustc::session::filesearch: testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-3904.rs-stage2-x86_64-unknown-linux-gnu.stamp
---
[00:55:29] test result: FAILED. 2959 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out
[00:55:29] 
[00:55:29] 
[00:55:29] 
[00:55:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "alwa3641936 .
2752776 ./obj/build
1977756 ./obj/build/x86_64-unknown-linux-gnu
725596 ./src
584812 ./obj/build/bootstrap
---
149760 ./.git/modules
149756 ./.git/modules/src
149116 ./src/llvm-emscripten/test
124332 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b
124328 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b/s-f0x22uc5sm-xumldg-22v3gsapbxtg7
112620 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
112616 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
108624 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
103728 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp
103728 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp
103724 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp/s-f0x36vpjrf-o2ocm6-1nozvcbv9keul
98500 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
93512 ./obj/build/x86_64-unknown-linux-gnu/stage1
93488 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90848 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90848 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90844 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0x34il8w7-19a0omf-2w93x474pe4e6
87816 ./obj/build/x86_64-unknown-linux-gnu/doc/core
81448 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
80984 ./obj/build/x86_64-unknown-linux-gnu/doc/std
79036 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
