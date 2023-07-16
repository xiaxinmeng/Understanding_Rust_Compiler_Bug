plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3dc5c96c-d372-4a84-94c7-abb630b03773.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73500/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73500/merge:refs/remotes/pull/73500/merge
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
.................................................................................................... 1900/10324
.................................................................................................... 2000/10324
................i..i................................................................................ 2100/10324
.................................................................................................... 2200/10324
......iiiii......................................................................................... 2300/10324
.................................................................................................... 2500/10324
.................................................................................................... 2600/10324
.................................................................................................... 2700/10324
.................................................................................................... 2800/10324
---
.................................................................................................... 5300/10324
........................................................................................i........... 5400/10324
..................................................................................i................. 5500/10324
.................................................................................................... 5600/10324
.ii.ii........i...i................................................................................. 5700/10324
.......................................................i............................................ 5900/10324
.................................................................................................... 6000/10324
.........ii.....................................i................................................... 6100/10324
.................................................................................................... 6200/10324
.................................................................................................... 6200/10324
.................................................................................................... 6300/10324
........................................................................ii...i..ii...........i...... 6400/10324
.................................................................................................... 6600/10324
.................................................................................................... 6700/10324
.................................................................................................... 6800/10324
.................................................................................................... 6800/10324
......i..ii......................................................................................... 6900/10324
.................................................................................................... 7100/10324
.............................................................i...................................... 7200/10324
.................................................................................................... 7300/10324
.................................................................................................... 7400/10324
---
.................................................................................................... 8200/10324
.................................................................................................... 8300/10324
.................................................................................................... 8400/10324
....i............................................................................................... 8500/10324
..........................................................iiiiii.iiiiii..i.......................... 8600/10324
...............i.................................................................................... 8800/10324
.................................................................................................... 8900/10324
.................................................................................................... 9000/10324
.................................................................................................... 9100/10324
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 195 tests
iiii......i...............ii.i..........i......................i...........i..i................i.... 100/195
i.............i.i.i...iii..iiiiiiiiiiiiiiiii.......................iii.................ii......

 finished in 4.809
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiiiiiiiiiiii

 finished in 0.132
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii. 100/116
....iiii.....ii.

 finished in 13.202
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.744
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
......................i...ii...................................................................F.... 100/212
i........................................iiiiii......i..............iii..................F.......... 200/212
........ii..

---- [run-make] run-make-fulldeps/issue-69368 stdout ----

error: make failed
error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368  a.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368  b.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368  c.rs
Makefile:16: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
error: linking with `cc` failed: exit code: 1
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.13.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.14.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.15.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.c.7rcbfp3g-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/c.2rnwddj6nqg1m891.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/libb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-69368/issue-69368/liba.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-1a23e2695794a0f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-38715a9db39d4c85.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-09df22d4231a8f0a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-209edba2a7b638c9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-7c3e9aa2914b892e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-56a274f083f88017.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-309455f80c28d8d6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-41db7bc7c5392854.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-ldl" "-lutil"
  = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-7c3e9aa2914b892e.rlib(alloc-7c3e9aa2914b892e.alloc.etdgpvh6-cgu.0.rcgu.o): In function `alloc::alloc::handle_alloc_error':
          alloc.etdgpvh6-cgu.0:(.text._ZN5alloc5alloc18handle_alloc_error17h8973a748905ce329E+0x3): undefined reference to `rust_oom'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-309455f80c28d8d6.rlib(core-309455f80c28d8d6.core.bomhagm8-cgu.0.rcgu.o): In function `core::panicking::panic_fmt':
          core.bomhagm8-cgu.0:(.text._ZN4core9panicking9panic_fmt17h4f7b7d668688bbefE+0x2d): undefined reference to `rust_begin_unwind'
          collect2: error: ld returned 1 exit status

error: aborting due to previous error


make: *** [all] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/std-core-cycle stdout ----
---- [run-make] run-make-fulldeps/std-core-cycle stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle  bar.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle  foo.rs -C link-args=-Wl,--no-undefined
Makefile:14: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
error: linking with `cc` failed: exit code: 1
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.foo.3a1fbbbh-cgu.8.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/libfoo.so" "-Wl,--version-script=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/rustcIpYx2f/list" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/foo.4a2a78br05n0rjfc.rcgu.o" "-Wl,--gc-sections" "-shared" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/std-core-cycle/std-core-cycle/libbar.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-3f64b6bfeb99d131.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-1a23e2695794a0f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-51b5456956acd847.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-1d711dd6b6810085.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-7396f51583e9660b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-b4bbdf0a3532a6a4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-c7632167461a5d02.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-38715a9db39d4c85.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-209edba2a7b638c9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-09df22d4231a8f0a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-7c3e9aa2914b892e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-56a274f083f88017.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-309455f80c28d8d6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-41db7bc7c5392854.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-ldl" "-lutil" "-Wl,--no-undefined"
  = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-309455f80c28d8d6.rlib(core-309455f80c28d8d6.core.bomhagm8-cgu.0.rcgu.o): In function `core::panicking::panic_fmt':
          core.bomhagm8-cgu.0:(.text._ZN4core9panicking9panic_fmt17h4f7b7d668688bbefE+0x2d): undefined reference to `rust_begin_unwind'
          collect2: error: ld returned 1 exit status

error: aborting due to previous error


make: *** [all] Error 1
------------------------------------------



---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430asmprinter msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-8/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:17:15
Build completed unsuccessfully in 1:17:15
== clock drift check ==
  local time: Fri Jun 19 11:26:19 UTC 2020
  network time: Fri, 19 Jun 2020 11:26:19 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73500/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73500/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3550) (python)
##[section]Finishing: Finalize Job
