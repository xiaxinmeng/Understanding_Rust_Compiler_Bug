
error: internal compiler error: src/librustc/ty/sty.rs:2522: expected bits of usize, got Const {
    ty: usize,
    val: Param(
        N/#1,
    ),
}

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:881:9
stack backtrace:
   0:     0x7f8276766644 - backtrace::backtrace::libunwind::trace::h9d5fa1647efe1ab1
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1:     0x7f8276766644 - backtrace::backtrace::trace_unsynchronized::h86fb9c5f1eca30ef
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2:     0x7f8276766644 - std::sys_common::backtrace::_print_fmt::he9ba33a8eb3eb9ee
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x7f8276766644 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5b9c1af25ae7c717
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7f827679f7ec - core::fmt::write::h307eb561be2a404c
                               at src/libcore/fmt/mod.rs:1052
   5:     0x7f8276757fa7 - std::io::Write::write_fmt::h9f10c0557c95103f
                               at src/libstd/io/mod.rs:1428
   6:     0x7f827676b435 - std::sys_common::backtrace::_print::h9ba9ae783f57eedf
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7f827676b435 - std::sys_common::backtrace::print::hfc2586ace23d36ac
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7f827676b435 - std::panicking::default_hook::{{closure}}::hae11ef594f739717
                               at src/libstd/panicking.rs:204
   9:     0x7f827676b176 - std::panicking::default_hook::h91a2179e24328404
                               at src/libstd/panicking.rs:224
  10:     0x563c96f7b591 - clippy_driver::report_clippy_ice::h00b78117f8199052
  11:     0x7f827676bb65 - std::panicking::rust_panic_with_hook::h5394bdfc184d7cc7
                               at src/libstd/panicking.rs:474
  12:     0x7f82795865d3 - std::panicking::begin_panic::h586c0171fbdab168
  13:     0x7f82795c2a10 - rustc_errors::HandlerInner::bug::hca42a38f4a7f5b9a
  14:     0x7f82795c1860 - rustc_errors::Handler::bug::he56a55254aefe7ac
  15:     0x7f82792b2d29 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::hcf8a9328b7d9651c
  16:     0x7f82792a90bb - rustc::ty::context::tls::with_opt::{{closure}}::hc97cc9e8bd8ae5d8
  17:     0x7f82792a9062 - rustc::ty::context::tls::with_opt::hc754df500ae2375b
  18:     0x7f82792b2c38 - rustc::util::bug::opt_span_bug_fmt::h7906b53d89b4c2e1
  19:     0x7f82792b2ba2 - rustc::util::bug::bug_fmt::hd3ba947be3f6a8df
  20:     0x563c96f9f47f - rustc::ty::sty::Const::eval_bits::{{closure}}::he6bca075ade2f14a
  21:     0x563c96fb1f47 - <clippy_lints::indexing_slicing::IndexingSlicing as rustc_lint::passes::LateLintPass>::check_expr::hc5e15fc7d2e672c0
  22:     0x7f8278973e43 - <rustc_lint::late::LateLintPassObjects as rustc_lint::passes::LateLintPass>::check_expr::hf6d54270d843a380
  23:     0x7f827702c6e2 - <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_expr::h9f4179d7d857c919
  24:     0x7f8276fc6686 - rustc_hir::intravisit::walk_expr::he754354f137a537e
  25:     0x7f827702c6ed - <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_expr::h9f4179d7d857c919
  26:     0x7f8276fc6882 - rustc_hir::intravisit::walk_expr::he754354f137a537e
  27:     0x7f827702c6ed - <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_expr::h9f4179d7d857c919
  28:     0x7f8276fc4bf4 - rustc_hir::intravisit::walk_fn::h3ff39dabd4ec61b9
  29:     0x7f827702d8a8 - <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_fn::hbb0b96ca91285ee7
  30:     0x7f8276fc226b - rustc_hir::intravisit::walk_impl_item::h598229146e9414df
  31:     0x7f827702ed5a - rustc_hir::intravisit::Visitor::visit_nested_impl_item::hdf4d97b76cbfda60
  32:     0x7f8276fc79d0 - rustc_hir::intravisit::walk_item::h44e80ad5b6277e08
  33:     0x7f827702e470 - rustc_hir::intravisit::Visitor::visit_nested_item::h290b2b9ac183f311
  34:     0x7f8276fc706e - rustc_hir::intravisit::walk_item::h44e80ad5b6277e08
  35:     0x7f827702e470 - rustc_hir::intravisit::Visitor::visit_nested_item::h290b2b9ac183f311
  36:     0x7f8276fc126e - rustc_hir::intravisit::walk_crate::h68de15a1d07fe82c
  37:     0x7f827701fecd - rustc_lint::late::late_lint_pass_crate::he7676702f03bb31a
  38:     0x7f827701f6eb - rustc_lint::late::late_lint_crate::h303b0802dd8c9a09
  39:     0x7f8276fde60b - rustc_data_structures::sync::join::hf5650324bb2ed613
  40:     0x7f8277020c9b - std::panicking::try::do_call::he8ffa9e7e95783d2
  41:     0x7f827677d457 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:86
  42:     0x7f8276fd9413 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hf6c4f0787ce2bd29
  43:     0x7f827677d457 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:86
  44:     0x7f8276fdc60a - rustc_session::utils::<impl rustc_session::session::Session>::time::h7249262b0ade9d94
  45:     0x7f827705aa36 - rustc_interface::passes::analysis::h63a5c50615a3546c
  46:     0x7f8276df5616 - rustc::ty::query::__query_compute::analysis::h7f7df2982d90698a
  47:     0x7f8276e5cc21 - rustc::dep_graph::graph::DepGraph::with_task_impl::h3547b6b631c468ed
  48:     0x7f8276e327d0 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hc2385fc085b423d4
  49:     0x7f8276e6c577 - rustc::ty::context::tls::enter_global::h8350ae2f23d40602
  50:     0x7f8276e3ed97 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hc7ccb7d92618cd77
  51:     0x7f8276df0394 - std::sys_common::backtrace::__rust_begin_short_backtrace::h4e648e17f1d26b8c
  52:     0x7f827677d457 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:86
  53:     0x7f8276e40796 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h34b4c1a65012f088
  54:     0x7f827674832f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h558ce43155c9780d
                               at /rustc/436494b8f8008b600d64b3951f63c2bb0ea81673/src/liballoc/boxed.rs:1017
  55:     0x7f827677bd60 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hb6d9c1adc82281fd
                               at /rustc/436494b8f8008b600d64b3951f63c2bb0ea81673/src/liballoc/boxed.rs:1017
  56:     0x7f827677bd60 - std::sys_common::thread::start_thread::hcd78bb6c3dec167d
                               at src/libstd/sys_common/thread.rs:13
  57:     0x7f827677bd60 - std::sys::unix::thread::Thread::new::thread_start::h1c16950397532ea6
                               at src/libstd/sys/unix/thread.rs:80
  58:     0x7f82766ce669 - start_thread
  59:     0x7f82765dc323 - clone
  60:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.0.212 (8fbb23f 2020-02-21)

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
