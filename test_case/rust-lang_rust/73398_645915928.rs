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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ac5dd478-8246-4657-83b0-554e48cd5073.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73398/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73398/merge:refs/remotes/pull/73398/merge
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
.................................................................................................... 1900/10322
.................................................................................................... 2000/10322
.................i..i............................................................................... 2100/10322
.................................................................................................... 2200/10322
.......iiiii........................................................................................ 2300/10322
.................................................................................................... 2500/10322
...................F................................................................................ 2600/10322
.................................................................................................... 2700/10322
.................................................................................................... 2800/10322
---
.................................................................................................... 5300/10322
........................................................................................i........... 5400/10322
..................................................................................i................. 5500/10322
.................................................................................................... 5600/10322
.ii.ii........i...i................................................................................. 5700/10322
.......................................................i............................................ 5900/10322
.................................................................................................... 6000/10322
.........ii.....................................i................................................... 6100/10322
.................................................................................................... 6200/10322
.................................................................................................... 6200/10322
.................................................................................................... 6300/10322
........................................................................ii...i..ii...........i...... 6400/10322
.................................................................................................... 6600/10322
.................................................................................................... 6700/10322
.................................................................................................... 6800/10322
.................................................................................................... 6800/10322
......i..ii......................................................................................... 6900/10322
.................................................................................................... 7100/10322
.............................................................i...................................... 7200/10322
.................................................................................................... 7300/10322
.................................................................................................... 7400/10322
---
.................................................................................................... 8200/10322
.................................................................................................... 8300/10322
.................................................................................................... 8400/10322
...i................................................................................................ 8500/10322
.........................................................iiiiii.iiiiii.i............................ 8600/10322
..............i..................................................................................... 8800/10322
.................................................................................................... 8900/10322
.................................................................................................... 9000/10322
.................................................................................................... 9100/10322
---

---- [ui] ui/consts/const-eval/const_raw_ptr_ops.rs stdout ----
diff of stderr:

4 LL | const X: bool = unsafe { &1 as *const i32 == &2 as *const i32 };
6    |
6    |
+    = note: see issue #53020 <***/issues/53020> for more information
7    = note: It is conceptually impossible for const eval to know in all cases whether two pointers are equal. While sometimes it is clear (the address of a non-zst static item is never equal to the address of another non-zst static item), comparing an integer address with any allocation's address is impossible to do at compile-time.
-    = note: That said, there's the `ptr_guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `ptr_guaranteed_eq` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
+    = note: That said, there's the `<*const T>::guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `<*const T>::guaranteed_ne` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
10 error: pointers cannot be compared in a meaningful way during const eval.
11   --> $DIR/const_raw_ptr_ops.rs:6:27


13 LL | const X2: bool = unsafe { 42 as *const i32 == 43 as *const i32 };
15    |
15    |
+    = note: see issue #53020 <***/issues/53020> for more information
16    = note: It is conceptually impossible for const eval to know in all cases whether two pointers are equal. While sometimes it is clear (the address of a non-zst static item is never equal to the address of another non-zst static item), comparing an integer address with any allocation's address is impossible to do at compile-time.
-    = note: That said, there's the `ptr_guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `ptr_guaranteed_eq` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
+    = note: That said, there's the `<*const T>::guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `<*const T>::guaranteed_ne` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/const_raw_ptr_ops.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_raw_ptr_ops.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be compared in a meaningful way during const eval.
  --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:4:26
   |
LL | const X: bool = unsafe { &1 as *const i32 == &2 as *const i32 }; //~ ERROR cannot be compared
   |
   |
   = note: see issue #53020 <***/issues/53020> for more information
   = note: It is conceptually impossible for const eval to know in all cases whether two pointers are equal. While sometimes it is clear (the address of a non-zst static item is never equal to the address of another non-zst static item), comparing an integer address with any allocation's address is impossible to do at compile-time.
   = note: That said, there's the `<*const T>::guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `<*const T>::guaranteed_ne` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
error: pointers cannot be compared in a meaningful way during const eval.
  --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:6:27
   |
   |
LL | const X2: bool = unsafe { 42 as *const i32 == 43 as *const i32 }; //~ ERROR cannot be compared
   |
   |
   = note: see issue #53020 <***/issues/53020> for more information
   = note: It is conceptually impossible for const eval to know in all cases whether two pointers are equal. While sometimes it is clear (the address of a non-zst static item is never equal to the address of another non-zst static item), comparing an integer address with any allocation's address is impossible to do at compile-time.
   = note: That said, there's the `<*const T>::guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `<*const T>::guaranteed_ne` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/error-codes/E0395.rs stdout ----
diff of stderr:

4 LL | static BAZ: bool = unsafe { (&FOO as *const i32) == (&BAR as *const i32) };
6    |
6    |
+    = note: see issue #53020 <***/issues/53020> for more information
7    = note: It is conceptually impossible for const eval to know in all cases whether two pointers are equal. While sometimes it is clear (the address of a non-zst static item is never equal to the address of another non-zst static item), comparing an integer address with any allocation's address is impossible to do at compile-time.
-    = note: That said, there's the `ptr_guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `ptr_guaranteed_eq` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
+    = note: That said, there's the `<*const T>::guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `<*const T>::guaranteed_ne` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0395/E0395.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0395.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0395.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0395" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0395/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be compared in a meaningful way during const eval.
  --> /checkout/src/test/ui/error-codes/E0395.rs:4:29
   |
LL | static BAZ: bool = unsafe { (&FOO as *const i32) == (&BAR as *const i32) };
   |
   |
   = note: see issue #53020 <***/issues/53020> for more information
   = note: It is conceptually impossible for const eval to know in all cases whether two pointers are equal. While sometimes it is clear (the address of a non-zst static item is never equal to the address of another non-zst static item), comparing an integer address with any allocation's address is impossible to do at compile-time.
   = note: That said, there's the `<*const T>::guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `<*const T>::guaranteed_ne` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-25826.rs stdout ----
diff of stderr:

4 LL |     const A: bool = unsafe { id::<u8> as *const () < id::<u16> as *const () };
6    |
6    |
+    = note: see issue #53020 <***/issues/53020> for more information
7    = note: It is conceptually impossible for const eval to know in all cases whether two pointers are equal. While sometimes it is clear (the address of a non-zst static item is never equal to the address of another non-zst static item), comparing an integer address with any allocation's address is impossible to do at compile-time.
-    = note: That said, there's the `ptr_guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `ptr_guaranteed_eq` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
+    = note: That said, there's the `<*const T>::guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `<*const T>::guaranteed_ne` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25826/issue-25826.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-25826.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-25826.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25826" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25826/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be compared in a meaningful way during const eval.
  --> /checkout/src/test/ui/issues/issue-25826.rs:3:30
   |
LL |     const A: bool = unsafe { id::<u8> as *const () < id::<u16> as *const () };
   |
   |
   = note: see issue #53020 <***/issues/53020> for more information
   = note: It is conceptually impossible for const eval to know in all cases whether two pointers are equal. While sometimes it is clear (the address of a non-zst static item is never equal to the address of another non-zst static item), comparing an integer address with any allocation's address is impossible to do at compile-time.
   = note: That said, there's the `<*const T>::guaranteed_eq` intrinsic which returns `true` for all comparisons where CTFE is sure that two addresses are equal. The mirror intrinsic `<*const T>::guaranteed_ne` returns `true` for all comparisons where CTFE is sure that two addresses are inequal.
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 10253 passed; 3 failed; 66 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:02:32
Build completed unsuccessfully in 1:02:32
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
== clock drift check ==
  local time: Thu Jun 18 09:58:20 UTC 2020
  network time: Thu, 18 Jun 2020 09:58:20 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73398/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73398/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3592) (python)
##[section]Finishing: Finalize Job
