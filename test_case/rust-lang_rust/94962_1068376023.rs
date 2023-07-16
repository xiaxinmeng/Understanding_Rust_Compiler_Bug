plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 67 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.................FF...........F..FFFFFFFF.FFF..F.FFFF..F.FFF.F.FF.F

---- [ui] ui-fulldeps/feature-gate-plugin.rs stdout ----


error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/empty-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/empty-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/feature-gate-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/feature-gate-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcjzD3Ic/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/feature-gate-plugin/auxiliary/empty-plugin.empty_plugin.e12799d7-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/feature-gate-plugin/auxiliary/empty-plugin.54gajmoui9z9jhxf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/feature-gate-plugin/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/feature-gate-plugin/auxiliary/libempty_plugin.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [ui] ui-fulldeps/gated-plugin.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/empty-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/empty-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcJLWqL8/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary/empty-plugin.empty_plugin.e12799d7-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary/empty-plugin.54gajmoui9z9jhxf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary/libempty_plugin.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [ui] ui-fulldeps/compiler-calls.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/compiler-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/compiler-calls/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/compiler-calls/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/compiler-calls/a.compiler_calls.eaf53c39-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/compiler-calls/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/compiler-calls/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:9:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustctY0NgK/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary/lint-plugin-test.lint_plugin_test.cc922696-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary/lint-plugin-test.2wizki4qa2pn3bwf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary/liblint_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:9:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustckjWGWP/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary/lint-plugin-test.lint_plugin_test.cc922696-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary/lint-plugin-test.2wizki4qa2pn3bwf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary/liblint_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:9:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcYgf9SY/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary/lint-plugin-test.lint_plugin_test.cc922696-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary/lint-plugin-test.2wizki4qa2pn3bwf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary/liblint_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:9:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcAcMlvE/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary/lint-plugin-test.lint_plugin_test.cc922696-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary/lint-plugin-test.2wizki4qa2pn3bwf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary/liblint_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-group-denied-lint-allowed.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:8:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:14:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcnyQd3d/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary/lint-group-plugin-test.lint_group_plugin_test.4e4bbb49-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary/lint-group-plugin-test.3vdp67z9foe7vl3j.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary/liblint_group_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 2 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-group-forbid-always-trumps-cli.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:8:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:14:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcuZ3mU4/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary/lint-group-plugin-test.lint_group_plugin_test.4e4bbb49-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary/lint-group-plugin-test.3vdp67z9foe7vl3j.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary/liblint_group_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 2 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:8:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:14:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcNGqHkf/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary/lint-group-plugin-test.lint_group_plugin_test.4e4bbb49-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary/lint-group-plugin-test.3vdp67z9foe7vl3j.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary/liblint_group_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 2 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:9:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustceqbPyf/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary/lint-plugin-test.lint_plugin_test.cc922696-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary/lint-plugin-test.2wizki4qa2pn3bwf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary/liblint_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:8:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:14:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustccHoeO9/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary/lint-group-plugin-test.lint_group_plugin_test.4e4bbb49-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary/lint-group-plugin-test.3vdp67z9foe7vl3j.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary/liblint_group_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 2 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:9:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcPyvn0b/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary/lint-plugin-test.lint_plugin_test.cc922696-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary/lint-plugin-test.2wizki4qa2pn3bwf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary/liblint_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/issue-40001.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcN6gLg8/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary/issue-40001-plugin.issue_40001_plugin.ae8d0f22-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary/issue-40001-plugin.2nkntz8ucilzmysv.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary/libissue_40001_plugin.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [ui] ui-fulldeps/lint-plugin.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:9:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:15:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcdFYXVf/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary/lint-plugin-test.lint_plugin_test.cc922696-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary/lint-plugin-test.2wizki4qa2pn3bwf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary/liblint_plugin_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lto-syntax-extension.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lto-syntax-extension-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lto-syntax-extension-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcSZiABC/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary/lto-syntax-extension-plugin.lto_syntax_extension_plugin.a2e530fb-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary/lto-syntax-extension-plugin.46hriirlhecbtpbz.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary/liblto_syntax_extension_plugin.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [ui] ui-fulldeps/plugin-args.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/empty-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/empty-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
---

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui-fulldeps/auxiliary/outlive-expansion-phase.rs:22:50
   |
   |
LL |     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
LL +     thread_local!(static FOO: RefCell<Option<Box<dyn Any+Send>>> = RefCell::new(None));

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustc56jYec/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/auxiliary/outlive-expansion-phase.outlive_expansion_phase.e706f0b9-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/auxiliary/outlive-expansion-phase.1bs3h3u1tb1w0hhp.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/auxiliary/liboutlive_expansion_phase.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 34 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/uninit_mask.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/uninit_mask.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/uninit_mask/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/uninit_mask/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/uninit_mask/a.uninit_mask.e1b0269f-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/uninit_mask/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_middle-977f9ea92cc57b24.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libchalk_ir-460be866323e7f9a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_xoshiro-3f2b609718d93d7d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_type_ir-54110432ccf644a4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_apfloat-0203a9a89213091e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgsgdt-cbc088c92dd86ca9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserde-b81612ec207aca25.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpolonius_engine-9a6ef456fce62eaa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libdatafrog-da1fe8c43db765bf.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_attr-ee3c56ed0a8fadde.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_ast_pretty-13ed1f4eb7599d45.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_query_system-41354f7ae6f49325.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_session-1c02e1db641de0b3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-3ed6b7fe1bbbaf04.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_fs_util-3c8ca6c1268d69cf.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_feature-a0ca2e90ef744d56.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_errors-5fe49b0aec35107f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtermize-ff68cc6dfd1481c2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libatty-cf79ec859b705932.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libannotate_snippets-4e0ab0234e165fa3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtermcolor-4e506bc0a1a3312d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lint_defs-5317a5acb430c828.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hir-7d04f15d97ca280a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libodht-efd7943bd1360c15.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_target-cda9a6010e9c96c8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_ast-e9e81c51b1641d94.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lexer-322f21668fa5035e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunic_emoji_char-e87bf5fec24e82d9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunic_ucd_version-7484968e031afc87.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunic_common-9572ab903c366524.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunic_char_property-f0eab2780a530bb5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunic_char_range-582d86236cb5e8c2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_xid-e455a7790eda7544.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_span-fa86e79b12080b86.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libscoped_tls-135688c32002751f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsha2-d5d56ae2d67a356d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsha1-385b53188f36dbfc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcpufeatures-6977e1ca0bc719a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmd5-93f7cb3d9e07612e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libdigest-f40cdb0171fbafeb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libblock_buffer-d5c5c0e0f58552e2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrypto_common-b219be144bb24ff3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgeneric_array-15d0a0d9d8e9238d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtypenum-fe01e9dca32f23c9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-9d2f6af24b0f876f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_arena-f866031a36736f3e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-7ca5bd39ce0dfd8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstacker-01d4a0b62eb373b7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpsm-487bfc18fdca7b69.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemmap2-54bbb75fcdf2d43d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtempfile-f1f626c50748f645.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-1e7066fa74fc7205.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_chacha-096b59c2feb5fae5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libppv_lite86-bda0d27f5a06bfe5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_core-b6d2813db59186c8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libremove_dir_all-35c9e3baf36a45cd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmeasureme-0f4756632e07b5db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libparking_lot-bd9ceb3bdc97dec8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libparking_lot_core-5f8bad8eb6cae8c7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblock_api-e15c4f4563438cce.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libinstant-8fe917a094b92a15.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libena-8b0d997259df8f2b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-0d7da93b712b997a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstable_deref_trait-f88d7854cb0926dc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_graphviz-8307ff5e6e91afd5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libjobserver-62341839c4163589.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hash-e06702d0e5f2eb0b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_index-af6bfa630c15a5df.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_serialize-20ac0d4e66524517.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libindexmap-b83ee5f86becd831.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_rayon-b88b54619e40e6b5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_rayon_core-a440b87b471b1b1b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libnum_cpus-cf1bcf70cbaf59bb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrossbeam_deque-68ec89dc36ec1e97.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrossbeam_epoch-8980a0047dc6178f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemoffset-56524b8fa2c6796e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libscopeguard-4abae55fd8b1be00.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrossbeam_utils-aacf4f21f3ee380b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libeither-924fd47eda5a462d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-90584dc81f46ad80.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libahash-81694a0707e95203.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetrandom-4dbf02532ef5ddbf.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-75a69883cc602ed5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libonce_cell-d99d7a67a1aeda0c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsmallvec-7a7afa36e12edff4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarrayvec-527ef136c69fe85a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-4287bc3f7ab5099f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtracing-e888ecbbde9a72c9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpin_project_lite-8660e27fcebe5f51.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtracing_core-0be511e0aee1d114.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblazy_static-ce6b5d09a2f00b22.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-2a619b3c131313ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbitflags-a0f9ad9e790380e4.rlib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/uninit_mask/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:7:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:13:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintId, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:13:79
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintId, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcpUJZGI/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary/lint-tool-test.lint_tool_test.f15869b6-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary/lint-tool-test.5f6b4k4uwx029mdf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary/liblint_tool_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:7:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `LintArray`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:13:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintId, LintPass};

warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:13:79
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintId, LintPass};

error: linking with `cc` failed: exit status: 1
   |
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustcLpmk8k/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary/lint-tool-test.lint_tool_test.f15869b6-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary/lint-tool-test.5f6b4k4uwx029mdf.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-07bdb291d9940293" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-325a42043fc94c77" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-113cdab8486abf7b.rlib" "-Wl,-Bdynamic" "-lLLVM-12" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary/liblint_tool_test.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find -lpsm_s
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 3 warnings emitted
------------------------------------------



---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:7:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: linking with `cc` failed: exit status: 1
