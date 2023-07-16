plain
failures:

---- [ui] tests/ui/async-await/issue-70935-complex-spans.rs#drop_tracking_mir stdout ----

error in revision `drop_tracking_mir`: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issue-70935-complex-spans.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking_mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking_mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking_mir/auxiliary" "--edition=2018" "-Zdrop-tracking-mir"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: NoSolution', compiler/rustc_borrowck/src/type_check/relate_tys.rs:199:14
   0:     0x7f4fc01f4494 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8db7d2ed480dd475
   1:     0x7f4fc025b128 - core::fmt::write::hfe5354a0b0217639
   2:     0x7f4fc01e8c41 - std::io::Write::write_fmt::h719d1e56fe2d1021
   3:     0x7f4fc01f42a1 - std::sys_common::backtrace::print::h26c74a2e648b900b
   3:     0x7f4fc01f42a1 - std::sys_common::backtrace::print::h26c74a2e648b900b
   4:     0x7f4fc01f73fa - std::panicking::default_hook::{{closure}}::hed6d5b3c1f7b9006
   5:     0x7f4fc01f70dc - std::panicking::default_hook::h81f2f3511a6a5ee0
   6:     0x7f4fc0c86f5b - rustc_driver_impl[715243229a8ddb67]::install_ice_hook::{closure#0}
   7:     0x7f4fc01f7b07 - std::panicking::rust_panic_with_hook::hd4e47e03f820bcbb
   8:     0x7f4fc01f7887 - std::panicking::begin_panic_handler::{{closure}}::habca8daf6eb19650
   9:     0x7f4fc01f4956 - std::sys_common::backtrace::__rust_end_short_backtrace::h5dc4325383f05def
  10:     0x7f4fc01f7577 - rust_begin_unwind
  11:     0x7f4fc01ac0d3 - core::panicking::panic_fmt::ha058aed329cefc28
  12:     0x7f4fc01ac6a3 - core::result::unwrap_failed::hb2517db8981baa30
  13:     0x7f4fc21f7317 - <rustc_borrowck[eac207c551e2650c]::type_check::relate_tys::NllTypeRelatingDelegate as rustc_infer[f8001351d85c9691]::infer::nll_relate::TypeRelatingDelegate>::register_obligations
  14:     0x7f4fc23b1358 - <rustc_infer[f8001351d85c9691]::infer::nll_relate::TypeRelating<rustc_borrowck[eac207c551e2650c]::type_check::relate_tys::NllTypeRelatingDelegate>>::relate_opaques
  15:     0x7f4fc23b755c - <rustc_infer[f8001351d85c9691]::infer::nll_relate::TypeRelating<rustc_borrowck[eac207c551e2650c]::type_check::relate_tys::NllTypeRelatingDelegate> as rustc_middle[a3aa3efdb4f7202f]::ty::relate::TypeRelation>::tys
  16:     0x7f4fc226e08f - <rustc_borrowck[eac207c551e2650c]::type_check::TypeChecker>::relate_types
  17:     0x7f4fc227a4a2 - <rustc_borrowck[eac207c551e2650c]::type_check::TypeChecker>::typeck_mir
  18:     0x7f4fc22578e1 - rustc_borrowck[eac207c551e2650c]::type_check::type_check
  19:     0x7f4fc222ae38 - rustc_borrowck[eac207c551e2650c]::nll::compute_regions
  20:     0x7f4fc20d3b75 - rustc_borrowck[eac207c551e2650c]::do_mir_borrowck
  21:     0x7f4fc20bed80 - rustc_borrowck[eac207c551e2650c]::mir_borrowck
  22:     0x7f4fc27bb78c - rustc_query_impl[44bb8cf2de23388a]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[44bb8cf2de23388a]::dynamic_query::mir_borrowck::{closure#2}::{closure#0}, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 8usize]>>
  23:     0x7f4fc275bc5c - <rustc_query_impl[44bb8cf2de23388a]::dynamic_query::mir_borrowck::{closure#2} as core[6d2ebebbb2674f83]::ops::function::FnOnce<(rustc_middle[a3aa3efdb4f7202f]::ty::context::TyCtxt, rustc_span[11c544ef48bdaed5]::def_id::LocalDefId)>>::call_once
  24:     0x7f4fc2953fe8 - rustc_query_system[d197a506a024d615]::query::plumbing::try_execute_query::<rustc_query_impl[44bb8cf2de23388a]::DynamicConfig<rustc_query_system[d197a506a024d615]::query::caches::VecCache<rustc_span[11c544ef48bdaed5]::def_id::LocalDefId, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[44bb8cf2de23388a]::plumbing::QueryCtxt, false>
  25:     0x7f4fc26732b6 - rustc_query_impl[44bb8cf2de23388a]::get_query_non_incr::mir_borrowck::__rust_end_short_backtrace
  26:     0x7f4fc1590f39 - rustc_middle[a3aa3efdb4f7202f]::query::plumbing::query_get_at::<rustc_query_system[d197a506a024d615]::query::caches::VecCache<rustc_span[11c544ef48bdaed5]::def_id::LocalDefId, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 8usize]>>>
  27:     0x7f4fc159e4ae - rustc_hir_analysis[937c69cd2ecbfdaf]::collect::type_of::find_opaque_ty_constraints_for_rpit
  28:     0x7f4fc159cb92 - rustc_hir_analysis[937c69cd2ecbfdaf]::collect::type_of::type_of
  29:     0x7f4fc27bfdee - rustc_query_impl[44bb8cf2de23388a]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[44bb8cf2de23388a]::dynamic_query::type_of::{closure#2}::{closure#0}, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 8usize]>>
  30:     0x7f4fc2778bf0 - <rustc_query_impl[44bb8cf2de23388a]::dynamic_query::type_of::{closure#2} as core[6d2ebebbb2674f83]::ops::function::FnOnce<(rustc_middle[a3aa3efdb4f7202f]::ty::context::TyCtxt, rustc_span[11c544ef48bdaed5]::def_id::DefId)>>::call_once
  31:     0x7f4fc290c578 - rustc_query_system[d197a506a024d615]::query::plumbing::try_execute_query::<rustc_query_impl[44bb8cf2de23388a]::DynamicConfig<rustc_query_system[d197a506a024d615]::query::caches::DefaultCache<rustc_span[11c544ef48bdaed5]::def_id::DefId, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[44bb8cf2de23388a]::plumbing::QueryCtxt, false>
  32:     0x7f4fc266e5d7 - rustc_query_impl[44bb8cf2de23388a]::get_query_non_incr::type_of::__rust_end_short_backtrace
  33:     0x7f4fc1432c3d - rustc_middle[a3aa3efdb4f7202f]::query::plumbing::query_get_at::<rustc_query_system[d197a506a024d615]::query::caches::DefaultCache<rustc_span[11c544ef48bdaed5]::def_id::DefId, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 8usize]>>>
  34:     0x7f4fc1438f6f - rustc_hir_analysis[937c69cd2ecbfdaf]::check::check::check_opaque
  35:     0x7f4fc143da31 - rustc_hir_analysis[937c69cd2ecbfdaf]::check::check::check_item_type
  36:     0x7f4fc144571a - rustc_hir_analysis[937c69cd2ecbfdaf]::check::check::check_mod_item_types
  37:     0x7f4fc27bdc0c - rustc_query_impl[44bb8cf2de23388a]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[44bb8cf2de23388a]::dynamic_query::check_mod_item_types::{closure#2}::{closure#0}, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 0usize]>>
  38:     0x7f4fc276a1cc - <rustc_query_impl[44bb8cf2de23388a]::dynamic_query::check_mod_item_types::{closure#2} as core[6d2ebebbb2674f83]::ops::function::FnOnce<(rustc_middle[a3aa3efdb4f7202f]::ty::context::TyCtxt, rustc_span[11c544ef48bdaed5]::def_id::LocalDefId)>>::call_once
  39:     0x7f4fc2949a44 - rustc_query_system[d197a506a024d615]::query::plumbing::try_execute_query::<rustc_query_impl[44bb8cf2de23388a]::DynamicConfig<rustc_query_system[d197a506a024d615]::query::caches::VecCache<rustc_span[11c544ef48bdaed5]::def_id::LocalDefId, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[44bb8cf2de23388a]::plumbing::QueryCtxt, false>
  40:     0x7f4fc26ad0cc - rustc_query_impl[44bb8cf2de23388a]::get_query_non_incr::check_mod_item_types::__rust_end_short_backtrace
  41:     0x7f4fc14238e4 - <rustc_middle[a3aa3efdb4f7202f]::hir::map::Map>::for_each_module::<rustc_hir_analysis[937c69cd2ecbfdaf]::check_crate::{closure#6}::{closure#0}>
  42:     0x7f4fc15e78a5 - <rustc_session[3467d102a85f177f]::session::Session>::time::<(), rustc_hir_analysis[937c69cd2ecbfdaf]::check_crate::{closure#6}>
  43:     0x7f4fc15665d1 - rustc_hir_analysis[937c69cd2ecbfdaf]::check_crate
  44:     0x7f4fc0d599f9 - rustc_interface[341f59238701a14c]::passes::analysis
  45:     0x7f4fc27bfe0a - rustc_query_impl[44bb8cf2de23388a]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[44bb8cf2de23388a]::dynamic_query::analysis::{closure#2}::{closure#0}, rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 1usize]>>
  46:     0x7f4fc2778de8 - <rustc_query_impl[44bb8cf2de23388a]::dynamic_query::analysis::{closure#2} as core[6d2ebebbb2674f83]::ops::function::FnOnce<(rustc_middle[a3aa3efdb4f7202f]::ty::context::TyCtxt, ())>>::call_once
  47:     0x7f4fc28c1042 - rustc_query_system[d197a506a024d615]::query::plumbing::try_execute_query::<rustc_query_impl[44bb8cf2de23388a]::DynamicConfig<rustc_query_system[d197a506a024d615]::query::caches::SingleCache<rustc_middle[a3aa3efdb4f7202f]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[44bb8cf2de23388a]::plumbing::QueryCtxt, false>
  48:     0x7f4fc26ac6bd - rustc_query_impl[44bb8cf2de23388a]::get_query_non_incr::analysis::__rust_end_short_backtrace
  49:     0x7f4fc0c9ac7d - <rustc_middle[a3aa3efdb4f7202f]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[715243229a8ddb67]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>
  50:     0x7f4fc0cd2638 - <rustc_interface[341f59238701a14c]::interface::Compiler>::enter::<rustc_driver_impl[715243229a8ddb67]::run_compiler::{closure#1}::{closure#2}, core[6d2ebebbb2674f83]::result::Result<core[6d2ebebbb2674f83]::option::Option<rustc_interface[341f59238701a14c]::queries::Linker>, rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>
  51:     0x7f4fc0c99540 - rustc_span[11c544ef48bdaed5]::set_source_map::<core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>, rustc_interface[341f59238701a14c]::interface::run_compiler<core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>, rustc_driver_impl[715243229a8ddb67]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  52:     0x7f4fc0c90ef9 - <scoped_tls[b02b2ed0bd888163]::ScopedKey<rustc_span[11c544ef48bdaed5]::SessionGlobals>>::set::<rustc_interface[341f59238701a14c]::interface::run_compiler<core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>, rustc_driver_impl[715243229a8ddb67]::run_compiler::{closure#1}>::{closure#0}, core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>
  53:     0x7f4fc0ca3ef6 - std[f5fb6b9d37451821]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[341f59238701a14c]::util::run_in_thread_pool_with_globals<rustc_interface[341f59238701a14c]::interface::run_compiler<core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>, rustc_driver_impl[715243229a8ddb67]::run_compiler::{closure#1}>::{closure#0}, core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>
  54:     0x7f4fc0ce5af6 - std[f5fb6b9d37451821]::panicking::try::<core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>, core[6d2ebebbb2674f83]::panic::unwind_safe::AssertUnwindSafe<<std[f5fb6b9d37451821]::thread::Builder>::spawn_unchecked_<rustc_interface[341f59238701a14c]::util::run_in_thread_pool_with_globals<rustc_interface[341f59238701a14c]::interface::run_compiler<core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>, rustc_driver_impl[715243229a8ddb67]::run_compiler::{closure#1}>::{closure#0}, core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  55:     0x7f4fc0c8e0bd - <<std[f5fb6b9d37451821]::thread::Builder>::spawn_unchecked_<rustc_interface[341f59238701a14c]::util::run_in_thread_pool_with_globals<rustc_interface[341f59238701a14c]::interface::run_compiler<core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>, rustc_driver_impl[715243229a8ddb67]::run_compiler::{closure#1}>::{closure#0}, core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6d2ebebbb2674f83]::result::Result<(), rustc_span[11c544ef48bdaed5]::ErrorGuaranteed>>::{closure#1} as core[6d2ebebbb2674f83]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f4fc020443e - std::sys::unix::thread::Thread::new::thread_start::h955de6c33d04e195
  57:     0x7f4fbff9fb43 - <unknown>
  58:     0x7f4fc0031a00 - <unknown>
  59:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
Build completed unsuccessfully in 0:12:24
Build completed unsuccessfully in 0:12:24

note: rustc 1.71.0-nightly (1a3a9a6e6 2023-05-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z drop-tracking-mir
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `foo`
#1 [type_of] computing type of `foo::{opaque#0}`
#2 [check_mod_item_types] checking item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: errors selecting obligation during MIR typeck: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<std::sync::mpsc::Sender<i32> as std::marker::Sync>, polarity:Positive), []), depth=1),Unimplemented)]
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
              2: <rustc_infer::infer::InferCtxt>::commit_if_ok::<(), rustc_middle::traits::query::NoSolution, rustc_trait_selection::traits::query::type_op::custom::scrape_region_constraints<rustc_borrowck::type_check::InstantiateOpaqueType, (), <rustc_borrowck::type_check::InstantiateOpaqueType as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform::{closure#0}>::{closure#0}>
              3: rustc_trait_selection::traits::query::type_op::custom::scrape_region_constraints::<rustc_borrowck::type_check::InstantiateOpaqueType, (), <rustc_borrowck::type_check::InstantiateOpaqueType as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform::{closure#0}>
              4: <rustc_borrowck::type_check::InstantiateOpaqueType as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform
              5: <rustc_borrowck::type_check::TypeChecker>::fully_perform_op::<(), rustc_borrowck::type_check::InstantiateOpaqueType>
              6: <rustc_borrowck::type_check::relate_tys::NllTypeRelatingDelegate as rustc_infer::infer::nll_relate::TypeRelatingDelegate>::register_obligations
              7: <rustc_infer::infer::nll_relate::TypeRelating<rustc_borrowck::type_check::relate_tys::NllTypeRelatingDelegate>>::relate_opaques
              8: <rustc_infer::infer::nll_relate::TypeRelating<rustc_borrowck::type_check::relate_tys::NllTypeRelatingDelegate> as rustc_middle::ty::relate::TypeRelation>::tys
              9: <rustc_borrowck::type_check::TypeChecker>::relate_types
             10: <rustc_borrowck::type_check::TypeChecker>::typeck_mir
             11: rustc_borrowck::type_check::type_check
             12: rustc_borrowck::nll::compute_regions
             13: rustc_borrowck::do_mir_borrowck
             14: rustc_borrowck::mir_borrowck
             15: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::dynamic_query::mir_borrowck::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
             16: <rustc_query_impl::dynamic_query::mir_borrowck::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
             17: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
             18: rustc_query_impl::get_query_non_incr::mir_borrowck::__rust_end_short_backtrace
             19: rustc_middle::query::plumbing::query_get_at::<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
             20: rustc_hir_analysis::collect::type_of::find_opaque_ty_constraints_for_rpit
             21: rustc_hir_analysis::collect::type_of::type_of
             22: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::dynamic_query::type_of::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
             23: <rustc_query_impl::dynamic_query::type_of::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId)>>::call_once
             24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
             25: rustc_query_impl::get_query_non_incr::type_of::__rust_end_short_backtrace
             26: rustc_middle::query::plumbing::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
             27: rustc_hir_analysis::check::check::check_opaque
             28: rustc_hir_analysis::check::check::check_item_type
             29: rustc_hir_analysis::check::check::check_mod_item_types
             30: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::dynamic_query::check_mod_item_types::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 0]>>
             31: <rustc_query_impl::dynamic_query::check_mod_item_types::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
             32: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 0]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
             33: rustc_query_impl::get_query_non_incr::check_mod_item_types::__rust_end_short_backtrace
             34: <rustc_middle::hir::map::Map>::for_each_module::<rustc_hir_analysis::check_crate::{closure#6}::{closure#0}>
             35: <rustc_session::session::Session>::time::<(), rustc_hir_analysis::check_crate::{closure#6}>
             36: rustc_hir_analysis::check_crate
             37: rustc_interface::passes::analysis
             38: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::dynamic_query::analysis::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
             39: <rustc_query_impl::dynamic_query::analysis::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, ())>>::call_once
             40: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
             41: rustc_query_impl::get_query_non_incr::analysis::__rust_end_short_backtrace
             42: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             44: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             45: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             45: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             46: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             47: std::panicking::try::<core::result::Result<(), rustc_span::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             48: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             50: <unknown>
             51: <unknown>
           


error: internal compiler error: {OpaqueTypeKey { def_id: DefId(0:15 ~ issue_70935_complex_spans[27d3]::foo::{opaque#0}), substs: [] }: OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: fake-test-src-base/async-await/issue-70935-complex-spans.rs:16:5: 20:6 (#0), ty: [async block@fake-test-src-base/async-await/issue-70935-complex-spans.rs:16:5: 20:6] }, origin: FnReturn(DefId(0:7 ~ issue_70935_complex_spans[27d3]::foo)) }}
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_session::session::Session>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
              2: <rustc_infer::infer::opaque_types::table::OpaqueTypeStorage as core::ops::drop::Drop>::drop
              3: core::ptr::drop_in_place::<rustc_infer::infer::opaque_types::table::OpaqueTypeStorage>
              3: core::ptr::drop_in_place::<rustc_infer::infer::opaque_types::table::OpaqueTypeStorage>
              4: core::ptr::drop_in_place::<rustc_infer::infer::InferCtxt>
              5: rustc_borrowck::mir_borrowck
              6: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::dynamic_query::mir_borrowck::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
              7: <rustc_query_impl::dynamic_query::mir_borrowck::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
              8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
              9: rustc_query_impl::get_query_non_incr::mir_borrowck::__rust_end_short_backtrace
             10: rustc_middle::query::plumbing::query_get_at::<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
             11: rustc_hir_analysis::collect::type_of::find_opaque_ty_constraints_for_rpit
             12: rustc_hir_analysis::collect::type_of::type_of
             13: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::dynamic_query::type_of::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
             14: <rustc_query_impl::dynamic_query::type_of::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId)>>::call_once
             15: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
             16: rustc_query_impl::get_query_non_incr::type_of::__rust_end_short_backtrace
             17: rustc_middle::query::plumbing::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
             18: rustc_hir_analysis::check::check::check_opaque
             19: rustc_hir_analysis::check::check::check_item_type
             20: rustc_hir_analysis::check::check::check_mod_item_types
             21: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::dynamic_query::check_mod_item_types::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 0]>>
             22: <rustc_query_impl::dynamic_query::check_mod_item_types::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
             23: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 0]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
             24: rustc_query_impl::get_query_non_incr::check_mod_item_types::__rust_end_short_backtrace
             25: <rustc_middle::hir::map::Map>::for_each_module::<rustc_hir_analysis::check_crate::{closure#6}::{closure#0}>
             26: <rustc_session::session::Session>::time::<(), rustc_hir_analysis::check_crate::{closure#6}>
             27: rustc_hir_analysis::check_crate
             28: rustc_interface::passes::analysis
             29: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::dynamic_query::analysis::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
             30: <rustc_query_impl::dynamic_query::analysis::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, ())>>::call_once
             31: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
             32: rustc_query_impl::get_query_non_incr::analysis::__rust_end_short_backtrace
             33: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             35: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             36: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             36: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             37: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             38: std::panicking::try::<core::result::Result<(), rustc_span::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             39: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             41: <unknown>
             42: <unknown>
           


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (1a3a9a6e6 2023-05-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z drop-tracking-mir
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
