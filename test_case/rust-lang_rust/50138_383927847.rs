
> rustc foo.rs -g; if ($?) { .\foo.exe }
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore\option.rs:335:21
stack backtrace:
   0: std::sys::windows::backtrace::unwind_backtrace
             at C:\projects\rust\src\libstd\sys\windows\backtrace\mod.rs:65
   1: std::sys_common::backtrace::_print
             at C:\projects\rust\src\libstd\sys_common\backtrace.rs:71
   2: std::sys_common::backtrace::print
             at C:\projects\rust\src\libstd\sys_common\backtrace.rs:59
   3: std::panicking::default_hook::{{closure}}
             at C:\projects\rust\src\libstd\panicking.rs:205
   4: std::panicking::default_hook
             at C:\projects\rust\src\libstd\panicking.rs:221
   5: std::panicking::rust_panic_with_hook
             at C:\projects\rust\src\libstd\panicking.rs:457
   6: std::panicking::begin_panic_fmt
             at C:\projects\rust\src\libstd\panicking.rs:344
   7: std::panicking::rust_begin_panic
             at C:\projects\rust\src\libstd\panicking.rs:322
   8: core::panicking::panic_fmt
             at C:\projects\rust\src\libcore\panicking.rs:71
   9: core::panicking::panic
             at C:\projects\rust\src\libcore\panicking.rs:51
  10: core::option::Option<bool>::unwrap<bool>
             at C:\projects\rust\src\libcore\macros.rs:20
  11: foo::bar
             at .\foo.rs:18
  12: foo::foo
             at .\foo.rs:22
  13: foo::main
             at .\foo.rs:26
  14: std::rt::lang_start::{{closure}}<()>
             at C:\projects\rust\src\libstd\rt.rs:74
  15: std::rt::lang_start_internal::{{closure}}
             at C:\projects\rust\src\libstd\rt.rs:59
  16: std::panicking::try::do_call<closure,i32>
             at C:\projects\rust\src\libstd\panicking.rs:304
  17: panic_unwind::__rust_maybe_catch_panic
             at C:\projects\rust\src\libpanic_unwind\lib.rs:105
  18: std::panicking::try
             at C:\projects\rust\src\libstd\panicking.rs:283
  19: std::panic::catch_unwind
             at C:\projects\rust\src\libstd\panic.rs:361
  20: std::rt::lang_start_internal
             at C:\projects\rust\src\libstd\rt.rs:58
  21: std::rt::lang_start<()>
             at C:\projects\rust\src\libstd\rt.rs:74
  22: main
  23: invoke_main
             at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  24: __scrt_common_main_seh
             at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:283
  25: BaseThreadInitThunk
  26: RtlUserThreadStart

> rustc -vV
rustc 1.27.0-nightly (ac3c2288f 2018-04-18)
binary: rustc
commit-hash: ac3c2288f9f9d977acb46406ba60033d65165a7b
commit-date: 2018-04-18
host: x86_64-pc-windows-msvc
release: 1.27.0-nightly
LLVM version: 6.0
