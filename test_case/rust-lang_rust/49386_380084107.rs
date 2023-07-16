
RUST_BACKTRACE=1 cargo +local doc
 Documenting zmq-sys v0.9.0 (file:///Users/imperio/rust/rust-zmq/zmq-sys)
thread 'rustc' panicked at 'slice index starts at 729355 but ends at 729354', libcore/slice/mod.rs:791:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic_fmt
   6: rust_begin_unwind
   7: core::panicking::panic_fmt
   8: core::slice::slice_index_order_fail
   9: <rustc_metadata::decoder::DecodeContext<'a, 'tcx> as serialize::serialize::SpecializedDecoder<rustc_metadata::schema::LazySeq<T>>>::specialized_decode
  10: <rustc_metadata::schema::CrateRoot as serialize::serialize::Decodable>::decode::{{closure}}
  11: rustc_metadata::decoder::<impl rustc_metadata::schema::Lazy<T>>::decode
  12: rustc_metadata::decoder::<impl rustc_metadata::cstore::MetadataBlob>::get_root
  13: rustc_metadata::locator::Context::extract_one
  14: rustc_metadata::locator::Context::maybe_load_library_crate
  15: rustc_metadata::creader::CrateLoader::load
  16: rustc_metadata::creader::CrateLoader::resolve_crate
  17: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  18: rustc_metadata::creader::CrateLoader::resolve_crate
  19: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  20: rustc_metadata::creader::CrateLoader::resolve_crate
  21: <rustc_metadata::creader::CrateLoader<'a> as rustc::middle::cstore::CrateLoader>::process_item
  22: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::build_reduced_graph_for_item
  23: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item
  24: syntax::visit::walk_item
  25: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item
  26: syntax::ext::expand::Expansion::visit_with
  27: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::visit_expansion
  28: syntax::ext::expand::MacroExpander::collect_invocations
  29: syntax::ext::expand::MacroExpander::expand
  30: syntax::ext::expand::MacroExpander::expand_crate
  31: rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}
  32: rustc::util::common::time
  33: rustc_driver::driver::phase_2_configure_and_expand_inner
  34: rustdoc::core::run_core
  35: <scoped_tls::ScopedKey<T>>::set

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-dev running on x86_64-apple-darwin

error: Could not document `zmq-sys`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name zmq_sys src/lib.rs -o /Users/imperio/rust/rust-zmq/zmq-sys/target/doc -L dependency=/Users/imperio/rust/rust-zmq/zmq-sys/target/debug/deps --extern libc=/Users/imperio/rust/rust-zmq/zmq-sys/target/debug/deps/liblibc-a6494ad5c7a8efaa.rmeta` (exit code: 101)
