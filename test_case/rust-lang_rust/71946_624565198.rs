plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b1740fe7-891b-4113-8e8c-4c7005bc581d.sh

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
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
......................i............................................................................. 1800/9988
.................................................................................................... 1900/9988
.......................................i............................................................ 2000/9988
.................................................................................................... 2100/9988
.............................iiiii.................................................................. 2200/9988
.................................................................................................... 2400/9988
.................................................................................................... 2500/9988
.................................................................................................... 2600/9988
.................................................................................................... 2700/9988
---
................i...............i................................................................... 5100/9988
.................................................................................................... 5200/9988
..............................................................i..................................... 5300/9988
.....................................................i.............................................. 5400/9988
.........................................................ii.ii........i...i......................... 5500/9988
..................................................................................................i. 5600/9988
....i............................................................................................... 5800/9988
........................................ii.....................................i.................... 5900/9988
.................................................................................................... 6000/9988
.................................................................................................... 6100/9988
.................................................................................................... 6100/9988
............................................................................ii...i..ii...........i.. 6200/9988
.................................................................................................... 6400/9988
.................................................................................................... 6500/9988
.................................................................................................... 6600/9988
.................................................................................................... 6600/9988
........i..ii....................................................................................... 6700/9988
.................................................................................................... 6900/9988
.........i.......................................................................................... 7000/9988
.................................................................................................... 7100/9988
...................................................i................................................ 7200/9988
---
.................................................................................................... 7900/9988
.................................................................................................... 8000/9988
.................................................................................................... 8100/9988
...................i................................................................................ 8200/9988
.........................................................................iiiiii.iiiii.i............. 8300/9988
..........................i......................................................................... 8500/9988
.................................................................................................... 8600/9988
.................................................................................................... 8700/9988
.................................................................................................... 8800/9988
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 187 tests
iiii......i.............ii.i..........i..............................i..i..................i....i... 100/187
.........i.i.i...iii..iiiiiiiiiiii.iiii......................iii...............ii......

 finished in 5.562
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.152
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
failures:

---- [incremental] incremental/hashes/while_let_loops.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_let_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/while_let_loops.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `optimized_mir(change_loop_body)` should be clean but is not
   |
LL | / pub fn change_loop_body() {
LL | |     let mut _x = 0;
LL | |     while let Some(0u32) = None {
LL | |     while let Some(0u32) = None {
LL | |         _x = 2;
LL | |         break;
LL | |     }
LL | | }
   | |_^

error: `optimized_mir(change_loop_condition)` should be clean but is not
   |
LL | / pub fn change_loop_condition() {
LL | |     let mut _x = 0;
LL | |     while let Some(1u32) = None {
LL | |     while let Some(1u32) = None {
LL | |         _x = 1;
LL | |         break;
LL | |     }
LL | | }
   | |_^

error: `optimized_mir(add_break)` should be clean but is not
   |
LL | / pub fn add_break() {
LL | |     let mut _x = 0;
LL | |     while let Some(0u32) = None {
LL | |     while let Some(0u32) = None {
LL | |         _x = 1;
LL | |         break;
LL | |     }
LL | | }
   | |_^

error: `optimized_mir(change_break_label)` should be clean but is not
   |
   |
LL | / pub fn change_break_label() {
LL | |     let mut _x = 0;
LL | |     'outer: while let Some(0u32) = None {
LL | |         'inner: while let Some(0u32) = None {
LL | |     }
LL | | }
   | |_^


error: `optimized_mir(change_continue_label)` should be clean but is not
   |
LL | / pub fn change_continue_label() {
LL | |     let mut _x = 0;
LL | |     'outer: while let Some(0u32) = None {
LL | |     'outer: while let Some(0u32) = None {
LL | |         'inner: while let Some(0u32) = None {
LL | |     }
LL | | }
   | |_^


error: `optimized_mir(change_continue_to_break)` should be clean but is not
   |
LL | / pub fn change_continue_to_break() {
LL | |     let mut _x = 0;
LL | |     while let Some(0u32) = None {
---
test result: FAILED. 118 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:59:49
== clock drift check ==
  local time: Wed May  6 10:23:33 UTC 2020
  network time: Wed, 06 May 2020 10:23:33 GMT
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
Terminate orphan process: pid (3742) (python)
##[section]Finishing: Finalize Job
