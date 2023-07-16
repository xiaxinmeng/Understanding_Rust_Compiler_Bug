plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: /checkout/compiler/rustc_middle/src/hir/map/mod.rs:175:13: local_def_id: no entry for `HirId { owner: OwnerId { def_id: DefId(0:6308 ~ core[3864]::iter::adapters::cloned::clone_try_fold) }, local_id: 20 }`, which has a map of `Some(Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6308 ~ core[3864]::iter::adapters::cloned::clone_try_fold) }, local_id: 20 }, kind: Block(Block { stmts: [], expr: Some(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6308 ~ core[3864]::iter::adapters::cloned::clone_try_fold) }, local_id: 18 }, kind: Closure(Closure { binder: Default, capture_clause: Value, bound_generic_params: [], fn_decl: FnDecl { inputs: [Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6308 ~ core[3864]::iter::adapters::cloned::clone_try_fold) }, local_id: 16 }, kind: Infer, span: library/core/src/iter/adapters/cloned.rs:28:11: 28:14 (#0) }, Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6308 ~ core[3864]::iter::adapters::cloned::clone_try_fold) }, local_id: 17 }, kind: Infer, span: library/core/src/iter/adapters/cloned.rs:28:16: 28:19 (#0) }], output: DefaultReturn(library/core/src/iter/adapters/cloned.rs:28:21: 28:21 (#0)), c_variadic: false, implicit_self: None }, body: BodyId { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6308 ~ core[3864]::iter::adapters::cloned::clone_try_fold) }, local_id: 15 } }, fn_decl_span: library/core/src/iter/adapters/cloned.rs:28:5: 28:20 (#0), movability: None }), span: library/core/src/iter/adapters/cloned.rs:28:5: 28:40 (#0) }), hir_id: HirId { owner: OwnerId { def_id: DefId(0:6308 ~ core[3864]::iter::adapters::cloned::clone_try_fold) }, local_id: 19 }, rules: DefaultBlock, span: library/core/src/iter/adapters/cloned.rs:27:97: 29:2 (#0), targeted_by_break: false }, None), span: library/core/src/iter/adapters/cloned.rs:27:97: 29:2 (#0) }))`
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1551:9
stack backtrace:
   0:     0x7f51c2d7f572 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   0:     0x7f51c2d7f572 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   1:     0x7f51c2ded308 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f51c2d6fda1 - std::io::Write::write_fmt::h9fe4e6d9c9b927ef
   3:     0x7f51c2d7f335 - std::sys_common::backtrace::print::hf38d1633e21dbba9
   4:     0x7f51c2d82717 - std::panicking::default_hook::{{closure}}::h12022b1a20dd35ce
   5:     0x7f51c2d82475 - std::panicking::default_hook::h5ab5b9712723f5dd
   6:     0x7f51c36fdc24 - rustc_driver[1f62b4d337d0a7e8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f51c2d83023 - std::panicking::rust_panic_with_hook::h57cd9b8bb3f6a82b
   8:     0x7f51c60ebd53 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[44c6e37299278c93]::ExplicitBug>::{closure#0}
   9:     0x7f51c60eba46 - std[389b380b19480123]::sys_common::backtrace::__rust_end_short_backtrace::<std[389b380b19480123]::panicking::begin_panic<rustc_errors[44c6e37299278c93]::ExplicitBug>::{closure#0}, !>
  10:     0x7f51c36a5116 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[44c6e37299278c93]::ExplicitBug>
  11:     0x7f51c6189d66 - std[389b380b19480123]::panic::panic_any::<rustc_errors[44c6e37299278c93]::ExplicitBug>
  12:     0x7f51c6184f37 - <rustc_errors[44c6e37299278c93]::HandlerInner>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  13:     0x7f51c6184a20 - <rustc_errors[44c6e37299278c93]::Handler>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  14:     0x7f51c624bb80 - rustc_middle[1a2146a359e71eef]::ty::context::tls::with_context_opt::<rustc_middle[1a2146a359e71eef]::ty::context::tls::with_opt<rustc_middle[1a2146a359e71eef]::util::bug::opt_span_bug_fmt<rustc_span[c623ba6ff50061ca]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f51c624cdd9 - rustc_middle[1a2146a359e71eef]::util::bug::opt_span_bug_fmt::<rustc_span[c623ba6ff50061ca]::span_encoding::Span>
  16:     0x7f51c36aa505 - rustc_middle[1a2146a359e71eef]::util::bug::bug_fmt
  17:     0x7f51c3b52d64 - rustc_ty_utils[74a4ef571ee4973e]::ty::param_env
  18:     0x7f51c5329150 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::DefId, rustc_middle[1a2146a359e71eef]::ty::ParamEnv>>
  19:     0x7f51c54322c3 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::param_env, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  20:     0x7f51c4ff276e - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::param_env
  21:     0x7f51c3d03197 - <rustc_hir_typeck[21af8057fb15f9fa]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[21af8057fb15f9fa]::typeck_with_fallback<rustc_hir_typeck[21af8057fb15f9fa]::typeck::{closure#0}>::{closure#1}, &rustc_middle[1a2146a359e71eef]::ty::context::TypeckResults>
  22:     0x7f51c3d7c564 - rustc_hir_typeck[21af8057fb15f9fa]::typeck
  23:     0x7f51c530eefe - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId, &rustc_middle[1a2146a359e71eef]::ty::context::TypeckResults>>
  24:     0x7f51c542eacf - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::typeck, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  25:     0x7f51c4fdb3c4 - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::typeck
  26:     0x7f51c61dd73f - <rustc_middle[1a2146a359e71eef]::ty::context::TyCtxt>::typeck_opt_const_arg
  27:     0x7f51c4794565 - rustc_mir_build[8df0e763745dd300]::thir::cx::thir_body
  28:     0x7f51c5432b25 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::thir_body, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  29:     0x7f51c4fbd5c8 - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::thir_body
  30:     0x7f51c47225c9 - rustc_mir_build[8df0e763745dd300]::build::mir_built
  31:     0x7f51c52fcfad - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<rustc_middle[1a2146a359e71eef]::ty::WithOptConstParam<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId>, &rustc_data_structures[7f2f9159a885a50a]::steal::Steal<rustc_middle[1a2146a359e71eef]::mir::Body>>>
  32:     0x7f51c5430d9e - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::mir_built, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  33:     0x7f51c4fbf613 - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::mir_built
  34:     0x7f51c4265f8c - rustc_mir_transform[cdef0e8202552a43]::check_unsafety::unsafety_check_result
  35:     0x7f51c4262ecf - <rustc_mir_transform[cdef0e8202552a43]::check_unsafety::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[1a2146a359e71eef]::ty::context::TyCtxt, rustc_span[c623ba6ff50061ca]::def_id::LocalDefId)>>::call_once
  36:     0x7f51c531097e - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId, &rustc_middle[1a2146a359e71eef]::mir::query::UnsafetyCheckResult>>
  37:     0x7f51c540250f - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::unsafety_check_result, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  38:     0x7f51c4fd2084 - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::unsafety_check_result
  39:     0x7f51c41c3afd - rustc_mir_transform[cdef0e8202552a43]::mir_const
  40:     0x7f51c52fcfad - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<rustc_middle[1a2146a359e71eef]::ty::WithOptConstParam<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId>, &rustc_data_structures[7f2f9159a885a50a]::steal::Steal<rustc_middle[1a2146a359e71eef]::mir::Body>>>
  41:     0x7f51c5430ec1 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::mir_const, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  42:     0x7f51c4fbfc83 - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::mir_const
  43:     0x7f51c41c4809 - rustc_mir_transform[cdef0e8202552a43]::mir_promoted
  44:     0x7f51c53e0786 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::mir_promoted, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  45:     0x7f51c4fc2a13 - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::mir_promoted
  46:     0x7f51c49dd6e7 - rustc_borrowck[99b7e83b29d78c3f]::mir_borrowck
  47:     0x7f51c49b0b8f - <rustc_borrowck[99b7e83b29d78c3f]::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[1a2146a359e71eef]::ty::context::TyCtxt, rustc_span[c623ba6ff50061ca]::def_id::LocalDefId)>>::call_once
  48:     0x7f51c530fc3e - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::LocalDefId, &rustc_middle[1a2146a359e71eef]::mir::query::BorrowCheckResult>>
  49:     0x7f51c53dfdce - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::mir_borrowck, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  50:     0x7f51c4fdda34 - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::mir_borrowck
  51:     0x7f51c3ed976b - rustc_hir_analysis[79b65cf038f16269]::collect::type_of::find_opaque_ty_constraints_for_rpit
  52:     0x7f51c3ed8a89 - rustc_hir_analysis[79b65cf038f16269]::collect::type_of::type_of
  53:     0x7f51c5328350 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<rustc_span[c623ba6ff50061ca]::def_id::DefId, rustc_middle[1a2146a359e71eef]::ty::Ty>>
  54:     0x7f51c542ed57 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::type_of, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  55:     0x7f51c4fb686e - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::type_of
  56:     0x7f51c3df4814 - <rustc_privacy[fb74ed5a89b26592]::ReachEverythingInTheInterfaceVisitor>::ty
  57:     0x7f51c3df300a - <rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  58:     0x7f51c3e01bc9 - rustc_hir[61c311f6e1052be3]::intravisit::walk_ty::<rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor>
  59:     0x7f51c3e015f7 - rustc_hir[61c311f6e1052be3]::intravisit::walk_fn::<rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor>
  60:     0x7f51c3e0522d - rustc_hir[61c311f6e1052be3]::intravisit::walk_item::<rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor>
  61:     0x7f51c3df390a - <rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  62:     0x7f51c3e0527d - rustc_hir[61c311f6e1052be3]::intravisit::walk_item::<rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor>
  63:     0x7f51c3df390a - <rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  64:     0x7f51c3e0527d - rustc_hir[61c311f6e1052be3]::intravisit::walk_item::<rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor>
  65:     0x7f51c3df390a - <rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  66:     0x7f51c3e0527d - rustc_hir[61c311f6e1052be3]::intravisit::walk_item::<rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor>
  67:     0x7f51c3df390a - <rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor as rustc_hir[61c311f6e1052be3]::intravisit::Visitor>::visit_item
  68:     0x7f51c3e02f1a - rustc_hir[61c311f6e1052be3]::intravisit::walk_mod::<rustc_privacy[fb74ed5a89b26592]::EmbargoVisitor>
  69:     0x7f51c3dfae5b - rustc_privacy[fb74ed5a89b26592]::effective_visibilities
  70:     0x7f51c53598da - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<(), &rustc_middle[1a2146a359e71eef]::middle::privacy::EffectiveVisibilities>>
  71:     0x7f51c5404a00 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::effective_visibilities, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  72:     0x7f51c4fe517a - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::effective_visibilities
  73:     0x7f51c464eb2b - rustc_passes[657de80b8ae38d58]::stability::check_unused_or_stable_features
  74:     0x7f51c3817040 - <rustc_session[83d455f51924b832]::session::Session>::time::<(), rustc_interface[584e3b1cb250972c]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  75:     0x7f51c380c8e5 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[584e3b1cb250972c]::passes::analysis::{closure#0}::{closure#2}>, ()>
  76:     0x7f51c3817d36 - <rustc_session[83d455f51924b832]::session::Session>::time::<(), rustc_interface[584e3b1cb250972c]::passes::analysis::{closure#0}>
  77:     0x7f51c38415d6 - rustc_interface[584e3b1cb250972c]::passes::analysis
  78:     0x7f51c5351b10 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::try_execute_query::<rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt, rustc_query_system[f71603c5ddc28df0]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>>
  79:     0x7f51c542ee60 - rustc_query_system[f71603c5ddc28df0]::query::plumbing::get_query::<rustc_query_impl[388594bf11bbf687]::queries::analysis, rustc_query_impl[388594bf11bbf687]::plumbing::QueryCtxt>
  80:     0x7f51c4fb754a - <rustc_query_impl[388594bf11bbf687]::Queries as rustc_middle[1a2146a359e71eef]::ty::query::QueryEngine>::analysis
  81:     0x7f51c3713d58 - <rustc_interface[584e3b1cb250972c]::passes::QueryContext>::enter::<rustc_driver[1f62b4d337d0a7e8]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  82:     0x7f51c376aa0b - <rustc_interface[584e3b1cb250972c]::interface::Compiler>::enter::<rustc_driver[1f62b4d337d0a7e8]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[584e3b1cb250972c]::queries::Linker>, rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  83:     0x7f51c36ff2f2 - rustc_span[c623ba6ff50061ca]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_interface[584e3b1cb250972c]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[1f62b4d337d0a7e8]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  84:     0x7f51c375e10c - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[c623ba6ff50061ca]::SessionGlobals>>::set::<rustc_interface[584e3b1cb250972c]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[1f62b4d337d0a7e8]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  85:     0x7f51c371d7fa - std[389b380b19480123]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[584e3b1cb250972c]::util::run_in_thread_pool_with_globals<rustc_interface[584e3b1cb250972c]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[1f62b4d337d0a7e8]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  86:     0x7f51c375f166 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[584e3b1cb250972c]::util::run_in_thread_pool_with_globals<rustc_interface[584e3b1cb250972c]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[1f62b4d337d0a7e8]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>
  87:     0x7f51c370f099 - <<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[584e3b1cb250972c]::util::run_in_thread_pool_with_globals<rustc_interface[584e3b1cb250972c]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>, rustc_driver[1f62b4d337d0a7e8]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[44c6e37299278c93]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  88:     0x7f51c2d8fc8e - std::sys::unix::thread::Thread::new::thread_start::hb7aff98f3d211bd8
  89:     0x7f51c2b25b43 - <unknown>
  90:     0x7f51c2bb7a00 - <unknown>
  91:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (f3e337ac0 2022-11-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
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
Build completed unsuccessfully in 0:03:55
