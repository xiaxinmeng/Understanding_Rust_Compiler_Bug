plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/33c70412-f4d0-4ecb-a2a8-33ff9c251cf3.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72445/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72445/merge:refs/remotes/pull/72445/merge
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
...........................................................................i........................ 1800/10214
.................................................................................................... 1900/10214
..............................................................................................i..i.. 2000/10214
.................................................................................................... 2100/10214
....................................................................................iiiii........... 2200/10214
.................................................................................................... 2400/10214
.................................................................................................... 2500/10214
.................................................................................................... 2600/10214
.................................................................................................... 2700/10214
---
.........i...............i.......................................................................... 5200/10214
.................................................................................................... 5300/10214
........................................................i........................................... 5400/10214
.................................................i.................................................. 5500/10214
..........................................................ii.ii........i...i........................ 5600/10214
i................................................................................................... 5800/10214
........i........................................................................................... 5900/10214
........i........................................................................................... 5900/10214
............................................................ii.F...................................i 6000/10214
.................................................................................................... 6200/10214
.................................................................................................... 6300/10214
.................................................................................................... 6300/10214
.....................ii...i..ii...........i......................................................... 6400/10214
.................................................................................................... 6600/10214
.................................................................................................... 6700/10214
.................................................................................................... 6700/10214
......................................................i..ii......................................... 6800/10214
.................................................................................................... 7000/10214
.................................................................................................... 7100/10214
........i........................................................................................... 7200/10214
.................................................................................................... 7300/10214
---
.................................................................................................... 7900/10214
.................................................................................................... 8000/10214
.................................................................................................... 8100/10214
.................................................................................................... 8200/10214
...........................................................F.F.F.F..FF.............................. 8300/10214
................................i................................................................... 8400/10214
......................................................................................iiiiii.iiiiii. 8500/10214
.........................................i.......................................................... 8700/10214
.................................................................................................... 8800/10214
.................................................................................................... 8900/10214
.................................................................................................... 9000/10214
---

---- [ui] ui/macros/issue-68060.rs stdout ----
diff of stderr:

1 error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
+   --> $DIR/issue-68060.rs:4:13
3    |
3    |
4 LL |             #[target_feature(enable = "")]


11    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
13 error: the feature named `` is not valid for this target
-   --> $DIR/issue-68060.rs:6:30
+   --> $DIR/issue-68060.rs:4:30
15    |
15    |
16 LL |             #[target_feature(enable = "")]
17    |                              ^^^^^^^^^^^ `` is not valid for this target
18 
19 error[E0737]: `#[track_caller]` requires Rust ABI
-   --> $DIR/issue-68060.rs:9:13
+   --> $DIR/issue-68060.rs:7:13
+   --> $DIR/issue-68060.rs:7:13
21    |
22 LL |             #[track_caller]
23    |             ^^^^^^^^^^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/issue-68060.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/issue-68060.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-68060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
   |
   |
LL |             #[target_feature(enable = "")]
...
LL |             |_| (),
   |             ------ not an `unsafe` function
   |
   |
   = note: see issue #69098 <***/issues/69098> for more information
   = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
error: the feature named `` is not valid for this target
  --> /checkout/src/test/ui/macros/issue-68060.rs:4:30
   |
   |
LL |             #[target_feature(enable = "")]
   |                              ^^^^^^^^^^^ `` is not valid for this target
error[E0737]: `#[track_caller]` requires Rust ABI
  --> /checkout/src/test/ui/macros/issue-68060.rs:7:13
   |
LL |             #[track_caller]
---
---- [ui] ui/rfc-2091-track-caller/const-caller-location.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/const-caller-location.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
1 error: malformed `track_caller` attribute input
-   --> $DIR/error-odd-syntax.rs:3:1
+   --> $DIR/error-odd-syntax.rs:1:1
3    |
4 LL | #[track_caller(1)]
5    | ^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[track_caller]`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-odd-syntax/error-odd-syntax.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-odd-syntax/error-odd-syntax.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-odd-syntax.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-odd-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-odd-syntax" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-odd-syntax/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: malformed `track_caller` attribute input
  --> /checkout/src/test/ui/rfc-2091-track-caller/error-odd-syntax.rs:1:1
   |
LL | #[track_caller(1)]
   | ^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[track_caller]`
error: aborting due to previous error


------------------------------------------
---
11    |     ^^^^^^^^^^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-invalid-abi/error-with-invalid-abi.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-with-invalid-abi.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-invalid-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-invalid-abi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-invalid-abi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
5    | ^^^^^^^^^^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/only-for-fns/only-for-fns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2091-track-caller/only-for-fns.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/only-for-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/only-for-fns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/only-for-fns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:56:52
Build completed unsuccessfully in 0:56:52
== clock drift check ==
  local time: Fri May 22 01:10:52 UTC 2020
  network time: Fri, 22 May 2020 01:10:52 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72445/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72445/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3938) (python)
##[section]Finishing: Finalize Job
