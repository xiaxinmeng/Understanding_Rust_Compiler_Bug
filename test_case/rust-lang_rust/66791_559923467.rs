
thread 'rustc' panicked at 'assertion failed: self.try_set(value).is_none()', src/librustc_data_structures/sync.rs:492:9
stack backtrace:
  [...]
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:475
  12: std::panicking::begin_panic
  13: rustc::session::Session::init_features
  14: rustc_interface::passes::register_plugins
  15: rustc_interface::queries::Queries::register_plugins
  16: rustc_interface::queries::Queries::expansion
  17: rustc_interface::queries::Queries::prepare_outputs
  18: rustc_interface::queries::Queries::global_ctxt
  19: <miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis::{{closure}}
             at src/bin/miri.rs:37
  20: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at /rustc/8f1bbd69e13c9e04a4c2b75612bc0c31af972439/src/librustc_interface/queries.rs:335
  21: <miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis
             at src/bin/miri.rs:36
