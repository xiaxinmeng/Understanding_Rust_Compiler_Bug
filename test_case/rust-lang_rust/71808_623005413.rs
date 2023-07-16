plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c53469c9-71a4-4cb5-9e6f-15e6731b3df9.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71808/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71808/merge:refs/remotes/pull/71808/merge
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
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.................i.................................................................................. 1800/9966
.................................................................................................... 1900/9966
.................................i.................................................................. 2000/9966
.................................................................................................... 2100/9966
.......................iiiii........................................................................ 2200/9966
.................................................................................................... 2400/9966
.................................................................................................... 2500/9966
.................................................................................................... 2600/9966
.................................................................................................... 2700/9966
---
.......i...............i............................................................................ 5100/9966
.................................................................................................... 5200/9966
.....................................................i.............................................. 5300/9966
............................................i....................................................... 5400/9966
..............................................ii.ii........i...i.................................... 5500/9966
.............................................................................................i...... 5700/9966
.................................................................................................... 5800/9966
............................ii.....................................i................................ 5900/9966
.................................................................................................... 6000/9966
.................................................................................................... 6000/9966
.................................................................................................... 6100/9966
..............................................................ii...i..ii...........i................ 6200/9966
.................................................................................................... 6400/9966
.................................................................................................... 6500/9966
.................................................................................................... 6500/9966
..............................................................................................i..ii. 6600/9966
.................................................................................................... 6800/9966
...............................................................................................i.... 6900/9966
.................................................................................................... 7000/9966
.................................................................................................... 7100/9966
---
.................................................................................................... 7900/9966
.................................................................................................... 8000/9966
.................................................................................................... 8100/9966
.....i.............................................................................................. 8200/9966
......................................................iiiiii.iiiii.i................................ 8300/9966
.......i............................................................................................ 8500/9966
.................................................................................................... 8600/9966
.................................................................................................... 8700/9966
.................................................................................................... 8800/9966
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 186 tests
iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 5.055
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.140
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 14.369
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2500 tests
......iiiii......................................................................................... 100/2500
......................................................................................ii............ 200/2500
......................i............................................................................. 400/2500
............................................................................i..i..................ii 500/2500
ii.................................................................................................. 600/2500
.................................................................................................... 700/2500
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

 finished in 136.137
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.897
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-17c36066b7f316a1

running 0 tests

---
Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0539 (line 10277) stdout ----
error[E0734]: stability attributes may not be used outside of the standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10278:1
  |
3 | #[rustc_deprecated(reason)] // error!

error[E0734]: stability attributes may not be used outside of the standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10279:1
  |
  |
4 | #[unstable(feature = "deprecated_fn", issue = "123")]

error[E0734]: stability attributes may not be used outside of the standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10282:1
  |
  |
7 | #[unstable(feature = "unstable_struct", issue)] // error!

error[E0734]: stability attributes may not be used outside of the standard library
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10285:1
   |
---

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0734`.
Some expected error codes were not found: ["E0539"]
error[E0734]: stability attributes may not be used outside of the standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10299:1
  |
  |
3 | #[rustc_deprecated(since = "1.39.0", reason = "reason")] // ok!

error[E0734]: stability attributes may not be used outside of the standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10300:1
  |
  |
4 | #[unstable(feature = "deprecated_fn", issue = "123")]

error[E0734]: stability attributes may not be used outside of the standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10303:1
  |
  |
7 | #[unstable(feature = "unstable_struct", issue = "123")] // ok!

error[E0734]: stability attributes may not be used outside of the standard library
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10306:1
   |
   |
10 | #[rustc_const_unstable(feature = "unstable_fn", issue = "124")] // ok!

error[E0734]: stability attributes may not be used outside of the standard library
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10309:1
   |
   |
13 | #[stable(feature = "stable_struct", since = "1.39.0")] // ok!
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0734]: stability attributes may not be used outside of the standard library
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10312:1
   |
16 | #[rustc_const_stable(feature = "stable_fn", since = "1.39.0")] // ok!

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0734`.
---
  local time: Sat May  2 19:55:02 UTC 2020
  network time: Sat, 02 May 2020 19:55:02 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71808/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71808/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4835) (python)
##[section]Finishing: Finalize Job
