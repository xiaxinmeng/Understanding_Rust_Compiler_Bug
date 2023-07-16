
thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:417
stack backtrace:
   1:     0x7fb011745f29 - std::sys::imp::backtrace::tracing::imp::write::hbb14611794d3841b
                        at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7fb0117545c9 - std::panicking::default_hook::{{closure}}::h6ed906c7818ac88c
                        at /checkout/src/libstd/panicking.rs:351
   3:     0x7fb011754166 - std::panicking::default_hook::h23eeafbf7c1c05c3
                        at /checkout/src/libstd/panicking.rs:361
   4:     0x7fb011754a2b - std::panicking::rust_panic_with_hook::hd0067971b6d1240e
                        at /checkout/src/libstd/panicking.rs:545
   5:     0x7fb009d54d27 - std::panicking::begin_panic::hc300cc1c3c24c382
   6:     0x7fb009d6a27d - rustc_errors::Handler::bug::h86684df8c5f40d57
   7:     0x7fb00e86c45a - rustc::session::opt_span_bug_fmt::{{closure}}::h3ad09996f9e31eb1
   8:     0x7fb00e86c267 - rustc::session::opt_span_bug_fmt::h4b0bb8c3e1670f11
   9:     0x7fb00e86bec2 - rustc::session::bug_fmt::h2dbe8dc3b71c81a2
  10:     0x7fb00eeaeaaa - rustc_typeck::check::dropck::revise_self_ty::{{closure}}::h8443f32a5645dfbf
  11:     0x7fb00ee7642a - rustc::ty::subst::<impl rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::fill_item::he6f2b1605174404e
  12:     0x7fb00eeae031 - rustc_typeck::check::dropck::iterate_over_potentially_unsafe_regions_in_type::h8efc493a06003174
  13:     0x7fb00eead4ec - rustc_typeck::check::dropck::check_safety_of_destructor_if_necessary::hfedb806f05d4f478
  14:     0x7fb00eec404a - rustc_typeck::check::regionck::RegionCtxt::check_safety_of_rvalue_destructor_if_necessary::h173d44d06b412e45
  15:     0x7fb00eec294d - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::h230f54fb51bf130b
  16:     0x7fb00eec1d27 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_local::h09384b04d000dbf1
  17:     0x7fb00ee810c5 - rustc::hir::intravisit::walk_expr::h8b16d458f89904f3
  18:     0x7fb00eec35fc - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::h230f54fb51bf130b
  19:     0x7fb00eec10da - rustc_typeck::check::regionck::RegionCtxt::visit_fn_body::hcd276f56b41f29b7
  20:     0x7fb00eec0847 - rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_fn::h6101a1bf9ef6ab14
  21:     0x7fb00ef11b92 - rustc_typeck::check::check_bare_fn::hb24e70a67e127683
  22:     0x7fb00ef0eec5 - rustc_typeck::check::check_item_bodies::h0cab204d56268561
  23:     0x7fb00ef7b627 - rustc_typeck::check_crate::h441df7c8151905be
  24:     0x7fb011af24e4 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h7d2a6da150c7eba7
  25:     0x7fb011a60a14 - rustc::ty::context::TyCtxt::create_and_enter::h825fbad7d24d1480
  26:     0x7fb011ad5e13 - rustc_driver::driver::compile_input::hf3e3aa4173908b86
  27:     0x7fb011b1d3dd - rustc_driver::run_compiler::h8f8d47f1d258a8a6
  28:     0x7fb011a277db - std::panicking::try::do_call::h206b9daee04f4ea2
  29:     0x7fb01175d8da - __rust_maybe_catch_panic
                        at /checkout/src/libpanic_unwind/lib.rs:98
  30:     0x7fb011a4fb92 - <F as alloc::boxed::FnBox<A>>::call_box::h5d196fbb3229f499
  31:     0x7fb011753404 - std::sys::imp::thread::Thread::new::thread_start::h2c901daa88f3cb32
                        at /checkout/src/liballoc/boxed.rs:648
                        at /checkout/src/libstd/sys_common/thread.rs:21
                        at /checkout/src/libstd/sys/unix/thread.rs:84
  32:     0x7fb0094ff6b9 - start_thread
  33:     0x7fb01140682c - clone
  34:                0x0 - <unknown>
