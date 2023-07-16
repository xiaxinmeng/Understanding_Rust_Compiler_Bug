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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b8406ae3-4779-4c48-bc01-45578a0b8ac2.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72259/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72259/merge:refs/remotes/pull/72259/merge
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
.......................................................i............................................ 1800/10173
.................................................................................................... 1900/10173
..........................................................................i..i...................... 2000/10173
.................................................................................................... 2100/10173
................................................................iiiii............................... 2200/10173
.................................................................................................... 2400/10173
.................................................................................................... 2500/10173
.................................................................................................... 2600/10173
.................................................................................................... 2700/10173
---
.................................................................................................... 5200/10173
.................................................................................................... 5300/10173
...........................i........................................................................ 5400/10173
....................i............................................................................... 5500/10173
............................ii.ii........i...i...................................................... 5600/10173
..............................................................................i..................... 5800/10173
.................................................................................................... 5900/10173
.........................ii.....................................i................................... 6000/10173
.................................................................................................... 6100/10173
.................................................................................................... 6100/10173
.................................................................................................... 6200/10173
......................................................................................ii...i..ii.... 6300/10173
.................................................................................................... 6500/10173
.................................................................................................... 6600/10173
.................................................................................................... 6700/10173
.................................................................................................... 6700/10173
...................i..ii............................................................................ 6800/10173
.................................................................................................... 7000/10173
.........................................................................i.......................... 7100/10173
.................................................................................................... 7200/10173
.................................................................................................... 7300/10173
---
.................................................................................................... 8100/10173
.................................................................................................... 8200/10173
................................................................................................i... 8300/10173
.................................................................................................... 8400/10173
.................................................iiiiii.iiiii.i..................................... 8500/10173
..i................................................................................................. 8700/10173
.................................................................................................... 8800/10173
.................................................................................................... 8900/10173
.................................................................................................... 9000/10173
---
---- [ui] ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:45:9
   |
LL | #![warn(x5400)] //~ WARN unknown lint: `x5400`
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:36:28
   |
   |
LL | #![warn(unused_attributes, unknown_lints)]
   |                            ^^^^^^^^^^^^^

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:46:10
   |
LL | #![allow(x5300)] //~ WARN unknown lint: `x5300`

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:47:11
   |
   |
LL | #![forbid(x5200)] //~ WARN unknown lint: `x5200`

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:48:9
   |
   |
LL | #![deny(x5100)] //~ WARN unknown lint: `x5100`

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:105:8
   |
   |
LL | #[warn(x5400)]
   |        ^^^^^

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:108:25
   |
LL |     mod inner { #![warn(x5400)] }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:111:12
   |
   |
LL |     #[warn(x5400)] fn f() { }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:114:12
   |
   |
LL |     #[warn(x5400)] struct S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:117:12
   |
   |
LL |     #[warn(x5400)] type T = S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:120:12
   |
   |
LL |     #[warn(x5400)] impl S { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:124:9
   |
   |
LL | #[allow(x5300)]
   |         ^^^^^

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:127:26
   |
LL |     mod inner { #![allow(x5300)] }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:130:13
   |
   |
LL |     #[allow(x5300)] fn f() { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:133:13
   |
   |
LL |     #[allow(x5300)] struct S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:136:13
   |
   |
LL |     #[allow(x5300)] type T = S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:139:13
   |
   |
LL |     #[allow(x5300)] impl S { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:143:10
   |
   |
LL | #[forbid(x5200)]

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:146:27
   |
   |
LL |     mod inner { #![forbid(x5200)] }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:149:14
   |
   |
LL |     #[forbid(x5200)] fn f() { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:152:14
   |
   |
LL |     #[forbid(x5200)] struct S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:155:14
   |
   |
LL |     #[forbid(x5200)] type T = S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:158:14
   |
   |
LL |     #[forbid(x5200)] impl S { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:162:8
   |
   |
LL | #[deny(x5100)]
   |        ^^^^^

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:165:25
   |
LL |     mod inner { #![deny(x5100)] }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:168:12
   |
   |
LL |     #[deny(x5100)] fn f() { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:171:12
   |
   |
LL |     #[deny(x5100)] struct S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:174:12
   |
   |
LL |     #[deny(x5100)] type T = S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:177:12
   |
   |
LL |     #[deny(x5100)] impl S { }


warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
   |
LL | #[macro_escape]
   | ^^^^^^^^^^^^^^^


warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
   |
   |
LL |     mod inner { #![macro_escape] }
   |
   = help: try an outer attribute: `#[macro_use]`


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
   |
   |
LL |     mod inner { #![plugin_registrar] }
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
   |
LL |     #[plugin_registrar] struct S;
   |     ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
   |
   |
LL |     #[plugin_registrar] type T = S;


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
   |
   |
LL |     #[plugin_registrar] impl S { }


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
   |
   |
LL | #![plugin_registrar] //~ WARN unused attribute

warning: use of deprecated attribute `crate_id`: no longer used.
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:91:1
   |
   |
LL | #![crate_id = "10"] //~ WARN use of deprecated attribute

warning: use of deprecated attribute `no_start`: no longer used.
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:96:1
   |
   |
LL | #![no_start] //~ WARN use of deprecated attribute


error: internal compiler error: src/librustc_middle/ty/mod.rs:2798: item_name: no name for DefPath { data: [DisambiguatedDefPathData { data: TypeNs("no_mangle"), disambiguator: 0 }, DisambiguatedDefPathData { data: Impl, disambiguator: 0 }], krate: crate0 }
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (3e10134b0 2020-05-16) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:94:12
   |
   |
LL | #![feature(rust1)] //~ WARN no longer requires an attribute to enable
   |
   = note: `#[warn(stable_features)]` on by default

warning: unused attribute
warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:185:5
   |
LL |     #[macro_use] fn f() { }
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:36:9
   |
---

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:191:5
   |
LL |     #[macro_use] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:194:5
   |
   |
LL |     #[macro_use] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:201:17
   |
   |
LL |     mod inner { #![macro_export] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:204:5
   |
   |
LL |     #[macro_export] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:207:5
   |
   |
LL |     #[macro_export] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:210:5
   |
   |
LL |     #[macro_export] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:213:5
   |
   |
LL |     #[macro_export] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:198:1
   |
   |
LL | #[macro_export]
   | ^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:221:17
   |
LL |     mod inner { #![plugin_registrar] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:227:5
   |
   |
LL |     #[plugin_registrar] struct S;
   |     ^^^^^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:231:5
   |
LL |     #[plugin_registrar] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:235:5
   |
   |
LL |     #[plugin_registrar] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:217:1
   |
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:243:17
   |
LL |     mod inner { #![main] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:248:5
   |
   |
LL |     #[main] struct S;
   |     ^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:251:5
   |
LL |     #[main] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:254:5
   |
   |
LL |     #[main] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:240:1
   |
   |
LL | #[main]
   | ^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:261:17
   |
LL |     mod inner { #![start] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:266:5
   |
   |
LL |     #[start] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:269:5
   |
   |
LL |     #[start] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:272:5
   |
   |
LL |     #[start] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:258:1
   |
   |
LL | #[start]
   | ^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:325:5
   |
LL |     #[path = "3800"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:328:5
   |
   |
LL |     #[path = "3800"]  struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:331:5
   |
   |
LL |     #[path = "3800"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:334:5
   |
   |
LL |     #[path = "3800"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:341:17
   |
   |
LL |     mod inner { #![automatically_derived] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:344:5
   |
   |
LL |     #[automatically_derived] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:347:5
   |
   |
LL |     #[automatically_derived] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:350:5
   |
   |
LL |     #[automatically_derived] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:353:5
   |
   |
LL |     #[automatically_derived] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:338:1
   |
   |
LL | #[automatically_derived]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:373:17
   |
   |
LL |     mod inner { #![no_link] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:376:5
   |
   |
LL |     #[no_link] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:379:5
   |
   |
LL |     #[no_link] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:382:5
   |
   |
LL |     #[no_link]type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:385:5
   |
   |
LL |     #[no_link] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:370:1
   |
   |
LL | #[no_link]
   | ^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:392:17
   |
LL |     mod inner { #![should_panic] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:395:5
   |
   |
LL |     #[should_panic] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:398:5
   |
   |
LL |     #[should_panic] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:401:5
   |
   |
LL |     #[should_panic] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:404:5
   |
   |
LL |     #[should_panic] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:389:1
   |
   |
LL | #[should_panic]
   | ^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:411:17
   |
LL |     mod inner { #![ignore] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:414:5
   |
   |
LL |     #[ignore] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:417:5
   |
   |
LL |     #[ignore] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:420:5
   |
   |
LL |     #[ignore] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:423:5
   |
   |
LL |     #[ignore] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:408:1
   |
   |
LL | #[ignore]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:430:17
   |
   |
LL |     mod inner { #![no_implicit_prelude] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:433:5
   |
   |
LL |     #[no_implicit_prelude] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:436:5
   |
   |
LL |     #[no_implicit_prelude] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:439:5
   |
   |
LL |     #[no_implicit_prelude] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:442:5
   |
   |
LL |     #[no_implicit_prelude] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:427:1
   |
   |
LL | #[no_implicit_prelude]
   | ^^^^^^^^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:449:17
   |
LL |     mod inner { #![reexport_test_harness_main="2900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:452:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:455:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:458:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:461:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:446:1
   |
   |
LL | #[reexport_test_harness_main = "2900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:472:5
   |
   |
LL |     #[macro_escape] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:475:5
   |
   |
LL |     #[macro_escape] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:478:5
   |
   |
LL |     #[macro_escape] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:481:5
   |
   |
LL |     #[macro_escape] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:489:17
   |
   |
LL |     mod inner { #![no_std] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:489:17
   |
   |
LL |     mod inner { #![no_std] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:493:5
   |
   |
LL |     #[no_std] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:497:5
   |
   |
LL |     #[no_std] struct S;
   |     ^^^^^^^^^

warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
LL |     #[no_std] struct S;
   |     ^^^^^^^^^


warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:501:5
   |
LL |     #[no_std] type T = S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:505:5
   |
   |
LL |     #[no_std] impl S { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:485:1
   |
   |
LL | #[no_std]
   | ^^^^^^^^^

warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
LL | #[no_std]
   | ^^^^^^^^^


warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:644:17
   |
LL |     mod inner { #![crate_name="0900"] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:644:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:648:5
   |
   |
LL |     #[crate_name = "0900"] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[crate_name = "0900"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:652:5
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:10:18
Build completed unsuccessfully in 1:10:18
== clock drift check ==
  local time: Sat May 16 09:48:02 UTC 2020
  network time: Sat, 16 May 2020 09:48:03 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72259/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72259/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4173) (python)
##[section]Finishing: Finalize Job
