plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 42'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9bed539b-c16f-4ca1-8883-77aa4e3f3d84.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71420/merge:refs/remotes/pull/71420/merge
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
...........................................................................F........................ 2600/10319
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
.....i.ii........................................................................................... 6900/10319
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
+    |
+ LL | #![feature(specialization)]
+    |            ^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #31844 <***/issues/31844> for more information
1 error[E0391]: cycle detected when building specialization graph of trait `Trait`
2   --> $DIR/coherence-inherited-assoc-ty-cycle-err.rs:8:1
3    |

---
17 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-inherited-assoc-ty-cycle-err/coherence-inherited-assoc-ty-cycle-err.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-inherited-assoc-ty-cycle-err.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-inherited-assoc-ty-cycle-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-inherited-assoc-ty-cycle-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-inherited-assoc-ty-cycle-err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/coherence/coherence-inherited-assoc-ty-cycle-err.rs:6:12
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
error[E0391]: cycle detected when building specialization graph of trait `Trait`
  --> /checkout/src/test/ui/coherence/coherence-inherited-assoc-ty-cycle-err.rs:8:1
   |
LL | trait Trait<T> { type Assoc; }
LL | trait Trait<T> { type Assoc; }
   | ^^^^^^^^^^^^^^
   |
   = note: ...which again requires building specialization graph of trait `Trait`, completing the cycle
note: cycle used when coherence checking all impls of trait `Trait`
   |
LL | trait Trait<T> { type Assoc; }
   | ^^^^^^^^^^^^^^

---
+    |
+ LL | #![feature(specialization)]
+    |            ^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #31844 <***/issues/31844> for more information
+ 
1 error[E0520]: `fly` specializes an item from a parent `impl`, but that item is not marked `default`
2   --> $DIR/E0520.rs:16:5

11    |
11    |
12    = note: to specialize, `fly` in the parent `impl` must be marked `default`
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
15 
16 For more information about this error, try `rustc --explain E0520`.
16 For more information about this error, try `rustc --explain E0520`.
17 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0520/E0520.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0520.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0520.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0520" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0520/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/error-codes/E0520.rs:1:12
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information

error[E0520]: `fly` specializes an item from a parent `impl`, but that item is not marked `default`
   |
   |
LL | / impl<T: Clone> SpaceLlama for T {
LL | |     fn fly(&self) {}
   | |_- parent `impl` is here
...
...
LL |       default fn fly(&self) {}
   |       ^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `fly`
   |
   = note: to specialize, `fly` in the parent `impl` must be marked `default`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0520`.

---
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35376/issue-35376.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-35376.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35376.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35376" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35376/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/issues/issue-35376.rs:2:12
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
warning: 1 warning emitted


------------------------------------------
---
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38091/issue-38091.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-38091.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-38091.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38091/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38091/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/issues/issue-38091.rs:2:12
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
warning: 1 warning emitted


------------------------------------------
---
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55380/issue-55380.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-55380.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55380.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55380/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55380/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/issues/issue-55380.rs:3:12
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/parser/assoc-static-semantic-fail.rs stdout ----
diff of stderr:

162 LL |     pub default static TD: u8;
163    |     ^^^ `pub` not permitted here because it's implied
- error: aborting due to 24 previous errors
+ warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
+   --> $DIR/assoc-static-semantic-fail.rs:3:12
+    |
+    |
+ LL | #![feature(specialization)]
+    |            ^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #31844 <***/issues/31844> for more information
+ 
+ error: aborting due to 24 previous errors; 1 warning emitted
167 For more information about this error, try `rustc --explain E0449`.
168 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/assoc-static-semantic-fail/assoc-static-semantic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/assoc-static-semantic-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/assoc-static-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/assoc-static-semantic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/assoc-static-semantic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:9:5
   |
LL |     static IA: u8 = 0;

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:11:5
   |
   |
LL |     static IB: u8;

error: a static item cannot be `default`
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:14:5
   |
   |
LL |     default static IC: u8 = 0;
   |     ^^^^^^^ `default` because of this
   = note: only associated `fn`, `const`, and `type` items can be `default`

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:14:5
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:14:5
   |
LL |     default static IC: u8 = 0;

error: a static item cannot be `default`
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:17:16
   |
---

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:26:5
   |
LL |     static TB: u8;

error: a static item cannot be `default`
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:28:5
   |
   |
LL |     default static TC: u8 = 0;
   |     ^^^^^^^ `default` because of this
   = note: only associated `fn`, `const`, and `type` items can be `default`

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:28:5
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:28:5
   |
LL |     default static TC: u8 = 0;

error: a static item cannot be `default`
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:31:16
   |
   |
LL |     pub(crate) default static TD: u8;
   |                ^^^^^^^ `default` because of this
   = note: only associated `fn`, `const`, and `type` items can be `default`

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:31:5
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:31:5
   |
LL |     pub(crate) default static TD: u8;

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:38:5
   |
   |
LL |     static TA: u8 = 0;
   |     ^^^^^^^^^^^^^^^^^^

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:40:5
   |
LL |     static TB: u8;

error: a static item cannot be `default`
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:43:5
   |
   |
LL |     default static TC: u8 = 0;
   |     ^^^^^^^ `default` because of this
   = note: only associated `fn`, `const`, and `type` items can be `default`

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:43:5
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:43:5
   |
LL |     default static TC: u8 = 0;

error: a static item cannot be `default`
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:46:9
   |
   |
LL |     pub default static TD: u8;
   |         ^^^^^^^ `default` because of this
   = note: only associated `fn`, `const`, and `type` items can be `default`

error: associated `static` items are not allowed
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:46:5
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:46:5
   |
LL |     pub default static TD: u8;

error: associated constant in `impl` without body
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:11:5
   |
   |
LL |     static IB: u8;
   |                  |
   |                  |
   |                  help: provide a definition for the constant: `= <expr>;`
error: associated constant in `impl` without body
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:17:5
   |
LL |     pub(crate) default static ID: u8;
LL |     pub(crate) default static ID: u8;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
   |                                     |
   |                                     help: provide a definition for the constant: `= <expr>;`
error[E0449]: unnecessary visibility qualifier
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:31:5
   |
   |
LL |     pub(crate) default static TD: u8;

error: associated constant in `impl` without body
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:40:5
   |
   |
LL |     static TB: u8;
   |                  |
   |                  |
   |                  help: provide a definition for the constant: `= <expr>;`
error: associated constant in `impl` without body
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:46:5
   |
   |
LL |     pub default static TD: u8;
   |                              |
   |                              |
   |                              help: provide a definition for the constant: `= <expr>;`
error[E0449]: unnecessary visibility qualifier
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:46:5
   |
   |
LL |     pub default static TD: u8;
   |     ^^^ `pub` not permitted here because it's implied
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/parser/assoc-static-semantic-fail.rs:3:12
   |
LL | #![feature(specialization)]
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information

error: aborting due to 24 previous errors; 1 warning emitted
For more information about this error, try `rustc --explain E0449`.

------------------------------------------



---- [ui] ui/parser/default.rs stdout ----
diff of stderr:

23 LL |     pub default fn foo<T: Default>() -> T {
24    |     ^^^ `pub` not permitted here because it's implied
+ warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
+   --> $DIR/default.rs:3:12
+    |
+ LL | #![feature(specialization)]
+ LL | #![feature(specialization)]
+    |            ^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #31844 <***/issues/31844> for more information
26 error[E0046]: not all trait items implemented, missing: `foo`
27   --> $DIR/default.rs:21:1
28    |


32 LL | impl Foo for u32 {
33    | ^^^^^^^^^^^^^^^^ missing `foo` in implementation
34 
- error: aborting due to 4 previous errors
+ error: aborting due to 4 previous errors; 1 warning emitted
37 Some errors have detailed explanations: E0046, E0449.
38 For more information about an error, try `rustc --explain E0046`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default/default.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/default.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `default` is not followed by an item
  --> /checkout/src/test/ui/parser/default.rs:22:5
   |
LL |     default pub fn foo<T: Default>() -> T { T::default() }
   |     ^^^^^^^ the `default` qualifier
   = note: only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`

error: non-item in item list
  --> /checkout/src/test/ui/parser/default.rs:22:13
  --> /checkout/src/test/ui/parser/default.rs:22:13
   |
LL | impl Foo for u32 { //~ ERROR not all trait items implemented, missing: `foo`
   |                  - item list starts here
LL |     default pub fn foo<T: Default>() -> T { T::default() }
   |             ^^^ non-item starts here
LL | }
   | - item list ends here

error[E0449]: unnecessary visibility qualifier
error[E0449]: unnecessary visibility qualifier
  --> /checkout/src/test/ui/parser/default.rs:16:5
   |
LL |     pub default fn foo<T: Default>() -> T { //~ ERROR unnecessary visibility qualifier
   |     ^^^ `pub` not permitted here because it's implied
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/parser/default.rs:3:12
   |
LL | #![feature(specialization)]
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <***/issues/31844> for more information
error[E0046]: not all trait items implemented, missing: `foo`
  --> /checkout/src/test/ui/parser/default.rs:21:1
   |
LL |     fn foo<T: Default>() -> T;
LL |     fn foo<T: Default>() -> T;
   |     -------------------------- `foo` from trait
...
LL | impl Foo for u32 { //~ ERROR not all trait items implemented, missing: `foo`


error: aborting due to 4 previous errors; 1 warning emitted
Some errors have detailed explanations: E0046, E0449.
For more information about an error, try `rustc --explain E0046`.

------------------------------------------
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:04:16
Build completed unsuccessfully in 1:04:16
== clock drift check ==
  local time: Tue Jun 16 09:15:36 UTC 2020
  network time: Tue, 16 Jun 2020 09:15:36 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71420/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3477) (python)
##[section]Finishing: Finalize Job
