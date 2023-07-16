
thread 'rustc' panicked at 'Failed to get crate data for crate18', compiler/rustc_metadata/src/creader.rs:136:32
stack backtrace:
   0: rust_begin_unwind
             at ./library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at ./library/std/src/panicking.rs:435:5
   2: <rustc_metadata::creader::CStore>::get_crate_data::{closure#0}
             at ./compiler/rustc_metadata/src/creader.rs:136:32
   3: <core::option::Option<&alloc::rc::Rc<rustc_metadata::rmeta::decoder::CrateMetadata>>>::unwrap_or_else::<<rustc_metadata::creader::CStore>::get_crate_data::{closure#0}>
             at ./library/core/src/option.rs:427:21
   4: <rustc_metadata::creader::CStore>::get_crate_data
             at ./compiler/rustc_metadata/src/creader.rs:134:21
   5: <rustc_metadata::creader::CrateLoader>::verify_no_stable_crate_id_hash_conflicts
             at ./compiler/rustc_metadata/src/creader.rs:333:31
   6: <rustc_metadata::creader::CrateLoader>::register_crate
             at ./compiler/rustc_metadata/src/creader.rs:361:9
   7: <rustc_metadata::creader::CrateLoader>::maybe_resolve_crate
             at ./compiler/rustc_metadata/src/creader.rs:547:17
   8: <rustc_metadata::creader::CrateLoader>::resolve_crate
             at ./compiler/rustc_metadata/src/creader.rs:484:9
   9: <rustc_metadata::creader::CrateLoader>::process_path_extern
             at ./compiler/rustc_metadata/src/creader.rs:988:20
  10: <rustc_resolve::Resolver>::extern_prelude_get::{closure#0}
             at ./compiler/rustc_resolve/src/lib.rs:3229:21
  11: <core::option::Option<rustc_resolve::ExternPreludeEntry>>::and_then::<&rustc_resolve::NameBinding, <rustc_resolve::Resolver>::extern_prelude_get::{closure#0}>
             at ./library/core/src/option.rs:724:24
  12: <rustc_resolve::Resolver>::extern_prelude_get
             at ./compiler/rustc_resolve/src/lib.rs:3221:9
  13: <rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope::{closure#0}
             at ./compiler/rustc_resolve/src/macros.rs:829:51
  14: <rustc_resolve::Resolver>::visit_scopes::<core::result::Result<&rustc_resolve::NameBinding, rustc_resolve::Determinacy>, <rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope::{closure#0}>
             at ./compiler/rustc_resolve/src/lib.rs:1733:50
  15: <rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope
             at ./compiler/rustc_resolve/src/macros.rs:680:28
  16: <rustc_resolve::Resolver>::resolve_ident_in_module_unadjusted_ext
  17: <rustc_resolve::Resolver>::resolve_ident_in_module_ext
             at ./compiler/rustc_resolve/src/lib.rs:2079:9
  18: <rustc_resolve::Resolver>::resolve_ident_in_module
             at ./compiler/rustc_resolve/src/lib.rs:2049:9
  19: <rustc_resolve::imports::ImportResolver>::finalize_import::{closure#0}
             at ./compiler/rustc_resolve/src/imports.rs:1021:31
  20: <rustc_resolve::Resolver>::per_ns::<<rustc_resolve::imports::ImportResolver>::finalize_import::{closure#0}>
             at ./compiler/rustc_resolve/src/lib.rs:1421:9
  21: <rustc_resolve::imports::ImportResolver>::finalize_import
             at ./compiler/rustc_resolve/src/imports.rs:1015:9
  22: <rustc_resolve::imports::ImportResolver>::finalize_imports
             at ./compiler/rustc_resolve/src/imports.rs:678:32
  23: <rustc_resolve::Resolver>::resolve_crate::{closure#0}::{closure#0}
             at ./compiler/rustc_resolve/src/lib.rs:1442:54
  24: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), <rustc_resolve::Resolver>::resolve_crate::{closure#0}::{closure#0}>
             at ./compiler/rustc_data_structures/src/profiling.rs:573:9
  25: <rustc_session::session::Session>::time::<(), <rustc_resolve::Resolver>::resolve_crate::{closure#0}::{closure#0}>
             at ./compiler/rustc_session/src/utils.rs:10:9
  26: <rustc_resolve::Resolver>::resolve_crate::{closure#0}
             at ./compiler/rustc_resolve/src/lib.rs:1442:13
  27: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), <rustc_resolve::Resolver>::resolve_crate::{closure#0}>
             at ./compiler/rustc_data_structures/src/profiling.rs:573:9
  28: <rustc_session::session::Session>::time::<(), <rustc_resolve::Resolver>::resolve_crate::{closure#0}>
             at ./compiler/rustc_session/src/utils.rs:10:9
  29: <rustc_resolve::Resolver>::resolve_crate
             at ./compiler/rustc_resolve/src/lib.rs:1441:9
  30: rustc_interface::passes::configure_and_expand_inner
             at ./compiler/rustc_interface/src/passes.rs:412:5
  31: rustc_interface::passes::configure_and_expand::{closure#0}
             at ./compiler/rustc_interface/src/passes.rs:120:19
  32: <core::pin::Pin<alloc::boxed::Box<dyn core::ops::generator::Generator<rustc_data_structures::box_region::Action, Return = rustc_middle::ty::ResolverOutputs, Yield = rustc_data_structures::box_region::YieldType<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorReported>, for<'a, 'b> fn((&'a mut rustc_resolve::Resolver<'b>,))>>>> as core::ops::generator::Generator<rustc_data_structures::box_region::Action>>::resume
             at ./library/alloc/src/boxed.rs:1668:9
  33: <rustc_data_structures::box_region::PinnedGenerator<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorReported>, for<'a, 'b> fn((&'a mut rustc_resolve::Resolver<'b>,)), rustc_middle::ty::ResolverOutputs>>::new::<rustc_interface::passes::configure_and_expand::{closure#0}>
             at ./compiler/rustc_data_structures/src/box_region.rs:44:26
  34: <rustc_interface::passes::BoxedResolver>::new::<rustc_interface::passes::configure_and_expand::{closure#0}>
             at ./compiler/rustc_data_structures/src/box_region.rs:101:41
  35: rustc_interface::passes::configure_and_expand
             at ./compiler/rustc_interface/src/passes.rs:116:30
  36: <rustc_interface::queries::Queries>::expansion::{closure#0}
             at ./compiler/rustc_interface/src/queries.rs:183:13
  37: <rustc_interface::queries::Query<(rustc_ast::ast::Crate, rustc_data_structures::steal::Steal<alloc::rc::Rc<core::cell::RefCell<rustc_interface::passes::BoxedResolver>>>, alloc::rc::Rc<rustc_lint::context::LintStore>)>>::compute::<<rustc_interface::queries::Queries>::expansion::{closure#0}>
             at ./compiler/rustc_interface/src/queries.rs:40:28
  38: <rustc_interface::queries::Queries>::expansion
             at ./compiler/rustc_interface/src/queries.rs:179:9
  39: rustc_driver::run_compiler::{closure#3}::{closure#2}
             at ./compiler/rustc_driver/src/lib.rs:386:13
  40: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#3}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
             at ./compiler/rustc_interface/src/queries.rs:422:19
  41: rustc_driver::run_compiler::{closure#3}
             at ./compiler/rustc_driver/src/lib.rs:334:22
  42: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}
             at ./compiler/rustc_interface/src/interface.rs:208:13
  43: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}>
             at ./compiler/rustc_span/src/lib.rs:789:5
  44: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>
             at ./compiler/rustc_interface/src/interface.rs:202:5
  45: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}
             at ./compiler/rustc_interface/src/interface.rs:224:12
  46: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}
             at ./compiler/rustc_interface/src/util.rs:155:13
  47: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/mw/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  48: rustc_span::with_session_globals::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}>
             at ./compiler/rustc_span/src/lib.rs:106:5
  49: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at ./compiler/rustc_interface/src/util.rs:153:9
  50: rustc_interface::util::scoped_thread::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at ./compiler/rustc_interface/src/util.rs:128:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
