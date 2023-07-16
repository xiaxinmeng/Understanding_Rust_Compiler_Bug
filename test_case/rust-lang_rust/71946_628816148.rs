plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 60'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c7e2c6a3-b7a2-4c11-936a-f9bf613ff307.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71946/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71946/merge:refs/remotes/pull/71946/merge
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
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 188 tests
iiii......i.............iiFi..........i...............................i..i..................i....i.. 100/188
..........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---- [codegen] codegen/consts.rs stdout ----
---- [codegen] codegen/consts.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/usr/lib/llvm-8/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll" "/checkout/src/test/codegen/consts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/consts.rs:13:11: error: CHECK: expected string not found in input
// CHECK:***@alloc7 = {{.*}}, align 2
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:11:1: note: scanning from here
@alloc8 = private unnamed_addr constant <{ [4 x i8] }> zeroinitializer, align 2
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:11:64: note: possible intended match here
@alloc8 = private unnamed_addr constant <{ [4 x i8] }> zeroinitializer, align 2
/checkout/src/test/codegen/consts.rs:46:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/consts.rs:46:12: error: CHECK: expected string not found in input
 // CHECK: load %"E<i16, [i16; 3]>"*, %"E<i16, [i16; 3]>"** bitcast (<{ i8*, [0 x i8] }>* [[LOW_HIGH]] to %"E<i16, [i16; 3]>"**),
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:41:28: note: scanning from here
define i64 @low_align_const() unnamed_addr #0 {
                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:41:28: note: uses undefined variable "LOW_HIGH"
define i64 @low_align_const() unnamed_addr #0 {
                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:44:2: note: possible intended match here
 %_2 = load %"E<i16, [i16; 3]>"*, %"E<i16, [i16; 3]>"** bitcast (<{ i8*, [0 x i8] }>* @1 to %"E<i16, [i16; 3]>"**), align 8, !nonnull !2
/checkout/src/test/codegen/consts.rs:54:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/consts.rs:54:12: error: CHECK: expected string not found in input
 // CHECK: load %"E<i16, i32>"*, %"E<i16, i32>"** bitcast (<{ i8*, [0 x i8] }>* [[LOW_HIGH]] to %"E<i16, i32>"**),
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:54:29: note: scanning from here
define i64 @high_align_const() unnamed_addr #0 {
                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:54:29: note: uses undefined variable "LOW_HIGH"
define i64 @high_align_const() unnamed_addr #0 {
                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:57:6: note: possible intended match here
 %_2 = load %"E<i16, i32>"*, %"E<i16, i32>"** bitcast (<{ i8*, [0 x i8] }>* @1 to %"E<i16, i32>"**), align 8, !nonnull !2

------------------------------------------


---
test result: FAILED. 147 passed; 1 failed; 40 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:08:42
Build completed unsuccessfully in 1:08:42
== clock drift check ==
  local time: Thu May 14 18:36:58 UTC 2020
  network time: Thu, 14 May 2020 18:36:58 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71946/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71946/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3556) (python)
##[section]Finishing: Finalize Job
