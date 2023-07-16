plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 7'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/45c34dba-9c3d-4a49-93bd-561f7e03ca5e.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72637/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72637/merge:refs/remotes/pull/72637/merge
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
..............................................................................i..................... 1800/10237
.................................................................................................... 1900/10237
.................................................................................................i.. 2000/10237
i................................................................................................... 2100/10237
.......................................................................................iiiii........ 2200/10237
.................................................................................................... 2400/10237
.................................................................................................... 2500/10237
.................................................................................................... 2600/10237
.................................................................................................... 2600/10237
....................................................................F.FF.FF......................... 2700/10237
.................................................................................................... 2900/10237
.................................................................................................... 3000/10237
.................................................................................................... 3100/10237
.................................................................................................... 3200/10237
---
...............i...............i.................................................................... 5200/10237
.................................................................................................... 5300/10237
..............................................................i..................................... 5400/10237
.......................................................i............................................ 5500/10237
..................................................................ii.ii........i...i................ 5600/10237
........i........................................................................................... 5800/10237
................i................................................................................... 5900/10237
....................................................................ii.............................. 6000/10237
.......i...............................................................................F............ 6100/10237
.......i...............................................................................F............ 6100/10237
.................................................................................................... 6200/10237
.................................................................................................... 6300/10237
.............................ii...i..ii...........i................................................. 6400/10237
.................................................................................................... 6600/10237
.................................................................................................... 6700/10237
.................................................................................................... 6700/10237
..............................................................i..ii................................. 6800/10237
.................................................................................................... 7000/10237
.................................................................................................... 7100/10237
................i................................................................................... 7200/10237
.................................................................................................... 7300/10237
---
.................................................................................................... 8200/10237
.................................................................................................... 8300/10237
...................................................i................................................ 8400/10237
.................................................................................................... 8500/10237
.....iiiiii.iiiiii.i................................................................................ 8600/10237
.................................................................................................... 8800/10237
.................................................................................................... 8900/10237
.................................................................................................... 9000/10237
.................................................................................................... 9100/10237
---
---- [ui] ui/extenv/extenv-no-args.rs stdout ----
diff of stderr:

3    |
4 LL | fn main() { env!(); }
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-no-args/extenv-no-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extenv/extenv-no-args.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extenv/extenv-no-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-no-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-no-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: env! takes 1 or 2 arguments
   |
   |
LL | fn main() { env!(); } //~ ERROR: env! takes 1 or 2 arguments
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
---- [ui] ui/extenv/extenv-not-defined-custom.rs stdout ----
diff of stderr:

3    |
4 LL | fn main() { env!("__HOPEFULLY_NOT_DEFINED__", "my error message"); }
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-not-defined-custom/extenv-not-defined-custom.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extenv/extenv-not-defined-custom.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extenv/extenv-not-defined-custom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-not-defined-custom" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-not-defined-custom/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: my error message
  --> /checkout/src/test/ui/extenv/extenv-not-defined-custom.rs:1:13
   |
LL | fn main() { env!("__HOPEFULLY_NOT_DEFINED__", "my error message"); } //~ ERROR: my error message
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
---- [ui] ui/extenv/extenv-not-defined-default.rs stdout ----
diff of stderr:

3    |
4 LL |     env!("__HOPEFULLY_NOT_DEFINED__");
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-not-defined-default/extenv-not-defined-default.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extenv/extenv-not-defined-default.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extenv/extenv-not-defined-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-not-defined-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-not-defined-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: environment variable `__HOPEFULLY_NOT_DEFINED__` not defined
   |
   |
LL |     env!("__HOPEFULLY_NOT_DEFINED__");
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
---- [ui] ui/extenv/extenv-too-many-args.rs stdout ----
diff of stderr:

3    |
4 LL | fn main() { env!("one", "two", "three"); }
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-too-many-args/extenv-too-many-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extenv/extenv-too-many-args.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extenv/extenv-too-many-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-too-many-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/extenv-too-many-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: env! takes 1 or 2 arguments
   |
   |
LL | fn main() { env!("one", "two", "three"); } //~ ERROR: env! takes 1 or 2 arguments
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
---- [ui] ui/extenv/issue-55897.rs stdout ----
diff of stderr:

3    |
4 LL |     include!(concat!(env!("NON_EXISTENT"), "/data.rs"));
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: suffixes on a string literal are invalid
7 error: suffixes on a string literal are invalid
8   --> $DIR/issue-55897.rs:16:22


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/issue-55897/issue-55897.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extenv/issue-55897.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extenv/issue-55897.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/issue-55897" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/issue-55897/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: environment variable `NON_EXISTENT` not defined
  --> /checkout/src/test/ui/extenv/issue-55897.rs:11:22
   |
LL |     include!(concat!(env!("NON_EXISTENT"), "/data.rs"));
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: suffixes on a string literal are invalid
error: suffixes on a string literal are invalid
  --> /checkout/src/test/ui/extenv/issue-55897.rs:16:22
   |
LL |     include!(concat!("NON_EXISTENT"suffix, "/data.rs"));
   |                      ^^^^^^^^^^^^^^^^^^^^ invalid suffix `suffix`
error[E0432]: unresolved import `prelude`
  --> /checkout/src/test/ui/extenv/issue-55897.rs:1:5
   |
LL | use prelude::*; //~ ERROR unresolved import `prelude`
---

error[E0432]: unresolved import `env`
  --> /checkout/src/test/ui/extenv/issue-55897.rs:4:9
   |
LL |     use env; //~ ERROR unresolved import `env`
   |         ^^^ no `env` in the root
error: cannot determine resolution for the macro `env`
  --> /checkout/src/test/ui/extenv/issue-55897.rs:6:22
   |
   |
LL |     include!(concat!(env!("NON_EXISTENT"), "/data.rs"));
   |
   = note: import resolution is stuck, try simplifying macro imports

error: aborting due to 5 previous errors
---
---- [ui] ui/macros/macros-nonfatal-errors.rs stdout ----
diff of stderr:

47    |
48 LL |     env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST");
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
50 
51 error: format argument must be a string literal
51 error: format argument must be a string literal
52   --> $DIR/macros-nonfatal-errors.rs:23:13


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/macros-nonfatal-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macros-nonfatal-errors.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macros-nonfatal-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: asm template must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:13:10
   |
LL |     asm!(invalid); //~ ERROR

error: inline assembly must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:14:15
   |
   |
LL |     llvm_asm!(invalid); //~ ERROR


error: concat_idents! requires ident args.
   |
   |
LL |     concat_idents!("not", "idents"); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:18:17
   |
   |
LL |     option_env!(invalid); //~ ERROR

error: expected string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:19:10
   |
   |
LL |     env!(invalid); //~ ERROR

error: expected string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:20:10
   |
   |
LL |     env!(foo, abr, baz); //~ ERROR


error: environment variable `RUST_HOPEFULLY_THIS_DOESNT_EXIST` not defined
   |
   |
LL |     env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST"); //~ ERROR
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: format argument must be a string literal
error: format argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:23:13
   |
LL |     format!(invalid); //~ ERROR
   |
help: you might be missing a string literal to format with
   |
LL |     format!("{}", invalid); //~ ERROR
LL |     format!("{}", invalid); //~ ERROR
   |             ^^^^^

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:25:14
   |
LL |     include!(invalid); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:27:18
   |
   |
LL |     include_str!(invalid); //~ ERROR
   |                  ^^^^^^^

error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
   |
   |
LL |     include_str!("i'd be quite surprised if a file with this name existed"); //~ ERROR
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: argument must be a string literal
error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:29:20
   |
LL |     include_bytes!(invalid); //~ ERROR
   |                    ^^^^^^^

error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
   |
   |
LL |     include_bytes!("i'd be quite surprised if a file with this name existed"); //~ ERROR
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: trace_macros! accepts only `true` or `false`
error: trace_macros! accepts only `true` or `false`
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:32:5
   |
LL |     trace_macros!(invalid); //~ ERROR

error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0665`.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:02:43
Build completed unsuccessfully in 1:02:43
== clock drift check ==
  local time: Wed May 27 00:55:08 UTC 2020
  network time: Wed, 27 May 2020 00:55:08 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72637/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72637/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4465) (python)
##[section]Finishing: Finalize Job
