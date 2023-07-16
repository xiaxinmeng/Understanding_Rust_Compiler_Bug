plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6e7712b0-e9e2-40aa-a35c-e5983a6e1350.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73130/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73130/merge:refs/remotes/pull/73130/merge
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
.............................i...............i...................................................... 5200/10289
.................................................................................................... 5300/10289
.............................................................................i...................... 5400/10289
.......................................................................i............................ 5500/10289
........................................................................................ii.ii....... 5600/10289
.i...i.............................................................................................. 5700/10289
.......................................i............................................................ 5900/10289
.............................................................................................ii..... 6000/10289
................................i................................................................... 6100/10289
.................................................................................................... 6200/10289
.................................................................................................... 6200/10289
.................................................................................................... 6300/10289
.......................................................ii...i..ii...........i....................... 6400/10289
.................................................................................................... 6600/10289
.................................................................................................... 6700/10289
.................................................................................................... 6700/10289
........................................................................................i..ii....... 6800/10289
.................................................................................................... 7000/10289
.................................................................................................... 7100/10289
..........................................i......................................................... 7200/10289
.................................................................................................... 7300/10289
---
.................................................................................................... 8200/10289
.................................................................................................... 8300/10289
..................................................................................i................. 8400/10289
.................................................................................................... 8500/10289
....................................iiiiii.iiiiii.i................................................. 8600/10289
.................................................................................................... 8800/10289
.................................................................................................... 8900/10289
.................................................................................................... 9000/10289
.................................................................................................... 9100/10289
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/193
.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.555
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.155
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 14.577
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.885
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-101a02b96cd72ebe

running 0 tests

---
Set({"/checkout/src/librustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_serialize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 212 tests
......................i...ii........................................................................ 100/212
i........................................iiiiii......i..............iii............................. 200/212
........ii..

 finished in 69.933
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
doc tests for: /checkout/src/doc/rustc/src/targets/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 4.242
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
Checking "alias-1" ... OK
Checking "alias-2" ... OK
Checking "alias-3" ... OK
Checking "alias" ... OK
Checking "basic" ... OK
Checking "deduplication" ... OK
Checking "enum-option" ... OK
Checking "filter-crate" ... OK
Checking "fn-forget" ... OK
Checking "from_u" ... OK
Checking "keyword" ... OK
Checking "macro-check" ... OK
Checking "macro-print" ... OK
Checking "multi-query" ... OK
Checking "never" ... OK
Checking "quoted" ... OK
Checking "return-specific-literal" ... OK
Checking "return-specific" ... OK
Checking "should-fail" ... OK
Checking "string-from_ut" ... OK
Checking "struct-vec" ... OK
Checking "vec-new" ... OK
Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 7 tests
.......
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 13 tests
.iiiiiii.iii.

 finished in 0.565
Build completed successfully in 1:34:20
Build completed successfully in 1:34:20
+ python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.22s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
2 + // MIR for `main` after ConstProp
3   
4   fn main() -> () {
-       let mut _0: ();                      // return place in scope 0 at $DIR/discriminant.rs:5:11: 5:11
-       let _1: i32;                         // in scope 0 at $DIR/discriminant.rs:6:9: 6:10
-       let mut _2: i32;                     // in scope 0 at $DIR/discriminant.rs:6:13: 6:64
-       let mut _3: std::option::Option<bool>; // in scope 0 at $DIR/discriminant.rs:6:34: 6:44
-       let mut _4: isize;                   // in scope 0 at $DIR/discriminant.rs:6:21: 6:31
+       let mut _0: ();                      // return place in scope 0 at $DIR/discriminant.rs:10:11: 10:11
+       let _1: i32;                         // in scope 0 at $DIR/discriminant.rs:11:9: 11:10
+       let mut _2: i32;                     // in scope 0 at $DIR/discriminant.rs:11:13: 11:64
+       let mut _3: std::option::Option<bool>; // in scope 0 at $DIR/discriminant.rs:11:34: 11:44
+       let mut _4: isize;                   // in scope 0 at $DIR/discriminant.rs:11:21: 11:31
-           debug x => _1;                   // in scope 1 at $DIR/discriminant.rs:6:9: 6:10
+           debug x => _1;                   // in scope 1 at $DIR/discriminant.rs:11:9: 11:10
12       }
13   
13   
14       bb0: {

-           StorageLive(_1);                 // scope 0 at $DIR/discriminant.rs:6:9: 6:10
-           StorageLive(_2);                 // scope 0 at $DIR/discriminant.rs:6:13: 6:64
-           StorageLive(_3);                 // scope 0 at $DIR/discriminant.rs:6:34: 6:44
- -         _3 = std::option::Option::<bool>::Some(const true); // scope 0 at $DIR/discriminant.rs:6:34: 6:44
- +         _3 = const std::option::Option::<bool>::Some(true); // scope 0 at $DIR/discriminant.rs:6:34: 6:44
+           StorageLive(_1);                 // scope 0 at $DIR/discriminant.rs:11:9: 11:10
+           StorageLive(_2);                 // scope 0 at $DIR/discriminant.rs:11:13: 11:64
+           StorageLive(_3);                 // scope 0 at $DIR/discriminant.rs:11:34: 11:44
+ -         _3 = std::option::Option::<bool>::Some(const true); // scope 0 at $DIR/discriminant.rs:11:34: 11:44
+ +         _3 = const std::option::Option::<bool>::Some(true); // scope 0 at $DIR/discriminant.rs:11:34: 11:44
20                                            // ty::Const
21 -                                          // + ty: bool
22 +                                          // + ty: std::option::Option<bool>

23                                            // + val: Value(Scalar(0x01))
24                                            // mir::Constant
- -                                          // + span: $DIR/discriminant.rs:6:39: 6:43
+ -                                          // + span: $DIR/discriminant.rs:11:39: 11:43
26 -                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
- -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:6:21: 6:31
- -         switchInt(move _4) -> [1isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:6:21: 6:31
- +                                          // + span: $DIR/discriminant.rs:6:34: 6:44
+ -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+ -         switchInt(move _4) -> [1isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+ +                                          // + span: $DIR/discriminant.rs:11:34: 11:44
30 +                                          // + literal: Const { ty: std::option::Option<bool>, val: Value(Scalar(0x01)) }
- +         _4 = const 1isize;               // scope 0 at $DIR/discriminant.rs:6:21: 6:31
+ +         _4 = const 1isize;               // scope 0 at $DIR/discriminant.rs:11:21: 11:31
32 +                                          // ty::Const
33 +                                          // + ty: isize
34 +                                          // + val: Value(Scalar(0x00000001))
35 +                                          // mir::Constant
35 +                                          // mir::Constant
- +                                          // + span: $DIR/discriminant.rs:6:21: 6:31
+ +                                          // + span: $DIR/discriminant.rs:11:21: 11:31
37 +                                          // + literal: Const { ty: isize, val: Value(Scalar(0x00000001)) }
- +         switchInt(const 1isize) -> [1isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:6:21: 6:31
+ +         switchInt(const 1isize) -> [1isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
39 +                                          // ty::Const
40 +                                          // + ty: isize
41 +                                          // + val: Value(Scalar(0x00000001))
42 +                                          // mir::Constant
42 +                                          // mir::Constant
- +                                          // + span: $DIR/discriminant.rs:6:21: 6:31
+ +                                          // + span: $DIR/discriminant.rs:11:21: 11:31
44 +                                          // + literal: Const { ty: isize, val: Value(Scalar(0x00000001)) }
46   

47       bb1: {
47       bb1: {
-           _2 = const 10i32;                // scope 0 at $DIR/discriminant.rs:6:59: 6:61
+           _2 = const 10i32;                // scope 0 at $DIR/discriminant.rs:11:59: 11:61
49                                            // ty::Const
50                                            // + ty: i32
51                                            // + val: Value(Scalar(0x0000000a))
52                                            // mir::Constant
52                                            // mir::Constant
-                                            // + span: $DIR/discriminant.rs:6:59: 6:61
+                                            // + span: $DIR/discriminant.rs:11:59: 11:61
54                                            // + literal: Const { ty: i32, val: Value(Scalar(0x0000000a)) }
-           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:6:13: 6:64
+           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
57   
58       bb2: {


- -         switchInt(((_3 as Some).0: bool)) -> [false: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:6:26: 6:30
- +         switchInt(const true) -> [false: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:6:26: 6:30
- +                                          // ty::Const
- +                                          // + ty: bool
- +                                          // + val: Value(Scalar(0x01))
- +                                          // mir::Constant
- +                                          // + span: $DIR/discriminant.rs:6:26: 6:30
- +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
+           switchInt(((_3 as Some).0: bool)) -> [false: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:26: 11:30
68   
69       bb3: {


-           _2 = const 42i32;                // scope 0 at $DIR/discriminant.rs:6:47: 6:49
+           _2 = const 42i32;                // scope 0 at $DIR/discriminant.rs:11:47: 11:49
71                                            // ty::Const
72                                            // + ty: i32
73                                            // + val: Value(Scalar(0x0000002a))
74                                            // mir::Constant
74                                            // mir::Constant
-                                            // + span: $DIR/discriminant.rs:6:47: 6:49
+                                            // + span: $DIR/discriminant.rs:11:47: 11:49
76                                            // + literal: Const { ty: i32, val: Value(Scalar(0x0000002a)) }
-           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:6:13: 6:64
+           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
79   
80       bb4: {


-           _1 = Add(move _2, const 0i32);   // scope 0 at $DIR/discriminant.rs:6:13: 6:68
+           _1 = Add(move _2, const 0i32);   // scope 0 at $DIR/discriminant.rs:11:13: 11:68
82                                            // ty::Const
83                                            // + ty: i32
84                                            // + val: Value(Scalar(0x00000000))
85                                            // mir::Constant
85                                            // mir::Constant
-                                            // + span: $DIR/discriminant.rs:6:67: 6:68
+                                            // + span: $DIR/discriminant.rs:11:67: 11:68
87                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
-           StorageDead(_2);                 // scope 0 at $DIR/discriminant.rs:6:67: 6:68
-           StorageDead(_3);                 // scope 0 at $DIR/discriminant.rs:6:68: 6:69
-           _0 = const ();                   // scope 0 at $DIR/discriminant.rs:5:11: 7:2
+           StorageDead(_2);                 // scope 0 at $DIR/discriminant.rs:11:67: 11:68
+           StorageDead(_3);                 // scope 0 at $DIR/discriminant.rs:11:68: 11:69
+           _0 = const ();                   // scope 0 at $DIR/discriminant.rs:10:11: 12:2
91                                            // ty::Const
92                                            // + ty: ()
93                                            // + val: Value(Scalar(<ZST>))
94                                            // mir::Constant
94                                            // mir::Constant
-                                            // + span: $DIR/discriminant.rs:5:11: 7:2
+                                            // + span: $DIR/discriminant.rs:10:11: 12:2
96                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
-           StorageDead(_1);                 // scope 0 at $DIR/discriminant.rs:7:1: 7:2
-           return;                          // scope 0 at $DIR/discriminant.rs:7:2: 7:2
+           StorageDead(_1);                 // scope 0 at $DIR/discriminant.rs:12:1: 12:2
+           return;                          // scope 0 at $DIR/discriminant.rs:12:2: 12:2
100   }
101   


thread '[mir-opt] mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3171:25


failures:
    [mir-opt] mir-opt/const_prop/discriminant.rs
    [mir-opt] mir-opt/const_prop/discriminant.rs

test result: FAILED. 105 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
== clock drift check ==
  local time: Tue Jun  9 02:26:01 UTC 2020
  network time: Tue, 09 Jun 2020 02:26:01 GMT
== end clock drift check ==
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73130/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73130/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3479) (python)
##[section]Finishing: Finalize Job
