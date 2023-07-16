plain
2019-12-06T23:33:18.3967300Z ---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----
2019-12-06T23:33:18.3968830Z diff of stderr:
2019-12-06T23:33:18.3969450Z 
2019-12-06T23:33:18.3970010Z 6    |
2019-12-06T23:33:18.3970250Z 7    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-12-06T23:33:18.3971320Z - error: aborting due to previous error
2019-12-06T23:33:18.3971320Z - error: aborting due to previous error
2019-12-06T23:33:18.3971560Z + error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
2019-12-06T23:33:18.3972310Z +   --> $DIR/cycle-cache-err-60010.rs:31:5
2019-12-06T23:33:18.3972530Z + LL |     type Storage;
2019-12-06T23:33:18.3973220Z +    |          ------- associated type defined here
2019-12-06T23:33:18.3973350Z + ...
2019-12-06T23:33:18.3973350Z + ...
2019-12-06T23:33:18.3973420Z + LL | impl Database for RootDatabase {
2019-12-06T23:33:18.3974110Z +    | ------------------------------ in this `impl` item
2019-12-06T23:33:18.3974220Z + LL |     type Storage = SalsaStorage;
2019-12-06T23:33:18.3974400Z +    |
2019-12-06T23:33:18.3974400Z +    |
2019-12-06T23:33:18.3974520Z +    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-12-06T23:33:18.3974650Z +    = note: required because it appears within the type `SalsaStorage`
2019-12-06T23:33:18.3974830Z + error: aborting due to 2 previous errors
2019-12-06T23:33:18.3974910Z 10 
2019-12-06T23:33:18.3975630Z 11 For more information about this error, try `rustc --explain E0275`.
2019-12-06T23:33:18.3975750Z 12 
2019-12-06T23:33:18.3975750Z 12 
2019-12-06T23:33:18.3975820Z 
2019-12-06T23:33:18.3975860Z 
2019-12-06T23:33:18.3975960Z The actual stderr differed from the expected stderr.
2019-12-06T23:33:18.3976780Z Actual stderr saved to /Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/ui/traits/cycle-cache-err-60010/cycle-cache-err-60010.stderr
2019-12-06T23:33:18.3977540Z To update references, rerun the tests and pass the `--bless` flag
2019-12-06T23:33:18.3978270Z To only update this specific test, also pass `--test-args traits/cycle-cache-err-60010.rs`
2019-12-06T23:33:18.3978460Z error: 1 errors occurred comparing output.
2019-12-06T23:33:18.3978560Z status: exit code: 1
2019-12-06T23:33:18.3978560Z status: exit code: 1
2019-12-06T23:33:18.3980360Z command: "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/ui/traits/cycle-cache-err-60010" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/ui/traits/cycle-cache-err-60010/auxiliary" "-A" "unused"
2019-12-06T23:33:18.3981500Z ------------------------------------------
2019-12-06T23:33:18.3981580Z 
2019-12-06T23:33:18.3982250Z ------------------------------------------
2019-12-06T23:33:18.3982370Z stderr:
2019-12-06T23:33:18.3982370Z stderr:
2019-12-06T23:33:18.3983000Z ------------------------------------------
2019-12-06T23:33:18.3983150Z error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
2019-12-06T23:33:18.3983880Z   --> /Users/runner/runners/2.160.1/work/1/s/src/test/ui/traits/cycle-cache-err-60010.rs:27:5
2019-12-06T23:33:18.3984030Z    |
2019-12-06T23:33:18.3984130Z LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data, //~ ERROR overflow
2019-12-06T23:33:18.3984630Z    |
2019-12-06T23:33:18.3984630Z    |
2019-12-06T23:33:18.3984730Z    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-12-06T23:33:18.3984820Z 
2019-12-06T23:33:18.3984900Z error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
2019-12-06T23:33:18.3985700Z   --> /Users/runner/runners/2.160.1/work/1/s/src/test/ui/traits/cycle-cache-err-60010.rs:31:5
2019-12-06T23:33:18.3985910Z LL |     type Storage;
2019-12-06T23:33:18.3986880Z    |          ------- associated type defined here
2019-12-06T23:33:18.3987010Z ...
2019-12-06T23:33:18.3987080Z LL | impl Database for RootDatabase {
2019-12-06T23:33:18.3987080Z LL | impl Database for RootDatabase {
2019-12-06T23:33:18.3987780Z    | ------------------------------ in this `impl` item
2019-12-06T23:33:18.3987930Z LL |     type Storage = SalsaStorage;
2019-12-06T23:33:18.3988100Z    |
2019-12-06T23:33:18.3988100Z    |
2019-12-06T23:33:18.3988200Z    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-12-06T23:33:18.3988330Z    = note: required because it appears within the type `SalsaStorage`
2019-12-06T23:33:18.3988480Z error: aborting due to 2 previous errors
2019-12-06T23:33:18.3988530Z 
2019-12-06T23:33:18.3989250Z For more information about this error, try `rustc --explain E0275`.
2019-12-06T23:33:18.3989330Z 
---
2019-12-06T23:33:18.4092650Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-06T23:33:18.4093100Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-06T23:33:18.4113950Z 
2019-12-06T23:33:18.4114590Z 
2019-12-06T23:33:18.4118320Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-06T23:33:18.4119420Z 
2019-12-06T23:33:18.4119460Z 
2019-12-06T23:33:18.4131520Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-06T23:33:18.4132020Z Build completed unsuccessfully in 1:02:18
2019-12-06T23:33:18.4132020Z Build completed unsuccessfully in 1:02:18
2019-12-06T23:33:18.4210790Z == clock drift check ==
2019-12-06T23:33:18.4272560Z   local time: Fri Dec  6 23:33:18 UTC 2019
2019-12-06T23:33:18.5177040Z   network time: Fri, 06 Dec 2019 23:33:18 GMT
2019-12-06T23:33:18.5178690Z == end clock drift check ==
2019-12-06T23:33:18.5229010Z 
2019-12-06T23:33:18.5383900Z ##[error]Bash exited with code '1'.
2019-12-06T23:33:18.5446240Z ##[section]Starting: Checkout
2019-12-06T23:33:18.5449630Z ==============================================================================
2019-12-06T23:33:18.5449760Z Task         : Get sources
2019-12-06T23:33:18.5449850Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
