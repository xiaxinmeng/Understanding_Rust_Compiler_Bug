plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 51'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200604.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/90794349-0f3c-409d-bfa0-32112c3fb3fb.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72771/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72771/merge:refs/remotes/pull/72771/merge
---
 ---> 29a56a071ad9
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> eb826cd6a4d7
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 9841042138f8
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 00b49f7048de
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
...............................i...............i.................................................... 5200/10301
.................................................................................................... 5300/10301
...............................................................................i.................... 5400/10301
.........................................................................i.......................... 5500/10301
............................................................................................ii.ii... 5600/10301
.....i...i.......................................................................................... 5700/10301
............................................i....................................................... 5900/10301
..................................................................................................ii 6000/10301
.....................................i.............................................................. 6100/10301
.................................................................................................... 6200/10301
.................................................................................................... 6200/10301
.................................................................................................... 6300/10301
.............................................................ii...i..ii...........i................. 6400/10301
.................................................................................................... 6600/10301
.................................................................................................... 6700/10301
.................................................................................................... 6700/10301
..............................................................................................i..ii. 6800/10301
.................................................................................................... 7000/10301
.................................................................................................... 7100/10301
.................................................i.................................................. 7200/10301
.................................................................................................... 7300/10301
---
.................................................................................................... 8200/10301
.................................................................................................... 8300/10301
............................................................................................i....... 8400/10301
.................................................................................................... 8500/10301
.............................................iiiiii.iiiiii.i........................................ 8600/10301
..i................................................................................................. 8800/10301
.................................................................................................... 8900/10301
.................................................................................................... 9000/10301
.................................................................................................... 9100/10301
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
iiii......i..............ii..i.........i......................i...........i..i................i....i 100/193
.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii................ii......

 finished in 5.334
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.146
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 14.606
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
.................................................................................................... 500/769
...........................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:32
....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:31
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:28
.......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:23
..........................................thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:32
......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:31
.....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:23
.............................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:634:13
...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:588:13
....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:564:13
thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:695:13
thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:695:13
..thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/libstd/sync/mutex.rs:658:13
..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:789:13
....thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:765:13
..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:701:13
thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:629:13
thread '<unnamed>.' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:641:13
..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:616:13
.......................................... 700/769
.........................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1577:37
.............thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1712:13
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.830
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-101a02b96cd72ebe

running 0 tests

---
Set({"/checkout/src/librustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_serialize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 212 tests
......................i...ii........................................................................ 100/212
i........................................iiiiii......i..............iii............................. 200/212
........ii..

 finished in 63.877
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
doc tests for: /checkout/src/doc/rustc/src/targets/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 4.373
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

31    |
32    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
33 
+ warning: `[Uniooon::X]` cannot be resolved, ignoring it.
+    |
+    |
+ LL |       //! , [Uniooon::X] and [Qux::Z].
+    |
+    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
+ 
+ 
+ warning: `[Qux::Z]` cannot be resolved, ignoring it.
+    |
+    |
+ LL |       //! , [Uniooon::X] and [Qux::Z].
+    |
+    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
+ 
+ 
34 warning: `[Qux:Y]` cannot be resolved, ignoring it.
36    |

161    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
162    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intra-links-warning.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `[Foo::baz]` cannot be resolved, ignoring it.
   |
   |
LL |        //! Test with [Foo::baz], [Bar::foo], ...
   |
   = note: `#[warn(intra_doc_link_resolution_failure)]` on by default
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[Bar::foo]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3:35
   |
LL |        //! Test with [Foo::baz], [Bar::foo], ...
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[Uniooon::X]` cannot be resolved, ignoring it.
   |
   |
LL |      //! , [Uniooon::X] and [Qux::Z].
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[Qux::Z]` cannot be resolved, ignoring it.
   |
   |
LL |      //! , [Uniooon::X] and [Qux::Z].
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[Uniooon::X]` cannot be resolved, ignoring it.
   |
   |
LL |       //! , [Uniooon::X] and [Qux::Z].
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[Qux::Z]` cannot be resolved, ignoring it.
   |
   |
LL |       //! , [Uniooon::X] and [Qux::Z].
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[Qux:Y]` cannot be resolved, ignoring it.
   |
   |
LL |        /// [Qux:Y]
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[error]` cannot be resolved, ignoring it.
warning: `[error]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:58:30
   |
LL |  * time to introduce a link [error]*/ //~ WARNING `[error]` cannot be resolved
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[error]` cannot be resolved, ignoring it.
warning: `[error]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:64:30
   |
LL |  * time to introduce a link [error] //~ WARNING `[error]` cannot be resolved
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[error]` cannot be resolved, ignoring it.
warning: `[error]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:68:1
   |
LL | #[doc = "single line [error]"] //~ WARNING `[error]` cannot be resolved
   |
   = note: the link appears in this line:
           
           single line [error]
           single line [error]
                        ^^^^^
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[error]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:71:1
   |
LL | #[doc = "single line with \"escaping\" [error]"] //~ WARNING `[error]` cannot be resolved
   |
   = note: the link appears in this line:
           
           
           single line with "escaping" [error]
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[error]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:74:1
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:74:1
   |
LL | / /// Item docs. //~ WARNING `[error]` cannot be resolved
LL | | #[doc="Hello there!"]
LL | | /// [error]
   |
   = note: the link appears in this line:
           
           [error]
           [error]
            ^^^^^
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[error1]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:80:11
   |
LL | /// docs [error1] //~ WARNING `[error1]` cannot be resolved
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[error2]` cannot be resolved, ignoring it.
warning: `[error2]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:82:11
   |
LL | /// docs [error2] //~ WARNING `[error2]` cannot be resolved
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[BarA]` cannot be resolved, ignoring it.
   |
   |
LL | /// bar [BarA] bar //~ WARNING `[BarA]` cannot be resolved
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[BarB]` cannot be resolved, ignoring it.
   |
   |
LL |  * bar [BarB] bar //~ WARNING `[BarB]` cannot be resolved
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[BarC]` cannot be resolved, ignoring it.
   |
   |
LL | bar [BarC] bar //~ WARNING `[BarC]` cannot be resolved
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[BarD]` cannot be resolved, ignoring it.
warning: `[BarD]` cannot be resolved, ignoring it.
  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:45:1
   |
LL | #[doc = "Foo\nbar [BarD] bar\nbaz"] //~ WARNING `[BarD]` cannot be resolved
   |
   = note: the link appears in this line:
           
           
           bar [BarD] bar
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`


warning: `[BarF]` cannot be resolved, ignoring it.
   |
   |
LL |         #[doc = $f] //~ WARNING `[BarF]` cannot be resolved
...
...
LL | f!("Foo\nbar [BarF] bar\nbaz");
   |
   = note: the link appears in this line:
           
           
           bar [BarF] bar
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
   = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 19 warnings emitted
---

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:34:11
Build completed unsuccessfully in 1:34:11
== clock drift check ==
  local time: Fri Jun 12 03:56:59 UTC 2020
  network time: Fri, 12 Jun 2020 03:56:59 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72771/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72771/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3563) (python)
##[section]Finishing: Finalize Job
