plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 65'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d3950e23-7aed-4b5f-ac8d-c45350af3853.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72904/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72904/merge:refs/remotes/pull/72904/merge
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
.................................................................................................... 1700/10277
...................................................................................i................ 1800/10277
.................................................................................................... 1900/10277
.................................................................................................... 2000/10277
....i..i............................................................................................ 2100/10277
..............................................................................................iiiii. 2200/10277
.................................................................................................... 2400/10277
.................................................................................................... 2500/10277
.................................................................................................... 2600/10277
.................................................................................................... 2700/10277
---
.........................i...............i.......................................................... 5200/10277
.................................................................................................... 5300/10277
.........................................................................i.......................... 5400/10277
...................................................................i................................ 5500/10277
...................................................................................ii.ii........i... 5600/10277
..........................i......................................................................... 5800/10277
..................................i................................................................. 5900/10277
........................................................................................ii.......... 6000/10277
...........................i........................................................................ 6100/10277
...........................i........................................................................ 6100/10277
.................................................................................................... 6200/10277
.................................................................................................... 6300/10277
..................................................ii...i..ii...........i............................ 6400/10277
.................................................................................................... 6600/10277
.................................................................................................... 6700/10277
.................................................................................................... 6700/10277
...................................................................................i..ii............ 6800/10277
.................................................................................................... 7000/10277
.................................................................................................... 7100/10277
.....................................i.............................................................. 7200/10277
............................................................................................F....... 7300/10277
---
.................................................................................................... 8200/10277
.................................................................................................... 8300/10277
..........................................................................i......................... 8400/10277
.................................................................................................... 8500/10277
............................iiiiii.ii.iiiii......................................................... 8600/10277
.................................................................................................... 8800/10277
.................................................................................................... 8900/10277
.................................................................................................... 9000/10277
.................................................................................................... 9100/10277
---

---- [ui] ui/codemap_tests/unicode.rs stdout ----
diff of stderr:

4 LL | extern "路濫狼á́́" fn foo() {}
5    |        ^^^^^^^^^ invalid ABI
6    |
-    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
+    = help: valid ABIs: Rust, C, cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/unicode.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args codemap_tests/unicode.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/unicode.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0703]: invalid ABI: found `路濫狼á́́`
   |
   |
LL | extern "路濫狼á́́" fn foo() {} //~ ERROR invalid ABI
   |        ^^^^^^^^^ invalid ABI
   |
   = help: valid ABIs: Rust, C, cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
error: aborting due to previous error

For more information about this error, try `rustc --explain E0703`.

---

4 LL |   "invalid-ab_isize"
5    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
6    |
-    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
+    = help: valid ABIs: Rust, C, cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/issue-8537.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-8537.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-8537.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0703]: invalid ABI: found `invalid-ab_isize`
   |
   |
LL |   "invalid-ab_isize" //~ ERROR invalid ABI
   |   ^^^^^^^^^^^^^^^^^^ invalid ABI
   |
   = help: valid ABIs: Rust, C, cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
error: aborting due to previous error

For more information about this error, try `rustc --explain E0703`.

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:04
Build completed unsuccessfully in 1:05:04
== clock drift check ==
  local time: Tue Jun  2 03:32:09 UTC 2020
  network time: Tue, 02 Jun 2020 03:32:09 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72904/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72904/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3523) (python)
##[section]Finishing: Finalize Job
