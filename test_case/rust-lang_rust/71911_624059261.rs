plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dcf2d173-fe24-4eb9-8361-322c7a8cc6da.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71911/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71911/merge:refs/remotes/pull/71911/merge
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
......................i............................................................................. 1800/9987
.................................................................................................... 1900/9987
.......................................i............................................................ 2000/9987
.................................................................................................... 2100/9987
.............................iiiii.................................................................. 2200/9987
.................................................................................................... 2400/9987
.................................................................................................... 2500/9987
.................................................................................................... 2600/9987
.................................................................................................... 2700/9987
---
................i...............i................................................................... 5100/9987
.................................................................................................... 5200/9987
..............................................................i..................................... 5300/9987
.....................................................i.............................................. 5400/9987
.........................................................ii.ii........i...i......................... 5500/9987
..................................................................................................i. 5600/9987
....i............................................................................................... 5800/9987
........................................ii.....................................i.................... 5900/9987
.................................................................................................... 6000/9987
.................................................................................................... 6100/9987
.................................................................................................... 6100/9987
............................................................................ii...i..ii...........i.. 6200/9987
.................................................................................................... 6400/9987
.................................................................................................... 6500/9987
.................................................................................................... 6600/9987
.................................................................................................... 6600/9987
........i..ii....................................................................................... 6700/9987
.................................................................................................... 6900/9987
.........i.......................................................................................... 7000/9987
.................................................................................................... 7100/9987
...................................................i................................................ 7200/9987
---
.................................................................................................... 7900/9987
.................................................................................................... 8000/9987
.................................................................................................... 8100/9987
...................i................................................................................ 8200/9987
.........................................................................iiiiii.iiiii.i............. 8300/9987
..........................i......................................................................... 8500/9987
.................................................................................................... 8600/9987
.................................................................................................... 8700/9987
.................................................................................................... 8800/9987
---
running 96 tests
...............................................................................F................
failures:

---- [mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
27           _0 = const std::option::Option::<std::boxed::Box<()>>::None; // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:3:17: 3:21
28                                            // ty::Const
29                                            // + ty: std::option::Option<std::boxed::Box<()>>
-                                            // + val: Value(Scalar(0x00000000))
+                                            // + val: Value(Scalar(0x0000000000000000))
31                                            // mir::Constant
32                                            // + span: $DIR/simplify-locals-removes-unused-discriminant-reads.rs:3:17: 3:21
-                                            // + literal: Const { ty: std::option::Option<std::boxed::Box<()>>, val: Value(Scalar(0x00000000)) }
+                                            // + literal: Const { ty: std::option::Option<std::boxed::Box<()>>, val: Value(Scalar(0x0000000000000000)) }
34           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:2:5: 5:6
36   


thread '[mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads/rustc.map.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3166:25


failures:
    [mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs
    [mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs

test result: FAILED. 95 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:55:48
Build completed unsuccessfully in 0:55:48
== clock drift check ==
  local time: Tue May  5 13:35:46 UTC 2020
  network time: Tue, 05 May 2020 13:35:46 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71911/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71911/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4168) (python)
##[section]Finishing: Finalize Job
