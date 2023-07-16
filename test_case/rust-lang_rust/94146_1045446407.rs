plain
............................i....................................................................... 1300/12646
..............................................................i..................................... 1400/12646
.................................................................................................... 1500/12646
.................................................................................................... 1600/12646
.........................i..F....................................................................... 1700/12646
..........................................i......................................................... 1900/12646
.................................................................................................... 2000/12646
.................................................................................................... 2100/12646
...............i.................................................................................... 2200/12646
---

---- [ui] ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs stdout ----
diff of stderr:

1 error: <unknown>:0:0: in function entry_function i32 (i32, i32, i32, i32, i32): secure entry function requires arguments on stack
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1470:21
+ stack backtrace:
+    0:     0x7f2c3b43eebc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h47aeb2d8f72b397d
+    1:     0x7f2c3b4b064f - core::fmt::write::h9721ea598362cf25
+    2:     0x7f2c3b42c481 - std::io::Write::write_fmt::hff3cdbcbf137c998
+    3:     0x7f2c3b4433be - std::panicking::default_hook::{{closure}}::h82be8229fb447305
+    4:     0x7f2c3b442f89 - std::panicking::default_hook::h4ed0aa1eb7da8a45
+    5:     0x7f2c3be60181 - rustc_driver[ff7f5aad0ad61854]::DEFAULT_HOOK::{closure#0}::{closure#0}
+    6:     0x7f2c3b443be8 - std::panicking::rust_panic_with_hook::h67cc2a282d04dec5
+    7:     0x7f2c3b443a17 - std::panicking::begin_panic_handler::{{closure}}::h47d1e658b48c405b
+    8:     0x7f2c3b43f3b4 - std::sys_common::backtrace::__rust_end_short_backtrace::h078a87354dacea13
+    9:     0x7f2c3b4436f9 - rust_begin_unwind
+   10:     0x7f2c3b3f7803 - core::panicking::panic_fmt::he9d2d568cb356fd7
+   11:     0x7f2c3b4acb48 - core::panicking::assert_failed_inner::hc1b5496e266762e3
+   12:     0x7f2c3bb62f64 - core[99b05a1f48df5ac]::panicking::assert_failed::<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::MainThreadWorkerState, rustc_codegen_ssa[92d2f24577e8b99e]::back::write::MainThreadWorkerState>
+   13:     0x7f2c3c1944ef - std[2d8d369c56f69b1f]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend as rustc_codegen_ssa[92d2f24577e8b99e]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::start_executing_work<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend>::{closure#4}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#0}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>
+   14:     0x7f2c3c228b48 - std[2d8d369c56f69b1f]::panicking::try::<core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>, core[99b05a1f48df5ac]::panic::unwind_safe::AssertUnwindSafe<<std[2d8d369c56f69b1f]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend as rustc_codegen_ssa[92d2f24577e8b99e]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::start_executing_work<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend>::{closure#4}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#0}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
+   15:     0x7f2c3c15e57b - <<std[2d8d369c56f69b1f]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend as rustc_codegen_ssa[92d2f24577e8b99e]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::start_executing_work<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend>::{closure#4}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#0}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#1} as core[99b05a1f48df5ac]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   16:     0x7f2c3b4537c3 - std::sys::unix::thread::Thread::new::thread_start::h623e57fa0ec4b1c2
+   17:     0x7f2c357c2609 - start_thread
+   18:     0x7f2c3b2b9293 - clone
+   19:                0x0 - <unknown>
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.60.0-nightly (740609861 2022-02-19) running on x86_64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: <unknown>:0:0: in function entry_function i32 (i32, i32, i32, i32, i32): secure entry function requires arguments on stack
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1470:21
stack backtrace:
   0:     0x7f2c3b43eebc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h47aeb2d8f72b397d
   1:     0x7f2c3b4b064f - core::fmt::write::h9721ea598362cf25
   2:     0x7f2c3b42c481 - std::io::Write::write_fmt::hff3cdbcbf137c998
   3:     0x7f2c3b4433be - std::panicking::default_hook::{{closure}}::h82be8229fb447305
   4:     0x7f2c3b442f89 - std::panicking::default_hook::h4ed0aa1eb7da8a45
   5:     0x7f2c3be60181 - rustc_driver[ff7f5aad0ad61854]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2c3b443be8 - std::panicking::rust_panic_with_hook::h67cc2a282d04dec5
   7:     0x7f2c3b443a17 - std::panicking::begin_panic_handler::{{closure}}::h47d1e658b48c405b
   8:     0x7f2c3b43f3b4 - std::sys_common::backtrace::__rust_end_short_backtrace::h078a87354dacea13
   9:     0x7f2c3b4436f9 - rust_begin_unwind
  10:     0x7f2c3b3f7803 - core::panicking::panic_fmt::he9d2d568cb356fd7
  11:     0x7f2c3b4acb48 - core::panicking::assert_failed_inner::hc1b5496e266762e3
  12:     0x7f2c3bb62f64 - core[99b05a1f48df5ac]::panicking::assert_failed::<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::MainThreadWorkerState, rustc_codegen_ssa[92d2f24577e8b99e]::back::write::MainThreadWorkerState>
  13:     0x7f2c3c1944ef - std[2d8d369c56f69b1f]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend as rustc_codegen_ssa[92d2f24577e8b99e]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::start_executing_work<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend>::{closure#4}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#0}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>
  14:     0x7f2c3c228b48 - std[2d8d369c56f69b1f]::panicking::try::<core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>, core[99b05a1f48df5ac]::panic::unwind_safe::AssertUnwindSafe<<std[2d8d369c56f69b1f]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend as rustc_codegen_ssa[92d2f24577e8b99e]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::start_executing_work<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend>::{closure#4}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#0}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
  15:     0x7f2c3c15e57b - <<std[2d8d369c56f69b1f]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend as rustc_codegen_ssa[92d2f24577e8b99e]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::start_executing_work<rustc_codegen_llvm[be3cffe250350432]::LlvmCodegenBackend>::{closure#4}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#0}, core[99b05a1f48df5ac]::result::Result<rustc_codegen_ssa[92d2f24577e8b99e]::back::write::CompiledModules, ()>>::{closure#1} as core[99b05a1f48df5ac]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  16:     0x7f2c3b4537c3 - std::sys::unix::thread::Thread::new::thread_start::h623e57fa0ec4b1c2
  17:     0x7f2c357c2609 - start_thread
  18:     0x7f2c3b2b9293 - clone
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  19:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (740609861 2022-02-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
error: aborting due to previous error

