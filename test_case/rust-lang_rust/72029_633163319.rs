plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az578'
Current agent version: '2.169.0'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b2749ab5-6b7a-4f53-8e80-f78d6e1a608e.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72029/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72029/merge:refs/remotes/pull/72029/merge
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
...........................................................................i........................ 1800/10220
.................................................................................................... 1900/10220
..............................................................................................i..i.. 2000/10220
.................................................................................................... 2100/10220
....................................................................................iiiii........... 2200/10220
.................................................................................................... 2400/10220
.................................................................................................... 2500/10220
.................................................................................................... 2600/10220
.................................................................................................... 2700/10220
---
...........i...............i........................................................................ 5200/10220
.................................................................................................... 5300/10220
..........................................................i......................................... 5400/10220
...................................................i................................................ 5500/10220
..............................................................ii.ii........i...i.................... 5600/10220
....i............................................................................................... 5800/10220
............i....................................................................................... 5900/10220
................................................................ii.................................. 6000/10220
...i................................................................................................ 6100/10220
...i................................................................................................ 6100/10220
.................................................................................................... 6200/10220
.................................................................................................... 6300/10220
.........................ii...i..ii...........i..................................................... 6400/10220
.................................................................................................... 6600/10220
.................................................................................................... 6700/10220
.................................................................................................... 6700/10220
..........................................................i..ii..................................... 6800/10220
.................................................................................................... 7000/10220
.................................................................................................... 7100/10220
............i....................................................................................... 7200/10220
.................................................................................................... 7300/10220
---
.................................................................................................... 8100/10220
.................................................................................................... 8200/10220
.................................................................................................... 8300/10220
.....................................i.............................................................. 8400/10220
...........................................................................................iiiiii.ii 8500/10220
iiii.i.............................................................................................. 8600/10220
.................................................................................................... 8800/10220
.................................................................................................... 8900/10220
.................................................................................................... 9000/10220
.................................................................................................... 9100/10220
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 191 tests
iiii......i..............ii.i..........i.................................i..i................i....i. 100/191
............i.i.i...iii..iiiii.iiiiiiiiiii......................iii................ii......

 finished in 6.076
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 20 tests
iiiiiiiiiiiiiiiiiiii

 finished in 0.158
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 15.188
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["srcUplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2568 tests
......iiiii......................................................................................... 100/2568
.................................................................................................ii. 200/2568
...................................i................................................................ 400/2568
.........................................................................................i..i....... 500/2568
.........................................................................................i..i....... 500/2568
...........iiii..................................................................................... 600/2568
.................................................................................................... 800/2568
.................................................................................................... 900/2568
.................................................................................................... 1000/2568
.................................................................................................... 1100/2568
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

 finished in 158.400
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 1.039
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-badfa60cf5f683cd

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
Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 211 tests
......................i...ii.......................................................................i 100/211
........................................iiiiii......i..............iii.............................. 200/211
.......ii..

 finished in 71.001
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
  |
2 | #![feature(unsized_locals)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #48055 <***/issues/48055> for more information
error[E0277]: the size for values of type `(dyn std::any::Any + 'static)` cannot be known at compilation time
  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:23:5
   |
11 |     foo(x);
---

error[E0277]: the size for values of type `(dyn std::any::Any + 'static)` cannot be known at compilation time
  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:26:8
   |
14 | fn foo(_: dyn Any) {}
   |        ^ doesn't have a size known at compile-time
   = help: the trait `std::marker::Sized` is not implemented for `(dyn std::any::Any + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: all function arguments must have a statically known size
   = help: unsized fn params are gated as an unstable feature
   = help: unsized fn params are gated as an unstable feature

error: aborting due to 2 previous errors; 1 warning emitted
For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - The_tracking_issue_for_this_feature_is__::By_value_trait_objects (line 104) stdout ----
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
 --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:105:12
  |
2 | #![feature(unsized_locals)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #48055 <***/issues/48055> for more information
error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:108:12
  |
5 |     fn foo(self) {}
---
  = note: all function arguments must have a statically known size
  = help: unsized fn params are gated as an unstable feature
help: consider further restricting `Self`
  |
5 |     fn foo(self) where Self: std::marker::Sized {}

error[E0277]: the size for values of type `dyn Foo` cannot be known at compilation time
  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:116:5
   |
   |
13 |     <dyn Foo as Foo>::foo(*slice);
   |
   = help: the trait `std::marker::Sized` is not implemented for `dyn Foo`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: all function arguments must have a statically known size
   = note: all function arguments must have a statically known size
   = help: unsized fn params are gated as an unstable feature

error: aborting due to 2 previous errors; 1 warning emitted
For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - The_tracking_issue_for_this_feature_is__::By_value_trait_objects (line 87) stdout ----
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
 --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:88:12
  |
2 | #![feature(unsized_locals)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #48055 <***/issues/48055> for more information
error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:91:12
  |
5 |     fn foo(self) {}
---
  = note: all function arguments must have a statically known size
  = help: unsized fn params are gated as an unstable feature
help: consider further restricting `Self`
  |
5 |     fn foo(self) where Self: std::marker::Sized {}

error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:98:5
   |
   |
12 |     <[i32] as Foo>::foo(*slice);
   |
   = help: the trait `std::marker::Sized` is not implemented for `[i32]`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: all function arguments must have a statically known size
   = note: all function arguments must have a statically known size
   = help: unsized fn params are gated as an unstable feature

error: aborting due to 2 previous errors; 1 warning emitted
For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.

failures:
---
  local time: Sun May 24 01:25:38 UTC 2020
  network time: Sun, 24 May 2020 01:25:38 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72029/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72029/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3287) (python)
##[section]Finishing: Finalize Job
