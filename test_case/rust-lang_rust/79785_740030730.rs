
  Compiling openssl-sys v0.9.58
   Compiling bytes v0.4.12
thread 'rustc' panicked at 'assertion failed: self.start_pos.to_u32() + total_extra_bytes <= bpos.to_u32()', compiler/rustc_span/src/lib.rs:1495:9
stack backtrace:
   0:     0x7fe080bc56a7 - std::backtrace_rs::backtrace::libunwind::trace::h746c3e9529d524bc
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7fe080bc56a7 - std::backtrace_rs::backtrace::trace_unsynchronized::h86340908ff889faa
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fe080bc56a7 - std::sys_common::backtrace::_print_fmt::h43f85f9b18230404
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fe080bc56a7 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc132ae1a5b5aa7cd
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fe080c3951c - core::fmt::write::hdf023a0036d2a25f
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/core/src/fmt/mod.rs:1078:17
   5:     0x7fe080bb7692 - std::io::Write::write_fmt::h8580846154bcb66a
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/io/mod.rs:1519:15
   6:     0x7fe080bc93a5 - std::sys_common::backtrace::_print::h7ee55fed88d107a3
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fe080bc93a5 - std::sys_common::backtrace::print::h54a7d3e52a524177
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fe080bc93a5 - std::panicking::default_hook::{{closure}}::h60921e857bf55a40
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/panicking.rs:208:50
   9:     0x7fe080bc8efa - std::panicking::default_hook::hf0f9afb1017317fc
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/panicking.rs:225:9
  10:     0x7fe081480868 - rustc_driver::report_ice::hff78d76a39ffbb86
  11:     0x7fe080bc9ca6 - std::panicking::rust_panic_with_hook::h8d66bf42b407aaea
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/panicking.rs:595:17
  12:     0x7fe080bc9797 - std::panicking::begin_panic_handler::{{closure}}::hde71edcd925d0c5e
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/panicking.rs:495:13
  13:     0x7fe080bc5b6c - std::sys_common::backtrace::__rust_end_short_backtrace::h8a3c7d6cea578919
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/sys_common/backtrace.rs:141:18
  14:     0x7fe080bc9729 - rust_begin_unwind
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/panicking.rs:493:5
  15:     0x7fe080c35931 - core::panicking::panic_fmt::h20225113c4a2f8fd
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/core/src/panicking.rs:92:14
  16:     0x7fe080c3587d - core::panicking::panic::h35b77276aa4b0c12
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/core/src/panicking.rs:50:5
  17:     0x7fe08477da6b - rustc_span::SourceFile::lookup_file_pos::hca092a67fb0031d9
  18:     0x7fe082d7e282 - <rustc_mir::transform::coverage::InstrumentCoverage as rustc_mir::transform::MirPass>::run_pass::h63163fe939ee6941
  19:     0x7fe082ce0f4d - rustc_mir::transform::run_passes::hde85252c218e451d
  20:     0x7fe082ce1cea - rustc_mir::transform::mir_promoted::hbf030df47a64ef91
  21:     0x7fe082d37ec8 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_promoted>::compute::hd22b0a9058b7bb65
  22:     0x7fe082b5464b - rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}::hd68e6cb746e49f3c
  23:     0x7fe08293c559 - rustc_query_system::query::plumbing::get_query_impl::h20d9617555a285f4
  24:     0x7fe082d6c6d6 - rustc_mir::borrow_check::mir_borrowck::h8fcfde9d3bc3965f
  25:     0x7fe082d3f477 - core::ops::function::FnOnce::call_once::hda8d7f4d3e0163df
  26:     0x7fe0816c5fae - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute::h6c5455464ebb9737
  27:     0x7fe08167f530 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h5c4a029b2aaaad43
  28:     0x7fe0816c725b - rustc_data_structures::stack::ensure_sufficient_stack::h0ec271e889f1b221
  29:     0x7fe081662447 - rustc_query_system::query::plumbing::get_query_impl::h780e1793a19b633f
  30:     0x7fe081672dd9 - rustc_query_system::query::plumbing::ensure_query_impl::hac52838f4bbe8a0d
  31:     0x7fe08169d649 - rustc_session::utils::<impl rustc_session::session::Session>::time::h1b0e95cbe477ae94
  32:     0x7fe0816e2712 - rustc_interface::passes::analysis::h34f9b972d8af6f3d
  33:     0x7fe0814eb6eb - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute::h95ff81a8a73eaff3
  34:     0x7fe0814c7616 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h05a5aa3f40ff3b2d
  35:     0x7fe0814dc736 - rustc_data_structures::stack::ensure_sufficient_stack::h371083fbc17898bc
  36:     0x7fe08146abd5 - rustc_query_system::query::plumbing::get_query_impl::hcc63ca689fa3c69b
  37:     0x7fe0814f07bf - rustc_interface::passes::QueryContext::enter::h2071e0a8631ee0e4
  38:     0x7fe08142949e - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hd899306a06575d0c
  39:     0x7fe0814bc5f7 - rustc_span::with_source_map::ha4e07ff263d0dc1d
  40:     0x7fe08142a68b - rustc_interface::interface::create_compiler_and_run::h1d6d732867d1f489
  41:     0x7fe0814d6b60 - scoped_tls::ScopedKey<T>::set::h39c0aa543118d3f3
  42:     0x7fe0814dd346 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1e5aa72fb9cd6d86
  43:     0x7fe08143207a - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc793837e985b77ce
  44:     0x7fe080bd965a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hea1090dbdcecbf5a
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/alloc/src/boxed.rs:1328:9
  45:     0x7fe080bd965a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8d5723d3912bd325
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/alloc/src/boxed.rs:1328:9
  46:     0x7fe080bd965a - std::sys::unix::thread::Thread::new::thread_start::hc17a425ca2995724
                               at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/sys/unix/thread.rs:71:17
  47:     0x7fe08091314a - start_thread
  48:     0x7fe080238f23 - __GI___clone
  49:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
