plain
failures:

---- [ui] src/test/ui/rustc-rust-log.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/auxiliary/rustc-rust-log-aux.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/auxiliary/rustc-rust-log-aux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary"
stdout: none
--- stderr -------------------------------
DEBUG rustc_target::spec got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", pointer_width: 64, arch: "x86_64", data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128", options: TargetOptions { is_builtin: true, endian: little, c_int_width: "32", os: "linux", env: "gnu", abi: "", vendor: "unknown", linker_flavor: Gcc, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-m64"]}, pre_link_objects: {}, post_link_objects: {}, pre_link_objects_fallback: {}, post_link_objects_fallback: {}, crt_objects_fallback: None, late_link_args: {}, late_link_args_dynamic: {}, late_link_args_static: {}, post_link_args: {}, link_script: None, link_env: [], link_env_remove: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: Pic, code_model: None, tls_model: GeneralDynamic, disable_redzone: false, frame_pointer: MayOmit, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", families: ["unix"], abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_emscripten: false, is_like_fuchsia: false, is_like_wasm: false, dwarf_version: None, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, static_position_independent_executables: true, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, main_needs_argc_argv: true, has_thread_local: true, obj_is_bitcode: false, forces_embed_bitcode: false, bitcode_llvm_cmdline: "", min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Unwind, crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: true, stack_probes: Call, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, default_hidden_visibility: false, emit_debug_gdb_scripts: true, requires_uwtable: false, default_uwtable: false, simd_types_indirect: true, limit_rdylib_exports: true, override_export_symbols: None, merge_functions: Aliases, mcount: "mcount", llvm_abiname: "", relax_elf_relocations: false, llvm_args: [], use_ctors_section: false, eh_frame_header: true, has_thumb_interworking: false, split_debuginfo: Off, supported_sanitizers: ADDRESS | LEAK | MEMORY | THREAD | CFI, default_adjusted_cabi: None, c_enum_min_bits: 32, generate_arange_section: true, supports_stack_protector: true } }
DEBUG rustc_target::spec got builtin target: Target { llvm_target: "x86_64-unknown-linux-gnu", pointer_width: 64, arch: "x86_64", data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128", options: TargetOptions { is_builtin: true, endian: little, c_int_width: "32", os: "linux", env: "gnu", abi: "", vendor: "unknown", linker_flavor: Gcc, linker: None, lld_flavor: Ld, pre_link_args: {Gcc: ["-m64"]}, pre_link_objects: {}, post_link_objects: {}, pre_link_objects_fallback: {}, post_link_objects_fallback: {}, crt_objects_fallback: None, late_link_args: {}, late_link_args_dynamic: {}, late_link_args_static: {}, post_link_args: {}, link_script: None, link_env: [], link_env_remove: [], asm_args: [], cpu: "x86-64", features: "", dynamic_linking: true, only_cdylib: false, executables: true, relocation_model: Pic, code_model: None, tls_model: GeneralDynamic, disable_redzone: false, frame_pointer: MayOmit, function_sections: true, dll_prefix: "lib", dll_suffix: ".so", exe_suffix: "", staticlib_prefix: "lib", staticlib_suffix: ".a", families: ["unix"], abi_return_struct_as_int: false, is_like_osx: false, is_like_solaris: false, is_like_windows: false, is_like_msvc: false, is_like_emscripten: false, is_like_fuchsia: false, is_like_wasm: false, dwarf_version: None, linker_is_gnu: true, allows_weak_linkage: true, has_rpath: true, no_default_libraries: true, position_independent_executables: true, static_position_independent_executables: true, needs_plt: false, relro_level: Full, archive_format: "gnu", allow_asm: true, main_needs_argc_argv: true, has_thread_local: true, obj_is_bitcode: false, forces_embed_bitcode: false, bitcode_llvm_cmdline: "", min_atomic_width: None, max_atomic_width: Some(64), atomic_cas: true, panic_strategy: Unwind, crt_static_allows_dylibs: false, crt_static_default: false, crt_static_respected: true, stack_probes: Call, min_global_align: None, default_codegen_units: None, trap_unreachable: true, requires_lto: false, singlethread: false, no_builtins: false, default_hidden_visibility: false, emit_debug_gdb_scripts: true, requires_uwtable: false, default_uwtable: false, simd_types_indirect: true, limit_rdylib_exports: true, override_export_symbols: None, merge_functions: Aliases, mcount: "mcount", llvm_abiname: "", relax_elf_relocations: false, llvm_args: [], use_ctors_section: false, eh_frame_header: true, has_thumb_interworking: false, split_debuginfo: Off, supported_sanitizers: ADDRESS | LEAK | MEMORY | THREAD | CFI, default_adjusted_cabi: None, c_enum_min_bits: 32, generate_arange_section: true, supports_stack_protector: true } }
DEBUG rustc_parse::lexer next_token: LineComment { doc_style: None }("// rustc-env:RUSTC_LOG=debug")
DEBUG rustc_parse::lexer next_token: Whitespace("\n")
DEBUG rustc_hir::definitions DefPathTable::insert() - DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } } <-> DefIndex(0)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_span byte pos BytePos(29) is on the line at byte pos BytePos(0)
DEBUG rustc_span char pos CharPos(29) is on the line at char pos CharPos(0)
DEBUG rustc_span byte is on line: 1
DEBUG rustc_resolve resolve_crate_root($crate#0)
DEBUG rustc_resolve resolve_crate_root: marks=[]
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
DEBUG rustc_resolve::def_collector visit_item: Item { attrs: [Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(1), style: Outer, span: no-location (#1) }], id: NodeId(1), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: #0, kind: Use(UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(3), args: None }, PathSegment { ident: std#1, id: NodeId(4), args: None }, PathSegment { ident: prelude#1, id: NodeId(5), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(6), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }), tokens: None }
DEBUG rustc_resolve::def_collector create_def(node_id=NodeId(1), data=Misc, parent_def=DefId(0:0))
DEBUG rustc_hir::definitions create_def(parent=DefId(0:0), data=Misc, expn_id=crate0::{{expn0}})
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_hir::definitions create_def: after disambiguation, key = DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } }
DEBUG rustc_hir::definitions DefPathTable::insert() - DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } } <-> DefIndex(1)
DEBUG rustc_resolve create_def: def_id_to_node_id[DefId(0:1)] <-> NodeId(1)
DEBUG rustc_resolve::def_collector visit_item: Item { attrs: [Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(0), style: Outer, span: no-location (#1) }], id: NodeId(7), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: std#2, kind: ExternCrate(None), tokens: None }
DEBUG rustc_resolve::def_collector create_def(node_id=NodeId(7), data=TypeNs("std"), parent_def=DefId(0:0))
DEBUG rustc_hir::definitions create_def(parent=DefId(0:0), data=TypeNs("std"), expn_id=crate0::{{expn0}})
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_hir::definitions create_def: after disambiguation, key = DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } }
DEBUG rustc_hir::definitions DefPathTable::insert() - DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } } <-> DefIndex(2)
DEBUG rustc_resolve create_def: def_id_to_node_id[DefId(0:2)] <-> NodeId(7)
DEBUG rustc_resolve::build_reduced_graph build_reduced_graph_for_use_tree(parent_prefix=[], use_tree=UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(3), args: None }, PathSegment { ident: std#1, id: NodeId(4), args: None }, PathSegment { ident: prelude#1, id: NodeId(5), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(6), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }, nested=false)
DEBUG rustc_resolve::build_reduced_graph build_reduced_graph_for_use_tree: prefix=[Segment { ident: {{root}}#1, id: Some(NodeId(3)), has_generic_args: false }, Segment { ident: std#1, id: Some(NodeId(4)), has_generic_args: false }, Segment { ident: prelude#1, id: Some(NodeId(5)), has_generic_args: false }, Segment { ident: rust_2015#1, id: Some(NodeId(6)), has_generic_args: false }]
DEBUG rustc_metadata::creader resolving extern crate stmt. ident: std orig_name: None
INFO rustc_metadata::creader resolving crate `std`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
INFO rustc_metadata::locator Rejecting via crate name
INFO rustc_metadata::locator metadata mismatch
INFO rustc_metadata::creader register crate `std` (cnum = 1. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
INFO rustc_metadata::creader register crate `core` (cnum = 2. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate2 is [crate2]
INFO rustc_metadata::creader resolving dep crate compiler_builtins hash: `bf5dd4d0dc6a35b7` extra filename: `-113cdab8486abf7b`
INFO rustc_metadata::creader resolving crate `compiler_builtins`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
INFO rustc_metadata::creader register crate `compiler_builtins` (cnum = 3. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f6982f01a782d8db` extra filename: `-2deeba5fb4de62b0`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
INFO rustc_metadata::creader register crate `rustc_std_workspace_core` (cnum = 4. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate4 is [crate4, crate2]
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate3 is [crate3, crate4, crate2]
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f6982f01a782d8db` extra filename: `-2deeba5fb4de62b0`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
INFO rustc_metadata::creader resolving dep crate alloc hash: `95ff422dfc714af4` extra filename: `-2858890a1193dc97`
INFO rustc_metadata::creader resolving crate `alloc`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
INFO rustc_metadata::creader register crate `alloc` (cnum = 5. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader resolving dep crate compiler_builtins hash: `bf5dd4d0dc6a35b7` extra filename: `-113cdab8486abf7b`
INFO rustc_metadata::creader resolving crate `compiler_builtins`
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f6982f01a782d8db` extra filename: `-2deeba5fb4de62b0`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate5 is [crate5, crate2, crate3, crate4]
INFO rustc_metadata::creader resolving dep crate libc hash: `6350d7550ebaa7f3` extra filename: `-73995a372eb744b2`
INFO rustc_metadata::creader resolving crate `libc`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
INFO rustc_metadata::creader register crate `libc` (cnum = 6. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f6982f01a782d8db` extra filename: `-2deeba5fb4de62b0`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate6 is [crate6, crate4, crate2]
INFO rustc_metadata::creader resolving dep crate unwind hash: `ae0a0c8ed2df309c` extra filename: `-b57f33e444a42584`
INFO rustc_metadata::creader resolving crate `unwind`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::creader register crate `unwind` (cnum = 7. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader resolving dep crate compiler_builtins hash: `bf5dd4d0dc6a35b7` extra filename: `-113cdab8486abf7b`
INFO rustc_metadata::creader resolving crate `compiler_builtins`
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f6982f01a782d8db` extra filename: `-2deeba5fb4de62b0`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
INFO rustc_metadata::creader resolving dep crate cfg_if hash: `a8963af1ee140cd4` extra filename: `-610b6edb811d23b8`
INFO rustc_metadata::creader resolving crate `cfg_if`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
INFO rustc_metadata::creader register crate `cfg_if` (cnum = 8. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f6982f01a782d8db` extra filename: `-2deeba5fb4de62b0`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader resolving dep crate compiler_builtins hash: `bf5dd4d0dc6a35b7` extra filename: `-113cdab8486abf7b`
INFO rustc_metadata::creader resolving crate `compiler_builtins`
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate8 is [crate8, crate4, crate2, crate3]
INFO rustc_metadata::creader resolving dep crate libc hash: `6350d7550ebaa7f3` extra filename: `-73995a372eb744b2`
INFO rustc_metadata::creader resolving crate `libc`
DEBUG rustc_metadata::creader resolve_crate_deps: cnum_map for crate7 is [crate7, crate2, crate3, crate4, crate8, crate6]
INFO rustc_metadata::creader resolving dep crate cfg_if hash: `a8963af1ee140cd4` extra filename: `-610b6edb811d23b8`
INFO rustc_metadata::creader resolving crate `cfg_if`
INFO rustc_metadata::creader resolving dep crate miniz_oxide hash: `6e277d9d4a8d0824` extra filename: `-15e2dc5275b79ae1`
INFO rustc_metadata::creader resolving crate `miniz_oxide`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
INFO rustc_metadata::creader register crate `miniz_oxide` (cnum = 9. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f6982f01a782d8db` extra filename: `-2deeba5fb4de62b0`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_core`
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader resolving dep crate compiler_builtins hash: `bf5dd4d0dc6a35b7` extra filename: `-113cdab8486abf7b`
INFO rustc_metadata::creader resolving crate `compiler_builtins`
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_alloc hash: `bcee205cf60f1423` extra filename: `-02c5c19766fd1e65`
INFO rustc_metadata::creader resolving crate `rustc_std_workspace_alloc`
INFO rustc_metadata::creader falling back to a load
DEBUG rustc_session::filesearch using sysroot = /checkout/obj/build/x86_64-unknown-linux-gnu/stage2, triple = x86_64-unknown-linux-gnu
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-rust-log/auxiliary/rustc-rust-log-aux
DEBUG rustc_metadata::locator searching /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-2e3969d646408ab9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
INFO rustc_metadata::locator lib candidate: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-b83c7948c64e04d0.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-9ba7d6b406667995.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-3a40c8fab5a00730.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-16a10f39fde3c4ed.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-44bd0e98fa81e525.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-05a148fedab5967c.so
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib
DEBUG rustc_metadata::locator testing /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
INFO rustc_metadata::locator rlib reading metadata from: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
INFO rustc_metadata::creader register crate `rustc_std_workspace_alloc` (cnum = 10. private_dep = false)
DEBUG rustc_metadata::creader resolving deps of external crate
INFO rustc_metadata::creader resolving dep crate alloc hash: `95ff422dfc714af4` extra filename: `-2858890a1193dc97`
INFO rustc_metadata::creader resolving crate `alloc`
INFO rustc_metadata::creader resolving dep crate core hash: `808fc9bce8a947d1` extra filename: `-a2a7040fb9f918eb`
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader resolving dep crate compiler_builtins hash: `bf5dd4d0dc6a35b7` extra filename: `-113cdab8486abf7b`
INFO rustc_metadata::creader resolving crate `compiler_builtins`
INFO rustc_metadata::creader resolving dep crate rustc_std_workspace_core hash: `f6982f01a782d8db` extra filename: `-2deeba5fb4de62b0`
---
  hash: f6982f01a782d8db
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib
  name: alloc
  cnum: 5
  hash: 95ff422dfc714af4
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-2858890a1193dc97.rlib
  name: libc
  cnum: 6
  hash: 6350d7550ebaa7f3
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-73995a372eb744b2.rlib
  name: unwind
  cnum: 7
  hash: ae0a0c8ed2df309c
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-b57f33e444a42584.rlib
  name: cfg_if
  cnum: 8
  hash: a8963af1ee140cd4
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-610b6edb811d23b8.rlib
  cnum: 9
  hash: 6e277d9d4a8d0824
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-15e2dc5275b79ae1.rlib
  name: rustc_std_workspace_alloc
  cnum: 10
  hash: bcee205cf60f1423
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-02c5c19766fd1e65.rlib
  name: adler
  cnum: 11
  hash: 4cfa1da89b85f277
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-275f56d7e5200df4.rlib
  name: hashbrown
  cnum: 12
  hash: b1c428c1ed534349
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0f11c9965c8c91c9.rlib
  cnum: 13
  hash: d896194d016985a8
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e0e19a32ccc87f50.rlib
  name: rustc_demangle
  cnum: 14
  hash: 70d5567c7f550f46
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-547af7bc2fec9f91.rlib
  name: addr2line
  cnum: 15
  hash: f52932e783822e8c
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d79e5a95fb762d2.rlib
  name: gimli
  cnum: 16
  hash: c8d5432a8c0394af
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-0aee660e30c6d37d.rlib
  name: object
  cnum: 17
  hash: ff5bf5805212fd12
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d0c6ea0fda611e7e.rlib
  name: memchr
  cnum: 18
  hash: 032b6ee6d0e25240
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8e1b2117fe8362f7.rlib
  cnum: 19
  hash: d93101eac258d716
  hash: d93101eac258d716
  reqd: Implicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a74da4686d67242a.rlib

DEBUG rustc_ast_passes::feature_gate gate_feature(feature = "prelude_import", span = no-location (#1)); has? false
INFO rustc_interface::passes 0 parse sess buffered_lints
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(1), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(1), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(0), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(0), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 3788753291874631433))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 14496962626200708948))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 3788753291874631433))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 14496962626200708948))
DEBUG rustc_ast_lowering::index visit_nested_item: ItemId { def_id: DefId(0:1) }
DEBUG rustc_ast_lowering::index visit_nested_item: ItemId { def_id: DefId(0:2) }
DEBUG rustc_ast_lowering::item lower_use_tree(tree=UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(3), args: None }, PathSegment { ident: std#1, id: NodeId(4), args: None }, PathSegment { ident: prelude#1, id: NodeId(5), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(6), args: None }], tokens: None }, kind: Glob, span: no-location (#1) })
DEBUG rustc_ast_lowering::item lower_use_tree: vis = Spanned { node: Inherited, span: no-location (#1) }
DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: {{root}}#1, id: NodeId(3), args: None }, expected_lifetimes: 0)
DEBUG rustc_ast_lowering::path lower_path_segment: ident={{root}}#1 original-id=NodeId(3) new-id=HirId { owner: DefId(0:1), local_id: 1 }
DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: std#1, id: NodeId(4), args: None }, expected_lifetimes: 0)
DEBUG rustc_ast_lowering::path lower_path_segment: ident=std#1 original-id=NodeId(4) new-id=HirId { owner: DefId(0:1), local_id: 2 }
DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: prelude#1, id: NodeId(5), args: None }, expected_lifetimes: 0)
DEBUG rustc_ast_lowering::path lower_path_segment: ident=prelude#1 original-id=NodeId(5) new-id=HirId { owner: DefId(0:1), local_id: 3 }
DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: rust_2015#1, id: NodeId(6), args: None }, expected_lifetimes: 0)
DEBUG rustc_ast_lowering::path lower_path_segment: ident=rust_2015#1 original-id=NodeId(6) new-id=HirId { owner: DefId(0:1), local_id: 4 }
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(0) len=65248
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(69) len=65248
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(283) len=65248
DEBUG rustc_ast_lowering::index visit_item: Item { ident: #0, def_id: DefId(0:1), kind: Use(Path { span: no-location (#1), res: Err, segments: [PathSegment { ident: {{root}}#1, hir_id: Some(HirId { owner: DefId(0:1), local_id: 1 }), res: Some(Err), args: None, infer_args: false }, PathSegment { ident: std#1, hir_id: Some(HirId { owner: DefId(0:1), local_id: 2 }), res: Some(Def(Mod, DefId(1:0))), args: None, infer_args: false }, PathSegment { ident: prelude#1, hir_id: Some(HirId { owner: DefId(0:1), local_id: 3 }), res: Some(Def(Mod, DefId(1:69))), args: None, infer_args: false }, PathSegment { ident: rust_2015#1, hir_id: Some(HirId { owner: DefId(0:1), local_id: 4 }), res: Some(Def(Mod, DefId(1:283))), args: None, infer_args: false }] }, Glob), vis: Spanned { node: Inherited, span: no-location (#1) }, span: no-location (#1) }
DEBUG rustc_ast_lowering::index visit_item: Item { ident: std#2, def_id: DefId(0:2), kind: ExternCrate(None), vis: Spanned { node: Inherited, span: no-location (#1) }, span: no-location (#1) }
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 3788753291874631433))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 14496962626200708948))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 3788753291874631433))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 14496962626200708948))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<analysis>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<hir_module_items>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<hir_owner>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<hir_crate>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<hir_owner>(key=DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{misc#0}), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<hir_owner>(key=DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<entry_fn>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<proc_macro_decls_static>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<hir_attrs>(key=DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{misc#0}), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<hir_attrs>(key=DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std), span=no-location (#0))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<check_mod_loops>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<check_mod_attrs>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<hir_attrs>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<check_mod_naked_functions>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<check_mod_unstable_api_usage>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<extern_mod_stmt_cnum>(key=DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<resolutions>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<lookup_deprecation_entry>(key=DefId(1:0 ~ std[5811]), span=no-location (#0))
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(0) len=46224
DEBUG rustc_query_system::query::plumbing ty::query::get_query<lookup_stability>(key=DefId(1:0 ~ std[5811]), span=no-location (#0))
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(0) len=64140
DEBUG rustc_middle::middle::stability stability: inspecting def_id=DefId(1:0 ~ std[5811]) span=no-location (#1) of stability=Some(Stability { level: Stable { since: "1.0.0" }, feature: "rust1" })
DEBUG rustc_query_system::query::plumbing ty::query::get_query<opt_def_kind>(key=DefId(1:0 ~ std[5811]), span=no-location (#0))
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(0) len=65248
DEBUG rustc_query_system::query::plumbing ty::query::get_query<visibility>(key=DefId(1:0 ~ std[5811]), span=no-location (#0))
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(0) len=64420
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<check_mod_const_bodies>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<features_query>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<lib_features>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<limits>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<stability_index>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<local_def_id_to_hir_id>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_passes::stability annotate(id = DefId(0:0 ~ rustc_rust_log_aux[8cb6]), attrs = [])
DEBUG rustc_query_system::query::plumbing ty::query::get_query<local_def_id_to_hir_id>(key=DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{misc#0}), span=no-location (#0))
DEBUG rustc_passes::stability annotate(id = DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{misc#0}), attrs = [Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(1), style: Outer, span: no-location (#1) }])
DEBUG rustc_query_system::query::plumbing ty::query::get_query<local_def_id_to_hir_id>(key=DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std), span=no-location (#0))
DEBUG rustc_passes::stability annotate(id = DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std), attrs = [Attribute { kind: Normal(AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, None), id: AttrId(0), style: Outer, span: no-location (#1) }])
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<collect_mod_item_types>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_typeck::collect convert: item  with id HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{misc#0}), local_id: 0 }
DEBUG rustc_typeck::collect convert: item std with id HirId { owner: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std), local_id: 0 }
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<check_mod_impl_wf>(key=DefId(0:0 ~ rustc_rust_log_aux[8cb6]), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<orphan_check_crate>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<all_local_trait_impls>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<crate_inherent_impls>(key=(), span=no-location (#0))
DEBUG rustc_query_system::query::plumbing ty::query::get_query<crate_inherent_impls_overlap_check>(key=(), span=no-location (#0))
thread 'rustc' panicked at 'internal error: entered unreachable code: TypeId calls write_u64', /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/registry/extensions.rs:22:9
stack backtrace:
   0:     0x7fe71a9ad0dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6ca0fe84aa9e27e7
   1:     0x7fe71aa105a8 - core::fmt::write::he17de9233fea93d8
   2:     0x7fe71a99d471 - std::io::Write::write_fmt::h7672e0bb17d0b82b
   3:     0x7fe71a9b03fe - std::panicking::default_hook::{{closure}}::hae276b26c77ed5c4
   4:     0x7fe71a9b002d - std::panicking::default_hook::h8e0458bb9c9f6634
   5:     0x7fe71b52e121 - rustc_driver[c601ba54c6e222ee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe71a9b0c2b - std::panicking::rust_panic_with_hook::h7b1078509d51085d
   7:     0x7fe71a9b0a57 - std::panicking::begin_panic_handler::{{closure}}::h485a7b7faf2ae919
   8:     0x7fe71a9ad5f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4d9235268f973538
   9:     0x7fe71a9b0739 - rust_begin_unwind
  10:     0x7fe71a963d83 - core::panicking::panic_fmt::he594899f1980f3f4
  11:     0x7fe71c958b0f - core[3f508ffac36d4e4a]::panicking::unreachable_display::<&str>
  12:     0x7fe71c95a786 - <tracing_subscriber[f9e872bc9ba58c4b]::registry::extensions::IdHasher as core[3f508ffac36d4e4a]::hash::Hasher>::write
  13:     0x7fe71b5b57eb - <core[3f508ffac36d4e4a]::any::TypeId as core[3f508ffac36d4e4a]::hash::Hash>::hash::<tracing_subscriber[f9e872bc9ba58c4b]::registry::extensions::IdHasher>
  14:     0x7fe71b5b27c5 - <hashbrown[28deab1329d4f444]::map::HashMap<core[3f508ffac36d4e4a]::any::TypeId, alloc[6cc0bba574eed3e2]::boxed::Box<dyn core[3f508ffac36d4e4a]::any::Any + core[3f508ffac36d4e4a]::marker::Sync + core[3f508ffac36d4e4a]::marker::Send>, core[3f508ffac36d4e4a]::hash::BuildHasherDefault<tracing_subscriber[f9e872bc9ba58c4b]::registry::extensions::IdHasher>>>::insert
  15:     0x7fe71b5b4698 - <tracing_subscriber[f9e872bc9ba58c4b]::registry::extensions::ExtensionsMut>::insert::<tracing_tree[1d97f823963a05]::Data>
  16:     0x7fe71b5bc542 - <tracing_tree[1d97f823963a05]::HierarchicalLayer<std[5811ffa2c04cb93a]::io::stdio::stderr> as tracing_subscriber[f9e872bc9ba58c4b]::layer::Layer<tracing_subscriber[f9e872bc9ba58c4b]::layer::layered::Layered<tracing_subscriber[f9e872bc9ba58c4b]::filter::env::EnvFilter, tracing_subscriber[f9e872bc9ba58c4b]::registry::sharded::Registry>>>::on_new_span
  17:     0x7fe71b5bfc8b - <tracing_subscriber[f9e872bc9ba58c4b]::layer::layered::Layered<tracing_tree[1d97f823963a05]::HierarchicalLayer<std[5811ffa2c04cb93a]::io::stdio::stderr>, tracing_subscriber[f9e872bc9ba58c4b]::layer::layered::Layered<tracing_subscriber[f9e872bc9ba58c4b]::filter::env::EnvFilter, tracing_subscriber[f9e872bc9ba58c4b]::registry::sharded::Registry>> as tracing_core[e3c24fc7c131d1e0]::subscriber::Subscriber>::new_span
  18:     0x7fe71e2347b5 - <tracing[471dbb73490f935a]::span::Span>::make_with
  19:     0x7fe71e235541 - tracing_core[e3c24fc7c131d1e0]::dispatcher::get_default::<tracing[471dbb73490f935a]::span::Span, <tracing[471dbb73490f935a]::span::Span>::new::{closure#0}>
  20:     0x7fe71e23473e - <tracing[471dbb73490f935a]::span::Span>::new
  21:     0x7fe71bffb10c - <rustc_typeck[78acda3e4a59be1]::check::wfcheck::CheckTypeWellFormedVisitor as rustc_hir[9c1d87007976d9de]::intravisit::Visitor>::visit_item
  22:     0x7fe71c063926 - <core[3f508ffac36d4e4a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[aab9713cd2ee2b12]::sync::par_for_each_in<&alloc[6cc0bba574eed3e2]::vec::Vec<rustc_hir[9c1d87007976d9de]::hir::MaybeOwner<&rustc_hir[9c1d87007976d9de]::hir::OwnerInfo>>, <rustc_middle[dd5f58d53aee067b]::hir::map::Map>::par_visit_all_item_likes<rustc_typeck[78acda3e4a59be1]::check::wfcheck::CheckTypeWellFormedVisitor>::{closure#0}>::{closure#0}::{closure#0}> as core[3f508ffac36d4e4a]::ops::function::FnOnce<()>>::call_once
  23:     0x7fe71be76426 - std[5811ffa2c04cb93a]::panicking::try::<(), core[3f508ffac36d4e4a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[aab9713cd2ee2b12]::sync::par_for_each_in<&alloc[6cc0bba574eed3e2]::vec::Vec<rustc_hir[9c1d87007976d9de]::hir::MaybeOwner<&rustc_hir[9c1d87007976d9de]::hir::OwnerInfo>>, <rustc_middle[dd5f58d53aee067b]::hir::map::Map>::par_visit_all_item_likes<rustc_typeck[78acda3e4a59be1]::check::wfcheck::CheckTypeWellFormedVisitor>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7fe71bfd91dc - rustc_data_structures[aab9713cd2ee2b12]::sync::par_for_each_in::<&alloc[6cc0bba574eed3e2]::vec::Vec<rustc_hir[9c1d87007976d9de]::hir::MaybeOwner<&rustc_hir[9c1d87007976d9de]::hir::OwnerInfo>>, <rustc_middle[dd5f58d53aee067b]::hir::map::Map>::par_visit_all_item_likes<rustc_typeck[78acda3e4a59be1]::check::wfcheck::CheckTypeWellFormedVisitor>::{closure#0}>
  25:     0x7fe71bec262c - rustc_typeck[78acda3e4a59be1]::check_crate
  26:     0x7fe71b64a601 - rustc_interface[928319cfe292e0ae]::passes::analysis
  27:     0x7fe71cc133be - rustc_query_system[94605439fea80b15]::query::plumbing::try_execute_query::<rustc_query_impl[3b80519096b02c90]::plumbing::QueryCtxt, rustc_query_system[94605439fea80b15]::query::caches::DefaultCache<(), core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>>
  28:     0x7fe71cd107e8 - rustc_query_system[94605439fea80b15]::query::plumbing::get_query::<rustc_query_impl[3b80519096b02c90]::queries::analysis, rustc_query_impl[3b80519096b02c90]::plumbing::QueryCtxt>
  29:     0x7fe71b50f854 - <rustc_interface[928319cfe292e0ae]::passes::QueryContext>::enter::<rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>
  30:     0x7fe71b4bd674 - <rustc_interface[928319cfe292e0ae]::interface::Compiler>::enter::<rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}::{closure#2}, core[3f508ffac36d4e4a]::result::Result<core[3f508ffac36d4e4a]::option::Option<rustc_interface[928319cfe292e0ae]::queries::Linker>, rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>
  31:     0x7fe71b51ba5b - rustc_span[151fbe2421202e93]::with_source_map::<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_interface[928319cfe292e0ae]::interface::create_compiler_and_run<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7fe71b4d1990 - rustc_interface[928319cfe292e0ae]::interface::create_compiler_and_run::<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>
  33:     0x7fe71b4d4a2f - <scoped_tls[b65b4136f2dff78e]::ScopedKey<rustc_span[151fbe2421202e93]::SessionGlobals>>::set::<rustc_interface[928319cfe292e0ae]::interface::run_compiler<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>
  34:     0x7fe71b513119 - std[5811ffa2c04cb93a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[928319cfe292e0ae]::util::run_in_thread_pool_with_globals<rustc_interface[928319cfe292e0ae]::interface::run_compiler<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>
  35:     0x7fe71b4d69a1 - std[5811ffa2c04cb93a]::panicking::try::<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, core[3f508ffac36d4e4a]::panic::unwind_safe::AssertUnwindSafe<<std[5811ffa2c04cb93a]::thread::Builder>::spawn_unchecked_<rustc_interface[928319cfe292e0ae]::util::run_in_thread_pool_with_globals<rustc_interface[928319cfe292e0ae]::interface::run_compiler<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  36:     0x7fe71b515562 - <<std[5811ffa2c04cb93a]::thread::Builder>::spawn_unchecked_<rustc_interface[928319cfe292e0ae]::util::run_in_thread_pool_with_globals<rustc_interface[928319cfe292e0ae]::interface::run_compiler<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#1} as core[3f508ffac36d4e4a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7fe71a9bba63 - std::sys::unix::thread::Thread::new::thread_start::h9d76c622918a675d
  38:     0x7fe714f0e609 - start_thread
  39:     0x7fe71a821163 - clone
  40:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (86303f4a2 2022-04-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'internal error: entered unreachable code: TypeId calls write_u64', /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/registry/extensions.rs:22:9
stack backtrace:
   0:     0x7fe71a9ad0dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6ca0fe84aa9e27e7
   1:     0x7fe71aa105a8 - core::fmt::write::he17de9233fea93d8
   2:     0x7fe71a99d471 - std::io::Write::write_fmt::h7672e0bb17d0b82b
   3:     0x7fe71a9b03fe - std::panicking::default_hook::{{closure}}::hae276b26c77ed5c4
   4:     0x7fe71a9b002d - std::panicking::default_hook::h8e0458bb9c9f6634
   5:     0x7fe71b52e121 - rustc_driver[c601ba54c6e222ee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe71a9b0c2b - std::panicking::rust_panic_with_hook::h7b1078509d51085d
   7:     0x7fe71a9b0a57 - std::panicking::begin_panic_handler::{{closure}}::h485a7b7faf2ae919
   8:     0x7fe71a9ad5f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4d9235268f973538
   9:     0x7fe71a9b0739 - rust_begin_unwind
  10:     0x7fe71a963d83 - core::panicking::panic_fmt::he594899f1980f3f4
  11:     0x7fe71c958b0f - core[3f508ffac36d4e4a]::panicking::unreachable_display::<&str>
  12:     0x7fe71c95a786 - <tracing_subscriber[f9e872bc9ba58c4b]::registry::extensions::IdHasher as core[3f508ffac36d4e4a]::hash::Hasher>::write
  13:     0x7fe71b5b57eb - <core[3f508ffac36d4e4a]::any::TypeId as core[3f508ffac36d4e4a]::hash::Hash>::hash::<tracing_subscriber[f9e872bc9ba58c4b]::registry::extensions::IdHasher>
  14:     0x7fe71b5b27c5 - <hashbrown[28deab1329d4f444]::map::HashMap<core[3f508ffac36d4e4a]::any::TypeId, alloc[6cc0bba574eed3e2]::boxed::Box<dyn core[3f508ffac36d4e4a]::any::Any + core[3f508ffac36d4e4a]::marker::Sync + core[3f508ffac36d4e4a]::marker::Send>, core[3f508ffac36d4e4a]::hash::BuildHasherDefault<tracing_subscriber[f9e872bc9ba58c4b]::registry::extensions::IdHasher>>>::insert
  15:     0x7fe71b5b4698 - <tracing_subscriber[f9e872bc9ba58c4b]::registry::extensions::ExtensionsMut>::insert::<tracing_tree[1d97f823963a05]::Data>
  16:     0x7fe71b5bc542 - <tracing_tree[1d97f823963a05]::HierarchicalLayer<std[5811ffa2c04cb93a]::io::stdio::stderr> as tracing_subscriber[f9e872bc9ba58c4b]::layer::Layer<tracing_subscriber[f9e872bc9ba58c4b]::layer::layered::Layered<tracing_subscriber[f9e872bc9ba58c4b]::filter::env::EnvFilter, tracing_subscriber[f9e872bc9ba58c4b]::registry::sharded::Registry>>>::on_new_span
  17:     0x7fe71b5bfc8b - <tracing_subscriber[f9e872bc9ba58c4b]::layer::layered::Layered<tracing_tree[1d97f823963a05]::HierarchicalLayer<std[5811ffa2c04cb93a]::io::stdio::stderr>, tracing_subscriber[f9e872bc9ba58c4b]::layer::layered::Layered<tracing_subscriber[f9e872bc9ba58c4b]::filter::env::EnvFilter, tracing_subscriber[f9e872bc9ba58c4b]::registry::sharded::Registry>> as tracing_core[e3c24fc7c131d1e0]::subscriber::Subscriber>::new_span
  18:     0x7fe71e2347b5 - <tracing[471dbb73490f935a]::span::Span>::make_with
  19:     0x7fe71e235541 - tracing_core[e3c24fc7c131d1e0]::dispatcher::get_default::<tracing[471dbb73490f935a]::span::Span, <tracing[471dbb73490f935a]::span::Span>::new::{closure#0}>
  20:     0x7fe71e23473e - <tracing[471dbb73490f935a]::span::Span>::new
  21:     0x7fe71bffb10c - <rustc_typeck[78acda3e4a59be1]::check::wfcheck::CheckTypeWellFormedVisitor as rustc_hir[9c1d87007976d9de]::intravisit::Visitor>::visit_item
  22:     0x7fe71c063926 - <core[3f508ffac36d4e4a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[aab9713cd2ee2b12]::sync::par_for_each_in<&alloc[6cc0bba574eed3e2]::vec::Vec<rustc_hir[9c1d87007976d9de]::hir::MaybeOwner<&rustc_hir[9c1d87007976d9de]::hir::OwnerInfo>>, <rustc_middle[dd5f58d53aee067b]::hir::map::Map>::par_visit_all_item_likes<rustc_typeck[78acda3e4a59be1]::check::wfcheck::CheckTypeWellFormedVisitor>::{closure#0}>::{closure#0}::{closure#0}> as core[3f508ffac36d4e4a]::ops::function::FnOnce<()>>::call_once
  23:     0x7fe71be76426 - std[5811ffa2c04cb93a]::panicking::try::<(), core[3f508ffac36d4e4a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[aab9713cd2ee2b12]::sync::par_for_each_in<&alloc[6cc0bba574eed3e2]::vec::Vec<rustc_hir[9c1d87007976d9de]::hir::MaybeOwner<&rustc_hir[9c1d87007976d9de]::hir::OwnerInfo>>, <rustc_middle[dd5f58d53aee067b]::hir::map::Map>::par_visit_all_item_likes<rustc_typeck[78acda3e4a59be1]::check::wfcheck::CheckTypeWellFormedVisitor>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7fe71bfd91dc - rustc_data_structures[aab9713cd2ee2b12]::sync::par_for_each_in::<&alloc[6cc0bba574eed3e2]::vec::Vec<rustc_hir[9c1d87007976d9de]::hir::MaybeOwner<&rustc_hir[9c1d87007976d9de]::hir::OwnerInfo>>, <rustc_middle[dd5f58d53aee067b]::hir::map::Map>::par_visit_all_item_likes<rustc_typeck[78acda3e4a59be1]::check::wfcheck::CheckTypeWellFormedVisitor>::{closure#0}>
  25:     0x7fe71bec262c - rustc_typeck[78acda3e4a59be1]::check_crate
  26:     0x7fe71b64a601 - rustc_interface[928319cfe292e0ae]::passes::analysis
  27:     0x7fe71cc133be - rustc_query_system[94605439fea80b15]::query::plumbing::try_execute_query::<rustc_query_impl[3b80519096b02c90]::plumbing::QueryCtxt, rustc_query_system[94605439fea80b15]::query::caches::DefaultCache<(), core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>>
  28:     0x7fe71cd107e8 - rustc_query_system[94605439fea80b15]::query::plumbing::get_query::<rustc_query_impl[3b80519096b02c90]::queries::analysis, rustc_query_impl[3b80519096b02c90]::plumbing::QueryCtxt>
  29:     0x7fe71b50f854 - <rustc_interface[928319cfe292e0ae]::passes::QueryContext>::enter::<rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>
  30:     0x7fe71b4bd674 - <rustc_interface[928319cfe292e0ae]::interface::Compiler>::enter::<rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}::{closure#2}, core[3f508ffac36d4e4a]::result::Result<core[3f508ffac36d4e4a]::option::Option<rustc_interface[928319cfe292e0ae]::queries::Linker>, rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>
  31:     0x7fe71b51ba5b - rustc_span[151fbe2421202e93]::with_source_map::<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_interface[928319cfe292e0ae]::interface::create_compiler_and_run<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7fe71b4d1990 - rustc_interface[928319cfe292e0ae]::interface::create_compiler_and_run::<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>
  33:     0x7fe71b4d4a2f - <scoped_tls[b65b4136f2dff78e]::ScopedKey<rustc_span[151fbe2421202e93]::SessionGlobals>>::set::<rustc_interface[928319cfe292e0ae]::interface::run_compiler<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>
  34:     0x7fe71b513119 - std[5811ffa2c04cb93a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[928319cfe292e0ae]::util::run_in_thread_pool_with_globals<rustc_interface[928319cfe292e0ae]::interface::run_compiler<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>
  35:     0x7fe71b4d69a1 - std[5811ffa2c04cb93a]::panicking::try::<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, core[3f508ffac36d4e4a]::panic::unwind_safe::AssertUnwindSafe<<std[5811ffa2c04cb93a]::thread::Builder>::spawn_unchecked_<rustc_interface[928319cfe292e0ae]::util::run_in_thread_pool_with_globals<rustc_interface[928319cfe292e0ae]::interface::run_compiler<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  36:     0x7fe71b515562 - <<std[5811ffa2c04cb93a]::thread::Builder>::spawn_unchecked_<rustc_interface[928319cfe292e0ae]::util::run_in_thread_pool_with_globals<rustc_interface[928319cfe292e0ae]::interface::run_compiler<core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>, rustc_driver[c601ba54c6e222ee]::run_compiler::{closure#1}>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#0}, core[3f508ffac36d4e4a]::result::Result<(), rustc_errors[fe0d17187870fa8b]::ErrorGuaranteed>>::{closure#1} as core[3f508ffac36d4e4a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7fe71a9bba63 - std::sys::unix::thread::Thread::new::thread_start::h9d76c622918a675d
  38:     0x7fe714f0e609 - start_thread
  39:     0x7fe71a821163 - clone
  40:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (86303f4a2 2022-04-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
------------------------------------------



