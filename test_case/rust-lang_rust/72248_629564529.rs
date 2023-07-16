plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 7'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/720ca2ab-1571-453a-b2ed-a606e6947122.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72248/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72248/merge:refs/remotes/pull/72248/merge
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
......................................................i............................................. 1800/10167
.................................................................................................... 1900/10167
........................................................................i..i........................ 2000/10167
.................................................................................................... 2100/10167
..............................................................iiiii................................. 2200/10167
.................................................................................................... 2400/10167
.................................................................................................... 2500/10167
.................................................................................................... 2600/10167
.................................................................................................... 2700/10167
---
.................................................................................................... 5200/10167
.................................................................................................... 5300/10167
.........................i.......................................................................... 5400/10167
..................i................................................................................. 5500/10167
..........................ii.ii........i...i........................................................ 5600/10167
............................................................................i....................... 5800/10167
.................................................................................................... 5900/10167
.......................ii.....................................i..................................... 6000/10167
.................................................................................................... 6100/10167
.................................................................................................... 6100/10167
.................................................................................................... 6200/10167
....................................................................................ii...i..ii...... 6300/10167
.................................................................................................... 6500/10167
.................................................................................................... 6600/10167
.................................................................................................... 6700/10167
.................................................................................................... 6700/10167
.................i..ii.............................................................................. 6800/10167
.................................................................................................... 7000/10167
.......................................................................i............................ 7100/10167
.................................................................................................... 7200/10167
.................................................................................................... 7300/10167
---
.................................................................................................... 8100/10167
.................................................................................................... 8200/10167
.........................................................................................i.......... 8300/10167
.................................................................................................... 8400/10167
...........................................iiiiii..iiiiii........................................... 8500/10167
.................................................................................................... 8700/10167
.................................................................................................... 8800/10167
.................................................................................................... 8900/10167
.................................................................................................... 9000/10167
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 188 tests
iiii......i.............ii.i..........i...............................i..i..................i....i.. 100/188
..........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 5.296
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.136
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 14.583
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2557 tests
......iiiii......................................................................................... 100/2557
.......................................................................................ii........... 200/2557
.........................i.......................................................................... 400/2557
...............................................................................i..i................. 500/2557
.iiii............................................................................................... 600/2557
.................................................................................................... 700/2557
---
.................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:944:13
. 300/764
.................................................................................................... 400/764
.................................................................................................... 500/764
......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
............................................thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
.........thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvErrorthread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
............. 600/764
........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:634:13
..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:588:13
...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:564:13
---

running 1020 tests
i................................................................................................... 100/1020
.................................................................................................... 200/1020
...................iii......i......i...i......i..................................................... 300/1020
.................................................................................................... 400/1020
....................................................i....i......................................ii.. 500/1020
.................................................................................................... 700/1020
.................................................................................................... 700/1020
..............................................iiii.................................................. 800/1020
.................................................................................................... 900/1020
....................................................................iiii............................ 1000/1020
test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out

 finished in 142.879
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
   Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
error[E0308]: mismatched types
   --> src/librustc_interface/tests.rs:414:31
    |
414 |     tracked!(code_model, Some(String::from("code model")));
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::spec::CodeModel`, found struct `std::string::String`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_interface`.
error: could not compile `rustc_interface`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_interface" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:21:25
Build completed unsuccessfully in 1:21:25
== clock drift check ==
  local time: Sat May 16 00:56:50 UTC 2020
  network time: Sat, 16 May 2020 00:56:51 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72248/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72248/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3779) (python)
##[section]Finishing: Finalize Job
