plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/35a4cefd-48a3-4a9a-acac-a6ed11176652.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72770/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72770/merge:refs/remotes/pull/72770/merge
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
   Compiling chalk-engine v0.11.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.11.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-ir v0.11.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-engine v0.11.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.....................i.............................................................................. 1900/10388
.................................................................................................... 2000/10388
................................................i..i................................................ 2100/10388
.................................................................................................... 2200/10388
......................................iiiii......................................................... 2300/10388
.................................................................................................... 2500/10388
.................................................................................................... 2600/10388
.................................................................................................... 2700/10388
.................................................................................................... 2800/10388
---
.................................................................................................... 5300/10388
.................................................................................................... 5400/10388
............................i....................................................................... 5500/10388
......................i............................................................................. 5600/10388
..........................................ii.ii........i...i........................................ 5700/10388
...........i........................................................................................ 5900/10388
.......i............................................................................................ 6000/10388
................................................................ii.................................. 6100/10388
...i................................................................................................ 6200/10388
...i................................................................................................ 6200/10388
.................................................................................................... 6300/10388
.................................................................................................... 6400/10388
...........................ii...i..ii...........i................................................... 6500/10388
.................................................................................................... 6700/10388
.................................................................................................... 6800/10388
.................................................................................................... 6800/10388
.............................................................i..ii.................................. 6900/10388
.................................................................................................... 7100/10388
.................................................................................................... 7200/10388
.................i.................................................................................. 7300/10388
.................................................................................................... 7400/10388
---
.................................................................................................... 8300/10388
.................................................................................................... 8400/10388
..............................................................i..................................... 8500/10388
.................................................................................................... 8600/10388
................iiiiii..iiiiii.i.................................................................... 8700/10388
.................................................................................................... 8900/10388
.................................................................................................... 9000/10388
.................................................................................................... 9100/10388
.................................................................................................... 9200/10388
---

---- [ui] ui/utf8_idents.rs stdout ----
diff of stderr:

34    = note: see issue #55467 <***/issues/55467> for more information
35    = help: add `#![feature(non_ascii_idents)]` to the crate attributes to enable
36 
+ warning: The usage of Script Group `AugmentedScriptSet {Grek}` in this crate consists solely of mixed script confusables
+    |
+ LL |     γ
+    |     ^
+    |
+    |
+    = note: `#[warn(mixed_script_confusables)]` on by default
+    = note: The usage includes '\u{3b1}'(U+03B1), '\u{3b3}'(U+03B3), '\u{3b4}'(U+03B4).
+    = note: Please recheck to make sure their usages are indeed what you want.
37 warning: type parameter `γ` should have an upper camel case name
38   --> $DIR/utf8_idents.rs:3:5
39    |


42    |
43    = note: `#[warn(non_camel_case_types)]` on by default
44 
- error: aborting due to 4 previous errors; 1 warning emitted
+ error: aborting due to 4 previous errors; 2 warnings emitted
47 For more information about this error, try `rustc --explain E0658`.
48 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/utf8_idents/utf8_idents.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args utf8_idents.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/utf8_idents.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/utf8_idents" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/utf8_idents/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: non-ascii idents are not fully supported
  --> /checkout/src/test/ui/utf8_idents.rs:2:5
   |
LL |     'β, //~ ERROR non-ascii idents are not fully supported
   |
   |
   = note: see issue #55467 <***/issues/55467> for more information
   = help: add `#![feature(non_ascii_idents)]` to the crate attributes to enable
error[E0658]: non-ascii idents are not fully supported
  --> /checkout/src/test/ui/utf8_idents.rs:3:5
   |
LL |     γ  //~ ERROR non-ascii idents are not fully supported
LL |     γ  //~ ERROR non-ascii idents are not fully supported
   |     ^
   |
   = note: see issue #55467 <***/issues/55467> for more information
   = help: add `#![feature(non_ascii_idents)]` to the crate attributes to enable
error[E0658]: non-ascii idents are not fully supported
  --> /checkout/src/test/ui/utf8_idents.rs:8:5
   |
   |
LL |     δ: usize //~ ERROR non-ascii idents are not fully supported
   |
   |
   = note: see issue #55467 <***/issues/55467> for more information
   = help: add `#![feature(non_ascii_idents)]` to the crate attributes to enable
error[E0658]: non-ascii idents are not fully supported
  --> /checkout/src/test/ui/utf8_idents.rs:12:9
   |
   |
LL |     let α = 0.00001f64; //~ ERROR non-ascii idents are not fully supported
   |
   |
   = note: see issue #55467 <***/issues/55467> for more information
   = help: add `#![feature(non_ascii_idents)]` to the crate attributes to enable

warning: The usage of Script Group `AugmentedScriptSet {Grek}` in this crate consists solely of mixed script confusables
   |
LL |     γ  //~ ERROR non-ascii idents are not fully supported
   |     ^
   |
   |
   = note: `#[warn(mixed_script_confusables)]` on by default
   = note: The usage includes '\u{3b1}'(U+03B1), '\u{3b3}'(U+03B3), '\u{3b4}'(U+03B4).
   = note: Please recheck to make sure their usages are indeed what you want.
warning: type parameter `γ` should have an upper camel case name
  --> /checkout/src/test/ui/utf8_idents.rs:3:5
   |
LL |     γ  //~ ERROR non-ascii idents are not fully supported
LL |     γ  //~ ERROR non-ascii idents are not fully supported
   |     ^ help: convert the identifier to upper camel case: `Γ`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

error: aborting due to 4 previous errors; 2 warnings emitted
For more information about this error, try `rustc --explain E0658`.

------------------------------------------

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:347:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:30
Build completed unsuccessfully in 1:05:30
== clock drift check ==
  local time: Tue Jun 23 12:56:54 UTC 2020
  network time: Tue, 23 Jun 2020 12:56:54 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72770/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72770/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3530) (python)
##[section]Finishing: Finalize Job
