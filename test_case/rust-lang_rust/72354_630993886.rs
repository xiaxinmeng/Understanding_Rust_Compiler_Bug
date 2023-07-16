plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 5'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200512.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200512.2/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5e5e0899-007c-4297-bf97-bb37ace41a2b.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72354/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72354/merge:refs/remotes/pull/72354/merge
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
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
...................................................................i................................ 1800/10189
.................................................................................................... 1900/10189
......................................................................................i..i.......... 2000/10189
.................................................................................................... 2100/10189
............................................................................iiiii................... 2200/10189
.................................................................................................... 2400/10189
.................................................................................................... 2500/10189
.................................................................................................... 2600/10189
.................................................................................................... 2700/10189
---
........i........................................................................................... 5200/10189
.................................................................................................... 5300/10189
.......................................i............................................................ 5400/10189
................................i................................................................... 5500/10189
.........................................ii.ii........i...i......................................... 5600/10189
...........................................................................................i........ 5800/10189
.................................................................................................... 5900/10189
......................................ii.....................................i...................... 6000/10189
.................................................................................................... 6100/10189
.................................................................................................... 6100/10189
.................................................................................................... 6200/10189
...................................................................................................i 6300/10189
i...i..ii...........i............................................................................... 6400/10189
.................................................................................................... 6600/10189
.................................................................................................... 6700/10189
.................................................................................................... 6700/10189
................................i..ii............................................................... 6800/10189
.................................................................................................... 7000/10189
......................................................................................i............. 7100/10189
.................................................................................................... 7200/10189
.................................................................................................... 7300/10189
---
.................................................................................................... 8100/10189
.................................................................................................... 8200/10189
.................................................................................................... 8300/10189
.........i.......................................................................................... 8400/10189
...............................................................iiiiii.iiiiii.i...................... 8500/10189
..................i................................................................................. 8700/10189
.................................................................................................... 8800/10189
.................................................................................................... 8900/10189
.................................................................................................... 9000/10189
---
+   |
+ 4 | type A = B;
+   | ----------- similarly named type alias `A` defined here
15 ...
- 10 | type C = D;
+ 7 | type C = D;
+   |          ^ help: a type alias with a similar name exists: `A`
18 
19 error[E0412]: cannot find type `F` in this scope
---
28 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ui-testing-optout/ui-testing-optout.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args ui-testing-optout.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/ui-testing-optout.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ui-testing-optout" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "ui-testing=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ui-testing-optout/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
  |
4 | type A = B; //~ ERROR
  | ----------- similarly named type alias `A` defined here
...
7 | type C = D; //~ ERROR

error[E0412]: cannot find type `F` in this scope
  --> /checkout/src/test/ui/ui-testing-optout.rs:92:10
   |
   |
4  | type A = B; //~ ERROR
   | ----------- similarly named type alias `A` defined here
...
92 | type E = F; //~ ERROR

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0412`.
---
5    |     ^^^^^^ dereference of raw pointer


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr/unsafe-fn-assign-deref-ptr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/unsafe-fn-assign-deref-ptr.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-assign-deref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/unsafe-fn-assign-deref-ptr.rs:2:5
   |
LL |     *p = 0; //~ ERROR dereference of raw pointer is unsafe
   |     ^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:00:51
Build completed unsuccessfully in 1:00:51
== clock drift check ==
  local time: Tue May 19 18:16:39 UTC 2020
  network time: Tue, 19 May 2020 18:16:40 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72354/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72354/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4132) (python)
##[section]Finishing: Finalize Job
