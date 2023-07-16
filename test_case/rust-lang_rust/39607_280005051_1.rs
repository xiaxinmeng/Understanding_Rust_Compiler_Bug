
error: internal compiler error: /checkout/src/librustc/ty/mod.rs:1889: No def'n found for DefId { krate: CrateNum(0), node: DefIndex(6) => catatonic/b26150dea8d5ed2d7e01758a53ca82f5::{{impl}}[0] } in tcx.impl_trait_refs

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:417
stack backtrace:
   1:     0x7f3b250bfc6c - std::sys::imp::backtrace::tracing::imp::write::hbb14611794d3841b
                        at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f3b250ce25e - std::panicking::default_hook::{{closure}}::h6ed906c7818ac88c
                        at /checkout/src/libstd/panicking.rs:351
   3:     0x7f3b250cde03 - std::panicking::default_hook::h23eeafbf7c1c05c3
                        at /checkout/src/libstd/panicking.rs:361
   4:     0x7f3b250ce6fb - std::panicking::rust_panic_with_hook::hd0067971b6d1240e
                        at /checkout/src/libstd/panicking.rs:545
   5:     0x7f3b1d712fb7 - std::panicking::begin_panic::hc300cc1c3c24c382
   6:     0x7f3b1d72854d - rustc_errors::Handler::bug::h86684df8c5f40d57
   7:     0x7f3b22220eba - rustc::session::opt_span_bug_fmt::{{closure}}::h3ad09996f9e31eb1
   8:     0x7f3b22220cc5 - rustc::session::opt_span_bug_fmt::h4b0bb8c3e1670f11
   9:     0x7f3b22220922 - rustc::session::bug_fmt::h2dbe8dc3b71c81a2
  10:     0x7f3b222ad4b9 - rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::impl_trait_ref::h900e5b606ada5d7b
  11:     0x7f3b229068fa - <rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o>::associated_path_def_to_ty::hf76c72c0ac489aae
  12:     0x7f3b22909b43 - <rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o>::ast_ty_to_ty::h9ff30c990d450a79
  13:     0x7f3b228fcaec - <rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o>::create_substs_for_ast_path::{{closure}}::hd03791d22bb2421e
  14:     0x7f3b2282d69d - rustc::ty::subst::<impl rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::fill_item::h237a29bcbae752fb
  15:     0x7f3b228fc645 - <rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o>::create_substs_for_ast_path::h82a4170662063c25
  16:     0x7f3b2290020f - <rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o>::create_substs_for_ast_trait_ref::h5a9d38af4735e350
  17:     0x7f3b228fff18 - <rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o>::ast_path_to_mono_trait_ref::h948d3ba9dce820dc
  18:     0x7f3b2290d440 - <rustc_typeck::collect::CollectItemTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h1ab3f3fbeb8eecde
  19:     0x7f3b2290c722 - rustc_typeck::collect::collect_item_types::h62789b6036667a93
  20:     0x7f3b22933fd9 - rustc_typeck::check_crate::h441df7c8151905be
  21:     0x7f3b2546a457 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h7d2a6da150c7eba7
  22:     0x7f3b25461b56 - rustc_driver::driver::phase_3_run_analysis_passes::hb4049a05b492319a
  23:     0x7f3b2544a480 - rustc_driver::driver::compile_input::hf3e3aa4173908b86
  24:     0x7f3b25494e04 - rustc_driver::run_compiler::h8f8d47f1d258a8a6
  25:     0x7f3b253a117b - std::panicking::try::do_call::h206b9daee04f4ea2
  26:     0x7f3b250d753a - __rust_maybe_catch_panic
                        at /checkout/src/libpanic_unwind/lib.rs:98
  27:     0x7f3b253c94f2 - <F as alloc::boxed::FnBox<A>>::call_box::ha024973179acfe4c
  28:     0x7f3b250cd0b4 - std::sys::imp::thread::Thread::new::thread_start::h2c901daa88f3cb32
                        at /checkout/src/liballoc/boxed.rs:640
                        at /checkout/src/libstd/sys_common/thread.rs:21
                        at /checkout/src/libstd/sys/unix/thread.rs:84
  29:     0x7f3b1cebd463 - start_thread
  30:     0x7f3b24d8e9de - __clone
  31:                0x0 - <unknown>
