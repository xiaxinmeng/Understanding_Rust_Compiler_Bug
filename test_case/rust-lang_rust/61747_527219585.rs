
error: internal compiler error: src/librustc/ty/subst.rs:597: const parameter `C/#0` (Const { ty: usize, val: Param(C/#0) }/0) out of range when substituting substs=[]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:578:9
stack backtrace:
   0:     0x7fe89b8328e2 - backtrace::backtrace::libunwind::trace::hd6bd421890c8dc15
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/libunwind.rs:88
   1:     0x7fe89b8328e2 - backtrace::backtrace::trace_unsynchronized::h5794e5e8aeae251a
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/mod.rs:66
   2:     0x7fe89b8328e2 - std::sys_common::backtrace::_print::hac9caa5aa5e7e21f
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7fe89b8328e2 - std::sys_common::backtrace::print::hd000d0e9e39c8ae2
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7fe89b8328e2 - std::panicking::default_hook::{{closure}}::hbd4c7db8b4db9df0
                               at src/libstd/panicking.rs:200
   5:     0x7fe89b8325c6 - std::panicking::default_hook::h01de047d1b30b7c3
                               at src/libstd/panicking.rs:214
   6:     0x7fe89d711051 - rustc::util::common::panic_hook::h959c89d52f8e38a3
   7:     0x7fe89b83311c - std::panicking::rust_panic_with_hook::h9c39618c52808332
                               at src/libstd/panicking.rs:481
   8:     0x7fe89d69fb5d - std::panicking::begin_panic::h733f239de789cec9
   9:     0x7fe89d1e36ed - rustc_errors::Handler::span_bug::hdbbf5b45f64a3a48
  10:     0x7fe89d3c1d8e - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h7b245dcf8371dcf6
  11:     0x7fe89d3bca13 - rustc::ty::context::tls::with_opt::{{closure}}::h3e31f0d12daf60e6
  12:     0x7fe89d3bc6b3 - rustc::ty::context::tls::with_context_opt::hc4b09da8394c7a33
  13:     0x7fe89d3bc6f7 - rustc::ty::context::tls::with_opt::hc886ba09634c690b
  14:     0x7fe89d3c1c98 - rustc::util::bug::opt_span_bug_fmt::hb9038d097b69641d
  15:     0x7fe89d3c1c4a - rustc::util::bug::span_bug_fmt::h2ea8aa55f8f30a61
  16:     0x7fe89d1cd43b - <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_const::hb48e47c51b05122f
  17:     0x7fe89c78d030 - rustc::traits::codegen::<impl rustc::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::hd959ef59d4c7a154
  18:     0x7fe89c88be07 - rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand::h5439d8be59fa7e84
  19:     0x7fe89c89942b - rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run::h7748d8ea907f8dd2
  20:     0x7fe89c6f54fa - rustc_mir::const_eval::const_eval_raw_provider::h50f4217b07101ea6
  21:     0x7fe89c7ab453 - rustc::ty::query::__query_compute::const_eval_raw::h80089ae490de50d3
  22:     0x7fe89c70353a - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute::hfbe2000d47326943
  23:     0x7fe89c85b4ea - rustc::dep_graph::graph::DepGraph::with_task_impl::h3d581bdb0f038452
  24:     0x7fe89c715b25 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h04f0276d1d39b5ce
  25:     0x7fe89c6f404e - rustc_mir::const_eval::const_eval_provider::h5ef25350ec6bfb7a
  26:     0x7fe89c7aa843 - rustc::ty::query::__query_compute::const_eval::hcfec520566953d99
  27:     0x7fe89c70334a - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute::h15b53c1ead49c954
  28:     0x7fe89c855fcd - rustc::dep_graph::graph::DepGraph::with_task_impl::h0722aefbbfab94b7
  29:     0x7fe89c719660 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h054986b9770876b7
  30:     0x7fe89c6f3fdc - rustc_mir::const_eval::const_eval_provider::h5ef25350ec6bfb7a
  31:     0x7fe89d1db67a - rustc::ty::query::__query_compute::const_eval::h5400c26d8b541525
  32:     0x7fe89d5d2c8a - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute::h15b53c1ead49c954
  33:     0x7fe89d11a829 - rustc::dep_graph::graph::DepGraph::with_task_impl::h0ea8b6413eda918d
  34:     0x7fe89d42b617 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h336bbeabb89a5c31
  35:     0x7fe89d62e40e - rustc::ty::sty::Const::eval::h4fdf07c982e42327
  36:     0x7fe89d1e7edd - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h2470bdf143c8a4dd
  37:     0x7fe89d61983b - rustc::ty::fold::TypeFoldable::fold_with::h57f75657b5d54335
  38:     0x7fe89d611407 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::hd827ef16defe2054
  39:     0x7fe89c30c087 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::hda36c0d6c509b04b
  40:     0x7fe89c3a469c - rustc::ty::fold::TypeFoldable::fold_with::hed3185312e3e454b
  41:     0x7fe89c4cf734 - rustc::traits::project::normalize::h96653d622de1d8d5
  42:     0x7fe89c2f8688 - rustc_typeck::check::FnCtxt::normalize_associated_types_in::h2cd17f386a544bdd
  43:     0x7fe89c458f53 - rustc::ty::context::GlobalCtxt::enter_local::h8b3f565217cb0b38
  44:     0x7fe89c320f14 - rustc_typeck::check::wfcheck::check_associated_item::hd01e97a71be22267
  45:     0x7fe89c33d65a - rustc::ty::query::__query_compute::check_impl_item_well_formed::h75094b4f0bfdd75b
  46:     0x7fe89c3cf4ab - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_impl_item_well_formed>::compute::h68b9eb8bd2d71f64
  47:     0x7fe89c36ab57 - rustc::dep_graph::graph::DepGraph::with_task_impl::h2563a8ba111929f4
  48:     0x7fe89c41e4f8 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h93b7ffc635470162
  49:     0x7fe89c323836 - <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc::hir::itemlikevisit::ParItemLikeVisitor>::visit_impl_item::h1a5a3cb080bed249
  50:     0x7fe89b8437da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  51:     0x7fe89c484d0c - rustc_data_structures::sync::par_for_each_in::h0d521e56dc92ea2d
  52:     0x7fe89b8437da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  53:     0x7fe89c3180f9 - rustc::hir::Crate::par_visit_all_item_likes::hc09c5cb8eb261279
  54:     0x7fe89c34120b - rustc::util::common::time::ha4a40dc4ea9e9cd3
  55:     0x7fe89c516c43 - rustc_typeck::check_crate::h364c57db6cf5f232
  56:     0x7fe89be1329a - rustc_interface::passes::analysis::ha4ca96561e248eef
  57:     0x7fe89bd008e1 - rustc::ty::query::__query_compute::analysis::hdc6f2eb00bbcb349
  58:     0x7fe89bd01c69 - rustc::dep_graph::graph::DepGraph::with_task_impl::h772ba78078a0c1e4
  59:     0x7fe89bd115ce - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h028ccb11c7ddf2a3
  60:     0x7fe89bd3231a - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h12666f2fa53b6ad2
  61:     0x7fe89be903da - rustc_interface::passes::create_global_ctxt::{{closure}}::hcb4b3b12c08b760f
  62:     0x7fe89bd3453e - rustc_interface::interface::run_compiler_in_existing_thread_pool::h6ec02d4511eb35d1
  63:     0x7fe89bd44022 - std::thread::local::LocalKey<T>::with::h062bc1d23e3251b0
  64:     0x7fe89bd57d9e - scoped_tls::ScopedKey<T>::set::h437003735b7b6b10
  65:     0x7fe89bd8cf12 - syntax::with_globals::hedfc891a8b9153ef
  66:     0x7fe89bcfb580 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0e9208b9b24491cd
  67:     0x7fe89b8437da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  68:     0x7fe89bd20149 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h220082988942ccab
  69:     0x7fe89b8163df - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hf09088b4e23e576a
                               at /rustc/dfd43f0fdd4e6969c7d82c0670d70bf305fbccf8/src/liballoc/boxed.rs:922
  70:     0x7fe89b842480 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hce7a5df2f9d13bbe
                               at /rustc/dfd43f0fdd4e6969c7d82c0670d70bf305fbccf8/src/liballoc/boxed.rs:922
  71:     0x7fe89b842480 - std::sys_common::thread::start_thread::h29f82b1659010c48
                               at src/libstd/sys_common/thread.rs:13
  72:     0x7fe89b842480 - std::sys::unix::thread::Thread::new::thread_start::hf8921307a11df38f
                               at src/libstd/sys/unix/thread.rs:79
  73:     0x7fe89b7a0f27 - start_thread
                               at /builddir/glibc-2.30/nptl/pthread_create.c:479
  74:     0x7fe89b6c1e0f - __GI___clone
  75:                0x0 - <unknown>
query stack during panic:
#0 [const_eval_raw] const-evaluating `Const::<C>::successor::{{constant}}#0`
#1 [const_eval] const-evaluating + checking `Const::<C>::successor::{{constant}}#0`
#2 [const_eval] const-evaluating + checking `Const::<C>::successor::{{constant}}#0`
#3 [check_impl_item_well_formed] processing `Const::<C>::successor`
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (dfd43f0fd 2019-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin
