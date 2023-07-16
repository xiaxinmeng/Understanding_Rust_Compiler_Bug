plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'error when insert 9..=9 to IntervalSet { map: [(9, 9)], domain: 30, _data: PhantomData }', /checkout/compiler/rustc_index/src/interval.rs:136:9
stack backtrace:
   0:     0x7fd9aee54c32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7fd9aeebc848 - core::fmt::write::ha01458c252ca8e28
   2:     0x7fd9aee44fa1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7fd9aee57f59 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7fd9aee57bfa - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7fd9af93c1b1 - rustc_driver[112af0a3c96a71b0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd9aee587bf - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7fd9aee58607 - std::panicking::begin_panic_handler::{{closure}}::h72285187ceca975e
   8:     0x7fd9aee551d4 - std::sys_common::backtrace::__rust_end_short_backtrace::ha2fc13ea7c6faa9f
   9:     0x7fd9aee582f9 - rust_begin_unwind
  10:     0x7fd9aee0c053 - core::panicking::panic_fmt::hd9df166e5b75fe7b
  11:     0x7fd9b0a09890 - <rustc_index[9046b320305714a8]::interval::IntervalSet<rustc_borrowck[ac8661645517fed3]::region_infer::values::PointIndex>>::insert_range::<core[6d9550a4e960c99f]::ops::range::RangeInclusive<rustc_borrowck[ac8661645517fed3]::region_infer::values::PointIndex>>
  12:     0x7fd9b0a188bf - <rustc_index[9046b320305714a8]::interval::SparseIntervalMatrix<rustc_middle[28e2656d34123ccd]::ty::sty::RegionVid, rustc_borrowck[ac8661645517fed3]::region_infer::values::PointIndex>>::insert
  13:     0x7fd9b0bc1f14 - <rustc_borrowck[ac8661645517fed3]::region_infer::values::LivenessValues<rustc_middle[28e2656d34123ccd]::ty::sty::RegionVid>>::add_element
  14:     0x7fd9b0af1a45 - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::check_terminator
  15:     0x7fd9b0afa258 - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::typeck_mir
  16:     0x7fd9b0ae95f3 - rustc_borrowck[ac8661645517fed3]::type_check::type_check_internal::<rustc_data_structures[c7efe020d8efc160]::vec_map::VecMap<rustc_middle[28e2656d34123ccd]::ty::OpaqueTypeKey, (rustc_middle[28e2656d34123ccd]::ty::OpaqueHiddenType, rustc_hir[a6634ce7832da43e]::hir::OpaqueTyOrigin)>, rustc_borrowck[ac8661645517fed3]::type_check::type_check::{closure#0}>
  17:     0x7fd9b0ada410 - rustc_borrowck[ac8661645517fed3]::type_check::type_check
  18:     0x7fd9b09b3767 - rustc_borrowck[ac8661645517fed3]::nll::compute_regions
  19:     0x7fd9b0b70246 - rustc_borrowck[ac8661645517fed3]::do_mir_borrowck
  20:     0x7fd9b0a4e5a2 - <rustc_infer[11d1fb973888adab]::infer::InferCtxtBuilder>::enter::<rustc_middle[28e2656d34123ccd]::mir::query::BorrowCheckResult, rustc_borrowck[ac8661645517fed3]::mir_borrowck::{closure#0}>
  21:     0x7fd9b0b66d3b - rustc_borrowck[ac8661645517fed3]::mir_borrowck
  22:     0x7fd9b0b32a5c - <rustc_borrowck[ac8661645517fed3]::provide::{closure#0} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[28e2656d34123ccd]::ty::context::TyCtxt, rustc_span[3917deda49487040]::def_id::LocalDefId)>>::call_once
  23:     0x7fd9b138d584 - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<rustc_span[3917deda49487040]::def_id::LocalDefId, &rustc_middle[28e2656d34123ccd]::mir::query::BorrowCheckResult>>
  24:     0x7fd9b144cd18 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::mir_borrowck, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  25:     0x7fd9b12e7904 - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::mir_borrowck
  26:     0x7fd9b0ae1325 - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::prove_closure_bounds
  27:     0x7fd9b0af415a - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::check_rvalue
  28:     0x7fd9b0af98cf - <rustc_borrowck[ac8661645517fed3]::type_check::TypeChecker>::typeck_mir
  29:     0x7fd9b0ae95f3 - rustc_borrowck[ac8661645517fed3]::type_check::type_check_internal::<rustc_data_structures[c7efe020d8efc160]::vec_map::VecMap<rustc_middle[28e2656d34123ccd]::ty::OpaqueTypeKey, (rustc_middle[28e2656d34123ccd]::ty::OpaqueHiddenType, rustc_hir[a6634ce7832da43e]::hir::OpaqueTyOrigin)>, rustc_borrowck[ac8661645517fed3]::type_check::type_check::{closure#0}>
  30:     0x7fd9b0ada410 - rustc_borrowck[ac8661645517fed3]::type_check::type_check
  31:     0x7fd9b09b3767 - rustc_borrowck[ac8661645517fed3]::nll::compute_regions
  32:     0x7fd9b0b70246 - rustc_borrowck[ac8661645517fed3]::do_mir_borrowck
  33:     0x7fd9b0a4e5a2 - <rustc_infer[11d1fb973888adab]::infer::InferCtxtBuilder>::enter::<rustc_middle[28e2656d34123ccd]::mir::query::BorrowCheckResult, rustc_borrowck[ac8661645517fed3]::mir_borrowck::{closure#0}>
  34:     0x7fd9b0b66d3b - rustc_borrowck[ac8661645517fed3]::mir_borrowck
  35:     0x7fd9b0b32a5c - <rustc_borrowck[ac8661645517fed3]::provide::{closure#0} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[28e2656d34123ccd]::ty::context::TyCtxt, rustc_span[3917deda49487040]::def_id::LocalDefId)>>::call_once
  36:     0x7fd9b138d584 - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<rustc_span[3917deda49487040]::def_id::LocalDefId, &rustc_middle[28e2656d34123ccd]::mir::query::BorrowCheckResult>>
  37:     0x7fd9b144cd18 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::mir_borrowck, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  38:     0x7fd9b12e7904 - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::mir_borrowck
  39:     0x7fd9b024b6e2 - rustc_typeck[fcda030e8ecb822c]::collect::type_of::type_of
  40:     0x7fd9b13a38bd - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<rustc_span[3917deda49487040]::def_id::DefId, rustc_middle[28e2656d34123ccd]::ty::Ty>>
  41:     0x7fd9b149bed1 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::type_of, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  42:     0x7fd9b12c91b9 - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::type_of
  43:     0x7fd9afd7e829 - <rustc_privacy[21a49a04a7759f10]::EmbargoVisitor as rustc_hir[a6634ce7832da43e]::intravisit::Visitor>::visit_item
  44:     0x7fd9afd9ec31 - rustc_hir[a6634ce7832da43e]::intravisit::walk_ty::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  45:     0x7fd9afd9e648 - rustc_hir[a6634ce7832da43e]::intravisit::walk_fn::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  46:     0x7fd9afd9a7b5 - rustc_hir[a6634ce7832da43e]::intravisit::walk_impl_item::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  47:     0x7fd9afda2bde - rustc_hir[a6634ce7832da43e]::intravisit::walk_item::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  48:     0x7fd9afd7f135 - <rustc_privacy[21a49a04a7759f10]::EmbargoVisitor as rustc_hir[a6634ce7832da43e]::intravisit::Visitor>::visit_item
  49:     0x7fd9afda27be - rustc_hir[a6634ce7832da43e]::intravisit::walk_item::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  50:     0x7fd9afd7f135 - <rustc_privacy[21a49a04a7759f10]::EmbargoVisitor as rustc_hir[a6634ce7832da43e]::intravisit::Visitor>::visit_item
  51:     0x7fd9afda27be - rustc_hir[a6634ce7832da43e]::intravisit::walk_item::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  52:     0x7fd9afd7f135 - <rustc_privacy[21a49a04a7759f10]::EmbargoVisitor as rustc_hir[a6634ce7832da43e]::intravisit::Visitor>::visit_item
  53:     0x7fd9afd9fe5a - rustc_hir[a6634ce7832da43e]::intravisit::walk_mod::<rustc_privacy[21a49a04a7759f10]::EmbargoVisitor>
  54:     0x7fd9afd84a7b - rustc_privacy[21a49a04a7759f10]::privacy_access_levels
  55:     0x7fd9b13cee7c - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<(), &rustc_middle[28e2656d34123ccd]::middle::privacy::AccessLevels>>
  56:     0x7fd9b146c9a5 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::privacy_access_levels, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  57:     0x7fd9b12ee39e - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::privacy_access_levels
  58:     0x7fd9b07d6adc - rustc_passes[ec05b53c5a31811e]::stability::check_unused_or_stable_features
  59:     0x7fd9afa706ce - <rustc_session[239c49e2c3a68631]::session::Session>::time::<(), rustc_interface[af1a4dc4697f9b6c]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  60:     0x7fd9afa717be - <rustc_session[239c49e2c3a68631]::session::Session>::time::<(), rustc_interface[af1a4dc4697f9b6c]::passes::analysis::{closure#0}>
  61:     0x7fd9afa5df86 - rustc_interface[af1a4dc4697f9b6c]::passes::analysis
  62:     0x7fd9b13c722c - rustc_query_system[4edf926b197ba46b]::query::plumbing::try_execute_query::<rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt, rustc_query_system[4edf926b197ba46b]::query::caches::DefaultCache<(), core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>>
  63:     0x7fd9b149bff2 - rustc_query_system[4edf926b197ba46b]::query::plumbing::get_query::<rustc_query_impl[787100ad65211a60]::queries::analysis, rustc_query_impl[787100ad65211a60]::plumbing::QueryCtxt>
  64:     0x7fd9b12c971e - <rustc_query_impl[787100ad65211a60]::Queries as rustc_middle[28e2656d34123ccd]::ty::query::QueryEngine>::analysis
  65:     0x7fd9af9ad41a - <rustc_interface[af1a4dc4697f9b6c]::passes::QueryContext>::enter::<rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  66:     0x7fd9af95864d - <rustc_interface[af1a4dc4697f9b6c]::interface::Compiler>::enter::<rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[af1a4dc4697f9b6c]::queries::Linker>, rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  67:     0x7fd9af9b9c86 - rustc_span[3917deda49487040]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_interface[af1a4dc4697f9b6c]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#1}>
  68:     0x7fd9af9598cd - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[3917deda49487040]::SessionGlobals>>::set::<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  69:     0x7fd9af9bea0f - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[af1a4dc4697f9b6c]::util::run_in_thread_pool_with_globals<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  70:     0x7fd9af9b1289 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[af1a4dc4697f9b6c]::util::run_in_thread_pool_with_globals<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  71:     0x7fd9aee65163 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  72:     0x7fd9a93b5609 - start_thread
  73:     0x7fd9aecc8133 - clone
  74:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (758c18ce3 2022-06-08) running on x86_64-unknown-linux-gnu

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

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:3660 ~ core[47bd]::ops::try_trait::{impl#0}::wrap_mut_2::{opaque#1}), substs: [T, A, B, impl FnMut(A, B) -> T] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: library/core/src/ops/try_trait.rs:382:9: 382:47 (#0), ty: [closure@library/core/src/ops/try_trait.rs:382:9: 382:47] }, origin: FnReturn(DefId(0:3656 ~ core[47bd]::ops::try_trait::{impl#0}::wrap_mut_2)) })])
  = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1368:13
stack backtrace:
stack backtrace:
   0:     0x7fd9aee54c32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7fd9aeebc848 - core::fmt::write::ha01458c252ca8e28
   2:     0x7fd9aee44fa1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7fd9aee57f59 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7fd9aee57bfa - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7fd9af93c1b1 - rustc_driver[112af0a3c96a71b0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd9aee587bf - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7fd9b2516363 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>::{closure#0}
   8:     0x7fd9b2514d26 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[9d4038935dd1aec6]::ExplicitBug>::{closure#0}, !>
   9:     0x7fd9af8fb126 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>
  10:     0x7fd9b2523c76 - std[836a811975e52724]::panic::panic_any::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>
  11:     0x7fd9b2528917 - <rustc_errors[9d4038935dd1aec6]::HandlerInner as core[6d9550a4e960c99f]::ops::drop::Drop>::drop
  12:     0x7fd9af94e842 - core[6d9550a4e960c99f]::ptr::drop_in_place::<rustc_session[239c49e2c3a68631]::parse::ParseSess>
  13:     0x7fd9af953745 - <alloc[f55ce12b9f25f528]::rc::Rc<rustc_session[239c49e2c3a68631]::session::Session> as core[6d9550a4e960c99f]::ops::drop::Drop>::drop
  14:     0x7fd9af9bdcdc - core[6d9550a4e960c99f]::ptr::drop_in_place::<rustc_interface[af1a4dc4697f9b6c]::interface::Compiler>
  15:     0x7fd9af9ba334 - rustc_span[3917deda49487040]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_interface[af1a4dc4697f9b6c]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fd9af9598cd - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[3917deda49487040]::SessionGlobals>>::set::<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  17:     0x7fd9af9bea0f - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[af1a4dc4697f9b6c]::util::run_in_thread_pool_with_globals<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  18:     0x7fd9af9b1289 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[af1a4dc4697f9b6c]::util::run_in_thread_pool_with_globals<rustc_interface[af1a4dc4697f9b6c]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[112af0a3c96a71b0]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7fd9aee65163 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  20:     0x7fd9a93b5609 - start_thread
  21:     0x7fd9aecc8133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (758c18ce3 2022-06-08) running on x86_64-unknown-linux-gnu

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
