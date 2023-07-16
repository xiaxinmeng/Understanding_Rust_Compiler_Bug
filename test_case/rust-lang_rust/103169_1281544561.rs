plain
....................iii................................................................. 13640/13681
.........................................
failures:

---- [ui] src/test/ui/error-trait/error-in-panic.rs stdout ----
diff of run.stderr:
4 Caused by:
4 Caused by:
5       my source's source error message
-    0:        0x10347e5cc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h739002e9033ecbde
-    1:        0x1034c1ef4 - core::fmt::write::h7ee3c88f1a3812c3
-    2:        0x103467ad8 - std::io::Write::write_fmt::h46e675c7a43a1dc6
-    3:        0x10347e494 - std::sys_common::backtrace::print::h2db63c69899aa986
-    3:        0x10347e494 - std::sys_common::backtrace::print::h2db63c69899aa986
-    4:        0x103441588 - std::panicking::default_hook::{{closure}}::hbf1cbfed6089b966
-    5:        0x103441348 - std::panicking::default_hook::h394fc36f8066489c
-    6:        0x103441a3c - std::panicking::rust_panic_with_hook::h022616bb2ba4e178
-    7:        0x10344907c - std::panicking::begin_panic_handler::{{closure}}::h3128246bc073fcbe
-    8:        0x103448fdc - std::sys_common::backtrace::__rust_end_short_backtrace::hbb524939a68ed8b5
-    9:        0x103441660 - _rust_begin_unwind
-   10:        0x1034dd910 - core::panicking::panic_source::hf14287b3ee2e7392
-   11:        0x102aeb728 - error_in_panic::main::hf4bbb2ae489ce4fd
-   12:        0x102aeb404 - core::ops::function::FnOnce::call_once::h0f54538c8c239b20
-   13:        0x102aeafd8 - std::sys_common::backtrace::__rust_begin_short_backtrace::h2b8f58dd23d62fa6
-   14:        0x102aeb074 - std::rt::lang_start::{{closure}}::h6d7fe7ddaa109f16
-   15:        0x10344478c - std::panicking::try::h0286e30fd7b2d786
-   16:        0x103452248 - std::rt::lang_start_internal::hc2ec5a7105141124
-   17:        0x102aeb048 - std::rt::lang_start::h8ea75b901d46518a
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   18:        0x102aeb7f0 - _main
+    0:     0x7f54568a61de - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd857c22806db1171
+    1:     0x7f545690f908 - core::fmt::write::h65a67a15421a1fc1
+    2:     0x7f54568978f1 - std::io::Write::write_fmt::hb52ec731a842c8d1
+    3:     0x7f54568a5fe1 - std::sys_common::backtrace::print::hf003363f0b27a3c4
+    4:     0x7f54568a920a - std::panicking::default_hook::{{closure}}::h0aedd680e19ca052
+    5:     0x7f54568a8e5a - std::panicking::default_hook::h52ee606c34175468
+    6:     0x7f54568a99d9 - std::panicking::rust_panic_with_hook::h1f3990a6410fc18f
+    7:     0x7f54568a96c8 - std::panicking::begin_panic_handler::{{closure}}::h86038d5397ca8cd6
+    8:     0x7f54568a6725 - std::sys_common::backtrace::__rust_end_short_backtrace::hfa84816a3e4eb5e8
+    9:     0x7f54568a93af - rust_begin_unwind
+   10:     0x7f5456859acd - core::panicking::panic_source::hd9027bed6f1a9bd8
+   11:     0x556eda062776 - error_in_panic::main::hf4bbb2ae489ce4fd
+   12:     0x556eda062473 - core::ops::function::FnOnce::call_once::he350982c45f6b3ed
+   13:     0x556eda062149 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1beb194095d7c0db
+   14:     0x556eda062199 - std::rt::lang_start::{{closure}}::h64ba537dd910eea5
+   15:     0x7f54568855b4 - std::rt::lang_start_internal::he8621bcc11fbffde
+   16:     0x556eda062177 - std::rt::lang_start::hc4ac859d88949488
+   17:     0x556eda062838 - main
+   18:     0x7f54565c4d90 - <unknown>
+   20:     0x556eda062075 - _start
+   21:                0x0 - <unknown>
26 



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-trait/error-in-panic/error-in-panic.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-trait/error-in-panic/a"
stdout: none
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'here's my panic error message', /checkout/src/test/ui/error-trait/error-in-panic.rs:51:5
Source: my source error message
Caused by:
Caused by:
      my source's source error message
stack backtrace:
   0:     0x7f54568a61de - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd857c22806db1171
   1:     0x7f545690f908 - core::fmt::write::h65a67a15421a1fc1
   2:     0x7f54568978f1 - std::io::Write::write_fmt::hb52ec731a842c8d1
   3:     0x7f54568a5fe1 - std::sys_common::backtrace::print::hf003363f0b27a3c4
   4:     0x7f54568a920a - std::panicking::default_hook::{{closure}}::h0aedd680e19ca052
   5:     0x7f54568a8e5a - std::panicking::default_hook::h52ee606c34175468
   6:     0x7f54568a99d9 - std::panicking::rust_panic_with_hook::h1f3990a6410fc18f
   7:     0x7f54568a96c8 - std::panicking::begin_panic_handler::{{closure}}::h86038d5397ca8cd6
   8:     0x7f54568a6725 - std::sys_common::backtrace::__rust_end_short_backtrace::hfa84816a3e4eb5e8
   9:     0x7f54568a93af - rust_begin_unwind
  10:     0x7f5456859acd - core::panicking::panic_source::hd9027bed6f1a9bd8
  11:     0x556eda062776 - error_in_panic::main::hf4bbb2ae489ce4fd
  12:     0x556eda062473 - core::ops::function::FnOnce::call_once::he350982c45f6b3ed
  13:     0x556eda062149 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1beb194095d7c0db
  14:     0x556eda062199 - std::rt::lang_start::{{closure}}::h64ba537dd910eea5
  15:     0x7f54568855b4 - std::rt::lang_start_internal::he8621bcc11fbffde
  16:     0x556eda062177 - std::rt::lang_start::hc4ac859d88949488
  17:     0x556eda062838 - main
  18:     0x7f54565c4d90 - <unknown>
  20:     0x556eda062075 - _start
  21:                0x0 - <unknown>
------------------------------------------

