plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: self.check_invariants()', /checkout/compiler/rustc_index/src/interval.rs:136:9
stack backtrace:
   0:     0x7f40b9fa1c32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7f40ba009848 - core::fmt::write::ha01458c252ca8e28
   2:     0x7f40b9f91fa1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7f40b9fa4f59 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7f40b9fa4bfa - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7f40baa891b1 - rustc_driver[112af0a3c96a71b0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f40b9fa57bf - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7f40b9fa55c9 - std::panicking::begin_panic_handler::{{closure}}::h72285187ceca975e
   8:     0x7f40b9fa21d4 - std::sys_common::backtrace::__rust_end_short_backtrace::ha2fc13ea7c6faa9f
   9:     0x7f40b9fa52f9 - rust_begin_unwind
  10:     0x7f40b9f59053 - core::panicking::panic_fmt::hd9df166e5b75fe7b
  11:     0x7f40b9f58f1d - core::panicking::panic::haec14f2162e74144
  12:     0x7f40bbb8d2ae - <rustc_index[9046b320305714a8]::interval::IntervalSet<rustc_borrowck[ac8661645517fed3]::region_infer::values::PointIndex>>::insert_range::<core[6d9550a4e960c99f]::ops::range::RangeInclusive<rustc_borrowck[ac8661645517fed3]::region_infer::values::PointIndex>>
  13:     0x7f40bbbc05da - <rustc_index[9046b320305714a8]::interval::SparseIntervalMatrix<rustc_middle[28e2656d34123ccd]::ty::sty::RegionVid, rustc_borrowck[ac8661645517fed3]::region_infer::values::PointIndex>>::insert
  14:     0x7f40bbd0e8a4 - <rustc_borrowck[ac8661645517fed3]::region_infer::values::LivenessValues<rustc_middle[28e2656d34123ccd]::ty::sty::RegionVid>>::add_element
  15:     0x7f40bbc3e3d5 - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::check_terminator
  16:     0x7f40bbc46be8 - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::typeck_mir
  17:     0x7f40bbc35f83 - rustc_borrowck[ac8661645517fed3]::type_check::type_check_internal::<rustc_data_structures[c7efe020d8efc160]::vec_map::VecMap<rustc_middle[28e2656d34123ccd]::ty::OpaqueTypeKey, (rustc_middle[28e2656d34123ccd]::ty::OpaqueHiddenType, rustc_hir[a6634ce7832da43e]::hir::OpaqueTyOrigin)>, rustc_borrowck[ac8661645517fed3]::type_check::type_check::{closure#0}>
  18:     0x7f40bbc26da0 - rustc_borrowck[ac8661645517fed3]::type_check::type_check
  19:     0x7f40bbb00767 - rustc_borrowck[ac8661645517fed3]::nll::compute_regions
  20:     0x7f40bbcbcbd6 - rustc_borrowck[ac8661645517fed3]::do_mir_borrowck
  21:     0x7f40bbb9b2b2 - <rustc_infer[11d1fb973888adab]::infer::InferCtxtBuilder>::enter::<rustc_middle[28e2656d34123ccd]::mir::query::BorrowCheckResult, rustc_borrowck[ac8661645517fed3]::mir_borrowck::{closure#0}>
  22:     0x7f40bbcb36cb - rustc_borrowck[ac8661645517fed3]::mir_borrowck
  23:     0x7f40bbc7f3ec - <rustc_borrowck[ac8661645517fed3]::provide::{closure#0} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[28e2656d34123ccd]::ty::context::TyCtxt, rustc_span[3917deda49487040]::def_id::LocalDefId)>>::call_once
  24:     0x7f40bc4d9f14 - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<rustc_span[3917deda49487040]::def_id::LocalDefId, &rustc_middle[28e2656d34123ccd]::mir::query::BorrowCheckResult>>
  25:     0x7f40bc5996a8 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::mir_borrowck, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  26:     0x7f40bc434294 - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::mir_borrowck
  27:     0x7f40bbc2dcb5 - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::prove_closure_bounds
  28:     0x7f40bbc40aea - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::check_rvalue
  29:     0x7f40bbc4625f - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::typeck_mir
  30:     0x7f40bbc35f83 - rustc_borrowck[ac8661645517fed3]::type_check::type_check_internal::<rustc_data_structures[c7efe020d8efc160]::vec_map::VecMap<rustc_middle[28e2656d34123ccd]::ty::OpaqueTypeKey, (rustc_middle[28e2656d34123ccd]::ty::OpaqueHiddenType, rustc_hir[a6634ce7832da43e]::hir::OpaqueTyOrigin)>, rustc_borrowck[ac8661645517fed3]::type_check::type_check::{closure#0}>
  31:     0x7f40bbc26da0 - rustc_borrowck[ac8661645517fed3]::type_check::type_check
  32:     0x7f40bbb00767 - rustc_borrowck[ac8661645517fed3]::nll::compute_regions
  33:     0x7f40bbcbcbd6 - rustc_borrowck[ac8661645517fed3]::do_mir_borrowck
  34:     0x7f40bbb9b2b2 - <rustc_infer[11d1fb973888adab]::infer::InferCtxtBuilder>::enter::<rustc_middle[28e2656d34123ccd]::mir::query::BorrowCheckResult, rustc_borrowck[ac8661645517fed3]::mir_borrowck::{closure#0}>
  35:     0x7f40bbcb36cb - rustc_borrowck[ac8661645517fed3]::mir_borrowck
  36:     0x7f40bbc7f3ec - <rustc_borrowck[ac8661645517fed3]::provide::{closure#0} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[28e2656d34123ccd]::ty::context::TyCtxt, rustc_span[3917deda49487040]::def_id::LocalDefId)>>::call_once
  37:     0x7f40bc4d9f14 - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<rustc_span[3917deda49487040]::def_id::LocalDefId, &rustc_middle[28e2656d34123ccd]::mir::query::BorrowCheckResult>>
  38:     0x7f40bc5996a8 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::mir_borrowck, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  39:     0x7f40bc434294 - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::mir_borrowck
  40:     0x7f40bb3986e2 - rustc_typeck[fcda030e8ecb822c]::collect::type_of::type_of
  41:     0x7f40bc4f024d - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<rustc_span[3917deda49487040]::def_id::DefId, rustc_middle[28e2656d34123ccd]::ty::Ty>>
  42:     0x7f40bc5e8861 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::type_of, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  43:     0x7f40bc415b49 - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::type_of
  44:     0x7f40baecb829 - <rustc_privacy[21a49a04a7759f10]::EmbargoVisitor as rustc_hir[a6634ce7832da43e]::intravisit::Visitor>::visit_item
  45:     0x7f40baeebc31 - rustc_hir[a6634ce7832da43e]::intravisit::walk_ty::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  46:     0x7f40baeeb648 - rustc_hir[a6634ce7832da43e]::intravisit::walk_fn::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  47:     0x7f40baee77b5 - rustc_hir[a6634ce7832da43e]::intravisit::walk_impl_item::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  48:     0x7f40baeefbde - rustc_hir[a6634ce7832da43e]::intravisit::walk_item::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  49:     0x7f40baecc135 - <rustc_privacy[21a49a04a7759f10]::EmbargoVisitor as rustc_hir[a6634ce7832da43e]::intravisit::Visitor>::visit_item
  50:     0x7f40baeef7be - rustc_hir[a6634ce7832da43e]::intravisit::walk_item::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  51:     0x7f40baecc135 - <rustc_privacy[21a49a04a7759f10]::EmbargoVisitor as rustc_hir[a6634ce7832da43e]::intravisit::Visitor>::visit_item
  52:     0x7f40baeef7be - rustc_hir[a6634ce7832da43e]::intravisit::walk_item::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  53:     0x7f40baecc135 - <rustc_privacy[21a49a04a7759f10]::EmbargoVisitor as rustc_hir[a6634ce7832da43e]::intravisit::Visitor>::visit_item
  54:     0x7f40baeece5a - rustc_hir[a6634ce7832da43e]::intravisit::walk_mod::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  55:     0x7f40baed1a7b - rustc_privacy[21a49a04a7759f10]::privacy_access_levels
  56:     0x7f40bc51b80c - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<(), &rustc_middle[28e2656d34123ccd]::middle::privacy::AccessLevels>>
  57:     0x7f40bc5b9335 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::privacy_access_levels, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  58:     0x7f40bc43ad2e - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::privacy_access_levels
  59:     0x7f40bb923adc - rustc_passes[ec05b53c5a31811e]::stability::check_unused_or_stable_features
  60:     0x7f40babbd6ce - <rustc_session[239c49e2c3a68631]::session::Session>::time::<(), rustc_interface[af1a4dc4697f9b6c]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  61:     0x7f40babbe7be - <rustc_session[239c49e2c3a68631]::session::Session>::time::<(), rustc_interface[af1a4dc4697f9b6c]::passes::analysis::{closure#0}>
  62:     0x7f40babaaf86 - rustc_interface[af1a4dc4697f9b6c]::passes::analysis
  63:     0x7f40bc513bbc - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<(), core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>>
  64:     0x7f40bc5e8982 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::analysis, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  65:     0x7f40bc4160ae - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::analysis
  66:     0x7f40baafa41a - <rustc_interface[af1a4dc4697f9b6c]::passes::QueryContext>::enter::<rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  67:     0x7f40baaa564d - <rustc_interface[af1a4dc4697f9b6c]::interface::Compiler>::enter::<rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[af1a4dc4697f9b6c]::queries::Linker>, rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  68:     0x7f40bab06c86 - rustc_span[3917deda49487040]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_interface[af1a4dc4697f9b6c]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#1}>
  69:     0x7f40baaa68cd - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[3917deda49487040]::SessionGlobals>>::set::<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  70:     0x7f40bab0ba0f - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[af1a4dc4697f9b6c]::util::run_in_thread_pool_with_globals<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  71:     0x7f40baafe289 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[af1a4dc4697f9b6c]::util::run_in_thread_pool_with_globals<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  72:     0x7f40b9fb2163 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  73:     0x7f40b4502609 - start_thread
  74:     0x7f40b9e15133 - clone
  75:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (883442ca6 2022-06-08) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 384:2>::wrap_mut_2::{closure#0}`
#1 [mir_borrowck] borrow-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 384:2>::wrap_mut_2`
#2 [type_of] computing type of `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 384:2>::wrap_mut_2::{opaque#1}`
#3 [privacy_access_levels] privacy access levels
#4 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:3660 ~ core[fe06]::ops::try_trait::{impl#0}::wrap_mut_2::{opaque#1}), substs: [T, A, B, impl FnMut(A, B) -> T] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: library/core/src/ops/try_trait.rs:382:9: 382:47 (#0), ty: [closure@library/core/src/ops/try_trait.rs:382:9: 382:47] }, origin: FnReturn(DefId(0:3656 ~ core[fe06]::ops::try_trait::{impl#0}::wrap_mut_2)) })])
  = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1368:13
stack backtrace:
stack backtrace:
   0:     0x7f40b9fa1c32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7f40ba009848 - core::fmt::write::ha01458c252ca8e28
   2:     0x7f40b9f91fa1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7f40b9fa4f59 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7f40b9fa4bfa - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7f40baa891b1 - rustc_driver[112af0a3c96a71b0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f40b9fa57bf - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7f40bd662cf3 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>::{closure#0}
   8:     0x7f40bd6616b6 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[9d4038935dd1aec6]::ExplicitBug>::{closure#0}, !>
   9:     0x7f40baa48126 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>
  10:     0x7f40bd670606 - std[836a811975e52724]::panic::panic_any::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>
  11:     0x7f40bd6752a7 - <rustc_errors[9d4038935dd1aec6]::HandlerInner as core[6d9550a4e960c99f]::ops::drop::Drop>::drop
  12:     0x7f40baa9b842 - core[6d9550a4e960c99f]::ptr::drop_in_place::<rustc_session[239c49e2c3a68631]::parse::ParseSess>
  13:     0x7f40baaa0745 - <alloc[f55ce12b9f25f528]::rc::Rc<rustc_session[239c49e2c3a68631]::session::Session> as core[6d9550a4e960c99f]::ops::drop::Drop>::drop
  14:     0x7f40bab0acdc - core[6d9550a4e960c99f]::ptr::drop_in_place::<rustc_interface[af1a4dc4697f9b6c]::interface::Compiler>
  15:     0x7f40bab07334 - rustc_span[3917deda49487040]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_interface[af1a4dc4697f9b6c]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f40baaa68cd - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[3917deda49487040]::SessionGlobals>>::set::<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  17:     0x7f40bab0ba0f - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[af1a4dc4697f9b6c]::util::run_in_thread_pool_with_globals<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  18:     0x7f40baafe289 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[af1a4dc4697f9b6c]::util::run_in_thread_pool_with_globals<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f40b9fb2163 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  20:     0x7f40b4502609 - start_thread
  21:     0x7f40b9e15133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (883442ca6 2022-06-08) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=a2a7040fb9f918eb -C extra-filename=-a2a7040fb9f918eb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
