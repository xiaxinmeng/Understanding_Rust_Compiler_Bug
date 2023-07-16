plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200604.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7a52a077-0b17-4a57-99c8-176c80002b42.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73300/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73300/merge:refs/remotes/pull/73300/merge
---
 ---> 29a56a071ad9
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> eb826cd6a4d7
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 9841042138f8
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 00b49f7048de
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.................................................................................................... 1900/10320
.................................................................................................... 2000/10320
...............i..i................................................................................. 2100/10320
.................................................................................................... 2200/10320
.....iiiii.......................................................................................... 2300/10320
.................................................................................................... 2500/10320
.................................................................................................... 2600/10320
.................................................................................................... 2700/10320
.................................................................................................... 2800/10320
---
.................................................................................................... 6000/10320
........ii.....................................i.................................................... 6100/10320
.................................................................................................... 6200/10320
.................................................................................................... 6300/10320
.......................................................................ii...i..ii...........i....... 6400/10320
.................................................................................................... 6600/10320
.................................................................................................... 6700/10320
.................................................................................................... 6800/10320
.................................................................................................... 6800/10320
.....i..ii.......................................................................................... 6900/10320
.................................................................................................... 7100/10320
............................................................i....................................... 7200/10320
.................................................................................................... 7300/10320
.................................................................................................... 7400/10320
---
.................................................................................................... 8200/10320
.................................................................................................... 8300/10320
.................................................................................................... 8400/10320
..i................................................................................................. 8500/10320
........................................................iiiiii.iiiiii.i............................. 8600/10320
.............i...................................................................................... 8800/10320
.................................................................................................... 8900/10320
.................................................................................................... 9000/10320
.................................................................................................... 9100/10320
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/193
.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii................ii......

 finished in 4.934
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.133
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 15.091
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---

---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
diff of stderr:

- error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
+ warning[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
3    |
4 LL | #[allow(test_lint)]

6    |
6    |
7    = note: `forbid` lint level was set on command line
8 
- error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
-    |
-    |
- LL | #[allow(test_lint)]
-    |         ^^^^^^^^^ overruled by previous forbid
-    = note: `forbid` lint level was set on command line
- 
- 
17 warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
19    |

30    |
31    = note: requested on the command line with `-F test-lint`
31    = note: requested on the command line with `-F test-lint`
32 
- error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
-    |
-    |
- LL | #[allow(test_lint)]
-    |         ^^^^^^^^^ overruled by previous forbid
-    = note: `forbid` lint level was set on command line
- 
- 
- error: aborting due to 4 previous errors; 1 warning emitted
+ error: aborting due to previous error; 2 warnings emitted
43 For more information about this error, try `rustc --explain E0453`.
44 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/lint-plugin-forbid-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-plugin-forbid-cmdline.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
   |
   |
LL | #[allow(test_lint)] //~ ERROR allow(test_lint) overruled by outer forbid(test_lint)
   |         ^^^^^^^^^ overruled by previous forbid
   = note: `forbid` lint level was set on command line


warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   |
   |
   = note: `#[warn(deprecated)]` on by default

error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
   = note: requested on the command line with `-F test-lint`

error: aborting due to previous error; 2 warnings emitted
---

---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
diff of stderr:

- error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
+ warning[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
3    |
3    |
4 LL | #![forbid(test_lint)]
7 LL | #[allow(test_lint)]
7 LL | #[allow(test_lint)]
8    |         ^^^^^^^^^ overruled by previous forbid
9 
- error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
-    |
-    |
- LL | #![forbid(test_lint)]
-    |           --------- `forbid` level set here
- ...
- LL | #[allow(test_lint)]
-    |         ^^^^^^^^^ overruled by previous forbid
- 
19 warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
21    |


36 LL | #![forbid(test_lint)]
38 
38 
- error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
-    |
-    |
- LL | #![forbid(test_lint)]
-    |           --------- `forbid` level set here
- ...
- LL | #[allow(test_lint)]
-    |         ^^^^^^^^^ overruled by previous forbid
- 
- error: aborting due to 4 previous errors; 1 warning emitted
+ error: aborting due to previous error; 2 warnings emitted
50 For more information about this error, try `rustc --explain E0453`.
51 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
LL | #[allow(test_lint)]
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   |
   |
   = note: `#[warn(deprecated)]` on by default

error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:7:11
   |
   |
LL | #![forbid(test_lint)]

error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0453`.
---


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:56:23
Build completed unsuccessfully in 0:56:23
== clock drift check ==
  local time: Tue Jun 16 02:49:26 UTC 2020
  network time: Tue, 16 Jun 2020 02:49:26 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73300/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73300/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3529) (python)
##[section]Finishing: Finalize Job
