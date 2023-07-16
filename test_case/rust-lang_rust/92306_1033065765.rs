plain
test [ui] ui/nll/relate_tys/fn-subtype.rs ... ok
test [ui] ui/nll/relate_tys/hr-fn-aba-as-aaa.rs ... ok
test [ui] ui/nll/relate_tys/impl-fn-ignore-binder-via-bottom.rs ... ok
test [ui] ui/nll/relate_tys/issue-48071.rs ... ok
test [ui] ui/nll/relate_tys/opaque-hrtb.rs ... ok
test [ui] ui/nll/return-ref-mut-issue-46557.rs ... ok
test [ui] ui/nll/ty-outlives/impl-trait-captures.rs ... ok
test [ui] ui/nll/return_from_loop.rs ... ok
test [ui] ui/nll/relate_tys/trait-hrtb.rs ... ok
---

---- [ui] ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs stdout ----
diff of stderr:

1 error: <unknown>:0:0: in function test i32 (i32, i32, i32, i32, i32): call to non-secure function would require passing arguments on stack
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1470:21
+    0:     0x7feb3677b85c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h10995e31cbc01166
+    1:     0x7feb367e9cce - core::fmt::write::hd7d6faa80ce2553b
+    1:     0x7feb367e9cce - core::fmt::write::hd7d6faa80ce2553b
+    2:     0x7feb3676a771 - std::io::Write::write_fmt::h3f5abf7b9f1eca07
+    3:     0x7feb3677b68b - std::sys_common::backtrace::print::h4d653406e7d402ad
+    4:     0x7feb3677fea4 - std::panicking::default_hook::{{closure}}::h73e6d85c2226b99a
+    5:     0x7feb3677fa86 - std::panicking::default_hook::h578dc98dd31a3735
+    6:     0x7feb378f087a - rustc_driver[52bb940b4ffe0591]::DEFAULT_HOOK::{closure#0}::{closure#0}
+    7:     0x7feb367805c3 - std::panicking::rust_panic_with_hook::hf831cb87f6160c47
+    8:     0x7feb367803d7 - std::panicking::begin_panic_handler::{{closure}}::h3962d95fa5d93909
+    9:     0x7feb3677bd74 - std::sys_common::backtrace::__rust_end_short_backtrace::hbbae4df880a75e04
+   10:     0x7feb36780099 - rust_begin_unwind
+   11:     0x7feb36736a53 - core::panicking::panic_fmt::h2cba5c8061a950b9
+   12:     0x7feb367e6658 - core::panicking::assert_failed_inner::h19ad8376367d71e3
+   13:     0x7feb373fb93b - core[ce52307a21260d31]::panicking::assert_failed::<rustc_codegen_ssa[f59136af200a9e6b]::back::write::MainThreadWorkerState, rustc_codegen_ssa[f59136af200a9e6b]::back::write::MainThreadWorkerState>
+   14:     0x7feb37c7df5e - rustc_codegen_ssa[f59136af200a9e6b]::back::write::start_executing_work::<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend>::{closure#4}
+   15:     0x7feb37c9a646 - std[962aeb78bac5d12c]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend as rustc_codegen_ssa[f59136af200a9e6b]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[f59136af200a9e6b]::back::write::start_executing_work<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend>::{closure#4}, core[ce52307a21260d31]::result::Result<rustc_codegen_ssa[f59136af200a9e6b]::back::write::CompiledModules, ()>>::{closure#0}, core[ce52307a21260d31]::result::Result<rustc_codegen_ssa[f59136af200a9e6b]::back::write::CompiledModules, ()>>
+   16:     0x7feb37c77a39 - <<std[962aeb78bac5d12c]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend as rustc_codegen_ssa[f59136af200a9e6b]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[f59136af200a9e6b]::back::write::start_executing_work<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend>::{closure#4}, core[ce52307a21260d31]::result::Result<rustc_codegen_ssa[f59136af200a9e6b]::back::write::CompiledModules, ()>>::{closure#0}, core[ce52307a21260d31]::result::Result<rustc_codegen_ssa[f59136af200a9e6b]::back::write::CompiledModules, ()>>::{closure#1} as core[ce52307a21260d31]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   17:     0x7feb3678f3a3 - std::sys::unix::thread::Thread::new::thread_start::h631d624327c9afc0
+   18:     0x7feb364b8609 - start_thread
+   19:     0x7feb365fa293 - clone
+   20:                0x0 - <unknown>
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.60.0-nightly (27ee5ca00 2022-02-08) running on x86_64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack/auxiliary"
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: <unknown>:0:0: in function test i32 (i32, i32, i32, i32, i32): call to non-secure function would require passing arguments on stack
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1470:21
   0:     0x7feb3677b85c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h10995e31cbc01166
   1:     0x7feb367e9cce - core::fmt::write::hd7d6faa80ce2553b
   1:     0x7feb367e9cce - core::fmt::write::hd7d6faa80ce2553b
   2:     0x7feb3676a771 - std::io::Write::write_fmt::h3f5abf7b9f1eca07
   3:     0x7feb3677b68b - std::sys_common::backtrace::print::h4d653406e7d402ad
   4:     0x7feb3677fea4 - std::panicking::default_hook::{{closure}}::h73e6d85c2226b99a
   5:     0x7feb3677fa86 - std::panicking::default_hook::h578dc98dd31a3735
   6:     0x7feb378f087a - rustc_driver[52bb940b4ffe0591]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7feb367805c3 - std::panicking::rust_panic_with_hook::hf831cb87f6160c47
   8:     0x7feb367803d7 - std::panicking::begin_panic_handler::{{closure}}::h3962d95fa5d93909
   9:     0x7feb3677bd74 - std::sys_common::backtrace::__rust_end_short_backtrace::hbbae4df880a75e04
  10:     0x7feb36780099 - rust_begin_unwind
  11:     0x7feb36736a53 - core::panicking::panic_fmt::h2cba5c8061a950b9
  12:     0x7feb367e6658 - core::panicking::assert_failed_inner::h19ad8376367d71e3
  13:     0x7feb373fb93b - core[ce52307a21260d31]::panicking::assert_failed::<rustc_codegen_ssa[f59136af200a9e6b]::back::write::MainThreadWorkerState, rustc_codegen_ssa[f59136af200a9e6b]::back::write::MainThreadWorkerState>
  14:     0x7feb37c7df5e - rustc_codegen_ssa[f59136af200a9e6b]::back::write::start_executing_work::<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend>::{closure#4}
  15:     0x7feb37c9a646 - std[962aeb78bac5d12c]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend as rustc_codegen_ssa[f59136af200a9e6b]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[f59136af200a9e6b]::back::write::start_executing_work<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend>::{closure#4}, core[ce52307a21260d31]::result::Result<rustc_codegen_ssa[f59136af200a9e6b]::back::write::CompiledModules, ()>>::{closure#0}, core[ce52307a21260d31]::result::Result<rustc_codegen_ssa[f59136af200a9e6b]::back::write::CompiledModules, ()>>
  16:     0x7feb37c77a39 - <<std[962aeb78bac5d12c]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend as rustc_codegen_ssa[f59136af200a9e6b]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[f59136af200a9e6b]::back::write::start_executing_work<rustc_codegen_llvm[96c54cfd82f181b2]::LlvmCodegenBackend>::{closure#4}, core[ce52307a21260d31]::result::Result<rustc_codegen_ssa[f59136af200a9e6b]::back::write::CompiledModules, ()>>::{closure#0}, core[ce52307a21260d31]::result::Result<rustc_codegen_ssa[f59136af200a9e6b]::back::write::CompiledModules, ()>>::{closure#1} as core[ce52307a21260d31]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  17:     0x7feb3678f3a3 - std::sys::unix::thread::Thread::new::thread_start::h631d624327c9afc0
  18:     0x7feb364b8609 - start_thread
  19:     0x7feb365fa293 - clone
  20:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (27ee5ca00 2022-02-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
error: aborting due to previous error

