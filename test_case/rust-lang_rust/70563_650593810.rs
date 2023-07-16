plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 8'
Agent machine name: 'fv-az659'
Current agent version: '2.171.1'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/be58172c-8d8c-46c8-a157-6f548ddb8893.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/70563/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70563/merge:refs/remotes/pull/70563/merge
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
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
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
...........................i........................................................................ 1900/10409
.................................................................................................... 2000/10409
......................................................i..i.......................................... 2100/10409
.................................................................................................... 2200/10409
............................................iiiii................................................... 2300/10409
.................................................................................................... 2500/10409
.................................................................................................... 2600/10409
.................................................................................................... 2700/10409
.................................................................................................... 2800/10409
---
...i................................................................................................ 5300/10409
.................................................................................................... 5400/10409
....................................i............................................................... 5500/10409
..............................i..................................................................... 5600/10409
..................................................ii.ii........i...i................................ 5700/10409
...................i................................................................................ 5900/10409
................i................................................................................... 6000/10409
..........................................................................ii........................ 6100/10409
.............i...................................................................................... 6200/10409
.............i...................................................................................... 6200/10409
.................................................................................................... 6300/10409
.................................................................................................... 6400/10409
.....................................ii...i..ii...........i......................................... 6500/10409
.................................................................................................... 6700/10409
.................................................................................................... 6800/10409
.................................................................................................... 6800/10409
........................................................................i..ii....................... 6900/10409
.................................................................................................... 7100/10409
.................................................................................................... 7200/10409
............................i....................................................................... 7300/10409
.................................................................................................... 7400/10409
---
.................................................................................................... 8300/10409
.................................................................................................... 8400/10409
...............................................................................i.................... 8500/10409
.................................................................................................... 8600/10409
.................................iiiiii..iiiiii.i................................................... 8700/10409
.................................................................................................... 8900/10409
.................................................................................................... 9000/10409
.................................................................................................... 9100/10409
.................................................................................................... 9200/10409
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 202 tests
iiii......i..i...............ii..i..........i...........i............i...........i..i........i...... 100/202
..i....i.............i.i.i...iii..iiii....................................iii.................ii.... 200/202
test result: ok. 170 passed; 0 failed; 32 ignored; 0 measured; 0 filtered out

 finished in 6.910
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiiiiiiiiiiiii

 finished in 0.159
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 16.258
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2586 tests
......iiiii......................................................................................... 100/2586
.................................................................................................ii. 200/2586
.......................................i............................................................ 400/2586
...............................................................................................i..i. 500/2586
...............................................................................................i..i. 500/2586
.................iiii............................................................................... 600/2586
.................................................................................................... 800/2586
.................................................................................................... 900/2586
.................................................................................................... 1000/2586
.................................................................................................... 1100/2586
---

running 1043 tests
i................................................................................................... 100/1043
.................................................................................................... 200/1043
...................iii......i......i...i.........i.................................................. 300/1043
..........................................................i....i.................................... 500/1043
...................ii............................................................................... 600/1043
.................................................................................................... 700/1043
.................................................................................................... 700/1043
...................................................................iiii............................. 800/1043
.................................................................................................... 900/1043
..........................................................................................iiii...... 1000/1043
test result: ok. 1023 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out

 finished in 155.168
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
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
Documenting error index (x86_64-unknown-linux-gnu)
({"src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Rustbook (x86_64-unknown-linux-gnu) - edition-guide
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.59s
core/option/enum.Option.html:28: broken link fragment `#Some.v` pointing to `core/option/enum.Option.html`
core/option/enum.Option.html:100: broken link fragment `#Some.v` pointing to `core/option/enum.Option.html`
core/option/enum.Option.html:432: broken link fragment `#Some.v` pointing to `core/option/enum.Option.html`
core/option/enum.Option.html:433: broken link fragment `#Some.v` pointing to `core/option/enum.Option.html`
core/option/enum.Option.html:527: broken link fragment `#None.v` pointing to `core/option/enum.Option.html`
core/option/enum.Option.html:564: broken link fragment `#None.v` pointing to `core/option/enum.Option.html`
core/option/enum.Option.html:565: broken link fragment `#None.v` pointing to `core/option/enum.Option.html`
core/option/enum.Option.html:566: broken link fragment `#None.v` pointing to `core/option/enum.Option.html`
core/str/trait.FromStr.html:90: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/str/trait.FromStr.html:125: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:32: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:45: broken link fragment `#Err.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:172: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:183: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:420: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:433: broken link fragment `#Err.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:436: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:451: broken link fragment `#Err.v` pointing to `core/result/enum.Result.html`
core/result/enum.Result.html:528: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/alloc/enum.AllocInit.html:16: broken link fragment `#Uninitialized.v` pointing to `core/alloc/enum.AllocInit.html`
core/alloc/enum.AllocInit.html:17: broken link fragment `#Zeroed.v` pointing to `core/alloc/enum.AllocInit.html`
core/alloc/trait.AllocRef.html:87: broken link fragment `#InPlace.v` pointing to `core/alloc/enum.ReallocPlacement.html`
core/alloc/trait.AllocRef.html:88: broken link fragment `#MayMove.v` pointing to `core/alloc/enum.ReallocPlacement.html`
core/alloc/trait.AllocRef.html:125: broken link fragment `#InPlace.v` pointing to `core/alloc/enum.ReallocPlacement.html`
core/iter/trait.IntoIterator.html:109: broken link fragment `#Ok.v` pointing to `core/result/enum.Result.html`
core/iter/trait.FromIterator.html:139: broken link fragment `#None.v` pointing to `core/option/enum.Option.html`
core/iter/trait.FromIterator.html:140: broken link fragment `#None.v` pointing to `core/option/enum.Option.html`
core/iter/trait.FromIterator.html:141: broken link fragment `#None.v` pointing to `core/option/enum.Option.html`
core/default/trait.Default.html:102: broken link fragment `#None.v` pointing to `core/option/enum.Option.html`
alloc/alloc/enum.AllocInit.html:16: broken link fragment `#Uninitialized.v` pointing to `alloc/alloc/enum.AllocInit.html`
alloc/alloc/enum.AllocInit.html:17: broken link fragment `#Zeroed.v` pointing to `alloc/alloc/enum.AllocInit.html`
alloc/alloc/trait.AllocRef.html:87: broken link fragment `#InPlace.v` pointing to `alloc/alloc/enum.ReallocPlacement.html`
alloc/alloc/trait.AllocRef.html:88: broken link fragment `#MayMove.v` pointing to `alloc/alloc/enum.ReallocPlacement.html`
alloc/alloc/trait.AllocRef.html:125: broken link fragment `#InPlace.v` pointing to `alloc/alloc/enum.ReallocPlacement.html`
std/io/struct.IoSliceMut.html:721: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/io/struct.IoSliceMut.html:724: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/io/struct.IoSliceMut.html:753: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/io/struct.IoSliceMut.html:756: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/io/struct.IoSliceMut.html:778: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/io/struct.IoSliceMut.html:781: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/option/enum.Option.html:28: broken link fragment `#Some.v` pointing to `std/option/enum.Option.html`
std/option/enum.Option.html:100: broken link fragment `#Some.v` pointing to `std/option/enum.Option.html`
std/option/enum.Option.html:432: broken link fragment `#Some.v` pointing to `std/option/enum.Option.html`
std/option/enum.Option.html:433: broken link fragment `#Some.v` pointing to `std/option/enum.Option.html`
std/option/enum.Option.html:527: broken link fragment `#None.v` pointing to `std/option/enum.Option.html`
std/option/enum.Option.html:564: broken link fragment `#None.v` pointing to `std/option/enum.Option.html`
std/option/enum.Option.html:565: broken link fragment `#None.v` pointing to `std/option/enum.Option.html`
std/option/enum.Option.html:566: broken link fragment `#None.v` pointing to `std/option/enum.Option.html`
std/str/trait.FromStr.html:90: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/str/trait.FromStr.html:125: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:32: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:45: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:172: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:183: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:420: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:433: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:436: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:451: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/result/enum.Result.html:528: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/vec/struct.Vec.html:1539: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/vec/struct.Vec.html:1542: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/vec/struct.Vec.html:1571: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/vec/struct.Vec.html:1574: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/vec/struct.Vec.html:1596: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/vec/struct.Vec.html:1599: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/primitive.f32.html:816: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/primitive.f64.html:822: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/alloc/enum.AllocInit.html:16: broken link fragment `#Uninitialized.v` pointing to `std/alloc/enum.AllocInit.html`
std/alloc/enum.AllocInit.html:17: broken link fragment `#Zeroed.v` pointing to `std/alloc/enum.AllocInit.html`
std/alloc/trait.AllocRef.html:87: broken link fragment `#InPlace.v` pointing to `std/alloc/enum.ReallocPlacement.html`
std/alloc/trait.AllocRef.html:88: broken link fragment `#MayMove.v` pointing to `std/alloc/enum.ReallocPlacement.html`
std/alloc/trait.AllocRef.html:125: broken link fragment `#InPlace.v` pointing to `std/alloc/enum.ReallocPlacement.html`
std/primitive.slice.html:710: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/primitive.slice.html:713: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/primitive.slice.html:742: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/primitive.slice.html:745: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/primitive.slice.html:767: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/primitive.slice.html:770: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/iter/trait.IntoIterator.html:192: broken link fragment `#Ok.v` pointing to `std/result/enum.Result.html`
std/iter/trait.FromIterator.html:140: broken link fragment `#None.v` pointing to `std/option/enum.Option.html`
std/iter/trait.FromIterator.html:141: broken link fragment `#None.v` pointing to `std/option/enum.Option.html`
std/iter/trait.FromIterator.html:142: broken link fragment `#None.v` pointing to `std/option/enum.Option.html`
std/default/trait.Default.html:116: broken link fragment `#None.v` pointing to `std/option/enum.Option.html`
std/os/unix/net/struct.UnixDatagram.html:188: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/os/unix/net/struct.UnixDatagram.html:201: broken link fragment `#Err.v` pointing to `std/result/enum.Result.html`
std/os/unix/fs/trait.FileExt.html:38: broken link fragment `#write.v` pointing to `std/fs/struct.File.html`
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
---
  local time: Sat Jun 27 18:07:31 UTC 2020
  network time: Sat, 27 Jun 2020 18:07:31 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/70563/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/70563/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3604) (python)
##[section]Finishing: Finalize Job
