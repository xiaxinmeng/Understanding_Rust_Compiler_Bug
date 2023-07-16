
thread '<main>' panicked at 'wow!', src\main.rs:3
stack backtrace:
   1:     0x7ff6bbfe576e - sys::backtrace::write::hcf05c95842d5d9db4Bs
   2:     0x7ff6bbfea061 - rust_begin_unwind
   3:     0x7ff6bbfe1f7f - rt::unwind::begin_unwind_inner::h0aa6600eac0475fexKw
   4:     0x7ff6bbfe10a3 - begin_unwind<&str>
   5:     0x7ff6bbfe1030 - inner
   6:     0x7ff6bbfe11df - main
   7:     0x7ff6bbfeac99 - rt::lang_start::hdcf3e41d3b4ba1ffzbx
   8:     0x7ff6bbfe41c4 - rt::unwind::try::inner_try::heb5bc5c4cae3a32eqGw
   9:     0x7ff6bbfeaba8 - rt::lang_start::hdcf3e41d3b4ba1ffzbx
  10:     0x7ff6bbfe121a - main
  11:     0x7ff6bbfeec1c - __scrt_common_main_seh
  12:     0x7ffc2c362d92 - BaseThreadInitThunk
