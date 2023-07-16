plain
2019-11-22T18:13:30.9806490Z test [codegen] codegen/remap_path_prefix/xcrate-generic.rs ... ok
2019-11-22T18:13:31.0209740Z test [codegen] codegen/repeat-trusted-len.rs ... ok
2019-11-22T18:13:31.0892010Z test [codegen] codegen/repr-transparent-sysv64.rs ... ok
2019-11-22T18:13:31.1194440Z test [codegen] codegen/repr-transparent-aggregates-1.rs ... ok
2019-11-22T18:13:31.1217100Z test [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-0 ... FAILED
2019-11-22T18:13:31.1798800Z test [codegen] codegen/repr-transparent.rs ... ok
2019-11-22T18:13:31.2000250Z test [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-1 ... FAILED
2019-11-22T18:13:31.2302080Z test [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-2 ... FAILED
2019-11-22T18:13:31.2904570Z test [codegen] codegen/sanitizer-recover.rs#ASAN ... ok
2019-11-22T18:13:31.3106080Z test [codegen] codegen/sanitizer-recover.rs#MSAN ... FAILED
2019-11-22T18:13:31.3407880Z test [codegen] codegen/sanitizer-recover.rs#MSAN-RECOVER ... FAILED
2019-11-22T18:13:31.3409640Z test [codegen] codegen/sanitizer-recover.rs#ASAN-RECOVER ... ok
2019-11-22T18:13:31.5810280Z test [codegen] codegen/simd-intrinsic/simd-intrinsic-float-abs.rs ... ok
2019-11-22T18:13:31.6143950Z test [codegen] codegen/simd-intrinsic/simd-intrinsic-float-ceil.rs ... ok
2019-11-22T18:13:31.6244670Z test [codegen] codegen/simd-intrinsic/simd-intrinsic-float-cos.rs ... ok
2019-11-22T18:13:31.7167540Z test [codegen] codegen/simd-intrinsic/simd-intrinsic-float-exp.rs ... ok
---
2019-11-22T18:13:33.8658700Z test [codegen] codegen/zip.rs ... ok
2019-11-22T18:13:33.8658920Z 
2019-11-22T18:13:33.8659050Z failures:
2019-11-22T18:13:33.8659100Z 
2019-11-22T18:13:33.8661940Z ---- [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-0 stdout ----
2019-11-22T18:13:33.8662130Z 
2019-11-22T18:13:33.8662850Z error in revision `MSAN-0`: compilation failed!
2019-11-22T18:13:33.8663000Z status: exit code: 1
2019-11-22T18:13:33.8664610Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/codegen/sanitizer-memory-track-orgins.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--cfg" "msan_0" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-memory-track-orgins.MSAN-0" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-Zsanitizer=memory" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-memory-track-orgins.MSAN-0/auxiliary" "--emit=llvm-ir"
2019-11-22T18:13:33.8665670Z ------------------------------------------
2019-11-22T18:13:33.8665770Z 
2019-11-22T18:13:33.8666810Z ------------------------------------------
2019-11-22T18:13:33.8666940Z stderr:
2019-11-22T18:13:33.8666940Z stderr:
2019-11-22T18:13:33.8667580Z ------------------------------------------
2019-11-22T18:13:33.8668310Z error: MemorySanitizer only works with the `x86_64-unknown-linux-gnu` target
2019-11-22T18:13:33.8668510Z error: aborting due to previous error
2019-11-22T18:13:33.8668570Z 
2019-11-22T18:13:33.8668610Z 
2019-11-22T18:13:33.8669270Z ------------------------------------------
2019-11-22T18:13:33.8669270Z ------------------------------------------
2019-11-22T18:13:33.8669350Z 
2019-11-22T18:13:33.8669410Z 
2019-11-22T18:13:33.8670100Z ---- [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-1 stdout ----
2019-11-22T18:13:33.8670190Z 
2019-11-22T18:13:33.8670860Z error in revision `MSAN-1`: compilation failed!
2019-11-22T18:13:33.8670990Z status: exit code: 1
2019-11-22T18:13:33.8672630Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/codegen/sanitizer-memory-track-orgins.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--cfg" "msan_1" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-memory-track-orgins.MSAN-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-Zsanitizer=memory" "-Zsanitizer-memory-track-origins=1" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-memory-track-orgins.MSAN-1/auxiliary" "--emit=llvm-ir"
2019-11-22T18:13:33.8673740Z ------------------------------------------
2019-11-22T18:13:33.8673820Z 
2019-11-22T18:13:33.8674480Z ------------------------------------------
2019-11-22T18:13:33.8674810Z stderr:
2019-11-22T18:13:33.8674810Z stderr:
2019-11-22T18:13:33.8675580Z ------------------------------------------
2019-11-22T18:13:33.8676290Z error: MemorySanitizer only works with the `x86_64-unknown-linux-gnu` target
2019-11-22T18:13:33.8676500Z error: aborting due to previous error
2019-11-22T18:13:33.8676570Z 
2019-11-22T18:13:33.8676610Z 
2019-11-22T18:13:33.8677280Z ------------------------------------------
2019-11-22T18:13:33.8677280Z ------------------------------------------
2019-11-22T18:13:33.8677360Z 
2019-11-22T18:13:33.8677400Z 
2019-11-22T18:13:33.8678250Z ---- [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-2 stdout ----
2019-11-22T18:13:33.8678340Z 
2019-11-22T18:13:33.8679020Z error in revision `MSAN-2`: compilation failed!
2019-11-22T18:13:33.8679130Z status: exit code: 1
2019-11-22T18:13:33.8680760Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/codegen/sanitizer-memory-track-orgins.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--cfg" "msan_2" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-memory-track-orgins.MSAN-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-Zsanitizer=memory" "-Zsanitizer-memory-track-origins" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-memory-track-orgins.MSAN-2/auxiliary" "--emit=llvm-ir"
2019-11-22T18:13:33.8681820Z ------------------------------------------
2019-11-22T18:13:33.8681920Z 
2019-11-22T18:13:33.8682570Z ------------------------------------------
2019-11-22T18:13:33.8682690Z stderr:
2019-11-22T18:13:33.8682690Z stderr:
2019-11-22T18:13:33.8683330Z ------------------------------------------
2019-11-22T18:13:33.8684060Z error: MemorySanitizer only works with the `x86_64-unknown-linux-gnu` target
2019-11-22T18:13:33.8684270Z error: aborting due to previous error
2019-11-22T18:13:33.8684320Z 
2019-11-22T18:13:33.8684380Z 
2019-11-22T18:13:33.8685020Z ------------------------------------------
2019-11-22T18:13:33.8685020Z ------------------------------------------
2019-11-22T18:13:33.8685100Z 
2019-11-22T18:13:33.8685160Z 
2019-11-22T18:13:33.8686130Z ---- [codegen] codegen/sanitizer-recover.rs#MSAN stdout ----
2019-11-22T18:13:33.8686230Z 
2019-11-22T18:13:33.8686310Z error in revision `MSAN`: compilation failed!
2019-11-22T18:13:33.8686410Z status: exit code: 1
2019-11-22T18:13:33.8688030Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/codegen/sanitizer-recover.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--cfg" "msan" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-recover.MSAN" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-Zsanitizer=memory" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-recover.MSAN/auxiliary" "--emit=llvm-ir"
2019-11-22T18:13:33.8689140Z ------------------------------------------
2019-11-22T18:13:33.8689250Z 
2019-11-22T18:13:33.8689900Z ------------------------------------------
2019-11-22T18:13:33.8690020Z stderr:
2019-11-22T18:13:33.8690020Z stderr:
2019-11-22T18:13:33.8690660Z ------------------------------------------
2019-11-22T18:13:33.8691390Z error: MemorySanitizer only works with the `x86_64-unknown-linux-gnu` target
2019-11-22T18:13:33.8691570Z error: aborting due to previous error
2019-11-22T18:13:33.8691620Z 
2019-11-22T18:13:33.8691670Z 
2019-11-22T18:13:33.8692320Z ------------------------------------------
2019-11-22T18:13:33.8692320Z ------------------------------------------
2019-11-22T18:13:33.8692400Z 
2019-11-22T18:13:33.8692440Z 
2019-11-22T18:13:33.8693140Z ---- [codegen] codegen/sanitizer-recover.rs#MSAN-RECOVER stdout ----
2019-11-22T18:13:33.8693220Z 
2019-11-22T18:13:33.8694090Z error in revision `MSAN-RECOVER`: compilation failed!
2019-11-22T18:13:33.8694250Z status: exit code: 1
2019-11-22T18:13:33.8696100Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/codegen/sanitizer-recover.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--cfg" "msan_recover" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-recover.MSAN-RECOVER" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-Zsanitizer=memory" "-Zsanitizer-recover=memory" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-recover.MSAN-RECOVER/auxiliary" "--emit=llvm-ir"
2019-11-22T18:13:33.8697240Z ------------------------------------------
2019-11-22T18:13:33.8697320Z 
2019-11-22T18:13:33.8697990Z ------------------------------------------
2019-11-22T18:13:33.8698100Z stderr:
2019-11-22T18:13:33.8698100Z stderr:
2019-11-22T18:13:33.8698760Z ------------------------------------------
2019-11-22T18:13:33.8699470Z error: MemorySanitizer only works with the `x86_64-unknown-linux-gnu` target
2019-11-22T18:13:33.8699680Z error: aborting due to previous error
2019-11-22T18:13:33.8699750Z 
2019-11-22T18:13:33.8699790Z 
2019-11-22T18:13:33.8700440Z ------------------------------------------
2019-11-22T18:13:33.8700440Z ------------------------------------------
2019-11-22T18:13:33.8700540Z 
2019-11-22T18:13:33.8700580Z 
2019-11-22T18:13:33.8700630Z 
2019-11-22T18:13:33.8700710Z failures:
2019-11-22T18:13:33.8701390Z     [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-0
2019-11-22T18:13:33.8702110Z     [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-1
2019-11-22T18:13:33.8702830Z     [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-2
2019-11-22T18:13:33.8703520Z     [codegen] codegen/sanitizer-recover.rs#MSAN
2019-11-22T18:13:33.8704230Z     [codegen] codegen/sanitizer-recover.rs#MSAN-RECOVER
2019-11-22T18:13:33.8705050Z test result: FAILED. 145 passed; 5 failed; 14 ignored; 0 measured; 0 filtered out
2019-11-22T18:13:33.8705140Z 
2019-11-22T18:13:33.8705850Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-22T18:13:33.8706270Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-22T18:13:33.8706270Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-22T18:13:33.8706340Z 
2019-11-22T18:13:33.8706410Z 
2019-11-22T18:13:33.8709740Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/codegen" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/codegen" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "codegen" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-22T18:13:33.8710790Z 
2019-11-22T18:13:33.8710840Z 
2019-11-22T18:13:33.8710940Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-11-22T18:13:33.8711050Z Build completed unsuccessfully in 0:59:28
2019-11-22T18:13:33.8711050Z Build completed unsuccessfully in 0:59:28
2019-11-22T18:13:33.8711180Z == clock drift check ==
2019-11-22T18:13:33.8770660Z   local time: Fri Nov 22 18:13:33 UTC 2019
2019-11-22T18:13:33.9668440Z   network time: Fri, 22 Nov 2019 18:13:33 GMT
2019-11-22T18:13:33.9670890Z == end clock drift check ==
2019-11-22T18:13:33.9720250Z 
2019-11-22T18:13:33.9863630Z ##[error]Bash exited with code '1'.
2019-11-22T18:13:33.9915990Z ##[section]Starting: Checkout
2019-11-22T18:13:33.9919010Z ==============================================================================
2019-11-22T18:13:33.9919140Z Task         : Get sources
2019-11-22T18:13:33.9919250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
