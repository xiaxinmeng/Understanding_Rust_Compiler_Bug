
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/compiler/rustc_errors/src/lib.rs:1551:9
stack backtrace:
   0:     0x7fca1533c3b0 - std::backtrace_rs::backtrace::libunwind::trace::h158b1ea709fc4e63
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fca1533c3b0 - std::backtrace_rs::backtrace::trace_unsynchronized::h9ddac896e1f23a6d
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fca1533c3b0 - std::sys_common::backtrace::_print_fmt::h08fd40ba3280fb37
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fca1533c3b0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc19a1f8ccfa0fcac
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fca114705ae - core::fmt::write::hb3853ff9af72c572
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fca15330395 - std::io::Write::write_fmt::h0fec3892489cf66f
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/io/mod.rs:1682:15
   6:     0x7fca1533c175 - std::sys_common::backtrace::_print::h1cdb877dbc7b463b
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fca1533c175 - std::sys_common::backtrace::print::he8d97843d257de9f
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fca1533e48f - std::panicking::default_hook::{{closure}}::h1b15307f2f12dd5c
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/panicking.rs:267:22
   9:     0x7fca1533e1ca - std::panicking::default_hook::h723dcdd3247d3ec2
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/panicking.rs:286:9
  10:     0x7fca145d73b1 - rustc_driver[3718276c0bc98693]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fca1533ec7d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h39a2a965904fa559
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/alloc/src/boxed.rs:2032:9
  12:     0x7fca1533ec7d - std::panicking::rust_panic_with_hook::h9559defc1382972e
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/panicking.rs:692:13
  13:     0x7fca14a12d71 - std[67d4a95a05869a39]::panicking::begin_panic::<rustc_errors[cdb7c2de71d369ae]::ExplicitBug>::{closure#0}
  14:     0x7fca14a12006 - std[67d4a95a05869a39]::sys_common::backtrace::__rust_end_short_backtrace::<std[67d4a95a05869a39]::panicking::begin_panic<rustc_errors[cdb7c2de71d369ae]::ExplicitBug>::{closure#0}, !>
  15:     0x7fca14a7b4a6 - std[67d4a95a05869a39]::panicking::begin_panic::<rustc_errors[cdb7c2de71d369ae]::ExplicitBug>
  16:     0x7fca14a7b496 - std[67d4a95a05869a39]::panic::panic_any::<rustc_errors[cdb7c2de71d369ae]::ExplicitBug>
  17:     0x7fca14a7a254 - <rustc_errors[cdb7c2de71d369ae]::HandlerInner>::bug::<&alloc[b45379bec5d81bc1]::string::String>
  18:     0x7fca14a79e20 - <rustc_errors[cdb7c2de71d369ae]::Handler>::bug::<&alloc[b45379bec5d81bc1]::string::String>
  19:     0x7fca14ae0cbd - rustc_middle[3b781ed61bfbd995]::ty::context::tls::with_context_opt::<rustc_middle[3b781ed61bfbd995]::ty::context::tls::with_opt<rustc_middle[3b781ed61bfbd995]::util::bug::opt_span_bug_fmt<rustc_span[adf92f1e3c211125]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7fca14ae0de6 - rustc_middle[3b781ed61bfbd995]::util::bug::opt_span_bug_fmt::<rustc_span[adf92f1e3c211125]::span_encoding::Span>
  21:     0x7fca128d48a3 - rustc_middle[3b781ed61bfbd995]::util::bug::bug_fmt
  22:     0x7fca12a01bd0 - rustc_middle[3b781ed61bfbd995]::ty::cast::mir_cast_kind
  23:     0x7fca129d4114 - <rustc_mir_build[1c3c0fb4b3f086ec]::build::Builder>::as_rvalue
  24:     0x7fca129e2416 - <rustc_mir_build[1c3c0fb4b3f086ec]::build::Builder>::expr_into_dest
  25:     0x7fca129e22fb - <rustc_mir_build[1c3c0fb4b3f086ec]::build::Builder>::expr_into_dest
  26:     0x7fca129e22fb - <rustc_mir_build[1c3c0fb4b3f086ec]::build::Builder>::expr_into_dest
  27:     0x7fca1377f1d8 - rustc_mir_build[1c3c0fb4b3f086ec]::build::mir_built
  28:     0x7fca12f65c29 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_middle[3b781ed61bfbd995]::ty::WithOptConstParam<rustc_span[adf92f1e3c211125]::def_id::LocalDefId>, &rustc_data_structures[e38fda9cb534705d]::steal::Steal<rustc_middle[3b781ed61bfbd995]::mir::Body>>>
  29:     0x7fca12f71d5e - rustc_mir_transform[542801fa7db5faae]::check_unsafety::unsafety_check_result
  30:     0x7fca12f6b398 - <rustc_mir_transform[542801fa7db5faae]::check_unsafety::provide::{closure#0} as core[b4b6f3a0ae56718b]::ops::function::FnOnce<(rustc_middle[3b781ed61bfbd995]::ty::context::TyCtxt, rustc_span[adf92f1e3c211125]::def_id::LocalDefId)>>::call_once
  31:     0x7fca12f6a32f - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_span[adf92f1e3c211125]::def_id::LocalDefId, &rustc_middle[3b781ed61bfbd995]::mir::query::UnsafetyCheckResult>>
  32:     0x7fca12f68361 - rustc_mir_transform[542801fa7db5faae]::mir_const
  33:     0x7fca12f65f95 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_middle[3b781ed61bfbd995]::ty::WithOptConstParam<rustc_span[adf92f1e3c211125]::def_id::LocalDefId>, &rustc_data_structures[e38fda9cb534705d]::steal::Steal<rustc_middle[3b781ed61bfbd995]::mir::Body>>>
  34:     0x7fca13fea580 - <rustc_query_impl[67fab5f6eb3a8b8e]::Queries as rustc_middle[3b781ed61bfbd995]::ty::query::QueryEngine>::mir_const
  35:     0x7fca134b4f25 - rustc_mir_transform[542801fa7db5faae]::mir_const_qualif
  36:     0x7fca134b38c0 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_span[adf92f1e3c211125]::def_id::DefId, rustc_middle[3b781ed61bfbd995]::mir::query::ConstQualifs>>
  37:     0x7fca134b018c - rustc_mir_transform[542801fa7db5faae]::mir_promoted
  38:     0x7fca134ae6f5 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_middle[3b781ed61bfbd995]::ty::WithOptConstParam<rustc_span[adf92f1e3c211125]::def_id::LocalDefId>, (&rustc_data_structures[e38fda9cb534705d]::steal::Steal<rustc_middle[3b781ed61bfbd995]::mir::Body>, &rustc_data_structures[e38fda9cb534705d]::steal::Steal<rustc_index[c47057c8f04c10f4]::vec::IndexVec<rustc_middle[3b781ed61bfbd995]::mir::Promoted, rustc_middle[3b781ed61bfbd995]::mir::Body>>)>>
  39:     0x7fca134accc0 - rustc_borrowck[383a36f275a895fb]::mir_borrowck
  40:     0x7fca13775e3c - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_span[adf92f1e3c211125]::def_id::LocalDefId, &rustc_middle[3b781ed61bfbd995]::mir::query::BorrowCheckResult>>
  41:     0x7fca13febed8 - <rustc_query_impl[67fab5f6eb3a8b8e]::Queries as rustc_middle[3b781ed61bfbd995]::ty::query::QueryEngine>::mir_borrowck
  42:     0x7fca12827f3b - rustc_mir_transform[542801fa7db5faae]::mir_drops_elaborated_and_const_checked
  43:     0x7fca12f65c29 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_middle[3b781ed61bfbd995]::ty::WithOptConstParam<rustc_span[adf92f1e3c211125]::def_id::LocalDefId>, &rustc_data_structures[e38fda9cb534705d]::steal::Steal<rustc_middle[3b781ed61bfbd995]::mir::Body>>>
  44:     0x7fca13fea81e - <rustc_query_impl[67fab5f6eb3a8b8e]::Queries as rustc_middle[3b781ed61bfbd995]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  45:     0x7fca132f48f5 - rustc_mir_transform[542801fa7db5faae]::inner_mir_for_ctfe
  46:     0x7fca132f4202 - rustc_mir_transform[542801fa7db5faae]::mir_for_ctfe
  47:     0x7fca12f1a666 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_span[adf92f1e3c211125]::def_id::DefId, &rustc_middle[3b781ed61bfbd995]::mir::Body>>
  48:     0x7fca13fea94f - <rustc_query_impl[67fab5f6eb3a8b8e]::Queries as rustc_middle[3b781ed61bfbd995]::ty::query::QueryEngine>::mir_for_ctfe
  49:     0x7fca135f562f - <rustc_const_eval[dd77448ea3c2216]::interpret::eval_context::InterpCx<rustc_const_eval[dd77448ea3c2216]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  50:     0x7fca135efaef - rustc_const_eval[dd77448ea3c2216]::const_eval::eval_queries::eval_to_allocation_raw_provider
  51:     0x7fca139e6dab - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_middle[3b781ed61bfbd995]::ty::ParamEnvAnd<rustc_middle[3b781ed61bfbd995]::mir::interpret::GlobalId>, core[b4b6f3a0ae56718b]::result::Result<rustc_middle[3b781ed61bfbd995]::mir::interpret::value::ConstAlloc, rustc_middle[3b781ed61bfbd995]::mir::interpret::error::ErrorHandled>>>
  52:     0x7fca13fec186 - <rustc_query_impl[67fab5f6eb3a8b8e]::Queries as rustc_middle[3b781ed61bfbd995]::ty::query::QueryEngine>::eval_to_allocation_raw
  53:     0x7fca135f061b - rustc_const_eval[dd77448ea3c2216]::const_eval::eval_queries::eval_to_allocation_raw_provider
  54:     0x7fca139e6dab - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_middle[3b781ed61bfbd995]::ty::ParamEnvAnd<rustc_middle[3b781ed61bfbd995]::mir::interpret::GlobalId>, core[b4b6f3a0ae56718b]::result::Result<rustc_middle[3b781ed61bfbd995]::mir::interpret::value::ConstAlloc, rustc_middle[3b781ed61bfbd995]::mir::interpret::error::ErrorHandled>>>
  55:     0x7fca13fec186 - <rustc_query_impl[67fab5f6eb3a8b8e]::Queries as rustc_middle[3b781ed61bfbd995]::ty::query::QueryEngine>::eval_to_allocation_raw
  56:     0x7fca13cdf638 - rustc_const_eval[dd77448ea3c2216]::const_eval::eval_to_valtree
  57:     0x7fca13cdf470 - <rustc_const_eval[dd77448ea3c2216]::provide::{closure#0} as core[b4b6f3a0ae56718b]::ops::function::FnOnce<(rustc_middle[3b781ed61bfbd995]::ty::context::TyCtxt, rustc_middle[3b781ed61bfbd995]::ty::ParamEnvAnd<rustc_middle[3b781ed61bfbd995]::mir::interpret::GlobalId>)>>::call_once
  58:     0x7fca13c7a734 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_middle[3b781ed61bfbd995]::ty::ParamEnvAnd<rustc_middle[3b781ed61bfbd995]::mir::interpret::GlobalId>, core[b4b6f3a0ae56718b]::result::Result<core[b4b6f3a0ae56718b]::option::Option<rustc_middle[3b781ed61bfbd995]::ty::consts::valtree::ValTree>, rustc_middle[3b781ed61bfbd995]::mir::interpret::error::ErrorHandled>>>
  59:     0x7fca13c7a2b1 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::get_query::<rustc_query_impl[67fab5f6eb3a8b8e]::queries::eval_to_valtree, rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt>
  60:     0x7fca13c7a1d1 - <rustc_query_impl[67fab5f6eb3a8b8e]::Queries as rustc_middle[3b781ed61bfbd995]::ty::query::QueryEngine>::eval_to_valtree
  61:     0x7fca12cfb05b - <rustc_middle[3b781ed61bfbd995]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  62:     0x7fca12cfa9d1 - <rustc_middle[3b781ed61bfbd995]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  63:     0x7fca12cfa334 - <rustc_middle[3b781ed61bfbd995]::ty::consts::Const>::eval
  64:     0x7fca12cf68a5 - <rustc_trait_selection[3658a109e551de35]::traits::project::AssocTypeNormalizer as rustc_middle[3b781ed61bfbd995]::ty::fold::TypeFolder>::fold_const
  65:     0x7fca12cd809a - <rustc_trait_selection[3658a109e551de35]::traits::project::AssocTypeNormalizer as rustc_middle[3b781ed61bfbd995]::ty::fold::TypeFolder>::fold_ty
  66:     0x7fca1322e466 - <rustc_trait_selection[3658a109e551de35]::traits::engine::ObligationCtxt>::normalize::<rustc_middle[3b781ed61bfbd995]::ty::Ty>
  67:     0x7fca13245831 - rustc_hir_analysis[a461a26ae41891db]::check::wfcheck::check_type_defn
  68:     0x7fca13225a3e - rustc_hir_analysis[a461a26ae41891db]::check::wfcheck::check_well_formed
  69:     0x7fca127db545 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::get_query::<rustc_query_impl[67fab5f6eb3a8b8e]::queries::check_well_formed, rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt>
  70:     0x7fca13ac36a9 - rustc_data_structures[e38fda9cb534705d]::sync::par_for_each_in::<&[rustc_hir[7f5703e8cfa51d6b]::hir::ItemId], <rustc_middle[3b781ed61bfbd995]::hir::ModuleItems>::par_items<rustc_hir_analysis[a461a26ae41891db]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  71:     0x7fca13f10bc1 - rustc_hir_analysis[a461a26ae41891db]::check::wfcheck::check_mod_type_wf
  72:     0x7fca12f54368 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<rustc_span[adf92f1e3c211125]::def_id::LocalDefId, ()>>
  73:     0x7fca13a413b9 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::get_query::<rustc_query_impl[67fab5f6eb3a8b8e]::queries::check_mod_type_wf, rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt>
  74:     0x7fca12a35ef8 - rustc_data_structures[e38fda9cb534705d]::sync::par_for_each_in::<&[rustc_hir[7f5703e8cfa51d6b]::hir_id::OwnerId], <rustc_middle[3b781ed61bfbd995]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[a461a26ae41891db]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  75:     0x7fca12a35d23 - <rustc_session[dc0ce365742b6185]::session::Session>::track_errors::<rustc_hir_analysis[a461a26ae41891db]::check_crate::{closure#5}, ()>
  76:     0x7fca12a35681 - rustc_hir_analysis[a461a26ae41891db]::check_crate
  77:     0x7fca12a3532b - rustc_interface[8edb24c6a1fd808]::passes::analysis
  78:     0x7fca13e004a4 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::try_execute_query::<rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt, rustc_query_system[e265f4acbd77df8f]::query::caches::DefaultCache<(), core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>>>
  79:     0x7fca13e001a7 - rustc_query_system[e265f4acbd77df8f]::query::plumbing::get_query::<rustc_query_impl[67fab5f6eb3a8b8e]::queries::analysis, rustc_query_impl[67fab5f6eb3a8b8e]::plumbing::QueryCtxt>
  80:     0x7fca138c92ce - <rustc_interface[8edb24c6a1fd808]::passes::QueryContext>::enter::<rustc_driver[3718276c0bc98693]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>>
  81:     0x7fca138c639f - <rustc_interface[8edb24c6a1fd808]::interface::Compiler>::enter::<rustc_driver[3718276c0bc98693]::run_compiler::{closure#1}::{closure#2}, core[b4b6f3a0ae56718b]::result::Result<core[b4b6f3a0ae56718b]::option::Option<rustc_interface[8edb24c6a1fd808]::queries::Linker>, rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>>
  82:     0x7fca138c13d2 - rustc_span[adf92f1e3c211125]::with_source_map::<core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>, rustc_interface[8edb24c6a1fd808]::interface::run_compiler<core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>, rustc_driver[3718276c0bc98693]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  83:     0x7fca138c0eac - <scoped_tls[9f2ac4408a1c304c]::ScopedKey<rustc_span[adf92f1e3c211125]::SessionGlobals>>::set::<rustc_interface[8edb24c6a1fd808]::interface::run_compiler<core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>, rustc_driver[3718276c0bc98693]::run_compiler::{closure#1}>::{closure#0}, core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>>
  84:     0x7fca138c0498 - std[67d4a95a05869a39]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8edb24c6a1fd808]::util::run_in_thread_pool_with_globals<rustc_interface[8edb24c6a1fd808]::interface::run_compiler<core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>, rustc_driver[3718276c0bc98693]::run_compiler::{closure#1}>::{closure#0}, core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>>
  85:     0x7fca138c01bc - <<std[67d4a95a05869a39]::thread::Builder>::spawn_unchecked_<rustc_interface[8edb24c6a1fd808]::util::run_in_thread_pool_with_globals<rustc_interface[8edb24c6a1fd808]::interface::run_compiler<core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>, rustc_driver[3718276c0bc98693]::run_compiler::{closure#1}>::{closure#0}, core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4b6f3a0ae56718b]::result::Result<(), rustc_errors[cdb7c2de71d369ae]::ErrorGuaranteed>>::{closure#1} as core[b4b6f3a0ae56718b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  86:     0x7fca15345ba3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfaebe704c4eee862
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/alloc/src/boxed.rs:2000:9
  87:     0x7fca15345ba3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h923a5bacfa0b39e6
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/alloc/src/boxed.rs:2000:9
  88:     0x7fca15345ba3 - std::sys::unix::thread::Thread::new::thread_start::he482a3376f21a92c
                               at /rustc/6284998a2677d7e3e8420db783f3aa4fd80d7423/library/std/src/sys/unix/thread.rs:108:17
  89:     0x7fca112e4609 - start_thread
  90:     0x7fca11207133 - clone
  91:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (6284998a2 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_built] building MIR for `S::0::{constant#0}`
#1 [unsafety_check_result] unsafety-checking `S::0::{constant#0}`
#2 [mir_const] preparing `S::0::{constant#0}` for borrow checking
#3 [mir_const_qualif] const checking `S::0::{constant#0}`
#4 [mir_promoted] processing MIR for `S::0::{constant#0}`
#5 [mir_borrowck] borrow-checking `S::0::{constant#0}`
#6 [mir_drops_elaborated_and_const_checked] elaborating drops for `S::0::{constant#0}`
#7 [mir_for_ctfe] caching mir of `S::0::{constant#0}` for CTFE
#8 [eval_to_allocation_raw] const-evaluating + checking `S::0::{constant#0}`
#9 [eval_to_allocation_raw] const-evaluating + checking `S::0::{constant#0}`
#10 [eval_to_valtree] evaluating type-level constant
#11 [check_well_formed] checking that `S` is well-formed
#12 [check_mod_type_wf] checking that types are well-formed in top-level module
#13 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0412`.
