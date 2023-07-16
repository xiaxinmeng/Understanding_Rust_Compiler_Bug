plain
2019-08-14T05:14:52.2880722Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T05:14:52.3061265Z ##[command]git config gc.auto 0
2019-08-14T05:14:52.3131400Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T05:14:52.3189530Z ##[command]git config --get-all http.proxy
2019-08-14T05:14:52.3334863Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63531/merge:refs/remotes/pull/63531/merge
---
2019-08-14T05:15:27.9477828Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T05:15:27.9478042Z 
2019-08-14T05:15:27.9478470Z   git checkout -b <new-branch-name>
2019-08-14T05:15:27.9478660Z 
2019-08-14T05:15:27.9481265Z HEAD is now at 1aea4b90d Merge 58fe6835bdc1ddb80a3be81872c7f5759f4eda3c into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-14T05:15:27.9635804Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T05:15:27.9638244Z ==============================================================================
2019-08-14T05:15:27.9638296Z Task         : Bash
2019-08-14T05:15:27.9638338Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T06:19:10.5615259Z .................................................................................................... 1300/8876
2019-08-14T06:19:17.3504606Z .................................................................................................... 1400/8876
2019-08-14T06:19:23.8537211Z .................................................................................................... 1500/8876
2019-08-14T06:19:34.9042314Z ....................................................................................i............... 1600/8876
2019-08-14T06:19:42.9846088Z i................................................................................................... 1700/8876
2019-08-14T06:19:49.9461170Z ...........................................................................iiiii.................... 1800/8876
2019-08-14T06:20:13.1756801Z .................................................................................................... 2000/8876
2019-08-14T06:20:15.7348147Z .................................................................................................... 2100/8876
2019-08-14T06:20:18.5233328Z .................................................................................................... 2200/8876
2019-08-14T06:20:26.6186441Z .................................................................................................... 2300/8876
---
2019-08-14T06:24:29.7170595Z .................................................................................................... 5300/8876
2019-08-14T06:24:37.3203614Z ........i........................................................................................... 5400/8876
2019-08-14T06:24:43.0720713Z .................................................................................................... 5500/8876
2019-08-14T06:24:55.8306374Z .................................................................................................... 5600/8876
2019-08-14T06:25:13.0171616Z ...ii...i..ii...........i........................................................................... 5700/8876
2019-08-14T06:25:40.6819407Z .................................................................................................... 5900/8876
2019-08-14T06:25:46.5627180Z .................................................................................................... 6000/8876
2019-08-14T06:25:46.5627180Z .................................................................................................... 6000/8876
2019-08-14T06:26:01.7426334Z ....i..ii........................................................................................... 6100/8876
2019-08-14T06:26:22.4832905Z ...............................................i.................................................... 6300/8876
2019-08-14T06:26:24.8323485Z .................................................................................................... 6400/8876
2019-08-14T06:26:27.5863667Z ...................i................................................................................ 6500/8876
2019-08-14T06:26:32.6156389Z .................................................................................................... 6600/8876
---
2019-08-14T06:31:27.6528510Z  finished in 23.339
2019-08-14T06:31:27.6723546Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-14T06:31:27.8386232Z 
2019-08-14T06:31:27.8387050Z running 148 tests
2019-08-14T06:31:31.2118748Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/148
2019-08-14T06:31:33.2199075Z iii..............i......F..F.iii.i......ii......
2019-08-14T06:31:33.2203807Z 
2019-08-14T06:31:33.2204284Z ---- [codegen] codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs stdout ----
2019-08-14T06:31:33.2204882Z 
2019-08-14T06:31:33.2205964Z error: compilation failed!
2019-08-14T06:31:33.2205964Z error: compilation failed!
2019-08-14T06:31:33.2206027Z status: exit code: 101
2019-08-14T06:31:33.2207242Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/auxiliary" "--emit=llvm-ir"
2019-08-14T06:31:33.2207724Z ------------------------------------------
2019-08-14T06:31:33.2207782Z 
2019-08-14T06:31:33.2208038Z ------------------------------------------
2019-08-14T06:31:33.2208089Z stderr:
2019-08-14T06:31:33.2208089Z stderr:
2019-08-14T06:31:33.2208332Z ------------------------------------------
2019-08-14T06:31:33.2208678Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc/ty/layout.rs:749:25
2019-08-14T06:31:33.2208787Z 
2019-08-14T06:31:33.2208855Z error: internal compiler error: unexpected panic
2019-08-14T06:31:33.2208889Z 
2019-08-14T06:31:33.2208939Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-14T06:31:33.2208939Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-14T06:31:33.2208981Z 
2019-08-14T06:31:33.2209671Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-14T06:31:33.2210020Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-14T06:31:33.2210078Z 
2019-08-14T06:31:33.2210078Z 
2019-08-14T06:31:33.2210410Z note: compiler flags: -Z threads=1 -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 -C no-prepopulate-passes
2019-08-14T06:31:33.2210482Z 
2019-08-14T06:31:33.2210751Z ------------------------------------------
2019-08-14T06:31:33.2210786Z 
2019-08-14T06:31:33.2210816Z 
2019-08-14T06:31:33.2210816Z 
2019-08-14T06:31:33.2211106Z ---- [codegen] codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs stdout ----
2019-08-14T06:31:33.2211162Z 
2019-08-14T06:31:33.2211207Z error: compilation failed!
2019-08-14T06:31:33.2211256Z status: exit code: 101
2019-08-14T06:31:33.2212954Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/auxiliary" "--emit=llvm-ir"
2019-08-14T06:31:33.2213444Z ------------------------------------------
2019-08-14T06:31:33.2213485Z 
2019-08-14T06:31:33.2213738Z ------------------------------------------
2019-08-14T06:31:33.2213787Z stderr:
2019-08-14T06:31:33.2213787Z stderr:
2019-08-14T06:31:33.2214051Z ------------------------------------------
2019-08-14T06:31:33.2214370Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc/ty/layout.rs:749:25
2019-08-14T06:31:33.2214614Z 
2019-08-14T06:31:33.2214664Z error: internal compiler error: unexpected panic
2019-08-14T06:31:33.2214697Z 
2019-08-14T06:31:33.2214746Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-14T06:31:33.2214746Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-14T06:31:33.2214796Z 
2019-08-14T06:31:33.2215207Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-14T06:31:33.2215556Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-14T06:31:33.2215594Z 
2019-08-14T06:31:33.2215594Z 
2019-08-14T06:31:33.2215922Z note: compiler flags: -Z threads=1 -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 -C no-prepopulate-passes
2019-08-14T06:31:33.2216013Z 
2019-08-14T06:31:33.2216262Z ------------------------------------------
2019-08-14T06:31:33.2216297Z 
2019-08-14T06:31:33.2216326Z 
---
2019-08-14T06:31:33.2217448Z test result: FAILED. 115 passed; 2 failed; 31 ignored; 0 measured; 0 filtered out
2019-08-14T06:31:33.2217489Z 
2019-08-14T06:31:33.2217517Z 
2019-08-14T06:31:33.2217544Z 
2019-08-14T06:31:33.2219251Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-14T06:31:33.2219528Z 
2019-08-14T06:31:33.2219577Z 
2019-08-14T06:31:33.2225226Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-14T06:31:33.2225470Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-14T06:31:33.2225470Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-14T06:31:33.2229083Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-14T06:31:33.2229157Z Build completed unsuccessfully in 1:09:30
2019-08-14T06:31:36.1517538Z ##[error]Bash exited with code '1'.
2019-08-14T06:31:36.1560770Z ##[section]Starting: Checkout
2019-08-14T06:31:36.1563336Z ==============================================================================
2019-08-14T06:31:36.1563403Z Task         : Get sources
2019-08-14T06:31:36.1563454Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
