plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 15'
Agent machine name: 'fv-az578'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/98775d6a-7893-42ee-bfd4-2b43b71e9a27.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73733/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73733/merge:refs/remotes/pull/73733/merge
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
processor : 0
vendor_id : GenuineIntel
cpu family : 6
model  : 85
model name : Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz
microcode : 0xffffffff
cpu MHz  : 2593.906
cache size : 36608 KB
physical id : 0
---
processor : 1
vendor_id : GenuineIntel
cpu family : 6
model  : 85
model name : Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz
microcode : 0xffffffff
cpu MHz  : 2593.906
cache size : 36608 KB
physical id : 0
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
   Compiling chalk-engine v0.11.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.11.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
......................i............................................................................. 1900/10398
.................................................................................................... 2000/10398
.................................................i..i............................................... 2100/10398
.................................................................................................... 2200/10398
.......................................iiiii........................................................ 2300/10398
.................................................................................................... 2500/10398
.................................................................................................... 2600/10398
.................................................................................................... 2700/10398
.................................................................................................... 2800/10398
---
.................................................................................................... 5300/10398
.................................................................................................... 5400/10398
..............................i..................................................................... 5500/10398
........................i........................................................................... 5600/10398
............................................ii.ii........i...i...................................... 5700/10398
.............i...................................................................................... 5900/10398
..........i......................................................................................... 6000/10398
..................................................................ii................................ 6100/10398
.....i.............................................................................................. 6200/10398
.....i.............................................................................................. 6200/10398
.................................................................................................... 6300/10398
.................................................................................................... 6400/10398
.............................ii...i..ii...........i................................................. 6500/10398
.................................................................................................... 6700/10398
.................................................................................................... 6800/10398
.................................................................................................... 6800/10398
................................................................i..ii............................... 6900/10398
.................................................................................................... 7100/10398
.................................................................................................... 7200/10398
....................i............................................................................... 7300/10398
.................................................................................................... 7400/10398
---
.................................................................................................... 8300/10398
.................................................................................................... 8400/10398
.....................................................................i.............................. 8500/10398
.................................................................................................... 8600/10398
.......................iiiiii..iiiiii.i............................................................. 8700/10398
.................................................................................................... 8900/10398
.................................................................................................... 9000/10398
.................................................................................................... 9100/10398
.................................................................................................... 9200/10398
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 197 tests
iiii......i................ii.i..........i......................i...........i..i........i........i.. 100/197
..i.............i.i.i...iii..iiii....................................iii.................ii......

 finished in 5.676
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiiiiiiiiiiiii

 finished in 0.134
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 13.815
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
---- slice/mod.rs - slice::[T]::strip_prefix (line 1468) stdout ----
error[E0308]: mismatched types
 --> slice/mod.rs:1471:1
  |
6 | assert_eq!(v.strip_prefix(&[10]), Some(&[40, 30]));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[{integer}]`, found array `[{integer}; 2]`
  |
  = note: expected enum `std::option::Option<&[{integer}]>`
             found enum `std::option::Option<&[{integer}; 2]>`

error[E0308]: mismatched types
 --> slice/mod.rs:1472:1
  |
  |
7 | assert_eq!(v.strip_prefix(&[10, 40]), Some(&[30]));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[{integer}]`, found array `[{integer}; 1]`
  |
  = note: expected enum `std::option::Option<&[{integer}]>`
             found enum `std::option::Option<&[{integer}; 1]>`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---- slice/mod.rs - slice::[T]::strip_prefix (line 1479) stdout ----
error[E0308]: mismatched types
 --> slice/mod.rs:1482:1
  |
6 | assert_eq!(v.strip_prefix(&[]), Some(v));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[{integer}]`, found array `[{integer}; 3]`
  |
  = note: expected enum `std::option::Option<&[{integer}]>`
             found enum `std::option::Option<&[{integer}; 3]>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---- slice/mod.rs - slice::[T]::strip_suffix (line 1508) stdout ----
error[E0308]: mismatched types
 --> slice/mod.rs:1511:1
  |
6 | assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40]));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[{integer}]`, found array `[{integer}; 2]`
  |
  = note: expected enum `std::option::Option<&[{integer}]>`
             found enum `std::option::Option<&[{integer}; 2]>`

error[E0308]: mismatched types
 --> slice/mod.rs:1512:1
  |
  |
7 | assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10]));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[{integer}]`, found array `[{integer}; 1]`
  |
  = note: expected enum `std::option::Option<&[{integer}]>`
             found enum `std::option::Option<&[{integer}; 1]>`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---- slice/mod.rs - slice::[T]::strip_suffix (line 1519) stdout ----
error[E0308]: mismatched types
 --> slice/mod.rs:1522:1
  |
6 | assert_eq!(v.strip_suffix(&[]), Some(v));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[{integer}]`, found array `[{integer}; 3]`
  |
  = note: expected enum `std::option::Option<&[{integer}]>`
             found enum `std::option::Option<&[{integer}; 3]>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
  local time: Thu Jun 25 21:22:56 UTC 2020
  network time: Thu, 25 Jun 2020 21:22:56 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73733/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73733/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3866) (python)
##[section]Finishing: Finalize Job
