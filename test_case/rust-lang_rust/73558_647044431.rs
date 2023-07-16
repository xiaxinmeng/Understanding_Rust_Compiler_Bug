plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 28'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b2f18d36-993b-418a-a3fe-4c51bef9178a.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73558/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73558/merge:refs/remotes/pull/73558/merge
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
.............i...................................................................................... 1900/10368
.................................................................................................... 2000/10368
.......................................i..i......................................................... 2100/10368
.................................................................................................... 2200/10368
.............................iiiii.................................................................. 2300/10368
.................................................................................................... 2500/10368
.................................................................................................... 2600/10368
.................................................................................................... 2700/10368
.................................................................................................... 2800/10368
---
.................................................................................................... 5300/10368
.................................................................................................... 5400/10368
.................i.................................................................................. 5500/10368
...........i........................................................................................ 5600/10368
...............................ii.ii........i...i................................................... 5700/10368
...........................................................................i.......................i 5800/10368
.................................................................................................... 6000/10368
.................................................ii.....................................i........... 6100/10368
.................................................................................................... 6200/10368
.................................................................................................... 6300/10368
.................................................................................................... 6300/10368
.................................................................................................... 6400/10368
............ii...i..ii...........i.................................................................. 6500/10368
.................................................................................................... 6700/10368
.................................................................................................... 6800/10368
...............................................i.ii................................................. 6900/10368
.................................................................................................... 7000/10368
---
.................................................................................................... 8300/10368
.................................................................................................... 8400/10368
...............................................i.................................................... 8500/10368
.................................................................................................... 8600/10368
.iiiiii..iiiiii.i................................................................................... 8700/10368
.................................................................................................... 8900/10368
.................................................................................................... 9000/10368
.................................................................................................... 9100/10368
.................................................................................................... 9200/10368
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 196 tests
iiii......i...............ii.i..........i......................i...........i..i........i........i... 100/196
.i.............i.i.i...iii..iiii....................................iii.................ii......

 finished in 7.014
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiiiiiiiiiiiii

 finished in 0.166
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 15.722
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2585 tests
......iiiii......................................................................................... 100/2585
.................................................................................................ii. 200/2585
.......................................i............................................................ 400/2585
...............................................................................................i..i. 500/2585
...............................................................................................i..i. 500/2585
.................iiii............................................................................... 600/2585
.................................................................................................... 800/2585
.................................................................................................... 900/2585
.................................................................................................... 1000/2585
.................................................................................................... 1100/2585
---
---- result.rs - result::Result<T, E>::as_deref (line 1157) stdout ----
error[E0658]: use of unstable library feature 'inner_deref'
 --> result.rs:1160:14
  |
6 | assert_eq!(x.as_deref(), y);
  |
  |
  = note: see issue #50264 <***/issues/50264> for more information
  = help: add `#![feature(inner_deref)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'inner_deref'
  --> result.rs:1164:14
   |
   |
10 | assert_eq!(x.as_deref(), y);
   |
   |
   = note: see issue #50264 <***/issues/50264> for more information
   = help: add `#![feature(inner_deref)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- result.rs - result::Result<T, E>::as_deref_mut (line 1191) stdout ----
error[E0308]: mismatched types
 --> result.rs:1193:40
  |
5 | let y: Result<&mut str, &mut u32> = Ok("HELLO");
  |                                        ^^^^^^^ types differ in mutability
  = note: expected mutable reference `&mut str`
                     found reference `&'static str`

error[E0658]: use of unstable library feature 'inner_deref'
error[E0658]: use of unstable library feature 'inner_deref'
 --> result.rs:1194:14
  |
6 | assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
  |
  |
  = note: see issue #50264 <***/issues/50264> for more information
  = help: add `#![feature(inner_deref)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> result.rs:1197:41
  |
  |
9 | let y: Result<&mut str, &mut u32> = Err(&42);
  |                                         ^^^ types differ in mutability
  = note: expected mutable reference `&mut u32`
                     found reference `&u32`

error[E0658]: use of unstable library feature 'inner_deref'
error[E0658]: use of unstable library feature 'inner_deref'
  --> result.rs:1198:14
   |
10 | assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
   |
   |
   = note: see issue #50264 <***/issues/50264> for more information
   = help: add `#![feature(inner_deref)]` to the crate attributes to enable
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
---
  local time: Sat Jun 20 20:55:21 UTC 2020
  network time: Sat, 20 Jun 2020 20:55:21 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73558/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73558/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3713) (python)
##[section]Finishing: Finalize Job
