plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fae70460-ef9f-41fe-8ead-4a8b2b44d001.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71909/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71909/merge:refs/remotes/pull/71909/merge
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
...................i................................................................................ 1800/9986
.................................................................................................... 1900/9986
....................................i............................................................... 2000/9986
.................................................................................................... 2100/9986
..........................iiiii..................................................................... 2200/9986
.................................................................................................... 2400/9986
.................................................................................................... 2500/9986
.................................................................................................... 2600/9986
.................................................................................................... 2700/9986
---
...............i...............i.................................................................... 5100/9986
.................................................................................................... 5200/9986
.............................................................i...................................... 5300/9986
....................................................i............................................... 5400/9986
........................................................ii.ii........i...i.......................... 5500/9986
.................................................................................................i.. 5600/9986
...i................................................................................................ 5800/9986
.......................................ii.....................................i..................... 5900/9986
.................................................................................................... 6000/9986
.................................................................................................... 6100/9986
.................................................................................................... 6100/9986
...........................................................................ii...i..ii...........i... 6200/9986
.................................................................................................... 6400/9986
.................................................................................................... 6500/9986
.................................................................................................... 6600/9986
.................................................................................................... 6600/9986
.......i..ii........................................................................................ 6700/9986
.................................................................................................... 6900/9986
........i........................................................................................... 7000/9986
.................................................................................................... 7100/9986
..................................................i................................................. 7200/9986
---
.................................................................................................... 7900/9986
.................................................................................................... 8000/9986
.................................................................................................... 8100/9986
..................i................................................................................. 8200/9986
........................................................................iiiiii.iiiii.i.............. 8300/9986
.........................i.......................................................................... 8500/9986
.................................................................................................... 8600/9986
.................................................................................................... 8700/9986
.................................................................................................... 8800/9986
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 186 tests
iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 5.444
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.149
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 14.574
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2504 tests
......iiiii......................................................................................... 100/2504
.......................................................................................ii........... 200/2504
.......................i............................................................................ 400/2504
.............................................................................i..i..................i 500/2504
iii................................................................................................. 600/2504
.................................................................................................... 700/2504
---
---- option.rs - option::Option<&'a T>::from (line 1387) stdout ----
error[E0277]: `std::option::Option<std::string::String>` doesn't implement `std::fmt::Display`
 --> option.rs:1390:35
  |
6 | println!("Can still print s: {}", s);
  |                                   ^ `std::option::Option<std::string::String>` cannot be formatted with the default formatter
  = help: the trait `std::fmt::Display` is not implemented for `std::option::Option<std::string::String>`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  = note: required by `std::fmt::Display::fmt`
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
  local time: Tue May  5 01:47:33 UTC 2020
  network time: Tue, 05 May 2020 01:47:34 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71909/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71909/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4321) (python)
##[section]Finishing: Finalize Job
