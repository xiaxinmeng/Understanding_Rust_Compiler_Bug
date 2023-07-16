text
~\back [master +4 ~0 -0 !]> cargo +stable-x86_64-pc-windows-gnu run
   Compiling back v0.1.0 (C:\Users\steve\back)
    Finished dev [unoptimized + debuginfo] target(s) in 1.05s
     Running `target\debug\back.exe`
thread 'main' panicked at 'Hello, world!', src\main.rs:2:5
stack backtrace:
   0: backtrace::backtrace::dbghelp::trace
             at C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.29\src\backtrace/dbghelp.rs:90
   1: backtrace::backtrace::trace_unsynchronized
             at C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.29\src\backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src\libstd\sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src\libstd\sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src\libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src\libstd/panicking.rs:214
   6: std::panicking::rust_panic_with_hook
             at src\libstd/panicking.rs:477
   7: std::panicking::begin_panic
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\src\libstd/panicking.rs:411
   8: back::main
             at src/main.rs:2
   9: std::rt::lang_start::{{closure}}
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\src\libstd/rt.rs:64
  10: std::rt::lang_start_internal::{{closure}}
             at src\libstd/rt.rs:49
  11: std::panicking::try::do_call
             at src\libstd/panicking.rs:296
  12: __rust_maybe_catch_panic
             at src\libpanic_unwind/lib.rs:82
  13: std::panicking::try
             at src\libstd/panicking.rs:275
  14: std::panic::catch_unwind
             at src\libstd/panic.rs:394
  15: std::rt::lang_start_internal
             at src\libstd/rt.rs:48
  16: std::rt::lang_start
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\src\libstd/rt.rs:64
  17: main
  18: _tmainCRTStartup
  19: mainCRTStartup
  20: unit_addrs_search
  21: unit_addrs_search
error: process didn't exit successfully: `target\debug\back.exe` (exit code: 101)
