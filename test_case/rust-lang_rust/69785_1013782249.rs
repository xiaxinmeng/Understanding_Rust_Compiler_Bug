
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:966:9: no MIR available for DefId(20:1344 ~ cranelift_codegen[eeed]::machinst::buffer::{impl#3}::get_srclocs_sorted)

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::bug
   3: <rustc_errors::Handler>::bug
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_monomorphize::collector::collect_neighbours
   8: rustc_monomorphize::collector::collect_items_rec
   9: rustc_monomorphize::collector::collect_items_rec
  10: rustc_monomorphize::collector::collect_items_rec
  11: rustc_monomorphize::collector::collect_items_rec
  12: rustc_monomorphize::collector::collect_items_rec
  13: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
  14: rustc_monomorphize::collector::collect_crate_mono_items
  15: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  16: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  17: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  18: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  19: rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local
  20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::CrateNum, &[(rustc_middle::middle::exported_symbols::ExportedSymbol, rustc_middle::middle::exported_symbols::SymbolExportLevel)]>>
  21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::exported_symbols, rustc_query_impl::plumbing::QueryCtxt>
  22: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root
  23: rustc_metadata::rmeta::encoder::encode_metadata_impl
  24: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata::rmeta::encoder::EncodedMetadata, ()>
  25: rustc_metadata::rmeta::encoder::encode_metadata
  26: <rustc_interface::queries::Queries>::ongoing_codegen
  27: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
   Compiling kvdb-memorydb v0.10.0
  28: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  29: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  30: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (22e491ac7 2022-01-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] exported_symbols
end of query stack
error: could not compile `wasmtime-cranelift`
