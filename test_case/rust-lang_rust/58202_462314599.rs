plain
travis_time:end:08d6915c:start=1549885160560376841,finish=1549885268714441333,duration=108154064492
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:26] .................................................................................................... 500/5381
[00:59:29] ..............................i..................................................................... 600/5381
[00:59:33] .................................................................................................... 700/5381
[00:59:38] .................................................................................................... 800/5381
[00:59:43] .................................................................................i...............i.. 900/5381
[00:59:47] .....................F.............................................................................. 1000/5381
[00:59:50] .........iiiii...................................................................................... 1100/5381
[00:59:55] .................................................................................................... 1300/5381
[00:59:58] .................................................................................................... 1400/5381
[01:00:01] .................................................................................................... 1500/5381
[01:00:04] .................................................................................................... 1600/5381
---
[01:02:26] 
[01:02:26] ---- [ui] ui/deprecation/rustc_deprecation-in-future.rs stdout ----
[01:02:26] diff of stderr:
[01:02:26] 
[01:02:26] 1 error: use of item 'S' that will be deprecated in future version 99.99.99: effectively never
[01:02:26] 3    |
[01:02:26] 3    |
[01:02:26] - LL |     let _ = S; //~ ERROR  use of item 'S' that will be deprecated in future version 99.99.99: effectively never
[01:02:26] + LL |     let _ = S; //~ ERROR use of item 'S' that will be deprecated in future version 99.99.99: effectively never
[01:02:26] 6    |
[01:02:26] 6    |
[01:02:26] precation-in-future.rs","byte_start":341,"byte_end":342,"line_start":14,"line_end":14,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"    let _ = S; //~ ERROR use of item 'S' that will be deprecated in future version 99.99.99: effectively never","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/deprecation/rustc_deprecation-in-future.rs","byte_start":35,"byte_end":55,"line_start":3,"line_end":3,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![deny(deprecated_in_future)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of item 'S' that will be deprecated in future version 99.99.99: effectively never\n  --> /checkout/src/test/ui/deprecation/rustc_deprecation-in-future.rs:14:13\n   |\nLL |     let _ = S; //~ ERROR use of item 'S' that will be deprecated in future version 99.99.99: effectively never\n   |             ^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/deprecation/rustc_deprecation-in-future.rs:3:9\n   |\nLL | #![deny(deprecated_in_future)]\n   |         ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:02:26] 
[01:02:26] ------------------------------------------
[01:02:26] 
[01:02:26] thread '[ui] ui/deprecation/rustc_deprecation-in-future.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:02:26] 
[01:02:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:02:26] 
[01:02:26] 
[01:02:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:26] 
[01:02:26] 
[01:02:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:26] Build completed unsuccessfully in 0:04:16
[01:02:26] Build completed unsuccessfully in 0:04:16
[01:02:26] Makefile:48: recipe for target 'check' failed
[01:02:26] make: *** [check] Error 1
2651656 ./obj
2651616 ./obj/build
1980992 ./obj/build/x86_64-unknown-linux-gnu
1346608 ./src
---
151304 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
151300 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
147808 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
141200 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e
141196 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e/s-f9dzj1r959-z5nhvo-170x7pprjykmo
137064 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
134288 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123292 ./src/llvm-project/llvm/test/CodeGen
108528 ./src/llvm-project/lldb
