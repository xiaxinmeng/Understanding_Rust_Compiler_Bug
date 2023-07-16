
   Compiling glutin v0.20.0 (/home/gentz/Documents/gfx/glutin/glutin)
error: internal compiler error: src/librustc/ty/query/plumbing.rs:1195: Cannot force dep node: coherent_trait(core[c27c]::ops[0]::drop[0]::Drop[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:635:9
stack backtrace:
   0:     0x7fcdb8a8be33 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h3372618c1dfa6b4f
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1:     0x7fcdb8a83cbb - std::sys_common::backtrace::_print::h9f1a8b3b45fbc4ff
                               at src/libstd/sys_common/backtrace.rs:71
   2:     0x7fcdb8a88096 - std::panicking::default_hook::{{closure}}::h642660577a3fa93f
                               at src/libstd/sys_common/backtrace.rs:59
                               at src/libstd/panicking.rs:197
   3:     0x7fcdb8a87e29 - std::panicking::default_hook::h1f92c914f4826294
                               at src/libstd/panicking.rs:211
   4:     0x7fcdb67a9280 - rustc::util::common::panic_hook::hb27b7a7fe28535d3
   5:     0x7fcdb8a88888 - std::panicking::rust_panic_with_hook::h2499b323efcdee7b
                               at src/libstd/panicking.rs:478
   6:     0x7fcdb539c2fc - std::panicking::begin_panic::hbc393ece67ffccec
   7:     0x7fcdb53b520e - rustc_errors::Handler::bug::h992abb17fa9135c7
   8:     0x7fcdb6629e12 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::hba05039814938e28
   9:     0x7fcdb66273d9 - rustc::ty::context::tls::with_opt::{{closure}}::h2fbde6838cb0da14
  10:     0x7fcdb66272f4 - rustc::ty::context::tls::with_context_opt::h1848e9f588ed7c7c
  11:     0x7fcdb6627386 - rustc::ty::context::tls::with_opt::hda68cede86f45ca8
  12:     0x7fcdb6629d24 - rustc::util::bug::opt_span_bug_fmt::h8d39a2497ec10a39
  13:     0x7fcdb6629c96 - rustc::util::bug::bug_fmt::hd0622b7797bb272b
  14:     0x7fcdb67a817e - rustc::ty::query::plumbing::force_from_dep_node::hc00c0fa61ccff7f3
  15:     0x7fcdb66a3fa8 - rustc::dep_graph::graph::DepGraph::try_mark_previous_green::hffebd1981cd7e315
  16:     0x7fcdb66a3d50 - rustc::dep_graph::graph::DepGraph::try_mark_green::hbcce1d4eaeaf4b31
  17:     0x7fcdb61f9e33 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h36f8ae853d0c2ae4
  18:     0x7fcdb636652e - rustc::ty::context::GlobalCtxt::enter_local::ha26625901515bc1e
  19:     0x7fcdb66e7cdf - rustc::ty::util::<impl rustc::ty::ParamEnv>::can_type_implement_copy::h482dbb2018fbba75
  20:     0x7fcdb371f568 - rustc_typeck::coherence::builtin::check_trait::hdd4ff73d19d6adeb
  21:     0x7fcdb39035b7 - rustc_typeck::coherence::coherent_trait::h07cad80800294b1d
  22:     0x7fcdb37322ea - rustc::ty::query::__query_compute::coherent_trait::hf6b99107af7d594d
  23:     0x7fcdb382deec - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::coherent_trait>::compute::h27c41708022bb513
  24:     0x7fcdb37b528a - rustc::dep_graph::graph::DepGraph::with_task_impl::h0a345413412c2898
  25:     0x7fcdb3885901 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hbbc900180cbb5d4e
  26:     0x7fcdb3911e1c - rustc_typeck::check_crate::{{closure}}::{{closure}}::ha853412593ccc63e
  27:     0x7fcdb38e3540 - rustc::util::common::time::hb1ae215768260e12
  28:     0x7fcdb3926b2b - rustc_typeck::check_crate::h2b6870d3c28d4e12
  29:     0x7fcdb844d6c3 - rustc_interface::passes::analysis::h38b5dbf20157ecaf
  30:     0x7fcdb8d6a4c2 - rustc::ty::query::__query_compute::analysis::ha3e2013bf4448f36
  31:     0x7fcdb8d9cbc8 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::h06acbf4f66661171
  32:     0x7fcdb8d79972 - rustc::dep_graph::graph::DepGraph::with_task_impl::h81b152d7161e4277
  33:     0x7fcdb8da0fcd - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::he8d7ff02af11fb9d
  34:     0x7fcdb8d611b9 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h966f0c6e593796ff
  35:     0x7fcdb8490194 - rustc_interface::passes::create_global_ctxt::{{closure}}::he748dc4dda84a615
  36:     0x7fcdb8d63a28 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h98ba83166181d50a
  37:     0x7fcdb8d45eb3 - std::thread::local::LocalKey<T>::with::hc3ad01304ddb3f39
  38:     0x7fcdb8da9dd4 - scoped_tls::ScopedKey<T>::set::h63f4b18b85ac3f43
  39:     0x7fcdb8dda7ff - syntax::with_globals::h22ba1c6983c4b3b2
  40:     0x7fcdb8d64ec7 - std::sys_common::backtrace::__rust_begin_short_backtrace::h792e381dca8f473f
  41:     0x7fcdb8a99569 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:87
  42:     0x7fcdb8d65858 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc7b45209b191d8d6
  43:     0x7fcdb8a6afbe - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h7987e96fd90cd5f4
                               at /rustc/acd8dd6a50d505057a7d7ad8d0d7a4c2bd274200/src/liballoc/boxed.rs:702
  44:     0x7fcdb8a982cf - std::sys::unix::thread::Thread::new::thread_start::h0c39f9a5d3dffdee
                               at /rustc/acd8dd6a50d505057a7d7ad8d0d7a4c2bd274200/src/liballoc/boxed.rs:702
                               at src/libstd/sys_common/thread.rs:14
                               at src/libstd/sys/unix/thread.rs:80
  45:     0x7fcdb899fb91 - start_thread
                               at /usr/src/debug/glibc/nptl/pthread_create.c:486
  46:     0x7fcdb88aaf92 - __clone
                               at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  47:                0x0 - <unknown>
query stack during panic:
#0 [adt_destructor] processing `api::egl::SurfaceType`
#1 [coherent_trait] coherence checking all impls of trait `std::marker::Copy`
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.35.0-nightly (acd8dd6a5 2019-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `glutin`.

To learn more, run the command again with --verbose.
