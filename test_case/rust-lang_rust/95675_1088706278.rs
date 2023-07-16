plain

---- [ui] ui/invalid/invalid-llvm-passes.rs stdout ----
diff of stderr:

1 error: failed to run LLVM passes: unknown pass name 'unknown-pass'
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1471:21
+    0:     0x7f3d7fce478d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h59ae1a7b22004c8c
+    0:     0x7f3d7fce478d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h59ae1a7b22004c8c
+    1:     0x7f3d7fd47a38 - core::fmt::write::ha7e7cf912fe9539a
+    2:     0x7f3d7fcd5711 - std::io::Write::write_fmt::h5510ee268d07df7b
+    3:     0x7f3d7fce45ab - std::sys_common::backtrace::print::h08bb60edf9be83f1
+    4:     0x7f3d7fce7f24 - std::panicking::default_hook::{{closure}}::h3ffc2d123ec2b8d5
+    5:     0x7f3d7fce7b9c - std::panicking::default_hook::h2e979a8c86f0f0bf
+    6:     0x7f3d80e65a1a - rustc_driver[68b56281aac6e522]::DEFAULT_HOOK::{closure#0}::{closure#0}
+    7:     0x7f3d7fce85fa - std::panicking::rust_panic_with_hook::h6d85312f2998bd9b
+    8:     0x7f3d7fce8427 - std::panicking::begin_panic_handler::{{closure}}::h656f29de7596b1f2
+    9:     0x7f3d7fce4cc4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd163467b5b5c7a90
+   10:     0x7f3d7fce8119 - rust_begin_unwind
+   11:     0x7f3d7fc9cbb3 - core::panicking::panic_fmt::hb65d42c9d8beea97
+   12:     0x7f3d7fd444e8 - core::panicking::assert_failed_inner::hb5765767a3717cf6
+   13:     0x7f3d809d85fb - core[7aa401e17aad33d1]::panicking::assert_failed::<rustc_codegen_ssa[9a695caeb5983c6]::back::write::MainThreadWorkerState, rustc_codegen_ssa[9a695caeb5983c6]::back::write::MainThreadWorkerState>
+   14:     0x7f3d8105f7db - rustc_codegen_ssa[9a695caeb5983c6]::back::write::start_executing_work::<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend>::{closure#4}
+   15:     0x7f3d8105adc6 - std[a46efefb3fd5620c]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend as rustc_codegen_ssa[9a695caeb5983c6]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[9a695caeb5983c6]::back::write::start_executing_work<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend>::{closure#4}, core[7aa401e17aad33d1]::result::Result<rustc_codegen_ssa[9a695caeb5983c6]::back::write::CompiledModules, ()>>::{closure#0}, core[7aa401e17aad33d1]::result::Result<rustc_codegen_ssa[9a695caeb5983c6]::back::write::CompiledModules, ()>>
+   16:     0x7f3d811d3749 - <<std[a46efefb3fd5620c]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend as rustc_codegen_ssa[9a695caeb5983c6]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[9a695caeb5983c6]::back::write::start_executing_work<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend>::{closure#4}, core[7aa401e17aad33d1]::result::Result<rustc_codegen_ssa[9a695caeb5983c6]::back::write::CompiledModules, ()>>::{closure#0}, core[7aa401e17aad33d1]::result::Result<rustc_codegen_ssa[9a695caeb5983c6]::back::write::CompiledModules, ()>>::{closure#1} as core[7aa401e17aad33d1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   17:     0x7f3d7fcf4503 - std::sys::unix::thread::Thread::new::thread_start::h0e3ab9d360f09880
+   18:     0x7f3d7fa1c609 - start_thread
+   19:     0x7f3d7fb5c163 - clone
+   20:                0x0 - <unknown>
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.62.0-nightly (8537ba5b8 2022-04-05) running on x86_64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -C passes=unknown-pass -Z new-llvm-pass-manager=yes
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args invalid/invalid-llvm-passes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid/invalid-llvm-passes.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-llvm-passes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Cpasses=unknown-pass" "-Z" "new-llvm-pass-manager=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-llvm-passes/auxiliary"
stdout: none
--- stderr -------------------------------
error: failed to run LLVM passes: unknown pass name 'unknown-pass'
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1471:21
   0:     0x7f3d7fce478d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h59ae1a7b22004c8c
   0:     0x7f3d7fce478d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h59ae1a7b22004c8c
   1:     0x7f3d7fd47a38 - core::fmt::write::ha7e7cf912fe9539a
   2:     0x7f3d7fcd5711 - std::io::Write::write_fmt::h5510ee268d07df7b
   3:     0x7f3d7fce45ab - std::sys_common::backtrace::print::h08bb60edf9be83f1
   4:     0x7f3d7fce7f24 - std::panicking::default_hook::{{closure}}::h3ffc2d123ec2b8d5
   5:     0x7f3d7fce7b9c - std::panicking::default_hook::h2e979a8c86f0f0bf
   6:     0x7f3d80e65a1a - rustc_driver[68b56281aac6e522]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3d7fce85fa - std::panicking::rust_panic_with_hook::h6d85312f2998bd9b
   8:     0x7f3d7fce8427 - std::panicking::begin_panic_handler::{{closure}}::h656f29de7596b1f2
   9:     0x7f3d7fce4cc4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd163467b5b5c7a90
  10:     0x7f3d7fce8119 - rust_begin_unwind
  11:     0x7f3d7fc9cbb3 - core::panicking::panic_fmt::hb65d42c9d8beea97
  12:     0x7f3d7fd444e8 - core::panicking::assert_failed_inner::hb5765767a3717cf6
  13:     0x7f3d809d85fb - core[7aa401e17aad33d1]::panicking::assert_failed::<rustc_codegen_ssa[9a695caeb5983c6]::back::write::MainThreadWorkerState, rustc_codegen_ssa[9a695caeb5983c6]::back::write::MainThreadWorkerState>
  14:     0x7f3d8105f7db - rustc_codegen_ssa[9a695caeb5983c6]::back::write::start_executing_work::<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend>::{closure#4}
  15:     0x7f3d8105adc6 - std[a46efefb3fd5620c]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend as rustc_codegen_ssa[9a695caeb5983c6]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[9a695caeb5983c6]::back::write::start_executing_work<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend>::{closure#4}, core[7aa401e17aad33d1]::result::Result<rustc_codegen_ssa[9a695caeb5983c6]::back::write::CompiledModules, ()>>::{closure#0}, core[7aa401e17aad33d1]::result::Result<rustc_codegen_ssa[9a695caeb5983c6]::back::write::CompiledModules, ()>>
  16:     0x7f3d811d3749 - <<std[a46efefb3fd5620c]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend as rustc_codegen_ssa[9a695caeb5983c6]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[9a695caeb5983c6]::back::write::start_executing_work<rustc_codegen_llvm[36f3397ee907c4b6]::LlvmCodegenBackend>::{closure#4}, core[7aa401e17aad33d1]::result::Result<rustc_codegen_ssa[9a695caeb5983c6]::back::write::CompiledModules, ()>>::{closure#0}, core[7aa401e17aad33d1]::result::Result<rustc_codegen_ssa[9a695caeb5983c6]::back::write::CompiledModules, ()>>::{closure#1} as core[7aa401e17aad33d1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  17:     0x7f3d7fcf4503 - std::sys::unix::thread::Thread::new::thread_start::h0e3ab9d360f09880
  18:     0x7f3d7fa1c609 - start_thread
  19:     0x7f3d7fb5c163 - clone
  20:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (8537ba5b8 2022-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -C passes=unknown-pass -Z new-llvm-pass-manager=yes
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
