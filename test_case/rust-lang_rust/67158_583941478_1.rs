bash
error[E0658]: yield syntax is experimental
 --> src/main.rs:2:13
  |
2 |     async { yield print!(":C") };
  |             ^^^^^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/43122
  = help: add `#![feature(generators)]` to the crate attributes to enable

error[E0727]: `async` generators are not yet supported
 --> src/main.rs:2:13
  |
2 |     async { yield print!(":C") };
  |             ^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'src/librustc/hir/map/hir_id_validator.rs:27: 
ItemLocalIds not assigned densely in ::main[0]. Max ItemLocalId = 14, missing IDs = ["[local_id: 1, node:unknown node (hir_id=HirId { owner: DefIndex(3), local_id: 1 })]"]; seens IDs = ["(HirId { owner: DefIndex(3), local_id: 0 } fn main (hir_id=HirId { owner: DefIndex(3), local_id: 0 }))", "(HirId { owner: DefIndex(3), local_id: 13 } block { ::std::future::from_generator(|| { (/*ERROR*/) }); } (hir_id=HirId { owner: DefIndex(3), local_id: 13 }))", "(HirId { owner: DefIndex(3), local_id: 10 } expr ::std::future::from_generator (hir_id=HirId { owner: DefIndex(3), local_id: 10 }))", "(HirId { owner: DefIndex(3), local_id: 7 } path segment std (hir_id=HirId { owner: DefIndex(3), local_id: 7 }))", "(HirId { owner: DefIndex(3), local_id: 4 } expr { (/*ERROR*/) } (hir_id=HirId { owner: DefIndex(3), local_id: 4 }))", "(HirId { owner: DefIndex(3), local_id: 14 } expr { ::std::future::from_generator(|| { (/*ERROR*/) }); } (hir_id=HirId { owner: DefIndex(3), local_id: 14 }))", "(HirId { owner: DefIndex(3), local_id: 11 } expr ::std::future::from_generator(|| { (/*ERROR*/) }) (hir_id=HirId { owner: DefIndex(3), local_id: 11 }))", "(HirId { owner: DefIndex(3), local_id: 8 } path segment future (hir_id=HirId { owner: DefIndex(3), local_id: 8 }))", "(HirId { owner: DefIndex(3), local_id: 5 } expr || { (/*ERROR*/) } (hir_id=HirId { owner: DefIndex(3), local_id: 5 }))", "(HirId { owner: DefIndex(3), local_id: 2 } expr (/*ERROR*/) (hir_id=HirId { owner: DefIndex(3), local_id: 2 }))", "(HirId { owner: DefIndex(3), local_id: 12 } stmt ::std::future::from_generator(|| { (/*ERROR*/) }); (hir_id=HirId { owner: DefIndex(3), local_id: 12 }))", "(HirId { owner: DefIndex(3), local_id: 9 } path segment from_generator (hir_id=HirId { owner: DefIndex(3), local_id: 9 }))", "(HirId { owner: DefIndex(3), local_id: 6 } path segment  (hir_id=HirId { owner: DefIndex(3), local_id: 6 }))", "(HirId { owner: DefIndex(3), local_id: 3 } block { (/*ERROR*/) } (hir_id=HirId { owner: DefIndex(3), local_id: 3 }))"]', src/librustc/util/bug.rs:37:26
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1052
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1428
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  12: std::panicking::begin_panic
  13: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  14: rustc::ty::context::tls::with_opt::{{closure}}
  15: rustc::ty::context::tls::with_opt
  16: rustc::util::bug::opt_span_bug_fmt
  17: rustc::util::bug::bug_fmt
  18: rustc::hir::map::hir_id_validator::check_crate
  19: rustc_session::utils::<impl rustc_session::session::Session>::time
  20: rustc::hir::map::map_crate
  21: rustc_interface::passes::create_global_ctxt
  22: rustc_interface::queries::Queries::global_ctxt
  23: rustc_interface::interface::run_compiler_in_existing_thread_pool
  24: scoped_tls::ScopedKey<T>::set
  25: syntax::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (58b834344 2020-02-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0658, E0727.
For more information about an error, try `rustc --explain E0658`.

