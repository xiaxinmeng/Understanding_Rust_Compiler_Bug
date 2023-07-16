plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200430.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200430.2/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/255c2a92-8959-47b1-8466-ccccf4f22cd2.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72015/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72015/merge:refs/remotes/pull/72015/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
.......................i............................................................................ 1800/9999
.................................................................................................... 1900/9999
........................................i........................................................... 2000/9999
.................................................................................................... 2100/9999
..............................iiiii................................................................. 2200/9999
.................................................................................................... 2400/9999
.................................................................................................... 2500/9999
.................................................................................................... 2600/9999
.................................................................................................... 2700/9999
---
.....................i...............i.............................................................. 5100/9999
.................................................................................................... 5200/9999
...................................................................i................................ 5300/9999
...........................................................i........................................ 5400/9999
................................................................ii.ii........i...i.................. 5500/9999
......i............................................................................................. 5700/9999
.............i...................................................................................... 5800/9999
.................................................ii.....................................i........... 5900/9999
.................................................................................................... 6000/9999
.................................................................................................... 6000/9999
.................................................................................................... 6100/9999
.....................................................................................ii...i..ii..... 6200/9999
.................................................................................................... 6400/9999
.................................................................................................... 6500/9999
.................................................................................................... 6600/9999
.................................................................................................... 6600/9999
.................i..ii.............................................................................. 6700/9999
.................................................................................................... 6900/9999
..................i................................................................................. 7000/9999
.................................................................................................... 7100/9999
............................................................i....................................... 7200/9999
---
.................................................................................................... 7900/9999
.................................................................................................... 8000/9999
.................................................................................................... 8100/9999
............................i....................................................................... 8200/9999
..................................................................................iiiiii.iiiii.i.... 8300/9999
...................................i................................................................ 8500/9999
.................................................................................................... 8600/9999
.................................................................................................... 8700/9999
.................................................................................................... 8800/9999
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 188 tests
iiii......i.............ii.i..........i...............................i..i..................i....i.. 100/188
..........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 5.568
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.150
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 14.232
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
failures:

---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintPass};
   |                                             ^^^^^^^^^               ^^^^^^^^

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:40:1
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   |
   = note: `#[warn(deprecated)]` on by default
   = note: `#[warn(deprecated)]` on by default

error[E0609]: no field `span` on type `rustc_hir::CrateItem<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:33:42
   |
LL |                     .set_span(krate.item.span)
   |
   = note: available fields are: `module`, `attrs`

error: aborting due to previous error; 3 warnings emitted
---


---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:66:1
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   |
   = note: `#[warn(deprecated)]` on by default
   = note: `#[warn(deprecated)]` on by default

error[E0609]: no field `span` on type `rustc_hir::CrateItem<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:35:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassOkay,
LL | |     Symbol::intern("crate_okay")
   | |_- in this macro invocation
   |
   = note: available fields are: `module`, `attrs`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0609]: no field `span` on type `rustc_hir::CrateItem<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:35:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassRedBlue,
LL | |     Symbol::intern("crate_red"), Symbol::intern("crate_blue")
   | |_- in this macro invocation
   |
   = note: available fields are: `module`, `attrs`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0609]: no field `span` on type `rustc_hir::CrateItem<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:35:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassRedBlue,
LL | |     Symbol::intern("crate_red"), Symbol::intern("crate_blue")
   | |_- in this macro invocation
   |
   = note: available fields are: `module`, `attrs`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0609]: no field `span` on type `rustc_hir::CrateItem<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:35:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassGreyGreen,
LL | |     Symbol::intern("crate_grey"), Symbol::intern("crate_green")
   | |_- in this macro invocation
   |
   = note: available fields are: `module`, `attrs`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0609]: no field `span` on type `rustc_hir::CrateItem<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:35:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassGreyGreen,
LL | |     Symbol::intern("crate_grey"), Symbol::intern("crate_green")
   | |_- in this macro invocation
   |
   = note: available fields are: `module`, `attrs`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 5 previous errors; 1 warning emitted
For more information about this error, try `rustc --explain E0609`.

------------------------------------------



---- [ui] ui-fulldeps/issue-40001.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintPass};
   |                                             ^^^^^^^^^               ^^^^^^^^

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:22:1
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   |
   = note: `#[warn(deprecated)]` on by default
---


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:59:56
Build completed unsuccessfully in 0:59:56
== clock drift check ==
  local time: Sat May  9 16:16:33 UTC 2020
  network time: Sat, 09 May 2020 16:16:33 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72015/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72015/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4182) (python)
##[section]Finishing: Finalize Job
