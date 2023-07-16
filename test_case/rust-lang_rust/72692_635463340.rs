plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 6'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fd3435dc-7ca3-484f-a20a-13e9d89d0601.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72692/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72692/merge:refs/remotes/pull/72692/merge
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
..............................................................................i..................... 1800/10249
.................................................................................................... 1900/10249
.................................................................................................i.. 2000/10249
i................................................................................................... 2100/10249
.......................................................................................iiiii........ 2200/10249
.................................................................................................... 2400/10249
.................................................................................................... 2500/10249
.................................................................................................... 2600/10249
.................................................................................................... 2700/10249
---
...............i...............i.................................................................... 5200/10249
.................................................................................................... 5300/10249
...............................................................i.................................... 5400/10249
........................................................i........................................... 5500/10249
....................................................................ii.ii........i...i.............. 5600/10249
...........i........................................................................................ 5800/10249
...................i................................................................................ 5900/10249
........................................................................ii.......................... 6000/10249
...........i........................................................................................ 6100/10249
...........i........................................................................................ 6100/10249
.................................................................................................... 6200/10249
.................................................................................................... 6300/10249
.................................ii...i..ii...........i............................................. 6400/10249
.................................................................................................... 6600/10249
.................................................................................................... 6700/10249
.................................................................................................... 6700/10249
..................................................................i..ii............................. 6800/10249
.................................................................................................... 7000/10249
.................................................................................................... 7100/10249
....................i............................................................................... 7200/10249
.................................................................................................... 7300/10249
---
.................................................................................................... 8200/10249
.................................................................................................... 8300/10249
.......................................................i............................................ 8400/10249
.................................................................................................... 8500/10249
.........iiiiii.iiiiii.i............................................................................ 8600/10249
.................................................................................................... 8800/10249
.................................................................................................... 8900/10249
.................................................................................................... 9000/10249
.................................................................................................... 9100/10249
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 191 tests
iiii......i..............ii.i..........i.................................i..i................i....i. 100/191
............i.i.i...iii..iiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.439
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 20 tests
iiiiiiiiiiiiiiiiiiii

 finished in 0.163
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 15.707
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2571 tests
......iiiii......................................................................................... 100/2571
.................................................................................................ii. 200/2571
...................................i................................................................ 400/2571
..........................................................................................i..i...... 500/2571
..........................................................................................i..i...... 500/2571
............iiii.................................................................................... 600/2571
.................................................................................................... 800/2571
.................................................................................................... 900/2571
.................................................................................................... 1000/2571
.................................................................................................... 1100/2571
---
.................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:944:13
. 300/764
.................................................................................................... 400/764
.................................................................................................... 500/764
......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
.thread '..<unnamed>....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
.' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError.', src/libstd/sync/mpsc/mod.rs:2778:21
........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
.......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
.............. 600/764
........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:634:13
..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:588:13
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

 finished in 145.430
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.980
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-0f7bb3477fa2da6f

running 0 tests

---
Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 211 tests
......................i...ii.......................................................................i 100/211
........................................iiiiii......i..............iii.............................. 200/211
.......ii..

 finished in 63.776
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
doc tests for: /checkout/src/doc/rustc/src/targets/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 3.998
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
Checking "alias-1" ... OK
Checking "alias-2" ... OK
Checking "alias-3" ... OK
Checking "alias" ... OK
Checking "basic" ... OK
Checking "deduplication" ... OK
Checking "enum-option" ... OK
Checking "filter-crate" ... OK
Checking "fn-forget" ... OK
Checking "from_u" ... OK
Checking "keyword" ... OK
Checking "macro-check" ... OK
Checking "macro-print" ... OK
Checking "multi-query" ... OK
Checking "never" ... OK
Checking "quoted" ... OK
Checking "return-specific-literal" ... OK
Checking "return-specific" ... OK
Checking "should-fail" ... OK
Checking "string-from_ut" ... OK
Checking "struct-vec" ... OK
Checking "vec-new" ... OK
Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 7 tests
.......
---
    Finished test [unoptimized] target(s) in 24.45s
     Running /checkout/obj/build/bootstrap/debug/deps/bootstrap-9767abf417bc796f

running 11 tests
test builder::tests::build_default ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::build_with_target_flag ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::dist_baseline ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::dist_only_cross_host ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::dist_with_hosts ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::dist_with_same_targets_and_hosts ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::dist_with_target_flag ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::dist_with_targets ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::dist_with_targets_and_hosts ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::test_exclude ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED
FAILED
test builder::tests::test_with_no_doc_stage0 ... error: Error loading target specification: Could not find specification for target "A". Use `--print target-list` for a list of built-in targets
FAILED

failures:


---- builder::tests::build_default stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- builder::tests::build_with_target_flag stdout ----
---- builder::tests::build_with_target_flag stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::dist_baseline stdout ----
---- builder::tests::dist_baseline stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::dist_only_cross_host stdout ----
---- builder::tests::dist_only_cross_host stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::dist_with_hosts stdout ----
---- builder::tests::dist_with_hosts stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::dist_with_same_targets_and_hosts stdout ----
---- builder::tests::dist_with_same_targets_and_hosts stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::dist_with_target_flag stdout ----
---- builder::tests::dist_with_target_flag stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::dist_with_targets stdout ----
---- builder::tests::dist_with_targets stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::dist_with_targets_and_hosts stdout ----
---- builder::tests::dist_with_targets_and_hosts stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::test_exclude stdout ----
---- builder::tests::test_exclude stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"

---- builder::tests::test_with_no_doc_stage0 stdout ----
---- builder::tests::test_with_no_doc_stage0 stdout ----
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--target" "A" "--print" "target-libdir"


failures:
    builder::tests::build_default
---
  local time: Thu May 28 16:45:06 UTC 2020
  network time: Thu, 28 May 2020 16:45:06 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72692/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72692/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4020) (python)
##[section]Finishing: Finalize Job
