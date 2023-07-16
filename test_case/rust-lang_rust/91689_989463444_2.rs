
  0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: <rustc_metadata::rmeta::decoder::DecodeContext as rustc_serialize::serialize::Decoder>::read_str
   4: <rustc_metadata::rmeta::Lazy<alloc::string::String, ()>>::decode::<&rustc_metadata::rmeta::decoder::MetadataBlob>
   5: <rustc_metadata::locator::CrateLocator>::extract_one
   6: <rustc_metadata::locator::CrateLocator>::extract_lib
   7: <rustc_metadata::locator::CrateLocator>::find_library_crate
   8: <rustc_metadata::locator::CrateLocator>::maybe_load_library_crate
   9: <rustc_metadata::creader::CrateLoader>::load
  10: <rustc_metadata::creader::CrateLoader>::maybe_resolve_crate
  11: <rustc_metadata::creader::CrateLoader>::maybe_resolve_crate
  12: <rustc_metadata::creader::CrateLoader>::maybe_process_path_extern
  13: <rustc_resolve::Resolver>::extern_prelude_get
  14: <rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope
  15: <rustc_resolve::Resolver>::resolve_path_with_ribs::{closure#1}
  16: <rustc_resolve::Resolver>::resolve_path_with_ribs
  17: <rustc_resolve::imports::ImportResolver>::resolve_imports
  18: <rustc_resolve::Resolver as rustc_expand::base::ResolverExpand>::resolve_imports
  19: <rustc_expand::expand::MacroExpander>::fully_expand_fragment
  20: <rustc_expand::expand::MacroExpander>::expand_crate
  21: <rustc_session::session::Session>::time::<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorReported>, rustc_interface::passes::configure_and_expand::{closure#1}>
  22: rustc_interface::passes::configure_and_expand
  23: <rustc_interface::queries::Queries>::expansion
  24: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  25: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  26: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
