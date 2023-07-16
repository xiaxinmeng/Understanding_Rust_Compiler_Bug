plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c2a5c3a50fc3fb6d16cd140f55f7db61cbf08a01 and ffc1c90dd8ac61b2690b461229faa37d81f3d1b9
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: /checkout/compiler/rustc_middle/src/hir/map/mod.rs:175:13: local_def_id: no entry for `HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[1728]::iter::adapters::cloned::clone_try_fold) }, local_id: 20 }`, which has a map of `Some(Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[1728]::iter::adapters::cloned::clone_try_fold) }, local_id: 20 }, kind: Block(Block { stmts: [], expr: Some(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[1728]::iter::adapters::cloned::clone_try_fold) }, local_id: 18 }, kind: Closure(Closure { binder: Default, capture_clause: Value, bound_generic_params: [], fn_decl: FnDecl { inputs: [Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[1728]::iter::adapters::cloned::clone_try_fold) }, local_id: 16 }, kind: Infer, span: library/core/src/iter/adapters/cloned.rs:28:11: 28:14 (#0) }, Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[1728]::iter::adapters::cloned::clone_try_fold) }, local_id: 17 }, kind: Infer, span: library/core/src/iter/adapters/cloned.rs:28:16: 28:19 (#0) }], output: DefaultReturn(library/core/src/iter/adapters/cloned.rs:28:21: 28:21 (#0)), c_variadic: false, implicit_self: None }, body: BodyId { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[1728]::iter::adapters::cloned::clone_try_fold) }, local_id: 15 } }, fn_decl_span: library/core/src/iter/adapters/cloned.rs:28:5: 28:20 (#0), movability: None }), span: library/core/src/iter/adapters/cloned.rs:28:5: 28:40 (#0) }), hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[1728]::iter::adapters::cloned::clone_try_fold) }, local_id: 19 }, rules: DefaultBlock, span: library/core/src/iter/adapters/cloned.rs:27:97: 29:2 (#0), targeted_by_break: false }, None), span: library/core/src/iter/adapters/cloned.rs:27:97: 29:2 (#0) }))`
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1551:9
stack backtrace:
stack backtrace:
   0:     0x7f893a0e6f52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbef12973c5507d53
   1:     0x7f893a14e7c8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f893a0d7631 - std::io::Write::write_fmt::h0fc5efafbc8ce341
   3:     0x7f893a0e6d15 - std::sys_common::backtrace::print::h7f5fe1a3e38a3041
   4:     0x7f893a0ea0f7 - std::panicking::default_hook::{{closure}}::h62c63d72dd478c99
   5:     0x7f893a0e9e55 - std::panicking::default_hook::h61d2d5a20d5abab2
   6:     0x7f893aa67786 - rustc_driver[6bf0f267d6b7b2e6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f893a0eaa03 - std::panicking::rust_panic_with_hook::h6fb0b7b5144c30a1
   8:     0x7f893dda0ec3 - std[b4d7b7572d6e9bb7]::panicking::begin_panic::<rustc_errors[44c6e37299278c93]::ExplicitBug>::{closure#0}
   9:     0x7f893dd99b46 - std[b4d7b7572d6e9bb7]::sys_common::backtrace::__rust_end_short_backtrace::<std[b4d7b7572d6e9bb7]::panicking::begin_panic<rustc_errors[44c6e37299278c93]::ExplicitBug>::{closure#0}, !>
  10:     0x7f893aa03b46 - std[b4d7b7572d6e9bb7]::panicking::begin_panic::<rustc_errors[44c6e37299278c93]::ExplicitBug>
  11:     0x7f893dd12196 - std[b4d7b7572d6e9bb7]::panic::panic_any::<rustc_errors[44c6e37299278c93]::ExplicitBug>
  12:     0x7f893dd03ac7 - <rustc_errors[44c6e37299278c93]::HandlerInner>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  13:     0x7f893dd035b0 - <rustc_errors[44c6e37299278c93]::Handler>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  14:     0x7f893ddf4ec0 - rustc_middle[80645aab16aa2417]::ty::context::tls::with_context_opt::<rustc_middle[80645aab16aa2417]::ty::context::tls::with_opt<rustc_middle[80645aab16aa2417]::util::bug::opt_span_bug_fmt<rustc_span[c623ba6ff50061ca]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f893ddf6549 - rustc_middle[80645aab16aa2417]::util::bug::opt_span_bug_fmt::<rustc_span[c623ba6ff50061ca]::span_encoding::Span>
  16:     0x7f893aa09485 - rustc_middle[80645aab16aa2417]::util::bug::bug_fmt
  17:     0x7f893afab799 - rustc_ty_utils[337830a8e4fc7a52]::ty::param_env
  18:     0x7f893cbe8235 - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::DefId, rustc_middle[80645aab16aa2417]::ty::ParamEnv>>
  19:     0x7f893ccf3833 - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::param_env, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  20:     0x7f893c8188de - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::param_env
  21:     0x7f893b210d2b - <rustc_hir_typeck[aa9109b4864bcc91]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[aa9109b4864bcc91]::typeck_with_fallback<rustc_hir_typeck[aa9109b4864bcc91]::typeck::{closure#0}>::{closure#1}, &rustc_middle[80645aab16aa2417]::ty::context::TypeckResults>
  22:     0x7f893b242cfc - rustc_hir_typeck[aa9109b4864bcc91]::typeck
  23:     0x7f893cbc3a11 - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId, &rustc_middle[80645aab16aa2417]::ty::context::TypeckResults>>
  24:     0x7f893ccf228f - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::typeck, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  25:     0x7f893c801534 - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::typeck
  26:     0x7f893dd71eeb - <rustc_middle[80645aab16aa2417]::ty::context::TyCtxt>::typeck_opt_const_arg
  27:     0x7f893be4d49b - rustc_mir_build[85a2d26c4b53d73a]::thir::cx::thir_body
  28:     0x7f893cb94cd6 - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_middle[80645aab16aa2417]::ty::WithOptConstParam<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId>, core[69c2305d6fa5d54f]::result::Result<(&rustc_data_structures[7f2f9159a885a50a]::steal::Steal<rustc_middle[80645aab16aa2417]::thir::Thir>, rustc_middle[80645aab16aa2417]::thir::ExprId), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>>
  29:     0x7f893ccf395f - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::thir_body, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  30:     0x7f893c7e3738 - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::thir_body
  31:     0x7f893bdcbc70 - rustc_mir_build[85a2d26c4b53d73a]::build::mir_built
  32:     0x7f893cb95f8e - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_middle[80645aab16aa2417]::ty::WithOptConstParam<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId>, &rustc_data_structures[7f2f9159a885a50a]::steal::Steal<rustc_middle[80645aab16aa2417]::mir::Body>>>
  33:     0x7f893ccf34be - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::mir_built, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  34:     0x7f893c7e5783 - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::mir_built
  35:     0x7f893b80011c - rustc_mir_transform[7fd88398f4be618a]::check_unsafety::unsafety_check_result
  36:     0x7f893b7fc1c6 - <rustc_mir_transform[7fd88398f4be618a]::check_unsafety::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[80645aab16aa2417]::ty::context::TyCtxt, rustc_span[c623ba6ff50061ca]::def_id::LocalDefId)>>::call_once
  37:     0x7f893cbc5c31 - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId, &rustc_middle[80645aab16aa2417]::mir::query::UnsafetyCheckResult>>
  38:     0x7f893cce975f - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::unsafety_check_result, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  39:     0x7f893c7f81f4 - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::unsafety_check_result
  40:     0x7f893b740910 - rustc_mir_transform[7fd88398f4be618a]::mir_const
  41:     0x7f893cb95f8e - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_middle[80645aab16aa2417]::ty::WithOptConstParam<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId>, &rustc_data_structures[7f2f9159a885a50a]::steal::Steal<rustc_middle[80645aab16aa2417]::mir::Body>>>
  42:     0x7f893ccf35e1 - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::mir_const, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  43:     0x7f893c7e5df3 - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::mir_const
  44:     0x7f893b741700 - rustc_mir_transform[7fd88398f4be618a]::mir_promoted
  45:     0x7f893cb97207 - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_middle[80645aab16aa2417]::ty::WithOptConstParam<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId>, (&rustc_data_structures[7f2f9159a885a50a]::steal::Steal<rustc_middle[80645aab16aa2417]::mir::Body>, &rustc_data_structures[7f2f9159a885a50a]::steal::Steal<rustc_index[6602892e1cf9c063]::vec::IndexVec<rustc_middle[80645aab16aa2417]::mir::Promoted, rustc_middle[80645aab16aa2417]::mir::Body>>)>>
  46:     0x7f893ccdcf3f - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::mir_promoted, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  47:     0x7f893c7e8b83 - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::mir_promoted
  48:     0x7f893c0e797b - rustc_borrowck[2750583ac5c452f]::mir_borrowck
  49:     0x7f893c0b31b6 - <rustc_borrowck[2750583ac5c452f]::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[80645aab16aa2417]::ty::context::TyCtxt, rustc_span[c623ba6ff50061ca]::def_id::LocalDefId)>>::call_once
  50:     0x7f893cbc4b21 - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId, &rustc_middle[80645aab16aa2417]::mir::query::BorrowCheckResult>>
  51:     0x7f893ccdccce - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::mir_borrowck, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  52:     0x7f893c803ba4 - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::mir_borrowck
  53:     0x7f893b3dde9a - rustc_hir_analysis[7ddfff9688df90c6]::collect::type_of::find_opaque_ty_constraints_for_rpit
  54:     0x7f893b3dd1ba - rustc_hir_analysis[7ddfff9688df90c6]::collect::type_of::type_of
  55:     0x7f893cbe7025 - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::DefId, rustc_middle[80645aab16aa2417]::ty::Ty>>
  56:     0x7f893ccf2517 - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::type_of, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  57:     0x7f893c7dc9de - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::type_of
  58:     0x7f893b2d15fb - <rustc_privacy[fb865fb5bb7d482d]::ReachEverythingInTheInterfaceVisitor>::ty
  59:     0x7f893b2cfd41 - <rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  60:     0x7f893b2e11e9 - rustc_hir[61c311f6e1052be3]::intravisit::walk_ty::<rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor>
  61:     0x7f893b2e0b97 - rustc_hir[61c311f6e1052be3]::intravisit::walk_fn::<rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor>
  62:     0x7f893b2e4e4d - rustc_hir[61c311f6e1052be3]::intravisit::walk_item::<rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor>
  63:     0x7f893b2cfe49 - <rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  64:     0x7f893b2e4e9d - rustc_hir[61c311f6e1052be3]::intravisit::walk_item::<rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor>
  65:     0x7f893b2cfe49 - <rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  66:     0x7f893b2e4e9d - rustc_hir[61c311f6e1052be3]::intravisit::walk_item::<rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor>
  67:     0x7f893b2cfe49 - <rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  68:     0x7f893b2e4e9d - rustc_hir[61c311f6e1052be3]::intravisit::walk_item::<rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor>
  69:     0x7f893b2cfe49 - <rustc_privacy[fb865fb5bb7d482d]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  70:     0x7f893b2d858f - rustc_privacy[fb865fb5bb7d482d]::effective_visibilities
  71:     0x7f893cc4600a - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<(), &rustc_middle[80645aab16aa2417]::middle::privacy::EffectiveVisibilities>>
  72:     0x7f893cce9e30 - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::effective_visibilities, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  73:     0x7f893c80b2ea - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::effective_visibilities
  74:     0x7f893bcc4141 - rustc_passes[2738dbc0b228869a]::stability::check_unused_or_stable_features
  75:     0x7f893ac05b3d - <rustc_session[acc81e6a9479a650]::session::Session>::time::<(), rustc_interface[8d41ef18a2df1324]::passes::analysis::{closure#0}>
  76:     0x7f893ac54ae9 - rustc_interface[8d41ef18a2df1324]::passes::analysis
  77:     0x7f893cc3bfc2 - rustc_query_system[b8e662a09ac41318]::query::plumbing::try_execute_query::<rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt, rustc_query_system[b8e662a09ac41318]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>>
  78:     0x7f893ccf2620 - rustc_query_system[b8e662a09ac41318]::query::plumbing::get_query::<rustc_query_impl[7af5504625afbf08]::queries::analysis, rustc_query_impl[7af5504625afbf08]::plumbing::QueryCtxt>
  79:     0x7f893c7dd6ba - <rustc_query_impl[7af5504625afbf08]::Queries as rustc_middle[80645aab16aa2417]::ty::query::QueryEngine>::analysis
  80:     0x7f893aae79b3 - <rustc_interface[8d41ef18a2df1324]::passes::QueryContext>::enter::<rustc_driver[6bf0f267d6b7b2e6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  81:     0x7f893ab021d2 - <rustc_interface[8d41ef18a2df1324]::interface::Compiler>::enter::<rustc_driver[6bf0f267d6b7b2e6]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[8d41ef18a2df1324]::queries::Linker>, rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  82:     0x7f893aa692f2 - rustc_span[c623ba6ff50061ca]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_interface[8d41ef18a2df1324]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[6bf0f267d6b7b2e6]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  83:     0x7f893aaf0ca3 - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[c623ba6ff50061ca]::SessionGlobals>>::set::<rustc_interface[8d41ef18a2df1324]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[6bf0f267d6b7b2e6]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  84:     0x7f893aa99c20 - std[b4d7b7572d6e9bb7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8d41ef18a2df1324]::util::run_in_thread_pool_with_globals<rustc_interface[8d41ef18a2df1324]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[6bf0f267d6b7b2e6]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  85:     0x7f893aa80269 - <<std[b4d7b7572d6e9bb7]::thread::Builder>::spawn_unchecked_<rustc_interface[8d41ef18a2df1324]::util::run_in_thread_pool_with_globals<rustc_interface[8d41ef18a2df1324]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[6bf0f267d6b7b2e6]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  86:     0x7f893a0f766e - std::sys::unix::thread::Thread::new::thread_start::h05afa8442b34c479
  87:     0x7f8939e92b43 - <unknown>
  88:     0x7f8939f24a00 - <unknown>
  89:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (ffc1c90dd 2022-11-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [param_env] computing normalized predicates of `iter::adapters::cloned::clone_try_fold`
#1 [typeck] type-checking `iter::adapters::cloned::clone_try_fold`
#2 [thir_body] building THIR for `iter::adapters::cloned::clone_try_fold`
#3 [mir_built] building MIR for `iter::adapters::cloned::clone_try_fold`
#4 [unsafety_check_result] unsafety-checking `iter::adapters::cloned::clone_try_fold`
#5 [mir_const] preparing `iter::adapters::cloned::clone_try_fold` for borrow checking
#6 [mir_promoted] processing MIR for `iter::adapters::cloned::clone_try_fold`
#7 [mir_borrowck] borrow-checking `iter::adapters::cloned::clone_try_fold`
#8 [type_of] computing type of `iter::adapters::cloned::clone_try_fold::{opaque#1}`
#9 [effective_visibilities] checking effective visibilities
#10 [analysis] running analysis passes on this crate
error: could not compile `core`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:36
cat: /tmp/toolstate/toolstates.json: No such file or directory
