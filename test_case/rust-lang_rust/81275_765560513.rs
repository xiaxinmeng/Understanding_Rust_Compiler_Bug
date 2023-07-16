
time: 0.342; rss: 696MB	render_html
thread 'rustc' panicked at 'attempt to steal from stolen value', compiler/rustc_interface/src/queries.rs:265:34
stack backtrace:
   0: rust_begin_unwind
             at /home/joshua/rustc3/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /home/joshua/rustc3/library/core/src/panicking.rs:92:14
   2: core::option::expect_failed
             at /home/joshua/rustc3/library/core/src/option.rs:1258:5
   3: core::option::Option<T>::expect
             at /home/joshua/rustc3/library/core/src/option.rs:349:21
   4: rustc_data_structures::steal::Steal<T>::steal
             at /home/joshua/rustc3/compiler/rustc_data_structures/src/steal.rs:45:9
   5: rustc_interface::queries::Queries::global_ctxt::{{closure}}
             at /home/joshua/rustc3/compiler/rustc_interface/src/queries.rs:265:17
   6: rustc_interface::queries::Query<T>::compute
             at /home/joshua/rustc3/compiler/rustc_interface/src/queries.rs:39:28
   7: rustc_interface::queries::Queries::global_ctxt
             at /home/joshua/rustc3/compiler/rustc_interface/src/queries.rs:252:9
   8: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at /home/joshua/rustc3/compiler/rustc_interface/src/queries.rs:420:26
   9: rustdoc::main_options::{{closure}}
             at /home/joshua/rustc3/src/librustdoc/lib.rs:530:9
  10: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at /home/joshua/rustc3/compiler/rustc_interface/src/interface.rs:197:13
  11: rustc_span::with_source_map
             at /home/joshua/rustc3/compiler/rustc_span/src/lib.rs:787:5
  12: rustc_interface::interface::create_compiler_and_run
             at /home/joshua/rustc3/compiler/rustc_interface/src/interface.rs:191:5
  13: rustdoc::main_options
             at /home/joshua/rustc3/src/librustdoc/lib.rs:529:5
  14: rustdoc::main_args::{{closure}}
             at /home/joshua/rustc3/src/librustdoc/lib.rs:454:17
  15: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at /home/joshua/rustc3/compiler/rustc_interface/src/util.rs:152:13
  16: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  17: rustc_span::with_session_globals
             at /home/joshua/rustc3/compiler/rustc_span/src/lib.rs:103:5
  18: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at /home/joshua/rustc3/compiler/rustc_interface/src/util.rs:150:9
  19: rustc_interface::util::scoped_thread::{{closure}}
             at /home/joshua/rustc3/compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

error: Unrecognized option: 'crate-version'

time: 0.076; rss: 840MB	create_global_ctxt
