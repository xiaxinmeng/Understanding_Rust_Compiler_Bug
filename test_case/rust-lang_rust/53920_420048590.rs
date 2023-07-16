
error: internal compiler error: librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(318/0:334 ~ hydrabadger[6497]::peer[0]::{{impl}}[0]::hdb[0])

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:578:9
stack backtrace:
   0:     0x7fd699c0ccbe - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hc38b730e4fac0255
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7fd699be88a6 - std::sys_common::backtrace::print::h14587871a85be1d1
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7fd699be229d - std::panicking::default_hook::{{closure}}::h3598f21b0ef93330
                               at libstd/panicking.rs:211
   3:     0x7fd699be2010 - std::panicking::default_hook::h8168a325cc2a7e4b
                               at libstd/panicking.rs:227
   4:     0x7fd69633192d - rustc::util::common::panic_hook::h377e19a02109a0d9
   5:     0x7fd699be2a23 - std::panicking::rust_panic_with_hook::hd4f331d1faf6d125
                               at libstd/panicking.rs:479
   6:     0x7fd694fc5d1e - std::panicking::begin_panic::hed18f2dfd78a2f6d
   7:     0x7fd694fe0451 - rustc_errors::Handler::bug::h288e744370c67e9f
   8:     0x7fd695e5eb4c - rustc::session::opt_span_bug_fmt::{{closure}}::h4fcb79e90c410083
   9:     0x7fd695f197b9 - rustc::ty::context::tls::with_opt::{{closure}}::h2f8fc392cd1c03eb
  10:     0x7fd695e5f94f - rustc::ty::context::tls::with_context_opt::hbae69a80d2573e4d
  11:     0x7fd695f19246 - rustc::ty::context::tls::with_opt::hfa24305e0dd1e123
  12:     0x7fd695dbb534 - rustc::session::opt_span_bug_fmt::hfb7915fc33f379ad
  13:     0x7fd695dbb4a6 - rustc::session::bug_fmt::h93f77b183d77c02a
  14:     0x7fd696c761d9 - rustc_mir::monomorphize::collector::should_monomorphize_locally::h64c89d199ca23581
  15:     0x7fd696c75e54 - rustc_mir::monomorphize::collector::visit_instance_use::hbb874cd8dad31ce0
  16:     0x7fd696c75e22 - rustc_mir::monomorphize::collector::visit_fn_use::h9fe7bd383d9f49ad
  17:     0x7fd696c75982 - <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind::h361ce6d615c46199
  18:     0x7fd696c74386 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  19:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  20:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  21:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  22:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  23:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  24:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  25:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  26:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  27:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  28:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  29:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  30:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  31:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  32:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  33:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  34:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  35:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  36:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  37:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  38:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  39:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  40:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  41:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  42:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  43:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  44:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  45:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  46:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  47:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  48:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  49:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  50:     0x7fd696c749b9 - rustc_mir::monomorphize::collector::collect_items_rec::h15cbb74d560a196f
  51:     0x7fd696f306f0 - rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}::h42377768293a947b
  52:     0x7fd696f086d4 - rustc::util::common::time::hb3164c62cd6cc215
  53:     0x7fd696c73030 - rustc_mir::monomorphize::collector::collect_crate_mono_items::hc77a074be340e459
  54:     0x7fd68f6f6c86 - rustc::util::common::time::hf1e4990b4a639374
  55:     0x7fd68f7b2274 - rustc_codegen_llvm::base::collect_and_partition_mono_items::hd1a070a2376bb9c5
  56:     0x7fd695f1b348 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute::hea75f44dc97797d0
  57:     0x7fd695d1274e - rustc::dep_graph::graph::DepGraph::with_task_impl::h835f0a4d2b18bc07
  58:     0x7fd695ec0f5b - rustc::ty::context::tls::with_related_context::h3c83330713b13e99
  59:     0x7fd69601dae7 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job::h5c8689f533db834c
  60:     0x7fd696116298 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query::hc459420e2d1f7906
  61:     0x7fd68f7b7350 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate::h16304cb944eb3a10
  62:     0x7fd699f92971 - rustc::util::common::time::h27952c3dcecf4d9d
  63:     0x7fd699f8967c - rustc_driver::driver::phase_4_codegen::h11f31d1ce05dd8cc
  64:     0x7fd69a029f98 - rustc_driver::driver::compile_input::{{closure}}::hbcfd05b05ee10962
  65:     0x7fd69a024ad9 - rustc::ty::context::tls::enter_context::he6792d96b1a8593b
  66:     0x7fd699fd8c5a - <std::thread::local::LocalKey<T>>::with::hff342551e22b97f4
  67:     0x7fd69a0464fd - rustc::ty::context::TyCtxt::create_and_enter::h900d40186934f964
  68:     0x7fd699f82d3c - rustc_driver::driver::compile_input::he3d8ffa7fc6fb0b1
  69:     0x7fd69a02ebd9 - rustc_driver::run_compiler_with_pool::h857bb7f679bff0bc
  70:     0x7fd699f4989c - <scoped_tls::ScopedKey<T>>::set::h5dd3e2bfa44e92e2
  71:     0x7fd699f49591 - <scoped_tls::ScopedKey<T>>::set::h4da18d6fc1a78347
  72:     0x7fd699fe57ba - syntax::with_globals::h0ede7174af3ad87c
  73:     0x7fd699f48012 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hb5f73936aff14500
  74:     0x7fd699c22c99 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:105
  75:     0x7fd69a02bf9e - rustc_driver::run::hcfb204ccb1735e19
  76:     0x7fd69a0398ca - rustc_driver::main::ha16d2705f2f29af0
  77:     0x556fa6531b52 - std::rt::lang_start::{{closure}}::h0dc69d8f000bba40
  78:     0x7fd699be2462 - std::panicking::try::do_call::h09f2e11537387364
                               at libstd/rt.rs:59
                               at libstd/panicking.rs:310
  79:     0x7fd699c22c99 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:105
  80:     0x7fd699be7175 - std::rt::lang_start_internal::h62ba303339938384
                               at libstd/panicking.rs:289
                               at libstd/panic.rs:392
                               at libstd/rt.rs:58
  81:     0x556fa6531bb3 - main
  82:     0x7fd6997bcb96 - __libc_start_main
  83:     0x556fa6531a38 - <unknown>
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.29.0-beta.14 (edfc72bc7 2018-09-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `parity-ethereum`.

