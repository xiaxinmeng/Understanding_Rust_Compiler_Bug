plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 27'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5f915c9f-5ac4-4904-a878-23ce4a686083.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72788/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72788/merge:refs/remotes/pull/72788/merge
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
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
---
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
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
.............i...................................................................................... 1900/10365
.................................................................................................... 2000/10365
.......................................i..i......................................................... 2100/10365
.................................................................................................... 2200/10365
.............................iiiii.................................................................. 2300/10365
.................................................................................................... 2500/10365
.................................................................................................... 2600/10365
.................................................................................................... 2700/10365
.................................................................................................... 2800/10365
---
.................................................................................................... 5300/10365
.................................................................................................... 5400/10365
.................i.................................................................................. 5500/10365
...........i........................................................................................ 5600/10365
...............................ii.ii........i...i................................................... 5700/10365
...........................................................................i.......................i 5800/10365
.................................................................................................... 6000/10365
..............................................ii.....................................i.............. 6100/10365
.................................................................................................... 6200/10365
.................................................................................................... 6300/10365
.................................................................................................... 6300/10365
.................................................................................................... 6400/10365
.........ii...i..ii...........i..................................................................... 6500/10365
.................................................................................................... 6700/10365
.................................................................................................... 6800/10365
.................................................................................................... 6800/10365
...........................................i..ii.................................................... 6900/10365
.................................................................................................... 7100/10365
...................................................................................................i 7200/10365
.................................................................................................... 7300/10365
.................................................................................................... 7400/10365
---
.................................................................................................... 8200/10365
.................................................................................................... 8300/10365
.................................................................................................... 8400/10365
............................................i....................................................... 8500/10365
..................................................................................................ii 8600/10365
iiii..iiiiii.i...................................................................................... 8700/10365
......................................................................................FF.F.......... 8900/10365
.................................................................................................... 9000/10365
.................................................................................................... 9100/10365
.................................................................................................... 9200/10365
---
.....................................................................i.............................. 10300/10365
.................................................................
failures:

---- [ui] ui/specialization/deafult-associated-type-bound-1.rs stdout ----

+ warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
+ warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
+   --> $DIR/deafult-associated-type-bound-1.rs:4:12
+ LL | #![feature(specialization)]
+    |            ^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #31844 <***/issues/31844> for more information
1 error[E0277]: the trait bound `str: std::clone::Clone` is not satisfied
1 error[E0277]: the trait bound `str: std::clone::Clone` is not satisfied
2   --> $DIR/deafult-associated-type-bound-1.rs:17:5

7 LL |     default type U = str;
8    |     ^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `str`
9 
---
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-1/deafult-associated-type-bound-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/deafult-associated-type-bound-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/deafult-associated-type-bound-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/deafult-associated-type-bound-1.rs:4:12
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
error[E0277]: the trait bound `str: std::clone::Clone` is not satisfied
error[E0277]: the trait bound `str: std::clone::Clone` is not satisfied
  --> /checkout/src/test/ui/specialization/deafult-associated-type-bound-1.rs:17:5
   |
LL |     type U: Clone;
   |     -------------- required by `X::U`
LL |     default type U = str;
   |     ^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `str`

error: aborting due to previous error; 1 warning emitted
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/specialization/deafult-associated-type-bound-2.rs stdout ----

+ warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
+ warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
+   --> $DIR/deafult-associated-type-bound-2.rs:2:12
+ LL | #![feature(specialization)]
+    |            ^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #31844 <***/issues/31844> for more information
1 error[E0277]: can't compare `&'static B` with `B`
1 error[E0277]: can't compare `&'static B` with `B`
2   --> $DIR/deafult-associated-type-bound-2.rs:15:5

9    |
9    |
10    = help: the trait `std::cmp::PartialEq<B>` is not implemented for `&'static B`
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
13 
14 For more information about this error, try `rustc --explain E0277`.
14 For more information about this error, try `rustc --explain E0277`.
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-2/deafult-associated-type-bound-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/deafult-associated-type-bound-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/deafult-associated-type-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/deafult-associated-type-bound-2.rs:2:12
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
error[E0277]: can't compare `&'static B` with `B`
error[E0277]: can't compare `&'static B` with `B`
  --> /checkout/src/test/ui/specialization/deafult-associated-type-bound-2.rs:15:5
   |
LL |     type U: PartialEq<T>;
   |     --------------------- required by `X::U`
...
LL |     default type U = &'static B;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&'static B == B`
   |
   = help: the trait `std::cmp::PartialEq<B>` is not implemented for `&'static B`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.

---
+    |
+ LL | #![feature(specialization)]
+    |            ^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #31844 <***/issues/31844> for more information
1 warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
2   --> $DIR/deafult-generic-associated-type-bound.rs:4:12
3    |


4 LL | #![feature(generic_associated_types)]
5    |            ^^^^^^^^^^^^^^^^^^^^^^^^
6    |
-    = note: `#[warn(incomplete_features)]` on by default
8    = note: see issue #44265 <***/issues/44265> for more information
10 error[E0277]: can't compare `T` with `T`


23 LL | impl<T: 'static + std::cmp::PartialEq> X for T {
25 
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to previous error; 2 warnings emitted
27 
27 
28 For more information about this error, try `rustc --explain E0277`.
29 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-generic-associated-type-bound/deafult-generic-associated-type-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/deafult-generic-associated-type-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/deafult-generic-associated-type-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-generic-associated-type-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-generic-associated-type-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/deafult-generic-associated-type-bound.rs:3:12
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/deafult-generic-associated-type-bound.rs:4:12
   |
LL | #![feature(generic_associated_types)]
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #44265 <***/issues/44265> for more information
error[E0277]: can't compare `T` with `T`
  --> /checkout/src/test/ui/specialization/deafult-generic-associated-type-bound.rs:18:5
   |
   |
LL |     type U<'a>: PartialEq<&'a Self>;
   |     -------------------------------- required by `X::U`
...
LL |     default type U<'a> = &'a T;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `T == T`
   = help: the trait `std::cmp::PartialEq` is not implemented for `T`
   = note: required because of the requirements on the impl of `std::cmp::PartialEq` for `&'a T`
help: consider further restricting this bound
   |
   |
LL | impl<T: 'static + std::cmp::PartialEq> X for T {

error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------



failures:
    [ui] ui/specialization/deafult-associated-type-bound-1.rs
    [ui] ui/specialization/deafult-associated-type-bound-2.rs
    [ui] ui/specialization/deafult-generic-associated-type-bound.rs
test result: FAILED. 10296 passed; 3 failed; 66 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:06:21
Build completed unsuccessfully in 1:06:21
== clock drift check ==
  local time: Sat Jun 20 12:46:06 UTC 2020
  network time: Sat, 20 Jun 2020 12:46:06 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72788/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72788/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4059) (python)
##[section]Finishing: Finalize Job
