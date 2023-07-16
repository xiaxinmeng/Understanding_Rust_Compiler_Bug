plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/da083453-20dc-4478-90df-111018c3166c.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71540/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71540/merge:refs/remotes/pull/71540/merge
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
  local time: Wed Apr 29 04:06:10 UTC 2020
  network time: Wed, 29 Apr 2020 04:06:10 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.dist-src        := False
---
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 1m 02s
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
   Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
   Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
   Compiling rustc-main v0.0.0 (/checkout/src/rustc)
{"reason":"build-finished","success":true}
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.50
---
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 1m 13s
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
   Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
   Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
   Compiling rustc-main v0.0.0 (/checkout/src/rustc)
    Finished release [optimized] target(s) in 23m 14s
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

running 9941 tests
.................................................................................................... 100/9941
---
.................................................................................................... 1800/9941
.................................................................................................... 1900/9941
..............i..................................................................................... 2000/9941
.................................................................................................... 2100/9941
....iiiii........................................................................................... 2200/9941
.................................................................................................... 2400/9941
.................................................................................................... 2500/9941
.................................................................................................... 2600/9941
.................................................................................................... 2700/9941
---
....i............................................................................................... 5100/9941
.................................................................................................... 5200/9941
..................................i................................................................. 5300/9941
.........................i.......................................................................... 5400/9941
..........................ii.ii........i...i........................................................ 5500/9941
.........................................................................i.......................... 5700/9941
.................................................................................................... 5800/9941
......ii.....................................i...................................................... 5900/9941
.................................................................................................... 6000/9941
.................................................................................................... 6000/9941
.................................................................................................... 6100/9941
.......................................ii...i..ii...........i....................................... 6200/9941
.................................................................................................... 6400/9941
.................................................................................................... 6500/9941
.................................................................................................... 6500/9941
.....................................................................i..ii.......................... 6600/9941
.................................................................................................... 6800/9941
......................................................................i............................. 6900/9941
.................................................................................................... 7000/9941
.................................................................................................... 7100/9941
---
.................................................................................................... 7900/9941
.................................................................................................... 8000/9941
................................................................................i................... 8100/9941
.................................................................................................... 8200/9941
..............................iiiiiiiiiii.i......................................................... 8300/9941
.................................................................................................... 8500/9941
.................................................................................................... 8600/9941
.................................................................................................... 8700/9941
.................................................................................................... 8800/9941
---
---- [ui] ui/issues/issue-32122-1.rs stdout ----

error: fixed code is still producing diagnostics
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32122-1.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32122-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32122-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused variable: `b`
  --> /checkout/src/test/ui/issues/issue-32122-1.fixed:16:9
   |
LL |     let b: *const u8 = &*a; //~ ERROR mismatched types
   |         ^ help: if this is intentional, prefix it with an underscore: `_b`
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

---
---- [ui] ui/issues/issue-32122-2.rs stdout ----

error: failed to compile fixed code
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32122-2.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32122-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32122-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-32122-2.fixed:27:24
   |
LL |     let b: *const u8 = &*a; //~ ERROR mismatched types
   |            |           |
   |            |           expected `u8`, found struct `Foo`
   |            |           expected `u8`, found struct `Foo`
   |            |           help: consider using `&*` rather than `&`: `&**a`
   |
   |
   = note: expected raw pointer `*const u8`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:01:44
Build completed unsuccessfully in 1:01:44
== clock drift check ==
  local time: Wed Apr 29 05:09:32 UTC 2020
  network time: Wed, 29 Apr 2020 05:09:32 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71540/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71540/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4244) (python)
##[section]Finishing: Finalize Job
