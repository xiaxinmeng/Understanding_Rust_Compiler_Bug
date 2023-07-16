
error: internal compiler error: ../src/librustc_metadata/decoder.rs:490: entry: id not found: DefIndex(1) in crate "serde_derive" with number 41

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:424
stack backtrace:
   1:        0x111d774ea - std::sys::imp::backtrace::tracing::imp::write::h944c02ac40aee2d7
   2:        0x111d8477f - std::panicking::default_hook::{{closure}}::h6875a2976258b020
   3:        0x111d8432d - std::panicking::default_hook::h88ffbc5922643264
   4:        0x111d84c46 - std::panicking::rust_panic_with_hook::ha5aed1dfc0e220e3
   5:        0x10f508f2a - std::panicking::begin_panic::h264cdc75d51b518b
   6:        0x10f519d34 - rustc_errors::Handler::bug::h620f7270292f0095
   7:        0x10e87fecc - rustc::session::opt_span_bug_fmt::{{closure}}::h4a9b70c3df8b4b3a
   8:        0x10e87fcd9 - rustc::session::opt_span_bug_fmt::h7d83586c6e2c7ae6
   9:        0x10e87f85a - rustc::session::bug_fmt::he2d2f00a4afa9d1e
  10:        0x10ddcab73 - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry::h5e3d8114267e9116
  11:        0x10ddd34ab - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore<'tcx> for rustc_metadata::cstore::CStore>::visibility::h2949b511f18961ee
  12:        0x10dddbf66 - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore<'tcx> for rustc_metadata::cstore::CStore>::visible_parent_map::hab53861eb74bce8d
  13:        0x10e8c91c4 - rustc::ty::item_path::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::push_item_path::haa7f7a9afda5f72a
  14:        0x10e9152d5 - rustc::util::ppaux::parameterized::h7bcfb1fd3d8a947a
  15:        0x111dc8cb5 - core::fmt::write::h01739b8f12f355f9
  16:        0x10e8b1c4c - rustc::traits::specialize::specialization_graph::Graph::insert::h62250bf0632d4218
  17:        0x10e8e5d12 - rustc::ty::trait_def::TraitDef::add_impl_for_specialization::h39896639aba068c7
  18:        0x10dbc6b67 - <rustc_typeck::coherence::overlap::OverlapChecker<'cx, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_item::h5ba64bfdace3da91
  19:        0x10dbcfaab - rustc_typeck::coherence::check_coherence::h7b433ad2f4520d1a
  20:        0x10dbd66a1 - rustc_typeck::check_crate::h4045752b69a796e8
  21:        0x10d67e1ac - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h97a3a12d948df547
  22:        0x10d67b37d - rustc_driver::driver::phase_3_run_analysis_passes::hb0ad9de18d423e67
  23:        0x10d662bc5 - rustc_driver::driver::compile_input::h8e119234b60571d5
  24:        0x10d6a8229 - rustc_driver::run_compiler::hbdfc4f84e2e0f4b9
  25:        0x10d5c7988 - std::panicking::try::do_call::hf679f17bf3b43b0b
  26:        0x111d871fa - __rust_maybe_catch_panic
  27:        0x10d5ebaaf - <F as alloc::boxed::FnBox<A>>::call_box::h506fb5d7b8891cd4
  28:        0x111d838d4 - std::sys::imp::thread::Thread::new::thread_start::h8084b1107992ae5b
  29:     0x7fff859de99c - _pthread_body
  30:     0x7fff859de919 - _pthread_start
