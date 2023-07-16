plain
- LL |     unsafe { intrinsics::unreachable() }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^
-    |              |
-    |              entering unreachable code
-    |              inside `unreachable_unchecked` at $SRC_DIR/core/src/hint.rs:LL:COL
+ LL |         unreachable!()
+    |         |
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/hint.rs:LL:COL
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/hint.rs:LL:COL
+    |         inside `unreachable_unchecked` at $SRC_DIR/core/src/panic.rs:LL:COL
10   ::: $DIR/const_unsafe_unreachable_ub.rs:6:18
11    |

14 ...
14 ...
15 LL | const BAR: bool = unsafe { foo(false) };
16    |                            ---------- inside `BAR` at $DIR/const_unsafe_unreachable_ub.rs:10:28
+    |
+    = note: this error originates in the macro `$crate::panic::unreachable_2021` which comes from the expansion of the macro `unreachable` (in Nightly builds, run with -Z macro-backtrace for more info)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub/const_unsafe_unreachable_ub.stderr
To only update this specific test, also pass `--test-args consts/const_unsafe_unreachable_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/library/core/src/hint.rs:102:9
   |
LL |         unreachable!()
   |         ^^^^^^^^^^^^^^
---
   |
LL |         false => std::hint::unreachable_unchecked(),
   |                  ---------------------------------- inside `foo` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:6:18
...
LL | const BAR: bool = unsafe { foo(false) };
   |                            ---------- inside `BAR` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:10:28
   |
   = note: this error originates in the macro `$crate::panic::unreachable_2021` which comes from the expansion of the macro `unreachable` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 stdout ----

error in revision `fat0`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fat0" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/tmp/rustcJLHty1/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a.issue_64655_extern_rust_must_allow_unwind.2a84b153-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-07150088dcceb2ba.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,--strip-debug" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-07150088dcceb2ba.rlib(compiler_builtins-07150088dcceb2ba.compiler_builtins.ea377314-cgu.117.rcgu.o): in function `compiler_builtins::int::specialized_div_rem::zero_div_fn':
           compiler_builtins.ea377314-cgu.117:(.text._ZN17compiler_builtins3int19specialized_div_rem11zero_div_fn17hf34ec29fbbf93557E+0x16): undefined reference to `core::panicking::panic'
           collect2: error: ld returned 1 exit status
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-44056.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44056.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ctarget-feature=+avx" "-Clto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/tmp/rustcyXhZ3b/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/issue-44056.issue_44056.9f62d9c4-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-07150088dcceb2ba.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/issue-44056" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,--strip-debug" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-07150088dcceb2ba.rlib(compiler_builtins-07150088dcceb2ba.compiler_builtins.ea377314-cgu.117.rcgu.o): in function `compiler_builtins::int::specialized_div_rem::zero_div_fn':
           compiler_builtins.ea377314-cgu.117:(.text._ZN17compiler_builtins3int19specialized_div_rem11zero_div_fn17hf34ec29fbbf93557E+0x16): undefined reference to `core::panicking::panic'
           collect2: error: ld returned 1 exit status
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------


