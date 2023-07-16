plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
Agent machine name: 'fv-az578'
Current agent version: '2.169.1'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ea891e53-af2a-420f-9fe8-7fb6142d3214.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73079/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73079/merge:refs/remotes/pull/73079/merge
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
............................i...............i....................................................... 5200/10285
....F............................................................................................... 5300/10285
............................................................................i....................... 5400/10285
......................................................................i............................. 5500/10285
.......................................................................................ii.ii........ 5600/10285
i...i.......................................FF...................................................... 5700/10285
......................................i............................................................. 5900/10285
............................................................................................ii...... 6000/10285
...............................i.................................................................... 6100/10285
.................................................................................................... 6200/10285
.................................................................................................... 6200/10285
.................................................................................................... 6300/10285
......................................................ii...i..ii...........i........................ 6400/10285
.................................................................................................... 6600/10285
.................................................................................................... 6700/10285
.................................................................................................... 6700/10285
.......................................................................................i..ii........ 6800/10285
.................................................................................................... 7000/10285
.................................F.................................................................. 7100/10285
.........................................i.......................................................... 7200/10285
.................................................................................................... 7300/10285
.................................................................................................... 7300/10285
...................................................................................i................ 7400/10285
.................................................................................................... 7500/10285
.................................................................................................... 7600/10285
............................................FFF.FFFFFFFFF.................FFFF...................... 7700/10285
..............i........i............................................................................ 7900/10285
.................................................................................................... 8000/10285
.................................................................................................... 8100/10285
....................................................................F............................... 8200/10285
....................................................................F............................... 8200/10285
.................................................................................................... 8300/10285
................................................................................i................... 8400/10285
.................................................................................................... 8500/10285
..................................iiiiii.iiiiii.iF.................................................. 8600/10285
.................................................................................................... 8800/10285
.................................................................................................... 8900/10285
..........F......................................................................................... 9000/10285
.................................................................................................... 9100/10285
---

---- [ui] ui/associated-types/associated-types-ICE-when-projecting-out-of-err.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `(): Add<A>` is not satisfied
-    |
-    |
- LL |     r = r + a;
-    |           ^ the trait `Add<A>` is not implemented for `()`
+ error: requires `panic_location` lang_item
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0277`.
- For more information about this error, try `rustc --explain E0277`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err/associated-types-ICE-when-projecting-out-of-err.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-ICE-when-projecting-out-of-err.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/conditional-compilation/cfg-attr-crate-2.rs stdout ----
diff of stderr:

7    = note: see issue #29639 <***/issues/29639> for more information
8    = help: add `#![feature(no_core)]` to the crate attributes to enable
- error: aborting due to previous error
+ error: requires `panic_location` lang_item
+ 
+ error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors
11 
12 For more information about this error, try `rustc --explain E0658`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-crate-2/cfg-attr-crate-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args conditional-compilation/cfg-attr-crate-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-attr-crate-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-crate-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "broken" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-crate-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `#[no_core]` attribute is an experimental feature
  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-crate-2.rs:6:21
   |
LL | #![cfg_attr(broken, no_core)] //~ ERROR the `#[no_core]` attribute is an experimental feature
   |
   |
   = note: see issue #29639 <***/issues/29639> for more information
   = help: add `#![feature(no_core)]` to the crate attributes to enable
error: requires `panic_location` lang_item

error: aborting due to 2 previous errors

---

---- [ui] ui/conditional-compilation/cfg-attr-multi-invalid-1.rs stdout ----
diff of stderr:

7    = note: see issue #29639 <***/issues/29639> for more information
8    = help: add `#![feature(no_core)]` to the crate attributes to enable
- error: aborting due to previous error
+ error: requires `panic_location` lang_item
+ 
+ error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors
11 
12 For more information about this error, try `rustc --explain E0658`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-invalid-1/cfg-attr-multi-invalid-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args conditional-compilation/cfg-attr-multi-invalid-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-invalid-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-invalid-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "broken" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-invalid-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `#[no_core]` attribute is an experimental feature
  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-invalid-1.rs:4:21
   |
LL | #![cfg_attr(broken, no_core, no_std)]
   |
   |
   = note: see issue #29639 <***/issues/29639> for more information
   = help: add `#![feature(no_core)]` to the crate attributes to enable
error: requires `panic_location` lang_item

error: aborting due to 2 previous errors

---

---- [ui] ui/conditional-compilation/cfg-attr-multi-invalid-2.rs stdout ----
diff of stderr:

7    = note: see issue #29639 <***/issues/29639> for more information
8    = help: add `#![feature(no_core)]` to the crate attributes to enable
- error: aborting due to previous error
+ error: requires `panic_location` lang_item
+ 
+ error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors
11 
12 For more information about this error, try `rustc --explain E0658`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-invalid-2/cfg-attr-multi-invalid-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args conditional-compilation/cfg-attr-multi-invalid-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-invalid-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-invalid-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "broken" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-invalid-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `#[no_core]` attribute is an experimental feature
  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-invalid-2.rs:4:29
   |
LL | #![cfg_attr(broken, no_std, no_core)]
   |
   |
   = note: see issue #29639 <***/issues/29639> for more information
   = help: add `#![feature(no_core)]` to the crate attributes to enable
error: requires `panic_location` lang_item

error: aborting due to 2 previous errors

---

---- [ui] ui/feature-gates/feature-gate-cfg-target-has-atomic.rs stdout ----
diff of stderr:

160    = note: see issue #32976 <***/issues/32976> for more information
161    = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
- error: aborting due to 18 previous errors
+ error: requires `panic_location` lang_item
+ 
+ error: aborting due to 19 previous errors
+ error: aborting due to 19 previous errors
164 
165 For more information about this error, try `rustc --explain E0658`.
166 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-has-atomic/feature-gate-cfg-target-has-atomic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-cfg-target-has-atomic.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-has-atomic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-has-atomic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:15:7
   |
LL | #[cfg(target_has_atomic = "8")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:21:7
   |
   |
LL | #[cfg(target_has_atomic = "8")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:26:7
   |
   |
LL | #[cfg(target_has_atomic = "16")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:31:7
   |
   |
LL | #[cfg(target_has_atomic = "16")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:36:7
   |
   |
LL | #[cfg(target_has_atomic = "32")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:41:7
   |
   |
LL | #[cfg(target_has_atomic = "32")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:46:7
   |
   |
LL | #[cfg(target_has_atomic = "64")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:51:7
   |
   |
LL | #[cfg(target_has_atomic = "64")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:56:7
   |
   |
LL | #[cfg(target_has_atomic = "128")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:61:7
   |
   |
LL | #[cfg(target_has_atomic = "128")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:66:7
   |
   |
LL | #[cfg(target_has_atomic = "ptr")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:71:7
   |
   |
LL | #[cfg(target_has_atomic = "ptr")]
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:78:10
   |
   |
LL |     cfg!(target_has_atomic = "8");
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:80:10
   |
   |
LL |     cfg!(target_has_atomic = "16");
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:82:10
   |
   |
LL |     cfg!(target_has_atomic = "32");
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:84:10
   |
   |
LL |     cfg!(target_has_atomic = "64");
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:86:10
   |
   |
LL |     cfg!(target_has_atomic = "128");
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs:88:10
   |
   |
LL |     cfg!(target_has_atomic = "ptr");
   |
   |
   = note: see issue #32976 <***/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
error: requires `panic_location` lang_item

error: aborting due to 19 previous errors

---

---- [ui] ui/feature-gates/feature-gate-no_core.rs stdout ----
diff of stderr:

7    = note: see issue #29639 <***/issues/29639> for more information
8    = help: add `#![feature(no_core)]` to the crate attributes to enable
- error: aborting due to previous error
+ error: requires `panic_location` lang_item
+ 
+ error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors
11 
12 For more information about this error, try `rustc --explain E0658`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_core/feature-gate-no_core.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-no_core.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-no_core.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_core" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_core/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `#[no_core]` attribute is an experimental feature
  --> /checkout/src/test/ui/feature-gates/feature-gate-no_core.rs:3:1
   |
LL | #![no_core] //~ ERROR the `#[no_core]` attribute is an experimental feature
   |
   |
   = note: see issue #29639 <***/issues/29639> for more information
   = help: add `#![feature(no_core)]` to the crate attributes to enable
error: requires `panic_location` lang_item

error: aborting due to 2 previous errors

---
---- [ui] ui/hygiene/unpretty-debug.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/unpretty-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=expanded,hygiene" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19660/issue-19660.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-19660.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-19660.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19660" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19660/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/issues/issue-31076.rs stdout ----
diff of stderr:

- error[E0369]: cannot add `{integer}` to `{integer}`
-    |
- LL |     let x = 5 + 6;
- LL |     let x = 5 + 6;
-    |             - ^ - {integer}
-    |             {integer}
+ error: requires `panic_location` lang_item
8 
- error[E0369]: cannot add `i32` to `i32`
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/issue-31076.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-31076.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/issues/issue-50993.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50993.rs" "-Zthreads=1" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50993" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "--target" "thumbv7em-none-eabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50993/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- error: requires `generator` lang_item
-   --> $DIR/lang-item-missing-generator.rs:15:17
-    |
- LL | pub fn abc() -> impl FnOnce(f32) {
+ error: requires `panic_location` lang_item
6 
7 error: aborting due to previous error
8 
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-item-missing-generator/lang-item-missing-generator.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lang-item-missing-generator.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-item-missing-generator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-item-missing-generator" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-item-missing-generator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
4 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-item-missing/lang-item-missing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lang-item-missing.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-item-missing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-item-missing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-item-missing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
4 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-requires-panic-info/panic-handler-requires-panic-info.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panic-handler/panic-handler-requires-panic-info.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-requires-panic-info.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-requires-panic-info" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-requires-panic-info/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---


---- [ui] ui/print_type_sizes/anonymous.rs stdout ----
normalized stdout:
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/anonymous/anonymous.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/anonymous/anonymous.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/anonymous.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/anonymous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/anonymous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/anonymous/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/generics.rs stdout ----
diff of stdout:

3 print-type-size     field `._cdr`: 50 bytes
4 print-type-size type: `FiftyBytes`: 50 bytes, alignment: 1 bytes
5 print-type-size     field `.0`: 50 bytes
+ print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
6 print-type-size type: `Pair<SevenBytes>`: 14 bytes, alignment: 1 bytes
7 print-type-size     field `._car`: 7 bytes
8 print-type-size     field `._cdr`: 7 bytes

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/generics/generics.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/generics/generics.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/generics.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/generics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/generics/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `Pair<FiftyBytes>`: 100 bytes, alignment: 1 bytes
print-type-size     field `._car`: 50 bytes
print-type-size     field `._cdr`: 50 bytes
print-type-size type: `FiftyBytes`: 50 bytes, alignment: 1 bytes
print-type-size     field `.0`: 50 bytes
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
print-type-size type: `Pair<SevenBytes>`: 14 bytes, alignment: 1 bytes
print-type-size     field `._car`: 7 bytes
print-type-size     field `._cdr`: 7 bytes
print-type-size type: `SevenBytes`: 7 bytes, alignment: 1 bytes
print-type-size     field `.0`: 7 bytes
print-type-size type: `Pair<u8>`: 2 bytes, alignment: 1 bytes
print-type-size     field `._car`: 1 bytes
print-type-size     field `._cdr`: 1 bytes
print-type-size type: `ZeroSized`: 0 bytes, alignment: 1 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/multiple_types.rs stdout ----
diff of stdout:

6 print-type-size         field `.0`: 7 bytes
7 print-type-size type: `FiftyBytes`: 50 bytes, alignment: 1 bytes
8 print-type-size     field `.0`: 50 bytes
+ print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
9 print-type-size type: `SevenBytes`: 7 bytes, alignment: 1 bytes
10 print-type-size     field `.0`: 7 bytes


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/multiple_types/multiple_types.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/multiple_types/multiple_types.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/multiple_types.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/multiple_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/multiple_types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/multiple_types/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `Enum`: 51 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Large`: 50 bytes
print-type-size         field `.0`: 50 bytes
print-type-size     variant `Small`: 7 bytes
print-type-size         field `.0`: 7 bytes
print-type-size type: `FiftyBytes`: 50 bytes, alignment: 1 bytes
print-type-size     field `.0`: 50 bytes
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
print-type-size type: `SevenBytes`: 7 bytes, alignment: 1 bytes
print-type-size     field `.0`: 7 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/niche-filling.rs stdout ----
diff of stdout:

+ print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
1 print-type-size type: `IndirectNonZero`: 12 bytes, alignment: 4 bytes
2 print-type-size     field `.nested`: 8 bytes
3 print-type-size     field `.post`: 2 bytes

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling/niche-filling.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling/niche-filling.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/niche-filling.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/niche-filling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
print-type-size type: `IndirectNonZero`: 12 bytes, alignment: 4 bytes
print-type-size     field `.nested`: 8 bytes
print-type-size     field `.post`: 2 bytes
print-type-size     field `.pre`: 1 bytes
print-type-size     end padding: 1 bytes
print-type-size type: `MyOption<IndirectNonZero>`: 12 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 12 bytes
print-type-size         field `.0`: 12 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `EmbeddedDiscr`: 8 bytes, alignment: 4 bytes
print-type-size     variant `Record`: 7 bytes
print-type-size         field `.val`: 4 bytes
print-type-size         field `.post`: 2 bytes
print-type-size         field `.pre`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size     end padding: 1 bytes
print-type-size type: `MyOption<Union1<std::num::NonZeroU32>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<Union2<std::num::NonZeroU32, std::num::NonZeroU32>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<Union2<std::num::NonZeroU32, u32>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `NestedNonZero`: 8 bytes, alignment: 4 bytes
print-type-size     field `.val`: 4 bytes
print-type-size     field `.post`: 2 bytes
print-type-size     field `.pre`: 1 bytes
print-type-size     end padding: 1 bytes
print-type-size type: `Enum4<(), char, (), ()>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Two`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Three`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Four`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyOption<char>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `Union1<std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union1`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size type: `Union2<std::num::NonZeroU32, std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union2`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size         field `.b`: 4 bytes, offset: 0 bytes, alignment: 4 bytes
print-type-size type: `Union2<std::num::NonZeroU32, u32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union2`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size         field `.b`: 4 bytes, offset: 0 bytes, alignment: 4 bytes
print-type-size type: `std::num::NonZeroU32`: 4 bytes, alignment: 4 bytes
print-type-size     field `.0`: 4 bytes
print-type-size type: `Enum4<(), (), (), MyOption<u8>>`: 2 bytes, alignment: 1 bytes
print-type-size     variant `Four`: 2 bytes
print-type-size         field `.0`: 2 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Two`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Three`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyOption<MyOption<u8>>`: 2 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 2 bytes
print-type-size         field `.0`: 2 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<u8>`: 2 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `Enum4<(), (), bool, ()>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Three`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Two`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Four`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyOption<bool>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<std::cmp::Ordering>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `std::cmp::Ordering`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Less`: 0 bytes
print-type-size     variant `Equal`: 0 bytes
print-type-size     variant `Greater`: 0 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/no_duplicates.rs stdout ----
diff of stdout:

+ print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
1 print-type-size type: `SevenBytes`: 7 bytes, alignment: 1 bytes
2 print-type-size     field `.0`: 7 bytes


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/no_duplicates/no_duplicates.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/no_duplicates/no_duplicates.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/no_duplicates.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/no_duplicates.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/no_duplicates" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/no_duplicates/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
print-type-size type: `SevenBytes`: 7 bytes, alignment: 1 bytes
print-type-size     field `.0`: 7 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/packed.rs stdout ----
diff of stdout:

+ print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
1 print-type-size type: `Packed2C`: 12 bytes, alignment: 2 bytes
2 print-type-size     field `.a`: 1 bytes
3 print-type-size     field `.b`: 1 bytes

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/packed/packed.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/packed/packed.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/packed.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/packed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/packed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/packed/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
print-type-size type: `Packed2C`: 12 bytes, alignment: 2 bytes
print-type-size     field `.a`: 1 bytes
print-type-size     field `.b`: 1 bytes
print-type-size     field `.g`: 4 bytes
print-type-size     field `.c`: 1 bytes
print-type-size     padding: 1 bytes
print-type-size     field `.h`: 2 bytes
print-type-size     field `.d`: 1 bytes
print-type-size     end padding: 1 bytes
print-type-size type: `Padded`: 12 bytes, alignment: 4 bytes
print-type-size     field `.g`: 4 bytes
print-type-size     field `.h`: 2 bytes
print-type-size     field `.a`: 1 bytes
print-type-size     field `.b`: 1 bytes
print-type-size     field `.c`: 1 bytes
print-type-size     field `.d`: 1 bytes
print-type-size     end padding: 2 bytes
print-type-size type: `Packed1`: 10 bytes, alignment: 1 bytes
print-type-size     field `.a`: 1 bytes
print-type-size     field `.b`: 1 bytes
print-type-size     field `.g`: 4 bytes
print-type-size     field `.c`: 1 bytes
print-type-size     field `.h`: 2 bytes
print-type-size     field `.d`: 1 bytes
print-type-size type: `Packed2`: 10 bytes, alignment: 2 bytes
print-type-size     field `.g`: 4 bytes
print-type-size     field `.h`: 2 bytes
print-type-size     field `.a`: 1 bytes
print-type-size     field `.b`: 1 bytes
print-type-size     field `.c`: 1 bytes
print-type-size     field `.d`: 1 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/padding.rs stdout ----
diff of stdout:

+ print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
1 print-type-size type: `E1`: 12 bytes, alignment: 4 bytes
2 print-type-size     discriminant: 1 bytes
3 print-type-size     variant `B`: 11 bytes

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/padding/padding.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/padding/padding.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/padding.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/padding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/padding" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/padding/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
print-type-size type: `E1`: 12 bytes, alignment: 4 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `B`: 11 bytes
print-type-size         padding: 3 bytes
print-type-size         field `.0`: 8 bytes, alignment: 4 bytes
print-type-size     variant `A`: 7 bytes
print-type-size         field `.1`: 1 bytes
print-type-size         padding: 2 bytes
print-type-size         field `.0`: 4 bytes, alignment: 4 bytes
print-type-size type: `E2`: 12 bytes, alignment: 4 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `B`: 11 bytes
print-type-size         padding: 3 bytes
print-type-size         field `.0`: 8 bytes, alignment: 4 bytes
print-type-size     variant `A`: 7 bytes
print-type-size         field `.0`: 1 bytes
print-type-size         padding: 2 bytes
print-type-size         field `.1`: 4 bytes, alignment: 4 bytes
print-type-size type: `S`: 8 bytes, alignment: 4 bytes
print-type-size     field `.g`: 4 bytes
print-type-size     field `.a`: 1 bytes
print-type-size     field `.b`: 1 bytes
print-type-size     end padding: 2 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/repr-align.rs stdout ----
diff of stdout:

11 print-type-size     field `.b`: 4 bytes
12 print-type-size     field `.d`: 1 bytes
13 print-type-size     end padding: 7 bytes
+ print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
14 print-type-size type: `A`: 16 bytes, alignment: 16 bytes
15 print-type-size     field `.0`: 4 bytes
16 print-type-size     end padding: 12 bytes

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr-align/repr-align.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr-align/repr-align.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/repr-align.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/repr-align.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr-align" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr-align/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `E`: 32 bytes, alignment: 16 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `B`: 28 bytes
print-type-size         padding: 12 bytes
print-type-size         field `.0`: 16 bytes, alignment: 16 bytes
print-type-size     variant `A`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size type: `S`: 32 bytes, alignment: 16 bytes
print-type-size     field `.c`: 16 bytes
print-type-size     field `.a`: 4 bytes
print-type-size     field `.b`: 4 bytes
print-type-size     field `.d`: 1 bytes
print-type-size     end padding: 7 bytes
print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
print-type-size type: `A`: 16 bytes, alignment: 16 bytes
print-type-size     field `.0`: 4 bytes
print-type-size     end padding: 12 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/repr_int_c.rs stdout ----
diff of stdout:

+ print-type-size type: `std::panic::Location`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
1 print-type-size type: `ReprCu8`: 4 bytes, alignment: 2 bytes
2 print-type-size     discriminant: 1 bytes
3 print-type-size     variant `A`: 3 bytes

The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr_int_c/repr_int_c.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/repr_int_c.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
---
156 
- error[E0624]: associated function `bar` is private
-   --> $DIR/privacy1.rs:77:23
-    |
- LL |         self::baz::A::bar();
+ error: requires `panic_location` lang_item
162 
- error[E0624]: associated function `bar` is private
-   --> $DIR/privacy1.rs:95:13
---
168 
- error[E0624]: associated function `bar` is private
-   --> $DIR/privacy1.rs:102:19
-    |
- LL |         ::bar::A::bar();
- 
- error[E0624]: associated function `bar` is private
-   --> $DIR/privacy1.rs:105:24
-    |
-    |
- LL |         ::bar::baz::A::bar();
- 
- error[E0624]: associated function `bar2` is private
-   --> $DIR/privacy1.rs:108:23
-    |
-    |
- LL |         ::bar::baz::A.bar2();
- 
- error: aborting due to 18 previous errors
- 
- Some errors have detailed explanations: E0603, E0624.
- Some errors have detailed explanations: E0603, E0624.
- For more information about an error, try `rustc --explain E0603`.
+ For more information about this error, try `rustc --explain E0603`.
191 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1/privacy1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/privacy1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: module `baz` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:132:18
   |
LL |         use bar::baz::{foo, bar};
   |
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
   |
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:132:18
   |
LL |         use bar::baz::{foo, bar};
   |
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
---

error[E0603]: module `i` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:165:20
   |
LL |     use self::foo::i::A; //~ ERROR: module `i` is private
   |                    ^ private module
note: the module `i` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:170:9
   |
LL |         mod i {
LL |         mod i {
   |         ^^^^^

error[E0603]: module `baz` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:104:16
   |
LL |         ::bar::baz::A::foo();   //~ ERROR: module `baz` is private
   |
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
   |
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:105:16
   |
LL |         ::bar::baz::A::bar();   //~ ERROR: module `baz` is private
   |
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
   |
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:107:16
   |
LL |         ::bar::baz::A.foo2();   //~ ERROR: module `baz` is private
   |
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
   |
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:108:16
   |
LL |         ::bar::baz::A.bar2();   //~ ERROR: module `baz` is private
   |
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
   |
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: trait `B` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:112:16
   |
LL |         ::bar::B::foo();        //~ ERROR: trait `B` is private
   |                ^ private trait
note: the trait `B` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:40:5
   |
LL |     trait B {
LL |     trait B {
   |     ^^^^^^^

error[E0603]: function `epriv` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:118:20
   |
LL |             ::bar::epriv(); //~ ERROR: function `epriv` is private
   |
note: the function `epriv` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:65:9
   |
   |
LL |         fn epriv();

error[E0603]: module `baz` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:127:16
   |
   |
LL |         ::bar::baz::foo(); //~ ERROR: module `baz` is private
   |
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
   |
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:128:16
   |
LL |         ::bar::baz::bar(); //~ ERROR: module `baz` is private
   |
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
   |
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: trait `B` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:157:17
   |
LL |     impl ::bar::B for f32 { fn foo() -> f32 { 1.0 } }
   |                 ^ private trait
note: the trait `B` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:40:5
   |
LL |     trait B {
---
27 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/privacy2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/privacy2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0603]: function import `foo` is private
  --> /checkout/src/test/ui/privacy/privacy2.rs:23:20
   |
LL |     use bar::glob::foo;
   |
note: the function import `foo` is defined here...
  --> /checkout/src/test/ui/privacy/privacy2.rs:10:13
   |
---

---- [ui] ui/privacy/privacy3.rs stdout ----
diff of stderr:

4 LL |     use bar::gpriv;
5    |         ^^^^^^^^^^ no `gpriv` in `bar`
- error: requires `sized` lang_item
+ error: requires `panic_location` lang_item
8 
9 error: aborting due to 2 previous errors
9 error: aborting due to 2 previous errors
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/privacy3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/privacy3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `bar::gpriv`
   |
   |
LL |     use bar::gpriv;
   |         ^^^^^^^^^^ no `gpriv` in `bar`
error: requires `panic_location` lang_item

error: aborting due to 2 previous errors

---
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy4/privacy4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/privacy4.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: module `glob` is private
  --> /checkout/src/test/ui/privacy/privacy4.rs:21:14
   |
LL |     use bar::glob::gpriv; //~ ERROR: module `glob` is private
   |
note: the module `glob` is defined here
  --> /checkout/src/test/ui/privacy/privacy4.rs:13:5
   |
---

6    |
7    = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)
8 
- error[E0601]: `main` function not found in crate `issue_59191_replace_root_with_fn`
-    |
- LL | / #![feature(custom_inner_attributes)]
- LL | |
- LL | |
- LL | | #![issue_59191::no_main]
-    | |________________________^ consider adding a `main` function to `$DIR/issue-59191-replace-root-with-fn.rs`
+ error: requires `panic_location` lang_item
17 error: aborting due to 2 previous errors
18 

- For more information about this error, try `rustc --explain E0601`.
- For more information about this error, try `rustc --explain E0601`.
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-59191-replace-root-with-fn/issue-59191-replace-root-with-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/issue-59191-replace-root-with-fn.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-59191-replace-root-with-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-59191-replace-root-with-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-59191-replace-root-with-fn/auxiliary" "--extern" "issue_59191=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-59191-replace-root-with-fn/auxiliary/libissue_59191.so"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/required-lang-item.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/required-lang-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/required-lang-item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/required-lang-item/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/sanitize/unsupported-target.rs stdout ----
diff of stderr:

1 error: LeakSanitizer only works with the `x86_64-unknown-linux-gnu` or `x86_64-apple-darwin` target
- error: aborting due to previous error
+ error: requires `panic_location` lang_item
+ 
+ error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors
4 
5 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/unsupported-target/unsupported-target.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args sanitize/unsupported-target.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/sanitize/unsupported-target.rs" "-Zthreads=1" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/unsupported-target" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "sanitizer=leak" "--target" "i686-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/unsupported-target/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: LeakSanitizer only works with the `x86_64-unknown-linux-gnu` or `x86_64-apple-darwin` target
error: requires `panic_location` lang_item

error: aborting due to 2 previous errors

---
---- [ui] ui/static_sized_requirement.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static_sized_requirement.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static_sized_requirement" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static_sized_requirement/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 10187 passed; 33 failed; 65 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:02:12
Build completed unsuccessfully in 1:02:12
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
== clock drift check ==
  local time: Sun Jun  7 00:34:05 UTC 2020
  network time: Sun, 07 Jun 2020 00:34:05 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73079/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73079/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3490) (python)
##[section]Finishing: Finalize Job
