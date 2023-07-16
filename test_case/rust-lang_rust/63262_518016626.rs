plain
2019-08-04T16:16:34.4871180Z 
2019-08-04T16:16:34.4871890Z ---- [ui] ui/consts/miri_unleashed/mutable_references_ice.rs stdout ----
2019-08-04T16:16:34.4872020Z diff of stderr:
2019-08-04T16:16:34.4872100Z 
2019-08-04T16:16:34.4872170Z 4 LL |         *MUH.x.get() = 99;
2019-08-04T16:16:34.4872330Z 6 
2019-08-04T16:16:34.4873000Z - thread '<unnamed>' panicked at 'assertion failed: `(left != right)`
2019-08-04T16:16:34.4873000Z - thread '<unnamed>' panicked at 'assertion failed: `(left != right)`
2019-08-04T16:16:34.4873690Z + thread 'rustc' panicked at 'assertion failed: `(left != right)`
2019-08-04T16:16:34.4873800Z 8   left: `Const`,
2019-08-04T16:16:34.4874780Z 9  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites may arbitrarily decide to change, too.', src/librustc_mir/interpret/intern.rs:LL:CC
2019-08-04T16:16:34.4875090Z 
2019-08-04T16:16:34.4875130Z 
2019-08-04T16:16:34.4875220Z The actual stderr differed from the expected stderr.
2019-08-04T16:16:34.4876260Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/consts/miri_unleashed/mutable_references_ice/mutable_references_ice.stderr
2019-08-04T16:16:34.4876260Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/consts/miri_unleashed/mutable_references_ice/mutable_references_ice.stderr
2019-08-04T16:16:34.4877070Z To update references, rerun the tests and pass the `--bless` flag
2019-08-04T16:16:34.4877790Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references_ice.rs`
2019-08-04T16:16:34.4877980Z error: 1 errors occurred comparing output.
2019-08-04T16:16:34.4878070Z status: exit code: 101
2019-08-04T16:16:34.4878070Z status: exit code: 101
2019-08-04T16:16:34.4879580Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/consts/miri_unleashed/mutable_references_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary" "-A" "unused"
2019-08-04T16:16:34.4880860Z ------------------------------------------
2019-08-04T16:16:34.4880930Z 
2019-08-04T16:16:34.4881540Z ------------------------------------------
2019-08-04T16:16:34.4881660Z stderr:
2019-08-04T16:16:34.4881660Z stderr:
2019-08-04T16:16:34.4882260Z ------------------------------------------
2019-08-04T16:16:34.4882380Z warning: skipping const checks
2019-08-04T16:16:34.4883110Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:27:9
2019-08-04T16:16:34.4883240Z    |
2019-08-04T16:16:34.4883330Z LL |         *MUH.x.get() = 99; //~ WARN skipping const checks
2019-08-04T16:16:34.4883470Z 
2019-08-04T16:16:34.4883470Z 
2019-08-04T16:16:34.4884120Z thread 'rustc' panicked at 'assertion failed: `(left != right)`
2019-08-04T16:16:34.4884380Z   left: `Const`,
2019-08-04T16:16:34.4885390Z  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites may arbitrarily decide to change, too.', src/librustc_mir/interpret/intern.rs:126:17
2019-08-04T16:16:34.4885700Z 
2019-08-04T16:16:34.4885800Z error: internal compiler error: unexpected panic
2019-08-04T16:16:34.4885860Z 
2019-08-04T16:16:34.4885930Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-04T16:16:34.4885930Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-04T16:16:34.4886000Z 
2019-08-04T16:16:34.4886730Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-08-04T16:16:34.4887510Z note: rustc 1.37.0-dev running on i686-apple-darwin
2019-08-04T16:16:34.4887590Z 
2019-08-04T16:16:34.4887590Z 
2019-08-04T16:16:34.4888510Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-04T16:16:34.4888680Z 
2019-08-04T16:16:34.4889290Z ------------------------------------------
2019-08-04T16:16:34.4889360Z 
2019-08-04T16:16:34.4889420Z 
2019-08-04T16:16:34.4889420Z 
2019-08-04T16:16:34.4890040Z ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
2019-08-04T16:16:34.4890160Z diff of stderr:
2019-08-04T16:16:34.4890210Z 
2019-08-04T16:16:34.4890940Z - thread '<unnamed>' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:LL:CC
2019-08-04T16:16:34.4891730Z + thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:LL:CC
2019-08-04T16:16:34.4892230Z 3 
2019-08-04T16:16:34.4892320Z 4 error: internal compiler error: unexpected panic
2019-08-04T16:16:34.4892370Z 
2019-08-04T16:16:34.4892410Z 
2019-08-04T16:16:34.4892410Z 
2019-08-04T16:16:34.4892500Z The actual stderr differed from the expected stderr.
2019-08-04T16:16:34.4893280Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
2019-08-04T16:16:34.4893990Z To update references, rerun the tests and pass the `--bless` flag
2019-08-04T16:16:34.4894770Z To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
2019-08-04T16:16:34.4895190Z error: 1 errors occurred comparing output.
2019-08-04T16:16:34.4895270Z status: exit code: 101
2019-08-04T16:16:34.4895270Z status: exit code: 101
2019-08-04T16:16:34.4896690Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/pattern/const-pat-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
2019-08-04T16:16:34.4897650Z ------------------------------------------
2019-08-04T16:16:34.4897720Z 
2019-08-04T16:16:34.4898330Z ------------------------------------------
2019-08-04T16:16:34.4898470Z stderr:
2019-08-04T16:16:34.4898470Z stderr:
2019-08-04T16:16:34.4899080Z ------------------------------------------
2019-08-04T16:16:34.4899850Z thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1088:5
2019-08-04T16:16:34.4900090Z 
2019-08-04T16:16:34.4900160Z error: internal compiler error: unexpected panic
2019-08-04T16:16:34.4900230Z 
2019-08-04T16:16:34.4900300Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-04T16:16:34.4900300Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-04T16:16:34.4900370Z 
2019-08-04T16:16:34.4901080Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-08-04T16:16:34.4901810Z note: rustc 1.37.0-dev running on i686-apple-darwin
2019-08-04T16:16:34.4901880Z 
2019-08-04T16:16:34.4901880Z 
2019-08-04T16:16:34.4902580Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-04T16:16:34.4902730Z 
2019-08-04T16:16:34.4903350Z ------------------------------------------
2019-08-04T16:16:34.4903420Z 
2019-08-04T16:16:34.4903460Z 
2019-08-04T16:16:34.4903460Z 
2019-08-04T16:16:34.4904110Z ---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----
2019-08-04T16:16:34.4904220Z diff of stderr:
2019-08-04T16:16:34.4904290Z 
2019-08-04T16:16:34.4904360Z 6    |
2019-08-04T16:16:34.4904460Z 7    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-08-04T16:16:34.4905180Z - error: aborting due to previous error
2019-08-04T16:16:34.4905180Z - error: aborting due to previous error
2019-08-04T16:16:34.4905300Z + error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
2019-08-04T16:16:34.4905950Z +   --> $DIR/cycle-cache-err-60010.rs:30:6
2019-08-04T16:16:34.4906050Z +    |
2019-08-04T16:16:34.4906130Z + LL | impl Database for RootDatabase {
2019-08-04T16:16:34.4906300Z +    |
2019-08-04T16:16:34.4906300Z +    |
2019-08-04T16:16:34.4906390Z +    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-08-04T16:16:34.4906500Z +    = note: required because it appears within the type `SalsaStorage`
2019-08-04T16:16:34.4906850Z + error: aborting due to 2 previous errors
2019-08-04T16:16:34.4906990Z 10 
2019-08-04T16:16:34.4907700Z 11 For more information about this error, try `rustc --explain E0275`.
2019-08-04T16:16:34.4907810Z 12 
2019-08-04T16:16:34.4907810Z 12 
2019-08-04T16:16:34.4907870Z 
2019-08-04T16:16:34.4907910Z 
2019-08-04T16:16:34.4907990Z The actual stderr differed from the expected stderr.
2019-08-04T16:16:34.4908760Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/traits/cycle-cache-err-60010/cycle-cache-err-60010.stderr
2019-08-04T16:16:34.4909460Z To update references, rerun the tests and pass the `--bless` flag
2019-08-04T16:16:34.4910170Z To only update this specific test, also pass `--test-args traits/cycle-cache-err-60010.rs`
2019-08-04T16:16:34.4910580Z error: 1 errors occurred comparing output.
2019-08-04T16:16:34.4910650Z status: exit code: 1
2019-08-04T16:16:34.4910650Z status: exit code: 1
2019-08-04T16:16:34.4912110Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/traits/cycle-cache-err-60010" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/traits/cycle-cache-err-60010/auxiliary" "-A" "unused"
2019-08-04T16:16:34.4913070Z ------------------------------------------
2019-08-04T16:16:34.4913150Z 
2019-08-04T16:16:34.4913770Z ------------------------------------------
2019-08-04T16:16:34.4913940Z stderr:
2019-08-04T16:16:34.4913940Z stderr:
2019-08-04T16:16:34.4914750Z ------------------------------------------
2019-08-04T16:16:34.4914870Z error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
2019-08-04T16:16:34.4915700Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/traits/cycle-cache-err-60010.rs:27:5
2019-08-04T16:16:34.4915830Z    |
2019-08-04T16:16:34.4915930Z LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data, //~ ERROR overflow
2019-08-04T16:16:34.4916120Z    |
2019-08-04T16:16:34.4916120Z    |
2019-08-04T16:16:34.4916210Z    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-08-04T16:16:34.4916300Z 
2019-08-04T16:16:34.4916380Z error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
2019-08-04T16:16:34.4917150Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/traits/cycle-cache-err-60010.rs:30:6
2019-08-04T16:16:34.4917280Z    |
2019-08-04T16:16:34.4917360Z LL | impl Database for RootDatabase {
2019-08-04T16:16:34.4917520Z    |
2019-08-04T16:16:34.4917520Z    |
2019-08-04T16:16:34.4917620Z    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-08-04T16:16:34.4917740Z    = note: required because it appears within the type `SalsaStorage`
2019-08-04T16:16:34.4917880Z error: aborting due to 2 previous errors
2019-08-04T16:16:34.4917930Z 
2019-08-04T16:16:34.4918760Z For more information about this error, try `rustc --explain E0275`.
2019-08-04T16:16:34.4918860Z 
---
2019-08-04T16:16:34.4922850Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-08-04T16:16:34.4923010Z note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-04T16:16:34.4929130Z 
2019-08-04T16:16:34.4929470Z 
2019-08-04T16:16:34.4933010Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0-rust-1.37.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-04T16:16:34.4934180Z 
2019-08-04T16:16:34.4934230Z 
2019-08-04T16:16:34.4944540Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-04T16:16:34.4944800Z Build completed unsuccessfully in 0:56:50
2019-08-04T16:16:34.4944800Z Build completed unsuccessfully in 0:56:50
2019-08-04T16:16:34.5211480Z ##[error]Bash exited with code '1'.
2019-08-04T16:16:34.5257270Z ##[section]Starting: Upload CPU usage statistics
2019-08-04T16:16:34.5261660Z ==============================================================================
2019-08-04T16:16:34.5261760Z Task         : Bash
2019-08-04T16:16:34.5261840Z Description  : Run a Bash script on macOS, Linux, or Windows
