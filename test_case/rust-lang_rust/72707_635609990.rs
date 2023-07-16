plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 56'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5e00d692-1365-4b98-b0be-7b22d04a6efc.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72707/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72707/merge:refs/remotes/pull/72707/merge
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
.................................................................................................... 1600/10249
.................................................................................................... 1700/10249
..............................................................................i..................... 1800/10249
.................................................................................................... 1900/10249
.................................................................................................i.. 2000/10249
i.......FF.......................................................................................... 2100/10249
.......................................................................................iiiii........ 2200/10249
.................................................................................................... 2400/10249
.................................................................................................... 2500/10249
.................................................................................................... 2600/10249
.................................................................................................... 2700/10249
---
.................................................................................................... 3200/10249
....................................i............................................................... 3300/10249
...................................................F................................................ 3400/10249
.................................................................................................... 3500/10249
................................................ii...................FF............................. 3600/10249
.................................................................................................... 3800/10249
............................................i....................................................... 3900/10249
.................................................................................................... 4000/10249
.................................................................................................... 4100/10249
---
...............i...............i.................................................................... 5200/10249
.................................................................................................... 5300/10249
...............................................................i.................................... 5400/10249
........................................................i........................................... 5500/10249
....................................................................ii.ii........i...i.............. 5600/10249
...........i........................................................................................ 5800/10249
...................i................................................................................ 5900/10249
........................................................................ii.......................... 6000/10249
...........i........................................................................................ 6100/10249
...........i........................................................................................ 6100/10249
.................................................................................................... 6200/10249
.................................................................................................... 6300/10249
.................................ii...i..ii...........i............................................. 6400/10249
.................................................................................................... 6600/10249
.................................................................................................... 6700/10249
.................................................................................................... 6700/10249
..................................................................i..ii............................. 6800/10249
.................................................................................................... 7000/10249
.................................................................................................... 7100/10249
....................i............................................................................... 7200/10249
.................................................................................................... 7300/10249
---
...............................................................F...................................F 8200/10249
.................................................................................................... 8300/10249
.......................................................i............................................ 8400/10249
.................................................................................................... 8500/10249
.........iiiiii.iiiiii.i............................................................................ 8600/10249
.................................................................................................... 8800/10249
.................................................................................................... 8900/10249
.................................................................................................... 9000/10249
.................................................................................................... 9100/10249
---
---- [ui] ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs stdout ----
diff of stderr:

30    |
31 LL |     const BAR: u32 = IMPL_REF_BAR;
32    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
+ note: ...which requires optimizing MIR for `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
35    |
35    |
36 LL |     const BAR: u32 = IMPL_REF_BAR;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl/issue-24949-assoc-const-static-recursion-impl.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl/issue-24949-assoc-const-static-recursion-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-24949-assoc-const-static-recursion-impl.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when const-evaluating + checking `IMPL_REF_BAR`
   |
   |
LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR; //~ ERROR E0391
   |
   |
note: ...which requires const-evaluating + checking `IMPL_REF_BAR`...
   |
   |
LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating `IMPL_REF_BAR`...
   |
   |
LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
note: ...which requires const-evaluating + checking `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = IMPL_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = IMPL_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = IMPL_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires optimizing MIR for `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = IMPL_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `IMPL_REF_BAR`...
   = note: ...which again requires const-evaluating + checking `IMPL_REF_BAR`, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.


------------------------------------------


---- [ui] ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default.rs stdout ----
diff of stderr:

30    |
31 LL |     const BAR: u32 = DEFAULT_REF_BAR;
32    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `FooDefault::BAR`...
+ note: ...which requires optimizing MIR for `FooDefault::BAR`...
35    |
35    |
36 LL |     const BAR: u32 = DEFAULT_REF_BAR;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default/issue-24949-assoc-const-static-recursion-trait-default.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default/issue-24949-assoc-const-static-recursion-trait-default.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-24949-assoc-const-static-recursion-trait-default.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when const-evaluating + checking `DEFAULT_REF_BAR`
   |
   |
LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR; //~ ERROR E0391
   |
   |
note: ...which requires const-evaluating + checking `DEFAULT_REF_BAR`...
   |
   |
LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating `DEFAULT_REF_BAR`...
   |
   |
LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `<GlobalDefaultRef as FooDefault>::BAR`...
note: ...which requires const-evaluating + checking `FooDefault::BAR`...
   |
   |
LL |     const BAR: u32 = DEFAULT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `FooDefault::BAR`...
   |
   |
LL |     const BAR: u32 = DEFAULT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating `FooDefault::BAR`...
   |
   |
LL |     const BAR: u32 = DEFAULT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires optimizing MIR for `FooDefault::BAR`...
   |
   |
LL |     const BAR: u32 = DEFAULT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `DEFAULT_REF_BAR`...
   = note: ...which again requires const-evaluating + checking `DEFAULT_REF_BAR`, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.


------------------------------------------


---- [ui] ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs stdout ----
diff of stderr:

30    |
31 LL |     const BAR: u32 = TRAIT_REF_BAR;
32    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `<impl at $DIR/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
+ note: ...which requires optimizing MIR for `<impl at $DIR/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
35    |
35    |
36 LL |     const BAR: u32 = TRAIT_REF_BAR;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait/issue-24949-assoc-const-static-recursion-trait.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait/issue-24949-assoc-const-static-recursion-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-24949-assoc-const-static-recursion-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when const-evaluating + checking `TRAIT_REF_BAR`
   |
   |
LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR; //~ ERROR E0391
   |
   |
note: ...which requires const-evaluating + checking `TRAIT_REF_BAR`...
   |
   |
LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating `TRAIT_REF_BAR`...
   |
   |
LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `<GlobalTraitRef as Foo>::BAR`...
note: ...which requires const-evaluating + checking `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = TRAIT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = TRAIT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = TRAIT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires optimizing MIR for `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = TRAIT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `TRAIT_REF_BAR`...
   = note: ...which again requires const-evaluating + checking `TRAIT_REF_BAR`, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.

---
diff of stderr:

5    |                   ^^^^^^^
6    |
7    = note: ...which again requires computing the bounds for type parameter `T`, completing the cycle
- note: cycle used when processing `A`
+ note: cycle used when getting explicit predicates for `A`
9   --> $DIR/cycle-projection-based-on-where-clause.rs:17:19
10    |
11 LL |           T : Add<T::Item>

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-projection-based-on-where-clause/cycle-projection-based-on-where-clause.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-projection-based-on-where-clause/cycle-projection-based-on-where-clause.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cycle-projection-based-on-where-clause.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-projection-based-on-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-projection-based-on-where-clause" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-projection-based-on-where-clause/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing the bounds for type parameter `T`
  --> /checkout/src/test/ui/cycle-projection-based-on-where-clause.rs:17:19
   |
LL |           T : Add<T::Item>
   |
   |
   = note: ...which again requires computing the bounds for type parameter `T`, completing the cycle
note: cycle used when getting explicit predicates for `A`
   |
   |
LL |           T : Add<T::Item>

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---
- error[E0391]: cycle detected when processing `Foo::X`
+ error[E0391]: cycle detected when getting type of `Foo::X`
2   --> $DIR/cycle-trait-default-type-trait.rs:4:23
3    |
4 LL | trait Foo<X = Box<dyn Foo>> {
5    |                       ^^^
6    |
6    |
-    = note: ...which again requires processing `Foo::X`, completing the cycle
+    = note: ...which again requires getting type of `Foo::X`, completing the cycle
8 note: cycle used when collecting item types in top-level module
9   --> $DIR/cycle-trait-default-type-trait.rs:4:1


11 LL | trait Foo<X = Box<dyn Foo>> {
13 
- error[E0391]: cycle detected when processing `Foo::X`
+ error[E0391]: cycle detected when getting type of `Foo::X`
15   --> $DIR/cycle-trait-default-type-trait.rs:4:23
15   --> $DIR/cycle-trait-default-type-trait.rs:4:23
16    |
17 LL | trait Foo<X = Box<dyn Foo>> {
18    |                       ^^^
19    |
19    |
-    = note: ...which again requires processing `Foo::X`, completing the cycle
+    = note: ...which again requires getting type of `Foo::X`, completing the cycle
21 note: cycle used when collecting item types in top-level module
22   --> $DIR/cycle-trait-default-type-trait.rs:4:1


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/cycle-trait-default-type-trait.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/cycle-trait-default-type-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cycle-trait/cycle-trait-default-type-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when getting type of `Foo::X`
  --> /checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs:4:23
   |
LL | trait Foo<X = Box<dyn Foo>> {
   |
   |
   = note: ...which again requires getting type of `Foo::X`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {

error[E0391]: cycle detected when getting type of `Foo::X`
  --> /checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs:4:23
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {
   |
   |
   = note: ...which again requires getting type of `Foo::X`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.

------------------------------------------


---- [ui] ui/impl-trait/auto-trait-leak.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
+ error[E0391]: cycle detected when getting type of `cycle1::{{opaque}}#0`
3    |
4 LL | fn cycle1() -> impl Clone {

14    |
14    |
15 LL | fn cycle1() -> impl Clone {
16    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `cycle1`...
+ note: ...which requires processing MIR for `cycle1`...
19    |
20 LL | fn cycle1() -> impl Clone {

24    |
24    |
25 LL | fn cycle1() -> impl Clone {
26    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires building MIR for...
+ note: ...which requires building MIR for `cycle1`...
29    |
30 LL | fn cycle1() -> impl Clone {

35 LL | fn cycle1() -> impl Clone {
35 LL | fn cycle1() -> impl Clone {
36    | ^^^^^^^^^^^^^^^^^^^^^^^^^
37    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
- note: ...which requires processing `cycle2::{{opaque}}#0`...
+ note: ...which requires getting type of `cycle2::{{opaque}}#0`...
40    |
41 LL | fn cycle2() -> impl Clone {

50    |
50    |
51 LL | fn cycle2() -> impl Clone {
52    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `cycle2`...
+ note: ...which requires processing MIR for `cycle2`...
55    |
56 LL | fn cycle2() -> impl Clone {

60    |
60    |
61 LL | fn cycle2() -> impl Clone {
62    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires building MIR for...
+ note: ...which requires building MIR for `cycle2`...
65    |
66 LL | fn cycle2() -> impl Clone {

71 LL | fn cycle2() -> impl Clone {
71 LL | fn cycle2() -> impl Clone {
72    | ^^^^^^^^^^^^^^^^^^^^^^^^^
73    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
-    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
+    = note: ...which again requires getting type of `cycle1::{{opaque}}#0`, completing the cycle
75 note: cycle used when checking item types in top-level module
77    |

84 LL | | }
85    | |_^
85    | |_^
86 
- error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
+ error[E0391]: cycle detected when getting type of `cycle1::{{opaque}}#0`
89    |
90 LL | fn cycle1() -> impl Clone {

100    |
100    |
101 LL | fn cycle1() -> impl Clone {
102    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `cycle1`...
+ note: ...which requires processing MIR for `cycle1`...
105    |
106 LL | fn cycle1() -> impl Clone {

110    |
110    |
111 LL | fn cycle1() -> impl Clone {
112    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires building MIR for...
+ note: ...which requires building MIR for `cycle1`...
115    |
116 LL | fn cycle1() -> impl Clone {

121 LL | fn cycle1() -> impl Clone {
121 LL | fn cycle1() -> impl Clone {
122    | ^^^^^^^^^^^^^^^^^^^^^^^^^
123    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
- note: ...which requires processing `cycle2::{{opaque}}#0`...
+ note: ...which requires getting type of `cycle2::{{opaque}}#0`...
126    |
127 LL | fn cycle2() -> impl Clone {

136    |
136    |
137 LL | fn cycle2() -> impl Clone {
138    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `cycle2`...
+ note: ...which requires processing MIR for `cycle2`...
141    |
142 LL | fn cycle2() -> impl Clone {

146    |
146    |
147 LL | fn cycle2() -> impl Clone {
148    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires building MIR for...
+ note: ...which requires building MIR for `cycle2`...
151    |
152 LL | fn cycle2() -> impl Clone {

156    |
156    |
157 LL | fn cycle2() -> impl Clone {
158    | ^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
+    = note: ...which again requires getting type of `cycle1::{{opaque}}#0`, completing the cycle
160 note: cycle used when checking item types in top-level module
162    |

169 LL | | }
170    | |_^
170    | |_^
171 
- error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
+ error[E0391]: cycle detected when getting type of `cycle1::{{opaque}}#0`
174    |
175 LL | fn cycle1() -> impl Clone {

185    |
185    |
186 LL | fn cycle1() -> impl Clone {
187    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `cycle1`...
+ note: ...which requires processing MIR for `cycle1`...
190    |
191 LL | fn cycle1() -> impl Clone {

195    |
195    |
196 LL | fn cycle1() -> impl Clone {
197    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires building MIR for...
+ note: ...which requires building MIR for `cycle1`...
200    |
201 LL | fn cycle1() -> impl Clone {

206 LL | fn cycle1() -> impl Clone {
206 LL | fn cycle1() -> impl Clone {
207    | ^^^^^^^^^^^^^^^^^^^^^^^^^
208    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
- note: ...which requires processing `cycle2::{{opaque}}#0`...
+ note: ...which requires getting type of `cycle2::{{opaque}}#0`...
211    |
212 LL | fn cycle2() -> impl Clone {

221    |
221    |
222 LL | fn cycle2() -> impl Clone {
223    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `cycle2`...
+ note: ...which requires processing MIR for `cycle2`...
226    |
227 LL | fn cycle2() -> impl Clone {

231    |
231    |
232 LL | fn cycle2() -> impl Clone {
233    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires building MIR for...
+ note: ...which requires building MIR for `cycle2`...
236    |
237 LL | fn cycle2() -> impl Clone {

241    |
241    |
242 LL | fn cycle2() -> impl Clone {
243    | ^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
+    = note: ...which again requires getting type of `cycle1::{{opaque}}#0`, completing the cycle
245 note: cycle used when checking item types in top-level module
247    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when getting type of `cycle1::{{opaque}}#0`
   |
LL | fn cycle1() -> impl Clone {
   |                ^^^^^^^^^^
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
note: ...which requires getting type of `cycle2::{{opaque}}#0`...
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
   = note: ...which again requires getting type of `cycle1::{{opaque}}#0`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | |
LL | | fn send<T: Send>(_: T) {}
LL | |     Rc::new(String::from("foo"))
LL | | }
   | |_^


error[E0391]: cycle detected when getting type of `cycle1::{{opaque}}#0`
   |
LL | fn cycle1() -> impl Clone {
   |                ^^^^^^^^^^
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
note: ...which requires getting type of `cycle2::{{opaque}}#0`...
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires getting type of `cycle1::{{opaque}}#0`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | |
LL | | fn send<T: Send>(_: T) {}
LL | |     Rc::new(String::from("foo"))
LL | | }
   | |_^


error[E0391]: cycle detected when getting type of `cycle1::{{opaque}}#0`
   |
LL | fn cycle1() -> impl Clone {
   |                ^^^^^^^^^^
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
note: ...which requires getting type of `cycle2::{{opaque}}#0`...
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
---
---- [ui] ui/infinite/infinite-tag-type-recursion.rs stdout ----
diff of stderr:

8    |
9    = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `MList` representable
10 
- error[E0391]: cycle detected when processing `MList`
+ error[E0391]: cycle detected when getting drop check constraints for `MList`
13    |
13    |
14 LL | enum MList { Cons(isize, MList), Nil }
15    | ^^^^^^^^^^
16    |
16    |
-    = note: ...which again requires processing `MList`, completing the cycle
+    = note: ...which again requires getting drop check constraints for `MList`, completing the cycle
18    = note: cycle used when computing dropck types for `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }, value: MList } }`
20 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-tag-type-recursion/infinite-tag-type-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-tag-type-recursion.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-tag-type-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-tag-type-recursion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-tag-type-recursion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0072]: recursive type `MList` has infinite size
  --> /checkout/src/test/ui/infinite/infinite-tag-type-recursion.rs:1:1
   |
LL | enum MList { Cons(isize, MList), Nil }
   | ^^^^^^^^^^               ----- recursive without indirection
   | recursive type has infinite size
   |
   |
   = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `MList` representable

error[E0391]: cycle detected when getting drop check constraints for `MList`
   |
   |
LL | enum MList { Cons(isize, MList), Nil }
   |
   |
   = note: ...which again requires getting drop check constraints for `MList`, completing the cycle
   = note: cycle used when computing dropck types for `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }, value: MList } }`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0072, E0391.
For more information about an error, try `rustc --explain E0072`.
---
- error[E0391]: cycle detected when processing `X`
+ error[E0391]: cycle detected when getting type of `X`
2   --> $DIR/infinite-vec-type-recursion.rs:1:14
3    |
4 LL | type X = Vec<X>;
5    |              ^
6    |
6    |
-    = note: ...which again requires processing `X`, completing the cycle
+    = note: ...which again requires getting type of `X`, completing the cycle
8 note: cycle used when collecting item types in top-level module
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion/infinite-vec-type-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-vec-type-recursion.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-vec-type-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when getting type of `X`
  --> /checkout/src/test/ui/infinite/infinite-vec-type-recursion.rs:1:14
   |
LL | type X = Vec<X>;
   |
   |
   = note: ...which again requires getting type of `X`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
   |
LL | / type X = Vec<X>;
LL | | //~^ ERROR cycle detected
LL | |
LL | | fn main() { let b: X = Vec::new(); }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---
diff of stderr:

5    |                     ^^^^
6    |
7    = note: ...which again requires computing the bounds for type parameter `T`, completing the cycle
- note: cycle used when processing `foo`
+ note: cycle used when getting explicit predicates for `foo`
10    |
10    |
11 LL | fn foo<T: Trait<A = T::B>>() { }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/issue-21177.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/issue-21177.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-21177.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21177.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing the bounds for type parameter `T`
  --> /checkout/src/test/ui/issues/issue-21177.rs:6:21
   |
LL | fn foo<T: Trait<A = T::B>>() { }
   |
   |
   = note: ...which again requires computing the bounds for type parameter `T`, completing the cycle
note: cycle used when getting explicit predicates for `foo`
   |
   |
LL | fn foo<T: Trait<A = T::B>>() { }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---
- error[E0391]: cycle detected when processing `Foo::T`
+ error[E0391]: cycle detected when getting type of `Foo::T`
2   --> $DIR/issue-34373.rs:7:30
3    |
4 LL | pub struct Foo<T = Box<Trait<DefaultFoo>>>;
5    |                              ^^^^^^^^^^
6    |
6    |
- note: ...which requires processing `DefaultFoo`...
+ note: ...which requires getting type of `DefaultFoo`...
9    |
9    |
10 LL | type DefaultFoo = Foo;
11    |                   ^^^
11    |                   ^^^
-    = note: ...which again requires processing `Foo::T`, completing the cycle
+    = note: ...which again requires getting type of `Foo::T`, completing the cycle
13 note: cycle used when collecting item types in top-level module
15    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/issue-34373.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-34373.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34373.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when getting type of `Foo::T`
  --> /checkout/src/test/ui/issues/issue-34373.rs:7:30
   |
LL | pub struct Foo<T = Box<Trait<DefaultFoo>>>;  //~ ERROR cycle detected
   |
   |
note: ...which requires getting type of `DefaultFoo`...
   |
   |
LL | type DefaultFoo = Foo;
   |                   ^^^
   = note: ...which again requires getting type of `Foo::T`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
   |
LL | / #![allow(warnings)]
LL | | trait Trait<T> {
LL | |     fn foo(_: T) {}
...  |
LL | | fn main() {
---
---- [ui] ui/recursion/issue-26548-recursion-via-normalize.rs stdout ----
diff of stderr:

2    |
3    = note: ...which requires computing layout of `S`...
4    = note: ...which again requires computing layout of `std::option::Option<S>`, completing the cycle
- note: cycle used when processing `main`
+ note: cycle used when optimizing MIR for `main`
7    |
8 LL | fn main() {



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-26548-recursion-via-normalize/issue-26548-recursion-via-normalize.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/issue-26548-recursion-via-normalize.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/issue-26548-recursion-via-normalize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-26548-recursion-via-normalize" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-26548-recursion-via-normalize/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing layout of `std::option::Option<S>`
   |
   = note: ...which requires computing layout of `S`...
   = note: ...which again requires computing layout of `std::option::Option<S>`, completing the cycle
note: cycle used when optimizing MIR for `main`
   |
   |
LL | fn main() { //~ NOTE cycle used when processing `main`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.

------------------------------------------


---- [ui] ui/resolve/issue-23305.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when processing `<impl at $DIR/issue-23305.rs:5:1: 5:24>`
+ error[E0391]: cycle detected when getting type of `<impl at $DIR/issue-23305.rs:5:1: 5:24>`
3    |
3    |
4 LL | impl dyn ToNbt<Self> {}
5    |                ^^^^
6    |
6    |
-    = note: ...which again requires processing `<impl at $DIR/issue-23305.rs:5:1: 5:24>`, completing the cycle
+    = note: ...which again requires getting type of `<impl at $DIR/issue-23305.rs:5:1: 5:24>`, completing the cycle
8 note: cycle used when collecting item types in top-level module
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/issue-23305.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-23305.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-23305.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when getting type of `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:5:1: 5:24>`
   |
   |
LL | impl dyn ToNbt<Self> {}
   |
   |
   = note: ...which again requires getting type of `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:5:1: 5:24>`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
   |
LL | pub trait ToNbt<T> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.

------------------------------------------


---- [ui] ui/resolve/resolve-self-in-impl.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:14:1: 14:20>`
+ error[E0391]: cycle detected when getting type of `<impl at $DIR/resolve-self-in-impl.rs:14:1: 14:20>`
3    |
4 LL | impl Tr for Self {}

5    |             ^^^^
5    |             ^^^^
6    |
-    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:14:1: 14:20>`, completing the cycle
+    = note: ...which again requires getting type of `<impl at $DIR/resolve-self-in-impl.rs:14:1: 14:20>`, completing the cycle
8 note: cycle used when collecting item types in top-level module
10    |

17 LL | | fn main() {}
18    | |____________^
18    | |____________^
19 
- error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:15:1: 15:23>`
+ error[E0391]: cycle detected when getting type of `<impl at $DIR/resolve-self-in-impl.rs:15:1: 15:23>`
22    |
22    |
23 LL | impl Tr for S<Self> {}
24    |               ^^^^
25    |
25    |
-    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:15:1: 15:23>`, completing the cycle
+    = note: ...which again requires getting type of `<impl at $DIR/resolve-self-in-impl.rs:15:1: 15:23>`, completing the cycle
27 note: cycle used when collecting item types in top-level module
29    |

36 LL | | fn main() {}
37    | |____________^
37    | |____________^
38 
- error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:13>`
+ error[E0391]: cycle detected when getting type of `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:13>`
41    |
42 LL | impl Self {}

43    |      ^^^^
43    |      ^^^^
44    |
-    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:13>`, completing the cycle
+    = note: ...which again requires getting type of `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:13>`, completing the cycle
46 note: cycle used when collecting item types in top-level module
48    |

55 LL | | fn main() {}
56    | |____________^
56    | |____________^
57 
- error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:16>`
+ error[E0391]: cycle detected when getting type of `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:16>`
60    |
60    |
61 LL | impl S<Self> {}
62    |        ^^^^
63    |
63    |
-    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:16>`, completing the cycle
+    = note: ...which again requires getting type of `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:16>`, completing the cycle
65 note: cycle used when collecting item types in top-level module
67    |

74 LL | | fn main() {}
75    | |____________^
75    | |____________^
76 
- error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:18:1: 18:26>`
+ error[E0391]: cycle detected when getting trait implemented by `<impl at $DIR/resolve-self-in-impl.rs:18:1: 18:26>`
79    |
79    |
80 LL | impl Tr<Self::A> for S {}
81    | ^^^^^^^^^^^^^^^^^^^^^^
82    |
82    |
-    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:18:1: 18:26>`, completing the cycle
+    = note: ...which again requires getting trait implemented by `<impl at $DIR/resolve-self-in-impl.rs:18:1: 18:26>`, completing the cycle
84 note: cycle used when collecting item types in top-level module
86    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl/resolve-self-in-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/resolve-self-in-impl.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/resolve-self-in-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when getting type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:14:1: 14:20>`
   |
   |
LL | impl Tr for Self {} //~ ERROR cycle detected
   |
   |
   = note: ...which again requires getting type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:14:1: 14:20>`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when getting type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:15:1: 15:23>`
   |
   |
LL | impl Tr for S<Self> {} //~ ERROR cycle detected
   |
   |
   = note: ...which again requires getting type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:15:1: 15:23>`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when getting type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:16:1: 16:13>`
   |
   |
LL | impl Self {} //~ ERROR cycle detected
   |
   |
   = note: ...which again requires getting type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:16:1: 16:13>`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when getting type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:17:1: 17:16>`
   |
   |
LL | impl S<Self> {} //~ ERROR cycle detected
   |
   |
   = note: ...which again requires getting type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:17:1: 17:16>`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when getting trait implemented by `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:18:1: 18:26>`
   |
   |
LL | impl Tr<Self::A> for S {} //~ ERROR cycle detected
   |
   |
   = note: ...which again requires getting trait implemented by `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:18:1: 18:26>`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:59:39
Build completed unsuccessfully in 0:59:39
== clock drift check ==
  local time: Thu May 28 21:11:07 UTC 2020
  network time: Thu, 28 May 2020 21:11:07 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72707/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72707/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3528) (python)
##[section]Finishing: Finalize Job
