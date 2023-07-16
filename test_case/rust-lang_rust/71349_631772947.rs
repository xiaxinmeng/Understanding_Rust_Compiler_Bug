plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 6'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200512.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200512.2/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0336a291-567d-4115-8da0-88d7aa56f0ed.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71349/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71349/merge:refs/remotes/pull/71349/merge
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
..........................................................................i......................... 1800/10201
.................................................................................................... 1900/10201
.............................................................................................i..i... 2000/10201
.................................................................................................... 2100/10201
...................................................................................iiiii............ 2200/10201
.................................................................................................... 2400/10201
.................................................................................................... 2500/10201
.................................................................................................... 2600/10201
.................................................................................................... 2700/10201
---
...............i.................................................................................... 5200/10201
.................................................................................................... 5300/10201
..............................................i..................................................... 5400/10201
.......................................i............................................................ 5500/10201
................................................ii.ii........i...i.................................. 5600/10201
..................................................................................................i. 5800/10201
.................................................................................................... 5900/10201
..................................................ii.....................................i.......... 6000/10201
.................................................................................................... 6100/10201
.................................................................................................... 6100/10201
.................................................................................................... 6200/10201
.................................................................................................... 6300/10201
...........ii...i..ii...........i................................................................... 6400/10201
.................................................................................................... 6600/10201
.................................................................................................... 6700/10201
.................................................................................................... 6700/10201
............................................i..ii................................................... 6800/10201
.................................................................................................... 7000/10201
..................................................................................................i. 7100/10201
.................................................................................................... 7200/10201
.................................................................................................... 7300/10201
---
.................................................................................................... 8100/10201
.................................................................................................... 8200/10201
.................................................................................................... 8300/10201
.....................i.............................................................................. 8400/10201
...........................................................................iiiiii.iiiiii.i.......... 8500/10201
..............................i..................................................................... 8700/10201
.................................................................................................... 8800/10201
.................................................................................................... 8900/10201
.................................................................................................... 9000/10201
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 190 tests
iiii......i..............ii.i..........i...............................i..i..................i....i. 100/190
...........i.i.i...iii..iiiiiiiiiiiiiiii.......................i.ii...............ii......

 finished in 5.926
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----

The following items were assigned to wrong codegen units:

fn core::ptr[0]::drop_in_place[0]<cgu_extern_drop_glue::Struct[0]>
  expected: extern_drop_glue-fallback.cgu[External] 
  actual:   extern_drop_glue-core-.cgu[External] 

fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::LocalStruct[0]>
  expected: extern_drop_glue-fallback.cgu[External] 
  actual:   extern_drop_glue-core-.cgu[External] 

fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]>
  expected: extern_drop_glue-fallback.cgu[External] 
  actual:   extern_drop_glue-core-.cgu[External] 
thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2489:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----

The following items were assigned to wrong codegen units:

fn core::ptr[0]::drop_in_place[0]<(u32, local_drop_glue::Struct[0])>
  expected: local_drop_glue-fallback.cgu[Internal] 
  actual:   local_drop_glue-core-.cgu[Internal] 

fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Outer[0]>
  expected: local_drop_glue-fallback.cgu[External] 
  actual:   local_drop_glue-core-.cgu[External] 

fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Struct[0]>
  expected: local_drop_glue-fallback.cgu[External] 
  actual:   local_drop_glue-core-.cgu[External] 

fn core::ptr[0]::drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]>
  expected: local_drop_glue-fallback.cgu[External] 
  actual:   local_drop_glue-core-.cgu[External] 

fn local_drop_glue::{{impl}}[0]::drop[0]
  expected: local_drop_glue-fallback.cgu[External] 
  actual:   local_drop_glue[External] 
thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2489:13

---- [codegen-units] codegen-units/partitioning/shared-generics.rs stdout ----


The following items were assigned to wrong codegen units:

fn shared_generics_aux::generic_fn[0]<u16>
  expected: shared_generics_aux-in-shared_generics.volatile[External] 
  actual:   shared_generics-shared_generics_aux-.cgu[External] 
thread '[codegen-units] codegen-units/partitioning/shared-generics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2489:13


failures:
---
test result: FAILED. 35 passed; 3 failed; 3 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:01:46
== clock drift check ==
  local time: Wed May 20 22:51:36 UTC 2020
  network time: Wed, 20 May 2020 22:51:36 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71349/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71349/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3701) (python)
##[section]Finishing: Finalize Job
