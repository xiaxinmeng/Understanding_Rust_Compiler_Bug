plain
Test executable failed (exit code: 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"hello\r\n"`,
 right: `"hello\n"`', src\process.rs:19:1
stack backtrace:
   0:     0x7ff76a48825a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50f847d2350461a9
   1:     0x7ff76a4a6148 - core::fmt::write::h41197486a296c229
   2:     0x7ff76a483469 - <std::io::error::Error as core::fmt::Display>::fmt::h2505adad9b70e7f2
   3:     0x7ff76a48ab56 - std::panicking::default_hook::h43d61a3b1a287298
   4:     0x7ff76a48a74f - std::panicking::default_hook::h43d61a3b1a287298
   5:     0x7ff76a48b184 - std::panicking::rust_panic_with_hook::h71bee4f126409eea
   6:     0x7ff76a48b06d - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::hff17e89982af574f
   7:     0x7ff76a488ef7 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50f847d2350461a9
   8:     0x7ff76a48ad00 - rust_begin_unwind
   9:     0x7ff76a4ac335 - core::panicking::panic_fmt::h3b29e36fe8c3fc8c
  10:     0x7ff76a4a5471 - core::panicking::assert_failed_inner::h0428aec34d11c8e3
  11:     0x7ff76a473f2a - std::rt::lang_start::hebd1800a918147eb
  12:     0x7ff76a477123 - std::rt::lang_start::hebd1800a918147eb
  13:     0x7ff76a476fa9 - std::rt::lang_start::hebd1800a918147eb
  14:     0x7ff76a4729a6 - std::rt::lang_start::hebd1800a918147eb
  15:     0x7ff76a4713f9 - __ImageBase
  16:     0x7ff76a47199c - std::rt::lang_start::hebd1800a918147eb
  17:     0x7ff76a480c13 - std::rt::lang_start_internal::h45d07f31fc722f69
  18:     0x7ff76a471980 - std::rt::lang_start::hebd1800a918147eb
  19:     0x7ff76a477206 - main
  20:     0x7ff76a4aaac4 - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  21:     0x7ff76a4aaac4 - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  22:     0x7fff425e7974 - BaseThreadInitThunk
  23:     0x7fff4328a2f1 - RtlUserThreadStart


failures:
    src\process.rs - process::Command (line 461)
    src\process.rs - process::Command (line 461)

test result: FAILED. 1078 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 41.24s

error: test failed, to rerun pass '--doc'
 finished in 550.518 seconds
Build completed unsuccessfully in 0:52:02
make: *** [Makefile:70: ci-subset-1] Error 1
