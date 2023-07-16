
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/decoder.rs:400:22
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e4a60327063e82413eed50a10df3b7d19b77bda0/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/e4a60327063e82413eed50a10df3b7d19b77bda0/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/e4a60327063e82413eed50a10df3b7d19b77bda0/library/core/src/panicking.rs:50:5
   3: rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::hygiene::ExpnId>::decode
   4: rustc_span::hygiene::_DERIVE_rustc_serialize_Decodable_D_FOR_SyntaxContextData::<impl rustc_serialize::serialize::Decodable<__D> for rustc_span::hygiene::SyntaxContextData>::decode
   5: rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::hygiene::SyntaxContext>::decode
   6: rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::span_encoding::Span>::decode
   7: rustc_span::hygiene::_DERIVE_rustc_serialize_Decodable_D_FOR_ExpnData::<impl rustc_serialize::serialize::Decodable<__D> for rustc_span::hygiene::ExpnData>::decode
   8: rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::hygiene::ExpnId>::decode
   9: rustc_span::hygiene::_DERIVE_rustc_serialize_Decodable_D_FOR_SyntaxContextData::<impl rustc_serialize::serialize::Decodable<__D> for rustc_span::hygiene::SyntaxContextData>::decode
  10: rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::hygiene::SyntaxContext>::decode
  11: rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::span_encoding::Span>::decode
  12: rustc_middle::mir::_DERIVE_rustc_serialize_Decodable_D_FOR_Statement::<impl rustc_serialize::serialize::Decodable<__D> for rustc_middle::mir::Statement>::decode
  13: rustc_serialize::serialize::Decoder::read_seq
  14: rustc_middle::mir::_DERIVE_rustc_serialize_Decodable_D_FOR_BasicBlockData::<impl rustc_serialize::serialize::Decodable<__D> for rustc_middle::mir::BasicBlockData>::decode
  15: rustc_serialize::serialize::Decoder::read_seq
  16: rustc_middle::mir::_DERIVE_rustc_serialize_Decodable_D_FOR_Body::<impl rustc_serialize::serialize::Decodable<__D> for rustc_middle::mir::Body>::decode
  17: rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::get_optimized_mir
  18: rustc_metadata::rmeta::decoder::cstore_impl::provide_extern::optimized_mir
  19: rustc_query_system::query::plumbing::get_query_impl
  20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  21: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
  22: rustc_mir::transform::inline::cycle::mir_inliner_callees
  23: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::mir_inliner_callees>::compute
  24: rustc_data_structures::stack::ensure_sufficient_stack
  25: rustc_query_system::query::plumbing::get_query_impl
  26: rustc_query_system::query::plumbing::get_query
  27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_inliner_callees
  28: rustc_mir::transform::inline::cycle::mir_callgraph_reachable::process
  29: rustc_data_structures::stack::ensure_sufficient_stack
  30: rustc_mir::transform::inline::cycle::mir_callgraph_reachable::process
  31: rustc_data_structures::stack::ensure_sufficient_stack
  32: rustc_mir::transform::inline::cycle::mir_callgraph_reachable::process
  33: rustc_data_structures::stack::ensure_sufficient_stack
  34: rustc_mir::transform::inline::cycle::mir_callgraph_reachable::process
  35: rustc_mir::transform::inline::cycle::mir_callgraph_reachable
  36: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::mir_callgraph_reachable>::compute
  37: rustc_data_structures::stack::ensure_sufficient_stack
  38: rustc_query_system::query::plumbing::get_query_impl
  39: rustc_query_system::query::plumbing::get_query
  40: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_callgraph_reachable
  41: rustc_mir::transform::inline::Inliner::try_inlining
  42: rustc_mir::transform::inline::Inliner::process_blocks
  43: <rustc_mir::transform::inline::Inline as rustc_mir::transform::MirPass>::run_pass
  44: rustc_mir::transform::run_passes
  45: rustc_mir::transform::optimized_mir
  46: rustc_query_system::query::plumbing::get_query_impl
  47: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  48: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
  49: rustc_mir::transform::inline::Inliner::try_inlining
  50: rustc_mir::transform::inline::Inliner::process_blocks
  51: <rustc_mir::transform::inline::Inline as rustc_mir::transform::MirPass>::run_pass
  52: rustc_mir::transform::run_passes
  53: rustc_mir::transform::optimized_mir
  54: rustc_query_system::query::plumbing::get_query_impl
  55: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  56: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
  57: rustc_mir::transform::inline::Inliner::try_inlining
  58: rustc_mir::transform::inline::Inliner::process_blocks
  59: <rustc_mir::transform::inline::Inline as rustc_mir::transform::MirPass>::run_pass
  60: rustc_mir::transform::run_passes
  61: rustc_mir::transform::optimized_mir
  62: rustc_query_system::query::plumbing::get_query_impl
  63: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  64: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
  65: rustc_metadata::rmeta::encoder::encode_metadata_impl
  66: rustc_data_structures::sync::join
  67: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
  68: rustc_middle::ty::context::TyCtxt::encode_metadata
  69: rustc_interface::passes::QueryContext::enter
  70: rustc_interface::queries::Queries::ongoing_codegen
  71: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  72: rustc_span::with_source_map
  73: rustc_interface::interface::create_compiler_and_run
  74: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

error: Unrecognized option: 'clif'

error: could not compile `gimli`

To learn more, run the command again with --verbose.
