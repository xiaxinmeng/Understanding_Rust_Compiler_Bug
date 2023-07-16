
 λ  x test
Updating only changed submodules
  Submodules updated in 0.08 seconds
Building rustbuild
   Compiling bootstrap v0.0.0 (G:\Users\Chase\Code\Rust\rust\src\bootstrap)
    Finished dev [unoptimized] target(s) in 7.14s
Building stage0 tool tidy (x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.41s
tidy check
* 630 error codes
* highest error code: E0788
Checking which error codes lack tests...
* 378 features
Found 505 error codes
Found 0 error(s) in error codes
Done!
fmt check
Building stage0 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.62s
Copying stage0 std from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Building stage0 compiler artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling rustc_llvm v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_llvm)
   Compiling rustc_codegen_llvm v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_codegen_llvm)
   Compiling rustc_interface v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_interface)
   Compiling rustc_driver v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_driver)
   Compiling rustc-main v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc)
    Finished release [optimized] target(s) in 7m 56s
Copying stage0 rustc from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Assembling stage1 compiler (x86_64-pc-windows-msvc)
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling compiler_builtins v0.1.73
   Compiling core v0.0.0 (G:\Users\Chase\Code\Rust\rust\library\core)
   Compiling libc v0.2.126
   Compiling cc v1.0.69
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (G:\Users\Chase\Code\Rust\rust\library\std)
   Compiling unwind v0.0.0 (G:\Users\Chase\Code\Rust\rust\library\unwind)
error: internal compiler error: compiler\rustc_const_eval\src\interpret\operand.rs:438:18: invalid field access on immediate (0x0001, 0x00): (u16, bool), layout TyAndLayout {
                                    ty: (u16, bool),
                                    layout: Layout {
                                        fields: Arbitrary {
                                            offsets: [
                                                Size(2 bytes),
                                                Size(3 bytes),
                                            ],
                                            memory_index: [
                                                0,
                                                1,
                                            ],
                                        },
                                        variants: Single {
                                            index: 0,
                                        },
                                        abi: Aggregate {
                                            sized: true,
                                        },
                                        largest_niche: Some(
                                            Niche {
                                                offset: Size(2 bytes),
                                                value: Int(
                                                    I8,
                                                    false,
                                                ),
                                                valid_range: 0..=1,
                                            },
                                        ),
                                        align: AbiAndPrefAlign {
                                            abi: Align(2 bytes),
                                            pref: Align(8 bytes),
                                        },
                                        size: Size(4 bytes),
                                    },
                                }
   --> library\core\src\mem\valid_align.rs:125:19
    |
125 |     _Align1Shl0 = 1 << 0,
    |                   ^^^^^^

thread 'rustc' panicked at 'Box<dyn Any>', G:\Users\Chase\Code\Rust\rust\compiler\rustc_errors\src\lib.rs:1274:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: <rustc_errors::Handler>::span_err::<rustc_span::span_encoding::Span, &str>
   2: <rustc_errors::Handler>::span_err::<rustc_span::span_encoding::Span, &str>
   3: <rustc_errors::Handler>::span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
   4: rustc_const_eval::transform::check_consts::qualifs::in_operand::<rustc_const_eval::transform::check_consts::qualifs::NeedsDrop, <rustc_const_eval::transform::check_consts::resolver::TransferFunction<rustc_const_eval::transform::check_consts::qualifs::NeedsDrop> as rustc_middle::mir::visit::Visitor>::visit_terminator::{closure#0}>
   5: rustc_const_eval::transform::check_consts::qualifs::in_operand::<rustc_const_eval::transform::check_consts::qualifs::NeedsDrop, <rustc_const_eval::transform::check_consts::resolver::TransferFunction<rustc_const_eval::transform::check_consts::qualifs::NeedsDrop> as rustc_middle::mir::visit::Visitor>::visit_terminator::{closure#0}>
   6: rustc_middle::util::bug::span_bug_fmt::<rustc_span::span_encoding::Span>
   7: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::operand_field
   8: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::operand_projection
   9: RINvXs_NtNtNtCs9qroIgEvBu5_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterINtNtCscInEZjIJEzE_12rustc_middle3mir14ProjectionElemNtB1v_5LocalNtNtB1x_2ty2TyEEENtNtNtB9_6traits8iterator8Iterator8try_foldNtNtNtCshqJ9Ba8i3JL_16rustc_const_eval
  10: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place_to_op
  11: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place_to_op
  12: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::run
  13: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
  14: RINvXNtCscInEZjIJEzE_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph7DepKind9with_depsNCINvMs1_NtB17_5graphINtB2i_8DepGraphBH_E14with_task_implNtNtNtB5_2ty7context6TyCtxtINtB38_11ParamEnvAndNtNtNtB5_3mir9in
  15: RINvMs1_NtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph5graphINtB6_8DepGraphNtNtNtCscInEZjIJEzE_12rustc_middle9dep_graph8dep_node7DepKindE9with_taskNtNtNtB1j_2ty7context6TyCtxtINtB2r_11ParamEnvAndNtNtNtB1j_3mir9interpret8GlobalIdEINtNtCs9qroIgEvBu5_4core
  16: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
  17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green
  18: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  19: RINvXNtCscInEZjIJEzE_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph7DepKind9with_depsNCINvMs1_NtB17_5graphINtB2i_8DepGraphBH_E14with_task_implNtNtNtB5_2ty7context6TyCtxtINtB38_11ParamEnvAndNtNtNtB5_3mir9in
  20: RINvMs1_NtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph5graphINtB6_8DepGraphNtNtNtCscInEZjIJEzE_12rustc_middle9dep_graph8dep_node7DepKindE9with_taskNtNtNtB1j_2ty7context6TyCtxtINtB2r_11ParamEnvAndNtNtNtB1j_3mir9interpret8GlobalIdEINtNtCs9qroIgEvBu5_4core
  21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green
  23: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  24: RINvXNtCscInEZjIJEzE_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph7DepKind9with_depsNCINvMs1_NtB17_5graphINtB2i_8DepGraphBH_E14with_task_implNtNtNtB5_2ty7context6TyCtxtINtB38_11ParamEnvAndNtNtNtB5_3mir9in
  25: RINvMs1_NtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph5graphINtB6_8DepGraphNtNtNtCscInEZjIJEzE_12rustc_middle9dep_graph8dep_node7DepKindE9with_taskNtNtNtB1j_2ty7context6TyCtxtINtB2r_11ParamEnvAndNtNtNtB1j_3mir9interpret8GlobalIdEINtNtCs9qroIgEvBu5_4core
  26: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
  27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green
  28: <rustc_middle::ty::context::TyCtxt>::const_eval_global_id
  29: <rustc_middle::ty::context::TyCtxt>::const_eval_poly
  30: <rustc_typeck::collect::ItemCtxt>::bound_defines_assoc_item
  31: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  32: <rustc_middle::hir::map::Map>::deep_visit_item_likes_in_module::<rustc_typeck::collect::CollectItemTypesVisitor>
  33: RNvXNvNtCsbXJ7WfxSejG_12rustc_typeck6errors119__DERIVE_rustc_session_SessionDiagnostic_session_diagnostic_sess_rustc_errors_ErrorGuaranteed_FOR_SubstsOnOverriddenImplNtB4_22SubstsOnOverriddenImplNtNtCsjCzBQpETHRl_13rustc_session7session17SessionDiagnostic
  34: RINvXNtCscInEZjIJEzE_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph7DepKind9with_depsNCINvMs1_NtB17_5graphINtB2i_8DepGraphBH_E14with_task_implNtNtNtB5_2ty7context6TyCtxtINtNtCs9qroIgEvBu5_4core6option6Opti
  35: RINvMs1_NtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph5graphINtB6_8DepGraphNtNtNtCscInEZjIJEzE_12rustc_middle9dep_graph8dep_node7DepKindE9with_taskNtNtNtB1j_2ty7context6TyCtxtNtNtCs2vb4bKWGfvL_10rustc_span6def_id10LocalDefIduECs590zEwioTEG_16rustc_query
  36: rustc_query_system::query::plumbing::force_query::<rustc_query_impl::queries::trait_def, rustc_query_impl::plumbing::QueryCtxt>
  37: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
  38: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green
  39: <rustc_middle::hir::map::Map>::for_each_module::<rustc_typeck::check_crate::{closure#0}::{closure#0}::{closure#0}>
  40: <rustc_session::session::Session>::track_errors::<rustc_typeck::check_crate::{closure#0}, ()>
  41: rustc_typeck::check_crate
  42: rustc_interface::passes::analysis
  43: RINvXNtCscInEZjIJEzE_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph7DepKind9with_depsNCINvMs1_NtB17_5graphINtB2i_8DepGraphBH_E14with_task_implNtNtNtB5_2ty7context6TyCtxtuINtNtCs9qroIgEvBu5_4core6option6Opt
  44: RINvMs1_NtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph5graphINtB6_8DepGraphNtNtNtCscInEZjIJEzE_12rustc_middle9dep_graph8dep_node7DepKindE9with_taskNtNtNtB1j_2ty7context6TyCtxtuINtNtCs9qroIgEvBu5_4core6option6OptionNtNtNtCsfSnYSweX8G0_9rustc_ast6expand9a
  45: rustc_query_system::query::plumbing::force_query::<rustc_query_impl::queries::trait_def, rustc_query_impl::plumbing::QueryCtxt>
  46: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  47: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green
  48: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  49: RINvMs2_NtCs6lxe6u1d1HN_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCsg2BpsMvkavS_12rustc_driver12run_compilers_0s0_0INtNtCs9qroIgEvBu5_4core6result6ResultINtNtB2f_6option6OptionNtB6_6LinkerENtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuarante
  50: RINvCs2vb4bKWGfvL_10rustc_span15with_source_mapINtNtCs9qroIgEvBu5_4core6result6ResultuNtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuaranteedENCINvNtCs6lxe6u1d1HN_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCsg2BpsMvkavS_12rustc_driver12run_compile
  51: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
  52: RINvMs_CslsHbb2RadMK_10scoped_tlsINtB5_9ScopedKeyNtCs2vb4bKWGfvL_10rustc_span14SessionGlobalsE3setNCINvNtCs6lxe6u1d1HN_15rustc_interface9interface12run_compilerINtNtCs9qroIgEvBu5_4core6result6ResultuNtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuaranteedENCNvCsg2
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-dev running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C incremental -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C target-feature=+crt-static -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `mem::valid_align::ValidAlignEnum16::_Align1Shl0::{constant#0}`
#1 [eval_to_const_value_raw] simplifying constant for the type system `mem::valid_align::ValidAlignEnum16::_Align1Shl0::{constant#0}`
#2 [eval_to_const_value_raw] simplifying constant for the type system `mem::valid_align::ValidAlignEnum16::_Align1Shl0::{constant#0}`
#3 [collect_mod_item_types] collecting item types in module `mem::valid_align`
#4 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `core`

Caused by:
  process didn't exit successfully: `G:\Users\Chase\Code\Rust\rust\build\bootstrap\debug\rustc --crate-name core --edition=2021 library\core\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Zunstable-options --check-cfg names() --check-cfg values() -C metadata=a73f3dcab1ae4e39 -C extra-filename=-a73f3dcab1ae4e39 --out-dir G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -C incremental=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\incremental -L dependency=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps -L dependency=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\release\deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) "--check-cfg=values(target_env,\"libnx\")" "--check-cfg=values(target_os,\"watchos\")" "--check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\")" --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Ctarget-feature=+crt-static -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes "-Zcrate-attr=doc(html_root_url=\"https://doc.rust-lang.org/nightly/\")" -Z binary-dep-depinfo` (exit code: 0x80000003)
Build completed unsuccessfully in 0:08:30
