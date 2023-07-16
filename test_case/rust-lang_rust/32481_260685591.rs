
$ RUST_BACKTRACE=1 ./a
thread 'main' panicked at 'Nesting!', a.rs:1
stack backtrace:
   0:     0x7ff764709994 - std::panicking::default_hook::{{closure}}
                        at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\panicking.rs:252
   1:     0x7ff76470841b - std::panicking::default_hook
                        at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\panicking.rs:263
   2:     0x7ff764708d1d - std::panicking::rust_panic_with_hook
                        at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\panicking.rs:451
   3:     0x7ff764701094 - CREATE_PROCESS_LOCK
   4:     0x7ff76470125f - CREATE_PROCESS_LOCK
   5:     0x7ff76470122a - CREATE_PROCESS_LOCK
   6:     0x7ff76470120a - CREATE_PROCESS_LOCK
   7:     0x7ff7647011ea - CREATE_PROCESS_LOCK
   8:     0x7ff76470127a - CREATE_PROCESS_LOCK
   9:     0x7ff76470be61 - panic_unwind::__rust_maybe_catch_panic
                        at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libpanic_unwind\lib.rs:97
  10:     0x7ff764707e5a - std::rt::lang_start
                        at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\rt.rs:51
  11:     0x7ff7647012b9 - main
  12:     0x7ff76470f9d4 - __scrt_common_main_seh
                        at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:253
  13:     0x7ffadc838363 - BaseThreadInitThunk
