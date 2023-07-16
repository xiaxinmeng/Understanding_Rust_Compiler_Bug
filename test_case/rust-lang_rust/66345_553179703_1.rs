
thread 'rustc' panicked at 'no value for given alloc ID', src/libcore/option.rs:1187:5
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::continue_panic_fmt
   8: rust_begin_unwind
   9: core::panicking::panic_fmt
  10: core::option::expect_failed
  11: rustc::mir::interpret::specialized_encode_alloc_id
  12: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
  13: rustc::ty::context::tls::with_context::{{closure}}
  14: rustc_metadata::rmeta::encoder::encode_metadata
  15: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  16: rustc::ty::context::TyCtxt::encode_metadata
  17: rustc_interface::passes::start_codegen::{{closure}}
  18: rustc_interface::passes::start_codegen
  19: rustc::ty::context::tls::enter_global
  20: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  21: rustc_interface::passes::create_global_ctxt::{{closure}}
  22: rustc_interface::passes::BoxedGlobalCtxt::enter
  23: rustc_interface::queries::Query<T>::compute
  24: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  25: rustc_interface::interface::run_compiler_in_existing_thread_pool
  26: std::thread::local::LocalKey<T>::with
  27: scoped_tls::ScopedKey<T>::set
  28: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (bc0e288ad 2019-11-11) running on x86_64-apple-darwin

note: compiler flags: -Z mir-opt-level=3 -C opt-level=3 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
