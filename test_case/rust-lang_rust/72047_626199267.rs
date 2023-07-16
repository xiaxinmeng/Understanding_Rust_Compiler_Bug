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
Downloading task: Bash (3.163.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/95a76d67-d8a1-4333-8d46-e784501d439c.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72047/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72047/merge:refs/remotes/pull/72047/merge
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
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
.......................i............................................................................ 1800/9999
.................................................................................................... 1900/9999
........................................i........................................................... 2000/9999
.................................................................................................... 2100/9999
..............................iiiii................................................................. 2200/9999
.................................................................................................... 2400/9999
.................................................................................................... 2500/9999
.................................................................................................... 2600/9999
.................................................................................................... 2700/9999
---
.....................i...............i.............................................................. 5100/9999
.................................................................................................... 5200/9999
...................................................................i................................ 5300/9999
...........................................................i........................................ 5400/9999
................................................................ii.ii........i...i.................. 5500/9999
......i............................................................................................. 5700/9999
.............i...................................................................................... 5800/9999
.................................................ii.....................................i........... 5900/9999
.................................................................................................... 6000/9999
.................................................................................................... 6000/9999
.................................................................................................... 6100/9999
.....................................................................................ii...i...ii.... 6200/9999
.................................................................................................... 6400/9999
.................................................................................................... 6500/9999
.................................................................................................... 6600/9999
.................................................................................................... 6600/9999
.................i..ii.............................................................................. 6700/9999
.................................................................................................... 6900/9999
..................i................................................................................. 7000/9999
.................................................................................................... 7100/9999
............................................................i....................................... 7200/9999
---
.................................................................................................... 7900/9999
.................................................................................................... 8000/9999
.................................................................................................... 8100/9999
............................i....................................................................... 8200/9999
..................................................................................iiiiii.iiiii.i.... 8300/9999
...................................i................................................................ 8500/9999
.................................................................................................... 8600/9999
.................................................................................................... 8700/9999
.................................................................................................... 8800/9999
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 188 tests
iiii......i.............ii.i..........i................................i.i..................i....i.. 100/188
..........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 6.013
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.162
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 15.725
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2554 tests
......iiiii......................................................................................... 100/2554
.......................................................................................ii........... 200/2554
.........................i.......................................................................... 400/2554
...............................................................................i..i................. 500/2554
.iiii............................................................................................... 600/2554
.................................................................................................... 700/2554
---
.................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:944:13
. 300/764
.................................................................................................... 400/764
.................................................................................................... 500/764
......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
...<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
....thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
.........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
............thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rsthread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
............. 600/764
........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:634:.13
..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:588:13
..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:564:13
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

 finished in 162.918
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
   Compiling rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
error[E0425]: cannot find function `unescape_str` in this scope
   --> src/librustc_lexer/src/unescape/tests.rs:105:9
    |
105 |         unescape_str(literal_text, &mut |range, c| {
    |         ^^^^^^^^^^^^ help: a function with a similar name exists: `unescape_byte`
   ::: src/librustc_lexer/src/unescape.rs:107:1
    |
    |
107 | pub fn unescape_byte(literal_text: &str) -> Result<u8, (usize, EscapeError)> {
    | ---------------------------------------------------------------------------- similarly named function `unescape_byte` defined here
error[E0425]: cannot find function `unescape_byte_str` in this scope
   --> src/librustc_lexer/src/unescape/tests.rs:225:9
    |
    |
225 |         unescape_byte_str(literal_text, &mut |range, c| {
    |         ^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `unescape_byte`
   ::: src/librustc_lexer/src/unescape.rs:107:1
    |
    |
107 | pub fn unescape_byte(literal_text: &str) -> Result<u8, (usize, EscapeError)> {
    | ---------------------------------------------------------------------------- similarly named function `unescape_byte` defined here
error[E0425]: cannot find function `unescape_raw_str` in this scope
   --> src/librustc_lexer/src/unescape/tests.rs:249:9
    |
246 | fn test_unescape_raw_str() {
246 | fn test_unescape_raw_str() {
    | -------------------------- similarly named constant `test_unescape_raw_str` defined here
...
249 |         unescape_raw_str(literal, &mut |range, res| unescaped.push((range, res)));

error[E0425]: cannot find function `unescape_raw_byte_str` in this scope
   --> src/librustc_lexer/src/unescape/tests.rs:261:9
    |
    |
258 | fn test_unescape_raw_byte_str() {
    | ------------------------------- similarly named constant `test_unescape_raw_byte_str` defined here
...
261 |         unescape_raw_byte_str(literal, &mut |range, res| unescaped.push((range, res)));

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_lexer`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_lexer" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:26:04
Build completed unsuccessfully in 1:26:04
== clock drift check ==
  local time: Sat May  9 16:13:14 UTC 2020
  network time: Sat, 09 May 2020 16:13:14 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72047/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72047/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4609) (python)
##[section]Finishing: Finalize Job
