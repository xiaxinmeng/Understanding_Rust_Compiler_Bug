
   Compiling farklemaster v0.1.0 (/home/gentz/Documents/gfx/farklemaster)
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> src/main.rs:1:12
  |
1 | #![feature(const_generics, untagged_unions)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default

error: internal compiler error: src/librustc/ty/subst.rs:597: const parameter `N/#0` (Const { ty: usize, val: Param(N/#0) }/0) out of range when substituting substs=[]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:578:9
stack backtrace:
   0:     0x7ff3551608e2 - backtrace::backtrace::libunwind::trace::hd6bd421890c8dc15
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/libunwind.rs:88
   1:     0x7ff3551608e2 - backtrace::backtrace::trace_unsynchronized::h5794e5e8aeae251a
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/mod.rs:66
   2:     0x7ff3551608e2 - std::sys_common::backtrace::_print::hac9caa5aa5e7e21f
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7ff3551608e2 - std::sys_common::backtrace::print::hd000d0e9e39c8ae2
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7ff3551608e2 - std::panicking::default_hook::{{closure}}::hbd4c7db8b4db9df0
                               at src/libstd/panicking.rs:200
   5:     0x7ff3551605c6 - std::panicking::default_hook::h01de047d1b30b7c3
                               at src/libstd/panicking.rs:214
   6:     0x7ff357045051 - rustc::util::common::panic_hook::h959c89d52f8e38a3
   7:     0x7ff35516111c - std::panicking::rust_panic_with_hook::h9c39618c52808332
                               at src/libstd/panicking.rs:481
   8:     0x7ff356fd3b5d - std::panicking::begin_panic::h733f239de789cec9
   9:     0x7ff356b176ed - rustc_errors::Handler::span_bug::hdbbf5b45f64a3a48
  10:     0x7ff356cf5d8e - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h7b245dcf8371dcf6
  11:     0x7ff356cf0a13 - rustc::ty::context::tls::with_opt::{{closure}}::h3e31f0d12daf60e6
  12:     0x7ff356cf06b3 - rustc::ty::context::tls::with_context_opt::hc4b09da8394c7a33
  13:     0x7ff356cf06f7 - rustc::ty::context::tls::with_opt::hc886ba09634c690b
  14:     0x7ff356cf5c98 - rustc::util::bug::opt_span_bug_fmt::hb9038d097b69641d
  15:     0x7ff356cf5c4a - rustc::util::bug::span_bug_fmt::h2ea8aa55f8f30a61
  16:     0x7ff356b0143b - <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_const::hb48e47c51b05122f
  17:     0x7ff3560c1030 - rustc::traits::codegen::<impl rustc::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::hd959ef59d4c7a154
  18:     0x7ff3561bfe07 - rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand::h5439d8be59fa7e84
  19:     0x7ff3561ce01c - rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run::h7748d8ea907f8dd2
  20:     0x7ff3560294fa - rustc_mir::const_eval::const_eval_raw_provider::h50f4217b07101ea6
  21:     0x7ff3560df453 - rustc::ty::query::__query_compute::const_eval_raw::h80089ae490de50d3
  22:     0x7ff35603753a - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute::hfbe2000d47326943
  23:     0x7ff35618f2ff - rustc::dep_graph::graph::DepGraph::with_task_impl::h3d581bdb0f038452
  24:     0x7ff356048ddd - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h04f0276d1d39b5ce
  25:     0x7ff35602804e - rustc_mir::const_eval::const_eval_provider::h5ef25350ec6bfb7a
  26:     0x7ff3560de843 - rustc::ty::query::__query_compute::const_eval::hcfec520566953d99
  27:     0x7ff35603734a - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute::h15b53c1ead49c954
  28:     0x7ff356189dfa - rustc::dep_graph::graph::DepGraph::with_task_impl::h0722aefbbfab94b7
  29:     0x7ff35604c3bb - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h054986b9770876b7
  30:     0x7ff356027fdc - rustc_mir::const_eval::const_eval_provider::h5ef25350ec6bfb7a
  31:     0x7ff356b0f67a - rustc::ty::query::__query_compute::const_eval::h5400c26d8b541525
  32:     0x7ff356f06c8a - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute::h15b53c1ead49c954
  33:     0x7ff356a4e5d6 - rustc::dep_graph::graph::DepGraph::with_task_impl::h0ea8b6413eda918d
  34:     0x7ff356d5e114 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h336bbeabb89a5c31
  35:     0x7ff356f6240e - rustc::ty::sty::Const::eval::h4fdf07c982e42327
  36:     0x7ff356f454c1 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::hd827ef16defe2054
  37:     0x7ff356f44713 - rustc::traits::project::normalize_with_depth::h10b5af39e0fe3c60
  38:     0x7ff357032db6 - rustc::traits::fully_normalize::hb8f66f19d4a2446a
  39:     0x7ff356efadbb - rustc::ty::context::GlobalCtxt::enter_local::hd3e03ea515f79db1
  40:     0x7ff356f5a001 - rustc::ty::util::<impl rustc::ty::ParamEnv>::can_type_implement_copy::heb7b7aded3373b0d
  41:     0x7ff355c917ec - rustc_typeck::coherence::builtin::check_trait::hcc1aeeaad417967b
  42:     0x7ff355e48559 - rustc_typeck::coherence::coherent_trait::ha4a271b5ca3a4a7c
  43:     0x7ff355c6f6aa - rustc::ty::query::__query_compute::coherent_trait::h2362077cee3c4017
  44:     0x7ff355d0304b - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::coherent_trait>::compute::h082442909284edf8
  45:     0x7ff355ca96ed - rustc::dep_graph::graph::DepGraph::with_task_impl::had495ae8aaa5b460
  46:     0x7ff355d7e1aa - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hf784bfb861e065c4
  47:     0x7ff355e48abc - rustc_typeck::coherence::check_coherence::h7d3c988311e2f6fb
  48:     0x7ff355c75516 - rustc::util::common::time::he8df946c085bb506
  49:     0x7ff355e4abb8 - rustc_typeck::check_crate::h364c57db6cf5f232
  50:     0x7ff35574729a - rustc_interface::passes::analysis::ha4ca96561e248eef
  51:     0x7ff3556348e1 - rustc::ty::query::__query_compute::analysis::hdc6f2eb00bbcb349
  52:     0x7ff355635bad - rustc::dep_graph::graph::DepGraph::with_task_impl::h772ba78078a0c1e4
  53:     0x7ff355645c8e - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h028ccb11c7ddf2a3
  54:     0x7ff35566631a - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h12666f2fa53b6ad2
  55:     0x7ff3557c43da - rustc_interface::passes::create_global_ctxt::{{closure}}::hcb4b3b12c08b760f
  56:     0x7ff35566853e - rustc_interface::interface::run_compiler_in_existing_thread_pool::h6ec02d4511eb35d1
  57:     0x7ff355678022 - std::thread::local::LocalKey<T>::with::h062bc1d23e3251b0
  58:     0x7ff35568bd9e - scoped_tls::ScopedKey<T>::set::h437003735b7b6b10
  59:     0x7ff3556c0f12 - syntax::with_globals::hedfc891a8b9153ef
  60:     0x7ff35562f580 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0e9208b9b24491cd
  61:     0x7ff3551717da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  62:     0x7ff355654149 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h220082988942ccab
  63:     0x7ff3551443df - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hf09088b4e23e576a
                               at /rustc/dfd43f0fdd4e6969c7d82c0670d70bf305fbccf8/src/liballoc/boxed.rs:922
  64:     0x7ff355170480 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hce7a5df2f9d13bbe
                               at /rustc/dfd43f0fdd4e6969c7d82c0670d70bf305fbccf8/src/liballoc/boxed.rs:922
  65:     0x7ff355170480 - std::sys_common::thread::start_thread::h29f82b1659010c48
                               at src/libstd/sys_common/thread.rs:13
  66:     0x7ff355170480 - std::sys::unix::thread::Thread::new::thread_start::hf8921307a11df38f
                               at src/libstd/sys/unix/thread.rs:79
  67:     0x7ff35507957f - start_thread
  68:     0x7ff354f8c0e3 - __clone
  69:                0x0 - <unknown>
query stack during panic:
#0 [const_eval_raw] const-evaluating `NibblePack::bytes::{{constant}}#0`
#1 [const_eval] const-evaluating + checking `NibblePack::bytes::{{constant}}#0`
#2 [const_eval] const-evaluating + checking `NibblePack::bytes::{{constant}}#0`
#3 [coherent_trait] coherence checking all impls of trait `std::marker::Copy`
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (dfd43f0fd 2019-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `farklemaster`.

To learn more, run the command again with --verbose.

