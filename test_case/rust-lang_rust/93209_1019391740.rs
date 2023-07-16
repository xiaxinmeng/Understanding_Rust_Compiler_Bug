
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:755:13: Broken MIR: generator contains type http::uri::Parts in MIR, but typeck only knows about {ResumeTy, &mut client::grpc::Grpc<T>, C, http::Request<UnsyncBoxBody<bytes::Bytes, Status>>, client::grpc::Grpc<T>, <T as GrpcService<UnsyncBoxBody<bytes::Bytes, Status>>>::Future, ()} and [&mut client::grpc::Grpc<T>, request::Request<S>, http::uri::PathAndQuery, C]
   --> /Users/tison/.cargo/registry/src/github.com-1ecc6299db9ec823/tonic-0.6.2/src/client/grpc.rs:234:5
    |
234 | /     {
235 | |         let mut parts = Parts::default();
236 | |         parts.path_and_query = Some(path);
237 | |
...   |
324 | |         Ok(Response::from_http(response))
325 | |     }
    | |_____^

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/17d29dcdce9b9e838635eb0adefd9b8b1588410b/compiler/rustc_errors/src/lib.rs:1115:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::span_bug::<rustc_span::span_encoding::Span>
   3: <rustc_errors::Handler>::span_bug::<rustc_span::span_encoding::Span>
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::span_bug_fmt::<rustc_span::span_encoding::Span>
   7: <rustc_mir_transform::generator::StateTransform as rustc_middle::mir::MirPass>::run_pass
   8: rustc_mir_transform::pass_manager::run_passes
   9: rustc_mir_transform::optimized_mir
  10: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  11: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
  12: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached
  13: rustc_middle::ty::layout::layout_of
  14: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  16: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_middle::ty::layout::LayoutOf>::layout_of
  17: core::iter::adapters::process_results::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::FieldDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>, rustc_middle::ty::layout::LayoutError, <core::result::Result<alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>, rustc_middle::ty::layout::LayoutError> as core::iter::traits::collect::FromIterator<core::result::Result<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>, rustc_middle::ty::layout::LayoutError>>>::from_iter<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::FieldDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>>::{closure#0}, alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>>
  18: core::iter::adapters::process_results::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::VariantDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>, rustc_middle::ty::layout::LayoutError, <core::result::Result<rustc_index::vec::IndexVec<rustc_target::abi::VariantIdx, alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>>, rustc_middle::ty::layout::LayoutError> as core::iter::traits::collect::FromIterator<core::result::Result<alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>, rustc_middle::ty::layout::LayoutError>>>::from_iter<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::VariantDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>>::{closure#0}, rustc_index::vec::IndexVec<rustc_target::abi::VariantIdx, alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>>>
  19: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached
  20: rustc_middle::ty::layout::layout_of
  21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  23: rustc_middle::ty::layout::layout_of
  24: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  26: <rustc_mir_transform::const_prop::ConstProp as rustc_middle::mir::MirPass>::run_pass
  27: rustc_mir_transform::pass_manager::run_passes
  28: rustc_mir_transform::optimized_mir
  29: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  30: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
  31: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root
  32: rustc_metadata::rmeta::encoder::encode_metadata_impl
  33: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata::rmeta::encoder::EncodedMetadata, ()>
  34: rustc_metadata::rmeta::encoder::encode_metadata
  35: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
  36: <rustc_interface::queries::Queries>::ongoing_codegen
  37: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  38: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  39: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (17d29dcdc 2022-01-21) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `client::grpc::<impl at /Users/tison/.cargo/registry/src/github.com-1ecc6299db9ec823/tonic-0.6.2/src/client/grpc.rs:42:1: 326:2>::streaming::{closure#0}`
#1 [layout_of] computing layout of `[static generator@/Users/tison/.cargo/registry/src/github.com-1ecc6299db9ec823/tonic-0.6.2/src/client/grpc.rs:234:5: 325:6]`
#2 [layout_of] computing layout of `core::future::from_generator::GenFuture<[static generator@/Users/tison/.cargo/registry/src/github.com-1ecc6299db9ec823/tonic-0.6.2/src/client/grpc.rs:234:5: 325:6]>`
#3 [layout_of] computing layout of `impl core::future::future::Future<Output = [async output]>`
#4 [optimized_mir] optimizing MIR for `client::grpc::<impl at /Users/tison/.cargo/registry/src/github.com-1ecc6299db9ec823/tonic-0.6.2/src/client/grpc.rs:42:1: 326:2>::streaming`
end of query stack
error: could not compile `tonic`
