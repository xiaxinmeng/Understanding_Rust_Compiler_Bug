plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200512.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200512.2/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/68f35356-0716-4941-9040-e95420002930.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72342/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72342/merge:refs/remotes/pull/72342/merge
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
   Compiling chalk-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.........................................................................i.......................... 1800/10208
.................................................................................................... 1900/10208
............................................................................................i..i.... 2000/10208
.................................................................................................... 2100/10208
..................................................................................iiiii............. 2200/10208
.................................................................................................... 2400/10208
.................................................................................................... 2500/10208
.................................................................................................... 2600/10208
.................................................................................................... 2700/10208
---
...............i.................................................................................... 5200/10208
.................................................................................................... 5300/10208
.............................................i...................................................... 5400/10208
......................................i............................................................. 5500/10208
...............................................ii.ii........i...i................................... 5600/10208
.................................................................................................i.. 5800/10208
.................................................................................................... 5900/10208
.................................................ii.....................................i........... 6000/10208
.................................................................................................... 6100/10208
.................................................................................................... 6100/10208
.................................................................................................... 6200/10208
.................................................................................................... 6300/10208
..........ii...i..ii...........i.................................................................... 6400/10208
.................................................................................................... 6600/10208
.................................................................................................... 6700/10208
.................................................................................................... 6700/10208
...........................................i..ii.................................................... 6800/10208
.................................................................................................... 7000/10208
.................................................................................................i.. 7100/10208
.................................................................................................... 7200/10208
.................................................................................................... 7300/10208
---
.................................................................................................... 8100/10208
.................................................................................................... 8200/10208
.................................................................................................... 8300/10208
....................i............................................................................... 8400/10208
..........................................................................iiiiii.iiiiii.i........... 8500/10208
.............................i...................................................................... 8700/10208
.................................................................................................... 8800/10208
.................................................................................................... 8900/10208
.................................................................................................... 9000/10208
---
...i................................................................................................ 9700/10208
.................................................................................................... 9800/10208
.................................................................................................... 9900/10208
.................................................................................................... 10000/10208
.........FF.FFFF.................................................................................... 10100/10208
........
failures:

---- [ui] ui/unused-crate-deps/unused-aliases-dylib.rs stdout ----
---- [ui] ui/unused-crate-deps/unused-aliases-dylib.rs stdout ----
diff of stderr:

1 warning: External crate `barbar` unused in `unused_aliases_dylib`. Remove the dependency or add `use barbar as _;`.
-   --> $DIR/unused-aliases-dylib.rs:8:1
+   --> $DIR/unused-aliases-dylib.rs:9:1
4 LL | #![warn(unused_crate_deps)]
5    | ^

6    |
---
11    |         ^^^^^^^^^^^^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib/unused-aliases-dylib.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/unused-aliases-dylib.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/unused-aliases-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib/auxiliary/libbar.so" "--extern" "barbar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: External crate `barbar` unused in `unused_aliases_dylib`. Remove the dependency or add `use barbar as _;`.
  --> /checkout/src/test/ui/unused-crate-deps/unused-aliases-dylib.rs:9:1
LL | #![warn(unused_crate_deps)]
   | ^
   |
note: the lint level is defined here
---

---- [ui] ui/unused-crate-deps/unused-aliases.rs stdout ----
diff of stderr:

1 warning: External crate `barbar` unused in `unused_aliases`. Remove the dependency or add `use barbar as _;`.
+   --> $DIR/unused-aliases.rs:10:1
3    |
4 LL | #![warn(unused_crate_deps)]
5    | ^
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases/unused-aliases.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/unused-aliases.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/unused-aliases.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases/auxiliary/libbar.so" "--extern" "barbar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: External crate `barbar` unused in `unused_aliases`. Remove the dependency or add `use barbar as _;`.
   |
LL | #![warn(unused_crate_deps)]
   | ^
   |
---

---- [ui] ui/unused-crate-deps/warn-attr.rs stdout ----
diff of stderr:

1 warning: External crate `bar` unused in `warn_attr`. Remove the dependency or add `use bar as _;`.
-   --> $DIR/warn-attr.rs:8:1
+   --> $DIR/warn-attr.rs:9:1
4 LL | #![warn(unused_crate_deps)]
5    | ^

6    |
---
11    |         ^^^^^^^^^^^^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr/warn-attr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/warn-attr.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/warn-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: External crate `bar` unused in `warn_attr`. Remove the dependency or add `use bar as _;`.
  --> /checkout/src/test/ui/unused-crate-deps/warn-attr.rs:9:1
LL | #![warn(unused_crate_deps)]
   | ^
   |
note: the lint level is defined here
---

---- [ui] ui/unused-crate-deps/warn-attr-dylib.rs stdout ----
diff of stderr:

1 warning: External crate `bar` unused in `warn_attr_dylib`. Remove the dependency or add `use bar as _;`.
-   --> $DIR/warn-attr-dylib.rs:7:1
+   --> $DIR/warn-attr-dylib.rs:8:1
4 LL | #![warn(unused_crate_deps)]
5    | ^

6    |
---
11    |         ^^^^^^^^^^^^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr-dylib/warn-attr-dylib.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/warn-attr-dylib.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/warn-attr-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr-dylib" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr-dylib/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr-dylib/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: External crate `bar` unused in `warn_attr_dylib`. Remove the dependency or add `use bar as _;`.
  --> /checkout/src/test/ui/unused-crate-deps/warn-attr-dylib.rs:8:1
LL | #![warn(unused_crate_deps)]
   | ^
   |
note: the lint level is defined here
---

---- [ui] ui/unused-crate-deps/warn-cmdline.rs stdout ----
diff of stderr:

1 warning: External crate `bar` unused in `warn_cmdline`. Remove the dependency or add `use bar as _;`.
-   --> $DIR/warn-cmdline.rs:9:1
+   --> $DIR/warn-cmdline.rs:10:1
4 LL | fn main() {}
5    | ^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline/warn-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/warn-cmdline.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/warn-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Wunused-crate-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: External crate `bar` unused in `warn_cmdline`. Remove the dependency or add `use bar as _;`.
  --> /checkout/src/test/ui/unused-crate-deps/warn-cmdline.rs:10:1
LL | fn main() {}
   | ^
   |
   |
   = note: requested on the command line with `-W unused-crate-deps`
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/unused-crate-deps/warn-cmdline-dylib.rs stdout ----
diff of stderr:

1 warning: External crate `bar` unused in `warn_cmdline_dylib`. Remove the dependency or add `use bar as _;`.
-   --> $DIR/warn-cmdline-dylib.rs:8:1
+   --> $DIR/warn-cmdline-dylib.rs:9:1
4 LL | fn main() {}
5    | ^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline-dylib/warn-cmdline-dylib.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/warn-cmdline-dylib.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/warn-cmdline-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline-dylib" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Wunused-crate-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline-dylib/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline-dylib/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: External crate `bar` unused in `warn_cmdline_dylib`. Remove the dependency or add `use bar as _;`.
  --> /checkout/src/test/ui/unused-crate-deps/warn-cmdline-dylib.rs:9:1
LL | fn main() {}
   | ^
   |
   |
   = note: requested on the command line with `-W unused-crate-deps`
warning: 1 warning emitted


------------------------------------------
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:08:02
Build completed unsuccessfully in 1:08:02
== clock drift check ==
  local time: Wed May 20 06:21:24 UTC 2020
  network time: Wed, 20 May 2020 06:21:24 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72342/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72342/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3820) (python)
##[section]Finishing: Finalize Job
