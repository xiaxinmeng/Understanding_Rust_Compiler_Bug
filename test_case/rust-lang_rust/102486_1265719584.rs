plain

---- [ui] src/test/ui/consts/const_in_pattern/issue-73431.rs stdout ----
diff of stderr:

- 2:rustcWARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
+ WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/issue-73431.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/issue-73431.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_in_pattern/issue-73431.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_in_pattern/issue-73431.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/auxiliary"
stdout: none
--- stderr -------------------------------
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.


---- [ui] src/test/ui/limits/huge-array.rs stdout ----
diff of stderr:
diff of stderr:

4 LL |     let s: [T; 1518600000] = [t; 1518600000];
6 
6 
- thread '<unnamed>' panicked at 'Could not send Message::CodegenItem to main thread', $COMPILER_DIR/rustc_codegen_ssa/src/back/write.rs:1307:29
- stack backtrace:
-    0:        0x104aaa0f8 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h644688b77e474c96
-    1:        0x104b073ac - core::fmt::write::ha94f749aaf09ea53
-    2:        0x104ab5e5c - std::io::Write::write_fmt::h8e3f533c91ae9f93
-    3:        0x104aa9fc0 - std::sys_common::backtrace::print::h46f51eb26779a5c7
-    4:        0x104aaf5b4 - std::panicking::default_hook::{{closure}}::hc3b969550fc1dc8f
-    5:        0x104aaf3d8 - std::panicking::default_hook::h4ebc9e2b710bcf1a
-    6:        0x10d78027c - rustc_driver[43b72fe45c695470]::DEFAULT_HOOK::{closure#0}::{closure#0}
-    7:        0x104aafaf0 - std::panicking::rust_panic_with_hook::ha9a3e6b88e2f8614
-    8:        0x104acba0c - std::panicking::begin_panic_handler::{{closure}}::h53a345a249fce38d
-    9:        0x104acb97c - std::sys_common::backtrace::__rust_end_short_backtrace::h2da7fafd452eaa50
-   10:        0x104aaf674 - _rust_begin_unwind
-   11:        0x104b25e44 - core::panicking::panic_fmt::h4d833ba0c5316123
-   12:        0x10d8d4c14 - std[b8b81ef226880973]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[66ba86ec87aa19c0]::LlvmCodegenBackend as rustc_codegen_ssa[46bef89ecaecfeb8]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::start_executing_work<rustc_codegen_llvm[66ba86ec87aa19c0]::LlvmCodegenBackend>::{closure#4}, core[cb88d880ee832a3c]::result::Result<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::CompiledModules, ()>>::{closure#0}, core[cb88d880ee832a3c]::result::Result<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::CompiledModules, ()>>
-   13:        0x10d993134 - std[b8b81ef226880973]::panicking::try::<core[cb88d880ee832a3c]::result::Result<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::CompiledModules, ()>, core[cb88d880ee832a3c]::panic::unwind_safe::AssertUnwindSafe<<std[b8b81ef226880973]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[66ba86ec87aa19c0]::LlvmCodegenBackend as rustc_codegen_ssa[46bef89ecaecfeb8]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::start_executing_work<rustc_codegen_llvm[66ba86ec87aa19c0]::LlvmCodegenBackend>::{closure#4}, core[cb88d880ee832a3c]::result::Result<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::CompiledModules, ()>>::{closure#0}, core[cb88d880ee832a3c]::result::Result<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
-   14:        0x10d8faaec - <<std[b8b81ef226880973]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[66ba86ec87aa19c0]::LlvmCodegenBackend as rustc_codegen_ssa[46bef89ecaecfeb8]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::start_executing_work<rustc_codegen_llvm[66ba86ec87aa19c0]::LlvmCodegenBackend>::{closure#4}, core[cb88d880ee832a3c]::result::Result<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::CompiledModules, ()>>::{closure#0}, core[cb88d880ee832a3c]::result::Result<rustc_codegen_ssa[46bef89ecaecfeb8]::back::write::CompiledModules, ()>>::{closure#1} as core[cb88d880ee832a3c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
-   15:        0x104ab4584 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h79d2ec9f34c31672
-   16:        0x104ac9c04 - std::sys::unix::thread::Thread::new::thread_start::h8d8d6d846b61d5a4
-   17:        0x1a3881240 - __pthread_deallocate
- error: internal compiler error: unexpected panic
- 
- note: the compiler unexpectedly panicked. this is a bug.
- 
- 
- note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
- 
- note: rustc 1.66.0-dev running on aarch64-apple-darwin
- 
- note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
- query stack during panic:
- end of query stack
40 error: aborting due to previous error
41 
---
To only update this specific test, also pass `--test-args limits/huge-array.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/limits/huge-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/huge-array" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/huge-array/auxiliary"
stdout: none
--- stderr -------------------------------
error: values of the type `[[u8; 1518599999]; 1518600000]` are too big for the current architecture
   |
   |
LL |     let s: [T; 1518600000] = [t; 1518600000];

error: aborting due to previous error
------------------------------------------

