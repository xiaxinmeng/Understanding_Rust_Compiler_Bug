plain
2019-09-17T06:16:37.7000000Z diff of stderr:
2019-09-17T06:16:37.7000050Z 
2019-09-17T06:16:37.7000640Z 2   --> $DIR/due-to-where-clause.rs:5:5
2019-09-17T06:16:37.7000760Z 3    |
2019-09-17T06:16:37.7000850Z 4 LL |     test::<FooS>(&mut 42);
2019-09-17T06:16:37.7001480Z -    |     ^^^^^^^^^^^^ doesn't satisfy where-clause
2019-09-17T06:16:37.7001590Z +    |     ^^^^^^^^^^^^ implementation of `Foo` is not general enough
2019-09-17T06:16:37.7002440Z 7 LL | trait Foo<'a> {}
2019-09-17T06:16:37.7002440Z 7 LL | trait Foo<'a> {}
2019-09-17T06:16:37.7003070Z 8    | ---------------- trait `Foo` defined here
2019-09-17T06:16:37.7003720Z - ...
2019-09-17T06:16:37.7003720Z - ...
2019-09-17T06:16:37.7004340Z - LL | fn test<'a, F>(data: &'a mut u32) where F: for<'b> Foo<'b> {}
2019-09-17T06:16:37.7005070Z -    | ------------------------------------------------------------- due to a where-clause on `test`...
2019-09-17T06:16:37.7005190Z 12    |
2019-09-17T06:16:37.7005860Z -    = note: ...`FooS<'_>` must implement `Foo<'0>`, for any lifetime `'0`...
2019-09-17T06:16:37.7007360Z +    = note: `FooS<'_>` must implement `Foo<'0>`, for any lifetime `'0`...
2019-09-17T06:16:37.7008080Z 14    = note: ...but `FooS<'_>` actually implements `Foo<'1>`, for some specific lifetime `'1`
2019-09-17T06:16:37.7008700Z 16 error: aborting due to previous error
2019-09-17T06:16:37.7008790Z 
2019-09-17T06:16:37.7008880Z 
2019-09-17T06:16:37.7008990Z The actual stderr differed from the expected stderr.
2019-09-17T06:16:37.7008990Z The actual stderr differed from the expected stderr.
2019-09-17T06:16:37.7009970Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/hrtb/due-to-where-clause/due-to-where-clause.stderr
2019-09-17T06:16:37.7010970Z To update references, rerun the tests and pass the `--bless` flag
2019-09-17T06:16:37.7011960Z To only update this specific test, also pass `--test-args hrtb/due-to-where-clause.rs`
2019-09-17T06:16:37.7012500Z error: 1 errors occurred comparing output.
2019-09-17T06:16:37.7012630Z status: exit code: 1
2019-09-17T06:16:37.7012630Z status: exit code: 1
2019-09-17T06:16:37.7014070Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/hrtb/due-to-where-clause.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/hrtb/due-to-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/hrtb/due-to-where-clause/auxiliary" "-A" "unused"
2019-09-17T06:16:37.7015550Z ------------------------------------------
2019-09-17T06:16:37.7015890Z 
2019-09-17T06:16:37.7016820Z ------------------------------------------
2019-09-17T06:16:37.7017250Z stderr:
2019-09-17T06:16:37.7017250Z stderr:
2019-09-17T06:16:37.7017980Z ------------------------------------------
2019-09-17T06:16:37.7018410Z error: implementation of `Foo` is not general enough
2019-09-17T06:16:37.7019640Z    |
2019-09-17T06:16:37.7019640Z    |
2019-09-17T06:16:37.7020300Z LL |     test::<FooS>(&mut 42); //~ ERROR implementation of `Foo` is not general enough
2019-09-17T06:16:37.7020490Z    |     ^^^^^^^^^^^^ implementation of `Foo` is not general enough
2019-09-17T06:16:37.7021310Z LL | trait Foo<'a> {}
2019-09-17T06:16:37.7021310Z LL | trait Foo<'a> {}
2019-09-17T06:16:37.7022290Z    | ---------------- trait `Foo` defined here
2019-09-17T06:16:37.7022700Z    |
2019-09-17T06:16:37.7023470Z    = note: `FooS<'_>` must implement `Foo<'0>`, for any lifetime `'0`...
2019-09-17T06:16:37.7024480Z    = note: ...but `FooS<'_>` actually implements `Foo<'1>`, for some specific lifetime `'1`
2019-09-17T06:16:37.7025240Z error: aborting due to previous error
2019-09-17T06:16:37.7025630Z 
2019-09-17T06:16:37.7025750Z 
2019-09-17T06:16:37.7026470Z ------------------------------------------
---
2019-09-17T06:16:37.7100600Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-17T06:16:37.7100750Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-17T06:16:37.7116440Z 
2019-09-17T06:16:37.7116550Z 
2019-09-17T06:16:37.8380200Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-17T06:16:37.8381560Z 
2019-09-17T06:16:37.8381670Z 
2019-09-17T06:16:37.8381770Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-09-17T06:16:37.8381880Z Build completed unsuccessfully in 0:59:15
2019-09-17T06:16:37.8381880Z Build completed unsuccessfully in 0:59:15
2019-09-17T06:16:37.8381960Z == clock drift check ==
2019-09-17T06:16:37.8382050Z   local time: Tue Sep 17 06:16:37 UTC 2019
2019-09-17T06:16:37.8382140Z   network time: Tue, 17 Sep 2019 06:16:37 GMT
2019-09-17T06:16:37.8382220Z == end clock drift check ==
2019-09-17T06:16:37.8467380Z ##[error]Bash exited with code '1'.
2019-09-17T06:16:37.8522760Z ##[section]Starting: Upload CPU usage statistics
2019-09-17T06:16:37.8527360Z ==============================================================================
2019-09-17T06:16:37.8527470Z Task         : Bash
2019-09-17T06:16:37.8527870Z Description  : Run a Bash script on macOS, Linux, or Windows
