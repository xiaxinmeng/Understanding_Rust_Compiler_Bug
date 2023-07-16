
stack backtrace:
   1:     0x7f191863c40c - std::sys::imp::backtrace::tracing::imp::write::hf33ae72d0baa11ed
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f191864a9ae - std::panicking::default_hook::{{closure}}::h59672b733cc6a455
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:351
   3:     0x7f191864a553 - std::panicking::default_hook::h1670459d2f3f8843
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:361
   4:     0x7f191864ae4b - std::panicking::rust_panic_with_hook::hcf0ddb069e7beee7
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:555
   5:     0x7f1911958e47 - std::panicking::begin_panic::h2132875502929f32
   6:     0x7f191196e50d - rustc_errors::Handler::bug::h62761acaae866c21
   7:     0x7f191576d1aa - rustc::session::opt_span_bug_fmt::{{closure}}::hb8dfe55fdd29ba42
   8:     0x7f191576cc65 - rustc::session::opt_span_bug_fmt::h0cd2e09801797662
   9:     0x7f191576c8c2 - rustc::session::bug_fmt::hd23041cebc71c535
  10:     0x7f19157f8889 - rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::impl_trait_ref::h5f4b9b20d319713c
  11:     0x7f191707940a - <rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o>::associated_path_def_to_ty::he24118f4fd616682
  12:     0x7f191707c61c - <rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o>::ast_ty_to_ty::h1791398449212537
  13:     0x7f191708d61c - rustc_typeck::collect::ty_generic_predicates::h23a480c80275b04d
  14:     0x7f191707fd36 - <rustc_typeck::collect::CollectItemTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h968a75e928c186a0
  15:     0x7f191707f1b2 - rustc_typeck::collect::collect_item_types::h2a32ae570b87c3b5
  16:     0x7f19170a6cf9 - rustc_typeck::check_crate::h8c159f70d1009b33
  17:     0x7f19189e8cd7 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h040eea107b0d299b
  18:     0x7f19189e56c6 - rustc_driver::driver::phase_3_run_analysis_passes::haec055f3322a38aa
  19:     0x7f19189c9150 - rustc_driver::driver::compile_input::hd9f060ee16a643fb
  20:     0x7f1918a13844 - rustc_driver::run_compiler::h762802568c0e140e
  21:     0x7f191891fedb - std::panicking::try::do_call::h935e2f773deaf841
  22:     0x7f1918653c8a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  23:     0x7f1918948112 - <F as alloc::boxed::FnBox<A>>::call_box::he43811d1f6894655
  24:     0x7f1918649804 - std::sys::imp::thread::Thread::new::thread_start::he668872ac11287ba
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/liballoc/boxed.rs:624
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  25:     0x7f19111036b9 - start_thread
  26:     0x7f19182ff82c - clone
  27:                0x0 - <unknown>
