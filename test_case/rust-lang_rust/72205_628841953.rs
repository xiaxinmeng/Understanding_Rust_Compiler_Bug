plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 9'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2cd99d5a-3d37-45a5-b988-68cbf1dbaf28.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72205/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72205/merge:refs/remotes/pull/72205/merge
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
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling chalk-rust-ir v0.10.0
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
......................................................i............................................. 1800/10164
.................................................................................................... 1900/10164
........................................................................i..i........................ 2000/10164
.................................................................................................... 2100/10164
..............................................................iiiii................................. 2200/10164
.................................................................................................... 2400/10164
.................................................................................................... 2500/10164
.................................................................................................... 2600/10164
.................................................................................................... 2700/10164
---
.................................................................................................... 5200/10164
.................................................................................................... 5300/10164
.........................i.......................................................................... 5400/10164
..................i................................................................................. 5500/10164
.........................ii.ii........i...i......................................................... 5600/10164
..........................................................................i......................... 5800/10164
.................................................................................................... 5900/10164
.....................ii.....................................i....................................... 6000/10164
.................................................................................................... 6100/10164
.................................................................................................... 6100/10164
.................................................................................................... 6200/10164
..................................................................................ii...i..ii........ 6300/10164
.................................................................................................... 6500/10164
.................................................................................................... 6600/10164
.................................................................................................... 6700/10164
.................................................................................................... 6700/10164
...............i..ii................................................................................ 6800/10164
.................................................................................................... 7000/10164
.....................................................................i.............................. 7100/10164
.................................................................................................... 7200/10164
.................................................................................................... 7300/10164
---
.................................................................................................... 8100/10164
.................................................................................................... 8200/10164
......................................................................................i............. 8300/10164
.................................................................................................... 8400/10164
........................................iiiiii.iiiii.i.............................................. 8500/10164
.................................................................................................... 8700/10164
.................................................................................................... 8800/10164
.................................................................................................... 8900/10164
.................................................................................................... 9000/10164
---
............................................................F....................................... 100/102
..
failures:

---- [mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs stdout ----
8     let mut _4: &mut [T];                // in scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:6
9     scope 1 {
10         debug self => _4;                // in scope 1 at $SRC_DIR/libcore/convert/mod.rs:LL:COL
-         let mut _5: &mut [T];            // in scope 1 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
13 
14     bb0: {

16         StorageLive(_3);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
16         StorageLive(_3);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
17         StorageLive(_4);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:6
18         _4 = &mut (*_1);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:6
-         StorageLive(_5);                 // scope 1 at $SRC_DIR/libcore/convert/mod.rs:LL:COL
-         _5 = _4;                         // scope 1 at $SRC_DIR/libcore/convert/mod.rs:LL:COL
-         _3 = _5;                         // scope 1 at $SRC_DIR/libcore/convert/mod.rs:LL:COL
-         StorageDead(_5);                 // scope 1 at $SRC_DIR/libcore/convert/mod.rs:LL:COL
+         _3 = _4;                         // scope 1 at $SRC_DIR/libcore/convert/mod.rs:LL:COL
23         _2 = &mut (*_3);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
24         StorageDead(_4);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:14: 3:15
25         _0 = &mut (*_2);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15

thread '[mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue-58867-inline-as-ref-as-mut/rustc.a.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3166:25


failures:
    [mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs
    [mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs

test result: FAILED. 101 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:01:05
Build completed unsuccessfully in 1:01:05
== clock drift check ==
  local time: Thu May 14 19:27:28 UTC 2020
  network time: Thu, 14 May 2020 19:27:28 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72205/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72205/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3831) (python)
##[section]Finishing: Finalize Job
