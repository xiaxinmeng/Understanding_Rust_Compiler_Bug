
thread 'rustc' panicked at 'non-eager expansion without a parent scope', src/libcore/option.rs:1190:5
stack backtrace:
  <snip>
  15: core::option::expect_failed
             at src/libcore/option.rs:1190
  16: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver>::resolve_macro_invocation
  17: syntax::ext::expand::MacroExpander::fully_expand_fragment
  18: syntax::ext::expand::MacroExpander::expand_crate
  19: rustc_interface::passes::configure_and_expand_inner::{{closure}}
  20: rustc::util::common::time
  21: rustc_interface::passes::configure_and_expand_inner
  22: rustc_interface::passes::configure_and_expand::{{closure}}
  23: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  24: rustc_interface::passes::configure_and_expand
  25: rustc_interface::queries::Query<T>::compute
  26: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::expansion
  27: rustc_interface::interface::run_compiler_in_existing_thread_pool
  28: std::thread::local::LocalKey<T>::with
  29: scoped_tls::ScopedKey<T>::set
  30: syntax::with_globals
