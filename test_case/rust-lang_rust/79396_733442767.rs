
❯ rustdoc +stage1 --document-private-items src/test/rustdoc/non-exported-macros.rs
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:897:18: hir::map::Map::span_with_body: id not in map: HirId { owner: DefId(0:5 ~ non_exported_macros[8787]::foo_macro), local_id: 0 }

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::bug
   2: rustc_errors::Handler::bug
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
   5: rustc_middle::ty::context::tls::with_opt
   6: rustc_middle::util::bug::opt_span_bug_fmt
   7: rustc_middle::util::bug::bug_fmt
   8: rustc_middle::hir::map::Map::span_with_body
   9: rustdoc::clean::types::Item::from_def_id_and_parts::{{closure}}
             at ./src/librustdoc/clean/types.rs:141:17
  10: core::option::Option<T>::map_or_else
             at ./library/core/src/option.rs:503:24
  11: rustdoc::clean::types::Item::from_def_id_and_parts
             at ./src/librustdoc/clean/types.rs:137:22
  12: <rustdoc::doctree::Macro as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean
             at ./src/librustdoc/clean/mod.rs:2261:9
  13: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean::{{closure}}
             at ./src/librustdoc/clean/mod.rs:236:49
  14: core::iter::adapters::map::map_fold::{{closure}}
             at ./library/core/src/iter/adapters/map.rs:80:28
  15: core::iter::traits::iterator::Iterator::fold
             at ./library/core/src/iter/traits/iterator.rs:2023:21
  16: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at ./library/core/src/iter/adapters/map.rs:120:9
  17: core::iter::traits::iterator::Iterator::for_each
             at ./library/core/src/iter/traits/iterator.rs:678:9
  18: <alloc::vec::Vec<T,A> as alloc::vec::SpecExtend<T,I>>::spec_extend
             at ./library/alloc/src/vec.rs:2565:17
  19: <alloc::vec::Vec<T,A> as core::iter::traits::collect::Extend<T>>::extend
             at ./library/alloc/src/vec.rs:2251:9
  20: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean
             at ./src/librustdoc/clean/mod.rs:236:9
  21: rustdoc::clean::utils::krate
             at ./src/librustdoc/clean/utils.rs:44:22
  22: rustdoc::core::run_global_ctxt::{{closure}}
             at ./src/librustdoc/core.rs:548:53
  23: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
  24: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
  25: rustdoc::core::run_global_ctxt
             at ./src/librustdoc/core.rs:548:21
  26: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustdoc/core.rs:458:21
  27: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:725:42
  28: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1739:50
  29: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1723:9
  30: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1739:9
  31: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:725:9
  32: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustdoc/core.rs:457:17
  33: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
  34: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
  35: rustdoc::core::run_core::{{closure}}::{{closure}}
             at ./src/librustdoc/core.rs:456:46
  36: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:415:19
  37: rustdoc::core::run_core::{{closure}}
             at ./src/librustdoc/core.rs:415:9
  38: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:196:13
  39: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:764:5
  40: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:190:5
  41: rustdoc::core::run_core
             at ./src/librustdoc/core.rs:414:5
  42: rustdoc::main_options
             at ./src/librustdoc/lib.rs:531:53
  43: rustdoc::main_args::{{closure}}
             at ./src/librustdoc/lib.rs:464:17
  44: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:152:13
  45: scoped_tls::ScopedKey<T>::set
             at /Users/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  46: rustc_span::with_session_globals
             at ./compiler/rustc_span/src/lib.rs:93:5
  47: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:150:9
  48: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: Unrecognized option: 'document-private-items'

error: aborting due to previous error
