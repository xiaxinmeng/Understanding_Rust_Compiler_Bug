plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c9c30c8d-88c2-46b5-bc88-db0d2b6e5225.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72069/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72069/merge:refs/remotes/pull/72069/merge
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
.....................................................i.............................................. 1800/10159
.................................................................................................... 1900/10159
.......................................................................i............................ 2000/10159
.................................................................................................... 2100/10159
.............................................................iiiii.................................. 2200/10159
.................................................................................................... 2400/10159
.................................................................................................... 2500/10159
.................................................................................................... 2600/10159
.................................................................................................... 2700/10159
---
.................................................................................................... 5200/10159
.................................................................................................... 5300/10159
........................i........................................................................... 5400/10159
.................i.................................................................................. 5500/10159
........................ii.ii........i...i.......................................................... 5600/10159
.........................................................................i...............FF......... 5800/10159
.................................................................................................... 5900/10159
....................ii.....................................i........................................ 6000/10159
.................................................................................................... 6100/10159
.................................................................................................... 6100/10159
.................................................................................................... 6200/10159
.................................................................................ii...i..ii......... 6300/10159
.................................................................................................... 6500/10159
.................................................................................................... 6600/10159
.................................................................................................... 6700/10159
.................................................................................................... 6700/10159
..............i..ii................................................................................. 6800/10159
.................................................................................................... 7000/10159
...................................................................i................................ 7100/10159
...........................................................................................F........ 7200/10159
.................................................................................................... 7300/10159
---
.................................................................................................... 8100/10159
.................................................................................................... 8200/10159
..................................................................................i................. 8300/10159
.................................................................................................... 8400/10159
....................................iiiiii.iiiii.i.................................................. 8500/10159
.................................................................................................... 8700/10159
.................................................................................................... 8800/10159
.................................................................................................... 8900/10159
.................................................................................................... 9000/10159
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48508/issue-48508.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-48508.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-48508.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48508/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48508/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
25 error: identifier contains non-ASCII characters
-   --> $DIR/lint-non-ascii-idents.rs:10:20
+   --> $DIR/lint-non-ascii-idents.rs:4:7
27    |
- LL |     println!("{}", naïveté);
+ LL | const חלודה: usize = 2;
+    |       ^^^^^
30 
- error: aborting due to 4 previous errors
- error: aborting due to 4 previous errors
+ error: aborting due to 3 previous errors
32 
33 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents/lint-non-ascii-idents.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: identifier contains non-ASCII characters
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents.rs:6:4
   |
LL | fn coöperation() {} //~ ERROR identifier contains non-ASCII characters
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents.rs:2:9
   |
   |
LL | #![deny(non_ascii_idents)]
   |         ^^^^^^^^^^^^^^^^

error: identifier contains non-ASCII characters
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents.rs:9:9
   |
LL |     let naïveté = 2; //~ ERROR identifier contains non-ASCII characters

error: identifier contains non-ASCII characters
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents.rs:4:7
   |
   |
LL | const חלודה: usize = 2; //~ ERROR identifier contains non-ASCII characters

error: aborting due to 3 previous errors


---
1 error: identifier contains uncommon Unicode codepoints
-   --> $DIR/lint-uncommon-codepoints.rs:4:7
+   --> $DIR/lint-uncommon-codepoints.rs:6:4
3    |
- LL | const µ: f64 = 0.000001;
-    |       ^
+ LL | fn dĳkstra() {}
6    |
7 note: the lint level is defined here
8   --> $DIR/lint-uncommon-codepoints.rs:2:9


11    |         ^^^^^^^^^^^^^^^^^^^
12 
13 error: identifier contains uncommon Unicode codepoints
-   --> $DIR/lint-uncommon-codepoints.rs:6:4
+   --> $DIR/lint-uncommon-codepoints.rs:4:7
15    |
- LL | fn dĳkstra() {}
-    |    ^^^^^^^
+ LL | const µ: f64 = 0.000001;
18 
19 error: identifier contains uncommon Unicode codepoints
20   --> $DIR/lint-uncommon-codepoints.rs:9:9


22 LL |     let ㇻㇲㇳ = "rust";
24 
- error: identifier contains uncommon Unicode codepoints
-   --> $DIR/lint-uncommon-codepoints.rs:10:20
-    |
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints/lint-uncommon-codepoints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: identifier contains uncommon Unicode codepoints
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints.rs:6:4
   |
LL | fn dĳkstra() {} //~ ERROR identifier contains uncommon Unicode codepoints
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints.rs:2:9
   |
   |
LL | #![deny(uncommon_codepoints)]
   |         ^^^^^^^^^^^^^^^^^^^

error: identifier contains uncommon Unicode codepoints
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints.rs:4:7
   |
LL | const µ: f64 = 0.000001; //~ ERROR identifier contains uncommon Unicode codepoints

error: identifier contains uncommon Unicode codepoints
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-uncommon-codepoints.rs:9:9
   |
   |
LL |     let ㇻㇲㇳ = "rust"; //~ ERROR identifier contains uncommon Unicode codepoints

error: aborting due to 3 previous errors


---
+    | ^
+    |
+    = note: `#[warn(uncommon_codepoints)]` on by default
+ 
+ error: aborting due to 3 previous errors; 1 warning emitted
34 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62524/issue-62524.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-62524.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-62524.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62524" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62524/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | |__^
   |
help: change the delimiters to curly braces
   |
LL | y! { /* items */ }
help: add a semicolon
   |
LL | Ϥ,;
   |   ^
---
   | ^
   |
   = note: `#[warn(uncommon_codepoints)]` on by default

error: aborting due to 3 previous errors; 1 warning emitted

------------------------------------------


---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:44
Build completed unsuccessfully in 1:05:44
== clock drift check ==
  local time: Sun May 10 02:44:10 UTC 2020
  network time: Sun, 10 May 2020 02:44:10 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72069/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72069/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3532) (python)
##[section]Finishing: Finalize Job
