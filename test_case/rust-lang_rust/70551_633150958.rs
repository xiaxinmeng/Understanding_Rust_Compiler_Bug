plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 28'
Agent machine name: 'fv-az578'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6ea6d082-149d-4413-93e4-0d1670c906e9.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
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
..........i...............i......................................................................... 5200/10219
.................................................................................................... 5300/10219
.........................................................i.......................................... 5400/10219
..................................................i................................................. 5500/10219
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
..........................................................................................iiiiii.iii 8500/10219
iii.i............................................................................................... 8600/10219
.................................................................................................... 8800/10219
.................................................................................................... 8900/10219
.................................................................................................... 9000/10219
.................................................................................................... 9100/10219
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 191 tests
iiii......i..............ii.i..........i.................................i..i................i....i. 100/191
............i.i.i...iii..iiiiiiiiiiiiiiii.......................iii................ii......

 finished in 6.286
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 20 tests
iiiiiiiiiiiiiiiiiiii

 finished in 0.177
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 16.122
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:11:16
-    |
- LL |     let kind = TyKind::Bool;
-    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
- note: the lint level is defined here
-   --> $DIR/ty_tykind_usage.rs:9:8
-    |
-    |
- LL | #[deny(rustc::usage_of_ty_tykind)]
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:14:9
-    |
-    |
- LL |         TyKind::Bool => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:15:9
-    |
- LL |         TyKind::Char => (),
- LL |         TyKind::Char => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:16:9
-    |
- LL |         TyKind::Int(..) => (),
- LL |         TyKind::Int(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:17:9
-    |
- LL |         TyKind::Uint(..) => (),
- LL |         TyKind::Uint(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:18:9
-    |
- LL |         TyKind::Float(..) => (),
- LL |         TyKind::Float(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:19:9
-    |
- LL |         TyKind::Adt(..) => (),
- LL |         TyKind::Adt(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:20:9
-    |
- LL |         TyKind::Foreign(..) => (),
- LL |         TyKind::Foreign(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:21:9
-    |
- LL |         TyKind::Str => (),
- LL |         TyKind::Str => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:22:9
-    |
- LL |         TyKind::Array(..) => (),
- LL |         TyKind::Array(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:23:9
-    |
- LL |         TyKind::Slice(..) => (),
- LL |         TyKind::Slice(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:24:9
-    |
- LL |         TyKind::RawPtr(..) => (),
- LL |         TyKind::RawPtr(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:25:9
-    |
- LL |         TyKind::Ref(..) => (),
- LL |         TyKind::Ref(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:26:9
-    |
- LL |         TyKind::FnDef(..) => (),
- LL |         TyKind::FnDef(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:27:9
-    |
- LL |         TyKind::FnPtr(..) => (),
- LL |         TyKind::FnPtr(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:28:9
-    |
- LL |         TyKind::Dynamic(..) => (),
- LL |         TyKind::Dynamic(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:29:9
-    |
- LL |         TyKind::Closure(..) => (),
- LL |         TyKind::Closure(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:30:9
-    |
- LL |         TyKind::Generator(..) => (),
- LL |         TyKind::Generator(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:31:9
-    |
-    |
- LL |         TyKind::GeneratorWitness(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:32:9
-    |
- LL |         TyKind::Never => (),
- LL |         TyKind::Never => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:33:9
-    |
- LL |         TyKind::Tuple(..) => (),
- LL |         TyKind::Tuple(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:34:9
-    |
- LL |         TyKind::Projection(..) => (),
- LL |         TyKind::Projection(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:35:9
-    |
- LL |         TyKind::Opaque(..) => (),
- LL |         TyKind::Opaque(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:36:9
-    |
- LL |         TyKind::Param(..) => (),
- LL |         TyKind::Param(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:37:9
-    |
- LL |         TyKind::Bound(..) => (),
- LL |         TyKind::Bound(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:38:9
-    |
- LL |         TyKind::Placeholder(..) => (),
- LL |         TyKind::Placeholder(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:39:9
-    |
- LL |         TyKind::Infer(..) => (),
- LL |         TyKind::Infer(..) => (),
-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
+ error[E0532]: expected unit struct, unit variant or constant, found tuple variant `TyKind::Error`
170   --> $DIR/ty_tykind_usage.rs:40:9
171    |
171    |
172 LL |         TyKind::Error => (),

-    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:45:12
-   --> $DIR/ty_tykind_usage.rs:45:12
+    |         ^^^^^^^^^^^^^ did you mean `TyKind::Error( /* fields */ )`?
- LL |     if let TyKind::Int(int_ty) = kind {}
- LL |     if let TyKind::Int(int_ty) = kind {}
-    |            ^^^^^^ help: try using ty::<kind> directly: `ty`
- error: usage of `ty::TyKind`
-   --> $DIR/ty_tykind_usage.rs:47:24
+ help: consider importing one of these items instead
183    |
183    |
- LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {}
-    |                        ^^^^^^^^^^
+ LL | use rustc_middle::middle::resolve_lifetime::LifetimeDefOrigin::Error;
-    = help: try using `Ty` instead
+ LL | use std::fmt::Error;
+    |
188 
---
191 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0532]: expected unit struct, unit variant or constant, found tuple variant `TyKind::Error`
  --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:40:9
   |
LL |         TyKind::Error => (), //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^^^^^^^^ did you mean `TyKind::Error( /* fields */ )`?
help: consider importing one of these items instead
   |
   |
LL | use rustc_middle::middle::resolve_lifetime::LifetimeDefOrigin::Error;
LL | use std::fmt::Error;
   |

error: aborting due to previous error
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:07:35
Build completed unsuccessfully in 1:07:35
== clock drift check ==
  local time: Sat May 23 22:56:18 UTC 2020
  network time: Sat, 23 May 2020 22:56:18 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3540) (python)
##[section]Finishing: Finalize Job
