plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
Agent machine name: 'fv-az578'
Current agent version: '2.169.0'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/88eb7882-1294-44be-b21d-d3c952940b74.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71862/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71862/merge:refs/remotes/pull/71862/merge
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
...........................................................................i........................ 1800/10220
.................................................................................................... 1900/10220
..............................................................................................i..i.. 2000/10220
.................................................................................................... 2100/10220
....................................................................................iiiii........... 2200/10220
.................................................................................................... 2400/10220
.................................................................................................... 2500/10220
.................................................................................................... 2600/10220
.................................................................................................... 2700/10220
---
...........i...............i........................................................................ 5200/10220
.................................................................................................... 5300/10220
..........................................................i......................................... 5400/10220
...................................................i................................................ 5500/10220
.............................................................ii.ii........i...i..................... 5600/10220
...i................................................................................................ 5800/10220
...........i........................................................................................ 5900/10220
...............................................................ii................................... 6000/10220
..i................................................................................................. 6100/10220
..i................................................................................................. 6100/10220
.................................................................................................... 6200/10220
.................................................................................................... 6300/10220
........................ii...i..ii...........i...................................................... 6400/10220
.................................................................................................... 6600/10220
.................................................................................................... 6700/10220
.................................................................................................... 6700/10220
.........................................................i..ii...................................... 6800/10220
.................................................................................................... 7000/10220
.................................................................................................... 7100/10220
...........i........................................................................................ 7200/10220
.................................................................................................... 7300/10220
---
.................................................................................................... 8100/10220
.................................................................................................... 8200/10220
.................................................................................................... 8300/10220
....................................i............................................................... 8400/10220
..........................................................................................iiiiii.iii 8500/10220
iii.i............................................................................................... 8600/10220
.................................................................................................... 8800/10220
.................................................................................................... 8900/10220
.................................................................................................... 9000/10220
.................................................................................................... 9100/10220
---
failures:

---- [ui] ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs stdout ----

error: /checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs:1: unexpected error: '1:1: 1:33: the `unsafe_op_in_unsafe_fn` lint is unstable [E0658]'

error: /checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs:1: unexpected error: '1:1: 1:33: the `unsafe_op_in_unsafe_fn` lint is unstable [E0658]'
error: 2 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "1:1: 1:33: the `unsafe_op_in_unsafe_fn` lint is unstable [E0658]",
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "1:1: 1:33: the `unsafe_op_in_unsafe_fn` lint is unstable [E0658]",
]

thread '[ui] ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:04:15
Build completed unsuccessfully in 1:04:15
== clock drift check ==
  local time: Fri May 22 19:30:58 UTC 2020
  network time: Fri, 22 May 2020 19:30:58 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71862/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71862/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3870) (python)
##[section]Finishing: Finalize Job
