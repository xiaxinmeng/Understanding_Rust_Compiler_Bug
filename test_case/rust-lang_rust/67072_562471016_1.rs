none
error[E0308]: mismatched types
 --> src/main.rs:7:12
  |
7 |     if let Ok() = Res::Something() {}
  |            ^^^^   ---------------- this match expression has type `Res`
  |            |
  |            expected enum `Res`, found enum `std::result::Result`
  |
  = note: expected enum `Res`
             found enum `std::result::Result<_, _>`

error: internal compiler error: src/librustc/ty/subst.rs:610: type parameter `T/#0` (T/0) out of range when substituting (root type=Some(T)) substs=[]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:851:9
stack backtrace:
   0:     0x7f0bb42115e4 - backtrace::backtrace::libunwind::trace::ha87d00c939e2881d
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7f0bb42115e4 - backtrace::backtrace::trace_unsynchronized::h016b03a8ea0b9a9f
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7f0bb42115e4 - std::sys_common::backtrace::_print_fmt::hfe96d08d6bc9b78a
                               at src/libstd/sys_common/backtrace.rs:84
   3:     0x7f0bb42115e4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3b3abdb96a6d8973
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7f0bb4249cfc - core::fmt::write::h3f73f70567a9d69c
                               at src/libcore/fmt/mod.rs:1024
   5:     0x7f0bb4205947 - std::io::Write::write_fmt::h7a7dac6f9b6edbe2
                               at src/libstd/io/mod.rs:1428
   6:     0x7f0bb4215a8e - std::sys_common::backtrace::_print::h26e7c67b8d6504c8
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7f0bb4215a8e - std::sys_common::backtrace::print::h8adaf32a939cdd8b
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7f0bb4215a8e - std::panicking::default_hook::{{closure}}::h8bd3295a48e2a09e
                               at src/libstd/panicking.rs:193
   9:     0x7f0bb4215781 - std::panicking::default_hook::hc4fa4a1f08ea09c8
                               at src/libstd/panicking.rs:210
  10:     0x7f0bb475bdf3 - rustc_driver::report_ice::hbd3df6a32fc97681
  11:     0x7f0bb4216240 - std::panicking::rust_panic_with_hook::h21d3a9bbac50e22b
                               at src/libstd/panicking.rs:475
  12:     0x7f0bb616f6ed - std::panicking::begin_panic::he8a33eedeeea7040
  13:     0x7f0bb61f71bc - rustc_errors::HandlerInner::span_bug::h37e8ebba00204083
  14:     0x7f0bb61f787a - rustc_errors::Handler::span_bug::h02472eb2367a08df
  15:     0x7f0bb5d35149 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h4f95ef7460694392
  16:     0x7f0bb5d343d3 - rustc::ty::context::tls::with_opt::{{closure}}::h5afe3012647ec3fd
  17:     0x7f0bb5d3438b - rustc::ty::context::tls::with_opt::h257d11117491697c
  18:     0x7f0bb5d35038 - rustc::util::bug::opt_span_bug_fmt::h01598306a8f0573e
  19:     0x7f0bb5d34fea - rustc::util::bug::span_bug_fmt::h0835ed719d6e2ed8
  20:     0x7f0bb5dc5b2f - <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty::hbbb72cfb892c1b63
  21:     0x7f0bb5f61f85 - rustc::ty::FieldDef::ty::h140a6bb42e449639
  22:     0x7f0bb4ca6901 - rustc_typeck::check::pat::<impl rustc_typeck::check::FnCtxt>::check_pat_tuple_struct::h9511a8993dce923a
  23:     0x7f0bb4ca3f67 - rustc_typeck::check::pat::<impl rustc_typeck::check::FnCtxt>::check_pat::h736329b26084fbb5
  24:     0x7f0bb4d5e912 - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h49157446740fe904
  25:     0x7f0bb4ca20f0 - rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match::h0d5c3585f435190b
  26:     0x7f0bb4cb5907 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::hf2ab07d921747b7a
  27:     0x7f0bb4cb490b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::h95d2d822b3735bac
  28:     0x7f0bb4cec28e - rustc_typeck::check::FnCtxt::check_block_with_expected::hbffe500c6c048e59
  29:     0x7f0bb4cb53d4 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::hf2ab07d921747b7a
  30:     0x7f0bb4cb490b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::h95d2d822b3735bac
  31:     0x7f0bb4cbebfb - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr::h5fcf7e53ff250fd6
  32:     0x7f0bb4cd916f - rustc_typeck::check::check_fn::hbef22e81bc01eb20
  33:     0x7f0bb4e7a1b1 - rustc::ty::context::tls::with_context::{{closure}}::h7932aedc1560e459
  34:     0x7f0bb4cd7f92 - rustc_typeck::check::typeck_tables_of::h66eb2b8efcae4cc9
  35:     0x7f0bb4eadcbb - rustc::ty::query::__query_compute::typeck_tables_of::h382c93cac39eda56
  36:     0x7f0bb4dfa60b - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::hd4495859c5f91d5a
  37:     0x7f0bb4d8e39a - rustc::dep_graph::graph::DepGraph::with_task_impl::h80bc38d12cb846fc
  38:     0x7f0bb4e47126 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h9874f33e2172d33e
  39:     0x7f0bb4df6d2f - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::h21fb50e660913f81
  40:     0x7f0bb4cd78ad - rustc_typeck::check::typeck_item_bodies::hdd45daad422e9bfc
  41:     0x7f0bb4eae116 - rustc::ty::query::__query_compute::typeck_item_bodies::h08d665ea8132ac5b
  42:     0x7f0bb4d998ae - rustc::dep_graph::graph::DepGraph::with_task_impl::hf917987409a7e530
  43:     0x7f0bb4e5f922 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::he0a7b411ab4af04b
  44:     0x7f0bb4eb80e4 - rustc_typeck::check_crate::h7c28fac46c030bd0
  45:     0x7f0bb48b30a4 - rustc_interface::passes::analysis::hc79844b4fa52b1a0
  46:     0x7f0bb476c352 - rustc::ty::query::__query_compute::analysis::h33a745a409331ce9
  47:     0x7f0bb473e3fd - rustc::dep_graph::graph::DepGraph::with_task_impl::hed8a529c718b7ce2
  48:     0x7f0bb4764bff - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hcfc77b0db446486d
  49:     0x7f0bb470d305 - rustc::ty::context::tls::enter_global::h766d78e54e64fa09
  50:     0x7f0bb47232e5 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hdd156f0d1b582954
  51:     0x7f0bb46f3212 - std::thread::local::LocalKey<T>::with::h6655adf430a51c99
  52:     0x7f0bb46ec92e - scoped_tls::ScopedKey<T>::set::h94183f4219885431
  53:     0x7f0bb4767a44 - syntax::with_globals::hfc11c194934a0802
  54:     0x7f0bb46ed8d0 - std::sys_common::backtrace::__rust_begin_short_backtrace::h802025fd84452bdf
  55:     0x7f0bb4226d1a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:78
  56:     0x7f0bb4703f39 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hf9723bf79635cef7
  57:     0x7f0bb41f789f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hdc5bf436b8728899
                               at /rustc/6d77e45f01079fe3d40180b3e256e414ab379f63/src/liballoc/boxed.rs:969
  58:     0x7f0bb4225740 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h5b53ac75e3d696ad
                               at /rustc/6d77e45f01079fe3d40180b3e256e414ab379f63/src/liballoc/boxed.rs:969
  59:     0x7f0bb4225740 - std::sys_common::thread::start_thread::h6ac750f81e489ecc
                               at src/libstd/sys_common/thread.rs:13
  60:     0x7f0bb4225740 - std::sys::unix::thread::Thread::new::thread_start::hd08318bf11dd0ed0
                               at src/libstd/sys/unix/thread.rs:80
  61:     0x7f0bb4162669 - start_thread
  62:     0x7f0bb4079323 - clone
  63:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (6d77e45f0 2019-12-04) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors
