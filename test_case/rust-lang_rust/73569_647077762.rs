plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 61'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6302fa7a-8abc-4ea1-8c4a-bb48552d806d.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73569/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73569/merge:refs/remotes/pull/73569/merge
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
...............i.................................................................................... 1900/10376
.................................................................................................... 2000/10376
.........................................i..i....................................................... 2100/10376
.................................................................................................... 2200/10376
...............................iiiii................................................................ 2300/10376
.................................................................................................... 2500/10376
.................................................................................................... 2600/10376
.................................................................................................... 2700/10376
.................................................................................................... 2800/10376
---
.................................................................................................... 5300/10376
.................................................................................................... 5400/10376
...................i................................................................................ 5500/10376
.............i...................................................................................... 5600/10376
.................................ii.ii........i...i................................................. 5700/10376
.i...............................................................................................i.. 5900/10376
.................................................................................................... 6000/10376
...................................................ii.....................................i......... 6100/10376
.................................................................................................... 6200/10376
.................................................................................................... 6200/10376
.................................................................................................... 6300/10376
.................................................................................................... 6400/10376
..............ii...i..ii...........i................................................................ 6500/10376
.................................................................................................... 6700/10376
.................................................................................................... 6800/10376
.................................................................................................... 6800/10376
................................................i..ii............................................... 6900/10376
.................................................................................................... 7100/10376
.................................................................................................... 7200/10376
....i............................................................................................... 7300/10376
.................................................................................................... 7400/10376
---
.................................................................................................... 8300/10376
.................................................................................................... 8400/10376
..................................................i................................................. 8500/10376
.................................................................................................... 8600/10376
....iiiiii..iiiiii.i................................................................................ 8700/10376
.................................................................................................... 8900/10376
.................................................................................................... 9000/10376
.................................................................................................... 9100/10376
.................................................................................................... 9200/10376
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 197 tests
iiii......i................ii.i..........i......................i...........i..i........i........i.. 100/197
..i.............i.i.i...iii..iiii....................................iii.................ii......

 finished in 6.664
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiiiiiiiiiiiii

 finished in 0.162
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 15.595
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
.................................................................................................... 100/1026
.................................................................................................... 200/1026
.................................................................................................... 300/1026
.................................................................................................... 400/1026
..FFF....F...............FF.F..........ii........................................................... 500/1026
.................................................................................................... 700/1026
.................................................................................................... 800/1026
.................................................................................................... 900/1026
.................................................................................................... 1000/1026
.................................................................................................... 1000/1026
..........................
failures:

---- num::dec2flt::fast_path_correct stdout ----
thread 'num::dec2flt::fast_path_correct' panicked at 'assertion failed: `(left == right)`
  left: `Err(ParseFloatError { kind: Invalid })`,
 right: `Ok(1.448997445238699)`', src/libcore/../libcore/tests/num/dec2flt/mod.rs:97:5
---- num::dec2flt::infinity stdout ----
---- num::dec2flt::infinity stdout ----
thread 'num::dec2flt::infinity' panicked at 'assertion failed: `(left == right)`
  left: `Err(ParseFloatError { kind: Invalid })`,
 right: `Ok(inf)`', src/libcore/../libcore/tests/num/dec2flt/mod.rs:73:5
---- num::dec2flt::large stdout ----
---- num::dec2flt::large stdout ----
thread 'num::dec2flt::large' panicked at 'assertion failed: `(left == right)`
  left: `Err(ParseFloatError { kind: Invalid })`,
 right: `Ok(1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0)`', src/libcore/../libcore/tests/num/dec2flt/mod.rs:53:5
---- num::dec2flt::ordinary stdout ----
---- num::dec2flt::ordinary stdout ----
thread 'num::dec2flt::ordinary' panicked at 'assertion failed: `(left == right)`
  left: `Err(ParseFloatError { kind: Invalid })`,
 right: `Ok(1.0)`', src/libcore/../libcore/tests/num/dec2flt/mod.rs:27:5
---- num::dec2flt::special_code_paths stdout ----
---- num::dec2flt::special_code_paths stdout ----
thread 'num::dec2flt::special_code_paths' panicked at 'assertion failed: `(left == right)`
  left: `Err(ParseFloatError { kind: Invalid })`,
 right: `Ok(36893488147419103000.0)`', src/libcore/../libcore/tests/num/dec2flt/mod.rs:44:5
---- num::dec2flt::subnormals stdout ----
---- num::dec2flt::subnormals stdout ----
thread 'num::dec2flt::subnormals' panicked at 'assertion failed: `(left == right)`
  left: `Err(ParseFloatError { kind: Invalid })`,
 right: `Ok(0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005)`', src/libcore/../libcore/tests/num/dec2flt/mod.rs:61:5
---- num::dec2flt::zero stdout ----
---- num::dec2flt::zero stdout ----
thread 'num::dec2flt::zero' panicked at 'assertion failed: `(left == right)`
  left: `Err(ParseFloatError { kind: Invalid })`,
 right: `Ok(0.0)`', src/libcore/../libcore/tests/num/dec2flt/mod.rs:81:5

failures:
    num::dec2flt::fast_path_correct
    num::dec2flt::infinity
---
  local time: Sun Jun 21 04:35:00 UTC 2020
  network time: Sun, 21 Jun 2020 04:35:00 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73569/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73569/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4414) (python)
##[section]Finishing: Finalize Job
