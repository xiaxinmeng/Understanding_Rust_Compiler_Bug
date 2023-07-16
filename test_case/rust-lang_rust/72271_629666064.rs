plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 63'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e9bbeb9f-ae64-485f-8c32-9ace611c04fa.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72271/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72271/merge:refs/remotes/pull/72271/merge
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
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling chalk-rust-ir v0.10.0
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
.......................................................i............................................ 1800/10169
.................................................................................................... 1900/10169
..........................................................................i..i...................... 2000/10169
.................................................................................................... 2100/10169
................................................................iiiii............................... 2200/10169
.................................................................................................... 2400/10169
.................................................................................................... 2500/10169
.................................................................................................... 2600/10169
.................................................................................................... 2700/10169
---
.................................................................................................... 5200/10169
.................................................................................................... 5300/10169
...........................i........................................................................ 5400/10169
....................i............................................................................... 5500/10169
............................ii.ii........i...i...................................................... 5600/10169
..............................................................................i..................... 5800/10169
.................................................................................................... 5900/10169
.........................ii.....................................i................................... 6000/10169
.................................................................................................... 6100/10169
.................................................................................................... 6100/10169
.................................................................................................... 6200/10169
......................................................................................ii...i..ii.... 6300/10169
.................................................................................................... 6500/10169
.................................................................................................... 6600/10169
.................................................................................................... 6700/10169
.................................................................................................... 6700/10169
...................i..ii............................................................................ 6800/10169
.................................................................................................... 7000/10169
.........................................................................i.......................... 7100/10169
.................................................................................................... 7200/10169
.................................................................................................... 7300/10169
---
.................................................................................................... 8100/10169
.................................................................................................... 8200/10169
...........................................................................................i........ 8300/10169
.................................................................................................... 8400/10169
..............................................iiiiiiiiiii.i......................................... 8500/10169
..................................................................................................i. 8600/10169
.................................................................................................... 8800/10169
.................................................................................................... 8900/10169
.................................................................................................... 9000/10169
...................................................................................F................ 9100/10169
---

13    |                                   ^
14    |
15    = note: type arguments must be provided before constant arguments
+    = help: Wrong parameter order found. A possible order is: T, N
17 error: aborting due to previous error; 1 warning emitted
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-type-arg-misordered/const-arg-type-arg-misordered.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const-arg-type-arg-misordered.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-arg-type-arg-misordered.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-type-arg-misordered" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-type-arg-misordered/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0747]: constant provided when a type was expected
  --> /checkout/src/test/ui/const-generics/const-arg-type-arg-misordered.rs:6:35
   |
LL | fn foo<const N: usize>() -> Array<N, ()> { //~ ERROR constant provided when a type was expected
   |
   = note: type arguments must be provided before constant arguments
   = note: type arguments must be provided before constant arguments
   = help: Wrong parameter order found. A possible order is: T, N
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0747`.

---

125    |                                                        ^^
126    |
127    = note: type arguments must be provided before lifetime arguments
+    = help: Wrong parameter order found. A possible order is: 'a, 'b, 'c, Self, T, U, V
129 error[E0747]: lifetime provided when a type was expected
130   --> $DIR/suggest-move-types.rs:82:56

133    |                                                        ^^
133    |                                                        ^^
134    |
135    = note: type arguments must be provided before lifetime arguments
+    = help: Wrong parameter order found. A possible order is: 'a, 'b, 'c, Self, T, U, V
137 error: aborting due to 12 previous errors
138 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types/suggest-move-types.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-move-types.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-move-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generic arguments must come before the first constraint
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:26:26
   |
LL | struct A<T, M: One<A=(), T>> {
   |                    ----  ^ generic argument
   |                    constraint
   |
help: move the constraint after the generic argument
   |
   |
LL | struct A<T, M: One<T, A = ()>> {

error: generic arguments must come before the first constraint
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:33:43
   |
   |
LL | struct Al<'a, T, M: OneWithLifetime<A=(), T, 'a>> {
   |                                     ----  ^  ^^ generic arguments
   |                                     constraint
   |
help: move the constraint after the generic arguments
   |
   |
LL | struct Al<'a, T, M: OneWithLifetime<'a, T, A = ()>> {

error: generic arguments must come before the first constraint
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:40:46
   |
   |
LL | struct B<T, U, V, M: Three<A=(), B=(), C=(), T, U, V>> {
   |                            ----  ----  ----  ^  ^  ^ generic arguments
   |                            constraints
   |
help: move the constraints after the generic arguments
   |
   |
LL | struct B<T, U, V, M: Three<T, U, V, A = (), B = (), C = ()>> {

error: generic arguments must come before the first constraint
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:48:71
   |
   |
LL | struct Bl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<A=(), B=(), C=(), T, U, V, 'a, 'b, 'c>> {
   |                                                     ----  ----  ----  ^  ^  ^  ^^  ^^  ^^ generic arguments
   |                                                     constraints
   |
help: move the constraints after the generic arguments
   |
   |
LL | struct Bl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<'a, 'b, 'c, T, U, V, A = (), B = (), C = ()>> {

error: generic arguments must come before the first constraint
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:57:28
   |
   |
LL | struct C<T, U, V, M: Three<T, A=(), B=(), C=(), U, V>> {
   |                            ^  ----  ----  ----  ^  ^ generic arguments
   |                               constraints
   |
help: move the constraints after the generic arguments
   |
   |
LL | struct C<T, U, V, M: Three<T, U, V, A = (), B = (), C = ()>> {

error: generic arguments must come before the first constraint
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:65:53
   |
   |
LL | struct Cl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), C=(), U, 'b, V, 'c>> {
   |                                                     ^  ^^  ----  ----  ----  ^  ^^  ^  ^^ generic arguments
   |                                                            constraints
   |
help: move the constraints after the generic arguments
   |
   |
LL | struct Cl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<'a, 'b, 'c, T, U, V, A = (), B = (), C = ()>> {

error: generic arguments must come before the first constraint
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:74:28
   |
   |
LL | struct D<T, U, V, M: Three<T, A=(), B=(), U, C=(), V>> {
   |                            ^  ----  ----  ^  ----  ^ generic arguments
   |                               constraints
   |
help: move the constraints after the generic arguments
   |
   |
LL | struct D<T, U, V, M: Three<T, U, V, A = (), B = (), C = ()>> {

error: generic arguments must come before the first constraint
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:82:53
   |
   |
LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), U, 'b, C=(), V, 'c>> {
   |                                                     ^  ^^  ----  ----  ^  ^^  ----  ^  ^^ generic arguments
   |                                                            constraints
   |
help: move the constraints after the generic arguments
   |
   |
LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<'a, 'b, 'c, T, U, V, A = (), B = (), C = ()>> {

error[E0747]: type provided when a lifetime was expected
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:33:43
   |
   |
LL | struct Al<'a, T, M: OneWithLifetime<A=(), T, 'a>> {
   |
   = note: lifetime arguments must be provided before type arguments

error[E0747]: type provided when a lifetime was expected
error[E0747]: type provided when a lifetime was expected
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:48:71
   |
LL | struct Bl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<A=(), B=(), C=(), T, U, V, 'a, 'b, 'c>> {
   |
   = note: lifetime arguments must be provided before type arguments

error[E0747]: lifetime provided when a type was expected
error[E0747]: lifetime provided when a type was expected
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:65:56
   |
LL | struct Cl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), C=(), U, 'b, V, 'c>> {
   |
   = note: type arguments must be provided before lifetime arguments
   = note: type arguments must be provided before lifetime arguments
   = help: Wrong parameter order found. A possible order is: 'a, 'b, 'c, Self, T, U, V
error[E0747]: lifetime provided when a type was expected
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:82:56
   |
   |
LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), U, 'b, C=(), V, 'c>> {
   |
   = note: type arguments must be provided before lifetime arguments
   = note: type arguments must be provided before lifetime arguments
   = help: Wrong parameter order found. A possible order is: 'a, 'b, 'c, Self, T, U, V
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0747`.

---
test result: FAILED. 10106 passed; 2 failed; 61 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:00:06
== clock drift check ==
  local time: Sat May 16 15:48:39 UTC 2020
  network time: Sat, 16 May 2020 15:48:39 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72271/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72271/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3499) (python)
##[section]Finishing: Finalize Job
