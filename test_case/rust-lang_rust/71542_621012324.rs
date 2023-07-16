plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1d2c856c-5de8-493e-a2e6-b8916d35664a.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71542/merge:refs/remotes/pull/71542/merge
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
Looks like docker image is the same as before, not uploading
[CI_JOB_NAME=x86_64-gnu-llvm-8]
[CI_JOB_NAME=x86_64-gnu-llvm-8]
== clock drift check ==
  local time: Wed Apr 29 05:17:35 UTC 2020
  network time: Wed, 29 Apr 2020 05:17:36 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.dist-src        := False
---
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 52.48s
{"reason":"build-finished","success":true}
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cfg-if v0.1.10
   Compiling libc v0.2.69
   Compiling semver-parser v0.7.0
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
   Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
   Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
   Compiling rustc-main v0.0.0 (/checkout/src/rustc)
    Finished release [optimized] target(s) in 18m 30s
{"reason":"build-finished","success":true}
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.50
   Compiling core v0.0.0 (/checkout/src/libcore)
---
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 1m 00s
{"reason":"build-finished","success":true}
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cfg-if v0.1.10
   Compiling libc v0.2.69
   Compiling semver-parser v0.7.0
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
   Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
   Compiling rustc-main v0.0.0 (/checkout/src/rustc)
    Finished release [optimized] target(s) in 19m 23s
{"reason":"build-finished","success":true}
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building test helpers
---
   Compiling serde_derive v1.0.81
   Compiling serde_json v1.0.40
   Compiling rustfix v0.5.0
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
{"reason":"build-finished","success":true}
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9940 tests
.................................................................................................... 100/9940
---
.................................................................................................... 1800/9940
.................................................................................................... 1900/9940
..............i..................................................................................... 2000/9940
.................................................................................................... 2100/9940
....iiiii........................................................................................... 2200/9940
.................................................................................................... 2400/9940
.................................................................................................... 2500/9940
.................................................................................................... 2600/9940
.................................................................................................... 2700/9940
---
..i................................................................................................. 5100/9940
.................................................................................................... 5200/9940
................................i................................................................... 5300/9940
.......................i............................................................................ 5400/9940
........................ii.ii........i...i.......................................................... 5500/9940
.......................................................................i............................ 5700/9940
.................................................................................................... 5800/9940
.....ii.....................................i....................................................... 5900/9940
.................................................................................................... 6000/9940
.................................................................................................... 6000/9940
.................................................................................................... 6100/9940
......................................ii...i..ii...........i........................................ 6200/9940
.................................................................................................... 6400/9940
.................................................................................................... 6500/9940
.................................................................................................... 6500/9940
....................................................................i..ii........................... 6600/9940
.................................................................................................... 6800/9940
.....................................................................i.............................. 6900/9940
.................................................................................................... 7000/9940
.................................................................................................... 7100/9940
---
.................................................................................................... 7900/9940
.................................................................................................... 8000/9940
...............................................................................i.................... 8100/9940
.................................................................................................... 8200/9940
............................iiiiii.iiiii.i.......................................................... 8300/9940
.................................................................................................... 8500/9940
.................................................................................................... 8600/9940
.................................................................................................... 8700/9940
.................................................................................................... 8800/9940
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 186 tests
iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 4.689
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.121
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 13.179
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
     |                              ^
     | 
    ::: src/librustdoc/clean/types.rs:347:6
     |
347  | impl<I: IntoIterator<Item = ast::NestedMetaItem>> NestedAttributesExt for I {
     |      - this is where the previous identifier occurred
     = note: `#[warn(confusable_idents)]` on by default


warning: identifier pair considered confusable between `r1` and `rl`
    |
20  | use rustc_middle::middle::resolve_lifetime as rl;
    |                                               ^^
    | 
    | 
   ::: src/librustdoc/clean/auto_trait.rs:195:40
    |
195 |                 &Constraint::VarSubVar(r1, r2) => {
    |                                        -- this is where the previous identifier occurred
warning: 2 warnings emitted

   Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
    Finished release [optimized] target(s) in 2m 26s
---
    |             ^
    | 
   ::: src/liballoc/../liballoc/tests/arc.rs:97:23
    |
97  | fn assert_trusted_len<I: TrustedLen>(_: &I) {}
    |                       - this is where the previous identifier occurred
    = note: `-D confusable-idents` implied by `-D warnings`

error: aborting due to previous error

---

failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:01:11
== clock drift check ==
  local time: Wed Apr 29 06:20:10 UTC 2020
  network time: Wed, 29 Apr 2020 06:20:10 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71542/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3771) (python)
##[section]Finishing: Finalize Job
