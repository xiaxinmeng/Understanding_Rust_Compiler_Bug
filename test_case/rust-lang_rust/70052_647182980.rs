plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 7'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/03e86d8e-62b0-4fd5-b56c-0ea68264156e.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/70052/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70052/merge:refs/remotes/pull/70052/merge
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
...............i.................................................................................... 1900/10375
.................................................................................................... 2000/10375
.........................................i..i....................................................... 2100/10375
.................................................................................................... 2200/10375
...............................iiiii................................................................ 2300/10375
.................................................................................................... 2500/10375
.................................................................................................... 2600/10375
.................................................................................................... 2700/10375
.................................................................................................... 2800/10375
---
.................................................................................................... 5300/10375
.................................................................................................... 5400/10375
...................i................................................................................ 5500/10375
.............i...................................................................................... 5600/10375
.................................ii.ii........i...i................................................. 5700/10375
..i...............................................................................................i. 5900/10375
.................................................................................................... 6000/10375
....................................................ii.....................................i........ 6100/10375
.................................................................................................... 6200/10375
.................................................................................................... 6200/10375
.................................................................................................... 6300/10375
.................................................................................................... 6400/10375
...............ii...i..ii...........i............................................................... 6500/10375
.................................................................................................... 6700/10375
.................................................................................................... 6800/10375
.................................................................................................... 6800/10375
.................................................i..ii.............................................. 6900/10375
.................................................................................................... 7100/10375
.................................................................................................... 7200/10375
.....i.............................................................................................. 7300/10375
.................................................................................................... 7400/10375
---
.................................................................................................... 8300/10375
.................................................................................................... 8400/10375
..................................................i................................................. 8500/10375
.................................................................................................... 8600/10375
....iiiiii..iiiiii.i................................................................................ 8700/10375
.................................................................................................... 8900/10375
.................................................................................................... 9000/10375
.................................................................................................... 9100/10375
.................................................................................................... 9200/10375
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 197 tests
iiii......i................ii.i..........i......................i...........i..i........i........i.. 100/197
..i.............i.i.i...iii..iiii....................................iii.................ii......

 finished in 6.984
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiiiiiiiiiiiii

 finished in 0.158
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..Fi.......ii.i.ii. 100/116
....iiii.....ii.

---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8001000
NOTE: compiletest thinks it is using GDB version 8001000

error: line not found in debugger output: $9 = HashMap(size=4) = {[1] = 10, [2] = 20, [3] = 30, [4] = 40}
status: exit code: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
------------------------------------------
------------------------------------------
GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
Copyright (C) 2018 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x1f47f: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 140.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:140
140     zzz(); // #break
$1 = BTreeSet(size=15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
$2 = BTreeSet(size=0)
$3 = BTreeMap(size=15) = {[0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 5, [6] = 6, [7] = 7, [8] = 8, [9] = 9, [10] = 10, [11] = 11, [12] = 12, [13] = 13, [14] = 14}
$4 = BTreeMap(size=0)
$5 = BTreeMap(size=2) = {[false] = core::option::Option<bool>, [true] = core::option::Option<bool>}
$6 = BTreeMap(size=15) = {[0] = pretty_std_collections::MyLeafNode (0), [1] = pretty_std_collections::MyLeafNode (1), [2] = pretty_std_collections::MyLeafNode (2), [3] = pretty_std_collections::MyLeafNode (3), [4] = pretty_std_collections::MyLeafNode (4), [5] = pretty_std_collections::MyLeafNode (5), [6] = pretty_std_collections::MyLeafNode (6), [7] = pretty_std_collections::MyLeafNode (7), [8] = pretty_std_collections::MyLeafNode (8), [9] = pretty_std_collections::MyLeafNode (9), [10] = pretty_std_collections::MyLeafNode (10), [11] = pretty_std_collections::MyLeafNode (11), [12] = pretty_std_collections::MyLeafNode (12), [13] = pretty_std_collections::MyLeafNode (13), [14] = pretty_std_collections::MyLeafNode (14)}
$7 = VecDeque(size=3) = {5, 3, 7}
$8 = VecDeque(size=7) = {2, 3, 4, 5, 6, 7, 8}
$9 = std::collections::hash::map::HashMap<u64, u64, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>> {base: hashbrown::map::HashMap<u64, u64, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>> {hash_builder: core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher> (core::marker::PhantomData<pretty_std_collections::SimpleHasher>), table: hashbrown::raw::RawTable<(u64, u64)> {bucket_mask: 7, ctrl: core::ptr::non_null::NonNull<u8> {pointer: 0x5555557a3130 "\377\000"}, growth_left: 3, items: 4, marker: core::marker::PhantomData<(u64, u64)>}}}
$10 = std::collections::hash::set::HashSet<u64, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>> {map: std::collections::hash::map::HashMap<u64, (), core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>> {base: hashbrown::map::HashMap<u64, (), core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>> {hash_builder: core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher> (core::marker::PhantomData<pretty_std_collections::SimpleHasher>), table: hashbrown::raw::RawTable<(u64, ())> {bucket_mask: 7, ctrl: core::ptr::non_null::NonNull<u8> {pointer: 0x5555557a3090 "\377\000"}, growth_left: 3, items: 4, marker: core::marker::PhantomData<(u64, ())>}}}}


 Inferior 1 [process 10402] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
Python Exception <class 'gdb.error'> There is no member named data.: 
Python Exception <class 'gdb.error'> There is no member named data.: 
Python Exception <class 'gdb.error'> There is no member named data.: 
Python Exception <class 'gdb.error'> There is no member named data.: 
Python Exception <class 'gdb.error'> There is no member named data.: 
------------------------------------------




failures:
    [debuginfo-gdb] debuginfo/pretty-std-collections.rs

test result: FAILED. 77 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:347:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:09:49
== clock drift check ==
  local time: Sun Jun 21 21:18:14 UTC 2020
  network time: Sun, 21 Jun 2020 21:18:14 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/70052/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/70052/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3971) (python)
##[section]Finishing: Finalize Job
