
thread 'main' panicked at 'test', src\main.rs:2:5
stack backtrace:
   0:     0x7ff797ab6959 - backtrace::backtrace::trace_unsynchronized
                               at C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.40\src\backtrace\mod.rs:66
   1:     0x7ff797ab6959 - std::sys_common::backtrace::_print_fmt
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\sys_common\backtrace.rs:84
   2:     0x7ff797ab6959 - std::sys_common::backtrace::_print::{{impl}}::fmt
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\sys_common\backtrace.rs:61
   3:     0x7ff797ac5afb - core::fmt::write
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libcore\fmt\mod.rs:1030
   4:     0x7ff797ab4934 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\io\mod.rs:1412
   5:     0x7ff797ab9009 - std::sys_common::backtrace::_print
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\sys_common\backtrace.rs:65
   6:     0x7ff797ab9009 - std::sys_common::backtrace::print
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\sys_common\backtrace.rs:50
   7:     0x7ff797ab9009 - std::panicking::default_hook::{{closure}}
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:188
   8:     0x7ff797ab8c5c - std::panicking::default_hook
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:205
   9:     0x7ff797ab973c - std::panicking::rust_panic_with_hook
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:464
  10:     0x7ff797ab1025 - std::panicking::begin_panic<str*>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\src\libstd\panicking.rs:400
  11:     0x7ff797ab111c - test-remap::main
                               at \src\main.rs:2
  12:     0x7ff797ab10d6 - std::rt::lang_start::{{closure}}<()>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\src\libstd\rt.rs:61
  13:     0x7ff797ab9187 - std::rt::lang_start_internal::{{closure}}
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\rt.rs:48
  14:     0x7ff797ab9187 - std::panicking::try::do_call<closure-0,i32>
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:287
  15:     0x7ff797abbb12 - panic_unwind::__rust_maybe_catch_panic
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libpanic_unwind\lib.rs:81
  16:     0x7ff797ab9962 - std::panicking::try
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panicking.rs:265
  17:     0x7ff797ab9962 - std::panic::catch_unwind
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\panic.rs:395
  18:     0x7ff797ab9962 - std::rt::lang_start_internal
                               at /rustc/412f43ac5b4ae8c3599e71c6972112e9be4758fa\/src\libstd\rt.rs:47
  19:     0x7ff797ab1147 - main
  20:     0x7ff797ac8400 - invoke_main
                               at d:\agent\_work\3\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  21:     0x7ff797ac8400 - __scrt_common_main_seh
                               at d:\agent\_work\3\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  22:     0x7ff99d497974 - BaseThreadInitThunk
  23:     0x7ff99f98a271 - RtlUserThreadStart
