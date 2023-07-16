plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 69'
Agent machine name: 'fv-az619'
Current agent version: '2.169.0'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ecd69235-dcbd-4660-8dcb-58eef9e56dc7.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72449/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72449/merge:refs/remotes/pull/72449/merge
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
............................................................................i....................... 1800/10219
.................................................................................................... 1900/10219
...............................................................................................i..i. 2000/10219
.................................................................................................... 2100/10219
.....................................................................................iiiii.......... 2200/10219
.................................................................................................... 2400/10219
.................................................................................................... 2500/10219
.................................................................................................... 2600/10219
.................................................................................................... 2700/10219
---
...........i................i....................................................................... 5200/10219
.................................................................................................... 5300/10219
..........................................................i......................................... 5400/10219
...................................................i................................................ 5500/10219
.............................................................ii.ii........i...i..................... 5600/10219
...i................................................................................................ 5800/10219
...........i........................................................................................ 5900/10219
...............................................................ii................................... 6000/10219
..i................................................................................................. 6100/10219
..i................................................................................................. 6100/10219
.................................................................................................... 6200/10219
.................................................................................................... 6300/10219
........................ii...i..ii...........i...................................................... 6400/10219
.................................................................................................... 6600/10219
.................................................................................................... 6700/10219
.................................................................................................... 6700/10219
.........................................................i..ii...................................... 6800/10219
.................................................................................................... 7000/10219
.................................................................................................... 7100/10219
...........i........................................................................................ 7200/10219
.................................................................................................... 7300/10219
---
.................................................................................................... 8100/10219
.................................................................................................... 8200/10219
.................................................................................................... 8300/10219
....................................i............................................................... 8400/10219
..........................................................................................iiiiii.iii 8500/10219
iii.i............................................................................................... 8600/10219
.................................................................................................... 8800/10219
.................................................................................................... 8900/10219
.................................................................................................... 9000/10219
.................................................................................................... 9100/10219
---
---- [ui] ui/consts/const-float-bits-conv.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-float-bits-conv.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-conv/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-conv/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `core::f32::<impl f32>::is_nan` is not yet stable as a const fn
  --> /checkout/src/test/ui/consts/const-float-bits-conv.rs:47:19
   |
LL |     const_assert!(f32::from_bits(MASKED_NAN1).is_nan());
   |
   = help: add `#![feature(const_float_classify)]` to the crate attributes to enable


error: `core::f32::<impl f32>::is_nan` is not yet stable as a const fn
  --> /checkout/src/test/ui/consts/const-float-bits-conv.rs:48:19
   |
LL |     const_assert!(f32::from_bits(MASKED_NAN1).is_nan());
   |
   = help: add `#![feature(const_float_classify)]` to the crate attributes to enable


error: `core::f64::<impl f64>::is_nan` is not yet stable as a const fn
  --> /checkout/src/test/ui/consts/const-float-bits-conv.rs:75:19
   |
LL |     const_assert!(f64::from_bits(MASKED_NAN1).is_nan());
   |
   = help: add `#![feature(const_float_classify)]` to the crate attributes to enable


error: `core::f64::<impl f64>::is_nan` is not yet stable as a const fn
  --> /checkout/src/test/ui/consts/const-float-bits-conv.rs:76:19
   |
LL |     const_assert!(f64::from_bits(MASKED_NAN1).is_nan());
   |
   = help: add `#![feature(const_float_classify)]` to the crate attributes to enable

error: aborting due to 4 previous errors
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:00:57
Build completed unsuccessfully in 1:00:57
== clock drift check ==
  local time: Sat May 23 19:28:39 UTC 2020
  network time: Sat, 23 May 2020 19:28:39 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72449/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72449/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3672) (python)
##[section]Finishing: Finalize Job
