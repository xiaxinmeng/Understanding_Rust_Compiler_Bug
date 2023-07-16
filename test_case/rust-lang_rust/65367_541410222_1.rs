
error: internal compiler error: src/librustc/hir/def.rs:345: attempted .def_id() on invalid res: Err

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:917:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:189
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:206
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:469
  11: std::panicking::begin_panic
  12: rustc_errors::HandlerInner::bug
  13: rustc_errors::Handler::bug
  14: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  15: rustc::ty::context::tls::with_opt::{{closure}}
  16: rustc::ty::context::tls::with_context_opt
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: rustc::hir::def::Res<Id>::def_id::{{closure}}
  21: rustdoc::clean::register_res
  22: <syntax::source_map::Spanned<rustc::hir::VisibilityKind> as rustdoc::clean::Clean<rustdoc::clean::Visibility>>::clean
  23: <rustc::hir::ImplItem as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  24: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  25: <rustdoc::doctree::Impl as rustdoc::clean::Clean<alloc::vec::Vec<rustdoc::clean::Item>>>::clean
  26: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
  27: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  28: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  29: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  30: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  31: rustdoc::clean::krate
  32: rustc::ty::context::tls::enter_global
  33: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  34: rustc_interface::passes::create_global_ctxt::{{closure}}
  35: rustc_interface::passes::BoxedGlobalCtxt::enter
  36: rustc_interface::interface::run_compiler_in_existing_thread_pool
  37: rustdoc::core::run_core
  38: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  39: std::panicking::try::do_call
  40: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  41: rustc_driver::catch_fatal_errors
  42: rustdoc::main_options
  43: std::thread::local::LocalKey<T>::with
  44: scoped_tls::ScopedKey<T>::set
  45: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: aborting due to previous error

error: Could not document `lexical-core`.
