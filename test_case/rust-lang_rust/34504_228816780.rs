
C:\msys64\home\Peter\rust [please-be-robust-already ≡]> python.exe .\src\bootstrap\bootstrap.py
thread '<main>' panicked at 'couldn't determine visual studio generator', C:\Users\Peter\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.17\src\lib.rs:382
stack backtrace:
   0:     0x7ff6f805c482 - std::rt::lang_start::h77a570d0150f3bb2
   1:     0x7ff6f805ba6d - std::rt::lang_start::h77a570d0150f3bb2
   2:     0x7ff6f8041775 - std::panicking::rust_panic_with_hook::h86decb25b2d9b2e0
   3:     0x7ff6f7fde944 - begin_panic<&str>
                        at C:\msys64\home\Peter\src\libstd\panicking.rs:328
   4:     0x7ff6f7fe15f9 - visual_studio_generator
                        at C:\msys64\home\Peter\rust\<std macros>:3
   5:     0x7ff6f7fd30cf - build
                        at C:\Users\Peter\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.17\src\lib.rs:276
   6:     0x7ff6f7efc997 - llvm
                        at C:\msys64\home\Peter\rust\src\bootstrap\build\native.rs:113
   7:     0x7ff6f7ee978b - build
                        at C:\msys64\home\Peter\rust\src\bootstrap\build\mod.rs:230
   8:     0x7ff6f7ee12ba - main
                        at C:\msys64\home\Peter\rust\src\bootstrap\main.rs:48
   9:     0x7ff6f805b46c - std::rt::lang_start::h77a570d0150f3bb2
  10:     0x7ff6f80616c1 - _rust_maybe_catch_panic
  11:     0x7ff6f805b1a4 - std::rt::lang_start::h77a570d0150f3bb2
  12:     0x7ff6f7eee9f9 - main
  13:     0x7ff6f8068bef - __scrt_common_main_seh
                        at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:255
  14:     0x7ffe8ad78101 - BaseThreadInitThunk
