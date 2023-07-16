plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 32'
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
Downloading task: Bash (3.163.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c37d2016-8f33-473e-93ad-955e4571ce11.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71973/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71973/merge:refs/remotes/pull/71973/merge
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
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.........................................................i.......................................... 1800/10165
.................................................................................................... 1900/10165
...........................................................................i..i..................... 2000/10165
.................................................................................................... 2100/10165
.................................................................iiiii.............................. 2200/10165
.................................................................................................... 2400/10165
.................................................................................................... 2500/10165
.................................................................................................... 2600/10165
.................................................................................................... 2700/10165
---
.................................................................................................... 5200/10165
.................................................................................................... 5300/10165
............................i....................................................................... 5400/10165
.....................i.............................................................................. 5500/10165
............................ii.ii........i...i...................................................... 5600/10165
.............................................................................i...................... 5800/10165
.................................................................................................... 5900/10165
........................ii.....................................i.................................... 6000/10165
.................................................................................................... 6100/10165
.................................................................................................... 6100/10165
.................................................................................................... 6200/10165
.....................................................................................ii...i..ii..... 6300/10165
.................................................................................................... 6500/10165
.................................................................................................... 6600/10165
.................................................................................................... 6700/10165
.................................................................................................... 6700/10165
..................i..ii............................................................................. 6800/10165
.................................................................................................... 7000/10165
........................................................................i........................... 7100/10165
.................................................................................................... 7200/10165
.................................................................................................... 7300/10165
---
.................................................................................................... 8100/10165
.................................................................................................... 8200/10165
.......................................................................................i............ 8300/10165
.................................................................................................... 8400/10165
.........................................iiiiii.iiiii.i............................................. 8500/10165
.................................................................................................... 8700/10165
.................................................................................................... 8800/10165
.................................................................................................... 8900/10165
.................................................................................................... 9000/10165
---

---- [ui] ui/type-alias-impl-trait/generic_nondefining_use.rs stdout ----
diff of stderr:

25 LL | fn concrete_const() -> OneConst<{123}> {
27    |
- note: used non-generic constant `123usize` for generic parameter
+ note: used non-generic constant `{123}` for generic parameter
29   --> $DIR/generic_nondefining_use.rs:10:21
29   --> $DIR/generic_nondefining_use.rs:10:21
30    |
31 LL | type OneConst<const X: usize> = impl Debug;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use/generic_nondefining_use.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use/generic_nondefining_use.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_nondefining_use.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:14:21
   |
LL | fn concrete_ty() -> OneTy<u32> {
   |
note: used non-generic type `u32` for generic parameter
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:8:12
   |
   |
LL | type OneTy<T> = impl Debug;
   |            ^

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:19:27
   |
LL | type OneLifetime<'a> = impl Debug;
   |                  -- cannot use static lifetime; use a bound lifetime instead or remove the lifetime parameter from the opaque type
...
LL | fn concrete_lifetime() -> OneLifetime<'static> {

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:24:24
   |
   |
LL | fn concrete_const() -> OneConst<{123}> {
   |
note: used non-generic constant `{123}` for generic parameter
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:10:21
   |
   |
LL | type OneConst<const X: usize> = impl Debug;

error: aborting due to 3 previous errors


---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:20
Build completed unsuccessfully in 1:05:20
== clock drift check ==
  local time: Tue May 12 10:19:05 UTC 2020
  network time: Tue, 12 May 2020 10:19:05 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71973/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71973/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3772) (python)
##[section]Finishing: Finalize Job
