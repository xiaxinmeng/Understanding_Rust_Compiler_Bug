\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-52023-array-sizer: aborting due to 2 previous errors\n\n"}
[00:50:03] {"message":"Some errors occurred: E0080, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0080, E0658.\n"}
[00:50:03] {"message":"For more information about an error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0080`.\n"}
[00:50:03] ------------------------------------------
[00:50:03] 
[00:50:03] thread '[ui] ui/issues/issue-52023-array-size-pointer-cast.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:50:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:03] 
[00:50:03] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:50:03] 
[00:50:03] 
[00:50:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--travis_time:end:2dbd93fc:start=1542018334407824926,finish=1542021338470576700,duration=3004062751774
travis_time:start:08fac29d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 12 11:15:38 UTC 2018
Mon, 12 Nov 2018 11:15:38 GMT
