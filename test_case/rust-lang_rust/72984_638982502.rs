plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 6'
Agent machine name: 'fv-az578'
Current agent version: '2.169.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6ebadff4-2cdd-4571-91df-6f7a0cd13f66.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72984/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72984/merge:refs/remotes/pull/72984/merge
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
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
............................i...............i....................................................... 5200/10287
.................................................................................................... 5300/10287
............................................................................i....................... 5400/10287
......................................................................i............................. 5500/10287
.......................................................................................ii.ii........ 5600/10287
i...i............................................................................................... 5700/10287
......................................i............................................................. 5900/10287
............................................................................................ii...... 6000/10287
...............................i.................................................................... 6100/10287
.................................................................................................... 6200/10287
.................................................................................................... 6200/10287
.................................................................................................... 6300/10287
......................................................ii...i..ii...........i........................ 6400/10287
.................................................................................................... 6600/10287
.................................................................................................... 6700/10287
.................................................................................................... 6700/10287
.......................................................................................i..ii........ 6800/10287
.................................................................................................... 7000/10287
.................................................................................................... 7100/10287
.........................................i.......................................................... 7200/10287
.................................................................................................... 7300/10287
---
.................................................................................................... 8200/10287
....................................F............................................................... 8300/10287
................................................................................i................... 8400/10287
.................................................................................................... 8500/10287
..................................iiiiii.iiiiii.i................................................... 8600/10287
.................................................................................................... 8800/10287
.................................................................................................... 8900/10287
.................................................................................................... 9000/10287
.................................................................................................... 9100/10287
---
14 error: aborting due to previous error


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1/issue-31997-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derived-errors/issue-31997-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derived-errors/issue-31997-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/no_implicit_prelude.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/no_implicit_prelude.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/no_implicit_prelude.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0433]: failed to resolve: use of undeclared type or module `Vec`
  --> /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:11:9
   |
LL |     fn f() { ::bar::m!(); }
...
LL |         Vec::new(); //~ ERROR failed to resolve
   |         ^^^ not found in this scope
   |
---

error[E0599]: no method named `clone` found for unit type `()` in the current scope
  --> /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:12:12
   |
LL |     fn f() { ::bar::m!(); }
...
...
LL |         ().clone() //~ ERROR no method named `clone` found
   |            ^^^^^ method not found in `()`
   = help: items from traits can only be used if the trait is in scope
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
           `use std::clone::Clone;`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
-    |
- LL | use std::collections::hash_map::HashMap;
31    |
32 
33 error[E0412]: cannot find type `GooMap` in this scope

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion/use_suggestion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion/use_suggestion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/use_suggestion.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/use_suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type or module `GooMap`
   |
   |
LL |     let x2 = GooMap::new(); //~ ERROR failed to resolve
   |              ^^^^^^ use of undeclared type or module `GooMap`
error[E0433]: failed to resolve: use of undeclared type or module `HashMap`
  --> /checkout/src/test/ui/resolve/use_suggestion.rs:2:14
   |
LL |     let x1 = HashMap::new(); //~ ERROR failed to resolve
---

error[E0412]: cannot find type `HashMap` in this scope
  --> /checkout/src/test/ui/resolve/use_suggestion.rs:5:13
   |
LL |     let y1: HashMap; //~ ERROR cannot find type
   |
help: consider importing this struct
   |
LL | use std::collections::HashMap;
LL | use std::collections::HashMap;
   |

error[E0412]: cannot find type `GooMap` in this scope
   |
   |
LL |     let y2: GooMap; //~ ERROR cannot find type

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0412, E0433.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:58:17
Build completed unsuccessfully in 0:58:17
== clock drift check ==
  local time: Thu Jun  4 17:00:30 UTC 2020
  network time: Thu, 04 Jun 2020 17:00:30 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72984/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72984/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3679) (python)
##[section]Finishing: Finalize Job
