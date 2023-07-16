plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4e7e3fb0-72ac-4d66-a02d-442464ee3f4a.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72934/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72934/merge:refs/remotes/pull/72934/merge
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
.................................................................................................... 5300/10285
............................................................................i....................... 5400/10285
......................................................................i............................. 5500/10285
.......................................................................................ii.ii........ 5600/10285
i...i............................................................................................... 5700/10285
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
.................................................................................................... 7100/10285
.........................................i.......................................................... 7200/10285
.................................................................................................... 7300/10285
---
.................................................................................................... 8200/10285
.................................................................................................... 8300/10285
................................................................................i................... 8400/10285
.................................................................................................... 8500/10285
..................................iiiiii.iiiiii.i................................................... 8600/10285
.................................................................................................... 8800/10285
.................................................................................................... 8900/10285
.................................................................................................... 9000/10285
.................................................................................................... 9100/10285
---
---- [ui] ui/consts/const-mut-refs/const_mut_address_of.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-mut-refs/const_mut_address_of.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/const_mut_address_of" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/const_mut_address_of/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: references in constants may only refer to immutable values
  --> /checkout/src/test/ui/consts/const-mut-refs/const_mut_address_of.rs:26:5
   |
LL |     foo().bar();
   |     ^^^^^ constants require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error[E0658]: references in constants may only refer to immutable values
  --> /checkout/src/test/ui/consts/const-mut-refs/const_mut_address_of.rs:27:9
   |
LL |     baz(&mut foo());
LL |     baz(&mut foo());
   |         ^^^^^^^^^^ constants require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/consts/const-mut-refs/const_mut_refs.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-mut-refs/const_mut_refs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/const_mut_refs/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/const_mut_refs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: references in constants may only refer to immutable values
  --> /checkout/src/test/ui/consts/const-mut-refs/const_mut_refs.rs:33:17
   |
LL |     let _: [(); foo().bar()] = [(); 1];
   |                 ^^^^^ constants require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error[E0658]: references in constants may only refer to immutable values
  --> /checkout/src/test/ui/consts/const-mut-refs/const_mut_refs.rs:34:21
   |
   |
LL |     let _: [(); baz(&mut foo())] = [(); 2];
   |                     ^^^^^^^^^^ constants require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error[E0658]: references in constants may only refer to immutable values
  --> /checkout/src/test/ui/consts/const-mut-refs/const_mut_refs.rs:35:22
   |
   |
LL |     let _: [(); bazz(&mut foo())] = [(); 3];
   |                      ^^^^^^^^^^ constants require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.

---

+ error[E0658]: references in constants may only refer to immutable values
+   --> $DIR/projection_qualif.rs:10:27
+    |
+ LL |         let b: *mut u32 = &mut a;
+    |                           ^^^^^^ constants require immutable values
+    |
+    = note: see issue #57349 <***/issues/57349> for more information
+    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
1 error[E0658]: dereferencing raw pointers in constants is unstable
2   --> $DIR/projection_qualif.rs:11:18
3    |


7    = note: see issue #51911 <***/issues/51911> for more information
9 
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
11 
11 
12 For more information about this error, try `rustc --explain E0658`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/projection_qualif.mut_refs/projection_qualif.mut_refs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/projection_qualif.rs`

error in revision `mut_refs`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/projection_qualif.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mut_refs" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/projection_qualif.mut_refs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/projection_qualif.mut_refs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: references in constants may only refer to immutable values
  --> /checkout/src/test/ui/consts/projection_qualif.rs:10:27
   |
LL |         let b: *mut u32 = &mut a; //[stock]~ ERROR may only refer to immutable values
   |                           ^^^^^^ constants require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error[E0658]: dereferencing raw pointers in constants is unstable
  --> /checkout/src/test/ui/consts/projection_qualif.rs:11:18
   |
   |
LL |         unsafe { *b = 5; } //~ ERROR dereferencing raw pointers in constants
   |
   |
   = note: see issue #51911 <***/issues/51911> for more information

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
---
---- [ui] ui/consts/read_from_static_mut_ref.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/read_from_static_mut_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/read_from_static_mut_ref/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/read_from_static_mut_ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: references in statics may only refer to immutable values
  --> /checkout/src/test/ui/consts/read_from_static_mut_ref.rs:5:27
   |
LL | static OH_YES: &mut i32 = &mut 42;
   |                           ^^^^^^^ statics require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

---
-   --> $DIR/static_mut_containing_mut_ref2.rs:7:45
+ error[E0658]: references in statics may only refer to immutable values
+   --> $DIR/static_mut_containing_mut_ref2.rs:7:46
3    |
4 LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
-    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ modifying a static's initial value from another static's initializer
+    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ statics require immutable values
+    |
+    = note: see issue #57349 <***/issues/57349> for more information
+    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0080`.
- For more information about this error, try `rustc --explain E0080`.
+ For more information about this error, try `rustc --explain E0658`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2.mut_refs/static_mut_containing_mut_ref2.mut_refs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/static_mut_containing_mut_ref2.rs`

error in revision `mut_refs`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mut_refs" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2.mut_refs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2.mut_refs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: references in statics may only refer to immutable values
  --> /checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs:7:46
   |
LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ statics require immutable values
   |
   = note: see issue #57349 <***/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:52:07
Build completed unsuccessfully in 0:52:07
== clock drift check ==
  local time: Sun Jun  7 01:22:31 UTC 2020
  network time: Sun, 07 Jun 2020 01:22:31 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72934/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72934/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3815) (python)
##[section]Finishing: Finalize Job
