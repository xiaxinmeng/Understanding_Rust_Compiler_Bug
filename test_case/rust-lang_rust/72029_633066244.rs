plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 5'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a4dab2a4-602c-4c5f-b529-bda366fbeb27.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72029/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72029/merge:refs/remotes/pull/72029/merge
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
...........................................................................i........................ 1800/10219
.................................................................................................... 1900/10219
..............................................................................................i..i.. 2000/10219
.................................................................................................... 2100/10219
....................................................................................iiiii........... 2200/10219
.................................................................................................... 2400/10219
.................................................................................................... 2500/10219
.................................................................................................... 2600/10219
.................................................................................................... 2700/10219
---
...........i...............i........................................................................ 5200/10219
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
..........................................................................................iiiiii.i.i 8500/10219
iiiii............................................................................................... 8600/10219
.................................................................................................... 8800/10219
.................................................................................................... 8900/10219
.................................................................................................... 9000/10219
.................................................................................................... 9100/10219
---
---- [run-pass-valgrind] run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs stdout ----

error: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
 --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:1:12
  |
1 | #![feature(unsized_locals)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #48055 <***/issues/48055> for more information
warning: trait objects without an explicit `dyn` are deprecated
 --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:5:21
  |
  |
5 | fn gen_foo() -> Box<fmt::Display> {
  |                     ^^^^^^^^^^^^ help: use `dyn`: `dyn fmt::Display`
  = note: `#[warn(bare_trait_objects)]` on by default

warning: trait objects without an explicit `dyn` are deprecated
 --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:9:11
 --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:9:11
  |
9 | fn foo(x: fmt::Display) {
  |           ^^^^^^^^^^^^ help: use `dyn`: `dyn fmt::Display`

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:13:20
   |
13 | fn foo_indirect(x: fmt::Display) {
   |                    ^^^^^^^^^^^^ help: use `dyn`: `dyn fmt::Display`
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:22:16
   |
   |
22 |         let x: fmt::Display = *gen_foo();
   |                ^^^^^^^^^^^^ help: use `dyn`: `dyn fmt::Display`
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:27:16
   |
   |
27 |         let x: fmt::Display = *gen_foo();
   |                ^^^^^^^^^^^^ help: use `dyn`: `dyn fmt::Display`
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:28:16
   |
   |
28 |         let y: fmt::Display = *gen_foo();
   |                ^^^^^^^^^^^^ help: use `dyn`: `dyn fmt::Display`
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:36:20
   |
   |
36 |             let x: fmt::Display = *gen_foo();
   |                    ^^^^^^^^^^^^ help: use `dyn`: `dyn fmt::Display`
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs:47:16
   |
   |
47 |         let x: fmt::Display = *gen_foo();
   |                ^^^^^^^^^^^^ help: use `dyn`: `dyn fmt::Display`

error[E0277]: the size for values of type `(dyn std::fmt::Display + 'static)` cannot be known at compilation time
  |
9 | fn foo(x: fmt::Display) {
  |        ^ doesn't have a size known at compile-time
  |
  |
  = help: the trait `std::marker::Sized` is not implemented for `(dyn std::fmt::Display + 'static)`
  = note: all function arguments must have a statically known size
  = help: unsized fn params are gated as an unstable feature


error[E0277]: the size for values of type `(dyn std::fmt::Display + 'static)` cannot be known at compilation time
   |
   |
13 | fn foo_indirect(x: fmt::Display) {
   |                 ^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn std::fmt::Display + 'static)`
   = note: all function arguments must have a statically known size
   = help: unsized fn params are gated as an unstable feature


error[E0277]: the size for values of type `(dyn std::fmt::Display + 'static)` cannot be known at compilation time
   |
14 |     foo(x);
   |     ^^^ doesn't have a size known at compile-time
   |
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn std::fmt::Display + 'static)`
   = note: all function arguments must have a statically known size
   = help: unsized fn params are gated as an unstable feature


error[E0277]: the size for values of type `(dyn std::fmt::Display + 'static)` cannot be known at compilation time
   |
   |
18 |     foo(*gen_foo());
   |
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn std::fmt::Display + 'static)`
   = note: all function arguments must have a statically known size
   = help: unsized fn params are gated as an unstable feature


error: aborting due to 4 previous errors; 9 warnings emitted
For more information about this error, try `rustc --explain E0277`.

------------------------------------------

---
test result: FAILED. 16 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-valgrind" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass-valgrind" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:58:02
Build completed unsuccessfully in 0:58:02
== clock drift check ==
  local time: Sat May 23 14:35:43 UTC 2020
  network time: Sat, 23 May 2020 14:35:43 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72029/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72029/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3814) (python)
##[section]Finishing: Finalize Job
