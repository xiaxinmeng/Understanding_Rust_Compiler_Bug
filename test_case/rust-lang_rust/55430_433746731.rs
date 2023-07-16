
Checking hyper v0.12.13
Documenting hyper v0.12.13
thread '<unnamed>' panicked at 'librustc/hir/map/hir_id_validator.rs:31: 
HirIdValidator: The recorded owner of path segment super (id=36924) is ::server[0]::conn[0]::{{?}}[34] instead of ::server[0]::conn[0]::{{?}}[34]::{{?}}[0]
HirIdValidator: Same HirId ::server[0]::conn[0]::{{?}}[34]::{{?}}[0]/2 assigned for nodes path segment super (id=36924) and path segment spawn_all (id=92273)', librustc/util/bug.rs:47:26
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   7: rustc::ty::context::tls::with_opt::{{closure}}
   8: rustc::ty::context::tls::with_context_opt
   9: rustc::ty::context::tls::with_opt
  10: rustc::util::bug::opt_span_bug_fmt
  11: rustc::util::bug::bug_fmt
  12: rustc::hir::map::hir_id_validator::check_crate
  13: rustc::hir::map::map_crate
  14: <scoped_tls::ScopedKey<T>>::set
  15: rustdoc::core::run_core
  16: <scoped_tls::ScopedKey<T>>::set
  17: syntax::with_globals
  18: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  19: std::panicking::try::do_call
  20: __rust_maybe_catch_panic
  21: rustc_driver::monitor
  22: rustdoc::rust_input
  23: rustdoc::main_args
  24: <scoped_tls::ScopedKey<T>>::set
  25: syntax::with_globals
  26: std::panicking::try::do_call
  27: __rust_maybe_catch_panic
  28: <F as alloc::boxed::FnBox<A>>::call_box
  29: std::sys::unix::thread::Thread::new::thread_start
  30: _pthread_body
  31: _pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-nightly (cae6efc37 2018-10-27) running on x86_64-apple-darwin

error: Could not document `hyper`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name hyper /Users/davidlewis/.cargo/registry/src/github.com-1ecc6299db9ec823/hyper-0.12.13/src/lib.rs --cap-lints allow --color never -o '/Users/davidlewis/.../target/doc' --cfg 'feature="__internal_flaky_tests"' --cfg 'feature="default"' --cfg 'feature="futures-cpupool"' --cfg 'feature="net2"' --cfg 'feature="runtime"' --cfg 'feature="tokio"' --cfg 'feature="tokio-executor"' --cfg 'feature="tokio-reactor"' --cfg 'feature="tokio-tcp"' --cfg 'feature="tokio-threadpool"' --cfg 'feature="tokio-timer"' -L 'dependency=/Users/davidlewis/.../target/debug/deps' --extern 'bytes=/Users/davidlewis/.../target/debug/deps/libbytes-f09dd1f0fde3c31f.rmeta' --extern 'futures_cpupool=/Users/davidlewis/.../target/debug/deps/libfutures_cpupool-ec1c84c3b4fae462.rmeta' --extern 'futures=/Users/davidlewis/.../target/debug/deps/libfutures-bbb22cef245af5e6.rmeta' --extern 'h2=/Users/davidlewis/.../target/debug/deps/libh2-91694226b42db5d5.rmeta' --extern 'http=/Users/davidlewis/.../target/debug/deps/libhttp-e080a05507736dac.rmeta' --extern 'httparse=/Users/davidlewis/.../target/debug/deps/libhttparse-8b94599dc69a7606.rmeta' --extern 'iovec=/Users/davidlewis/.../target/debug/deps/libiovec-ba7cbef176ea62bc.rmeta' --extern 'itoa=/Users/davidlewis/.../target/debug/deps/libitoa-19c513e1e6215f07.rmeta' --extern 'log=/Users/davidlewis/.../target/debug/deps/liblog-6b4ba12b6620245d.rmeta' --extern 'net2=/Users/davidlewis/.../target/debug/deps/libnet2-cec22209fc97d93f.rmeta' --extern 'time=/Users/davidlewis/.../target/debug/deps/libtime-f1fcdd05d8991eba.rmeta' --extern 'tokio=/Users/davidlewis/.../target/debug/deps/libtokio-9083fd465082cae9.rmeta' --extern 'tokio_executor=/Users/davidlewis/.../target/debug/deps/libtokio_executor-b273932dca490ca6.rmeta' --extern 'tokio_io=/Users/davidlewis/.../target/debug/deps/libtokio_io-bae61787e8d0768d.rmeta' --extern 'tokio_reactor=/Users/davidlewis/.../target/debug/deps/libtokio_reactor-11352bbe18d59bcb.rmeta' --extern 'tokio_tcp=/Users/davidlewis/.../target/debug/deps/libtokio_tcp-252773747cc2d682.rmeta' --extern 'tokio_threadpool=/Users/davidlewis/.../target/debug/deps/libtokio_threadpool-f06c22195f72583b.rmeta' --extern 'tokio_timer=/Users/davidlewis/.../target/debug/deps/libtokio_timer-fd5fbfe53554987d.rmeta' --extern 'want=/Users/davidlewis/.../target/debug/deps/libwant-f4cd6728406ef512.rmeta'` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
thread 'main' panicked at 'librustc/hir/map/hir_id_validator.rs:31: 
HirIdValidator: The recorded owner of path segment super (id=36924) is ::server[0]::conn[0]::{{?}}[34] instead of ::server[0]::conn[0]::{{?}}[34]::{{?}}[0]
HirIdValidator: Same HirId ::server[0]::conn[0]::{{?}}[34]::{{?}}[0]/2 assigned for nodes path segment super (id=36924) and path segment spawn_all (id=89958)', librustc/util/bug.rs:47:26
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   8: rustc::ty::context::tls::with_opt::{{closure}}
   9: rustc::ty::context::tls::with_context_opt
  10: rustc::ty::context::tls::with_opt
  11: rustc::util::bug::opt_span_bug_fmt
  12: rustc::util::bug::bug_fmt
  13: rustc::hir::map::hir_id_validator::check_crate
  14: rustc::hir::map::map_crate
  15: rustc::util::common::time
  16: rustc_driver::driver::compile_input
  17: rustc_driver::run_compiler_with_pool
  18: rustc_driver::driver::spawn_thread_pool
  19: rustc_driver::run_compiler
  20: <scoped_tls::ScopedKey<T>>::set
  21: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  22: __rust_maybe_catch_panic
  23: rustc_driver::run
  24: rustc_driver::main
  25: std::rt::lang_start::{{closure}}
  26: std::panicking::try::do_call
  27: __rust_maybe_catch_panic
  28: std::rt::lang_start_internal
  29: main
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-nightly (cae6efc37 2018-10-27) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `hyper`.

To learn more, run the command again with --verbose.
