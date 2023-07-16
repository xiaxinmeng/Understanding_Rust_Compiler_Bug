
keruspe@Lou ~/rust-segv % rustc-stable foo.rs && ls -lh foo && RUST_BACKTRACE=1 ./foo    
-rwxr-xr-x 1 keruspe keruspe 1.3M Jul  9 11:46 foo
thread 'main' panicked at 'foo', foo.rs:2:1
zsh: segmentation fault (core dumped)  RUST_BACKTRACE=1 ./foo
keruspe@Lou ~/rust-segv % rustc-beta foo.rs && ls -lh foo && RUST_BACKTRACE=1 ./foo      
-rwxr-xr-x 1 keruspe keruspe 1.3M Jul  9 11:46 foo
thread 'main' panicked at 'foo', foo.rs:2:1
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: foo::main
   7: std::rt::lang_start::{{closure}}
   8: std::panicking::try::do_call
   9: __rust_maybe_catch_panic
  10: std::rt::lang_start_internal
  11: std::rt::lang_start
  12: main
  13: __libc_start_main
  14: _start
keruspe@Lou ~/rust-segv % rustc-nightly foo.rs && ls -lh foo && RUST_BACKTRACE=1 ./foo   
-rwxr-xr-x 1 keruspe keruspe 1.3M Jul  9 11:46 foo
thread 'main' panicked at 'foo', foo.rs:2:1
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: foo::main
   7: std::rt::lang_start::{{closure}}
   8: std::panicking::try::do_call
   9: __rust_maybe_catch_panic
  10: std::rt::lang_start_internal
  11: std::rt::lang_start
  12: main
  13: __libc_start_main
  14: _start
