
   0:     0x7ff66a138ccf - std::backtrace_rs::backtrace::dbghelp::trace
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
   1:     0x7ff66a138ccf - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff66a138ccf - std::sys_common::backtrace::_print_fmt
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\sys_common\backtrace.rs:67
   3:     0x7ff66a138ccf - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\sys_common\backtrace.rs:46
   4:     0x7ff66a147f2a - core::fmt::write
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\core\src\fmt\mod.rs:1163
   5:     0x7ff66a1368b8 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\io\mod.rs:1696
   6:     0x7ff66a13aff6 - std::sys_common::backtrace::_print
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\sys_common\backtrace.rs:49
   7:     0x7ff66a13aff6 - std::sys_common::backtrace::print
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\sys_common\backtrace.rs:36
   8:     0x7ff66a13aff6 - std::panicking::default_hook::closure$1
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:210
   9:     0x7ff66a13a9f7 - std::panicking::default_hook
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:227
  10:     0x7ff66a13b655 - std::panicking::rust_panic_with_hook
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:624
  11:     0x7ff66a13b23b - std::panicking::begin_panic_handler::closure$0
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:521
  12:     0x7ff66a1395f7 - std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure$0,never$>
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\sys_common\backtrace.rs:139
  13:     0x7ff66a13b199 - std::panicking::begin_panic_handler
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:517
  14:     0x7ff66a14cb40 - core::panicking::panic_fmt
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\core\src\panicking.rs:100
  15:     0x7ff66a14cbe3 - core::result::unwrap_failed
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\core\src\result.rs:1616
  16:     0x7ff66a131242 - <&T as core::fmt::Debug>::fmt::h6b71318ff72d370b
  17:     0x7ff66a131316 - std::sys_common::backtrace::__rust_begin_short_backtrace::hb40e258bfbdd6d09
  18:     0x7ff66a1312ec - std::rt::lang_start::{{closure}}::h302d67aaccd9565c
  19:     0x7ff66a1387bb - core::ops::function::impls::impl$2::call_once
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\library\core\src\ops\function.rs:259
  20:     0x7ff66a1387bb - std::panicking::try::do_call
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:403
  21:     0x7ff66a1387bb - std::panicking::try
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:367
  22:     0x7ff66a1387bb - std::panic::catch_unwind
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panic.rs:133
  23:     0x7ff66a1387bb - std::rt::lang_start_internal::closure$2
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\rt.rs:128
  24:     0x7ff66a1387bb - std::panicking::try::do_call
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:403
  25:     0x7ff66a1387bb - std::panicking::try
  26:     0x7ff66a1387bb - std::panic::catch_unwind
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panic.rs:133
  27:     0x7ff66a1387bb - std::rt::lang_start_internal
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\rt.rs:128
  28:     0x7ff66a1312d7 - main
  29:     0x7ff66a14b804 - invoke_main
                               at d:\A01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  30:     0x7ff66a14b804 - __scrt_common_main_seh
                               at d:\A01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  31:     0x7ff894cb7034 - BaseThreadInitThunk
  32:     0x7ff896482651 - RtlUserThreadStart
