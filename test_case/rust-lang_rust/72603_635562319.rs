plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 19'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0f30e412-6c99-440a-aecb-1facb41efe79.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72603/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72603/merge:refs/remotes/pull/72603/merge
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
..............................................................................i..................... 1800/10270
.................................................................................................... 1900/10270
.................................................................................................i.. 2000/10270
i................................................................................................... 2100/10270
.......................................................................................iiiii........ 2200/10270
.................................................................................................... 2400/10270
.................................................................................................... 2500/10270
.................................................................................................... 2600/10270
.................................................................................................... 2700/10270
---
...............i...............i.................................................................... 5200/10270
.................................................................................................... 5300/10270
...............................................................i.................................... 5400/10270
........................................................i........................................... 5500/10270
....................................................................ii.ii........i...i.............. 5600/10270
...........i........................................................................................ 5800/10270
...................i................................................................................ 5900/10270
........................................................................ii.......................... 6000/10270
...........i........................................................................................ 6100/10270
...........i........................................................................................ 6100/10270
.................................................................................................... 6200/10270
.................................................................................................... 6300/10270
.................................ii...i..ii...........i............................................. 6400/10270
.................................................................................................... 6600/10270
.................................................................................................... 6700/10270
.................................................................................................... 6700/10270
..................................................................i..ii............................. 6800/10270
.................................................................................................... 7000/10270
.................................................................................................... 7100/10270
....................i............................................................................... 7200/10270
.................................................................................................... 7300/10270
---
.................................................................................................... 8200/10270
.................................................................................................... 8300/10270
.......................................................i............................................ 8400/10270
.................................................................................................... 8500/10270
.........iiiiii.iiiiii.i............................................................................ 8600/10270
.................................................................................................... 8800/10270
.................................................................................................... 8900/10270
.................................................................................................... 9000/10270
.................................................................................................... 9100/10270
---

---- [ui] ui/unused-crate-deps/extern-loc-json-bad-json.rs stdout ----
diff of stderr:

- error: `--extern-location`: malformed json location `[{"malformed`
+ error: unknown location type `json`: use `file`, `span` or `raw`
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-bad-json/extern-loc-json-bad-json.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-json-bad-json.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-json-bad-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-bad-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=json:[{\"malformed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-bad-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-bad-json/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown location type `json`: use `file`, `span` or `raw`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-json-json.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-json-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=json:{\"key\":123,\"value\":{}}" "--error-format" "json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown location type `json`: use `file`, `span` or `raw`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-json.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=json:{\"key\":123,\"value\":{}}" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown location type `json`: use `file`, `span` or `raw`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-raw-json.rs stdout ----
diff of stderr:

- {"message":"external crate `bar` unused in `extern_loc_raw_json`: remove the dependency or add `use bar as _;`","code":{"code":"unused_crate_dependencies","explanation":null},"level":"warning","spans":[{"file_name":"$DIR/extern-loc-raw-json.rs","byte_start":162,"byte_end":162,"line_start":7,"line_end":7,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![warn(unused_crate_dependencies)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"$DIR/extern-loc-raw-json.rs","byte_start":170,"byte_end":195,"line_start":7,"line_end":7,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"#![warn(unused_crate_dependencies)]","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null,"tool_metadata":null},{"message":"remove unnecessary dependency `bar` at `in-the-test-file`","code":null,"level":"help","spans":[],"children":[],"rendered":null,"tool_metadata":null},{"message":"raw extern location","code":null,"level":"help","spans":[{"file_name":"$DIR/extern-loc-raw-json.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[],"label":null,"suggested_replacement":"in-the-test-file","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null,"tool_metadata":null},{"message":"json extern location","code":null,"level":"help","spans":[],"children":[],"rendered":null,"tool_metadata":"in-the-test-file"}],"rendered":"warning: external crate `bar` unused in `extern_loc_raw_json`: remove the dependency or add `use bar as _;`
+ {"message":"external crate `bar` unused in `extern_loc_raw_json`: remove the dependency or add `use bar as _;`","code":{"code":"unused_crate_dependencies","explanation":null},"level":"warning","spans":[{"file_name":"$DIR/extern-loc-raw-json.rs","byte_start":162,"byte_end":162,"line_start":7,"line_end":7,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![warn(unused_crate_dependencies)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"$DIR/extern-loc-raw-json.rs","byte_start":170,"byte_end":195,"line_start":7,"line_end":7,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"#![warn(unused_crate_dependencies)]","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove unnecessary dependency `bar` at `in-the-test-file`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"raw extern location","code":null,"level":"help","spans":[{"file_name":"$DIR/extern-loc-raw-json.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[],"label":null,"suggested_replacement":"in-the-test-file","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: external crate `bar` unused in `extern_loc_raw_json`: remove the dependency or add `use bar as _;`
2   --> $DIR/extern-loc-raw-json.rs:7:1
4 LL | #![warn(unused_crate_dependencies)]

11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
12    = help: remove unnecessary dependency `bar` at `in-the-test-file`
13 
- ","tool_metadata":null}
15 {"message":"1 warning emitted","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: 1 warning emitted
16 
16 
- ","tool_metadata":null}
18 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-json/extern-loc-raw-json.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-raw-json.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-raw-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=raw:in-the-test-file" "--error-format" "json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-json/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: external crate `bar` unused in `extern_loc_raw_json`: remove the dependency or add `use bar as _;`
  --> /checkout/src/test/ui/unused-crate-deps/extern-loc-raw-json.rs:7:1
LL | #![warn(unused_crate_dependencies)]
   | ^
   |
note: the lint level is defined here
note: the lint level is defined here
  --> /checkout/src/test/ui/unused-crate-deps/extern-loc-raw-json.rs:7:9
   |
LL | #![warn(unused_crate_dependencies)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: remove unnecessary dependency `bar` at `in-the-test-file`
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/unused-crate-deps/extern-loc-raw-missing-loc.rs stdout ----
diff of stderr:

- error: `--extern-location`: missing `raw` location
+ error: `--extern-location`: ill-formed `raw` specification
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-missing-loc/extern-loc-raw-missing-loc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-raw-missing-loc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-raw-missing-loc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-missing-loc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=raw" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-missing-loc/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-missing-loc/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location`: ill-formed `raw` specification

------------------------------------------


---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:45
Build completed unsuccessfully in 1:05:45
== clock drift check ==
  local time: Thu May 28 19:55:14 UTC 2020
  network time: Thu, 28 May 2020 19:55:14 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72603/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72603/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3673) (python)
##[section]Finishing: Finalize Job
