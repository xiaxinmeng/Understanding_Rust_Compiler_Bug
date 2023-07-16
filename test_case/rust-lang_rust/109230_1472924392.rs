plain
---- D:\a\rust\rust\src/doc/unstable-book\src\compiler-flags\sanitizer.md - ControlFlowIntegrity::Example (line 216) stdout ----
Test executable failed (exit code: 101).

stdout:
The answer is: 12
With CFI enabled, you should not see the next answer
stderr:
thread 'main' panicked at 'attempt to add with overflow', D:\a\rust\rust\src/doc/unstable-book\src\compiler-flags\sanitizer.md:37:5
stack backtrace:
   0:     0x7ff7522f96fc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1c8bee8166ca2dee
   0:     0x7ff7522f96fc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1c8bee8166ca2dee
   1:     0x7ff75230a708 - core::fmt::write::h11e849286d137000
   2:     0x7ff7522f7559 - std::io::stdio::_print::h21442c8313c32316
   3:     0x7ff7522f94ab - std::sys::common::alloc::realloc_fallback::hd68a4caebe063ddf
   4:     0x7ff7522fb8f5 - std::panicking::default_hook::h5b7dbf6749bd40bb
   5:     0x7ff7522fb4f0 - std::panicking::default_hook::h5b7dbf6749bd40bb
   6:     0x7ff7522fbe31 - std::panicking::rust_panic_with_hook::hd1cc9fe64f43d699
   7:     0x7ff7522fbccb - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h54705c3da9648eba
   8:     0x7ff7522fa0f9 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1c8bee8166ca2dee
   9:     0x7ff7522fb980 - rust_begin_unwind
  10:     0x7ff752310d45 - core::panicking::panic_fmt::h5df626a21a13d6ff
  11:     0x7ff752310dfc - core::panicking::panic::h5034edfcf3432d22
  12:     0x7ff7522f13cb - std::rt::lang_start::h9a691f139a8479f3
  13:     0x7ff7522f1485 - std::rt::lang_start::h9a691f139a8479f3
  14:     0x7ff7522f12a6 - std::rt::lang_start::h9a691f139a8479f3
  15:     0x7ff7522f1009 - __ImageBase
  16:     0x7ff7522f105c - std::rt::lang_start::h9a691f139a8479f3
  17:     0x7ff7522f47f1 - std::rt::lang_start_internal::h3e7f5970900b3191
  18:     0x7ff7522f1037 - std::rt::lang_start::h9a691f139a8479f3
  19:     0x7ff7522f1509 - main
  20:     0x7ff75230ed7c - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  21:     0x7ff75230ed7c - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  22:     0x7ffc0ad37ad4 - BaseThreadInitThunk
  23:     0x7ffc0bc0a371 - RtlUserThreadStart


failures:
    D:\a\rust\rust\src/doc/unstable-book\src\compiler-flags\sanitizer.md - ControlFlowIntegrity::Example (line 216)
    D:\a\rust\rust\src/doc/unstable-book\src\compiler-flags\sanitizer.md - ControlFlowIntegrity::Example (line 216)

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s

Build completed unsuccessfully in 0:58:05
make: *** [Makefile:68: ci-subset-1] Error 1
