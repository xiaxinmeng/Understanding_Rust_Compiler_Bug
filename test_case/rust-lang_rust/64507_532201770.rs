text
~\tmp> cargo new panictest
     Created binary (application) `panictest` package
~\tmp> cd panictest
~\tmp\panictest [master +3 ~0 -0 !]> $env:RUST_BACKTRACE=1
~\tmp\panictest [master +3 ~0 -0 !]> code .\src\main.rs
~\tmp\panictest [master +3 ~0 -0 !]> cat .\src\main.rs
fn main() {
    panic!("Hello, world!");
}
~\tmp\panictest [master +3 ~0 -0 !]> rustc --version
rustc 1.37.0 (eae3437df 2019-08-13)
~\tmp\panictest [master +3 ~0 -0 !]> cargo run
   Compiling panictest v0.1.0 (C:\Users\steve\tmp\panictest)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target\debug\panictest.exe`
thread 'main' panicked at 'Hello, world!', src\main.rs:2:5
stack backtrace:
   0: backtrace::backtrace::trace_unsynchronized
             at C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.29\src\backtrace\mod.rs:66
   1: std::sys_common::backtrace::_print
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\sys_common\backtrace.rs:47
   2: std::sys_common::backtrace::print
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\sys_common\backtrace.rs:36
   3: std::panicking::default_hook::{{closure}}
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\panicking.rs:200
   4: std::panicking::default_hook
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\panicking.rs:214
   5: std::panicking::rust_panic_with_hook
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\panicking.rs:477
   6: std::panicking::begin_panic<str*>
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\src\libstd\panicking.rs:411
   7: panictest::main
             at .\src\main.rs:2
   8: std::rt::lang_start::{{closure}}<()>
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\src\libstd\rt.rs:64
   9: std::rt::lang_start_internal::{{closure}}
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\rt.rs:49
  10: std::panicking::try::do_call<closure,i32>
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\panicking.rs:296
  11: panic_unwind::__rust_maybe_catch_panic
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libpanic_unwind\lib.rs:82
  12: std::panicking::try
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\panicking.rs:275
  13: std::panic::catch_unwind
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\panic.rs:394
  14: std::rt::lang_start_internal
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\/src\libstd\rt.rs:48
  15: std::rt::lang_start<()>
             at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b\src\libstd\rt.rs:64
  16: main
  17: invoke_main
             at d:\agent\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  18: __scrt_common_main_seh
             at d:\agent\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  19: BaseThreadInitThunk
  20: RtlUserThreadStart
error: process didn't exit successfully: `target\debug\panictest.exe` (exit code: 101)
~\tmp\panictest [master +4 ~0 -0 !]> rustc +nightly --version
rustc 1.39.0-nightly (96d07e0ac 2019-09-15)
~\tmp\panictest [master +4 ~0 -0 !]> cargo +nightly run
   Compiling panictest v0.1.0 (C:\Users\steve\tmp\panictest)
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
     Running `target\debug\panictest.exe`
thread 'main' panicked at 'Hello, world!', src\main.rs:2:5
stack backtrace:
   0: backtrace::backtrace::trace_unsynchronized
             at C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.37\src\backtrace\mod.rs:66
   1: std::sys_common::backtrace::_print_fmt
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\sys_common\backtrace.rs:77
   2: std::sys_common::backtrace::_print::{{impl}}::fmt
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\sys_common\backtrace.rs:61
   3: core::fmt::write
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libcore\fmt\mod.rs:1030
   4: std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\io\mod.rs:1412
   5: std::sys_common::backtrace::_print
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\sys_common\backtrace.rs:65
   6: std::sys_common::backtrace::print
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\sys_common\backtrace.rs:50
   7: std::panicking::default_hook::{{closure}}
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\panicking.rs:200
   8: std::panicking::default_hook
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\panicking.rs:214
   9: std::panicking::rust_panic_with_hook
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\panicking.rs:477
  10: std::panicking::begin_panic<str*>
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\src\libstd\panicking.rs:411
  11: panictest::main
             at .\src\main.rs:2
  12: std::rt::lang_start::{{closure}}<()>
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\src\libstd\rt.rs:64
  13: std::rt::lang_start_internal::{{closure}}
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\rt.rs:49
  14: std::panicking::try::do_call<closure-0,i32>
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\panicking.rs:296
  15: panic_unwind::__rust_maybe_catch_panic
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libpanic_unwind\lib.rs:80
  16: std::panicking::try
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\panicking.rs:275
  17: std::panic::catch_unwind
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\panic.rs:394
  18: std::rt::lang_start_internal
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\/src\libstd\rt.rs:48
  19: std::rt::lang_start<()>
             at /rustc/96d07e0ac9f0c56b95a2561c6cedac0b23a5d2a3\src\libstd\rt.rs:64
  20: main
  21: invoke_main
             at d:\agent\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  22: __scrt_common_main_seh
             at d:\agent\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  23: BaseThreadInitThunk
  24: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\panictest.exe` (exit code: 101)
~\tmp\panictest [master +4 ~0 -0 !]> rustc +stable-x86_64-pc-windows-gnu --version
rustc 1.37.0 (eae3437df 2019-08-13)
~\tmp\panictest [master +4 ~0 -0 !]> cargo +stable-x86_64-pc-windows-gnu run
   Compiling panictest v0.1.0 (C:\Users\steve\tmp\panictest)
    Finished dev [unoptimized + debuginfo] target(s) in 0.74s
     Running `target\debug\panictest.exe`
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
   8: panictest::main
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
error: process didn't exit successfully: `target\debug\panictest.exe` (exit code: 101)
