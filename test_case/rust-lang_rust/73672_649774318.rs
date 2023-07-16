plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
Agent machine name: 'fv-az578'
Current agent version: '2.171.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200621.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200621.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.171.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e99d61a8-4fe6-4791-9405-9e94ae0a0953.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73672/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73672/merge:refs/remotes/pull/73672/merge
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
   Compiling chalk-engine v0.11.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.11.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-ir v0.11.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-engine v0.11.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling chalk-solve v0.11.0
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
......................i............................................................................. 1900/10398
.................................................................................................... 2000/10398
.................................................i..i............................................... 2100/10398
.................................................................................................... 2200/10398
.......................................iiiii........................................................ 2300/10398
.................................................................................................... 2500/10398
.................................................................................................... 2600/10398
.................................................................................................... 2700/10398
.................................................................................................... 2800/10398
---
.................................................................................................... 5300/10398
.................................................................................................... 5400/10398
..............................i..................................................................... 5500/10398
........................i........................................................................... 5600/10398
............................................ii.ii........i...i...................................... 5700/10398
.............i...................................................................................... 5900/10398
..........i......................................................................................... 6000/10398
..................................................................ii................................ 6100/10398
.....i.............................................................................................. 6200/10398
.....i.............................................................................................. 6200/10398
.................................................................................................... 6300/10398
.................................................................................................... 6400/10398
.............................ii...i..ii...........i................................................. 6500/10398
.................................................................................................... 6700/10398
.................................................................................................... 6800/10398
.................................................................................................... 6800/10398
................................................................i..ii............................... 6900/10398
.................................................................................................... 7100/10398
.................................................................................................... 7200/10398
....................i............................................................................... 7300/10398
.................................................................................................... 7400/10398
---
.................................................................................................... 8300/10398
.................................................................................................... 8400/10398
.....................................................................i.............................. 8500/10398
.................................................................................................... 8600/10398
........................iiiiii.iiiiii.i............................................................. 8700/10398
.................................................................................................... 8900/10398
.................................................................................................... 9000/10398
.................................................................................................... 9100/10398
................................................................................................F... 9200/10398
---
9    |
+    = help: the trait `std::future::Future` is not implemented for `()`
10    = note: the return type of a function must have a statically known size
11 
12 error[E0698]: type inside `async fn` body must be known in this context

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/async-error-span.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/async-error-span.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/async-error-span.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-error-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `()` is not a future
  --> /checkout/src/test/ui/async-await/async-error-span.rs:7:20
   |
LL | fn get_future() -> impl Future<Output = ()> {
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^ `()` is not a future
LL | //~^ ERROR the trait bound `(): std::future::Future` is not satisfied
   |     -------- this returned value is of type `!`
   |
   = help: the trait `std::future::Future` is not implemented for `()`
   = note: the return type of a function must have a statically known size
   = note: the return type of a function must have a statically known size

error[E0698]: type inside `async fn` body must be known in this context
   |
   |
LL |     let a; //~ ERROR type inside `async fn` body must be known in this context
   |
   |
note: the type is part of the `async fn` body because of this `await`
   |
   |
LL |     get_future().await;

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0698.
---

---- [ui] ui/async-await/issue-70594.rs stdout ----
diff of stderr:

27 LL |     [1; ().await];
29 
- error[E0277]: the trait bound `(): std::future::Future` is not satisfied
+ error[E0277]: `()` is not a future
31   --> $DIR/issue-70594.rs:4:9
31   --> $DIR/issue-70594.rs:4:9
32    |
33 LL |     [1; ().await];
-    |         ^^^^^^^^ the trait `std::future::Future` is not implemented for `()`
+    |         ^^^^^^^^ `()` is not a future
35    |
+    = help: the trait `std::future::Future` is not implemented for `()`
+    = help: the trait `std::future::Future` is not implemented for `()`
36    = note: required by `std::future::Future::poll`
37 
38 error: aborting due to 5 previous errors


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70594/issue-70594.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-70594.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-70594.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70594" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70594/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0728]: `await` is only allowed inside `async` functions and blocks
  --> /checkout/src/test/ui/async-await/issue-70594.rs:4:9
   |
LL | async fn fun() {
   |          --- this is not `async`
LL |     [1; ().await];
   |         ^^^^^^^^ only allowed inside `async` functions and blocks
error[E0744]: `.await` is not allowed in a `const`
  --> /checkout/src/test/ui/async-await/issue-70594.rs:4:9
   |
   |
LL |     [1; ().await];

error[E0658]: `loop` is not allowed in a `const`
  --> /checkout/src/test/ui/async-await/issue-70594.rs:4:9
   |
   |
LL |     [1; ().await];
   |
   |
   = note: see issue #52000 <***/issues/52000> for more information

error[E0744]: `.await` is not allowed in a `const`
  --> /checkout/src/test/ui/async-await/issue-70594.rs:4:9
   |
   |
LL |     [1; ().await];

error[E0277]: `()` is not a future
  --> /checkout/src/test/ui/async-await/issue-70594.rs:4:9
   |
   |
LL |     [1; ().await];
   |         ^^^^^^^^ `()` is not a future
   = help: the trait `std::future::Future` is not implemented for `()`
   = note: required by `std::future::Future::poll`

error: aborting due to 5 previous errors
---

---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
diff of stderr:

27 LL |     (|_| 2333).await;
28    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
29 
- error[E0277]: the trait bound `[closure@$DIR/issue-62009-1.rs:12:5: 12:15]: std::future::Future` is not satisfied
+ error[E0277]: `[closure@$DIR/issue-62009-1.rs:12:5: 12:15]` is not a future
32    |
32    |
33 LL |     (|_| 2333).await;
-    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:12:5: 12:15]`
-    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:12:5: 12:15]`
+    |     ^^^^^^^^^^^^^^^^ `[closure@$DIR/issue-62009-1.rs:12:5: 12:15]` is not a future
+    = help: the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:12:5: 12:15]`
36    = note: required by `std::future::Future::poll`
37 
38 error: aborting due to 4 previous errors
38 error: aborting due to 4 previous errors


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0728]: `await` is only allowed inside `async` functions and blocks
  --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:6:5
   |
LL | fn main() {
   |    ---- this is not `async`
LL |     async { let (); }.await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
error[E0728]: `await` is only allowed inside `async` functions and blocks
  --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:8:5
   |
LL |   fn main() {
LL |   fn main() {
   |      ---- this is not `async`
...
LL | /     async {
LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
LL | |         let task1 = print_dur().await;
LL | |     }.await;
   | |___________^ only allowed inside `async` functions and blocks
error[E0728]: `await` is only allowed inside `async` functions and blocks
  --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:5
   |
LL | fn main() {
LL | fn main() {
   |    ---- this is not `async`
...
LL |     (|_| 2333).await;
   |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks

error[E0277]: `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:5: 12:15]` is not a future
   |
   |
LL |     (|_| 2333).await;
   |     ^^^^^^^^^^^^^^^^ `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:5: 12:15]` is not a future
   |
   = help: the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:5: 12:15]`
   = note: required by `std::future::Future::poll`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0728.
For more information about an error, try `rustc --explain E0277`.
---
- error[E0277]: the trait bound `u32: std::future::Future` is not satisfied
+ error[E0277]: `u32` is not a future
8   --> $DIR/issues-71798.rs:1:25
9    |
10 LL | fn test_ref(x: &u32) -> impl std::future::Future<Output = u32> + '_ {
-    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `u32`
+    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `u32` is not a future
12 LL |     *x
13    |     -- this returned value is of type `u32`
---
17 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues-71798/issues-71798.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues-71798.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues-71798.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues-71798" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues-71798/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `u` in this scope
  --> /checkout/src/test/ui/issues-71798.rs:6:24
   |
LL |     let _ = test_ref & u; //~ ERROR cannot find value `u` in this scope

error[E0277]: `u32` is not a future
  --> /checkout/src/test/ui/issues-71798.rs:1:25
   |
   |
LL | fn test_ref(x: &u32) -> impl std::future::Future<Output = u32> + '_ {
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `u32` is not a future
LL |     *x //~^ ERROR the trait bound `u32: std::future::Future` is not satisfied
   |
   = help: the trait `std::future::Future` is not implemented for `u32`
   = note: the return type of a function must have a statically known size

---
15 LL |     bar(foo());

16    |            ^^
17 
- error[E0277]: the trait bound `[closure@$DIR/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]: std::future::Future` is not satisfied
+ error[E0277]: `[closure@$DIR/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]` is not a future
20    |
21 LL | fn bar(f: impl Future<Output=()>) {}

24 LL |     let async_closure = async || ();
24 LL |     let async_closure = async || ();
25    |                         -------- consider calling this closure
26 LL |     bar(async_closure);
-    |         ^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]`
+    |         ^^^^^^^^^^^^^ `[closure@$DIR/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]` is not a future
+    = help: the trait `std::future::Future` is not implemented for `[closure@$DIR/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]`
29 help: use parentheses to call the closure
30    |
31 LL |     bar(async_closure());
31 LL |     bar(async_closure());


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |
LL |     bar(foo()); //~ERROR E0277
   |            ^^

error[E0277]: `[closure@/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]` is not a future
   |
LL | fn bar(f: impl Future<Output=()>) {}
   |                ----------------- required by this bound in `bar`
...
...
LL |     let async_closure = async || ();
   |                         -------- consider calling this closure
LL |     bar(async_closure); //~ERROR E0277
   |         ^^^^^^^^^^^^^ `[closure@/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]` is not a future
   |
   = help: the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]`
   |
LL |     bar(async_closure()); //~ERROR E0277
   |                      ^^

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:345:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:02:44
Build completed unsuccessfully in 1:02:44
== clock drift check ==
  local time: Thu Jun 25 19:29:51 UTC 2020
  network time: Thu, 25 Jun 2020 19:29:52 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73672/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73672/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4297) (python)
##[section]Finishing: Finalize Job
