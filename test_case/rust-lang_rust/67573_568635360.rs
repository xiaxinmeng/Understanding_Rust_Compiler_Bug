plain
2019-12-24T02:55:34.8246970Z 
2019-12-24T02:55:34.8247650Z ---- [ui] ui/consts/transmute-size-mismatch-before-typeck.rs stdout ----
2019-12-24T02:55:34.8247750Z diff of stderr:
2019-12-24T02:55:34.8247810Z 
2019-12-24T02:55:34.8247880Z 20 LL | const ZST: &[u8] = unsafe { std::mem::transmute(1usize) };
2019-12-24T02:55:34.8248040Z 22    |
2019-12-24T02:55:34.8248040Z 22    |
2019-12-24T02:55:34.8248620Z -    = note: source type: `usize` (64 bits)
2019-12-24T02:55:34.8249180Z -    = note: target type: `&'static [u8]` (128 bits)
2019-12-24T02:55:34.8249290Z +    = note: source type: `usize` (32 bits)
2019-12-24T02:55:34.8249850Z +    = note: target type: `&'static [u8]` (64 bits)
2019-12-24T02:55:34.8250010Z 26 error: aborting due to 3 previous errors
2019-12-24T02:55:34.8250070Z 27 
2019-12-24T02:55:34.8250390Z 
2019-12-24T02:55:34.8250450Z 
2019-12-24T02:55:34.8250450Z 
2019-12-24T02:55:34.8250520Z The actual stderr differed from the expected stderr.
2019-12-24T02:55:34.8251340Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/consts/transmute-size-mismatch-before-typeck/transmute-size-mismatch-before-typeck.stderr
2019-12-24T02:55:34.8251960Z To update references, rerun the tests and pass the `--bless` flag
2019-12-24T02:55:34.8252610Z To only update this specific test, also pass `--test-args consts/transmute-size-mismatch-before-typeck.rs`
2019-12-24T02:55:34.8253010Z error: 1 errors occurred comparing output.
2019-12-24T02:55:34.8253090Z status: exit code: 1
2019-12-24T02:55:34.8253090Z status: exit code: 1
2019-12-24T02:55:34.8254520Z command: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/transmute-size-mismatch-before-typeck.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/consts/transmute-size-mismatch-before-typeck" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/consts/transmute-size-mismatch-before-typeck/auxiliary" "-A" "unused"
2019-12-24T02:55:34.8255450Z ------------------------------------------
2019-12-24T02:55:34.8255520Z 
2019-12-24T02:55:34.8256060Z ------------------------------------------
2019-12-24T02:55:34.8256150Z stderr:
2019-12-24T02:55:34.8256150Z stderr:
2019-12-24T02:55:34.8256680Z ------------------------------------------
2019-12-24T02:55:34.8256770Z error: any use of this value will cause an error
2019-12-24T02:55:34.8257390Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/transmute-size-mismatch-before-typeck.rs:10:29
2019-12-24T02:55:34.8257530Z    |
2019-12-24T02:55:34.8257600Z LL | const ZST: &[u8] = unsafe { std::mem::transmute(1usize) };
2019-12-24T02:55:34.8258280Z    |                             |
2019-12-24T02:55:34.8258280Z    |                             |
2019-12-24T02:55:34.8258370Z    |                             tried to transmute from usize to &[u8], but their sizes differed
2019-12-24T02:55:34.8258530Z    = note: `#[deny(const_err)]` on by default
2019-12-24T02:55:34.8258590Z 
2019-12-24T02:55:34.8258660Z error: could not evaluate constant pattern
2019-12-24T02:55:34.8259270Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/transmute-size-mismatch-before-typeck.rs:5:9
2019-12-24T02:55:34.8259270Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/transmute-size-mismatch-before-typeck.rs:5:9
2019-12-24T02:55:34.8259400Z    |
2019-12-24T02:55:34.8259460Z LL |         ZST => {}
2019-12-24T02:55:34.8259540Z    |         ^^^
2019-12-24T02:55:34.8259580Z 
2019-12-24T02:55:34.8260190Z error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2019-12-24T02:55:34.8260820Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/transmute-size-mismatch-before-typeck.rs:10:29
2019-12-24T02:55:34.8260950Z    |
2019-12-24T02:55:34.8261010Z LL | const ZST: &[u8] = unsafe { std::mem::transmute(1usize) };
2019-12-24T02:55:34.8261180Z    |
2019-12-24T02:55:34.8261180Z    |
2019-12-24T02:55:34.8261260Z    = note: source type: `usize` (32 bits)
2019-12-24T02:55:34.8261830Z    = note: target type: `&'static [u8]` (64 bits)
2019-12-24T02:55:34.8261960Z error: aborting due to 3 previous errors
2019-12-24T02:55:34.8262020Z 
2019-12-24T02:55:34.8262580Z For more information about this error, try `rustc --explain E0512`.
2019-12-24T02:55:34.8262660Z 
---
2019-12-24T02:55:34.8340190Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-24T02:55:34.8340420Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-24T02:55:34.8359320Z 
2019-12-24T02:55:34.8360240Z 
2019-12-24T02:55:34.8363620Z command did not execute successfully: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-24T02:55:34.8365170Z 
2019-12-24T02:55:34.8365640Z 
2019-12-24T02:55:34.8372770Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-24T02:55:34.8373280Z Build completed unsuccessfully in 0:52:08
2019-12-24T02:55:34.8373280Z Build completed unsuccessfully in 0:52:08
2019-12-24T02:55:34.8441260Z == clock drift check ==
2019-12-24T02:55:34.8503400Z   local time: Tue Dec 24 02:55:34 UTC 2019
2019-12-24T02:55:34.9339680Z   network time: Tue, 24 Dec 2019 02:55:34 GMT
2019-12-24T02:55:34.9341640Z == end clock drift check ==
2019-12-24T02:55:34.9382710Z 
2019-12-24T02:55:34.9491880Z ##[error]Bash exited with code '1'.
2019-12-24T02:55:34.9537010Z ##[section]Starting: Checkout
2019-12-24T02:55:34.9539700Z ==============================================================================
2019-12-24T02:55:34.9539810Z Task         : Get sources
2019-12-24T02:55:34.9539890Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
