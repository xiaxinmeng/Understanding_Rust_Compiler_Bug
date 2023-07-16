`
Building stage1 tool cargo (x86_64-unknown-linux-gnu)
   Compiling curl-sys v0.4.51+curl-7.80.0
   Compiling libnghttp2-sys v0.1.4+1.41.0
   Compiling url v2.2.2
   Compiling serde_json v1.0.59
   Compiling cargo-platform v0.1.2 (/home/matthias/vcs/github/rust/src/tools/cargo/crates/cargo-platform)
thread 'rustc' panicked at 'assertion failed: sentinel == STR_SENTINEL', /home/matthias/vcs/github/rust/compiler/rustc_serialize/src/opaque.rs:669:9
stack backtrace:
   0: rust_begin_unwind
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
  12: <rustc_metadata::creader::CrateLoader>::resolve_crate
  13: <rustc_metadata::creader::CrateLoader>::process_path_extern
  14: <rustc_resolve::Resolver>::extern_prelude_get
  15: <rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope
  16: <rustc_resolve::Resolver>::resolve_ident_in_lexical_scope
  17: <rustc_resolve::Resolver>::resolve_path_with_ribs::{closure#1}
  18: <rustc_resolve::Resolver>::resolve_path_with_ribs
  19: <rustc_resolve::late::LateResolutionVisitor>::resolve_qpath_anywhere
  20: <rustc_resolve::late::LateResolutionVisitor>::smart_resolve_path_fragment
  21: <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_item
  22: <rustc_resolve::Resolver>::late_resolve_crate
  23: <rustc_session::session::Session>::time::<(), <rustc_resolve::Resolver>::resolve_crate::{closure#0}>
  24: rustc_interface::passes::configure_and_expand
  25: <rustc_interface::queries::Queries>::expansion
  26: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  27: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  28: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z symbol-mangling-version=v0 -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C codegen-units=4 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type rlib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
