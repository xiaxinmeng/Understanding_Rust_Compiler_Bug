
Using nightly
info: using existing install for 'nightly-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.50.0-nightly (0f6f2d681 2020-12-06)

   Compiling openssl-sys v0.9.58
   Compiling native-tls v0.2.4
   Compiling rand_chacha v0.1.1
   Compiling rand_pcg v0.1.2
thread 'rustc' panicked at 'assertion failed: self.start_pos.to_u32() + total_extra_bytes <= bpos.to_u32()', compiler/rustc_span/src/lib.rs:1495:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/core/src/panicking.rs:50:5
   3: rustc_span::SourceFile::lookup_file_pos
   4: <rustc_mir::transform::coverage::InstrumentCoverage as rustc_mir::transform::MirPass>::run_pass
   5: rustc_mir::transform::run_passes
   6: rustc_mir::transform::mir_promoted
   7: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_promoted>::compute
   8: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
   9: rustc_query_system::query::plumbing::get_query_impl
  10: rustc_mir::borrow_check::mir_borrowck
  11: core::ops::function::FnOnce::call_once
  12: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  14: rustc_data_structures::stack::ensure_sufficient_stack
  15: rustc_query_system::query::plumbing::get_query_impl
  16: rustc_query_system::query::plumbing::ensure_query_impl
  17: rustc_session::utils::<impl rustc_session::session::Session>::time
  18: rustc_interface::passes::analysis
  19: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
  21: rustc_data_structures::stack::ensure_sufficient_stack
  22: rustc_query_system::query::plumbing::get_query_impl
  23: rustc_interface::passes::QueryContext::enter
  24: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  25: rustc_span::with_source_map
  26: rustc_interface::interface::create_compiler_and_run
  27: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (0f6f2d681 2020-12-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z instrument-coverage -C embed-bitcode=no -C debuginfo=2 -C codegen-units=1 -C opt-level=0 -C link-dead-code -C overflow-checks=off --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_promoted] processing `err::ERR_PACK`
#1 [mir_borrowck] borrow-checking `err::ERR_PACK`
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `openssl-sys`
