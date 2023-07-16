plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMEd7XwPIBal
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---

---- [ui] src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs stdout ----
diff of stderr:

1 error: <unknown>:0:0: in function test i32 (i32, i32, i32, i32, i32): call to non-secure function would require passing arguments on stack
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', $COMPILER_DIR/rustc_codegen_ssa/src/back/write.rs:1494:21
+ stack backtrace:
+    0:        0x108fce904 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h61c58d4df24737b8
+    1:        0x109025c1b - core::fmt::write::h2f483c4d4388ea02
+    2:        0x108fc0f9c - std::io::Write::write_fmt::h2eedb57e7da3f2c4
+    3:        0x108fd1937 - std::panicking::default_hook::{{closure}}::h1df7a6c2cd565b88
+    4:        0x108fd1640 - std::panicking::default_hook::he0b569e6d222cabe
+    5:        0x10cd5b70a - rustc_driver[c7d744f0c5661564]::DEFAULT_HOOK::{closure#0}::{closure#0}
+    6:        0x108fd206d - std::panicking::rust_panic_with_hook::h37e4f4ad0e5f549d
+    7:        0x108fd1f13 - std::panicking::begin_panic_handler::{{closure}}::hd7aa257dedfe4195
+    8:        0x108fcedb7 - std::sys_common::backtrace::__rust_end_short_backtrace::heeee4f32e0c89f4b
+    9:        0x108fd1bed - _rust_begin_unwind
+   10:        0x109050e53 - core::panicking::panic_fmt::hedf0f135639938a8
+   11:        0x1090226d7 - core::panicking::assert_failed_inner::h1ff5a83524648fdd
+   12:        0x111679cbe - core[8d8c3749f58b3e93]::panicking::assert_failed::<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::MainThreadWorkerState, rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::MainThreadWorkerState>
+   13:        0x10cfc538c - std[b14a812ba5aaedfc]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[aeee86f5e2f4756e]::LlvmCodegenBackend as rustc_codegen_ssa[a5db3e433d49dbc1]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::start_executing_work<rustc_codegen_llvm[aeee86f5e2f4756e]::LlvmCodegenBackend>::{closure#4}, core[8d8c3749f58b3e93]::result::Result<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::CompiledModules, ()>>::{closure#0}, core[8d8c3749f58b3e93]::result::Result<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::CompiledModules, ()>>
+   14:        0x10cf9054c - <<std[b14a812ba5aaedfc]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[aeee86f5e2f4756e]::LlvmCodegenBackend as rustc_codegen_ssa[a5db3e433d49dbc1]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::start_executing_work<rustc_codegen_llvm[aeee86f5e2f4756e]::LlvmCodegenBackend>::{closure#4}, core[8d8c3749f58b3e93]::result::Result<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::CompiledModules, ()>>::{closure#0}, core[8d8c3749f58b3e93]::result::Result<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::CompiledModules, ()>>::{closure#1} as core[8d8c3749f58b3e93]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   15:        0x108fdac87 - std::sys::unix::thread::Thread::new::thread_start::hb51135fe0bcf0f8b
+   16:     0x7fff205f18fc - __pthread_start
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.64.0-nightly (0bbb9c2ab 2022-07-08) running on x86_64-apple-darwin
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack/auxiliary"
stdout: none
--- stderr -------------------------------
error: <unknown>:0:0: in function test i32 (i32, i32, i32, i32, i32): call to non-secure function would require passing arguments on stack
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /Users/runner/work/rust/rust/compiler/rustc_codegen_ssa/src/back/write.rs:1494:21
stack backtrace:
   0:        0x108fce904 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h61c58d4df24737b8
   1:        0x109025c1b - core::fmt::write::h2f483c4d4388ea02
   2:        0x108fc0f9c - std::io::Write::write_fmt::h2eedb57e7da3f2c4
   3:        0x108fd1937 - std::panicking::default_hook::{{closure}}::h1df7a6c2cd565b88
   4:        0x108fd1640 - std::panicking::default_hook::he0b569e6d222cabe
   5:        0x10cd5b70a - rustc_driver[c7d744f0c5661564]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:        0x108fd206d - std::panicking::rust_panic_with_hook::h37e4f4ad0e5f549d
   7:        0x108fd1f13 - std::panicking::begin_panic_handler::{{closure}}::hd7aa257dedfe4195
   8:        0x108fcedb7 - std::sys_common::backtrace::__rust_end_short_backtrace::heeee4f32e0c89f4b
   9:        0x108fd1bed - _rust_begin_unwind
  10:        0x109050e53 - core::panicking::panic_fmt::hedf0f135639938a8
  11:        0x1090226d7 - core::panicking::assert_failed_inner::h1ff5a83524648fdd
  12:        0x111679cbe - core[8d8c3749f58b3e93]::panicking::assert_failed::<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::MainThreadWorkerState, rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::MainThreadWorkerState>
  13:        0x10cfc538c - std[b14a812ba5aaedfc]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[aeee86f5e2f4756e]::LlvmCodegenBackend as rustc_codegen_ssa[a5db3e433d49dbc1]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::start_executing_work<rustc_codegen_llvm[aeee86f5e2f4756e]::LlvmCodegenBackend>::{closure#4}, core[8d8c3749f58b3e93]::result::Result<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::CompiledModules, ()>>::{closure#0}, core[8d8c3749f58b3e93]::result::Result<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::CompiledModules, ()>>
  14:        0x10cf9054c - <<std[b14a812ba5aaedfc]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[aeee86f5e2f4756e]::LlvmCodegenBackend as rustc_codegen_ssa[a5db3e433d49dbc1]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::start_executing_work<rustc_codegen_llvm[aeee86f5e2f4756e]::LlvmCodegenBackend>::{closure#4}, core[8d8c3749f58b3e93]::result::Result<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::CompiledModules, ()>>::{closure#0}, core[8d8c3749f58b3e93]::result::Result<rustc_codegen_ssa[a5db3e433d49dbc1]::back::write::CompiledModules, ()>>::{closure#1} as core[8d8c3749f58b3e93]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  15:        0x108fdac87 - std::sys::unix::thread::Thread::new::thread_start::hb51135fe0bcf0f8b
  16:     0x7fff205f18fc - __pthread_start
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (0bbb9c2ab 2022-07-08) running on x86_64-apple-darwin

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
