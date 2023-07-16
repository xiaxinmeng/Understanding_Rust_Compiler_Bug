plain
failures:

---- [ui] tests/ui/async-await/issue-70935-complex-spans.rs#drop_tracking_mir stdout ----

error in revision `drop_tracking_mir`: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issue-70935-complex-spans.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking_mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking_mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking_mir/auxiliary" "--edition=2018" "-Zdrop-tracking-mir"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: NoSolution', compiler/rustc_borrowck/src/type_check/relate_tys.rs:199:14
   0:     0x7fe323c32494 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6bc80e6de091fbc9
   1:     0x7fe323c99128 - core::fmt::write::h9f370158d1a22b89
   2:     0x7fe323c26ba1 - std::io::Write::write_fmt::h95de5a3ecddf8383
   3:     0x7fe323c322a1 - std::sys_common::backtrace::print::hbb123dceb9eed724
   3:     0x7fe323c322a1 - std::sys_common::backtrace::print::hbb123dceb9eed724
   4:     0x7fe323c353fa - std::panicking::default_hook::{{closure}}::hc9c8debe46e1ce3d
   5:     0x7fe323c350dc - std::panicking::default_hook::hc0586bbe2f2f2386
   6:     0x7fe3246c545b - rustc_driver_impl[6ed5859210d32a08]::install_ice_hook::{closure#0}
   7:     0x7fe323c35b07 - std::panicking::rust_panic_with_hook::hae0532d926418cf4
   8:     0x7fe323c35887 - std::panicking::begin_panic_handler::{{closure}}::h0ce0da498ac82511
   9:     0x7fe323c32956 - std::sys_common::backtrace::__rust_end_short_backtrace::h97b4960b2f0acceb
  10:     0x7fe323c35577 - rust_begin_unwind
  11:     0x7fe323bea0d3 - core::panicking::panic_fmt::h8b0b014d3a57b80f
  12:     0x7fe323bea6a3 - core::result::unwrap_failed::h9dc96968a35d7c53
  13:     0x7fe325c3d7f7 - <rustc_borrowck[ffb7a4a53609196a]::type_check::relate_tys::NllTypeRelatingDelegate as rustc_infer[85c683ed41f31a2f]::infer::nll_relate::TypeRelatingDelegate>::register_obligations
  14:     0x7fe325df4c98 - <rustc_infer[85c683ed41f31a2f]::infer::nll_relate::TypeRelating<rustc_borrowck[ffb7a4a53609196a]::type_check::relate_tys::NllTypeRelatingDelegate>>::relate_opaques
  15:     0x7fe325dfb274 - <rustc_infer[85c683ed41f31a2f]::infer::nll_relate::TypeRelating<rustc_borrowck[ffb7a4a53609196a]::type_check::relate_tys::NllTypeRelatingDelegate> as rustc_middle[80e851c391dd4f58]::ty::relate::TypeRelation>::tys
  16:     0x7fe325cb0b7f - <rustc_borrowck[ffb7a4a53609196a]::type_check::TypeChecker>::relate_types
  17:     0x7fe325cbcf82 - <rustc_borrowck[ffb7a4a53609196a]::type_check::TypeChecker>::typeck_mir
  18:     0x7fe325c99cb9 - rustc_borrowck[ffb7a4a53609196a]::type_check::type_check
  19:     0x7fe325c6baf7 - rustc_borrowck[ffb7a4a53609196a]::nll::compute_regions
  20:     0x7fe325b13b35 - rustc_borrowck[ffb7a4a53609196a]::do_mir_borrowck
  21:     0x7fe325afed40 - rustc_borrowck[ffb7a4a53609196a]::mir_borrowck
  22:     0x7fe3261fc4fc - rustc_query_impl[69d5ff43c637cc62]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[69d5ff43c637cc62]::dynamic_query::mir_borrowck::{closure#2}::{closure#0}, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 8usize]>>
  23:     0x7fe32619c84c - <rustc_query_impl[69d5ff43c637cc62]::dynamic_query::mir_borrowck::{closure#2} as core[7d9be083f2db1]::ops::function::FnOnce<(rustc_middle[80e851c391dd4f58]::ty::context::TyCtxt, rustc_span[9d65595379dd9678]::def_id::LocalDefId)>>::call_once
  24:     0x7fe3263987a8 - rustc_query_system[e8692c0e875adc1f]::query::plumbing::try_execute_query::<rustc_query_impl[69d5ff43c637cc62]::DynamicConfig<rustc_query_system[e8692c0e875adc1f]::query::caches::VecCache<rustc_span[9d65595379dd9678]::def_id::LocalDefId, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[69d5ff43c637cc62]::plumbing::QueryCtxt, false>
  25:     0x7fe3260b6cb6 - rustc_query_impl[69d5ff43c637cc62]::get_query_non_incr::mir_borrowck::__rust_end_short_backtrace
  26:     0x7fe324fd3259 - rustc_middle[80e851c391dd4f58]::query::plumbing::query_get_at::<rustc_query_system[e8692c0e875adc1f]::query::caches::VecCache<rustc_span[9d65595379dd9678]::def_id::LocalDefId, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 8usize]>>>
  27:     0x7fe324fe22f1 - rustc_hir_analysis[522899e829a14f4e]::collect::type_of::find_opaque_ty_constraints_for_rpit
  28:     0x7fe324fe07c2 - rustc_hir_analysis[522899e829a14f4e]::collect::type_of::type_of
  29:     0x7fe326200b5e - rustc_query_impl[69d5ff43c637cc62]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[69d5ff43c637cc62]::dynamic_query::type_of::{closure#2}::{closure#0}, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 8usize]>>
  30:     0x7fe3261b98f0 - <rustc_query_impl[69d5ff43c637cc62]::dynamic_query::type_of::{closure#2} as core[7d9be083f2db1]::ops::function::FnOnce<(rustc_middle[80e851c391dd4f58]::ty::context::TyCtxt, rustc_span[9d65595379dd9678]::def_id::DefId)>>::call_once
  31:     0x7fe3263559c8 - rustc_query_system[e8692c0e875adc1f]::query::plumbing::try_execute_query::<rustc_query_impl[69d5ff43c637cc62]::DynamicConfig<rustc_query_system[e8692c0e875adc1f]::query::caches::DefaultCache<rustc_span[9d65595379dd9678]::def_id::DefId, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[69d5ff43c637cc62]::plumbing::QueryCtxt, false>
  32:     0x7fe3260b1fd7 - rustc_query_impl[69d5ff43c637cc62]::get_query_non_incr::type_of::__rust_end_short_backtrace
  33:     0x7fe324e72f0d - rustc_middle[80e851c391dd4f58]::query::plumbing::query_get_at::<rustc_query_system[e8692c0e875adc1f]::query::caches::DefaultCache<rustc_span[9d65595379dd9678]::def_id::DefId, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 8usize]>>>
  34:     0x7fe324e792cf - rustc_hir_analysis[522899e829a14f4e]::check::check::check_opaque
  35:     0x7fe324e7dd91 - rustc_hir_analysis[522899e829a14f4e]::check::check::check_item_type
  36:     0x7fe324e859ea - rustc_hir_analysis[522899e829a14f4e]::check::check::check_mod_item_types
  37:     0x7fe3261fe97c - rustc_query_impl[69d5ff43c637cc62]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[69d5ff43c637cc62]::dynamic_query::check_mod_item_types::{closure#2}::{closure#0}, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 0usize]>>
  38:     0x7fe3261aadfc - <rustc_query_impl[69d5ff43c637cc62]::dynamic_query::check_mod_item_types::{closure#2} as core[7d9be083f2db1]::ops::function::FnOnce<(rustc_middle[80e851c391dd4f58]::ty::context::TyCtxt, rustc_span[9d65595379dd9678]::def_id::LocalDefId)>>::call_once
  39:     0x7fe32638e1e4 - rustc_query_system[e8692c0e875adc1f]::query::plumbing::try_execute_query::<rustc_query_impl[69d5ff43c637cc62]::DynamicConfig<rustc_query_system[e8692c0e875adc1f]::query::caches::VecCache<rustc_span[9d65595379dd9678]::def_id::LocalDefId, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[69d5ff43c637cc62]::plumbing::QueryCtxt, false>
  40:     0x7fe3260b65ac - rustc_query_impl[69d5ff43c637cc62]::get_query_non_incr::check_mod_item_types::__rust_end_short_backtrace
  41:     0x7fe324e63bd4 - <rustc_middle[80e851c391dd4f58]::hir::map::Map>::for_each_module::<rustc_hir_analysis[522899e829a14f4e]::check_crate::{closure#6}::{closure#0}>
  42:     0x7fe324fcdc35 - <rustc_session[b51cb1e13d514ec9]::session::Session>::time::<(), rustc_hir_analysis[522899e829a14f4e]::check_crate::{closure#6}>
  43:     0x7fe324fa65d1 - rustc_hir_analysis[522899e829a14f4e]::check_crate
  44:     0x7fe324798549 - rustc_interface[4b7c03a9a08a66c6]::passes::analysis
  45:     0x7fe326200b7a - rustc_query_impl[69d5ff43c637cc62]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[69d5ff43c637cc62]::dynamic_query::analysis::{closure#2}::{closure#0}, rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 1usize]>>
  46:     0x7fe3261b9ae8 - <rustc_query_impl[69d5ff43c637cc62]::dynamic_query::analysis::{closure#2} as core[7d9be083f2db1]::ops::function::FnOnce<(rustc_middle[80e851c391dd4f58]::ty::context::TyCtxt, ())>>::call_once
  47:     0x7fe326305732 - rustc_query_system[e8692c0e875adc1f]::query::plumbing::try_execute_query::<rustc_query_impl[69d5ff43c637cc62]::DynamicConfig<rustc_query_system[e8692c0e875adc1f]::query::caches::SingleCache<rustc_middle[80e851c391dd4f58]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[69d5ff43c637cc62]::plumbing::QueryCtxt, false>
  48:     0x7fe3260ec42d - rustc_query_impl[69d5ff43c637cc62]::get_query_non_incr::analysis::__rust_end_short_backtrace
  49:     0x7fe3246d917d - <rustc_middle[80e851c391dd4f58]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[6ed5859210d32a08]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>>
  50:     0x7fe324710b38 - <rustc_interface[4b7c03a9a08a66c6]::interface::Compiler>::enter::<rustc_driver_impl[6ed5859210d32a08]::run_compiler::{closure#1}::{closure#2}, core[7d9be083f2db1]::result::Result<core[7d9be083f2db1]::option::Option<rustc_interface[4b7c03a9a08a66c6]::queries::Linker>, rustc_span[9d65595379dd9678]::ErrorGuaranteed>>
  51:     0x7fe3246d7a40 - rustc_span[9d65595379dd9678]::set_source_map::<core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>, rustc_interface[4b7c03a9a08a66c6]::interface::run_compiler<core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>, rustc_driver_impl[6ed5859210d32a08]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  52:     0x7fe3246cf3f9 - <scoped_tls[10e468505437efad]::ScopedKey<rustc_span[9d65595379dd9678]::SessionGlobals>>::set::<rustc_interface[4b7c03a9a08a66c6]::interface::run_compiler<core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>, rustc_driver_impl[6ed5859210d32a08]::run_compiler::{closure#1}>::{closure#0}, core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>>
  53:     0x7fe3246e23f6 - std[6f35d623fa62ad51]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4b7c03a9a08a66c6]::util::run_in_thread_pool_with_globals<rustc_interface[4b7c03a9a08a66c6]::interface::run_compiler<core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>, rustc_driver_impl[6ed5859210d32a08]::run_compiler::{closure#1}>::{closure#0}, core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>>
  54:     0x7fe3247129e6 - std[6f35d623fa62ad51]::panicking::try::<core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>, core[7d9be083f2db1]::panic::unwind_safe::AssertUnwindSafe<<std[6f35d623fa62ad51]::thread::Builder>::spawn_unchecked_<rustc_interface[4b7c03a9a08a66c6]::util::run_in_thread_pool_with_globals<rustc_interface[4b7c03a9a08a66c6]::interface::run_compiler<core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>, rustc_driver_impl[6ed5859210d32a08]::run_compiler::{closure#1}>::{closure#0}, core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  55:     0x7fe3246cc5bd - <<std[6f35d623fa62ad51]::thread::Builder>::spawn_unchecked_<rustc_interface[4b7c03a9a08a66c6]::util::run_in_thread_pool_with_globals<rustc_interface[4b7c03a9a08a66c6]::interface::run_compiler<core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>, rustc_driver_impl[6ed5859210d32a08]::run_compiler::{closure#1}>::{closure#0}, core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7d9be083f2db1]::result::Result<(), rustc_span[9d65595379dd9678]::ErrorGuaranteed>>::{closure#1} as core[7d9be083f2db1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7fe323c4243e - std::sys::unix::thread::Thread::new::thread_start::hec235324356468b0
  57:     0x7fe3239ddb43 - <unknown>
  58:     0x7fe323a6fa00 - <unknown>
  59:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (a5060c69d 2023-05-18) running on x86_64-unknown-linux-gnu

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

note: rustc 1.71.0-nightly (a5060c69d 2023-05-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z drop-tracking-mir
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq/auxiliary" "-Ztrait-solver=next"
stdout: none
stderr: none

---- [ui] tests/ui/traits/new-solver/alias_eq_substs_eq_not_intercrate.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/alias_eq_substs_eq_not_intercrate.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/alias_eq_substs_eq_not_intercrate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/alias_eq_substs_eq_not_intercrate/auxiliary" "-Ztrait-solver=next"
stdout: none
stderr: none


failures:
    [ui] tests/ui/async-await/issue-70935-complex-spans.rs#drop_tracking_mir
