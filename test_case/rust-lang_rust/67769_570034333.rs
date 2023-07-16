plain
2020-01-01T06:17:56.6815190Z ---- [ui] ui/wrapping-int-api.rs stdout ----
2020-01-01T06:17:56.6815320Z 
2020-01-01T06:17:56.6816020Z error: test compilation failed although it shouldn't!
2020-01-01T06:17:56.6816160Z status: exit code: 1
2020-01-01T06:17:56.6817450Z command: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/wrapping-int-api.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/wrapping-int-api/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/wrapping-int-api/auxiliary"
2020-01-01T06:17:56.6818270Z ------------------------------------------
2020-01-01T06:17:56.6818330Z 
2020-01-01T06:17:56.6818840Z ------------------------------------------
2020-01-01T06:17:56.6819320Z stderr:
2020-01-01T06:17:56.6819320Z stderr:
2020-01-01T06:17:56.6819840Z ------------------------------------------
2020-01-01T06:17:56.6819960Z error: truncating cast: the value 18364758544493064727 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6820650Z    |
2020-01-01T06:17:56.6820650Z    |
2020-01-01T06:17:56.6821190Z LL |     check_mul_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -1);
2020-01-01T06:17:56.6821390Z    |
2020-01-01T06:17:56.6821450Z    = note: `#[deny(const_err)]` on by default
2020-01-01T06:17:56.6821490Z 
2020-01-01T06:17:56.6821490Z 
2020-01-01T06:17:56.6821590Z error: truncating cast: the value 18364758546783713944 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6822270Z    |
2020-01-01T06:17:56.6822270Z    |
2020-01-01T06:17:56.6822810Z LL |     check_mul_no_wrap!(0xfedc_ba98_fedc_ba98_u64 as u64 as isize, -2);
2020-01-01T06:17:56.6822960Z 
2020-01-01T06:17:56.6822960Z 
2020-01-01T06:17:56.6823220Z error: truncating cast: the value 18364758546783713944 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6823960Z    |
2020-01-01T06:17:56.6823960Z    |
2020-01-01T06:17:56.6824040Z LL |     check_mul_no_wrap!(0xfedc_ba98_fedc_ba98_u64 as u64 as isize, 2);
2020-01-01T06:17:56.6824180Z 
2020-01-01T06:17:56.6824180Z 
2020-01-01T06:17:56.6824260Z error: truncating cast: the value 18364758544493064727 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6824940Z    |
2020-01-01T06:17:56.6824940Z    |
2020-01-01T06:17:56.6825480Z LL |     check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -1);
2020-01-01T06:17:56.6825650Z 
2020-01-01T06:17:56.6825650Z 
2020-01-01T06:17:56.6825740Z error: truncating cast: the value 18364758544493064727 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6826400Z    |
2020-01-01T06:17:56.6826400Z    |
2020-01-01T06:17:56.6826940Z LL |     check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -2);
2020-01-01T06:17:56.6827110Z 
2020-01-01T06:17:56.6827110Z 
2020-01-01T06:17:56.6827180Z error: truncating cast: the value 18364758544493064727 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6827860Z    |
2020-01-01T06:17:56.6827860Z    |
2020-01-01T06:17:56.6827940Z LL |     check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, 2);
2020-01-01T06:17:56.6828090Z 
2020-01-01T06:17:56.6828090Z 
2020-01-01T06:17:56.6828170Z error: truncating cast: the value 18364758544493064727 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6828860Z    |
2020-01-01T06:17:56.6828860Z    |
2020-01-01T06:17:56.6829380Z LL |     check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -1);
2020-01-01T06:17:56.6829550Z 
2020-01-01T06:17:56.6829550Z 
2020-01-01T06:17:56.6829640Z error: truncating cast: the value 18364758544493064727 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6830490Z    |
2020-01-01T06:17:56.6830490Z    |
2020-01-01T06:17:56.6831040Z LL |     check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -2);
2020-01-01T06:17:56.6831210Z 
2020-01-01T06:17:56.6831210Z 
2020-01-01T06:17:56.6831310Z error: truncating cast: the value 18364758544493064727 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6831980Z    |
2020-01-01T06:17:56.6831980Z    |
2020-01-01T06:17:56.6832050Z LL |     check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, 2);
2020-01-01T06:17:56.6832190Z 
2020-01-01T06:17:56.6832190Z 
2020-01-01T06:17:56.6832280Z error: truncating cast: the value 18364758544493064727 requires 64 bits but the target type is only 32 bits
2020-01-01T06:17:56.6832960Z    |
2020-01-01T06:17:56.6832960Z    |
2020-01-01T06:17:56.6833030Z LL |     check_neg_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize);
2020-01-01T06:17:56.6833170Z 
2020-01-01T06:17:56.6833240Z error: aborting due to 10 previous errors
2020-01-01T06:17:56.6833430Z 
2020-01-01T06:17:56.6833510Z 
---
2020-01-01T06:17:56.6920420Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-01T06:17:56.6920590Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-01T06:17:56.6936780Z 
2020-01-01T06:17:56.6937040Z 
2020-01-01T06:17:56.6940300Z command did not execute successfully: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-01T06:17:56.6941190Z 
2020-01-01T06:17:56.6941240Z 
2020-01-01T06:17:56.6950830Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap test
2020-01-01T06:17:56.6950960Z Build completed unsuccessfully in 0:48:27
2020-01-01T06:17:56.6950960Z Build completed unsuccessfully in 0:48:27
2020-01-01T06:17:56.7012960Z == clock drift check ==
2020-01-01T06:17:56.7063040Z   local time: Wed Jan  1 06:17:56 UTC 2020
2020-01-01T06:17:56.7652020Z   network time: Wed, 01 Jan 2020 06:17:56 GMT
2020-01-01T06:17:56.7654020Z == end clock drift check ==
2020-01-01T06:17:56.7707600Z 
2020-01-01T06:17:56.7818470Z ##[error]Bash exited with code '1'.
2020-01-01T06:17:56.7862170Z ##[section]Starting: Checkout
2020-01-01T06:17:56.7864780Z ==============================================================================
2020-01-01T06:17:56.7864870Z Task         : Get sources
2020-01-01T06:17:56.7864960Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
