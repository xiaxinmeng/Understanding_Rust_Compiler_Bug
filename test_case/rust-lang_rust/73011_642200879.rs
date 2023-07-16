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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/85437eee-526e-4e7b-92fe-b4e439877297.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73011/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73011/merge:refs/remotes/pull/73011/merge
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
..............................i...............i..................................................... 5200/10292
.................................................................................................... 5300/10292
..............................................................................i..................... 5400/10292
........................................................................i........................... 5500/10292
.........................................................................................ii.ii...... 5600/10292
..i...i............................................................................................. 5700/10292
........................................i........................................................... 5900/10292
..............................................................................................ii.... 6000/10292
.................................i.................................................................. 6100/10292
.................................................................................................... 6200/10292
.................................................................................................... 6200/10292
.................................................................................................... 6300/10292
........................................................ii...i..ii...........i...................... 6400/10292
.................................................................................................... 6600/10292
.................................................................................................... 6700/10292
.................................................................................................... 6700/10292
.........................................................................................i..ii...... 6800/10292
.................................................................................................... 7000/10292
.................................................................................................... 7100/10292
...........................................i........................................................ 7200/10292
.................................................................................................... 7300/10292
---
.................................................................................................... 8200/10292
.................................................................................................... 8300/10292
...................................................................................i................ 8400/10292
.................................................................................................... 8500/10292
.....................................iiiiii.iiiiii.i................................................ 8600/10292
.................................................................................................... 8800/10292
.................................................................................................... 8900/10292
.................................................................................................... 9000/10292
.................................................................................................... 9100/10292
---
.............................................................F...................................... 100/107
.......
failures:

---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----
9 +     let mut _4: ();                      // in scope 0 at $DIR/instrument_coverage.rs:8:1: 14:2
11       bb0: {
11       bb0: {
- -         falseUnwind -> [real: bb1, cleanup: bb6]; // scope 0 at $DIR/instrument_coverage.rs:9:5: 13:6
+ -         falseUnwind -> [real: bb1, cleanup: bb2]; // scope 0 at $DIR/instrument_coverage.rs:9:5: 13:6
13 +         StorageLive(_4);                 // scope 0 at $DIR/instrument_coverage.rs:8:1: 14:2
14 +         _4 = const std::intrinsics::count_code_region(const 0u32) -> bb7; // scope 0 at $DIR/instrument_coverage.rs:8:1: 14:2
15 +                                          // ty::Const
28   
29       bb1: {
30           StorageLive(_2);                 // scope 0 at $DIR/instrument_coverage.rs:10:12: 10:17
30           StorageLive(_2);                 // scope 0 at $DIR/instrument_coverage.rs:10:12: 10:17
-           _2 = const bar() -> [return: bb2, unwind: bb6]; // scope 0 at $DIR/instrument_coverage.rs:10:12: 10:17
+           _2 = const bar() -> [return: bb3, unwind: bb2]; // scope 0 at $DIR/instrument_coverage.rs:10:12: 10:17
32                                            // ty::Const
33                                            // + ty: fn() -> bool {bar}
34                                            // + val: Value(Scalar(<ZST>))

37                                            // + literal: Const { ty: fn() -> bool {bar}, val: Value(Scalar(<ZST>)) }
39   
-       bb2: {
-       bb2: {
-           FakeRead(ForMatchedPlace, _2);   // scope 0 at $DIR/instrument_coverage.rs:10:12: 10:17
-           switchInt(_2) -> [false: bb4, otherwise: bb3]; // scope 0 at $DIR/instrument_coverage.rs:10:9: 12:10
+       bb2 (cleanup): {
+           resume;                          // scope 0 at $DIR/instrument_coverage.rs:8:1: 14:2
44   
45       bb3: {


-           falseEdges -> [real: bb5, imaginary: bb4]; // scope 0 at $DIR/instrument_coverage.rs:10:9: 12:10
+           FakeRead(ForMatchedPlace, _2);   // scope 0 at $DIR/instrument_coverage.rs:10:12: 10:17
+           switchInt(_2) -> [false: bb5, otherwise: bb4]; // scope 0 at $DIR/instrument_coverage.rs:10:9: 12:10
48   
49       bb4: {


+           falseEdge -> [real: bb6, imaginary: bb5]; // scope 0 at $DIR/instrument_coverage.rs:10:9: 12:10
+   
+       bb5: {
+       bb5: {
50           _1 = const ();                   // scope 0 at $DIR/instrument_coverage.rs:10:9: 12:10
51                                            // ty::Const
52                                            // + ty: ()

58           goto -> bb0;                     // scope 0 at $DIR/instrument_coverage.rs:9:5: 13:6
60   
-       bb5: {
+       bb6: {
+       bb6: {
62           _0 = const ();                   // scope 0 at $DIR/instrument_coverage.rs:11:13: 11:18
63                                            // ty::Const
64                                            // + ty: ()

68                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
69           StorageDead(_2);                 // scope 0 at $DIR/instrument_coverage.rs:13:5: 13:6
70           return;                          // scope 0 at $DIR/instrument_coverage.rs:14:2: 14:2
-   
-   
-       bb6 (cleanup): {
-           resume;                          // scope 0 at $DIR/instrument_coverage.rs:8:1: 14:2
76 + 
77 +     bb7: {


78 +         StorageDead(_4);                 // scope 0 at $DIR/instrument_coverage.rs:9:5: 13:6
- +         falseUnwind -> [real: bb1, cleanup: bb6]; // scope 0 at $DIR/instrument_coverage.rs:9:5: 13:6
+ +         falseUnwind -> [real: bb1, cleanup: bb2]; // scope 0 at $DIR/instrument_coverage.rs:9:5: 13:6
81   }
82   


thread '[mir-opt] mir-opt/instrument_coverage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/instrument_coverage/rustc.main.InstrumentCoverage.diff', src/tools/compiletest/src/runtest.rs:3171:25


failures:
    [mir-opt] mir-opt/instrument_coverage.rs
    [mir-opt] mir-opt/instrument_coverage.rs

test result: FAILED. 106 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:13:07
== clock drift check ==
  local time: Wed Jun 10 19:06:59 UTC 2020
  network time: Wed, 10 Jun 2020 19:06:59 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73011/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73011/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3449) (python)
##[section]Finishing: Finalize Job
