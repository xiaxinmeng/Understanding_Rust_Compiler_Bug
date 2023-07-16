
thread 'main' panicked at 'foo', src\main.rs:2:5
stack backtrace:
   0:     0x7ff6258483af - std::backtrace_rs::backtrace::dbghelp::trace
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
   1:     0x7ff6258483af - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff6258483af - std::sys_common::backtrace::_print_fmt
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\sys_common\backtrace.rs:67
   3:     0x7ff6258483af - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\sys_common\backtrace.rs:46
   4:     0x7ff625855e7a - core::fmt::write
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\core\src\fmt\mod.rs:1115
   5:     0x7ff6258464d8 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\io\mod.rs:1665
   6:     0x7ff62584abe6 - std::sys_common::backtrace::_print
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\sys_common\backtrace.rs:49
   7:     0x7ff62584abe6 - std::sys_common::backtrace::print
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\sys_common\backtrace.rs:36
   8:     0x7ff62584abe6 - std::panicking::default_hook::closure$1
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panicking.rs:208
   9:     0x7ff62584a6c9 - std::panicking::default_hook
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panicking.rs:225
  10:     0x7ff62584b245 - std::panicking::rust_panic_with_hook
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panicking.rs:622
  11:     0x7ff62584208c - std::panicking::begin_panic::closure$0<str>
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\library\std\src\panicking.rs:542
  12:     0x7ff62584249f - std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic::closure$0,never$>
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\library\std\src\sys_common\backtrace.rs:141
  13:     0x7ff625841faf - std::panicking::begin_panic<str>
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\library\std\src\panicking.rs:541
  14:     0x7ff62584152c - playground::foo
                               at G:\WorkSpace\rust-playground\src\main.rs:2
  15:     0x7ff625841539 - playground::main
                               at G:\WorkSpace\rust-playground\src\main.rs:6
  16:     0x7ff6258423bb - core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\library\core\src\ops\function.rs:227
  17:     0x7ff6258424cb - std::sys_common::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\library\std\src\sys_common\backtrace.rs:125
  18:     0x7ff625841251 - std::rt::lang_start::closure$0<tuple$<> >
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\library\std\src\rt.rs:63
  19:     0x7ff62584b666 - core::ops::function::impls::impl$2::call_once
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\library\core\src\ops\function.rs:259
  20:     0x7ff62584b666 - std::panicking::try::do_call
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panicking.rs:401
  21:     0x7ff62584b666 - std::panicking::try
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panicking.rs:365
  22:     0x7ff62584b666 - std::panic::catch_unwind
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panic.rs:434
  23:     0x7ff62584b666 - std::rt::lang_start_internal::closure$2
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\rt.rs:45
  24:     0x7ff62584b666 - std::panicking::try::do_call
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panicking.rs:401
  25:     0x7ff62584b666 - std::panicking::try
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panicking.rs:365
  26:     0x7ff62584b666 - std::panic::catch_unwind
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\panic.rs:434
  27:     0x7ff62584b666 - std::rt::lang_start_internal
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\/library\std\src\rt.rs:45
  28:     0x7ff62584121f - std::rt::lang_start<tuple$<> >
                               at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b\library\std\src\rt.rs:62
  29:     0x7ff625841556 - main
  30:     0x7ff6258596b8 - invoke_main
                               at d:\A01\_work\12\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  31:     0x7ff6258596b8 - __scrt_common_main_seh
                               at d:\A01\_work\12\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  32:     0x7ff99c1d7034 - BaseThreadInitThunk
  33:     0x7ff99d262651 - RtlUserThreadStart
