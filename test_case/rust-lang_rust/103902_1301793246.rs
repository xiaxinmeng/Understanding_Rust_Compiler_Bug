plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ce1a7e41f96be27103b6e3ba09dcefbf5bd320bd and 8e846bfc1905dc449c8d100b63a2168f13bd1350
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
error: internal compiler error: /checkout/compiler/rustc_middle/src/hir/map/mod.rs:175:13: local_def_id: no entry for `HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[e778]::iter::adapters::cloned::clone_try_fold) }, local_id: 20 }`, which has a map of `Some(Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[e778]::iter::adapters::cloned::clone_try_fold) }, local_id: 20 }, kind: Block(Block { stmts: [], expr: Some(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[e778]::iter::adapters::cloned::clone_try_fold) }, local_id: 18 }, kind: Closure(Closure { binder: Default, capture_clause: Value, bound_generic_params: [], fn_decl: FnDecl { inputs: [Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[e778]::iter::adapters::cloned::clone_try_fold) }, local_id: 16 }, kind: Infer, span: library/core/src/iter/adapters/cloned.rs:28:11: 28:14 (#0) }, Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[e778]::iter::adapters::cloned::clone_try_fold) }, local_id: 17 }, kind: Infer, span: library/core/src/iter/adapters/cloned.rs:28:16: 28:19 (#0) }], output: DefaultReturn(library/core/src/iter/adapters/cloned.rs:28:21: 28:21 (#0)), c_variadic: false, implicit_self: None }, body: BodyId { hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[e778]::iter::adapters::cloned::clone_try_fold) }, local_id: 15 } }, fn_decl_span: library/core/src/iter/adapters/cloned.rs:28:5: 28:20 (#0), movability: None }), span: library/core/src/iter/adapters/cloned.rs:28:5: 28:40 (#0) }), hir_id: HirId { owner: OwnerId { def_id: DefId(0:6260 ~ core[e778]::iter::adapters::cloned::clone_try_fold) }, local_id: 19 }, rules: DefaultBlock, span: library/core/src/iter/adapters/cloned.rs:27:97: 29:2 (#0), targeted_by_break: false }, None), span: library/core/src/iter/adapters/cloned.rs:27:97: 29:2 (#0) }))`
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1551:9
stack backtrace:
   0:     0x7f36c279df52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbef12973c5507d53
   1:     0x7f36c28057c8 - core::fmt::write::h6d6e96066401bc0f
   1:     0x7f36c28057c8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f36c278e631 - std::io::Write::write_fmt::h0fc5efafbc8ce341
   3:     0x7f36c279dd15 - std::sys_common::backtrace::print::h7f5fe1a3e38a3041
   4:     0x7f36c27a10f7 - std::panicking::default_hook::{{closure}}::h62c63d72dd478c99
   5:     0x7f36c27a0e55 - std::panicking::default_hook::h61d2d5a20d5abab2
   6:     0x7f36c311d476 - rustc_driver[220d861fe5665444]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f36c27a1a03 - std::panicking::rust_panic_with_hook::h6fb0b7b5144c30a1
   8:     0x7f36c64545d3 - std[b4d7b7572d6e9bb7]::panicking::begin_panic::<rustc_errors[319628db51427ac0]::ExplicitBug>::{closure#0}
   9:     0x7f36c644d256 - std[b4d7b7572d6e9bb7]::sys_common::backtrace::__rust_end_short_backtrace::<std[b4d7b7572d6e9bb7]::panicking::begin_panic<rustc_errors[319628db51427ac0]::ExplicitBug>::{closure#0}, !>
  10:     0x7f36c30b9836 - std[b4d7b7572d6e9bb7]::panicking::begin_panic::<rustc_errors[319628db51427ac0]::ExplicitBug>
  11:     0x7f36c6301636 - std[b4d7b7572d6e9bb7]::panic::panic_any::<rustc_errors[319628db51427ac0]::ExplicitBug>
  12:     0x7f36c62fd617 - <rustc_errors[319628db51427ac0]::HandlerInner>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  13:     0x7f36c62fcd90 - <rustc_errors[319628db51427ac0]::Handler>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  14:     0x7f36c64a85f0 - rustc_middle[df7c89aebf5ff77c]::ty::context::tls::with_context_opt::<rustc_middle[df7c89aebf5ff77c]::ty::context::tls::with_opt<rustc_middle[df7c89aebf5ff77c]::util::bug::opt_span_bug_fmt<rustc_span[316458062c9a7e81]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f36c64a9d69 - rustc_middle[df7c89aebf5ff77c]::util::bug::opt_span_bug_fmt::<rustc_span[316458062c9a7e81]::span_encoding::Span>
  16:     0x7f36c30bf175 - rustc_middle[df7c89aebf5ff77c]::util::bug::bug_fmt
  17:     0x7f36c36609b9 - rustc_ty_utils[249b3a327f3403a3]::ty::param_env
  18:     0x7f36c5291315 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_span[316458062c9a7e81]::def_id::DefId, rustc_middle[df7c89aebf5ff77c]::ty::ParamEnv>>
  19:     0x7f36c53a7853 - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::param_env, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  20:     0x7f36c4ecc7be - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::param_env
  21:     0x7f36c38c5bcb - <rustc_hir_typeck[52ccc0f71f4ad54]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[52ccc0f71f4ad54]::typeck_with_fallback<rustc_hir_typeck[52ccc0f71f4ad54]::typeck::{closure#0}>::{closure#1}, &rustc_middle[df7c89aebf5ff77c]::ty::context::TypeckResults>
  22:     0x7f36c38f7abc - rustc_hir_typeck[52ccc0f71f4ad54]::typeck
  23:     0x7f36c526caf1 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_span[316458062c9a7e81]::def_id::LocalDefId, &rustc_middle[df7c89aebf5ff77c]::ty::context::TypeckResults>>
  24:     0x7f36c53a62af - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::typeck, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  25:     0x7f36c4eb5414 - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::typeck
  26:     0x7f36c64254bb - <rustc_middle[df7c89aebf5ff77c]::ty::context::TyCtxt>::typeck_opt_const_arg
  27:     0x7f36c4501f8b - rustc_mir_build[27569b958f77a66c]::thir::cx::thir_body
  28:     0x7f36c5249f06 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_middle[df7c89aebf5ff77c]::ty::WithOptConstParam<rustc_span[316458062c9a7e81]::def_id::LocalDefId>, core[69c2305d6fa5d54f]::result::Result<(&rustc_data_structures[d503008fd0bcf4b6]::steal::Steal<rustc_middle[df7c89aebf5ff77c]::thir::Thir>, rustc_middle[df7c89aebf5ff77c]::thir::ExprId), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>>
  29:     0x7f36c53a797f - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::thir_body, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  30:     0x7f36c4e97618 - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::thir_body
  31:     0x7f36c4480730 - rustc_mir_build[27569b958f77a66c]::build::mir_built
  32:     0x7f36c524b1be - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_middle[df7c89aebf5ff77c]::ty::WithOptConstParam<rustc_span[316458062c9a7e81]::def_id::LocalDefId>, &rustc_data_structures[d503008fd0bcf4b6]::steal::Steal<rustc_middle[df7c89aebf5ff77c]::mir::Body>>>
  33:     0x7f36c53a74de - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::mir_built, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  34:     0x7f36c4e99663 - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::mir_built
  35:     0x7f36c3eb4f9c - rustc_mir_transform[c5a9c5bc67e700f9]::check_unsafety::unsafety_check_result
  36:     0x7f36c3eb1056 - <rustc_mir_transform[c5a9c5bc67e700f9]::check_unsafety::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[df7c89aebf5ff77c]::ty::context::TyCtxt, rustc_span[316458062c9a7e81]::def_id::LocalDefId)>>::call_once
  37:     0x7f36c526ed11 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_span[316458062c9a7e81]::def_id::LocalDefId, &rustc_middle[df7c89aebf5ff77c]::mir::query::UnsafetyCheckResult>>
  38:     0x7f36c539d77f - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::unsafety_check_result, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  39:     0x7f36c4eac0d4 - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::unsafety_check_result
  40:     0x7f36c3df5ad0 - rustc_mir_transform[c5a9c5bc67e700f9]::mir_const
  41:     0x7f36c524b1be - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_middle[df7c89aebf5ff77c]::ty::WithOptConstParam<rustc_span[316458062c9a7e81]::def_id::LocalDefId>, &rustc_data_structures[d503008fd0bcf4b6]::steal::Steal<rustc_middle[df7c89aebf5ff77c]::mir::Body>>>
  42:     0x7f36c53a7601 - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::mir_const, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  43:     0x7f36c4e99cd3 - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::mir_const
  44:     0x7f36c3df68c0 - rustc_mir_transform[c5a9c5bc67e700f9]::mir_promoted
  45:     0x7f36c524c437 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_middle[df7c89aebf5ff77c]::ty::WithOptConstParam<rustc_span[316458062c9a7e81]::def_id::LocalDefId>, (&rustc_data_structures[d503008fd0bcf4b6]::steal::Steal<rustc_middle[df7c89aebf5ff77c]::mir::Body>, &rustc_data_structures[d503008fd0bcf4b6]::steal::Steal<rustc_index[96bf19e46d9e2c50]::vec::IndexVec<rustc_middle[df7c89aebf5ff77c]::mir::Promoted, rustc_middle[df7c89aebf5ff77c]::mir::Body>>)>>
  46:     0x7f36c5390f5f - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::mir_promoted, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  47:     0x7f36c4e9ca63 - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::mir_promoted
  48:     0x7f36c479c9bb - rustc_borrowck[232374be1aec7b9a]::mir_borrowck
  49:     0x7f36c47681f6 - <rustc_borrowck[232374be1aec7b9a]::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[df7c89aebf5ff77c]::ty::context::TyCtxt, rustc_span[316458062c9a7e81]::def_id::LocalDefId)>>::call_once
  50:     0x7f36c526dc01 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_span[316458062c9a7e81]::def_id::LocalDefId, &rustc_middle[df7c89aebf5ff77c]::mir::query::BorrowCheckResult>>
  51:     0x7f36c5390cee - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::mir_borrowck, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  52:     0x7f36c4eb7a84 - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::mir_borrowck
  53:     0x7f36c3a93b4a - rustc_hir_analysis[63a9ce6a4a5ac4a9]::collect::type_of::find_opaque_ty_constraints_for_rpit
  54:     0x7f36c3a92e6a - rustc_hir_analysis[63a9ce6a4a5ac4a9]::collect::type_of::type_of
  55:     0x7f36c5290105 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_span[316458062c9a7e81]::def_id::DefId, rustc_middle[df7c89aebf5ff77c]::ty::Ty>>
  56:     0x7f36c53a6537 - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::type_of, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  57:     0x7f36c4e908be - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::type_of
  58:     0x7f36c398736b - <rustc_privacy[b9b4d32b0b72d79f]::ReachEverythingInTheInterfaceVisitor>::ty
  59:     0x7f36c3985ab1 - <rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_item
  60:     0x7f36c3996e59 - rustc_hir[bc81097ff660bad4]::intravisit::walk_ty::<rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor>
  61:     0x7f36c3996807 - rustc_hir[bc81097ff660bad4]::intravisit::walk_fn::<rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor>
  62:     0x7f36c399aa5d - rustc_hir[bc81097ff660bad4]::intravisit::walk_item::<rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor>
  63:     0x7f36c3985bb9 - <rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_item
  64:     0x7f36c399aaad - rustc_hir[bc81097ff660bad4]::intravisit::walk_item::<rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor>
  65:     0x7f36c3985bb9 - <rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_item
  66:     0x7f36c399aaad - rustc_hir[bc81097ff660bad4]::intravisit::walk_item::<rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor>
  67:     0x7f36c3985bb9 - <rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_item
  68:     0x7f36c399aaad - rustc_hir[bc81097ff660bad4]::intravisit::walk_item::<rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor>
  69:     0x7f36c3985bb9 - <rustc_privacy[b9b4d32b0b72d79f]::EmbargoVisitor as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_item
  70:     0x7f36c398e2ff - rustc_privacy[b9b4d32b0b72d79f]::effective_visibilities
  71:     0x7f36c52fa02a - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<(), &rustc_middle[df7c89aebf5ff77c]::middle::privacy::EffectiveVisibilities>>
  72:     0x7f36c539de50 - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::effective_visibilities, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  73:     0x7f36c4ebf1ca - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::effective_visibilities
  74:     0x7f36c4379061 - rustc_passes[51346f78edff3f7e]::stability::check_unused_or_stable_features
  75:     0x7f36c32bb7ad - <rustc_session[3283e5b6d6e5bbce]::session::Session>::time::<(), rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#0}>
  76:     0x7f36c330a7a9 - rustc_interface[155c1abf899b1ba8]::passes::analysis
  77:     0x7f36c52effe2 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>>
  78:     0x7f36c53a6640 - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::analysis, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  79:     0x7f36c4e9159a - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::analysis
  80:     0x7f36c319d6a3 - <rustc_interface[155c1abf899b1ba8]::passes::QueryContext>::enter::<rustc_driver[220d861fe5665444]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  81:     0x7f36c31b7ed2 - <rustc_interface[155c1abf899b1ba8]::interface::Compiler>::enter::<rustc_driver[220d861fe5665444]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[155c1abf899b1ba8]::queries::Linker>, rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  82:     0x7f36c311efe2 - rustc_span[316458062c9a7e81]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  83:     0x7f36c31a6993 - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[316458062c9a7e81]::SessionGlobals>>::set::<rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  84:     0x7f36c314f8f0 - std[b4d7b7572d6e9bb7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[155c1abf899b1ba8]::util::run_in_thread_pool_with_globals<rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  85:     0x7f36c3135f69 - <<std[b4d7b7572d6e9bb7]::thread::Builder>::spawn_unchecked_<rustc_interface[155c1abf899b1ba8]::util::run_in_thread_pool_with_globals<rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  86:     0x7f36c27ae66e - std::sys::unix::thread::Thread::new::thread_start::h05afa8442b34c479
  87:     0x7f36c2549b43 - <unknown>
  88:     0x7f36c25dba00 - <unknown>
  89:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (8e846bfc1 2022-11-03) running on x86_64-unknown-linux-gnu

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
Build completed unsuccessfully in 0:03:42
cat: /tmp/toolstate/toolstates.json: No such file or directory
