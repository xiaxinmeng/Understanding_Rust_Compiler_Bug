plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/47297546-6628-4387-a8dc-9db2a244d58d.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73787/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73787/merge:refs/remotes/pull/73787/merge
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
...........................i........................................................................ 1900/10408
.................................................................................................... 2000/10408
......................................................i..i.......................................... 2100/10408
.................................................................................................... 2200/10408
............................................iiiii................................................... 2300/10408
.................................................................................................... 2500/10408
.................................................................................................... 2600/10408
.................................................................................................... 2700/10408
.................................................................................................... 2800/10408
---
...i................................................................................................ 5300/10408
.................................................................................................... 5400/10408
...................................i................................................................ 5500/10408
.............................i...................................................................... 5600/10408
.................................................ii.ii........i...i................................. 5700/10408
..................i................................................................................. 5900/10408
...............i.................................................................................... 6000/10408
.........................................................................ii......................... 6100/10408
............i....................................................................................... 6200/10408
............i....................................................................................... 6200/10408
.................................................................................................... 6300/10408
.................................................................................................... 6400/10408
....................................ii...i..ii...........i.......................................... 6500/10408
.................................................................................................... 6700/10408
.................................................................................................... 6800/10408
.................................................................................................... 6800/10408
.......................................................................i..ii........................ 6900/10408
.................................................................................................... 7100/10408
.................................................................................................... 7200/10408
...........................i........................................................................ 7300/10408
.................................................................................................... 7400/10408
---
.................................................................................................... 8300/10408
.................................................................................................... 8400/10408
..............................................................................i..................... 8500/10408
.................................................................................................... 8600/10408
................................iiiiii..iiiiii.i.................................................... 8700/10408
.................................................................................................... 8900/10408
.................................................................................................... 9000/10408
.................................................................................................... 9100/10408
.................................................................................................... 9200/10408
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 202 tests
iiii......i..i...............ii..i..........i...........i............i...........i..i........i...... 100/202
..i....i.............i.i.i...iii..iiii....................................iii.................ii.... 200/202
test result: ok. 170 passed; 0 failed; 32 ignored; 0 measured; 0 filtered out

 finished in 6.337
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiiiiiiiiiiiii

 finished in 0.143
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 14.917
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

running 1038 tests
i................................................................................................... 100/1038
.................................................................................................... 200/1038
....................iii.....i......i...i.........i.................................................. 300/1038
..........................................................i....i.................................... 500/1038
..............ii.................................................................................... 600/1038
.................................................................................................... 700/1038
.................................................................................................... 700/1038
..............................................................iiii.................................. 800/1038
.................................................................................................... 900/1038
.....................................................................................iiii........... 1000/1038
test result: ok. 1018 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out

 finished in 146.262
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
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools skipping - compiler/librustdoc docs disabled
/tidy"]
Documenting stage2 rustdoc (x86_64-unknown-linux-gnu)
 skipping - compiler/librustdoc docs disabled
---
Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 212 tests
......................i...ii........................................................................ 100/212
i........................................iiiiii......i..............iii............................. 200/212
........ii..

 finished in 66.275
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---

stdout ----

running 2 tests
test /checkout/src/doc/unstable-book/src/language-features/rustc-attrs.md - This_feature_has_no_tracking_issue__and_is_therefore_likely_internal_to::Examples (line 30) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/rustc-attrs.md - This_feature_has_no_tracking_issue__and_is_therefore_likely_internal_to::Examples (line 18) ... FAILED
failures:


---- /checkout/src/doc/unstable-book/src/language-features/rustc-attrs.md - This_feature_has_no_tracking_issue__and_is_therefore_likely_internal_to::Examples (line 30) stdout ----
error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `{`
  |
3 | error: abi: Aggregate { sized: true }
  |           -           ^ expected one of 8 possible tokens
  |           |
  |           |
  |           tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
  = note: see issue #23416 <***/issues/23416> for more information
error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/rustc-attrs.md - This_feature_has_no_tracking_issue__and_is_therefore_likely_internal_to::Examples (line 18) stdout ----
error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
  |
  |
5 | #[rustc_Layout(abi, size)]

error: cannot find attribute `rustc_Layout` in this scope
 --> /checkout/src/doc/unstable-book/src/language-features/rustc-attrs.md:21:3
  |
  |
5 | #[rustc_Layout(abi, size)]
  |   ^^^^^^^^^^^^ help: a built-in attribute with a similar name exists: `rustc_layout`
error: aborting due to 2 previous errors

Couldn't compile the test.


failures:
    /checkout/src/doc/unstable-book/src/language-features/rustc-attrs.md - This_feature_has_no_tracking_issue__and_is_therefore_likely_internal_to::Examples (line 18)
    /checkout/src/doc/unstable-book/src/language-features/rustc-attrs.md - This_feature_has_no_tracking_issue__and_is_therefore_likely_internal_to::Examples (line 30)
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out


stderr ----
---
  local time: Sat Jun 27 06:52:41 UTC 2020
  network time: Sat, 27 Jun 2020 06:52:41 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73787/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73787/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4215) (python)
##[section]Finishing: Finalize Job
