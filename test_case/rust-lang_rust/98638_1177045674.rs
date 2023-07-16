plain

---- [ui] src/test/ui/invalid/invalid-llvm-passes.rs stdout ----
diff of stderr:

1 error: failed to run LLVM passes: unknown pass name 'unknown-pass'
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', $COMPILER_DIR/rustc_codegen_ssa/src/back/write.rs:1467:21
+    0:     0x7f1d09983000 - std::backtrace_rs::backtrace::libunwind::trace::h8414809a63f785cb
+                                at $SRC_DIR/std/src/../../backtrace/src/backtrace/mod.rs:LL:COL
+    1:     0x7f1d09983000 - std::backtrace_rs::backtrace::trace_unsynchronized::hbb276a73b2551f27
+                                at $SRC_DIR/std/src/../../backtrace/src/backtrace/mod.rs:LL:COL
+                                at $SRC_DIR/std/src/../../backtrace/src/backtrace/mod.rs:LL:COL
+    2:     0x7f1d09983000 - std::sys_common::backtrace::_print_fmt::hc58d84a65520ee78
+                                at $SRC_DIR/std/src/sys_common/backtrace.rs:LL:COL
+    3:     0x7f1d09983000 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf411b81d23da6379
+                                at $SRC_DIR/std/src/sys_common/backtrace.rs:LL:COL
+    4:     0x7f1d099dbcbc - core::fmt::write::hc9390b9d841fadfb
+                                at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+    5:     0x7f1d099748a5 - std::io::Write::write_fmt::hbf6048e30fe00cee
+                                at $SRC_DIR/std/src/io/mod.rs:LL:COL
+    6:     0x7f1d09985bf1 - std::sys_common::backtrace::_print::heb93f7770ccc092a
+                                at $SRC_DIR/std/src/sys_common/backtrace.rs:LL:COL
+    7:     0x7f1d09985bf1 - std::sys_common::backtrace::print::h49cd50d46dcdabd1
+                                at $SRC_DIR/std/src/sys_common/backtrace.rs:LL:COL
+    8:     0x7f1d09985bf1 - std::panicking::default_hook::{{closure}}::h2065e6b06787531a
+                                at $SRC_DIR/std/src/panicking.rs:LL:COL
+    9:     0x7f1d099858c3 - std::panicking::default_hook::hf4e733c3b40ff74a
+                                at $SRC_DIR/std/src/panicking.rs:LL:COL
+   10:     0x7f1d0a874bf6 - rustc_driver[7715d818b6b0e8df]::DEFAULT_HOOK::{closure#0}::{closure#0}
+   11:     0x7f1d099863c6 - std::panicking::rust_panic_with_hook::h7a01ba63a552e65b
+                                at $SRC_DIR/std/src/panicking.rs:LL:COL
+   12:     0x7f1d09986217 - std::panicking::begin_panic_handler::{{closure}}::h4f918d6844b6b067
+                                at $SRC_DIR/std/src/panicking.rs:LL:COL
+   13:     0x7f1d099834d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd208f629f3092398
+                                at $SRC_DIR/std/src/sys_common/backtrace.rs:LL:COL
+   14:     0x7f1d09985f42 - rust_begin_unwind
+                                at $SRC_DIR/std/src/panicking.rs:LL:COL
+   15:     0x7f1d0994ae03 - core::panicking::panic_fmt::hccbb92404164eb30
+                                at $SRC_DIR/core/src/panicking.rs:LL:COL
+   16:     0x7f1d099d8a18 - core::panicking::assert_failed_inner::h54d35828061e4a63
+   17:     0x7f1d0a57095b - core[e8f8ec297493aa9f]::panicking::assert_failed::<rustc_codegen_ssa[54c10ff5414869a7]::back::write::MainThreadWorkerState, rustc_codegen_ssa[54c10ff5414869a7]::back::write::MainThreadWorkerState>
+   18:     0x7f1d0ab48aad - std[74e7685dc83ab0f4]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[fd2cbb62e2ac9eef]::LlvmCodegenBackend as rustc_codegen_ssa[54c10ff5414869a7]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[54c10ff5414869a7]::back::write::start_executing_work<rustc_codegen_llvm[fd2cbb62e2ac9eef]::LlvmCodegenBackend>::{closure#4}, core[e8f8ec297493aa9f]::result::Result<rustc_codegen_ssa[54c10ff5414869a7]::back::write::CompiledModules, ()>>::{closure#0}, core[e8f8ec297493aa9f]::result::Result<rustc_codegen_ssa[54c10ff5414869a7]::back::write::CompiledModules, ()>>
+   19:     0x7f1d0aaeefa9 - <<std[74e7685dc83ab0f4]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[fd2cbb62e2ac9eef]::LlvmCodegenBackend as rustc_codegen_ssa[54c10ff5414869a7]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[54c10ff5414869a7]::back::write::start_executing_work<rustc_codegen_llvm[fd2cbb62e2ac9eef]::LlvmCodegenBackend>::{closure#4}, core[e8f8ec297493aa9f]::result::Result<rustc_codegen_ssa[54c10ff5414869a7]::back::write::CompiledModules, ()>>::{closure#0}, core[e8f8ec297493aa9f]::result::Result<rustc_codegen_ssa[54c10ff5414869a7]::back::write::CompiledModules, ()>>::{closure#1} as core[e8f8ec297493aa9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   20:     0x7f1d0998fd23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1b33063481ea261e
+                                at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+   21:     0x7f1d0998fd23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h492c0b3808aa6969
+                                at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+   22:     0x7f1d0998fd23 - std::sys::unix::thread::Thread::new::thread_start::h752f1da4a10216bc
+                                at $SRC_DIR/std/src/sys/unix/thread.rs:LL:COL
+   23:     0x7f1d08ef96ba - start_thread
+   24:     0x7f1d0962251d - clone
+   25:                0x0 - <unknown>
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.64.0-nightly (2808ff66b 2022-07-07) running on x86_64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C linker=cc -C passes=unknown-pass -Z new-llvm-pass-manager=yes
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args invalid/invalid-llvm-passes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid/invalid-llvm-passes.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-llvm-passes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-Cpasses=unknown-pass" "-Z" "new-llvm-pass-manager=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-llvm-passes/auxiliary"
stdout: none
--- stderr -------------------------------
error: failed to run LLVM passes: unknown pass name 'unknown-pass'
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/compiler/rustc_codegen_ssa/src/back/write.rs:1467:21
   0:     0x7f1d09983000 - std::backtrace_rs::backtrace::libunwind::trace::h8414809a63f785cb
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f1d09983000 - std::backtrace_rs::backtrace::trace_unsynchronized::hbb276a73b2551f27
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f1d09983000 - std::sys_common::backtrace::_print_fmt::hc58d84a65520ee78
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f1d09983000 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf411b81d23da6379
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f1d099dbcbc - core::fmt::write::hc9390b9d841fadfb
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f1d099748a5 - std::io::Write::write_fmt::hbf6048e30fe00cee
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/io/mod.rs:1672:15
   6:     0x7f1d09985bf1 - std::sys_common::backtrace::_print::heb93f7770ccc092a
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f1d09985bf1 - std::sys_common::backtrace::print::h49cd50d46dcdabd1
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f1d09985bf1 - std::panicking::default_hook::{{closure}}::h2065e6b06787531a
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/panicking.rs:295:22
   9:     0x7f1d099858c3 - std::panicking::default_hook::hf4e733c3b40ff74a
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/panicking.rs:314:9
  10:     0x7f1d0a874bf6 - rustc_driver[7715d818b6b0e8df]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f1d099863c6 - std::panicking::rust_panic_with_hook::h7a01ba63a552e65b
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/panicking.rs:702:17
  12:     0x7f1d09986217 - std::panicking::begin_panic_handler::{{closure}}::h4f918d6844b6b067
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/panicking.rs:588:13
  13:     0x7f1d099834d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd208f629f3092398
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f1d09985f42 - rust_begin_unwind
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/panicking.rs:584:5
  15:     0x7f1d0994ae03 - core::panicking::panic_fmt::hccbb92404164eb30
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/core/src/panicking.rs:142:14
  16:     0x7f1d099d8a18 - core::panicking::assert_failed_inner::h54d35828061e4a63
  17:     0x7f1d0a57095b - core[e8f8ec297493aa9f]::panicking::assert_failed::<rustc_codegen_ssa[54c10ff5414869a7]::back::write::MainThreadWorkerState, rustc_codegen_ssa[54c10ff5414869a7]::back::write::MainThreadWorkerState>
  18:     0x7f1d0ab48aad - std[74e7685dc83ab0f4]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[fd2cbb62e2ac9eef]::LlvmCodegenBackend as rustc_codegen_ssa[54c10ff5414869a7]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[54c10ff5414869a7]::back::write::start_executing_work<rustc_codegen_llvm[fd2cbb62e2ac9eef]::LlvmCodegenBackend>::{closure#4}, core[e8f8ec297493aa9f]::result::Result<rustc_codegen_ssa[54c10ff5414869a7]::back::write::CompiledModules, ()>>::{closure#0}, core[e8f8ec297493aa9f]::result::Result<rustc_codegen_ssa[54c10ff5414869a7]::back::write::CompiledModules, ()>>
  19:     0x7f1d0aaeefa9 - <<std[74e7685dc83ab0f4]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[fd2cbb62e2ac9eef]::LlvmCodegenBackend as rustc_codegen_ssa[54c10ff5414869a7]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[54c10ff5414869a7]::back::write::start_executing_work<rustc_codegen_llvm[fd2cbb62e2ac9eef]::LlvmCodegenBackend>::{closure#4}, core[e8f8ec297493aa9f]::result::Result<rustc_codegen_ssa[54c10ff5414869a7]::back::write::CompiledModules, ()>>::{closure#0}, core[e8f8ec297493aa9f]::result::Result<rustc_codegen_ssa[54c10ff5414869a7]::back::write::CompiledModules, ()>>::{closure#1} as core[e8f8ec297493aa9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f1d0998fd23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1b33063481ea261e
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/alloc/src/boxed.rs:1934:9
  21:     0x7f1d0998fd23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h492c0b3808aa6969
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/alloc/src/boxed.rs:1934:9
  22:     0x7f1d0998fd23 - std::sys::unix::thread::Thread::new::thread_start::h752f1da4a10216bc
                               at /rustc/2808ff66b08f8095a220aa2c7c733857803be5b2/library/std/src/sys/unix/thread.rs:108:17
  23:     0x7f1d08ef96ba - start_thread
  24:     0x7f1d0962251d - clone
  25:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2808ff66b 2022-07-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C linker=cc -C passes=unknown-pass -Z new-llvm-pass-manager=yes
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
