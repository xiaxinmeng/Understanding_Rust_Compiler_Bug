text
~\back [master +4 ~0 -0 !]> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target\debug\back.exe`
thread 'main' panicked at 'Hello, world!', src\main.rs:2:5
stack backtrace:
   0: std::sys::windows::backtrace::set_frames
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\sys\windows\backtrace\mod.rs:95
   1: std::sys::windows::backtrace::unwind_backtrace
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\sys\windows\backtrace\mod.rs:82
   2: std::sys_common::backtrace::_print
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\sys_common\backtrace.rs:71
   3: std::sys_common::backtrace::print
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\sys_common\backtrace.rs:59
   4: std::panicking::default_hook::{{closure}}
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\panicking.rs:197
   5: std::panicking::default_hook
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\panicking.rs:211
   6: std::panicking::rust_panic_with_hook
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\panicking.rs:474
   7: std::panicking::begin_panic<str*>
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\src\libstd\panicking.rs:408
   8: back::main
             at .\src\main.rs:2
   9: std::rt::lang_start::{{closure}}<()>
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\src\libstd\rt.rs:64
  10: std::rt::lang_start_internal::{{closure}}
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\rt.rs:49
  11: std::panicking::try::do_call<closure,i32>
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\panicking.rs:293
  12: panic_unwind::__rust_maybe_catch_panic
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libpanic_unwind\lib.rs:85
  13: std::panicking::try
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\panicking.rs:272
  14: std::panic::catch_unwind
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\panic.rs:394
  15: std::rt::lang_start_internal
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\/src\libstd\rt.rs:48
  16: std::rt::lang_start<()>
             at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c\src\libstd\rt.rs:64
  17: main
  18: invoke_main
             at d:\agent\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  19: __scrt_common_main_seh
             at d:\agent\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  20: BaseThreadInitThunk
  21: RtlUserThreadStart
error: process didn't exit successfully: `target\debug\back.exe` (exit code: 101)
