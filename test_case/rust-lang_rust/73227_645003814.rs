plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 5'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/936ad40d-5e01-44c7-8a6d-bf5b930853ed.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73227/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73227/merge:refs/remotes/pull/73227/merge
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
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 10320 tests
.................................................................................................... 100/10320
..............................................i.F...ii.............................................. 200/10320
.................................................................................................... 400/10320
.................................................................................................... 500/10320
.................................................................................................... 600/10320
.i.................................................................................................. 700/10320
---
.................................................................................................... 1900/10320
.................................................................................................... 2000/10320
................i..i................................................................................ 2100/10320
.................................................................................................... 2200/10320
......iiiii......................................................................................... 2300/10320
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
diff of stderr:

44   --> $DIR/duplicate-options.rs:22:21
45    |
46 LL |             options(nomem, nostack),
+    |                     ^^^^^
+    |
+ help: remove this option
+    |
+    |
+ LL |             options(, nostack),
48 
49 error: the `noreturn` option was already provided
50   --> $DIR/duplicate-options.rs:23:21



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/duplicate-options/duplicate-options.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/duplicate-options.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/duplicate-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/duplicate-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/duplicate-options/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `nomem` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:8:33
   |
LL |         asm!("", options(nomem, nomem));


error: the `att_syntax` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:10:38
   |
LL |         asm!("", options(att_syntax, att_syntax));


error: the `nostack` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:12:56
   |
LL |         asm!("", options(nostack, att_syntax), options(nostack));


error: the `nostack` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:14:35
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));


error: the `nostack` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:14:53
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));


error: the `nostack` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:14:71
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));

error: the `noreturn` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:21:33
   |
   |
LL |             options(att_syntax, noreturn), //~ ERROR the `noreturn` option was already provided


error: the `nomem` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:22:21
   |
LL |             options(nomem, nostack), //~ ERROR the `nomem` option was already provided
   |
help: remove this option
   |
   |
LL |             options(, nostack), //~ ERROR the `nomem` option was already provided

error: the `noreturn` option was already provided
  --> /checkout/src/test/ui/asm/duplicate-options.rs:23:21
   |
   |
LL |             options(noreturn), //~ ERROR the `noreturn` option was already provided

error: aborting due to 9 previous errors


---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:00:28
Build completed unsuccessfully in 1:00:28
== clock drift check ==
  local time: Tue Jun 16 20:48:29 UTC 2020
  network time: Tue, 16 Jun 2020 20:48:29 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73227/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73227/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3682) (python)
##[section]Finishing: Finalize Job
