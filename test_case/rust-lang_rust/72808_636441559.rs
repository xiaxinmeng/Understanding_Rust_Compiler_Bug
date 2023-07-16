plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
Agent machine name: 'fv-az619'
Current agent version: '2.169.1'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d8d954eb-84b8-4407-92c3-763f2a3401a1.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72808/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72808/merge:refs/remotes/pull/72808/merge
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
.................................................................................................... 1700/10276
..................................................................................i................. 1800/10276
.................................................................................................... 1900/10276
.................................................................................................... 2000/10276
...i..i............................................................................................. 2100/10276
.............................................................................................iiiii.. 2200/10276
.................................................................................................... 2400/10276
.................................................................................................... 2500/10276
.................................................................................................... 2600/10276
.................................................................................................... 2700/10276
---
........................i...............i........................................................... 5200/10276
.................................................................................................... 5300/10276
........................................................................i........................... 5400/10276
..................................................................i................................. 5500/10276
..................................................................................ii.ii........i...i 5600/10276
.........................i.......................................................................... 5800/10276
.................................i.................................................................. 5900/10276
.......................................................................................ii........... 6000/10276
..........................i......................................................................... 6100/10276
..........................i......................................................................... 6100/10276
.................................................................................................... 6200/10276
.................................................................................................... 6300/10276
.................................................ii...i..ii...........i............................. 6400/10276
.................................................................................................... 6600/10276
.................................................................................................... 6700/10276
.................................................................................................... 6700/10276
..................................................................................i..ii............. 6800/10276
.................................................................................................... 7000/10276
.................................................................................................... 7100/10276
....................................i............................................................... 7200/10276
.................................................................................................... 7300/10276
---
.................................................................................................... 8200/10276
.................................................................................................... 8300/10276
.........................................................................i.......................... 8400/10276
.................................................................................................... 8500/10276
...........................iiiiii.iiiiii.i.......................................................... 8600/10276
.................................................................................................... 8800/10276
.................................................................................................... 8900/10276
.................................................................................................... 9000/10276
.................................................................................................... 9100/10276
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 192 tests
iiii......i..............ii.i..........i......................i...........i..i................i....i 100/192
.............i.i.i...iii..iiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.439
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.154
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 13.990
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2577 tests
......iiiii......................................................................................... 100/2577
.................................................................................................ii. 200/2577
......................................i............................................................. 400/2577
..............................................................................................i..i.. 500/2577
..............................................................................................i..i.. 500/2577
................iiii................................................................................ 600/2577
.................................................................................................... 800/2577
.................................................................................................... 900/2577
.................................................................................................... 1000/2577
.................................................................................................... 1100/2577
---

running 769 tests
.................................................................................................... 100/769
.................................................................................................... 200/769
.................................................FFFthread '<unnamed>' panicked at 'explicit panic', src/libstd/io/buffered.rs:1705:17
.thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:944:13
................................................................................................... 400/769
.................................................................................................... 500/769
...........................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
---
 right: `2`', src/libstd/io/buffered.rs:1773:9

---- io::buffered::tests::line_vectored_partial_and_errors stdout ----
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `[[97, 98, 99, 120, 10]]`,
 right: `[[97, 98, 99]]`', src/libstd/io/buffered.rs:1833:25
---- io::buffered::tests::test_line_buffer_fail_flush stdout ----
---- io::buffered::tests::test_line_buffer_fail_flush stdout ----
thread '<unnamed>' panicked at 'didn't flush on new line', src/libstd/io/buffered.rs:1605:17

failures:
    io::buffered::tests::erroneous_flush_retried
    io::buffered::tests::line_vectored
    io::buffered::tests::line_vectored
    io::buffered::tests::line_vectored_partial_and_errors
    io::buffered::tests::test_line_buffer_fail_flush

test result: FAILED. 765 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '-p std --lib'

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--" "--quiet"
expected success, got: exit code: 101

---
  local time: Sun May 31 08:48:24 UTC 2020
  network time: Sun, 31 May 2020 08:48:25 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72808/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72808/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4343) (python)
##[section]Finishing: Finalize Job
