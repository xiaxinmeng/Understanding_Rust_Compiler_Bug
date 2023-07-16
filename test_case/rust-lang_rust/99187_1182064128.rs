plain

---- [ui] src/test/ui/invalid/invalid-llvm-passes.rs stdout ----
diff of stderr:

1 error: failed to run LLVM passes: unknown pass name 'unknown-pass'
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', $COMPILER_DIR/rustc_codegen_ssa/src/back/write.rs:1494:21
+ stack backtrace:
+    0:     0xffff793c187c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf6d3e5233ab5c32a
+    1:     0xffff7941e934 - core::fmt::write::h4e1a2f76f8e5c93a
+    2:     0xffff793b3d98 - std::io::Write::write_fmt::h797dabc6f9926b87
+    3:     0xffff793c46ec - std::panicking::default_hook::{{closure}}::h146a12c772b3b6db
+    4:     0xffff793c43e8 - std::panicking::default_hook::h40be46032d14cd3d
+    5:     0xffff7a3a6954 - rustc_driver[d444e7272f607202]::DEFAULT_HOOK::{closure#0}::{closure#0}
+    6:     0xffff793c4ea4 - std::panicking::rust_panic_with_hook::hc44eefe15148d043
+    7:     0xffff793c4cec - std::panicking::begin_panic_handler::{{closure}}::h2bedc385e7459238
+    8:     0xffff793c1dc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h1e18079ce2f9a15f
+    9:     0xffff793c49ec - rust_begin_unwind
+   10:     0xffff7938160c - core::panicking::panic_fmt::h6750ff8f7796de68
+   11:     0xffff7941b3e0 - core::panicking::assert_failed_inner::h5287120d772ad7c2
+   12:     0xffff7a14fac0 - core[ae2479b836263677]::panicking::assert_failed::<rustc_codegen_ssa[367127703d545860]::back::write::MainThreadWorkerState, rustc_codegen_ssa[367127703d545860]::back::write::MainThreadWorkerState>
+   13:     0xffff7a61e840 - rustc_codegen_ssa[367127703d545860]::back::write::start_executing_work::<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend>::{closure#4}
+   14:     0xffff7a61710c - std[ec9d766acf43f5c0]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend as rustc_codegen_ssa[367127703d545860]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[367127703d545860]::back::write::start_executing_work<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend>::{closure#4}, core[ae2479b836263677]::result::Result<rustc_codegen_ssa[367127703d545860]::back::write::CompiledModules, ()>>::{closure#0}, core[ae2479b836263677]::result::Result<rustc_codegen_ssa[367127703d545860]::back::write::CompiledModules, ()>>
+   15:     0xffff7a774f2c - <<std[ec9d766acf43f5c0]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend as rustc_codegen_ssa[367127703d545860]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[367127703d545860]::back::write::start_executing_work<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend>::{closure#4}, core[ae2479b836263677]::result::Result<rustc_codegen_ssa[367127703d545860]::back::write::CompiledModules, ()>>::{closure#0}, core[ae2479b836263677]::result::Result<rustc_codegen_ssa[367127703d545860]::back::write::CompiledModules, ()>>::{closure#1} as core[ae2479b836263677]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   16:     0xffff793cfc38 - std::sys::unix::thread::Thread::new::thread_start::h1843d9ec46b614d1
+   17:     0xffff7915e624 - start_thread
+   18:     0xffff7926d49c - <unknown>
+   19:                0x0 - <unknown>
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.64.0-nightly (ada51e608 2022-07-12) running on aarch64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C passes=unknown-pass -Z new-llvm-pass-manager=yes
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args invalid/invalid-llvm-passes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid/invalid-llvm-passes.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/invalid/invalid-llvm-passes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Cpasses=unknown-pass" "-Z" "new-llvm-pass-manager=yes" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/invalid/invalid-llvm-passes/auxiliary"
stdout: none
--- stderr -------------------------------
error: failed to run LLVM passes: unknown pass name 'unknown-pass'
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1494:21
stack backtrace:
   0:     0xffff793c187c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf6d3e5233ab5c32a
   1:     0xffff7941e934 - core::fmt::write::h4e1a2f76f8e5c93a
   2:     0xffff793b3d98 - std::io::Write::write_fmt::h797dabc6f9926b87
   3:     0xffff793c46ec - std::panicking::default_hook::{{closure}}::h146a12c772b3b6db
   4:     0xffff793c43e8 - std::panicking::default_hook::h40be46032d14cd3d
   5:     0xffff7a3a6954 - rustc_driver[d444e7272f607202]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0xffff793c4ea4 - std::panicking::rust_panic_with_hook::hc44eefe15148d043
   7:     0xffff793c4cec - std::panicking::begin_panic_handler::{{closure}}::h2bedc385e7459238
   8:     0xffff793c1dc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h1e18079ce2f9a15f
   9:     0xffff793c49ec - rust_begin_unwind
  10:     0xffff7938160c - core::panicking::panic_fmt::h6750ff8f7796de68
  11:     0xffff7941b3e0 - core::panicking::assert_failed_inner::h5287120d772ad7c2
  12:     0xffff7a14fac0 - core[ae2479b836263677]::panicking::assert_failed::<rustc_codegen_ssa[367127703d545860]::back::write::MainThreadWorkerState, rustc_codegen_ssa[367127703d545860]::back::write::MainThreadWorkerState>
  13:     0xffff7a61e840 - rustc_codegen_ssa[367127703d545860]::back::write::start_executing_work::<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend>::{closure#4}
  14:     0xffff7a61710c - std[ec9d766acf43f5c0]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend as rustc_codegen_ssa[367127703d545860]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[367127703d545860]::back::write::start_executing_work<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend>::{closure#4}, core[ae2479b836263677]::result::Result<rustc_codegen_ssa[367127703d545860]::back::write::CompiledModules, ()>>::{closure#0}, core[ae2479b836263677]::result::Result<rustc_codegen_ssa[367127703d545860]::back::write::CompiledModules, ()>>
  15:     0xffff7a774f2c - <<std[ec9d766acf43f5c0]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend as rustc_codegen_ssa[367127703d545860]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[367127703d545860]::back::write::start_executing_work<rustc_codegen_llvm[5f50436ea3ed4deb]::LlvmCodegenBackend>::{closure#4}, core[ae2479b836263677]::result::Result<rustc_codegen_ssa[367127703d545860]::back::write::CompiledModules, ()>>::{closure#0}, core[ae2479b836263677]::result::Result<rustc_codegen_ssa[367127703d545860]::back::write::CompiledModules, ()>>::{closure#1} as core[ae2479b836263677]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  16:     0xffff793cfc38 - std::sys::unix::thread::Thread::new::thread_start::h1843d9ec46b614d1
  17:     0xffff7915e624 - start_thread
  18:     0xffff7926d49c - <unknown>
  19:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (ada51e608 2022-07-12) running on aarch64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C passes=unknown-pass -Z new-llvm-pass-manager=yes
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
