plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 49'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200604.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1b30a742-b37b-4e9b-aa83-987e908d6f94.sh

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
 ---> 29a56a071ad9
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> eb826cd6a4d7
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 9841042138f8
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 00b49f7048de
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
.................................................................................................... 1900/10319
.................................................................................................... 2000/10319
...............i..i................................................................................. 2100/10319
.................................................................................................... 2200/10319
.....iiiii.......................................................................................... 2300/10319
.................................................................................................... 2500/10319
.................................................................................................... 2600/10319
.................................................................................................... 2700/10319
.................................................................................................... 2800/10319
---
.................................................................................................... 6000/10319
.......ii.....................................i..................................................... 6100/10319
.................................................................................................... 6200/10319
.................................................................................................... 6300/10319
......................................................................ii...i..ii...........i........ 6400/10319
.................................................................................................... 6600/10319
.................................................................................................... 6700/10319
.................................................................................................... 6800/10319
.................................................................................................... 6800/10319
....i..ii........................................................................................... 6900/10319
.................................................................................................... 7100/10319
...........................................................i........................................ 7200/10319
.................................................................................................... 7300/10319
.................................................................................................... 7400/10319
---
.................................................................................................... 8200/10319
.................................................................................................... 8300/10319
.................................................................................................... 8400/10319
.i.................................................................................................. 8500/10319
.......................................................iiiiii.iiiiii.i.............................. 8600/10319
............i....................................................................................... 8800/10319
.................................................................................................... 8900/10319
.................................................................................................... 9000/10319
.................................................................................................... 9100/10319
---

14    |                                   ^
15    |
16    = note: type arguments must be provided before constant arguments
-    = help: reorder the arguments: types, then consts: `<T, N>`
+    = help: reorder the arguments: type, then const: `<T, N>`
19 error: aborting due to previous error; 1 warning emitted
20 



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
stderr:
------------------------------------------
warning: the feature `const_generics` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/const-generics/const-arg-type-arg-misordered.rs:1:12
   |
LL | #![feature(const_generics)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44580 <***/issues/44580> for more information
error[E0747]: constant provided when a type was expected
  --> /checkout/src/test/ui/const-generics/const-arg-type-arg-misordered.rs:6:35
   |
   |
LL | fn foo<const N: usize>() -> Array<N, ()> { //~ ERROR constant provided when a type was expected
   |
   = note: type arguments must be provided before constant arguments
   = note: type arguments must be provided before constant arguments
   = help: reorder the arguments: type, then const: `<T, N>`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0747`.

---

125    |                                                        ^^
126    |
127    = note: lifetime arguments must be provided before type arguments
-    = help: reorder the arguments: lifetimes, then types: `<'a, 'b, 'c, T, U, V>`
+    = help: reorder the arguments: lifetime, then type: `<'a, 'b, 'c, T, U, V>`
130 error[E0747]: lifetime provided when a type was expected
131   --> $DIR/suggest-move-types.rs:82:56

134    |                                                        ^^
134    |                                                        ^^
135    |
136    = note: lifetime arguments must be provided before type arguments
-    = help: reorder the arguments: lifetimes, then types: `<'a, 'b, 'c, T, U, V>`
+    = help: reorder the arguments: lifetime, then type: `<'a, 'b, 'c, T, U, V>`
139 error: aborting due to 12 previous errors
140 



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
   = note: lifetime arguments must be provided before type arguments
   = note: lifetime arguments must be provided before type arguments
   = help: reorder the arguments: lifetime, then type: `<'a, 'b, 'c, T, U, V>`
error[E0747]: lifetime provided when a type was expected
  --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:82:56
   |
   |
LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), U, 'b, C=(), V, 'c>> {
   |
   = note: lifetime arguments must be provided before type arguments
   = note: lifetime arguments must be provided before type arguments
   = help: reorder the arguments: lifetime, then type: `<'a, 'b, 'c, T, U, V>`
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0747`.

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:59:20
Build completed unsuccessfully in 0:59:20
== clock drift check ==
  local time: Tue Jun 16 08:33:58 UTC 2020
  network time: Tue, 16 Jun 2020 08:33:58 GMT
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
Terminate orphan process: pid (4217) (python)
##[section]Finishing: Finalize Job
