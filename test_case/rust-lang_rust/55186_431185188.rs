
thread 'main' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:186:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:479
   6: std::panicking::begin_panic
   7: <scoped_tls::ScopedKey<T>>::with
   8: syntax_pos::<impl syntax_pos::span_encoding::Span>::macro_backtrace
   9: rustc_errors::emitter::EmitterWriter::fix_multispan_in_std_macros
  10: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit
  11: rustc_errors::Handler::emit_db
  12: <rustc_errors::Handler as core::ops::drop::Drop>::drop
  13: core::ptr::drop_in_place
  14: core::ptr::drop_in_place
  15: core::ptr::drop_in_place
  16: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  17: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  18: rustc_driver::run
  19: rustc_driver::main
  20: std::rt::lang_start::{{closure}}
  21: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  22: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  23: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  24: main
  25: __libc_start_main
  26: <unknown>
query stack during panic:
end of query stack
