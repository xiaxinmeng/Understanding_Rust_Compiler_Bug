
error: internal compiler error: src/librustc/hir/def.rs:385: attempted .def_id() on invalid res: Err

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:188
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:205
  10: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa/src/liballoc/boxed.rs:956
  11: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa/src/libproc_macro/bridge/client.rs:305
  12: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
  13: std::panicking::begin_panic
  14: rustc_errors::HandlerInner::bug
  15: rustc_errors::Handler::bug
  16: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  17: rustc::ty::context::tls::with_opt::{{closure}}
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: rustc::hir::def::Res<Id>::def_id::{{closure}}
  22: rustdoc::clean::register_res
  23: <syntax_pos::source_map::Spanned<rustc::hir::VisibilityKind> as rustdoc::clean::Clean<rustdoc::clean::Visibility>>::clean
  24: <rustc::hir::StructField as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  25: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  26: <rustdoc::doctree::Struct as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  27: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  28: rustdoc::clean::krate
  29: rustc::ty::context::tls::enter_global
  30: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  31: rustc_interface::passes::create_global_ctxt::{{closure}}
  32: rustc_interface::passes::BoxedGlobalCtxt::enter
  33: rustc_interface::interface::run_compiler_in_existing_thread_pool
  34: std::panicking::try::do_call
  35: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:81
  36: rustc_driver::catch_fatal_errors
  37: rustdoc::main_options
  38: std::thread::local::LocalKey<T>::with
  39: scoped_tls::ScopedKey<T>::set
  40: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: aborting due to previous error

error: Could not document `tokio-io`.
