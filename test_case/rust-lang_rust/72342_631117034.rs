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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5ed0cf94-5440-4e04-9e93-0463634fd737.sh

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
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
...................................................................i................................ 1800/10197
.................................................................................................... 1900/10197
......................................................................................i..i.......... 2000/10197
.................................................................................................... 2100/10197
............................................................................iiiii................... 2200/10197
.................................................................................................... 2400/10197
.................................................................................................... 2500/10197
.................................................................................................... 2600/10197
.................................................................................................... 2700/10197
---
........i........................................................................................... 5200/10197
.................................................................................................... 5300/10197
.......................................i............................................................ 5400/10197
................................i................................................................... 5500/10197
.........................................ii.ii........i...i......................................... 5600/10197
...........................................................................................i........ 5800/10197
.................................................................................................... 5900/10197
......................................ii.....................................i...................... 6000/10197
.................................................................................................... 6100/10197
.................................................................................................... 6100/10197
.................................................................................................... 6200/10197
...................................................................................................i 6300/10197
i...i..ii...........i............................................................................... 6400/10197
.................................................................................................... 6600/10197
.................................................................................................... 6700/10197
.................................................................................................... 6700/10197
................................i..ii............................................................... 6800/10197
.................................................................................................... 7000/10197
......................................................................................i............. 7100/10197
.................................................................................................... 7200/10197
.................................................................................................... 7300/10197
---
.................................................................................................... 8100/10197
.................................................................................................... 8200/10197
.................................................................................................... 8300/10197
.........i.......................................................................................... 8400/10197
...............................................................iiiiii.iiii.iii...................... 8500/10197
..................i................................................................................. 8700/10197
.................................................................................................... 8800/10197
.................................................................................................... 8900/10197
.................................................................................................... 9000/10197
---
failures:

---- [ui] ui/unused-crate-deps/unused-aliases-dylib.rs stdout ----

error: /checkout/src/test/ui/unused-crate-deps/unused-aliases-dylib.rs:9: unexpected warning: '9:1: 13:13: external crate `barbar` unused [unused_crate_deps]'

error: /checkout/src/test/ui/unused-crate-deps/unused-aliases-dylib.rs:1: expected message not found: external crate `barbar` unused
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/unused-aliases-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib/auxiliary/libbar.so" "--extern" "barbar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases-dylib/auxiliary/libbar.so"
    Error {
        line_num: 9,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "9:1: 13:13: external crate `barbar` unused [unused_crate_deps]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 1,
        kind: None,
        msg: "external crate `barbar` unused",
]

thread '[ui] ui/unused-crate-deps/unused-aliases-dylib.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/unused-crate-deps/unused-aliases.rs stdout ----

error: /checkout/src/test/ui/unused-crate-deps/unused-aliases.rs:10: unexpected warning: '10:1: 14:13: external crate `barbar` unused [unused_crate_deps]'

error: /checkout/src/test/ui/unused-crate-deps/unused-aliases.rs:1: expected message not found: external crate `barbar` unused
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/unused-aliases.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases/auxiliary/libbar.so" "--extern" "barbar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/unused-aliases/auxiliary/libbar.so"
    Error {
        line_num: 10,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "10:1: 14:13: external crate `barbar` unused [unused_crate_deps]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 1,
        kind: None,
        msg: "external crate `barbar` unused",
]

thread '[ui] ui/unused-crate-deps/unused-aliases.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13


---- [ui] ui/unused-crate-deps/warn-attr-dylib.rs stdout ----

error: /checkout/src/test/ui/unused-crate-deps/warn-attr-dylib.rs:8: unexpected warning: '8:1: 10:13: external crate `bar` unused [unused_crate_deps]'

error: /checkout/src/test/ui/unused-crate-deps/warn-attr-dylib.rs:1: expected message not found: external crate `bar` unused
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/warn-attr-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr-dylib" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr-dylib/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr-dylib/auxiliary/libbar.so"
    Error {
        line_num: 8,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "8:1: 10:13: external crate `bar` unused [unused_crate_deps]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 1,
        kind: None,
        msg: "external crate `bar` unused",
]

thread '[ui] ui/unused-crate-deps/warn-attr-dylib.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13


---- [ui] ui/unused-crate-deps/warn-attr.rs stdout ----

error: /checkout/src/test/ui/unused-crate-deps/warn-attr.rs:9: unexpected warning: '9:1: 11:13: external crate `bar` unused [unused_crate_deps]'

error: /checkout/src/test/ui/unused-crate-deps/warn-attr.rs:1: expected message not found: external crate `bar` unused
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/warn-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-attr/auxiliary/libbar.so"
    Error {
        line_num: 9,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "9:1: 11:13: external crate `bar` unused [unused_crate_deps]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 1,
        kind: None,
        msg: "external crate `bar` unused",
]

thread '[ui] ui/unused-crate-deps/warn-attr.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13


---- [ui] ui/unused-crate-deps/warn-cmdline-dylib.rs stdout ----

error: /checkout/src/test/ui/unused-crate-deps/warn-cmdline-dylib.rs:9: unexpected warning: '9:1: 9:13: external crate `bar` unused [unused_crate_deps]'

error: /checkout/src/test/ui/unused-crate-deps/warn-cmdline-dylib.rs:1: expected message not found: external crate `bar` unused
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/warn-cmdline-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline-dylib" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Wunused-crate-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline-dylib/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline-dylib/auxiliary/libbar.so"
    Error {
        line_num: 9,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "9:1: 9:13: external crate `bar` unused [unused_crate_deps]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 1,
        kind: None,
        msg: "external crate `bar` unused",
]

thread '[ui] ui/unused-crate-deps/warn-cmdline-dylib.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13


---- [ui] ui/unused-crate-deps/warn-cmdline.rs stdout ----

error: /checkout/src/test/ui/unused-crate-deps/warn-cmdline.rs:10: unexpected warning: '10:1: 10:13: external crate `bar` unused [unused_crate_deps]'

error: /checkout/src/test/ui/unused-crate-deps/warn-cmdline.rs:1: expected message not found: external crate `bar` unused
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/warn-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Wunused-crate-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/warn-cmdline/auxiliary/libbar.so"
    Error {
        line_num: 10,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "10:1: 10:13: external crate `bar` unused [unused_crate_deps]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 1,
        kind: None,
        msg: "external crate `bar` unused",
]

thread '[ui] ui/unused-crate-deps/warn-cmdline.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13

---

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:06:28
Build completed unsuccessfully in 1:06:28
== clock drift check ==
  local time: Tue May 19 22:27:26 UTC 2020
  network time: Tue, 19 May 2020 22:27:26 GMT
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
Terminate orphan process: pid (3715) (python)
##[section]Finishing: Finalize Job
