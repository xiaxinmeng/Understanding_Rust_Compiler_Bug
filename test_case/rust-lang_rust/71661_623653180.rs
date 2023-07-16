plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/825475b9-704c-4df9-8454-d298448874d9.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71661/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71661/merge:refs/remotes/pull/71661/merge
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
..................i................................................................................. 1800/9983
.................................................................................................... 1900/9983
...................................i................................................................ 2000/9983
.................................................................................................... 2100/9983
.........................iiiii...................................................................... 2200/9983
.................................................................................................... 2400/9983
.................................................................................................... 2500/9983
.................................................................................................... 2600/9983
.................................................................................................... 2700/9983
---
.............i...............i...................................................................... 5100/9983
.................................................................................................... 5200/9983
...........................................................i........................................ 5300/9983
..................................................i................................................. 5400/9983
......................................................ii.ii........i...i............................ 5500/9983
.................................................................................................... 5700/9983
.i.................................................................................................. 5800/9983
.....................................ii.....................................i....................... 5900/9983
.................................................................................................... 6000/9983
.................................................................................................... 6000/9983
.................................................................................................... 6100/9983
........................................................................ii...i..ii...........i...... 6200/9983
.................................................................................................... 6400/9983
.................................................................................................... 6500/9983
.................................................................................................... 6600/9983
.................................................................................................... 6600/9983
....i..ii........................................................................................... 6700/9983
.................................................................................................... 6900/9983
.....i.............................................................................................. 7000/9983
.................................................................................................... 7100/9983
...............................................i.................................................... 7200/9983
---
.................................................................................................... 7900/9983
.................................................................................................... 8000/9983
.................................................................................................... 8100/9983
...............i.................................................................................... 8200/9983
.....................................................................iiiiii.iiiii.i................. 8300/9983
......................i............................................................................. 8500/9983
.................................................................................................... 8600/9983
.................................................................................................... 8700/9983
.................................................................................................... 8800/9983
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 186 tests
iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
........i.i.i...iii..iiiii.iiiiiiiiiii......................iii...............ii......

 finished in 6.113
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.167
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 15.556
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2501 tests
......iiiii......................................................................................... 100/2501
.......................................................................................ii........... 200/2501
.......................i............................................................................ 400/2501
.............................................................................i..i..................i 500/2501
iii................................................................................................. 600/2501
.................................................................................................... 700/2501
---

running 1022 tests
i......F............................................................................................ 100/1022
.................................................................................................... 200/1022
.....................iii......i......i...i......i................................................... 300/1022
......................................................i....i......................................ii 500/1022
.................................................................................................... 600/1022
.................................................................................................... 700/1022
.................................................................................................... 700/1022
................................................iiii................................................ 800/1022
.................................................................................................... 900/1022
......................................................................iiii.......................... 1000/1022
failures:

---- collections/hash/map.rs - collections::hash::map::Entry::occupied (line 2058) stdout ----
---- collections/hash/map.rs - collections::hash::map::Entry::occupied (line 2058) stdout ----
error[E0596]: cannot borrow `o` as mutable, as it is not declared as mutable
  --> collections/hash/map.rs:2065:55
   |
10 | let maybe_replaced = map.entry(42).occupied().map(|o| o.insert("hee hee"));
   |                                                    -  ^ cannot borrow as mutable
   |                                                    help: consider changing this to be mutable: `mut o`

error: aborting due to previous error

---
  local time: Mon May  4 19:16:16 UTC 2020
  network time: Mon, 04 May 2020 19:16:17 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71661/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71661/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3770) (python)
##[section]Finishing: Finalize Job
