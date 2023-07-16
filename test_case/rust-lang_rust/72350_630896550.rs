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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/da75f8a5-ac44-4228-b7f1-a90360ab11cc.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72350/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72350/merge:refs/remotes/pull/72350/merge
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
..................................................................i................................. 1800/10186
.................................................................................................... 1900/10186
.....................................................................................i..i........... 2000/10186
.................................................................................................... 2100/10186
...........................................................................iiiii.................... 2200/10186
.................................................................................................... 2400/10186
.................................................................................................... 2500/10186
.................................................................................................... 2600/10186
.................................................................................................... 2700/10186
---
.......i............................................................................................ 5200/10186
.................................................................................................... 5300/10186
......................................i............................................................. 5400/10186
...............................i.................................................................... 5500/10186
.......................................ii.ii........i...i........................................... 5600/10186
.........................................................................................i.......... 5800/10186
.................................................................................................... 5900/10186
....................................ii.....................................i........................ 6000/10186
.................................................................................................... 6100/10186
.................................................................................................... 6100/10186
.................................................................................................... 6200/10186
.................................................................................................ii. 6300/10186
..i..ii...........i................................................................................. 6400/10186
.................................................................................................... 6600/10186
.................................................................................................... 6700/10186
.................................................................................................... 6700/10186
..............................i..ii................................................................. 6800/10186
.................................................................................................... 7000/10186
....................................................................................i............... 7100/10186
.................................................................................................... 7200/10186
.................................................................................................... 7300/10186
---
.................................................................................................... 8100/10186
.................................................................................................... 8200/10186
.................................................................................................... 8300/10186
.......i............................................................................................ 8400/10186
.............................................................iiiiii.iiiiii.i........................ 8500/10186
...............i.................................................................................... 8700/10186
.................................................................................................... 8800/10186
.................................................................................................... 8900/10186
.................................................................................................... 9000/10186
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 189 tests
iiii......i.............ii..i.........i...............................i..i..................i....i.. 100/189
..........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.714
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.158
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 19.844
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 15.412
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2558 tests
......iiiii......................................................................................... 100/2558
.......................................................................................ii........... 200/2558
.........................i.......................................................................... 400/2558
...............................................................................i..i................. 500/2558
.iiii............................................................................................... 600/2558
.................................................................................................... 700/2558
---
---- slice/mod.rs - slice::from_raw_parts (line 5780) stdout ----
error[E0424]: expected value, found module `self`
  --> slice/mod.rs:5784:19
   |
6  | / fn join_slices<'a, T>(fst: &'a [T], snd: &'a [T]) -> &'a [T] {
7  | |     let fst_end = self.as_ptr().wrapping_add(self.len());
   | |                   ^^^^ `self` value is a keyword only available in methods with a `self` parameter
8  | |     let snd_start = snd.as_ptr();
9  | |     assert_eq!(fst_end, snd_start, "Slices must be contiguous!");
14 | |     }
15 | | }
   | |_- this function doesn't have a `self` parameter


error[E0424]: expected value, found module `self`
  --> slice/mod.rs:5784:46
   |
6  | / fn join_slices<'a, T>(fst: &'a [T], snd: &'a [T]) -> &'a [T] {
7  | |     let fst_end = self.as_ptr().wrapping_add(self.len());
   | |                                              ^^^^ `self` value is a keyword only available in methods with a `self` parameter
8  | |     let snd_start = snd.as_ptr();
9  | |     assert_eq!(fst_end, snd_start, "Slices must be contiguous!");
14 | |     }
15 | | }
   | |_- this function doesn't have a `self` parameter

---
  local time: Tue May 19 15:26:18 UTC 2020
  network time: Tue, 19 May 2020 15:26:19 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72350/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72350/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3561) (python)
##[section]Finishing: Finalize Job
