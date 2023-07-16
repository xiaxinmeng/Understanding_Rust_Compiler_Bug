
   Compiling ice-test v0.1.0 ([...])
     Running `rustc --edition=2018 --crate-name ice_test src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C debuginfo=2 -C metadata=f3a24cbbef97f97a -C extra-filename=-f3a24cbbef97f97a --out-dir [...]/target/debug/deps -C incremental=[...]/target/debug/incremental -L dependency=[...]/target/debug/deps`
error: internal compiler error: src/librustc/ty/subst.rs:610: type parameter `A/#0` (A/0) out of range when substituting (root type=Some(fn(A))) substs=[]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:845:9
stack backtrace:
   0:     0x7fe11bdf9a34 - backtrace::backtrace::libunwind::trace::h8c53309f22d88c8a
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7fe11bdf9a34 - backtrace::backtrace::trace_unsynchronized::h6fd82dd611adbaf7
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7fe11bdf9a34 - std::sys_common::backtrace::_print_fmt::h9030d1f31a3f4bb6
                               at src/libstd/sys_common/backtrace.rs:84
   3:     0x7fe11bdf9a34 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h63caf1987b315e75
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7fe11be31f7c - core::fmt::write::ha838978e5554a3fd
                               at src/libcore/fmt/mod.rs:1030
   5:     0x7fe11bdeddc7 - std::io::Write::write_fmt::h9709be9e8aefaa67
                               at src/libstd/io/mod.rs:1412
   6:     0x7fe11bdfdede - std::sys_common::backtrace::_print::ha7997a59b8100963
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7fe11bdfdede - std::sys_common::backtrace::print::h0e6bebe91d0af16b
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7fe11bdfdede - std::panicking::default_hook::{{closure}}::h253a69014f4380df
                               at src/libstd/panicking.rs:188
   9:     0x7fe11bdfdbd1 - std::panicking::default_hook::h9903e5ad08e35d58
                               at src/libstd/panicking.rs:205
  10:     0x7fe11c322ed3 - rustc_driver::report_ice::h73c5774c65a339e0
  11:     0x7fe11bdfe6b0 - std::panicking::rust_panic_with_hook::h7fda44e006fd9fdb
                               at src/libstd/panicking.rs:468
  12:     0x7fe11de2913d - std::panicking::begin_panic::hd5bf2b3173a52cca
  13:     0x7fe11df472ba - rustc_errors::HandlerInner::span_bug::h089aba806cbdd369
  14:     0x7fe11df47b3a - rustc_errors::Handler::span_bug::h9be7751cca87b642
  15:     0x7fe11dcea5ae - rustc::util::bug::opt_span_bug_fmt::{{closure}}::hae1bc3a2d9f2c136
  16:     0x7fe11dce92d3 - rustc::ty::context::tls::with_opt::{{closure}}::h43e3789551ffc892
  17:     0x7fe11dce8fa3 - rustc::ty::context::tls::with_context_opt::h98ab1231a3b31ef6
  18:     0x7fe11dce8fe7 - rustc::ty::context::tls::with_opt::h0886b5e1cb2a2ade
  19:     0x7fe11dcea4b8 - rustc::util::bug::opt_span_bug_fmt::hfd2140e82a3785fe
  20:     0x7fe11dcea46a - rustc::util::bug::span_bug_fmt::h03c04fb296e4972a
  21:     0x7fe11da97faf - <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty::he25b479fa936b3d2
  22:     0x7fe11dab1f36 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h0bb7ca31d5f1319d
  23:     0x7fe11dd8425c - rustc::ty::fold::TypeFoldable::fold_with::h23c01612ac730b57
  24:     0x7fe11dda20be - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with::hdc0a8805009b919b
  25:     0x7fe11da97d2e - <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty::he25b479fa936b3d2
  26:     0x7fe11c89e6c2 - rustc_typeck::check::FnCtxt::instantiate_value_path::hbb8487102544e9e2
  27:     0x7fe11c8670b5 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::hac87f10cbb278c19
  28:     0x7fe11c86409b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::hd93b46544b24a2f9
  29:     0x7fe11c87e7b9 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h62a4ecde155086f2
  30:     0x7fe11c864b4c - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::hac87f10cbb278c19
  31:     0x7fe11c86409b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::hd93b46544b24a2f9
  32:     0x7fe11c89a389 - rustc_typeck::check::FnCtxt::check_stmt::h5aba78a030084850
  33:     0x7fe11c89a9db - rustc_typeck::check::FnCtxt::check_block_with_expected::h6a4dcd18788e61a5
  34:     0x7fe11c864b67 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::hac87f10cbb278c19
  35:     0x7fe11c86409b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::hd93b46544b24a2f9
  36:     0x7fe11c86df9b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr::h786ed8cdfb3041e1
  37:     0x7fe11c887f0f - rustc_typeck::check::check_fn::he2c2e43c32d58706
  38:     0x7fe11c9f6c0c - rustc::ty::context::tls::with_context::{{closure}}::h50e6d6cb9af96418
  39:     0x7fe11c886d1e - rustc_typeck::check::typeck_tables_of::hc70672beb76bdf97
  40:     0x7fe11c8fd2aa - rustc::ty::query::__query_compute::typeck_tables_of::hb8839e53cb93a3a2
  41:     0x7fe11c977b0b - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::h7acd6750469c3b84
  42:     0x7fe11c92e312 - rustc::dep_graph::graph::DepGraph::with_task_impl::hbf45fe6c06d0bb6a
  43:     0x7fe11c985ed8 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h000d0542680a1237
  44:     0x7fe11c97422f - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::h8eff7b833346bd0e
  45:     0x7fe11c88674d - rustc_typeck::check::typeck_item_bodies::h08dfc65035cef777
  46:     0x7fe11c8fd6f5 - rustc::ty::query::__query_compute::typeck_item_bodies::h28e24bddd31d09b0
  47:     0x7fe11c92f604 - rustc::dep_graph::graph::DepGraph::with_task_impl::hdb02b90e73ec1b7f
  48:     0x7fe11c9d7c48 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hf06fdbe7848d1f00
  49:     0x7fe11ca9f874 - rustc_typeck::check_crate::hbbaa02265f9abfd1
  50:     0x7fe11c3a9514 - rustc_interface::passes::analysis::h2e92fec7a93f5345
  51:     0x7fe11c2f8341 - rustc::ty::query::__query_compute::analysis::h8802febcbe5937d2
  52:     0x7fe11c2f9b01 - rustc::dep_graph::graph::DepGraph::with_task_impl::h1acefbafe76bf66d
  53:     0x7fe11c311566 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h5a1b63008bff09de
  54:     0x7fe11c30ec3a - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::hcb5ba44b71e57b24
  55:     0x7fe11c3d512a - rustc_interface::passes::create_global_ctxt::{{closure}}::habdec283a2026a49
  56:     0x7fe11c30e36e - rustc_interface::passes::BoxedGlobalCtxt::enter::h8e1c39c10064ea6f
  57:     0x7fe11c2d8880 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hb564b888470767db
  58:     0x7fe11c2e2aa1 - std::thread::local::LocalKey<T>::with::h2707847c2ea69d77
  59:     0x7fe11c2e443e - scoped_tls::ScopedKey<T>::set::h2134dafd21ff30c6
  60:     0x7fe11c2dfdc2 - syntax::with_globals::heb74cb8f2f2baa28
  61:     0x7fe11c2ea6e1 - std::sys_common::backtrace::__rust_begin_short_backtrace::hd2c3273064cc913a
  62:     0x7fe11be0ef9a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:81
  63:     0x7fe11c2ebd59 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h723616ac2301fe2c
  64:     0x7fe11bddfcff - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h70f621c89bd46176
                               at /rustc/0f0c640e0ee5a9ad365e78e3c62239b3d65b7045/src/liballoc/boxed.rs:942
  65:     0x7fe11be0d9c0 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h394f96b4177f765a
                               at /rustc/0f0c640e0ee5a9ad365e78e3c62239b3d65b7045/src/liballoc/boxed.rs:942
  66:     0x7fe11be0d9c0 - std::sys_common::thread::start_thread::haf8faa22fb2bc269
                               at src/libstd/sys_common/thread.rs:13
  67:     0x7fe11be0d9c0 - std::sys::unix::thread::Thread::new::thread_start::h09d290fc9c478975
                               at src/libstd/sys/unix/thread.rs:79
  68:     0x7fe11bd794cf - start_thread
  69:     0x7fe11bc962d3 - clone
  70:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (0f0c640e0 2019-11-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `func`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

error: could not compile `ice-test`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name ice_test src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C debuginfo=2 -C metadata=f3a24cbbef97f97a -C extra-filename=-f3a24cbbef97f97a --out-dir [...]/target/debug/deps -C incremental=[...]/target/debug/incremental -L dependency=[...]/target/debug/deps` (exit code: 101)
