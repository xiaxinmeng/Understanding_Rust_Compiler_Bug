
test> rustc -g nested.rs ; ./nested.exe
thread 'main' panicked at 'Nesting!', nested.rs:8
stack backtrace:
   0:     0x7ff62e26948a - std::panicking::rust_panic_with_hook::hb1322e5f2588b4db
   1:     0x7ff62e2681b2 - std::rt::lang_start::haaae1186de9de8cb
   2:     0x7ff62e268b5d - std::panicking::rust_panic_with_hook::hb1322e5f2588b4db
   3:     0x7ff62e2610d7 - std::panicking::begin_panic<&str>
                        at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\panicking.rs:383
   4:     0x7ff62e261325 - nested::a::b::c::d::foo
                        at C:\msys64\home\Peter\test\nested.rs:8
   5:     0x7ff62e2612f0 - nested::a::b::c::foo
                        at C:\msys64\home\Peter\test\nested.rs:6
   6:     0x7ff62e2612d0 - nested::a::b::foo
                        at C:\msys64\home\Peter\test\nested.rs:4
   7:     0x7ff62e2612b0 - nested::a::foo
                        at C:\msys64\home\Peter\test\nested.rs:2
   8:     0x7ff62e261340 - nested::main
                        at C:\msys64\home\Peter\test\nested.rs:13
   9:     0x7ff62e26bfb1 - _rust_maybe_catch_panic
  10:     0x7ff62e267bea - std::rt::lang_start::haaae1186de9de8cb
  11:     0x7ff62e261379 - main
  12:     0x7ff62e26fca4 - __scrt_common_main_seh
                        at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:253
  13:     0x7ff964778101 - BaseThreadInitThunk
