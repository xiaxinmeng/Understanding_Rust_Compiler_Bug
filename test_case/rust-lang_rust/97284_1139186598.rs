plain

---- [ui] src/test/ui/invalid/invalid-llvm-passes.rs stdout ----
diff of stderr:

1 error: failed to run LLVM passes: unknown pass name 'unknown-pass'
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', $COMPILER_DIR/rustc_codegen_ssa/src/back/write.rs:1471:21
+    0:     0x7fc5b0a9818d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha6d42905ae8495e1
+    1:     0x7fc5b0af401c - core::fmt::write::he61c0a83568f530b
+    1:     0x7fc5b0af401c - core::fmt::write::he61c0a83568f530b
+    2:     0x7fc5b0a89921 - std::io::Write::write_fmt::hf12b960ff621bcd4
+    3:     0x7fc5b0a9ae75 - std::panicking::default_hook::{{closure}}::h1e32ea7d973577fd
+    4:     0x7fc5b0a9ab96 - std::panicking::default_hook::h0f733119b617cb2f
+    5:     0x7fc5b1872b19 - rustc_driver[956148c2c5381e18]::DEFAULT_HOOK::{closure#0}::{closure#0}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-musl
+    6:     0x7fc5b0a9b54a - std::panicking::rust_panic_with_hook::h10f2619916c4d7f0
+    7:     0x7fc5b0a9b387 - std::panicking::begin_panic_handler::{{closure}}::hc10db1e63006eff3
+    8:     0x7fc5b0a98644 - std::sys_common::backtrace::__rust_end_short_backtrace::h8514961be8260c05
+    9:     0x7fc5b0a9b0b9 - rust_begin_unwind
+   10:     0x7fc5b0a60413 - core::panicking::panic_fmt::h5cbf0cc0bcd5482c
+   11:     0x7fc5b0af0dc8 - core::panicking::assert_failed_inner::h0cd989a79f6863cc
+   12:     0x7fc5b15e43bb - core[b767cee21b229350]::panicking::assert_failed::<rustc_codegen_ssa[b548771e8a124ff8]::back::write::MainThreadWorkerState, rustc_codegen_ssa[b548771e8a124ff8]::back::write::MainThreadWorkerState>
+   13:     0x7fc5b1ada995 - std[8a7407eec8ab889b]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[6af3a2890704cafe]::LlvmCodegenBackend as rustc_codegen_ssa[b548771e8a124ff8]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[b548771e8a124ff8]::back::write::start_executing_work<rustc_codegen_llvm[6af3a2890704cafe]::LlvmCodegenBackend>::{closure#4}, core[b767cee21b229350]::result::Result<rustc_codegen_ssa[b548771e8a124ff8]::back::write::CompiledModules, ()>>::{closure#0}, core[b767cee21b229350]::result::Result<rustc_codegen_ssa[b548771e8a124ff8]::back::write::CompiledModules, ()>>
+   14:     0x7fc5b1b4cfe4 - <<std[8a7407eec8ab889b]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[6af3a2890704cafe]::LlvmCodegenBackend as rustc_codegen_ssa[b548771e8a124ff8]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[b548771e8a124ff8]::back::write::start_executing_work<rustc_codegen_llvm[6af3a2890704cafe]::LlvmCodegenBackend>::{closure#4}, core[b767cee21b229350]::result::Result<rustc_codegen_ssa[b548771e8a124ff8]::back::write::CompiledModules, ()>>::{closure#0}, core[b767cee21b229350]::result::Result<rustc_codegen_ssa[b548771e8a124ff8]::back::write::CompiledModules, ()>>::{closure#1} as core[b767cee21b229350]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   15:     0x7fc5b0aa5463 - std::sys::unix::thread::Thread::new::thread_start::h6cfee43c7d984174
+   16:     0x7fc5b07e4609 - start_thread
+   17:     0x7fc5b0926133 - clone
+   18:                0x0 - <unknown>
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.63.0-nightly (2a0c6ac83 2022-05-27) running on x86_64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -C linker=x86_64-linux-musl-gcc -C passes=unknown-pass -Z new-llvm-pass-manager=yes
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args invalid/invalid-llvm-passes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid/invalid-llvm-passes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-llvm-passes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-Cpasses=unknown-pass" "-Z" "new-llvm-pass-manager=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-llvm-passes/auxiliary"
stdout: none
--- stderr -------------------------------
error: failed to run LLVM passes: unknown pass name 'unknown-pass'
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1471:21
   0:     0x7fc5b0a9818d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha6d42905ae8495e1
   1:     0x7fc5b0af401c - core::fmt::write::he61c0a83568f530b
   1:     0x7fc5b0af401c - core::fmt::write::he61c0a83568f530b
   2:     0x7fc5b0a89921 - std::io::Write::write_fmt::hf12b960ff621bcd4
   3:     0x7fc5b0a9ae75 - std::panicking::default_hook::{{closure}}::h1e32ea7d973577fd
   4:     0x7fc5b0a9ab96 - std::panicking::default_hook::h0f733119b617cb2f
   5:     0x7fc5b1872b19 - rustc_driver[956148c2c5381e18]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc5b0a9b54a - std::panicking::rust_panic_with_hook::h10f2619916c4d7f0
   7:     0x7fc5b0a9b387 - std::panicking::begin_panic_handler::{{closure}}::hc10db1e63006eff3
   8:     0x7fc5b0a98644 - std::sys_common::backtrace::__rust_end_short_backtrace::h8514961be8260c05
   9:     0x7fc5b0a9b0b9 - rust_begin_unwind
  10:     0x7fc5b0a60413 - core::panicking::panic_fmt::h5cbf0cc0bcd5482c
  11:     0x7fc5b0af0dc8 - core::panicking::assert_failed_inner::h0cd989a79f6863cc
  12:     0x7fc5b15e43bb - core[b767cee21b229350]::panicking::assert_failed::<rustc_codegen_ssa[b548771e8a124ff8]::back::write::MainThreadWorkerState, rustc_codegen_ssa[b548771e8a124ff8]::back::write::MainThreadWorkerState>
  13:     0x7fc5b1ada995 - std[8a7407eec8ab889b]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[6af3a2890704cafe]::LlvmCodegenBackend as rustc_codegen_ssa[b548771e8a124ff8]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[b548771e8a124ff8]::back::write::start_executing_work<rustc_codegen_llvm[6af3a2890704cafe]::LlvmCodegenBackend>::{closure#4}, core[b767cee21b229350]::result::Result<rustc_codegen_ssa[b548771e8a124ff8]::back::write::CompiledModules, ()>>::{closure#0}, core[b767cee21b229350]::result::Result<rustc_codegen_ssa[b548771e8a124ff8]::back::write::CompiledModules, ()>>
  14:     0x7fc5b1b4cfe4 - <<std[8a7407eec8ab889b]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[6af3a2890704cafe]::LlvmCodegenBackend as rustc_codegen_ssa[b548771e8a124ff8]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[b548771e8a124ff8]::back::write::start_executing_work<rustc_codegen_llvm[6af3a2890704cafe]::LlvmCodegenBackend>::{closure#4}, core[b767cee21b229350]::result::Result<rustc_codegen_ssa[b548771e8a124ff8]::back::write::CompiledModules, ()>>::{closure#0}, core[b767cee21b229350]::result::Result<rustc_codegen_ssa[b548771e8a124ff8]::back::write::CompiledModules, ()>>::{closure#1} as core[b767cee21b229350]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  15:     0x7fc5b0aa5463 - std::sys::unix::thread::Thread::new::thread_start::h6cfee43c7d984174
  16:     0x7fc5b07e4609 - start_thread
  17:     0x7fc5b0926133 - clone
  18:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (2a0c6ac83 2022-05-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -C linker=x86_64-linux-musl-gcc -C passes=unknown-pass -Z new-llvm-pass-manager=yes
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
