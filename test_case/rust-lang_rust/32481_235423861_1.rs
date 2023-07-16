
test> rustc nested.rs; ./nested.exe
thread 'main' panicked at 'Nesting!', nested.rs:9
stack backtrace:
   0:     0x7ff749049d6c - std::rt::lang_start::hfe4efe1fc39e4a30
   1:     0x7ff749049379 - std::rt::lang_start::hfe4efe1fc39e4a30
   2:     0x7ff7490424fd - std::panicking::rust_panic_with_hook::h983af77c1a2e581b
   3:     0x7ff749041109 - main
   4:     0x7ff74904109d - main
   5:     0x7ff749041068 - main
   6:     0x7ff749041058 - main
   7:     0x7ff749041018 - __ImageBase
   8:     0x7ff749041008 - __ImageBase
   9:     0x7ff749048d6c - std::rt::lang_start::hfe4efe1fc39e4a30
  10:     0x7ff74904cd81 - _rust_maybe_catch_panic
  11:     0x7ff749048aa4 - std::rt::lang_start::hfe4efe1fc39e4a30
  12:     0x7ff749041049 - main
  13:     0x7ff74905073b - __scrt_common_main_seh
                        at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:255
  14:     0x7ffa1d228101 - BaseThreadInitThunk

test> rustc -g nested.rs; ./nested.exe
thread 'main' panicked at 'Nesting!', nested.rs:9
stack backtrace:
   0:     0x7ff7338e9ddc - std::rt::lang_start::hfe4efe1fc39e4a30
   1:     0x7ff7338e93e9 - std::rt::lang_start::hfe4efe1fc39e4a30
   2:     0x7ff7338e256d - std::panicking::rust_panic_with_hook::h983af77c1a2e581b
   3:     0x7ff7338e1152 - begin_panic<&str>
                        at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\panicking.rs:328
   4:     0x7ff7338e10df - foo
                        at C:\msys64\home\Peter\test\<std macros>:3
   5:     0x7ff7338e109e - foo
                        at C:\msys64\home\Peter\test\nested.rs:7
   6:     0x7ff7338e107e - foo
                        at C:\msys64\home\Peter\test\nested.rs:5
   7:     0x7ff7338e102e - foo
                        at C:\msys64\home\Peter\test\nested.rs:3
   8:     0x7ff7338e100e - main
                        at C:\msys64\home\Peter\test\nested.rs:14
   9:     0x7ff7338e8ddc - std::rt::lang_start::hfe4efe1fc39e4a30
  10:     0x7ff7338ecdf1 - _rust_maybe_catch_panic
  11:     0x7ff7338e8b14 - std::rt::lang_start::hfe4efe1fc39e4a30
  12:     0x7ff7338e1069 - main
  13:     0x7ff7338f07ab - __scrt_common_main_seh
                        at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:255
  14:     0x7ffa1d228101 - BaseThreadInitThunk
