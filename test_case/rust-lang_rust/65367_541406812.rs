
error: internal compiler error: src/librustc/hir/def.rs:339: attempted .def_id() on invalid res: Err

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:644:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
   7: std::panicking::begin_panic
   8: rustc_errors::Handler::bug
   9: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  10: rustc::ty::context::tls::with_opt::{{closure}}
  11: rustc::ty::context::tls::with_context_opt
  12: rustc::ty::context::tls::with_opt
  13: rustc::util::bug::opt_span_bug_fmt
  14: rustc::util::bug::bug_fmt
  15: rustc::hir::def::Res<Id>::def_id::{{closure}}
  16: rustdoc::clean::register_res
  17: <syntax::source_map::Spanned<rustc::hir::VisibilityKind> as rustdoc::clean::Clean<core::option::Option<rustdoc::clean::Visibility>>>::clean
  18: <rustc::hir::ImplItem as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  19: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  20: <rustdoc::doctree::Impl as rustdoc::clean::Clean<alloc::vec::Vec<rustdoc::clean::Item>>>::clean
  21: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
  22: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  23: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  24: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  25: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  26: <rustc::hir::Crate as rustdoc::clean::Clean<rustdoc::clean::Crate>>::clean
  27: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  28: rustc_interface::passes::create_global_ctxt::{{closure}}
  29: rustc_interface::interface::run_compiler_in_existing_thread_pool
  30: rustdoc::core::run_core
  31: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  32: std::panicking::try::do_call
  33: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  34: rustc_driver::report_ices_to_stderr_if_any
  35: rustdoc::main_options
  36: std::thread::local::LocalKey<T>::with
  37: scoped_tls::ScopedKey<T>::set
  38: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: aborting due to previous error
