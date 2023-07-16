
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/cd1ef390e731ed77b90b11b1f77e2c5ca641b261/src/libcore/slice/mod.rs:2791:10
stack backtrace:
   0:     0x7f0574cb8f08 - backtrace::backtrace::libunwind::trace::ha50d6e1700b0eb2c
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7f0574cb8f08 - backtrace::backtrace::trace_unsynchronized::hed6300b0e8a8d34c
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7f0574cb8f08 - std::sys_common::backtrace::_print_fmt::h02d8d6e59c4a6ffd
                               at src/libstd/sys_common/backtrace.rs:77
   3:     0x7f0574cb8f08 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1f642adc5bdda227
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7f0574cf1f1c - core::fmt::write::h3f76f88b1fce4812
                               at src/libcore/fmt/mod.rs:1052
   5:     0x7f0574caa8c7 - std::io::Write::write_fmt::hbb03ae82f9cef6e9
                               at src/libstd/io/mod.rs:1428
   6:     0x7f0574cbdcd5 - std::sys_common::backtrace::_print::h21ddaa53463c7888
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7f0574cbdcd5 - std::sys_common::backtrace::print::h335952ea8eeb247c
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7f0574cbdcd5 - std::panicking::default_hook::{{closure}}::hf030245da7008606
                               at src/libstd/panicking.rs:204
   9:     0x7f0574cbda16 - std::panicking::default_hook::hf0ffbda78fb7d847
                               at src/libstd/panicking.rs:224
  10:     0x563d08964101 - clippy_driver::report_clippy_ice::he21cf02dfc7e1758
  11:     0x7f0574cbe405 - std::panicking::rust_panic_with_hook::hb1798496edc01b7d
                               at src/libstd/panicking.rs:476
  12:     0x7f0574cbdf1b - rust_begin_unwind
                               at src/libstd/panicking.rs:380
  13:     0x7f0574cee911 - core::panicking::panic_fmt::h59f3772040c83eb0
                               at src/libcore/panicking.rs:85
  14:     0x7f0574cee8d5 - core::panicking::panic_bounds_check::h189a5a5e8747cf2a
                               at src/libcore/panicking.rs:63
  15:     0x7f057651eb51 - rustc_mir::borrow_check::type_check::type_check::h576502a08b9b84c8
  16:     0x7f057607578f - rustc_mir::borrow_check::nll::compute_regions::h3259eef4b8afbcfc
  17:     0x7f05764e8f8d - rustc_mir::borrow_check::do_mir_borrowck::hd8604f292f501c96
  18:     0x7f05763a0768 - rustc::ty::context::GlobalCtxt::enter_local::h6691f4f1e1d907c0
  19:     0x7f05764e65fa - rustc_mir::borrow_check::mir_borrowck::h0254b9c083de3cd9
  20:     0x7f05753c2c62 - rustc::ty::query::__query_compute::mir_borrowck::h0117a0c701e4297a
  21:     0x7f057547e49c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::h7999cf1a6634c4a5
  22:     0x7f05754767c9 - rustc::dep_graph::graph::DepGraph::with_task_impl::h934c148d9ba13f5e
  23:     0x7f0575498fc2 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hd533d692e1fbc7b1
  24:     0x7f057547df96 - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::hc8f96788de8a2340
  25:     0x7f0575416846 - rustc_session::utils::<impl rustc_session::session::Session>::time::h3ae7f9e218ae904f
  26:     0x7f05754b1651 - rustc_interface::passes::analysis::h90283b0b6dab2283
  27:     0x7f0575217411 - rustc::ty::query::__query_compute::analysis::h33b18770fdd73590
  28:     0x7f05752702d5 - rustc::dep_graph::graph::DepGraph::with_task_impl::hb1aaaca00de6ca43
  29:     0x7f05752849f4 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h283712dc98c9b497
  30:     0x7f057526da25 - rustc::ty::context::tls::enter_global::hacc4d38b5d7a50f2
  31:     0x7f05752248d7 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h357db3898471cc2c
  32:     0x7f057521307d - scoped_tls::ScopedKey<T>::set::hf06d4d1755916e12
  33:     0x7f057520fdd4 - syntax::with_globals::h3b7b82ab2e985d2f
  34:     0x7f0575214420 - std::sys_common::backtrace::__rust_begin_short_backtrace::h7b107b73a1c5ae11
  35:     0x7f0574ccf9b7 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:86
  36:     0x7f0575228046 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hec461b14581c2073
  37:     0x7f0574c9b38f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hb48f4f4cca7ec7cf
                               at /rustc/cd1ef390e731ed77b90b11b1f77e2c5ca641b261/src/liballoc/boxed.rs:1015
  38:     0x7f0574cce5b0 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h2beff019c430fe28
                               at /rustc/cd1ef390e731ed77b90b11b1f77e2c5ca641b261/src/liballoc/boxed.rs:1015
  39:     0x7f0574cce5b0 - std::sys_common::thread::start_thread::h9de1d746dee0c0e7
                               at src/libstd/sys_common/thread.rs:13
  40:     0x7f0574cce5b0 - std::sys::unix::thread::Thread::new::thread_start::h881c9f01765d0bcb
                               at src/libstd/sys/unix/thread.rs:80
  41:     0x7f0574bdf4cf - start_thread
  42:     0x7f0574aed2d3 - clone
  43:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.0.212 (c0f39cf 2020-01-29)

query stack during panic:
#0 [mir_borrowck] processing `daemon::Daemon::start`
rust-lang/rust-clippy#1 [analysis] running analysis passes on this crate
end of query stack
