plain
failures:

---- [incremental] incremental/rustc-rust-log.rs stdout ----

error in revision `rpass1`: auxiliary build of "/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs" failed to compile: 
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
DEBUG rustc_parse::lexer next_token: Ident("rpass1")
DEBUG rustc_target::spec got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", pointer_width: 64, arch: "x86_64", data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128", options: TargetOptions { is_builtin: true, endian: little, c_int_width: "32", os: "linux", env: "gnu", abi: "", vendor: "unknown", linker_flavor: Gcc, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-m64"]}, pre_link_objects: {}, post_link_objects: {}, pre_link_objects_fallback: {}, post_link_objects_fallback: {}, crt_objects_fallback: None, late_link_args: {}, late_link_args_dynamic: {}, late_link_args_static: {}, post_link_args: {}, link_script: None, link_env: [], link_env_remove: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: Pic, code_model: None, tls_model: GeneralDynamic, disable_redzone: false, frame_pointer: MayOmit, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", families: ["unix"], abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_emscripten: false, is_like_fuchsia: false, is_like_wasm: false, dwarf_version: None, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, static_position_independent_executables: false, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, main_needs_argc_argv: true, has_elf_tls: true, obj_is_bitcode: false, forces_embed_bitcode: false, bitcode_llvm_cmdline: "", min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Unwind, crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: true, stack_probes: Call, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, default_hidden_visibility: false, emit_debug_gdb_scripts: true, requires_uwtable: false, default_uwtable: false, simd_types_indirect: true, limit_rdylib_exports: true, override_export_symbols: None, merge_functions: Aliases, mcount: "mcount", llvm_abiname: "", relax_elf_relocations: false, llvm_args: [], use_ctors_section: false, eh_frame_header: true, has_thumb_interworking: false, split_debuginfo: Off, supported_sanitizers: ADDRESS | LEAK | MEMORY | THREAD, default_adjusted_cabi: None, c_enum_min_bits: 32 } }
DEBUG rustc_target::spec got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", pointer_width: 64, arch: "x86_64", data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128", options: TargetOptions { is_builtin: true, endian: little, c_int_width: "32", os: "linux", env: "gnu", abi: "", vendor: "unknown", linker_flavor: Gcc, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-m64"]}, pre_link_objects: {}, post_link_objects: {}, pre_link_objects_fallback: {}, post_link_objects_fallback: {}, crt_objects_fallback: None, late_link_args: {}, late_link_args_dynamic: {}, late_link_args_static: {}, post_link_args: {}, link_script: None, link_env: [], link_env_remove: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: Pic, code_model: None, tls_model: GeneralDynamic, disable_redzone: false, frame_pointer: MayOmit, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", families: ["unix"], abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_emscripten: false, is_like_fuchsia: false, is_like_wasm: false, dwarf_version: None, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, static_position_independent_executables: false, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, main_needs_argc_argv: true, has_elf_tls: true, obj_is_bitcode: false, forces_embed_bitcode: false, bitcode_llvm_cmdline: "", min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Unwind, crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: true, stack_probes: Call, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, default_hidden_visibility: false, emit_debug_gdb_scripts: true, requires_uwtable: false, default_uwtable: false, simd_types_indirect: true, limit_rdylib_exports: true, override_export_symbols: None, merge_functions: Aliases, mcount: "mcount", llvm_abiname: "", relax_elf_relocations: false, llvm_args: [], use_ctors_section: false, eh_frame_header: true, has_thumb_interworking: false, split_debuginfo: Off, supported_sanitizers: ADDRESS | LEAK | MEMORY | THREAD, default_adjusted_cabi: None, c_enum_min_bits: 32 } }
DEBUG rustc_parse::lexer next_token: LineComment { doc_style: None }("// rustc-env:RUSTC_LOG=debug")
DEBUG rustc_parse::lexer next_token: Whitespace("\n")
DEBUG rustc_parse::lexer next_token: Pound("#")
DEBUG rustc_parse::lexer next_token: OpenBracket("[")
DEBUG rustc_parse::lexer next_token: Ident("cfg")
DEBUG rustc_parse::lexer next_token: OpenParen("(")
DEBUG rustc_parse::lexer next_token: Ident("rpass1")
DEBUG rustc_parse::lexer next_token: CloseParen(")")
DEBUG rustc_parse::lexer next_token: CloseBracket("]")
DEBUG rustc_parse::lexer next_token: Whitespace("\n")
DEBUG rustc_parse::lexer next_token: Ident("pub")
DEBUG rustc_parse::lexer next_token: Whitespace(" ")
DEBUG rustc_parse::lexer next_token: Ident("fn")
DEBUG rustc_parse::lexer next_token: Whitespace(" ")
DEBUG rustc_parse::lexer next_token: Ident("foo")
DEBUG rustc_parse::lexer next_token: OpenParen("(")
DEBUG rustc_parse::lexer next_token: CloseParen(")")
DEBUG rustc_parse::lexer next_token: Whitespace(" ")
DEBUG rustc_parse::lexer next_token: OpenBrace("{")
DEBUG rustc_parse::lexer next_token: CloseBrace("}")
DEBUG rustc_parse::lexer next_token: Whitespace("\n\n")
DEBUG rustc_parse::lexer next_token: Pound("#")
DEBUG rustc_parse::lexer next_token: OpenBracket("[")
DEBUG rustc_parse::lexer next_token: Ident("cfg")
DEBUG rustc_parse::lexer next_token: OpenParen("(")
DEBUG rustc_parse::lexer next_token: Ident("rpass2")
DEBUG rustc_parse::lexer next_token: CloseParen(")")
DEBUG rustc_parse::lexer next_token: CloseBracket("]")
DEBUG rustc_parse::lexer next_token: Whitespace("\n")
DEBUG rustc_parse::lexer next_token: Ident("pub")
DEBUG rustc_parse::lexer next_token: Whitespace(" ")
DEBUG rustc_parse::lexer next_token: Ident("fn")
DEBUG rustc_parse::lexer next_token: Whitespace(" ")
DEBUG rustc_parse::lexer next_token: Ident("foo")
DEBUG rustc_parse::lexer next_token: OpenParen("(")
DEBUG rustc_parse::lexer next_token: CloseParen(")")
DEBUG rustc_parse::lexer next_token: Whitespace(" ")
DEBUG rustc_parse::lexer next_token: OpenBrace("{")
DEBUG rustc_parse::lexer next_token: Whitespace("\n    ")
DEBUG rustc_parse::lexer next_token: Ident("println")
DEBUG rustc_parse::lexer next_token: Bang("!")
DEBUG rustc_parse::lexer next_token: OpenParen("(")
DEBUG rustc_parse::lexer next_token: CloseParen(")")
DEBUG rustc_parse::lexer next_token: Semi(";")
DEBUG rustc_parse::lexer next_token: Whitespace("\n")
DEBUG rustc_parse::lexer next_token: CloseBrace("}")
DEBUG rustc_parse::lexer next_token: Whitespace("\n")
DEBUG rustc_parse::parser::attr parse_attribute: inner_parse_policy=Forbidden { reason: "an inner attribute is not permitted in this context", saw_doc_comment: false, prev_attr_sp: None } self.token=Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:2 (#0) }
DEBUG rustc_parse::parser::attr parse_attribute: inner_parse_policy=Forbidden { reason: "an inner attribute is not permitted in this context", saw_doc_comment: false, prev_attr_sp: None } self.token=Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }
DEBUG rustc_parse::parser::diagnostics check_trailing_angle_brackets: parsed_angle_bracket_args=false
DEBUG rustc_incremental::persist::fs prepare_session_directory
DEBUG rustc_incremental::persist::fs crate-dir: /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc/rustc_rust_log_aux-3jpjw0cfg3asc
DEBUG rustc_incremental::persist::fs crate directory created successfully
DEBUG rustc_incremental::persist::fs generate_session_dir_path: timestamp = g3f11g9k3l
DEBUG rustc_incremental::persist::fs generate_session_dir_path: random_number = 1217206237
DEBUG rustc_incremental::persist::fs generate_session_dir_path: directory_name = s-g3f11g9k3l-k4oycd-working
DEBUG rustc_incremental::persist::fs generate_session_dir_path: directory_path = /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc/rustc_rust_log_aux-3jpjw0cfg3asc/s-g3f11g9k3l-k4oycd-working
DEBUG rustc_incremental::persist::fs session-dir: /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc/rustc_rust_log_aux-3jpjw0cfg3asc/s-g3f11g9k3l-k4oycd-working
DEBUG rustc_incremental::persist::fs lock_directory() - lock_file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc/rustc_rust_log_aux-3jpjw0cfg3asc/s-g3f11g9k3l-k4oycd.lock
DEBUG rustc_incremental::persist::fs session directory created successfully
DEBUG rustc_incremental::persist::fs find_source_directory_in_iter - inspecting `/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc/rustc_rust_log_aux-3jpjw0cfg3asc/s-g3f11g9k3l-k4oycd.lock`
DEBUG rustc_incremental::persist::fs find_source_directory_in_iter - ignoring
DEBUG rustc_incremental::persist::fs find_source_directory_in_iter - inspecting `/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc/rustc_rust_log_aux-3jpjw0cfg3asc/s-g3f11g9k3l-k4oycd-working`
DEBUG rustc_incremental::persist::fs find_source_directory_in_iter - ignoring
DEBUG rustc_incremental::persist::fs no source directory found. Continuing with empty session directory.
DEBUG rustc_incremental::persist::fs garbage_collect_session_directories() - begin
DEBUG rustc_incremental::persist::fs garbage_collect_session_directories() - session directory: /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc/rustc_rust_log_aux-3jpjw0cfg3asc/s-g3f11g9k3l-k4oycd-working
DEBUG rustc_incremental::persist::fs garbage_collect_session_directories() - crate directory: /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/rustc-rust-log.inc/rustc_rust_log_aux-3jpjw0cfg3asc
DEBUG rustc_incremental::persist::fs garbage_collect_session_directories() - inspecting: s-g3f11g9k3l-k4oycd-working
DEBUG rustc_incremental::persist::fs garbage_collect_session_directories() - not finalized, not old enough
DEBUG rustc_hir::definitions DefPathTable::insert() - DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } } <-> DefIndex(0)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:3: 2:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(4294967040), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:6: 2:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:13: 2:14 (#0) }, Parenthesis, TokenStream([(Token(Token { kind: Ident("rpass1", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:7: 2:13 (#0) }), Alone)])), tokens: None }, Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:2: 2:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:14: 2:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:3: 2:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:6: 2:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:13: 2:14 (#0) }, Paren, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass1", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:7: 2:13 (#0) }), Alone)])), Alone)])), Alone)])))), id: AttrId(0), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:15 (#0) }])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:3: 2:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(4294967040), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:6: 2:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:13: 2:14 (#0) }, Parenthesis, TokenStream([(Token(Token { kind: Ident("rpass1", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:7: 2:13 (#0) }), Alone)])), tokens: None }, Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:2: 2:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:14: 2:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:3: 2:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:6: 2:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:13: 2:14 (#0) }, Paren, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass1", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:7: 2:13 (#0) }), Alone)])), Alone)])), Alone)])))), id: AttrId(0), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:15 (#0) }])
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(4294967040), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, TokenStream([(Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }), Alone)])), tokens: None }, Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:2: 5:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:14: 5:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Paren, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }), Alone)])), Alone)])), Alone)])))), id: AttrId(1), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:15 (#0) }])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(4294967040), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, TokenStream([(Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }), Alone)])), tokens: None }, Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:2: 5:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:14: 5:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Paren, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }), Alone)])), Alone)])), Alone)])))), id: AttrId(1), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:15 (#0) }])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(16809380290718647644, 12779641950333497158))
DEBUG rustc_span byte pos BytePos(29) is on the line at byte pos BytePos(29)
DEBUG rustc_span char pos CharPos(29) is on the line at char pos CharPos(29)
DEBUG rustc_span byte is on line: 2
DEBUG rustc_resolve resolve_crate_root($crate#0)
DEBUG rustc_resolve resolve_crate_root: marks=[]
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
DEBUG rustc_resolve resolve_crate_root: found opaque mark None None
DEBUG rustc_resolve resolve_crate_root: found semi-transparent mark None None
DEBUG rustc_resolve resolve_crate_root($crate#0): found no mark (ident.span = no-location (#0))
DEBUG rustc_resolve resolve_crate_root($crate#1)
DEBUG rustc_span::hygiene marks: getting parent of #1
DEBUG rustc_resolve resolve_crate_root: marks=[(ExpnData { kind: AstPass(StdImports), parent: crate0::{{expn0}}, call_site: no-location (#0), disambiguator: 0, def_site: no-location (#0), allow_internal_unstable: Some(["prelude_import"]), allow_internal_unsafe: false, local_inner_macros: false, edition: Edition2015, macro_def_id: None, parent_module: None }, Opaque)]
DEBUG rustc_span::hygiene marks: getting parent of #1
DEBUG rustc_resolve resolve_crate_root: found opaque mark Some(crate0::{{expn1}}) Some(ExpnData { kind: AstPass(StdImports), parent: crate0::{{expn0}}, call_site: no-location (#0), disambiguator: 0, def_site: no-location (#0), allow_internal_unstable: Some(["prelude_import"]), allow_internal_unsafe: false, local_inner_macros: false, edition: Edition2015, macro_def_id: None, parent_module: None })
DEBUG rustc_resolve resolve_crate_root: found semi-transparent mark Some(crate0::{{expn1}}) Some(ExpnData { kind: AstPass(StdImports), parent: crate0::{{expn0}}, call_site: no-location (#0), disambiguator: 0, def_site: no-location (#0), allow_internal_unstable: Some(["prelude_import"]), allow_internal_unsafe: false, local_inner_macros: false, edition: Edition2015, macro_def_id: None, parent_module: None })
DEBUG rustc_resolve resolve_crate_root($crate#1): got module Some(Def(Mod, DefId(0:0))) (Some("")) (ident.span = no-location (#1))
DEBUG rustc_resolve resolve_crate_root($crate#2)
DEBUG rustc_resolve resolve_crate_root: marks=[]
DEBUG rustc_resolve resolve_crate_root: found opaque mark None None
DEBUG rustc_resolve resolve_crate_root: found semi-transparent mark None None
DEBUG rustc_resolve resolve_crate_root($crate#2): found no mark (ident.span = no-location (#2))
DEBUG rustc_ast_pretty::pp mk_printer 78
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_ast_pretty::pp mk_printer 78
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_resolve::def_collector visit_item: Item { attrs: [], id: NodeId(2), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 8:2 (#0), vis: Visibility { kind: Public, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:1 (#0), tokens: None }, ident: #0, kind: Mod(No, Loaded([Item { attrs: [Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(4), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(3), style: Outer, span: no-location (#1) }], id: NodeId(3), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: #0, kind: Use(UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(5), args: None }, PathSegment { ident: std#1, id: NodeId(6), args: None }, PathSegment { ident: prelude#1, id: NodeId(7), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(8), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }), tokens: None }, Item { attrs: [Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(10), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(2), style: Outer, span: no-location (#1) }], id: NodeId(9), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: std#2, kind: ExternCrate(None), tokens: None }, Item { attrs: [Attribute { kind: Normal(AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:3: 2:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(12), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:6: 2:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:13: 2:14 (#0) }, Parenthesis, TokenStream([(Token(Token { kind: Ident("rpass1", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:7: 2:13 (#0) }), Alone)])), tokens: None }, Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:2: 2:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:14: 2:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:3: 2:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:6: 2:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:13: 2:14 (#0) }, Paren, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass1", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:7: 2:13 (#0) }), Alone)])), Alone)])), Alone)])))), id: AttrId(0), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:15 (#0) }], id: NodeId(11), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:1: 3:16 (#0), vis: Visibility { kind: Public, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:1: 3:4 (#0), tokens: None }, ident: foo#0, kind: Fn(FnKind(Final, FnSig { header: FnHeader { unsafety: No, asyncness: No, constness: No, ext: None }, decl: FnDecl { inputs: [], output: Default(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:14: 3:14 (#0)) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:1: 3:13 (#0) }, Generics { params: [], where_clause: WhereClause { has_where_token: false, predicates: [], span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:13: 3:13 (#0) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:11: 3:11 (#0) }, Some(Block { stmts: [], id: NodeId(13), rules: Default, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:14: 3:16 (#0), tokens: None, could_be_bare_literal: false }))), tokens: None }], Yes, /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 8:2 (#0))), tokens: None }
DEBUG rustc_resolve::def_collector visit_item: Item { attrs: [Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(4), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(3), style: Outer, span: no-location (#1) }], id: NodeId(3), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: #0, kind: Use(UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(5), args: None }, PathSegment { ident: std#1, id: NodeId(6), args: None }, PathSegment { ident: prelude#1, id: NodeId(7), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(8), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }), tokens: None }
DEBUG rustc_resolve::def_collector create_def(node_id=NodeId(3), data=Misc, parent_def=DefId(0:0))
DEBUG rustc_hir::definitions create_def(parent=DefId(0:0), data=Misc, expn_id=crate0::{{expn0}})
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(16809380290718647644, 12779641950333497158))
DEBUG rustc_hir::definitions create_def: after disambiguation, key = DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } }
DEBUG rustc_hir::definitions DefPathTable::insert() - DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } } <-> DefIndex(1)
DEBUG rustc_resolve create_def: def_id_to_node_id[DefId(0:1)] <-> NodeId(3)
DEBUG rustc_resolve::def_collector visit_item: Item { attrs: [Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(10), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(2), style: Outer, span: no-location (#1) }], id: NodeId(9), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: std#2, kind: ExternCrate(None), tokens: None }
DEBUG rustc_resolve::def_collector create_def(node_id=NodeId(9), data=TypeNs("std"), parent_def=DefId(0:0))
DEBUG rustc_hir::definitions create_def(parent=DefId(0:0), data=TypeNs("std"), expn_id=crate0::{{expn0}})
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(16809380290718647644, 12779641950333497158))
DEBUG rustc_hir::definitions create_def: after disambiguation, key = DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } }
DEBUG rustc_hir::definitions DefPathTable::insert() - DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } } <-> DefIndex(2)
DEBUG rustc_resolve create_def: def_id_to_node_id[DefId(0:2)] <-> NodeId(9)
DEBUG rustc_resolve::def_collector visit_item: Item { attrs: [Attribute { kind: Normal(AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:3: 2:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(12), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:6: 2:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:13: 2:14 (#0) }, Parenthesis, TokenStream([(Token(Token { kind: Ident("rpass1", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:7: 2:13 (#0) }), Alone)])), tokens: None }, Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:2: 2:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:14: 2:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:3: 2:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:6: 2:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:13: 2:14 (#0) }, Paren, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass1", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:7: 2:13 (#0) }), Alone)])), Alone)])), Alone)])))), id: AttrId(0), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:15 (#0) }], id: NodeId(11), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:1: 3:16 (#0), vis: Visibility { kind: Public, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:1: 3:4 (#0), tokens: None }, ident: foo#0, kind: Fn(FnKind(Final, FnSig { header: FnHeader { unsafety: No, asyncness: No, constness: No, ext: None }, decl: FnDecl { inputs: [], output: Default(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:14: 3:14 (#0)) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:1: 3:13 (#0) }, Generics { params: [], where_clause: WhereClause { has_where_token: false, predicates: [], span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:13: 3:13 (#0) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:11: 3:11 (#0) }, Some(Block { stmts: [], id: NodeId(13), rules: Default, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:3:14: 3:16 (#0), tokens: None, could_be_bare_literal: false }))), tokens: None }
DEBUG rustc_resolve::def_collector create_def(node_id=NodeId(11), data=ValueNs("foo"), parent_def=DefId(0:0))
DEBUG rustc_hir::definitions create_def(parent=DefId(0:0), data=ValueNs("foo"), expn_id=crate0::{{expn0}})
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(16809380290718647644, 12779641950333497158))
DEBUG rustc_hir::definitions create_def: after disambiguation, key = DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: ValueNs("foo"), disambiguator: 0 } }
DEBUG rustc_hir::definitions DefPathTable::insert() - DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: ValueNs("foo"), disambiguator: 0 } } <-> DefIndex(3)
DEBUG rustc_resolve create_def: def_id_to_node_id[DefId(0:3)] <-> NodeId(11)
DEBUG rustc_resolve::build_reduced_graph build_reduced_graph_for_use_tree(parent_prefix=[], use_tree=UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(5), args: None }, PathSegment { ident: std#1, id: NodeId(6), args: None }, PathSegment { ident: prelude#1, id: NodeId(7), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(8), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }, nested=false)
DEBUG rustc_resolve::build_reduced_graph build_reduced_graph_for_use_tree: prefix=[Segment { ident: {{root}}#1, id: Some(NodeId(5)), has_generic_args: false }, Segment { ident: std#1, id: Some(NodeId(6)), has_generic_args: false }, Segment { ident: prelude#1, id: Some(NodeId(7)), has_generic_args: false }, Segment { ident: rust_2015#1, id: Some(NodeId(8)), has_generic_args: false }]
DEBUG rustc_resolve::build_reduced_graph add_import(Import { kind: Glob { is_prelude: true, max_vis: Cell { value: Invisible } }, id: NodeId(3), root_id: NodeId(3), use_span: no-location (#1), use_span_with_attributes: no-location (#1), has_attributes: true, span: no-location (#1), root_span: no-location (#1), parent_scope: ParentScope { module: Some(Def(Mod, DefId(0:0))), expansion: expn0, macro_rules: PtrKey(Cell { value: Empty }), derives: [] }, module_path: [Segment { ident: {{root}}#1, id: Some(NodeId(5)), has_generic_args: false }, Segment { ident: std#1, id: Some(NodeId(6)), has_generic_args: false }, Segment { ident: prelude#1, id: Some(NodeId(7)), has_generic_args: false }, Segment { ident: rust_2015#1, id: Some(NodeId(8)), has_generic_args: false }], imported_module: Cell { value: None }, vis: Cell { value: Restricted(DefId(0:0)) }, used: Cell { value: false } })
DEBUG rustc_metadata::creader resolving extern crate stmt. ident: std orig_name: None
INFO rustc_metadata::creader resolving crate `std`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
DEBUG rustc_session::filesearch picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
INFO rustc_metadata::locator Rejecting via crate name
INFO rustc_metadata::locator metadata mismatch
INFO rustc_metadata::creader register crate `std` (cnum = 1. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate core hash: `9f3be45e8676b6a5` extra filename: `-0286f51ede2a0cbe`
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
INFO rustc_metadata::creader register crate `core` (cnum = 2. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate2 is [crate2]
INFO rustc_metadata::creader resolving dep crate compiler_builtins hash: `7337dfd6317aa580` extra filename: `-cfcd5a8ed441a0ab`
INFO rustc_metadata::creader resolving crate `compiler_builtins`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
INFO rustc_metadata::creader register crate `compiler_builtins` (cnum = 3. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f5d7bf675bd510d8` extra filename: `-0ae23430f49edc9e`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
INFO rustc_metadata::creader register crate `rustc_std_workspace_core` (cnum = 4. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate core hash: `9f3be45e8676b6a5` extra filename: `-0286f51ede2a0cbe`
INFO rustc_metadata::creader resolving crate `core`
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate4 is [crate4, crate2]
INFO rustc_metadata::creader resolving dep crate core hash: `9f3be45e8676b6a5` extra filename: `-0286f51ede2a0cbe`
INFO rustc_metadata::creader resolving crate `core`
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate3 is [crate3, crate4, crate2]
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f5d7bf675bd510d8` extra filename: `-0ae23430f49edc9e`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
INFO rustc_metadata::creader resolving dep crate alloc hash: `07e01b656e8ffd10` extra filename: `-5f56516096d51b90`
INFO rustc_metadata::creader resolving crate `alloc`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_session::filesearch searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-335dc1191561f64c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1278c8d07ac46e5c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-d3cb48a7a718793b.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch picked /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-db974ab0a14d4763.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-7599f4d00b02b998.rlib
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-0c02d7fa8c5ec082.so
DEBUG rustc_session::filesearch testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
DEBUG rustc_session::filesearch rejected /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.so
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib
INFO rustc_metadata::creader register crate `alloc` (cnum = 5. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
---
test result: FAILED. 148 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.35s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:15
