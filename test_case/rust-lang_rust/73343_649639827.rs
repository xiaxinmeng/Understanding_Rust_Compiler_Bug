plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200621.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200621.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9d5cf8e6-5bc4-4aeb-a6ef-ce83de8e403b.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73343/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73343/merge:refs/remotes/pull/73343/merge
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
   Compiling chalk-solve v0.11.0
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
......................i............................................................................. 1900/10396
.................................................................................................... 2000/10396
.................................................i..i............................................... 2100/10396
.................................................................................................... 2200/10396
.......................................iiiii........................................................ 2300/10396
.................................................................................................... 2500/10396
.................................................................................................... 2600/10396
.................................................................................................... 2700/10396
.................................................................................................... 2800/10396
---
.................................................................................................... 5300/10396
.................................................................................................... 5400/10396
..............................i..................................................................... 5500/10396
........................i........................................................................... 5600/10396
............................................ii.ii........i...i...................................... 5700/10396
.............i...................................................................................... 5900/10396
.........i.......................................................................................... 6000/10396
.................................................................ii................................. 6100/10396
....i............................................................................................... 6200/10396
....i............................................................................................... 6200/10396
.................................................................................................... 6300/10396
.................................................................................................... 6400/10396
............................ii...i..ii...........i.................................................. 6500/10396
.................................................................................................... 6700/10396
.................................................................................................... 6800/10396
.................................................................................................... 6800/10396
...............................................................i..ii................................ 6900/10396
.................................................................................................... 7100/10396
.................................................................................................... 7200/10396
...................i................................................................................ 7300/10396
.................................................................................................... 7400/10396
---
.................................................................................................... 8300/10396
.................................................................................................... 8400/10396
....................................................................i............................... 8500/10396
.................................................................................................... 8600/10396
......................iiiiii..iiiiii.i.............................................................. 8700/10396
.................................................................................................... 8900/10396
.................................................................................................... 9000/10396
.................................................................................................... 9100/10396
.................................................................................................... 9200/10396
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 197 tests
iiii......i................ii.i..........i......................i...........i..i........i........i.. 100/197
..i.............i.i.i...iii..iiii....................................iii.................ii......

 finished in 6.835
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiiiiiiiiiiiii

 finished in 0.154
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 14.689
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2589 tests
......iiiii......................................................................................... 100/2589
.................................................................................................ii. 200/2589
.......................................i............................................................ 400/2589
...............................................................................................i..i. 500/2589
...............................................................................................i..i. 500/2589
.................iiii............................................................................... 600/2589
.................................................................................................... 800/2589
.................................................................................................... 900/2589
.................................................................................................... 1000/2589
.................................................................................................... 1100/2589
---

running 1029 tests
i................................................................................................... 100/1029
.................................................................................................... 200/1029
...................iii......i......i...i.........i.................................................. 300/1029
.......................................FF...................i....i.................................. 500/1029
.....ii............................................................................................. 600/1029
.................................................................................................... 700/1029
.................................................................................................... 700/1029
.....................................................iiii........................................... 800/1029
.................................................................................................... 900/1029
............................................................................iiii.................... 1000/1029
failures:

---- io/stdio.rs - io::stdio::Stdin::set_nonblocking (line 382) stdout ----
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
  --> io/stdio.rs:390:5
   |
10 |     handle.set_nonblocking(true)?;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error


For more information about this error, try `rustc --explain E0133`.
Couldn't compile the test.
---- io/stdio.rs - io::stdio::StdinLock::set_nonblocking (line 482) stdout ----
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
  --> io/stdio.rs:490:5
   |
10 |     handle.set_nonblocking(true)?;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error

---
  local time: Thu Jun 25 15:52:45 UTC 2020
  network time: Thu, 25 Jun 2020 15:52:46 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73343/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73343/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3787) (python)
##[section]Finishing: Finalize Job
