plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 5'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bd298ee6-64e0-44d6-b110-e9393b0aa049.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71948/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71948/merge:refs/remotes/pull/71948/merge
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
running 10161 tests
.................................................................................................... 100/10161
.................................................................................................... 200/10161
.........F.......................................................................................... 300/10161
...............................................FF..F......F....F...FFF..F........................... 400/10161
..................................................................................i................. 600/10161
.................................................................................................... 700/10161
.................................................................................................... 800/10161
.................................................................................................... 900/10161
.................................................................................................... 900/10161
...........................................................................................i........ 1000/10161
.................................................F.FF.F............................................. 1100/10161
.................................................................................................... 1300/10161
.................................................................................................... 1400/10161
.....................................F.F............................................................ 1500/10161
.................................................................................................... 1600/10161
.................................................................................................... 1600/10161
.................................................................................................... 1700/10161
...............................F......................i............................................. 1800/10161
.................................................................................................... 1900/10161
...........F....F.......................................................iF.i........................ 2000/10161
.................................................................................................... 2100/10161
..............................................................iiiii................................. 2200/10161
.................................................................................................... 2400/10161
.................................................................................................... 2500/10161
.................................................................................................... 2600/10161
.................................................................................................... 2700/10161
---
............................F....................................................................... 5200/10161
.................................................................................................... 5300/10161
.........................i...........................................................F.............. 5400/10161
..................i...................F............................................................. 5500/10161
....................F....ii.ii........i...i......................................................... 5600/10161
..........................................................................i......................... 5800/10161
.................................................................................................... 5900/10161
.....................ii.....................................i....................................... 6000/10161
.............................................................................F...................... 6100/10161
.............................................................................F...................... 6100/10161
.................................................................................................... 6200/10161
..................................................................................ii...i..ii........ 6300/10161
.................................................................................................... 6500/10161
.................................................................................................... 6600/10161
.................................................................................................... 6700/10161
.................................................................................................... 6700/10161
...............i..ii................................................................................ 6800/10161
.................................................................................................... 7000/10161
.....................................................................i.............................. 7100/10161
.................................................................................................... 7200/10161
.................................................................................................... 7300/10161
---
..........................i........i................................................................ 7800/10161
.................................................................................................... 7900/10161
.................................................................................................... 8000/10161
................................................................F................................... 8100/10161
....................................................................F..FF.F......................... 8200/10161
.................................................................................................... 8400/10161
.................................................................................................... 8400/10161
......................................iiiiii.iiiii.i................................................ 8500/10161
.................................................................................................... 8700/10161
...............................F.................................................................... 8800/10161
.................................................................................................... 8900/10161
.................................................................................................... 9000/10161
.................................................................................................... 9000/10161
..........................................F......................................................... 9100/10161
.................................................................................................... 9200/10161
..................................................................................................F. 9300/10161
.....FF..F.F.F...................................................................................... 9400/10161
................................................................i................................... 9600/10161
..................F................................................................................. 9700/10161
.................................................................................................... 9800/10161
...........................................F....F................................................... 9900/10161
...........................................F....F................................................... 9900/10161
.......................................................F.FF.F..F..F................................. 10000/10161
.................................................................i.......FF...FFFF..FFFF.F.F..F...FF 10100/10161
FFF..F......F................................................

---- [ui] ui/associated-type-bounds/bad-bounds-on-assoc-in-trait.rs stdout ----


error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ bad_bounds_on_assoc_in_trait[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ bad_bounds_on_assoc_in_trait[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/defaults-suitability.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-suitability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_suitability[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_suitability[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_suitability[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_suitability[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_suitability[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_suitability[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_suitability[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_suitability[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:27:5
   |
   |
LL |     type Ty = Vec<[u8]>;
   | 
  ::: /checkout/src/liballoc/vec.rs:300:16
   |
LL | pub struct Vec<T> {
---


---- [ui] ui/associated-types/defaults-unsound-62211-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_unsound_62211_1[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_unsound_62211_1[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/defaults-unsound-62211-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_unsound_62211_2[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ defaults_unsound_62211_2[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/issue-43924.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-43924.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43924" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43924/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_43924[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_43924[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_43924[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/issue-63593.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-63593.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-63593" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-63593/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_63593[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/issue-64855.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-64855.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-64855" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-64855/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_64855[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/issue-65774-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-65774-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-65774-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-65774-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_65774_1[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_65774_1[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/issue-65774-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-65774-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-65774-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-65774-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_65774_2[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_65774_2[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/point-at-type-on-obligation-failure-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ point_at_type_on_obligation_failure_2[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ point_at_type_on_obligation_failure_2[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ point_at_type_on_obligation_failure_2[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 3 previous errors


------------------------------------------
---
error: /checkout/src/test/ui/async-await/issue-61076.rs:8: unexpected error: '8:5: 8:11: the `?` operator can only be applied to values that implement `std::ops::Try` [E0277]'

error: 1 unexpected errors found, 0 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:11: the `?` operator can only be applied to values that implement `std::ops::Try` [E0277]",
]

thread '[ui] ui/async-await/issue-61076.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/async-await/issue-68523.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-68523.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68523" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68523/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_68523[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/builtin-superkinds/builtin-superkinds-double-superkind.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/builtin-superkinds/builtin-superkinds-double-superkind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-superkinds/builtin-superkinds-double-superkind" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-superkinds/builtin-superkinds-double-superkind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ builtin_superkinds_double_superkind[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ builtin_superkinds_double_superkind[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/builtin-superkinds/builtin-superkinds-in-metadata.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/builtin-superkinds/builtin-superkinds-in-metadata.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-superkinds/builtin-superkinds-in-metadata" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-superkinds/builtin-superkinds-in-metadata/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ builtin_superkinds_in_metadata[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/builtin-superkinds/builtin-superkinds-simple.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/builtin-superkinds/builtin-superkinds-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-superkinds/builtin-superkinds-simple" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-superkinds/builtin-superkinds-simple/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ builtin_superkinds_simple[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/builtin-superkinds/builtin-superkinds-typaram-not-send.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/builtin-superkinds/builtin-superkinds-typaram-not-send.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-superkinds/builtin-superkinds-typaram-not-send" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-superkinds/builtin-superkinds-typaram-not-send/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ builtin_superkinds_typaram_not_send[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/chalkify/impl_wf.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/impl_wf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/impl_wf" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "chalk" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/impl_wf/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ impl_wf[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z chalk -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/closures/closure-bounds-cant-promote-superkind-in-struct.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-bounds-cant-promote-superkind-in-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-cant-promote-superkind-in-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-cant-promote-superkind-in-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ closure_bounds_cant_promote_superkind_in_struct[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/issues/issue-61336-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61336-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_61336_2[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/issues/issue-61336.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61336.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_61336[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/consts/const-unsized.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-unsized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-unsized" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-unsized/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ const_unsized[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ const_unsized[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ const_unsized[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ const_unsized[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 4 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ fn_call_in_non_const[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/trait-error.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ trait_error[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/cross/cross-fn-cache-hole.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cross/cross-fn-cache-hole.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross/cross-fn-cache-hole" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross/cross-fn-cache-hole/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ cross_fn_cache_hole[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/dst/dst-sized-trait-param.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dst/dst-sized-trait-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-sized-trait-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-sized-trait-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ dst_sized_trait_param[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ dst_sized_trait_param[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/feature-gates/feature-gate-const_in_array_repeat_expressions.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_const_in_array_repeat_expressions[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/feature-gates/feature-gate-trivial_bounds.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trivial_bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trivial_bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ feature_gate_trivial_bounds[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to 11 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/generics/issue-61631-default-type-param-can-reference-self-in-trait.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/issue-61631-default-type-param-can-reference-self-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-61631-default-type-param-can-reference-self-in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-61631-default-type-param-can-reference-self-in-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ issue_61631_default_type_param_can_reference_self_in_trait[8787]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/impl-bounds-checking.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-bounds-checking.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-bounds-checking" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-bounds-checking/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: src/librustc_middle/hir/map/mod.rs:396: no entry for id `HirId { owner: DefId(0:0 ~ impl_bounds_checking[317d]), local_id: 0 }`
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (b4438a0a3 2020-05-10) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-10412.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10412.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:1:20
   |
LL | trait Serializable<'self, T> { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:2:25
   |
   |
LL |     fn serialize(val : &'self T) -> Vec<u8>; //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:3:38
   |
   |
LL |     fn deserialize(repr : &[u8]) -> &'self T; //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:6
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:36
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
---
test result: FAILED. 10012 passed; 88 failed; 61 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:00
== clock drift check ==
  local time: Sun May 10 15:51:10 UTC 2020
  network time: Sun, 10 May 2020 15:51:10 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71948/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71948/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4004) (python)
##[section]Finishing: Finalize Job
