
thread 'main' panicked at 'test', src\main.rs:2:5
stack backtrace:
   0:     0x7ff74bf77bb9 - backtrace::backtrace::trace_unsynchronized
                               at C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.40\src\backtrace\mod.rs:66
   1:     0x7ff74bf77bb9 - std::sys_common::backtrace::_print_fmt
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\sys_common\backtrace.rs:84
   2:     0x7ff74bf77bb9 - std::sys_common::backtrace::_print::{{impl}}::fmt
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\sys_common\backtrace.rs:61
   3:     0x7ff74bf86e8b - core::fmt::write
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libcore\fmt\mod.rs:1030
   4:     0x7ff74bf75b94 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\io\mod.rs:1412
   5:     0x7ff74bf7a269 - std::sys_common::backtrace::_print
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\sys_common\backtrace.rs:65
   6:     0x7ff74bf7a269 - std::sys_common::backtrace::print
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\sys_common\backtrace.rs:50
   7:     0x7ff74bf7a269 - std::panicking::default_hook::{{closure}}
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:188
   8:     0x7ff74bf79ebc - std::panicking::default_hook
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:205
   9:     0x7ff74bf7a99c - std::panicking::rust_panic_with_hook
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:464
  10:     0x7ff74bf71ef1 - std::panicking::begin_panic<str*>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\src\libstd\panicking.rs:400
  11:     0x7ff74bf71e4c - test-remap::main
                               at C:\Users\User\Documents\rust\test-remap\src\main.rs:2
  12:     0x7ff74bf716d0 - std::rt::lang_start::{{closure}}<()>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\src\libstd\rt.rs:61
  13:     0x7ff74bf7a3e7 - std::rt::lang_start_internal::{{closure}}
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\rt.rs:48
  14:     0x7ff74bf7a3e7 - std::panicking::try::do_call<closure-0,i32>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:287
  15:     0x7ff74bf7cd72 - panic_unwind::__rust_maybe_catch_panic
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libpanic_unwind\lib.rs:81
  16:     0x7ff74bf7abc2 - std::panicking::try
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:265
  17:     0x7ff74bf7abc2 - std::panic::catch_unwind
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panic.rs:395
  18:     0x7ff74bf7abc2 - std::rt::lang_start_internal
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\rt.rs:47
  19:     0x7ff74bf716ab - std::rt::lang_start<()>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\src\libstd\rt.rs:61
  20:     0x7ff74bf71e70 - main
  21:     0x7ff74bf89790 - invoke_main
                               at d:\agent\_work\3\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  22:     0x7ff74bf89790 - __scrt_common_main_seh
                               at d:\agent\_work\3\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  23:     0x7ff99d497974 - BaseThreadInitThunk
  24:     0x7ff99f98a271 - RtlUserThreadStart
