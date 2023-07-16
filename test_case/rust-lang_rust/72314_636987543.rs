plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 64'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e70766bd-f1c9-471a-9a49-bd58dd788408.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72314/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72314/merge:refs/remotes/pull/72314/merge
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
.................................................................................................... 1700/10278
...................................................................................i................ 1800/10278
.................................................................................................... 1900/10278
.................................................................................................... 2000/10278
....i..i............................................................................................ 2100/10278
..............................................................................................iiiii. 2200/10278
.................................................................................................... 2400/10278
.................................................................................................... 2500/10278
.................................................................................................... 2600/10278
.................................................................................................... 2700/10278
---
.........................i...............i.......................................................... 5200/10278
.................................................................................................... 5300/10278
.........................................................................i.......................... 5400/10278
...................................................................i................................ 5500/10278
...................................................................................ii.ii........i... 5600/10278
..........................i......................................................................... 5800/10278
..................................i................................................................. 5900/10278
........................................................................................ii.......... 6000/10278
...........................i........................................................................ 6100/10278
...........................i........................................................................ 6100/10278
.................................................................................................... 6200/10278
.................................................................................................... 6300/10278
..................................................ii...i..ii...........i............................ 6400/10278
.................................................................................................... 6600/10278
.................................................................................................... 6700/10278
.................................................................................................... 6700/10278
...................................................................................i..ii............ 6800/10278
.................................................................................................... 7000/10278
.................................................................................................... 7100/10278
.....................................i.............................................................. 7200/10278
.................................................................................................... 7300/10278
---
.................................................................................................... 8200/10278
.................................................................................................... 8300/10278
..........................................................................i......................... 8400/10278
.................................................................................................... 8500/10278
............................iiiiii.iiiiii.i......................................................... 8600/10278
.................................................................................................... 8800/10278
..........................................................................F......................... 8900/10278
.................................................................................................... 9000/10278
.................................................................................................... 9100/10278
---
25 error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:37:20
+   --> $DIR/generics-default-stability.rs:35:20
27    |
28 LL |     let _: Struct1<isize> = Struct1 { field: 1 };

31    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
32 
33 error[E0658]: use of unstable library feature 'unstable_default'
---
52 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/generics-default-stability/generics-default-stability.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args stability-attribute/generics-default-stability.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/generics-default-stability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/generics-default-stability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/generics-default-stability/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:16:13
   |
LL | impl Trait1<usize> for S { //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_default'
error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:20:13
   |
LL | impl Trait1<isize> for S { //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_default'
error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:24:13
   |
LL | impl Trait2<usize> for S { //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_default'
error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:35:20
   |
LL |     let _: Struct1<isize> = Struct1 { field: 1 }; //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_default'
error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:39:20
   |
LL |     let _: Struct1<usize> = STRUCT1; //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_default'
error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:40:20
   |
LL |     let _: Struct1<isize> = STRUCT1; //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:40:29
   |
LL |     let _: Struct1<isize> = STRUCT1; //~ ERROR use of unstable library feature 'unstable_default'
   |            --------------   ^^^^^^^ expected `isize`, found `usize`
   |            expected due to this
   |
   = note: expected struct `unstable_generic_param::Struct1<isize>`
              found struct `unstable_generic_param::Struct1<usize>`
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:01:43
Build completed unsuccessfully in 1:01:43
== clock drift check ==
  local time: Mon Jun  1 17:02:00 UTC 2020
  network time: Mon, 01 Jun 2020 17:02:01 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72314/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72314/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3811) (python)
##[section]Finishing: Finalize Job
