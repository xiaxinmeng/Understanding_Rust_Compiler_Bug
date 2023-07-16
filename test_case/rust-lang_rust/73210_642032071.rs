plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 9'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/debb96ae-f61b-4ec8-9f15-a8aa66e74813.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73210/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73210/merge:refs/remotes/pull/73210/merge
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
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/193
.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.978
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.168
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 15.678
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.966
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

 finished in 72.019
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/rustc/src/targets/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 4.430
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

 finished in 0.579
Build completed successfully in 1:40:26
Build completed successfully in 1:40:26
+ python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.63s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
...........F.................F.....F................................................................ 100/106
......
failures:

---- [mir-opt] mir-opt/const_prop/array_index.rs stdout ----
9       let mut _4: usize;                   // in scope 0 at $DIR/array_index.rs:5:18: 5:33
10       let mut _5: bool;                    // in scope 0 at $DIR/array_index.rs:5:18: 5:33
-           debug x => _1;                   // in scope 1 at $DIR/array_index.rs:5:9: 5:10
+ -         debug x => _1;                   // in scope 1 at $DIR/array_index.rs:5:9: 5:10
+ -         debug x => _1;                   // in scope 1 at $DIR/array_index.rs:5:9: 5:10
+ +         debug x => const 2u32;           // in scope 1 at $DIR/array_index.rs:5:9: 5:10
14   
15       bb0: {


thread '[mir-opt] mir-opt/const_prop/array_index.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/array_index/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3171:25

---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
11       let mut _7: bool;                    // in scope 0 at $DIR/optimizes_into_variable.rs:13:13: 13:34
12       let mut _9: Point;                   // in scope 0 at $DIR/optimizes_into_variable.rs:14:13: 14:36
-           debug x => _1;                   // in scope 1 at $DIR/optimizes_into_variable.rs:12:9: 12:10
+ -         debug x => _1;                   // in scope 1 at $DIR/optimizes_into_variable.rs:12:9: 12:10
+ -         debug x => _1;                   // in scope 1 at $DIR/optimizes_into_variable.rs:12:9: 12:10
+ +         debug x => const 4i32;           // in scope 1 at $DIR/optimizes_into_variable.rs:12:9: 12:10
15           let _3: i32;                     // in scope 1 at $DIR/optimizes_into_variable.rs:13:9: 13:10
-               debug y => _3;               // in scope 2 at $DIR/optimizes_into_variable.rs:13:9: 13:10
+ -             debug y => _3;               // in scope 2 at $DIR/optimizes_into_variable.rs:13:9: 13:10
+ -             debug y => _3;               // in scope 2 at $DIR/optimizes_into_variable.rs:13:9: 13:10
+ +             debug y => const 3i32;       // in scope 2 at $DIR/optimizes_into_variable.rs:13:9: 13:10
18               let _8: u32;                 // in scope 2 at $DIR/optimizes_into_variable.rs:14:9: 14:10
19               scope 3 {
-                   debug z => _8;           // in scope 3 at $DIR/optimizes_into_variable.rs:14:9: 14:10
+ -                 debug z => _8;           // in scope 3 at $DIR/optimizes_into_variable.rs:14:9: 14:10
+ +                 debug z => const 42u32;  // in scope 3 at $DIR/optimizes_into_variable.rs:14:9: 14:10
22           }
23       }


thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3171:25
---- [mir-opt] mir-opt/const_prop/repeat.rs stdout ----
10       let mut _5: usize;                   // in scope 0 at $DIR/repeat.rs:6:18: 6:28
10       let mut _5: usize;                   // in scope 0 at $DIR/repeat.rs:6:18: 6:28
11       let mut _6: bool;                    // in scope 0 at $DIR/repeat.rs:6:18: 6:28
-           debug x => _1;                   // in scope 1 at $DIR/repeat.rs:6:9: 6:10
+ -         debug x => _1;                   // in scope 1 at $DIR/repeat.rs:6:9: 6:10
+ -         debug x => _1;                   // in scope 1 at $DIR/repeat.rs:6:9: 6:10
+ +         debug x => const 42u32;          // in scope 1 at $DIR/repeat.rs:6:9: 6:10
15   
16       bb0: {


thread '[mir-opt] mir-opt/const_prop/repeat.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/repeat/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3171:25

failures:
    [mir-opt] mir-opt/const_prop/array_index.rs
    [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs
    [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs
    [mir-opt] mir-opt/const_prop/repeat.rs

test result: FAILED. 103 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
== clock drift check ==
  local time: Wed Jun 10 14:03:10 UTC 2020
  network time: Wed, 10 Jun 2020 14:03:10 GMT
== end clock drift check ==
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73210/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73210/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3546) (python)
##[section]Finishing: Finalize Job
