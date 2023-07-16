plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200430.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200430.2/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/54f81b26-ae68-4aab-b291-ed6a8731bfad.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72284/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72284/merge:refs/remotes/pull/72284/merge
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
.......................................................i............................................ 1800/10169
.................................................................................................... 1900/10169
..........................................................................i..i...................... 2000/10169
.................................................................................................... 2100/10169
................................................................iiiii............................... 2200/10169
.................................................................................................... 2400/10169
.................................................................................................... 2500/10169
.................................................................................................... 2600/10169
.................................................................................................... 2700/10169
---
.................................................................................................... 5200/10169
.................................................................................................... 5300/10169
...........................i........................................................................ 5400/10169
....................i............................................................................... 5500/10169
............................ii.ii........i...i...................................................... 5600/10169
..............................................................................i..................... 5800/10169
.................................................................................................... 5900/10169
.........................ii.....................................i................................... 6000/10169
.................................................................................................... 6100/10169
.................................................................................................... 6100/10169
.................................................................................................... 6200/10169
......................................................................................ii...i..ii.... 6300/10169
.................................................................................................... 6500/10169
.................................................................................................... 6600/10169
.................................................................................................... 6700/10169
.................................................................................................... 6700/10169
...................i..ii............................................................................ 6800/10169
.................................................................................................... 7000/10169
.........................................................................i.......................... 7100/10169
.................................................................................................... 7200/10169
.................................................................................................... 7300/10169
---
.................................................................................................... 8100/10169
.................................................................................................... 8200/10169
...........................................................................................i........ 8300/10169
.................................................................................................... 8400/10169
.............................................iiiiii.iiiii.i......................................... 8500/10169
..................................................................................................i. 8600/10169
.................................................................................................... 8800/10169
.................................................................................................... 8900/10169
.................................................................................................... 9000/10169
.................................................................................................... 9100/10169
---
diff of stdout:

17 /*
18 Expansions:
19 0: parent: ExpnId(0), call_site_ctxt: #0, kind: Root
- 1: parent: ExpnId(0), call_site_ctxt: #0, kind: Macro(Bang, "foo")
+ 1: parent: ExpnId(0), call_site_ctxt: #0, kind: Macro { kind: Bang, name: "foo", macro_def_id: Some(DefId(0:1 ~ unpretty_debug[317d]::foo[0])) }
22 SyntaxContexts:
22 SyntaxContexts:
23 #0: parent: #0, outer_mark: (ExpnId(0), Opaque)

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/unpretty-debug.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/unpretty-debug.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/unpretty-debug.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/unpretty-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=expanded,hygiene" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/auxiliary"
------------------------------------------
// check-pass
// compile-flags: -Zunpretty=expanded,hygiene


// Don't break whenever Symbol numbering changes
// normalize-stdout-test "\d+#" -> "0#"
// minimal junk
// minimal junk
#![feature /* 274#0 */(no_core)]
#![no_core /* 440#0 */]

macro_rules! foo /* 806#0 */ { ($ x : ident) => { y + $ x } }

fn bar /* 809#0 */() { let x /* 807#0 */ = 1; y /* 808#1 */ + x /* 807#0 */ }

fn y /* 808#0 */() { }
/*
Expansions:
Expansions:
0: parent: ExpnId(0), call_site_ctxt: #0, kind: Root
1: parent: ExpnId(0), call_site_ctxt: #0, kind: Macro { kind: Bang, name: "foo", macro_def_id: Some(DefId(0:1 ~ unpretty_debug[317d]::foo[0])) }
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (ExpnId(0), Opaque)
#1: parent: #0, outer_mark: (ExpnId(1), SemiTransparent)

------------------------------------------
stderr:
------------------------------------------
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:59:33
Build completed unsuccessfully in 0:59:33
== clock drift check ==
  local time: Sun May 17 02:21:47 UTC 2020
  network time: Sun, 17 May 2020 02:21:47 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72284/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72284/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3365) (python)
##[section]Finishing: Finalize Job
