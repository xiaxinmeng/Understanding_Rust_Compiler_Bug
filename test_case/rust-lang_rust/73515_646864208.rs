plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az578'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7552eb04-dc33-4761-a50d-0ae45fede5d1.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73515/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73515/merge:refs/remotes/pull/73515/merge
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
..................F..............................................................................F.. 1900/10329
................F....................................................................F.............. 2000/10329
..................i.i............................................................................... 2100/10329
.................................................................................................... 2200/10329
.......iiiii........................................................................................ 2300/10329
.................................................................................................... 2500/10329
.................................................................................................... 2600/10329
.................................................................................................... 2700/10329
.................................................................................................... 2800/10329
---
.................................................................................................... 5300/10329
.........................................................................................i.......... 5400/10329
...................................................................................i................ 5500/10329
.................................................................................................... 5600/10329
..ii.ii........i...i................................................................................ 5700/10329
........................................................i........................................... 5900/10329
.................................................................................................... 6000/10329
..........ii.....................................i.................................................. 6100/10329
.................................................................................................... 6200/10329
.................................................................................................... 6200/10329
.................................................................................................... 6300/10329
.........................................................................ii...i..ii...........i..... 6400/10329
.................................................................................................... 6600/10329
.................................................................................................... 6700/10329
.................................................................................................... 6800/10329
.................................................................................................... 6800/10329
.......i..ii........................................................................................ 6900/10329
.................................................................................................... 7100/10329
..............................................................i..................................... 7200/10329
.................................................................................................... 7300/10329
.................................................................................................... 7400/10329
---
.................................................................................................... 8200/10329
.................................................................................................... 8300/10329
.................................................................................................... 8400/10329
........i........................................................................................... 8500/10329
.............................................................iiiiii.iiiiii.i........................ 8600/10329
..................i.........................................F....................................... 8800/10329
.................................................................................................... 8900/10329
........................F........................................................................... 9000/10329
.................................................................................................... 9100/10329
---
diff of stderr:

5    |  ___________________________________________^
6 LL | |
7 LL | |                                                      field2: SafeEnum::Variant1}};
-    | |________________________________________________________________________________^ statics cannot evaluate destructors
+    | |                                                                                ^- value is dropped here
+    |                                                                                  statics cannot evaluate destructors
9 
10 error[E0010]: allocations are not allowed in statics
11   --> $DIR/check-static-values-constraints.rs:79:33
11   --> $DIR/check-static-values-constraints.rs:79:33


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/check-static-values-constraints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-static-values-constraints.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-static-values-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/check-static-values-constraints.rs:65:43
   |
LL |                                           ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),
   |  ___________________________________________^
LL | | //~^ ERROR destructors cannot be evaluated at compile-time
LL | |                                                      field2: SafeEnum::Variant1}};
   | |                                                                                ^- value is dropped here
   |                                                                                  statics cannot evaluate destructors

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
   |
LL | static STATIC11: Box<MyOwned> = box MyOwned;
   |                                 ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:37
   |
   |
LL | static STATIC11: Box<MyOwned> = box MyOwned;
   |
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/check-static-values-constraints.rs:90:32
   |
LL |     field2: SafeEnum::Variant4("str".to_string())

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:95:5
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |     ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:95:9
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0010]: allocations are not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:97:5
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |     ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:97:9
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0010]: allocations are not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:102:6
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |      ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:102:10
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0010]: allocations are not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:104:6
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |      ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:104:10
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0010]: allocations are not allowed in statics
---

error[E0507]: cannot move out of static item `x`
  --> /checkout/src/test/ui/check-static-values-constraints.rs:116:45
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                             |
   |                                             |
   |                                             move occurs because `x` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait
   |                                             help: consider borrowing here: `&x`
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:116:38
   |
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                      ^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:116:42
   |
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error: aborting due to 17 previous errors
---
diff of stderr:

2   --> $DIR/const_let.rs:16:32
3    |
4 LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
+    |                                ^^^^^                  - value is dropped here
+    |                                |
+    |                                constants cannot evaluate destructors
6 
6 
7 error[E0493]: destructors cannot be evaluated at compile-time
8   --> $DIR/const_let.rs:20:33

9    |
10 LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
+    |                                 ^^^^^                     - value is dropped here
+    |                                 |
+    |                                 constants cannot evaluate destructors
12 
12 
13 error[E0493]: destructors cannot be evaluated at compile-time
14   --> $DIR/const_let.rs:24:21

15    |
16 LL | const Z: () = { let mut x = None; x = Some(FakeNeedsDrop); };
+    |                     ^^^^^                                  - value is dropped here
+    |                     |
+    |                     constants cannot evaluate destructors
18 
18 
19 error[E0493]: destructors cannot be evaluated at compile-time
20   --> $DIR/const_let.rs:28:22

21    |
22 LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
+    |                      ^^^^^                                     - value is dropped here
+    |                      |
+    |                      constants cannot evaluate destructors
24 
24 
25 error: aborting due to 4 previous errors
26 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/const_let.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_let.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/const-eval/const_let.rs:16:32
   |
LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
   |                                |
   |                                constants cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/const-eval/const_let.rs:20:33
   |
LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
   |                                 |
   |                                 constants cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/const-eval/const_let.rs:24:21
   |
LL | const Z: () = { let mut x = None; x = Some(FakeNeedsDrop); };
   |                     |
   |                     constants cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/const-eval/const_let.rs:28:22
   |
LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
   |                      |
   |                      constants cannot evaluate destructors

error: aborting due to 4 previous errors
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-65394/issue-65394.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/issue-65394.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-65394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-65394" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-65394/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: references in constants may only refer to immutable values
  --> /checkout/src/test/ui/consts/const-eval/issue-65394.rs:8:13
   |
LL |     let r = &mut x; //~ ERROR references in constants may only refer to immutable values
   |             ^^^^^^ constants require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/const-eval/issue-65394.rs:7:9
   |
LL |     let mut x = Vec::<i32>::new(); //~ ERROR destructors cannot be evaluated at compile-time
---
7 error[E0493]: destructors cannot be evaluated at compile-time
8   --> $DIR/drop-fail.rs:23:9

9    |
10 LL |     let vec_tuple = (Vec::new(),);
+ ...
+ LL | };
+    | - value is dropped here
12 
---
26 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-fail.stock/drop-fail.stock.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/control-flow/drop-fail.rs`

error in revision `stock`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/drop-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-fail.stock" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-fail.stock/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/control-flow/drop-fail.rs:23:9
   |
LL |     let vec_tuple = (Vec::new(),);
...
LL | };
   | - value is dropped here

---
8   --> $DIR/min_const_fn.rs:39:36

17   --> $DIR/min_const_fn.rs:44:28
18    |
19 LL |     const fn into_inner_lt(self) -> T { self.0 }
+    |                            ^^^^                - value is dropped here
+    |                            |
+    |                            constant functions cannot evaluate destructors
21 
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
   |
LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
   |                         |
   |                         constant functions cannot evaluate destructors

error[E0723]: mutable references in const fn are unstable
error[E0723]: mutable references in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:36
   |
LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
   |                                    ^^^^^^
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:44:28
   |
   |
LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
   |                            |
   |                            constant functions cannot evaluate destructors

error[E0723]: mutable references in const fn are unstable
error[E0723]: mutable references in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:42
   |
LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:51:27
   |
   |
LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
   |                           |
   |                           constant functions cannot evaluate destructors

error[E0723]: mutable references in const fn are unstable
error[E0723]: mutable references in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:38
   |
LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: mutable references in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:39
   |
   |
LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:76:16
   |
LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
   |                ^
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:78:18
   |
   |
LL | const fn foo11_2<T: Send>(t: T) -> T { t }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: only int, `bool` and `char` operations are stable in const fn
   |
   |
LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: only int, `bool` and `char` operations are stable in const fn
   |
   |
LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: only int and `bool` operations are stable in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:84:35
   |
   |
LL | const fn foo19_3(f: f32) -> f32 { -f }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0723]: only int, `bool` and `char` operations are stable in const fn
   |
   |
LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: cannot access `static` items in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:90:27
   |
   |
LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: cannot access `static` items in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:91:37
   |
   |
LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: casting pointers to ints is unstable in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
   |
   |
LL | const fn foo30(x: *const u32) -> usize { x as usize }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: casting pointers to ints is unstable in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
   |
   |
LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: casting pointers to ints is unstable in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
   |
   |
LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: casting pointers to ints is unstable in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
   |
   |
LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:101:44
   |
   |
LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: loops and conditional expressions are not stable in const fn
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:103:44
   |
   |
LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: mutable references in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:105:14
   |
   |
LL | const fn inc(x: &mut i32) { *x += 1 }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:110:6
   |
LL | impl<T: std::fmt::Debug> Foo<T> {
LL | impl<T: std::fmt::Debug> Foo<T> {
   |      ^
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:115:6
   |
   |
LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:120:6
   |
   |
LL | impl<T: Sync + Sized> Foo<T> {
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:126:34
   |
   |
LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:128:22
   |
   |
LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:129:23
   |
   |
LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:130:32
   |
   |
LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:135:41
   |
   |
LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: function pointers in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:138:21
   |
   |
LL | const fn no_fn_ptrs(_x: fn()) {}
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: function pointers in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:140:27
   |
   |
LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error: aborting due to 32 previous errors

Some errors have detailed explanations: E0493, E0723.
For more information about an error, try `rustc --explain E0493`.
---
diff of stderr:

2   --> $DIR/feature-gate-unleash_the_miri_inside_of_you.rs:11:20
3    |
4 LL |     const F: u32 = (U::X, 42).1;
+    |                    ^^^^^^^^^^ - value is dropped here
+    |                    |
+    |                    constants cannot evaluate destructors
6 
6 
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you/feature-gate-unleash_the_miri_inside_of_you.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs:11:20
   |
LL |     const F: u32 = (U::X, 42).1; //~ ERROR destructors cannot be evaluated at compile-time
   |                    ^^^^^^^^^^ - value is dropped here
   |                    constants cannot evaluate destructors

error: aborting due to previous error

---
---- [ui] ui/consts/unstable-const-fn-in-libcore.rs stdout ----
diff of stderr:

9    |
10 LL |     const fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
11    |                                                     ^ constant functions cannot evaluate destructors
+ ...
+ LL |     }
12 
13 error[E0493]: destructors cannot be evaluated at compile-time
14   --> $DIR/unstable-const-fn-in-libcore.rs:19:47


15    |
16 LL |     const fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
+ ...
+ LL |     }
+    |     - value is dropped here
18 
18 
19 error: aborting due to 3 previous errors
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/unstable-const-fn-in-libcore/unstable-const-fn-in-libcore.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/unstable-const-fn-in-libcore.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/unstable-const-fn-in-libcore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/unstable-const-fn-in-libcore" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/unstable-const-fn-in-libcore/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/consts/unstable-const-fn-in-libcore.rs:24:26
   |
LL |             Opt::None => f(), //~ ERROR E0015

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/unstable-const-fn-in-libcore.rs:19:53
   |
   |
LL |     const fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
   |                                                     ^ constant functions cannot evaluate destructors
LL |     }
   |     - value is dropped here

error[E0493]: destructors cannot be evaluated at compile-time
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/unstable-const-fn-in-libcore.rs:19:47
   |
LL |     const fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
...
LL |     }
   |     - value is dropped here

---
diff of stderr:

2   --> $DIR/E0493.rs:17:17
3    |
4 LL | const F : Foo = (Foo { a : 0 }, Foo { a : 1 }).1;
+    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ - value is dropped here
+    |                 |
+    |                 constants cannot evaluate destructors
6 
6 
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493/E0493.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/E0493.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/E0493.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/span/E0493.rs:17:17
   |
LL | const F : Foo = (Foo { a : 0 }, Foo { a : 1 }).1;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ - value is dropped here
   |                 constants cannot evaluate destructors

error: aborting due to previous error

---
diff of stderr:

2   --> $DIR/static-drop-scope.rs:9:60
3    |
4 LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
-    |                                                            ^^^^^^^^ statics cannot evaluate destructors
+    |                                                            ^^^^^^^^- value is dropped here
+    |                                                            statics cannot evaluate destructors
6 
7 error[E0716]: temporary value dropped while borrowed
8   --> $DIR/static-drop-scope.rs:9:60
8   --> $DIR/static-drop-scope.rs:9:60

18   --> $DIR/static-drop-scope.rs:13:59
19    |
20 LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
+    |                                                           ^^^^^^^^- value is dropped here
+    |                                                           |
+    |                                                           constants cannot evaluate destructors
22 
22 
23 error[E0716]: temporary value dropped while borrowed
24   --> $DIR/static-drop-scope.rs:13:59

34   --> $DIR/static-drop-scope.rs:17:28
35    |
36 LL | static EARLY_DROP_S: i32 = (WithDtor, 0).1;
-    |                            ^^^^^^^^^^^^^ statics cannot evaluate destructors
+    |                            ^^^^^^^^^^^^^ - value is dropped here
+    |                            statics cannot evaluate destructors
38 
39 error[E0493]: destructors cannot be evaluated at compile-time
40   --> $DIR/static-drop-scope.rs:20:27
40   --> $DIR/static-drop-scope.rs:20:27

41    |
42 LL | const EARLY_DROP_C: i32 = (WithDtor, 0).1;
+    |                           ^^^^^^^^^^^^^ - value is dropped here
+    |                           |
+    |                           constants cannot evaluate destructors
44 
---
51 error[E0493]: destructors cannot be evaluated at compile-time
52   --> $DIR/static-drop-scope.rs:27:5

53    |
54 LL |     (x, ()).1
+ LL |
+ LL | }
+    | - value is dropped here
56 
56 
57 error[E0493]: destructors cannot be evaluated at compile-time
58   --> $DIR/static-drop-scope.rs:31:34

59    |
60 LL | const EARLY_DROP_C_OPTION: i32 = (Some(WithDtor), 0).1;
+    |                                  ^^^^^^^^^^^^^^^^^^^ - value is dropped here
+    |                                  |
+    |                                  constants cannot evaluate destructors
62 
62 
63 error[E0493]: destructors cannot be evaluated at compile-time
64   --> $DIR/static-drop-scope.rs:36:43

65    |
66 LL | const EARLY_DROP_C_OPTION_CONSTANT: i32 = (HELPER, 0).1;
+    |                                           ^^^^^^^^^^^ - value is dropped here
+    |                                           |
+    |                                           constants cannot evaluate destructors
68 
68 
69 error: aborting due to 10 previous errors
70 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/static-drop-scope.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-drop-scope.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-drop-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:9:60
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                            ^^^^^^^^- value is dropped here
   |                                                            statics cannot evaluate destructors

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-drop-scope.rs:9:60
  --> /checkout/src/test/ui/static/static-drop-scope.rs:9:60
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                      |     |       |
   |                                                      |     |       temporary value is freed at the end of this statement
   |                                                      |     creates a temporary which is freed while still in use
   |                                                      using this value as a static requires that borrow lasts for `'static`
   |                                                      using this value as a static requires that borrow lasts for `'static`

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:13:59
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                           ^^^^^^^^- value is dropped here
   |                                                           constants cannot evaluate destructors

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-drop-scope.rs:13:59
  --> /checkout/src/test/ui/static/static-drop-scope.rs:13:59
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                     |     |       |
   |                                                     |     |       temporary value is freed at the end of this statement
   |                                                     |     creates a temporary which is freed while still in use
   |                                                     using this value as a constant requires that borrow lasts for `'static`
   |                                                     using this value as a constant requires that borrow lasts for `'static`

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:17:28
   |
LL | static EARLY_DROP_S: i32 = (WithDtor, 0).1;
   |                            ^^^^^^^^^^^^^ - value is dropped here
   |                            statics cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:20:27
  --> /checkout/src/test/ui/static/static-drop-scope.rs:20:27
   |
LL | const EARLY_DROP_C: i32 = (WithDtor, 0).1;
   |                           ^^^^^^^^^^^^^ - value is dropped here
   |                           constants cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:23:24
---

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:27:5
   |
LL |     (x, ()).1
   |     ^^^^^^^ constant functions cannot evaluate destructors
LL |     //~^ ERROR destructors cannot be evaluated at compile-time
   | - value is dropped here

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:31:34
  --> /checkout/src/test/ui/static/static-drop-scope.rs:31:34
   |
LL | const EARLY_DROP_C_OPTION: i32 = (Some(WithDtor), 0).1;
   |                                  ^^^^^^^^^^^^^^^^^^^ - value is dropped here
   |                                  constants cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:36:43
  --> /checkout/src/test/ui/static/static-drop-scope.rs:36:43
   |
LL | const EARLY_DROP_C_OPTION_CONSTANT: i32 = (HELPER, 0).1;
   |                                           ^^^^^^^^^^^ - value is dropped here
   |                                           constants cannot evaluate destructors

error: aborting due to 10 previous errors

---
test result: FAILED. 10254 passed; 9 failed; 66 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:16
Build completed unsuccessfully in 1:05:16
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
== clock drift check ==
  local time: Fri Jun 19 21:18:59 UTC 2020
  network time: Fri, 19 Jun 2020 21:18:59 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73515/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73515/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3856) (python)
##[section]Finishing: Finalize Job
