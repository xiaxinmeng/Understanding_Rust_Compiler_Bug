plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:882:36: cannot convert `ReErased` to a region vid
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1643:9
stack backtrace:
   0:     0x7f821a4031f1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7f821a46ac78 - core::fmt::write::hf3b1e4fb936f95f6
   1:     0x7f821a46ac78 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f821a3f6df1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7f821a403005 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7f821a40607f - std::panicking::default_hook::{{closure}}::h83dd335ad7b1b7e8
   5:     0x7f821a405c3b - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7f821ae647f5 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f821a406879 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7f821d9d67c3 - std[9c6ad123400df44d]::panicking::begin_panic::<rustc_errors[d6bdbfc6e6d0f84]::ExplicitBug>::{closure#0}
   9:     0x7f821d9d3ab6 - std[9c6ad123400df44d]::sys_common::backtrace::__rust_end_short_backtrace::<std[9c6ad123400df44d]::panicking::begin_panic<rustc_errors[d6bdbfc6e6d0f84]::ExplicitBug>::{closure#0}, !>
  10:     0x7f821adfe816 - std[9c6ad123400df44d]::panicking::begin_panic::<rustc_errors[d6bdbfc6e6d0f84]::ExplicitBug>
  11:     0x7f821da88c16 - std[9c6ad123400df44d]::panic::panic_any::<rustc_errors[d6bdbfc6e6d0f84]::ExplicitBug>
  12:     0x7f821da8663a - <rustc_errors[d6bdbfc6e6d0f84]::HandlerInner>::bug::<&alloc[492b206f37fb556b]::string::String>
  13:     0x7f821da862b0 - <rustc_errors[d6bdbfc6e6d0f84]::Handler>::bug::<&alloc[492b206f37fb556b]::string::String>
  14:     0x7f821da56d85 - rustc_middle[1ba1e6a1499ccf69]::util::bug::opt_span_bug_fmt::<rustc_span[de5c6ef7fb663679]::span_encoding::Span>::{closure#0}
  15:     0x7f821da4d60c - rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::with_opt::<rustc_middle[1ba1e6a1499ccf69]::util::bug::opt_span_bug_fmt<rustc_span[de5c6ef7fb663679]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f821da4d5bc - rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::with_context_opt::<rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::with_opt<rustc_middle[1ba1e6a1499ccf69]::util::bug::opt_span_bug_fmt<rustc_span[de5c6ef7fb663679]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f821da56cc9 - rustc_middle[1ba1e6a1499ccf69]::util::bug::opt_span_bug_fmt::<rustc_span[de5c6ef7fb663679]::span_encoding::Span>
  18:     0x7f821ae07565 - rustc_middle[1ba1e6a1499ccf69]::util::bug::bug_fmt
  19:     0x7f821c308c8a - <rustc_borrowck[4331a6e99336b20a]::universal_regions::UniversalRegionIndices>::to_region_vid
  20:     0x7f821c34f8ef - <rustc_borrowck[4331a6e99336b20a]::type_check::relate_tys::NllTypeRelatingDelegate as rustc_infer[7a428d8b44770f2d]::infer::nll_relate::TypeRelatingDelegate>::push_outlives
  21:     0x7f821c211324 - <rustc_infer[7a428d8b44770f2d]::infer::nll_relate::TypeRelating<rustc_borrowck[4331a6e99336b20a]::type_check::relate_tys::NllTypeRelatingDelegate>>::push_outlives
  22:     0x7f821c214245 - <rustc_infer[7a428d8b44770f2d]::infer::nll_relate::TypeRelating<rustc_borrowck[4331a6e99336b20a]::type_check::relate_tys::NllTypeRelatingDelegate> as rustc_middle[1ba1e6a1499ccf69]::ty::relate::TypeRelation>::regions
  23:     0x7f821c46f811 - rustc_middle[1ba1e6a1499ccf69]::ty::relate::super_relate_tys::<rustc_infer[7a428d8b44770f2d]::infer::nll_relate::TypeRelating<rustc_borrowck[4331a6e99336b20a]::type_check::relate_tys::NllTypeRelatingDelegate>>
  24:     0x7f821c30c008 - <rustc_infer[7a428d8b44770f2d]::infer::InferCtxt>::super_combine_tys::<rustc_infer[7a428d8b44770f2d]::infer::nll_relate::TypeRelating<rustc_borrowck[4331a6e99336b20a]::type_check::relate_tys::NllTypeRelatingDelegate>>
  25:     0x7f821c213d57 - <rustc_infer[7a428d8b44770f2d]::infer::nll_relate::TypeRelating<rustc_borrowck[4331a6e99336b20a]::type_check::relate_tys::NllTypeRelatingDelegate> as rustc_middle[1ba1e6a1499ccf69]::ty::relate::TypeRelation>::tys
  26:     0x7f821c22d41e - <rustc_borrowck[4331a6e99336b20a]::type_check::TypeChecker>::relate_types
  27:     0x7f821c21ef74 - <rustc_borrowck[4331a6e99336b20a]::type_check::TypeVerifier as rustc_middle[1ba1e6a1499ccf69]::mir::visit::Visitor>::visit_place
  28:     0x7f821c221472 - <rustc_borrowck[4331a6e99336b20a]::type_check::TypeVerifier as rustc_middle[1ba1e6a1499ccf69]::mir::visit::Visitor>::visit_rvalue
  29:     0x7f821c221aeb - <rustc_borrowck[4331a6e99336b20a]::type_check::TypeVerifier as rustc_middle[1ba1e6a1499ccf69]::mir::visit::Visitor>::visit_body
  30:     0x7f821c21a3c0 - rustc_borrowck[4331a6e99336b20a]::type_check::type_check
  31:     0x7f821c34c37e - rustc_borrowck[4331a6e99336b20a]::nll::compute_regions
  32:     0x7f821c1ecc85 - rustc_borrowck[4331a6e99336b20a]::do_mir_borrowck
  33:     0x7f821c1d5e60 - rustc_borrowck[4331a6e99336b20a]::mir_borrowck
  34:     0x7f821c91e402 - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::enter_context<rustc_query_system[224c9a71c0f6b621]::query::plumbing::execute_job_non_incr<rustc_query_impl[c852027f5badba8d]::queries::mir_borrowck, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 8usize]>>
  35:     0x7f821cb386f4 - rustc_query_system[224c9a71c0f6b621]::query::plumbing::try_execute_query::<rustc_query_impl[c852027f5badba8d]::queries::mir_borrowck, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>
  36:     0x7f821cab8d81 - <rustc_query_impl[c852027f5badba8d]::Queries as rustc_middle[1ba1e6a1499ccf69]::ty::query::QueryEngine>::mir_borrowck
  37:     0x7f821c223667 - <rustc_borrowck[4331a6e99336b20a]::type_check::TypeChecker>::prove_closure_bounds
  38:     0x7f821c233d9a - <rustc_borrowck[4331a6e99336b20a]::type_check::TypeChecker>::check_rvalue
  39:     0x7f821c23a163 - <rustc_borrowck[4331a6e99336b20a]::type_check::TypeChecker>::typeck_mir
  40:     0x7f821c21a3d7 - rustc_borrowck[4331a6e99336b20a]::type_check::type_check
  41:     0x7f821c34c37e - rustc_borrowck[4331a6e99336b20a]::nll::compute_regions
  42:     0x7f821c1ecc85 - rustc_borrowck[4331a6e99336b20a]::do_mir_borrowck
  43:     0x7f821c1d5e60 - rustc_borrowck[4331a6e99336b20a]::mir_borrowck
  44:     0x7f821c91e402 - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::enter_context<rustc_query_system[224c9a71c0f6b621]::query::plumbing::execute_job_non_incr<rustc_query_impl[c852027f5badba8d]::queries::mir_borrowck, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 8usize]>>
  45:     0x7f821cb386f4 - rustc_query_system[224c9a71c0f6b621]::query::plumbing::try_execute_query::<rustc_query_impl[c852027f5badba8d]::queries::mir_borrowck, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>
  46:     0x7f821cab8d81 - <rustc_query_impl[c852027f5badba8d]::Queries as rustc_middle[1ba1e6a1499ccf69]::ty::query::QueryEngine>::mir_borrowck
  47:     0x7f821b714bf6 - rustc_hir_analysis[ea94c40ca1ee02eb]::collect::type_of::find_opaque_ty_constraints_for_rpit
  48:     0x7f821b712726 - rustc_hir_analysis[ea94c40ca1ee02eb]::collect::type_of::type_of
  49:     0x7f821c92a142 - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::enter_context<rustc_query_system[224c9a71c0f6b621]::query::plumbing::execute_job_non_incr<rustc_query_impl[c852027f5badba8d]::queries::type_of, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 8usize]>>
  50:     0x7f821cbd0adc - rustc_query_system[224c9a71c0f6b621]::query::plumbing::try_execute_query::<rustc_query_impl[c852027f5badba8d]::queries::type_of, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>
  51:     0x7f821ca93bab - <rustc_query_impl[c852027f5badba8d]::Queries as rustc_middle[1ba1e6a1499ccf69]::ty::query::QueryEngine>::type_of
  52:     0x7f821b586006 - <rustc_privacy[768983feb062fb4b]::ReachEverythingInTheInterfaceVisitor>::ty
  53:     0x7f821b584e80 - <rustc_privacy[768983feb062fb4b]::EmbargoVisitor as rustc_hir[bf59a86f2ef02f5b]::intravisit::Visitor>::visit_item
  54:     0x7f821b592345 - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_ty::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  55:     0x7f821b591d87 - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_fn::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  56:     0x7f821b595948 - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_item::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  57:     0x7f821b585635 - <rustc_privacy[768983feb062fb4b]::EmbargoVisitor as rustc_hir[bf59a86f2ef02f5b]::intravisit::Visitor>::visit_item
  58:     0x7f821b597536 - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_stmt::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  59:     0x7f821b58c4dc - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_block::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  60:     0x7f821b585943 - <rustc_privacy[768983feb062fb4b]::EmbargoVisitor as rustc_hir[bf59a86f2ef02f5b]::intravisit::Visitor>::visit_block
  61:     0x7f821b591e52 - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_fn::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  62:     0x7f821b58df00 - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_impl_item::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  63:     0x7f821b595e8f - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_item::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  64:     0x7f821b585635 - <rustc_privacy[768983feb062fb4b]::EmbargoVisitor as rustc_hir[bf59a86f2ef02f5b]::intravisit::Visitor>::visit_item
  65:     0x7f821b59599e - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_item::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  66:     0x7f821b585635 - <rustc_privacy[768983feb062fb4b]::EmbargoVisitor as rustc_hir[bf59a86f2ef02f5b]::intravisit::Visitor>::visit_item
  67:     0x7f821b59599e - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_item::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  68:     0x7f821b585635 - <rustc_privacy[768983feb062fb4b]::EmbargoVisitor as rustc_hir[bf59a86f2ef02f5b]::intravisit::Visitor>::visit_item
  69:     0x7f821b59599e - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_item::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  70:     0x7f821b585635 - <rustc_privacy[768983feb062fb4b]::EmbargoVisitor as rustc_hir[bf59a86f2ef02f5b]::intravisit::Visitor>::visit_item
  71:     0x7f821b59334a - rustc_hir[bf59a86f2ef02f5b]::intravisit::walk_mod::<rustc_privacy[768983feb062fb4b]::EmbargoVisitor>
  72:     0x7f821b58ae1b - rustc_privacy[768983feb062fb4b]::effective_visibilities
  73:     0x7f821c925bec - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::enter_context<rustc_query_system[224c9a71c0f6b621]::query::plumbing::execute_job_non_incr<rustc_query_impl[c852027f5badba8d]::queries::effective_visibilities, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 8usize]>>
  74:     0x7f821cb9703d - rustc_query_system[224c9a71c0f6b621]::query::plumbing::try_execute_query::<rustc_query_impl[c852027f5badba8d]::queries::effective_visibilities, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>
  75:     0x7f821cabf976 - <rustc_query_impl[c852027f5badba8d]::Queries as rustc_middle[1ba1e6a1499ccf69]::ty::query::QueryEngine>::effective_visibilities
  76:     0x7f821bf6d40d - rustc_passes[7bf7b024ae06d38d]::stability::check_unused_or_stable_features
  77:     0x7f821af4f1d3 - <rustc_session[995dae908e2471bb]::session::Session>::time::<(), rustc_interface[ce4c417bbf8a4c17]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  78:     0x7f821afcc3f5 - std[9c6ad123400df44d]::panicking::try::<(), core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ce4c417bbf8a4c17]::passes::analysis::{closure#0}::{closure#2}>>
  79:     0x7f821af4fed6 - <rustc_session[995dae908e2471bb]::session::Session>::time::<(), rustc_interface[ce4c417bbf8a4c17]::passes::analysis::{closure#0}>
  80:     0x7f821af1ce5b - rustc_interface[ce4c417bbf8a4c17]::passes::analysis
  81:     0x7f821c92a1cc - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::enter_context<rustc_query_system[224c9a71c0f6b621]::query::plumbing::execute_job_non_incr<rustc_query_impl[c852027f5badba8d]::queries::analysis, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[1ba1e6a1499ccf69]::query::erase::Erased<[u8; 1usize]>>
  82:     0x7f821cbd13f6 - rustc_query_system[224c9a71c0f6b621]::query::plumbing::try_execute_query::<rustc_query_impl[c852027f5badba8d]::queries::analysis, rustc_query_impl[c852027f5badba8d]::plumbing::QueryCtxt>
  83:     0x7f821ca9547a - <rustc_query_impl[c852027f5badba8d]::Queries as rustc_middle[1ba1e6a1499ccf69]::ty::query::QueryEngine>::analysis
  84:     0x7f821aeb622f - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[1ba1e6a1499ccf69]::ty::context::tls::enter_context<<rustc_middle[1ba1e6a1499ccf69]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  85:     0x7f821aea4b31 - <rustc_interface[ce4c417bbf8a4c17]::queries::QueryResult<&rustc_middle[1ba1e6a1499ccf69]::ty::context::GlobalCtxt>>::enter::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  86:     0x7f821aeb714c - <rustc_interface[ce4c417bbf8a4c17]::interface::Compiler>::enter::<rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[ce4c417bbf8a4c17]::queries::Linker>, rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  87:     0x7f821aed46d0 - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  88:     0x7f821ae76dda - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  89:     0x7f821aeda058 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  90:     0x7f821ae77f96 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  91:     0x7f821a412abe - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  92:     0x7f821a1afb43 - <unknown>
  93:     0x7f821a241a00 - <unknown>
  94:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (4ed18daba 2023-04-22) running on x86_64-unknown-linux-gnu


note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `iter::adapters::enumerate::<impl at library/core/src/iter/adapters/enumerate.rs:29:1: 29:34>::try_fold::enumerate::{closure#0}`
#1 [mir_borrowck] borrow-checking `iter::adapters::enumerate::<impl at library/core/src/iter/adapters/enumerate.rs:29:1: 29:34>::try_fold::enumerate`
#2 [type_of] computing type of `iter::adapters::enumerate::<impl at library/core/src/iter/adapters/enumerate.rs:29:1: 29:34>::try_fold::enumerate::{opaque#1}`
#3 [effective_visibilities] checking effective visibilities
#4 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:03:02
