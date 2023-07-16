plain

---- [ui] ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs stdout ----
diff of stderr:

1 error: <unknown>:0:0: in function entry_function i32 (i32, i32, i32, i32, i32): secure entry function requires arguments on stack
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1470:21
+ stack backtrace:
+    0:     0x7f60ef9f8d9d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h64b96de4e99eae29
+    1:     0x7f60efa6616e - core::fmt::write::hc88a696799dc2bbb
+    2:     0x7f60ef9e7c71 - std::io::Write::write_fmt::h2957b944daa153bd
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten
+    3:     0x7f60ef9f8bcb - std::sys_common::backtrace::print::h8a98b73144ee4249
+    4:     0x7f60ef9fd404 - std::panicking::default_hook::{{closure}}::h2be5b9b211d446d5
+    5:     0x7f60ef9fcfe7 - std::panicking::default_hook::h3e9a506db2321b7c
+    6:     0x7f60f0b1e521 - rustc_driver[d81be8bd9ce8ab21]::DEFAULT_HOOK::{closure#0}::{closure#0}
+    7:     0x7f60ef9fdb12 - std::panicking::rust_panic_with_hook::h947745481b02e598
+    8:     0x7f60ef9fd927 - std::panicking::begin_panic_handler::{{closure}}::hed14d0866abe6615
+    9:     0x7f60ef9f92d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h12da648731d41717
+   10:     0x7f60ef9fd5f9 - rust_begin_unwind
+   11:     0x7f60ef9b4a43 - core::panicking::panic_fmt::h6d9cdca55d882252
+   12:     0x7f60efa62b08 - core::panicking::assert_failed_inner::hbc9c05276dc2a6e7
+   13:     0x7f60f06d0924 - core[c2f8edecbf557fa0]::panicking::assert_failed::<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::MainThreadWorkerState, rustc_codegen_ssa[2222b9dda5efcec4]::back::write::MainThreadWorkerState>
+   14:     0x7f60f0f348a6 - std[33f86f8fe5804518]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[ab59ffb0489b5aec]::LlvmCodegenBackend as rustc_codegen_ssa[2222b9dda5efcec4]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::start_executing_work<rustc_codegen_llvm[ab59ffb0489b5aec]::LlvmCodegenBackend>::{closure#4}, core[c2f8edecbf557fa0]::result::Result<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::CompiledModules, ()>>::{closure#0}, core[c2f8edecbf557fa0]::result::Result<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::CompiledModules, ()>>
+   15:     0x7f60f0ef57f9 - <<std[33f86f8fe5804518]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[ab59ffb0489b5aec]::LlvmCodegenBackend as rustc_codegen_ssa[2222b9dda5efcec4]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::start_executing_work<rustc_codegen_llvm[ab59ffb0489b5aec]::LlvmCodegenBackend>::{closure#4}, core[c2f8edecbf557fa0]::result::Result<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::CompiledModules, ()>>::{closure#0}, core[c2f8edecbf557fa0]::result::Result<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::CompiledModules, ()>>::{closure#1} as core[c2f8edecbf557fa0]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   16:     0x7f60efa0cb93 - std::sys::unix::thread::Thread::new::thread_start::hae886abb3c6fe524
+   17:     0x7f60ef734609 - start_thread
+   18:     0x7f60ef874163 - clone
+   19:                0x0 - <unknown>
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.60.0-beta.6 (6c380750e 2022-03-20) running on x86_64-unknown-linux-gnu
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack/auxiliary"
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
   0:     0x7f60ef9f8d9d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h64b96de4e99eae29
   1:     0x7f60efa6616e - core::fmt::write::hc88a696799dc2bbb
   2:     0x7f60ef9e7c71 - std::io::Write::write_fmt::h2957b944daa153bd
   3:     0x7f60ef9f8bcb - std::sys_common::backtrace::print::h8a98b73144ee4249
   4:     0x7f60ef9fd404 - std::panicking::default_hook::{{closure}}::h2be5b9b211d446d5
   5:     0x7f60ef9fcfe7 - std::panicking::default_hook::h3e9a506db2321b7c
   6:     0x7f60f0b1e521 - rustc_driver[d81be8bd9ce8ab21]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f60ef9fdb12 - std::panicking::rust_panic_with_hook::h947745481b02e598
   8:     0x7f60ef9fd927 - std::panicking::begin_panic_handler::{{closure}}::hed14d0866abe6615
   9:     0x7f60ef9f92d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h12da648731d41717
  10:     0x7f60ef9fd5f9 - rust_begin_unwind
  11:     0x7f60ef9b4a43 - core::panicking::panic_fmt::h6d9cdca55d882252
  12:     0x7f60efa62b08 - core::panicking::assert_failed_inner::hbc9c05276dc2a6e7
  13:     0x7f60f06d0924 - core[c2f8edecbf557fa0]::panicking::assert_failed::<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::MainThreadWorkerState, rustc_codegen_ssa[2222b9dda5efcec4]::back::write::MainThreadWorkerState>
  14:     0x7f60f0f348a6 - std[33f86f8fe5804518]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[ab59ffb0489b5aec]::LlvmCodegenBackend as rustc_codegen_ssa[2222b9dda5efcec4]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::start_executing_work<rustc_codegen_llvm[ab59ffb0489b5aec]::LlvmCodegenBackend>::{closure#4}, core[c2f8edecbf557fa0]::result::Result<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::CompiledModules, ()>>::{closure#0}, core[c2f8edecbf557fa0]::result::Result<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::CompiledModules, ()>>
  15:     0x7f60f0ef57f9 - <<std[33f86f8fe5804518]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[ab59ffb0489b5aec]::LlvmCodegenBackend as rustc_codegen_ssa[2222b9dda5efcec4]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::start_executing_work<rustc_codegen_llvm[ab59ffb0489b5aec]::LlvmCodegenBackend>::{closure#4}, core[c2f8edecbf557fa0]::result::Result<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::CompiledModules, ()>>::{closure#0}, core[c2f8edecbf557fa0]::result::Result<rustc_codegen_ssa[2222b9dda5efcec4]::back::write::CompiledModules, ()>>::{closure#1} as core[c2f8edecbf557fa0]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  16:     0x7f60efa0cb93 - std::sys::unix::thread::Thread::new::thread_start::hae886abb3c6fe524
  17:     0x7f60ef734609 - start_thread
  18:     0x7f60ef874163 - clone
  19:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-beta.6 (6c380750e 2022-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
error: aborting due to previous error

