plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 62'
Agent machine name: 'fv-az578'
Current agent version: '2.171.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200625.0
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200625.0/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/717ef694-4ba0-460c-835a-d21c8a1ed247.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73670/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73670/merge:refs/remotes/pull/73670/merge
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
   Compiling tracing v0.1.15
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-engine v0.14.0
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling tracing v0.1.15
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-engine v0.14.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
..............................i..................................................................... 1900/10412
.................................................................................................... 2000/10412
...............................................i..i................................................. 2100/10412
.................................................................................................... 2200/10412
.....................................iiiii.......................................................... 2300/10412
.................................................................................................... 2500/10412
.................................................................................................... 2600/10412
.................................................................................................... 2700/10412
.................................................................................................... 2800/10412
---
i................................................................................................... 5300/10412
.................................................................................................... 5400/10412
.................................i.................................................................. 5500/10412
...........................i........................................................................ 5600/10412
...............................................ii.ii........i...i................................... 5700/10412
................i................................................................................... 5900/10412
.............i...................................................................................... 6000/10412
.......................................................................ii........................... 6100/10412
..........i......................................................................................... 6200/10412
..........i......................................................................................... 6200/10412
.................................................................................................... 6300/10412
.................................................................................................... 6400/10412
...................................ii....i.ii...........i........................................... 6500/10412
.................................................................................................... 6700/10412
.................................................................................................... 6800/10412
.................................................................................................... 6800/10412
.......................................................................i..ii........................ 6900/10412
.................................................................................................... 7100/10412
.................................................................................................... 7200/10412
...........................i........................................................................ 7300/10412
.................................................................................................... 7400/10412
---
.................................................................................................... 8300/10412
.................................................................................................... 8400/10412
................................................................................i................... 8500/10412
.................................................................................................... 8600/10412
..................................iiiiii..iiiiii..i................................................. 8700/10412
.................................................................................................... 8900/10412
.................................................................................................... 9000/10412
.................................................................................................... 9100/10412
.................................................................................................... 9200/10412
---
diff of stderr:

5    |             ^^^^^^^^^^^^^^^^
6    |
7    = note: did you intend to capture a variable `foo` from the surrounding scope?
-    = note: to avoid ambiguity format_args! cannot capture variables when the format string is expanded from a macro
+    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
10 
11 error: there is no argument named `bar`

15    |             ^^^^^^^^^^^^^^^^^^^^^^^
15    |             ^^^^^^^^^^^^^^^^^^^^^^^
16    |
17    = note: did you intend to capture a variable `bar` from the surrounding scope?
-    = note: to avoid ambiguity format_args! cannot capture variables when the format string is expanded from a macro
+    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
20 
21 error: aborting due to 2 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-macro-hygiene/format-args-capture-macro-hygiene.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/format-args-capture-macro-hygiene.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-args-capture-macro-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-macro-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-macro-hygiene/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: there is no argument named `foo`
  --> /checkout/src/test/ui/fmt/format-args-capture-macro-hygiene.rs:4:13
   |
LL |     format!(concat!("{foo}"));         //~ ERROR: there is no argument named `foo`
   |
   |
   = note: did you intend to capture a variable `foo` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro

error: there is no argument named `bar`
  --> /checkout/src/test/ui/fmt/format-args-capture-macro-hygiene.rs:5:13
   |
   |
LL |     format!(concat!("{ba", "r} {}"), 1);     //~ ERROR: there is no argument named `bar`
   |
   |
   = note: did you intend to capture a variable `bar` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro

error: aborting due to 2 previous errors


---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:344:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:03:00
Build completed unsuccessfully in 1:03:00
== clock drift check ==
  local time: Wed Jul  1 11:43:19 UTC 2020
  network time: Wed, 01 Jul 2020 11:43:19 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73670/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73670/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3673) (python)
##[section]Finishing: Finalize Job
