plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_middle/src/ty/context.rs:308:9: node root_crate with HirId::owner OwnerId { def_id: DefId(0:0 ~ core[58dc]) } cannot be placed in TypeckResults with hir_owner OwnerId { def_id: DefId(0:6308 ~ core[58dc]::iter::adapters::cloned::clone_try_fold) }
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1551:9
stack backtrace:
stack backtrace:
   0:     0x7fbb77921572 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   1:     0x7fbb7798f308 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7fbb77911da1 - std::io::Write::write_fmt::h9fe4e6d9c9b927ef
   3:     0x7fbb77921335 - std::sys_common::backtrace::print::hf38d1633e21dbba9
   4:     0x7fbb77924717 - std::panicking::default_hook::{{closure}}::h12022b1a20dd35ce
   5:     0x7fbb77924475 - std::panicking::default_hook::h5ab5b9712723f5dd
   6:     0x7fbb782a3854 - rustc_driver[b2f871249018b53f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fbb77925023 - std::panicking::rust_panic_with_hook::h57cd9b8bb3f6a82b
   8:     0x7fbb7ac9ae83 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[c6390cd32a21b219]::ExplicitBug>::{closure#0}
   9:     0x7fbb7ac9ab76 - std[389b380b19480123]::sys_common::backtrace::__rust_end_short_backtrace::<std[389b380b19480123]::panicking::begin_panic<rustc_errors[c6390cd32a21b219]::ExplicitBug>::{closure#0}, !>
  10:     0x7fbb7824ab06 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[c6390cd32a21b219]::ExplicitBug>
  11:     0x7fbb7ad38f56 - std[389b380b19480123]::panic::panic_any::<rustc_errors[c6390cd32a21b219]::ExplicitBug>
  12:     0x7fbb7ad34127 - <rustc_errors[c6390cd32a21b219]::HandlerInner>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  13:     0x7fbb7ad33c10 - <rustc_errors[c6390cd32a21b219]::Handler>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  14:     0x7fbb7adfb3d0 - rustc_middle[890156e5e48c7d12]::ty::context::tls::with_context_opt::<rustc_middle[890156e5e48c7d12]::ty::context::tls::with_opt<rustc_middle[890156e5e48c7d12]::util::bug::opt_span_bug_fmt<rustc_span[b4ca3e966db910d5]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7fbb7adfc629 - rustc_middle[890156e5e48c7d12]::util::bug::opt_span_bug_fmt::<rustc_span[b4ca3e966db910d5]::span_encoding::Span>
  16:     0x7fbb7824fef5 - rustc_middle[890156e5e48c7d12]::util::bug::bug_fmt
  17:     0x7fbb7824ad5d - rustc_middle[890156e5e48c7d12]::ty::context::invalid_hir_id_for_typeck_results
  18:     0x7fbb7882d2a1 - <rustc_middle[890156e5e48c7d12]::ty::context::LocalTableInContext<rustc_middle[890156e5e48c7d12]::ty::sty::FnSig>>::get
  19:     0x7fbb78794d12 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::body_fn_sig
  20:     0x7fbb787768a3 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_return_expr
  21:     0x7fbb788da6de - rustc_hir_typeck[58ea70b4d60fb259]::check::check_fn
  22:     0x7fbb787ceee5 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_expr_closure
  23:     0x7fbb787d6e6e - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_expr_kind
  24:     0x7fbb78775557 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  25:     0x7fbb787d5e12 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  26:     0x7fbb787904d1 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_block_with_expected
  27:     0x7fbb787d7083 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_expr_kind
  28:     0x7fbb78775557 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7fbb787d5e12 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  30:     0x7fbb787767d1 - <rustc_hir_typeck[58ea70b4d60fb259]::fn_ctxt::FnCtxt>::check_return_expr
  31:     0x7fbb788da6de - rustc_hir_typeck[58ea70b4d60fb259]::check::check_fn
  32:     0x7fbb788d7425 - <rustc_hir_typeck[58ea70b4d60fb259]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[58ea70b4d60fb259]::typeck_with_fallback<rustc_hir_typeck[58ea70b4d60fb259]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[890156e5e48c7d12]::ty::context::TypeckResults>
  33:     0x7fbb78890a02 - rustc_hir_typeck[58ea70b4d60fb259]::typeck
  34:     0x7fbb79eba90e - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId, &rustc_middle[890156e5e48c7d12]::ty::context::TypeckResults>>
  35:     0x7fbb79fda4df - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::typeck, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  36:     0x7fbb79b87074 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::typeck
  37:     0x7fbb7ad8cabf - <rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt>::typeck_opt_const_arg
  38:     0x7fbb7933c8b5 - rustc_mir_build[478ec3939b5ed996]::thir::cx::thir_body
  39:     0x7fbb79fde535 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::thir_body, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  40:     0x7fbb79b69278 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::thir_body
  41:     0x7fbb792caf79 - rustc_mir_build[478ec3939b5ed996]::build::mir_built
  42:     0x7fbb79ea984d - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_middle[890156e5e48c7d12]::ty::WithOptConstParam<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId>, &rustc_data_structures[d4b8fe12b073a5bc]::steal::Steal<rustc_middle[890156e5e48c7d12]::mir::Body>>>
  43:     0x7fbb79fdc7ae - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::mir_built, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  44:     0x7fbb79b6b2c3 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::mir_built
  45:     0x7fbb78e0e8dc - rustc_mir_transform[dac78d353aec7647]::check_unsafety::unsafety_check_result
  46:     0x7fbb78e0b80f - <rustc_mir_transform[dac78d353aec7647]::check_unsafety::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt, rustc_span[b4ca3e966db910d5]::def_id::LocalDefId)>>::call_once
  47:     0x7fbb79ebc38e - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId, &rustc_middle[890156e5e48c7d12]::mir::query::UnsafetyCheckResult>>
  48:     0x7fbb79fadf1f - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::unsafety_check_result, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  49:     0x7fbb79b7dd34 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::unsafety_check_result
  50:     0x7fbb78d6c26d - rustc_mir_transform[dac78d353aec7647]::mir_const
  51:     0x7fbb79ea984d - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_middle[890156e5e48c7d12]::ty::WithOptConstParam<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId>, &rustc_data_structures[d4b8fe12b073a5bc]::steal::Steal<rustc_middle[890156e5e48c7d12]::mir::Body>>>
  52:     0x7fbb79fdc8d1 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::mir_const, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  53:     0x7fbb79b6b933 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::mir_const
  54:     0x7fbb78d6cf79 - rustc_mir_transform[dac78d353aec7647]::mir_promoted
  55:     0x7fbb79f8c196 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::mir_promoted, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  56:     0x7fbb79b6e6c3 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::mir_promoted
  57:     0x7fbb79584e17 - rustc_borrowck[6e0694bc0540a036]::mir_borrowck
  58:     0x7fbb795584df - <rustc_borrowck[6e0694bc0540a036]::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt, rustc_span[b4ca3e966db910d5]::def_id::LocalDefId)>>::call_once
  59:     0x7fbb79ebb64e - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId, &rustc_middle[890156e5e48c7d12]::mir::query::BorrowCheckResult>>
  60:     0x7fbb79f8b7de - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::mir_borrowck, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  61:     0x7fbb79b896e4 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::mir_borrowck
  62:     0x7fbb78a810cb - rustc_hir_analysis[7606f196e15db70c]::collect::type_of::find_opaque_ty_constraints_for_rpit
  63:     0x7fbb78a803e9 - rustc_hir_analysis[7606f196e15db70c]::collect::type_of::type_of
  64:     0x7fbb79ed67a0 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::DefId, rustc_middle[890156e5e48c7d12]::ty::Ty>>
  65:     0x7fbb79fda767 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::type_of, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  66:     0x7fbb79b6251e - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::type_of
  67:     0x7fbb7899e234 - <rustc_privacy[4c9068ec0bd5a190]::ReachEverythingInTheInterfaceVisitor>::ty
  68:     0x7fbb7899ca2a - <rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor as rustc_hir[dddaa83688e4b55]::intravisit::Visitor>::visit_item
  69:     0x7fbb789ab5e9 - rustc_hir[dddaa83688e4b55]::intravisit::walk_ty::<rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor>
  70:     0x7fbb789ab017 - rustc_hir[dddaa83688e4b55]::intravisit::walk_fn::<rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor>
  71:     0x7fbb789aec4d - rustc_hir[dddaa83688e4b55]::intravisit::walk_item::<rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor>
  72:     0x7fbb7899d32a - <rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor as rustc_hir[dddaa83688e4b55]::intravisit::Visitor>::visit_item
  73:     0x7fbb789aec9d - rustc_hir[dddaa83688e4b55]::intravisit::walk_item::<rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor>
  74:     0x7fbb7899d32a - <rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor as rustc_hir[dddaa83688e4b55]::intravisit::Visitor>::visit_item
  75:     0x7fbb789aec9d - rustc_hir[dddaa83688e4b55]::intravisit::walk_item::<rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor>
  76:     0x7fbb7899d32a - <rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor as rustc_hir[dddaa83688e4b55]::intravisit::Visitor>::visit_item
  77:     0x7fbb789aec9d - rustc_hir[dddaa83688e4b55]::intravisit::walk_item::<rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor>
  78:     0x7fbb7899d32a - <rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor as rustc_hir[dddaa83688e4b55]::intravisit::Visitor>::visit_item
  79:     0x7fbb789ac93a - rustc_hir[dddaa83688e4b55]::intravisit::walk_mod::<rustc_privacy[4c9068ec0bd5a190]::EmbargoVisitor>
  80:     0x7fbb789a487b - rustc_privacy[4c9068ec0bd5a190]::effective_visibilities
  81:     0x7fbb79f052ea - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<(), &rustc_middle[890156e5e48c7d12]::middle::privacy::EffectiveVisibilities>>
  82:     0x7fbb79fb0410 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::effective_visibilities, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  83:     0x7fbb79b90e2a - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::effective_visibilities
  84:     0x7fbb791f753b - rustc_passes[b8738d6650e87629]::stability::check_unused_or_stable_features
  85:     0x7fbb783bd760 - <rustc_session[9207d45749bb4341]::session::Session>::time::<(), rustc_interface[f22dc26c8a2764b8]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  86:     0x7fbb783b3165 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f22dc26c8a2764b8]::passes::analysis::{closure#0}::{closure#2}>, ()>
  87:     0x7fbb783be456 - <rustc_session[9207d45749bb4341]::session::Session>::time::<(), rustc_interface[f22dc26c8a2764b8]::passes::analysis::{closure#0}>
  88:     0x7fbb783e7e46 - rustc_interface[f22dc26c8a2764b8]::passes::analysis
  89:     0x7fbb79efd520 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>>
  90:     0x7fbb79fda870 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::analysis, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  91:     0x7fbb79b631fa - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::analysis
  92:     0x7fbb782fdd59 - <rustc_interface[f22dc26c8a2764b8]::passes::QueryContext>::enter::<rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  93:     0x7fbb7831100b - <rustc_interface[f22dc26c8a2764b8]::interface::Compiler>::enter::<rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[f22dc26c8a2764b8]::queries::Linker>, rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  94:     0x7fbb782a4f22 - rustc_span[b4ca3e966db910d5]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  95:     0x7fbb783040dc - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[b4ca3e966db910d5]::SessionGlobals>>::set::<rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  96:     0x7fbb782c2eca - std[389b380b19480123]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f22dc26c8a2764b8]::util::run_in_thread_pool_with_globals<rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  97:     0x7fbb78305106 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[f22dc26c8a2764b8]::util::run_in_thread_pool_with_globals<rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  98:     0x7fbb782b5489 - <<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[f22dc26c8a2764b8]::util::run_in_thread_pool_with_globals<rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  99:     0x7fbb77931c8e - std::sys::unix::thread::Thread::new::thread_start::hb7aff98f3d211bd8
 100:     0x7fbb776c7b43 - <unknown>
 101:     0x7fbb77759a00 - <unknown>
 102:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (b7e438d9d 2022-11-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `iter::adapters::cloned::clone_try_fold`
#1 [thir_body] building THIR for `iter::adapters::cloned::clone_try_fold`
#2 [mir_built] building MIR for `iter::adapters::cloned::clone_try_fold`
#3 [unsafety_check_result] unsafety-checking `iter::adapters::cloned::clone_try_fold`
#4 [mir_const] preparing `iter::adapters::cloned::clone_try_fold` for borrow checking
#5 [mir_promoted] processing MIR for `iter::adapters::cloned::clone_try_fold`
#6 [mir_borrowck] borrow-checking `iter::adapters::cloned::clone_try_fold`
#7 [type_of] computing type of `iter::adapters::cloned::clone_try_fold::{opaque#1}`
#8 [effective_visibilities] checking effective visibilities
#9 [analysis] running analysis passes on this crate
error: could not compile `core`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:09
