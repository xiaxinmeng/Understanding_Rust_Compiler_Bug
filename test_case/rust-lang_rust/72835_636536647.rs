plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a2432483-5177-4ccb-9165-3bcf7d2d4c6b.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72835/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72835/merge:refs/remotes/pull/72835/merge
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
.................................................................................................... 1700/10276
..................................................................................i................. 1800/10276
.................................................................................................... 1900/10276
.................................................................................................... 2000/10276
...i..i............................................................................................. 2100/10276
.............................................................................................i.iiii. 2200/10276
.................................................................................................... 2400/10276
.................................................................................................... 2500/10276
.................................................................................................... 2600/10276
.................................................................................................... 2700/10276
---
........................i...............i........................................................... 5200/10276
.................................................................................................... 5300/10276
........................................................................i........................... 5400/10276
..................................................................i................................. 5500/10276
..................................................................................ii.ii........i...i 5600/10276
.........................i.......................................................................... 5800/10276
.................................i.................................................................. 5900/10276
.......................................................................................ii........... 6000/10276
..........................i......................................................................... 6100/10276
..........................i......................................................................... 6100/10276
.................................................................................................... 6200/10276
.................................................................................................... 6300/10276
.................................................ii...i..ii...........i............................. 6400/10276
.................................................................................................... 6600/10276
.................................................................................................... 6700/10276
.................................................................................................... 6700/10276
..................................................................................i..ii............. 6800/10276
.................................................................................................... 7000/10276
.................................................................................................... 7100/10276
....................................i............................................................... 7200/10276
.................................................................................................... 7300/10276
---
.................................................................................................... 8200/10276
.................................................................................................... 8300/10276
.........................................................................i.......................... 8400/10276
.................................................................................................... 8500/10276
...........................iiiiii.iiiiii.i.......................................................... 8600/10276
.................................................................................................... 8800/10276
.................................................................................................... 8900/10276
.................................................................................................... 9000/10276
.................................................................................................... 9100/10276
---
.....................................................FFF.F.......................................... 100/105
.....
failures:

---- [mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs stdout ----
26         StorageLive(_3);                 // scope 0 at $DIR/inline-closure-borrows-arg.rs:12:9: 12:10
27         _3 = [closure@foo::<T>::{{closure}}#0]; // scope 0 at $DIR/inline-closure-borrows-arg.rs:12:13: 15:6
28                                          // closure
-                                          // + def_id: DefId(0:6 ~ inline_closure_borrows_arg[317d]::foo[0]::{{closure}}[0])
+                                          // + def_id: DefId(0:7 ~ inline_closure_borrows_arg[317d]::foo[0]::{{closure}}[0])
30                                          // + substs: [
31                                          //     T,


thread '[mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-closure-borrows-arg/rustc.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3166:25

---- [mir-opt] mir-opt/inline/inline-closure-captures.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-closure-captures.rs stdout ----
30         _5 = &_1;                        // scope 0 at $DIR/inline-closure-captures.rs:11:13: 11:24
31         _3 = [closure@foo::<T>::{{closure}}#0] { q: move _4, t: move _5 }; // scope 0 at $DIR/inline-closure-captures.rs:11:13: 11:24
32                                          // closure
-                                          // + def_id: DefId(0:6 ~ inline_closure_captures[317d]::foo[0]::{{closure}}[0])
+                                          // + def_id: DefId(0:7 ~ inline_closure_captures[317d]::foo[0]::{{closure}}[0])
34                                          // + substs: [
35                                          //     T,


thread '[mir-opt] mir-opt/inline/inline-closure-captures.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-closure-captures/rustc.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/inline/inline-closure.rs stdout ----
23         StorageLive(_3);                 // scope 0 at $DIR/inline-closure.rs:11:9: 11:10
23         StorageLive(_3);                 // scope 0 at $DIR/inline-closure.rs:11:9: 11:10
24         _3 = [closure@foo::<T>::{{closure}}#0]; // scope 0 at $DIR/inline-closure.rs:11:13: 11:24
25                                          // closure
-                                          // + def_id: DefId(0:6 ~ inline_closure[317d]::foo[0]::{{closure}}[0])
+                                          // + def_id: DefId(0:7 ~ inline_closure[317d]::foo[0]::{{closure}}[0])
27                                          // + substs: [
28                                          //     T,


thread '[mir-opt] mir-opt/inline/inline-closure.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-closure/rustc.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/inline/inline-retag.rs stdout ----
38         _10 = const bar::promoted[1];    // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
39                                          // ty::Const
40                                          // + ty: &i32
40                                          // + ty: &i32
-                                          // + val: Unevaluated(DefId(0:4 ~ inline_retag[317d]::bar[0]), [], Some(promoted[1]))
+                                          // + val: Unevaluated(DefId(0:5 ~ inline_retag[317d]::bar[0]), [], Some(promoted[1]))
42                                          // mir::Constant
43                                          // + span: $DIR/inline-retag.rs:12:7: 12:9
-                                          // + literal: Const { ty: &i32, val: Unevaluated(DefId(0:4 ~ inline_retag[317d]::bar[0]), [], Some(promoted[1])) }
+                                          // + literal: Const { ty: &i32, val: Unevaluated(DefId(0:5 ~ inline_retag[317d]::bar[0]), [], Some(promoted[1])) }
45         Retag(_10);                      // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
46         _4 = &(*_10);                    // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
47         Retag(_4);                       // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
52         _9 = const bar::promoted[0];     // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
53                                          // ty::Const
54                                          // + ty: &i32
54                                          // + ty: &i32
-                                          // + val: Unevaluated(DefId(0:4 ~ inline_retag[317d]::bar[0]), [], Some(promoted[0]))
+                                          // + val: Unevaluated(DefId(0:5 ~ inline_retag[317d]::bar[0]), [], Some(promoted[0]))
56                                          // mir::Constant
57                                          // + span: $DIR/inline-retag.rs:12:11: 12:14
-                                          // + literal: Const { ty: &i32, val: Unevaluated(DefId(0:4 ~ inline_retag[317d]::bar[0]), [], Some(promoted[0])) }
+                                          // + literal: Const { ty: &i32, val: Unevaluated(DefId(0:5 ~ inline_retag[317d]::bar[0]), [], Some(promoted[0])) }
59         Retag(_9);                       // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
60         _7 = &(*_9);                     // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
61         Retag(_7);                       // scope 1 at $DIR/inline-retag.rs:12:11: 12:14

thread '[mir-opt] mir-opt/inline/inline-retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-retag/rustc.bar.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3166:25

failures:
    [mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs
    [mir-opt] mir-opt/inline/inline-closure-captures.rs
---

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:08:25
Build completed unsuccessfully in 1:08:25
== clock drift check ==
  local time: Sun May 31 22:00:27 UTC 2020
  network time: Sun, 31 May 2020 22:00:27 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72835/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72835/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3340) (python)
##[section]Finishing: Finalize Job
