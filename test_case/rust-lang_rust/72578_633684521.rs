plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 25'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7d5bf175-35dd-4a87-a490-a0cb61abe795.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72578/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72578/merge:refs/remotes/pull/72578/merge
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
...........................................................................i........................ 1800/10223
.................................................................................................... 1900/10223
..............................................................................................i..i.. 2000/10223
.................................................................................................... 2100/10223
....................................................................................iiiii........... 2200/10223
.................................................................................................... 2400/10223
.................................................................................................... 2500/10223
.................................................................................................... 2600/10223
.................................................................................................... 2700/10223
---
...........i...............i........................................................................ 5200/10223
.................................................................................................... 5300/10223
..........................................................i......................................... 5400/10223
...................................................i................................................ 5500/10223
..............................................................ii.ii........i...i.................... 5600/10223
....i............................................................................................... 5800/10223
............i....................................................................................... 5900/10223
................................................................ii.................................. 6000/10223
...i................................................................................................ 6100/10223
...i................................................................................................ 6100/10223
.................................................................................................... 6200/10223
.................................................................................................... 6300/10223
.........................ii...i..ii...........i..................................................... 6400/10223
.................................................................................................... 6600/10223
.................................................................................................... 6700/10223
.................................................................................................... 6700/10223
..........................................................i..ii..................................... 6800/10223
.................................................................................................... 7000/10223
.................................................................................................... 7100/10223
............i....................................................................................... 7200/10223
.................................................................................................... 7300/10223
---
.................................................................................................... 8100/10223
.................................................................................................... 8200/10223
.................................................................................................... 8300/10223
.......................................i............................................................ 8400/10223
.............................................................................................iiiiii. 8500/10223
iiiiii.i............................................................................................ 8600/10223
.................................................................................................... 8800/10223
.................................................................................................... 8900/10223
.................................................................................................... 9000/10223
.................................................................................................... 9100/10223
---
diff of stderr:

11   --> $DIR/types-mismatch-const-args.rs:13:41
12    |
13 LL |     let _: A<'a, u32, {2u32}, {3u32}> = A::<'a, u32, {4u32}, {3u32}> { data: PhantomData };
+    |            --------------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `2u32`, found `4u32`
+    |            |
+    |            expected due to this
15    |
15    |
-    = note: expected type `2u32`
-               found type `4u32`
+    = note: expected struct `A<'_, _, 2u32, _>`
+               found struct `A<'_, _, 4u32, _>`
19 error[E0308]: mismatched types
20   --> $DIR/types-mismatch-const-args.rs:15:41

24    |            |
24    |            |
25    |            expected due to this
26    |
-    = note: expected struct `A<'a, u16, {2u32}, {3u32}>`
-               found struct `A<'b, u32, {2u32}, {3u32}>`
+    = note: expected struct `A<'a, u16, _, _>`
+               found struct `A<'b, u32, _, _>`
29 
30 error: aborting due to 2 previous errors; 1 warning emitted


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/types-mismatch-const-args/types-mismatch-const-args.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/types-mismatch-const-args/types-mismatch-const-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/types-mismatch-const-args.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/types-mismatch-const-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/types-mismatch-const-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/types-mismatch-const-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `const_generics` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/const-generics/types-mismatch-const-args.rs:1:12
   |
LL | #![feature(const_generics)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44580 <***/issues/44580> for more information
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/types-mismatch-const-args.rs:13:41
   |
   |
LL |     let _: A<'a, u32, {2u32}, {3u32}> = A::<'a, u32, {4u32}, {3u32}> { data: PhantomData };
   |            |
   |            expected due to this
   |
   |
   = note: expected struct `A<'_, _, 2u32, _>`
              found struct `A<'_, _, 4u32, _>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/types-mismatch-const-args.rs:15:41
   |
   |
LL |     let _: A<'a, u16, {2u32}, {3u32}> = A::<'b, u32, {2u32}, {3u32}> { data: PhantomData };
   |            |
   |            expected due to this
   |
   |
   = note: expected struct `A<'a, u16, _, _>`
              found struct `A<'b, u32, _, _>`

error: aborting due to 2 previous errors; 1 warning emitted
For more information about this error, try `rustc --explain E0308`.

------------------------------------------



---- [ui] ui/type-alias-impl-trait/generic_nondefining_use.rs stdout ----
diff of stderr:

25 LL | fn concrete_const() -> OneConst<{123}> {
27    |
- note: used non-generic constant `{123}` for generic parameter
+ note: used non-generic constant `123usize` for generic parameter
29   --> $DIR/generic_nondefining_use.rs:10:21
29   --> $DIR/generic_nondefining_use.rs:10:21
30    |
31 LL | type OneConst<const X: usize> = impl Debug;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use/generic_nondefining_use.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use/generic_nondefining_use.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_nondefining_use.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:14:21
   |
LL | fn concrete_ty() -> OneTy<u32> {
   |
note: used non-generic type `u32` for generic parameter
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:8:12
   |
   |
LL | type OneTy<T> = impl Debug;
   |            ^

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:19:27
   |
LL | type OneLifetime<'a> = impl Debug;
   |                  -- cannot use static lifetime; use a bound lifetime instead or remove the lifetime parameter from the opaque type
...
LL | fn concrete_lifetime() -> OneLifetime<'static> {

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:24:24
   |
   |
LL | fn concrete_const() -> OneConst<{123}> {
   |
note: used non-generic constant `123usize` for generic parameter
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:10:21
   |
   |
LL | type OneConst<const X: usize> = impl Debug;

error: aborting due to 3 previous errors


---
test result: FAILED. 10158 passed; 2 failed; 63 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:54:25
Build completed unsuccessfully in 0:54:25
== clock drift check ==
  local time: Mon May 25 18:54:45 UTC 2020
  network time: Mon, 25 May 2020 18:54:46 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72578/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72578/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4507) (python)
##[section]Finishing: Finalize Job
