plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 6'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c1ca4711-4bd9-4f60-9f3a-57325df27e75.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73441/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73441/merge:refs/remotes/pull/73441/merge
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

############################################################              84.7%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-04-22/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
    Updating git repository `https://github.com/alexcrichton/addr2line`
    Updating git repository `https://github.com/gimli-rs/gimli`
    Updating git repository `https://github.com/Frommi/miniz_oxide`
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
   Compiling rustc-demangle v0.1.16
   Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
   Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
   Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
   Compiling gimli v0.21.0 (https://github.com/gimli-rs/gimli#ea707d78)
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling object v0.20.0
   Compiling addr2line v0.12.0 (https://github.com/alexcrichton/addr2line?branch=dep-of-std#94e604b2)
   Compiling term v0.0.0 (/checkout/src/libterm)
   Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
---
   Compiling block-padding v0.1.5
   Compiling getopts v0.2.21
   Compiling rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
   Compiling crossbeam-utils v0.7.2
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling petgraph v0.5.1
   Compiling rustc_version v0.2.3
   Compiling crossbeam-queue v0.1.2
   Compiling unicode-security v0.0.5
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
   Compiling rustc-demangle v0.1.16
   Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
   Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
   Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
   Compiling gimli v0.21.0 (https://github.com/gimli-rs/gimli#ea707d78)
   Compiling object v0.20.0
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling addr2line v0.12.0 (https://github.com/alexcrichton/addr2line?branch=dep-of-std#94e604b2)
   Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
   Compiling term v0.0.0 (/checkout/src/libterm)
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
---
   Compiling crossbeam-utils v0.7.2
   Compiling block-padding v0.1.5
   Compiling getopts v0.2.21
   Compiling rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
   Compiling miniz_oxide v0.3.7 (https://github.com/Frommi/miniz_oxide#7f5aedd7)
   Compiling petgraph v0.5.1
   Compiling rustc_version v0.2.3
   Compiling crossbeam-queue v0.1.2
   Compiling unicode-security v0.0.5
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
...........................i........................................................................ 1900/10407
.................................................................................................... 2000/10407
......................................................i..i.......................................... 2100/10407
.................................................................................................... 2200/10407
............................................iiiii................................................... 2300/10407
.................................................................................................... 2500/10407
.................................................................................................... 2600/10407
.................................................................................................... 2700/10407
.................................................................................................... 2800/10407
---
...i................................................................................................ 5300/10407
.................................................................................................... 5400/10407
...................................i................................................................ 5500/10407
.............................i...................................................................... 5600/10407
.................................................ii.ii........i...i................................. 5700/10407
..................i................................................................................. 5900/10407
...............i.................................................................................... 6000/10407
.........................................................................ii......................... 6100/10407
............i....................................................................................... 6200/10407
............i....................................................................................... 6200/10407
.................................................................................................... 6300/10407
.................................................................................................... 6400/10407
....................................ii...i..ii...........i.......................................... 6500/10407
.................................................................................................... 6700/10407
.................................................................................................... 6800/10407
.................................................................................................... 6800/10407
.......................................................................i..ii........................ 6900/10407
.................................................................................................... 7100/10407
.................................................................................................... 7200/10407
...........................i........................................................................ 7300/10407
.................................................................................................... 7400/10407
---
.................................................................................................... 8300/10407
.................................................................................................... 8400/10407
............................................................................i....................... 8500/10407
.................................................................................................... 8600/10407
..............................iiiiii..iiiiii.i...................................................... 8700/10407
.................................................................................................... 8900/10407
.................................................................................................... 9000/10407
.................................................................................................... 9100/10407
.................................................................................................... 9200/10407
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 202 tests
iiii......i..i...............ii..i..........i...........i............i...........i..i........i...... 100/202
..i....i.............i.i.i...iii..iiii....................................iii.................ii.... 200/202
test result: ok. 170 passed; 0 failed; 32 ignored; 0 measured; 0 filtered out

 finished in 9.526
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiiiiiiiiiiiii

 finished in 0.173
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 17.456
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
............
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests std
error[E0659]: `std` is ambiguous (name vs any other name during import resolution)
   --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:344:13
    |
344 |         use std::ffi::{OsStr, CStr};
    |
    = note: `std` could refer to a crate passed with `--extern`
    = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
note: `std` could also refer to the module defined here
   --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:25:1
25  | / mod std {
26  | |     pub use crate::*;
27  | | }
    | |_^
    | |_^
    = help: use `self::std` to refer to this module unambiguously

error[E0659]: `std` is ambiguous (name vs any other name during import resolution)
  --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:19:5
19 | use std::ffi::OsString;
   |     ^^^ ambiguous name
   |
   = note: `std` could refer to a crate passed with `--extern`
   = note: `std` could refer to a crate passed with `--extern`
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
  --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:25:1
25 | / mod std {
26 | |     pub use crate::*;
27 | | }
   | |_^
   | |_^
   = help: use `self::std` to refer to this module unambiguously

error[E0659]: `std` is ambiguous (name vs any other name during import resolution)
  --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:20:5
20 | use std::fs::File;
   |     ^^^ ambiguous name
   |
   = note: `std` could refer to a crate passed with `--extern`
   = note: `std` could refer to a crate passed with `--extern`
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
  --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:25:1
25 | / mod std {
26 | |     pub use crate::*;
27 | | }
   | |_^
   | |_^
   = help: use `self::std` to refer to this module unambiguously

error[E0659]: `std` is ambiguous (name vs any other name during import resolution)
  --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:21:5
21 | use std::path::Path;
   |     ^^^ ambiguous name
   |
   = note: `std` could refer to a crate passed with `--extern`
   = note: `std` could refer to a crate passed with `--extern`
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
  --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:25:1
25 | / mod std {
26 | |     pub use crate::*;
27 | | }
   | |_^
   | |_^
   = help: use `self::std` to refer to this module unambiguously

error[E0659]: `std` is ambiguous (name vs any other name during import resolution)
  --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:22:5
22 | use std::prelude::v1::*;
   |     ^^^ ambiguous name
   |
   = note: `std` could refer to a crate passed with `--extern`
   = note: `std` could refer to a crate passed with `--extern`
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
  --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:25:1
25 | / mod std {
26 | |     pub use crate::*;
27 | | }
   | |_^
   | |_^
   = help: use `self::std` to refer to this module unambiguously

error[E0659]: `std` is ambiguous (name vs any other name during import resolution)
   --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:343:13
343 |         use std::os::unix::prelude::*;
    |             ^^^ ambiguous name
    |
    = note: `std` could refer to a crate passed with `--extern`
    = note: `std` could refer to a crate passed with `--extern`
    = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
   --> /checkout/src/libstd/../backtrace/src/symbolize/gimli.rs:25:1
25  | / mod std {
26  | |     pub use crate::*;
27  | | }
    | |_^
---
  local time: Fri Jun 26 18:10:59 UTC 2020
  network time: Fri, 26 Jun 2020 18:11:00 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73441/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73441/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4042) (python)
##[section]Finishing: Finalize Job
