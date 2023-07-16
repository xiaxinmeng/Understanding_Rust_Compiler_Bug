plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 7'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/33a5bb50-7b7c-45a0-bdde-703fe757b4ff.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72988/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72988/merge:refs/remotes/pull/72988/merge
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
............................i...............i....................................................... 5200/10316
.................................................................................................... 5300/10316
............................................................................i....................... 5400/10316
......................................................................i............................. 5500/10316
.......................................................................................ii.ii........ 5600/10316
i...i............................................................................................... 5700/10316
.....................................................................i.............................. 5900/10316
.................................................................................................... 6000/10316
.......................ii.....................................i..................................... 6100/10316
.................................................................................................... 6200/10316
.................................................................................................... 6200/10316
.................................................................................................... 6300/10316
.....................................................................................ii...i..ii..... 6400/10316
.................................................................................................... 6600/10316
.................................................................................................... 6700/10316
.................................................................................................... 6800/10316
.................................................................................................... 6800/10316
..................i..ii............................................................................. 6900/10316
.................................................................................................... 7100/10316
........................................................................i........................... 7200/10316
.................................................................................................... 7300/10316
.................................................................................................... 7400/10316
---
.................................................................................................... 8200/10316
...................................................................FF............................... 8300/10316
.................................................................................................... 8400/10316
...........i........................................................................................ 8500/10316
............................F.....................................iiiiiiiiiiii.i.................... 8600/10316
....................i............................................................................... 8800/10316
.................................................................................................... 8900/10316
.................................................................................................... 9000/10316
.................................................................................................... 9100/10316
---
9 LL | use std::collections::HashMap;


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
24    |             ^^^^^^^ not found in this scope
25    |
+ help: found a libstd struct with a similar name
+    |
+ LL |     let y1: std::collections::HashMap;
26 help: consider importing one of these items
27    |
28 LL | use std::collections::HashMap;



The actual stderr differed from the expected stderr.
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
help: found a libstd struct with a similar name
   |
   |
LL |     let y1: std::collections::HashMap; //~ ERROR cannot find type
help: consider importing one of these items
   |
LL | use std::collections::HashMap;
   |
   |
LL | use std::collections::hash_map::HashMap;
   |

error[E0412]: cannot find type `GooMap` in this scope
   |
   |
LL |     let y2: GooMap; //~ ERROR cannot find type

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0412, E0433.
---

---- [ui] ui/resolve/use_suggestion_placement.rs stdout ----
diff of stderr:

26 LL |     type Dict<K, V> = HashMap<K, V>;
28    |
+ help: found a libstd struct with a similar name
+    |
+    |
+ LL |     type Dict<K, V> = std::collections::HashMap<K, V>;
29 help: consider importing one of these items
30    |
31 LL | use std::collections::HashMap;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion_placement/use_suggestion_placement.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/use_suggestion_placement.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/use_suggestion_placement.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion_placement" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion_placement/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0412]: cannot find type `HashMap` in this scope
  --> /checkout/src/test/ui/resolve/use_suggestion_placement.rs:27:23
   |
LL |     type Dict<K, V> = HashMap<K, V>; //~ ERROR cannot find
   |
help: found a libstd struct with a similar name
   |
   |
LL |     type Dict<K, V> = std::collections::HashMap<K, V>; //~ ERROR cannot find
help: consider importing one of these items
   |
LL | use std::collections::HashMap;
   |
---

---- [ui] ui/rust-2018/issue-52202-use-suggestions.rs stdout ----
diff of stderr:

4 LL |     let _d = Drain {};
6    |
+ help: found a libstd struct with a similar name
+    |
+    |
+ LL |     let _d = std::string::Drain {};
7 help: consider importing one of these items
8    |
9 LL | use crate::plumbing::Drain;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-52202-use-suggestions/issue-52202-use-suggestions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/issue-52202-use-suggestions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-52202-use-suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-52202-use-suggestions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0422]: cannot find struct, variant or union type `Drain` in this scope
  --> /checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs:11:14
   |
LL |     let _d = Drain {};
   |
help: found a libstd struct with a similar name
   |
   |
LL |     let _d = std::string::Drain {};
help: consider importing one of these items
   |
LL | use crate::plumbing::Drain;
   |
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:52:28
Build completed unsuccessfully in 0:52:28
== clock drift check ==
  local time: Sat Jun  6 15:06:33 UTC 2020
  network time: Sat, 06 Jun 2020 15:06:33 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72988/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72988/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3938) (python)
##[section]Finishing: Finalize Job
