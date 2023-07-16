
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:18:85
stack backtrace:
   0: rust_begin_unwind
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:107:14
   2: core::panicking::panic
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:48:5
   3: <rustc_metadata::creader::CStore as rustc_session::cstore::CrateStore>::def_path_hash_to_def_id
   4: <rustc_middle::ty::context::TyCtxt>::def_path_hash_to_def_id
   5: <rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id
   6: rustc_query_impl::query_callbacks::adt_def::force_from_dep_node
   7: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
   8: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   9: rustc_query_system::query::plumbing::ensure_must_run::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
  10: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_match
  11: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_interface::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  12: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#1}>
  13: rustc_interface::passes::analysis
  14: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  15: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  16: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  18: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  19: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  20: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  21: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
