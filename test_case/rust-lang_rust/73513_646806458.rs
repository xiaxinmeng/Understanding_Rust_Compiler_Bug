plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200614.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200614.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/380b736d-29f3-4d24-8e3c-db27483ffc14.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73513/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73513/merge:refs/remotes/pull/73513/merge
---
 ---> 31fea614d2f3
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> 4195cadf126d
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 4e90f6b48f05
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> dfa0a356d899
---
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
---
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
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
.................................................................................................... 1900/10329
.................................................................................................... 2000/10329
.................i..i............................................................................... 2100/10329
.................................................................................................... 2200/10329
.......iiiii........................................................................................ 2300/10329
.................................................................................................... 2500/10329
.................................................................................................... 2600/10329
.................................................................................................... 2700/10329
.................................................................................................... 2800/10329
---
.................................................................................................... 5300/10329
.........................................................................................i.......... 5400/10329
...................................................................................i................ 5500/10329
.................................................................................................... 5600/10329
..ii.ii........i...i................................................................................ 5700/10329
........................................................i........................................... 5900/10329
.................................................................................................... 6000/10329
..........ii.....................................i.................................................. 6100/10329
.................................................................................................... 6200/10329
.................................................................................................... 6200/10329
.................................................................................................... 6300/10329
.........................................................................ii...i..ii...........i..... 6400/10329
.................................................................................................... 6600/10329
.................................................................................................... 6700/10329
.................................................................................................... 6800/10329
.................................................................................................... 6800/10329
.......i..ii........................................................................................ 6900/10329
.................................................................................................... 7100/10329
..............................................................i..................................... 7200/10329
.................................................................................................... 7300/10329
.................................................................................................... 7400/10329
---
.................................................................................................... 8200/10329
.................................................................................................... 8300/10329
.................................................................................................... 8400/10329
.......i............................................................................................ 8500/10329
.............................................................iiiiii.iiiiii.i........................ 8600/10329
..................i................................................................................. 8800/10329
.................................................................................................... 8900/10329
.................................................................................................... 9000/10329
.................................................................................................... 9100/10329
---
............FF...F...F.......F.....F.........................i......F............................... 100/107
.......
failures:

---- [mir-opt] mir-opt/const_prop/bad_op_div_by_zero.rs stdout ----
46 +                                          // + span: $DIR/bad_op_div_by_zero.rs:5:14: 5:19
47 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
48           assert(!move _4, "attempt to divide by zero") -> bb1; // scope 1 at $DIR/bad_op_div_by_zero.rs:5:14: 5:19
+                                            // ty::Const
+                                            // + ty: i32
+                                            // + val: Value(Scalar(0x00000001))
+                                            // mir::Constant
+                                            // + span: $DIR/bad_op_div_by_zero.rs:5:14: 5:15
+                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
50   
51       bb1: {

91 +                                          // mir::Constant
91 +                                          // mir::Constant
92 +                                          // + span: $DIR/bad_op_div_by_zero.rs:5:14: 5:19
93 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
+                                            // ty::Const
+                                            // + ty: i32
+                                            // + val: Value(Scalar(0x00000001))
+                                            // mir::Constant
+                                            // + span: $DIR/bad_op_div_by_zero.rs:5:14: 5:15
+                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
95   
96       bb2: {


thread '[mir-opt] mir-opt/const_prop/bad_op_div_by_zero.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_div_by_zero/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3194:25


---- [mir-opt] mir-opt/const_prop/bad_op_mod_by_zero.rs stdout ----
46 +                                          // + span: $DIR/bad_op_mod_by_zero.rs:5:14: 5:19
47 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
48           assert(!move _4, "attempt to calculate the remainder with a divisor of zero") -> bb1; // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:14: 5:19
+                                            // ty::Const
+                                            // + ty: i32
+                                            // + val: Value(Scalar(0x00000001))
+                                            // mir::Constant
+                                            // + span: $DIR/bad_op_mod_by_zero.rs:5:14: 5:15
+                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
50   
51       bb1: {

91 +                                          // mir::Constant
91 +                                          // mir::Constant
92 +                                          // + span: $DIR/bad_op_mod_by_zero.rs:5:14: 5:19
93 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
+                                            // ty::Const
+                                            // + ty: i32
+                                            // + val: Value(Scalar(0x00000001))
+                                            // mir::Constant
+                                            // + span: $DIR/bad_op_mod_by_zero.rs:5:14: 5:15
+                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
95   
96       bb2: {


thread '[mir-opt] mir-opt/const_prop/bad_op_mod_by_zero.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_mod_by_zero/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3194:25
---- [mir-opt] mir-opt/const_prop/checked_add.rs stdout ----
---- [mir-opt] mir-opt/const_prop/checked_add.rs stdout ----
34 +                                          // + span: $DIR/checked_add.rs:5:18: 5:23
35 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
36 +         assert(!const false, "attempt to add with overflow") -> bb1; // scope 0 at $DIR/checked_add.rs:5:18: 5:23
- +                                          // ty::Const
+                                            // ty::Const
38 +                                          // + ty: bool
39 +                                          // + val: Value(Scalar(0x00))


41 +                                          // + span: $DIR/checked_add.rs:5:18: 5:23
42 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
+ +                                          // ty::Const
+                                            // + ty: u32
+                                            // + val: Value(Scalar(0x00000001))
+                                            // mir::Constant
+                                            // + span: $DIR/checked_add.rs:5:18: 5:19
+                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000001)) }
+                                            // ty::Const
+                                            // + ty: u32
+                                            // + val: Value(Scalar(0x00000001))
+                                            // mir::Constant
+                                            // + span: $DIR/checked_add.rs:5:22: 5:23
+                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000001)) }
44   
45       bb1: {


thread '[mir-opt] mir-opt/const_prop/checked_add.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/checked_add/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3194:25
---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
23                                            // mir::Constant
23                                            // mir::Constant
24 -                                          // + span: $DIR/indirect.rs:5:14: 5:18
25 -                                          // + literal: Const { ty: u32, val: Value(Scalar(0x00000002)) }
- -         _3 = CheckedAdd(move _2, const 1u8); // scope 0 at $DIR/indirect.rs:5:13: 5:29
+ -         _3 = CheckedAdd(_2, const 1u8);  // scope 0 at $DIR/indirect.rs:5:13: 5:29
27 +                                          // + span: $DIR/indirect.rs:5:13: 5:25
28 +                                          // + literal: Const { ty: u8, val: Value(Scalar(0x02)) }
29 +         _3 = (const 3u8, const false);   // scope 0 at $DIR/indirect.rs:5:13: 5:29

37 -         assert(!move (_3.1: bool), "attempt to add with overflow") -> bb1; // scope 0 at $DIR/indirect.rs:5:13: 5:29
38 +                                          // + span: $DIR/indirect.rs:5:13: 5:29
39 +                                          // + literal: Const { ty: u8, val: Value(Scalar(0x03)) }
- +                                          // ty::Const
+                                            // ty::Const
41 +                                          // + ty: bool
42 +                                          // + val: Value(Scalar(0x00))

50 +                                          // mir::Constant
50 +                                          // mir::Constant
51 +                                          // + span: $DIR/indirect.rs:5:13: 5:29
52 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
+ +                                          // ty::Const
+                                            // + ty: u8
+                                            // + val: Value(Scalar(0x01))
+                                            // mir::Constant
+                                            // + span: $DIR/indirect.rs:5:28: 5:29
+                                            // + literal: Const { ty: u8, val: Value(Scalar(0x01)) }
54   
55       bb1: {


thread '[mir-opt] mir-opt/const_prop/indirect.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/indirect/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3194:25
---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
47 +                                          // + span: $DIR/optimizes_into_variable.rs:12:13: 12:18
48 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
49 +         assert(!const false, "attempt to add with overflow") -> bb1; // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
- +                                          // ty::Const
+                                            // ty::Const
51 +                                          // + ty: bool
52 +                                          // + val: Value(Scalar(0x00))


54 +                                          // + span: $DIR/optimizes_into_variable.rs:12:13: 12:18
55 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
+ +                                          // ty::Const
+                                            // + ty: i32
+                                            // + val: Value(Scalar(0x00000002))
+                                            // mir::Constant
+                                            // + span: $DIR/optimizes_into_variable.rs:12:13: 12:14
+                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000002)) }
+                                            // ty::Const
+                                            // + ty: i32
+                                            // + val: Value(Scalar(0x00000002))
+                                            // mir::Constant
+                                            // + span: $DIR/optimizes_into_variable.rs:12:17: 12:18
+                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000002)) }
57   
58       bb1: {


thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable/64bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3194:25
---- [mir-opt] mir-opt/const_prop/return_place.rs stdout ----
---- [mir-opt] mir-opt/const_prop/return_place.rs stdout ----
29 +                                          // + span: $DIR/return_place.rs:6:5: 6:10
30 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
31 +         assert(!const false, "attempt to add with overflow") -> bb1; // scope 0 at $DIR/return_place.rs:6:5: 6:10
- +                                          // ty::Const
+                                            // ty::Const
33 +                                          // + ty: bool
34 +                                          // + val: Value(Scalar(0x00))


36 +                                          // + span: $DIR/return_place.rs:6:5: 6:10
37 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
+ +                                          // ty::Const
+                                            // + ty: u32
+                                            // + val: Value(Scalar(0x00000002))
+                                            // mir::Constant
+                                            // + span: $DIR/return_place.rs:6:5: 6:6
+                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000002)) }
+                                            // ty::Const
+                                            // + ty: u32
+                                            // + val: Value(Scalar(0x00000002))
+                                            // mir::Constant
+                                            // + span: $DIR/return_place.rs:6:9: 6:10
+                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000002)) }
39   
40       bb1: {


thread '[mir-opt] mir-opt/const_prop/return_place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/return_place/rustc.add.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3194:25
---- [mir-opt] mir-opt/issue-41697.rs stdout ----
---- [mir-opt] mir-opt/issue-41697.rs stdout ----
19                                          // + span: $DIR/issue-41697.rs:18:21: 18:22
20                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
21         assert(!move (_1.1: bool), "attempt to add with overflow") -> [success: bb2, unwind: bb1]; // scope 0 at $DIR/issue-41697.rs:18:19: 18:22
+                                          // ty::Const
+                                          // + ty: usize
+                                          // + val: Value(Scalar(0x0000000000000001))
+                                          // mir::Constant
+                                          // + span: $DIR/issue-41697.rs:18:19: 18:20
+                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
+                                          // ty::Const
+                                          // + ty: usize
+                                          // + val: Value(Scalar(0x0000000000000001))
+                                          // mir::Constant
+                                          // + span: $DIR/issue-41697.rs:18:21: 18:22
+                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
23 
23 
24     bb1 (cleanup): {

thread '[mir-opt] mir-opt/issue-41697.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue-41697/64bit/rustc.{{impl}}-{{constant}}.SimplifyCfg-qualify-consts.after.mir', src/tools/compiletest/src/runtest.rs:3194:25

failures:
    [mir-opt] mir-opt/const_prop/bad_op_div_by_zero.rs
    [mir-opt] mir-opt/const_prop/bad_op_mod_by_zero.rs
---

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:01:00
Build completed unsuccessfully in 1:01:00
== clock drift check ==
  local time: Fri Jun 19 18:27:11 UTC 2020
  network time: Fri, 19 Jun 2020 18:27:11 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73513/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73513/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3577) (python)
##[section]Finishing: Finalize Job
