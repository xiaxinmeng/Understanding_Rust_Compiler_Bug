plain
2019-09-20T11:54:31.6344300Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T11:54:31.6525779Z ##[command]git config gc.auto 0
2019-09-20T11:54:31.6600772Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T11:54:31.6666239Z ##[command]git config --get-all http.proxy
2019-09-20T11:54:31.6818717Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62330/merge:refs/remotes/pull/62330/merge
---
2019-09-20T12:58:00.7181058Z .................................................................................................... 1500/9028
2019-09-20T12:58:06.9486035Z .................................................................................................... 1600/9028
2019-09-20T12:58:19.9933977Z ..................................................................i...............i................. 1700/9028
2019-09-20T12:58:27.5876217Z .................................................................................................... 1800/9028
2019-09-20T12:58:43.5122606Z .........................................................iiiii...................................... 1900/9028
2019-09-20T12:58:55.4609536Z .................................................................................................... 2100/9028
2019-09-20T12:58:58.0290365Z .................................................................................................... 2200/9028
2019-09-20T12:59:01.5016512Z .................................................................................................... 2300/9028
2019-09-20T12:59:10.2153782Z .................................................................................................... 2400/9028
---
2019-09-20T13:02:15.3118062Z .............................................i...............i...................................... 4700/9028
2019-09-20T13:02:25.9659830Z .................................................................................................... 4800/9028
2019-09-20T13:02:33.7319098Z .................................................................................................... 4900/9028
2019-09-20T13:02:43.6902782Z .................................................................................................... 5000/9028
2019-09-20T13:02:51.8698688Z .............................ii.ii.................................................................. 5100/9028
2019-09-20T13:03:01.8285423Z .................................................................................................... 5300/9028
2019-09-20T13:03:12.9485644Z .............................................................................................i...... 5400/9028
2019-09-20T13:03:21.7258394Z .................................................................................................... 5500/9028
2019-09-20T13:03:26.8040446Z .................................................................................................... 5600/9028
2019-09-20T13:03:26.8040446Z .................................................................................................... 5600/9028
2019-09-20T13:03:37.8288394Z ........................................................................................ii...i..ii.. 5700/9028
2019-09-20T13:04:04.1950844Z .................................................................................................... 5900/9028
2019-09-20T13:04:14.5403603Z .................................................................................................... 6000/9028
2019-09-20T13:04:14.5403603Z .................................................................................................... 6000/9028
2019-09-20T13:04:21.7252634Z ..........................................................................................i..ii..... 6100/9028
2019-09-20T13:04:50.9490849Z .................................................................................................... 6300/9028
2019-09-20T13:04:55.9517586Z .................................................i.................................................. 6400/9028
2019-09-20T13:04:57.5986416Z .................................................................................................... 6500/9028
2019-09-20T13:05:00.1588403Z .....................i.............................................................................. 6600/9028
---
2019-09-20T13:09:14.8879609Z 
2019-09-20T13:09:14.8880172Z ---- [ui] ui/union/union-custom-drop.rs stdout ----
2019-09-20T13:09:14.8880236Z diff of stderr:
2019-09-20T13:09:14.8880295Z 
2019-09-20T13:09:14.8881108Z - error[E0601]: `main` function not found in crate `union_custom_drop`
2019-09-20T13:09:14.8881328Z -   --> $DIR/union-custom-drop.rs:4:1
2019-09-20T13:09:14.8881547Z -    |
2019-09-20T13:09:14.8881755Z - LL | / #![feature(untagged_unions)]
2019-09-20T13:09:14.8881929Z - LL | |
2019-09-20T13:09:14.8882108Z - LL | | union Foo {
2019-09-20T13:09:14.8882312Z - LL | |     bar: Bar,
2019-09-20T13:09:14.8882483Z - ...  |
2019-09-20T13:09:14.8882655Z - LL | |     }
2019-09-20T13:09:14.8882845Z - LL | | }
2019-09-20T13:09:14.8883078Z -    | |_^ consider adding a `main` function to `$DIR/union-custom-drop.rs`
2019-09-20T13:09:14.8883311Z 13 error[E0740]: unions may not contain fields that need dropping
2019-09-20T13:09:14.8883518Z 14   --> $DIR/union-custom-drop.rs:7:5
2019-09-20T13:09:14.8883568Z 15    |
2019-09-20T13:09:14.8883592Z 
---
2019-09-20T13:09:14.8885261Z 29 
2019-09-20T13:09:14.8885313Z 
2019-09-20T13:09:14.8885340Z 
2019-09-20T13:09:14.8885387Z The actual stderr differed from the expected stderr.
2019-09-20T13:09:14.8885717Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-custom-drop/union-custom-drop.stderr
2019-09-20T13:09:14.8886019Z To update references, rerun the tests and pass the `--bless` flag
2019-09-20T13:09:14.8886302Z To only update this specific test, also pass `--test-args union/union-custom-drop.rs`
2019-09-20T13:09:14.8886403Z error: 1 errors occurred comparing output.
2019-09-20T13:09:14.8886450Z status: exit code: 1
2019-09-20T13:09:14.8886450Z status: exit code: 1
2019-09-20T13:09:14.8887188Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-custom-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-custom-drop" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-custom-drop/auxiliary" "-A" "unused"
2019-09-20T13:09:14.8887551Z ------------------------------------------
2019-09-20T13:09:14.8887835Z 
2019-09-20T13:09:14.8888568Z ------------------------------------------
2019-09-20T13:09:14.8888611Z stderr:
2019-09-20T13:09:14.8888611Z stderr:
2019-09-20T13:09:14.8889220Z ------------------------------------------
2019-09-20T13:09:14.8889293Z error[E0740]: unions may not contain fields that need dropping
2019-09-20T13:09:14.8889540Z   --> /checkout/src/test/ui/union/union-custom-drop.rs:7:5
2019-09-20T13:09:14.8889737Z    |
2019-09-20T13:09:14.8889852Z LL |     bar: Bar, //~ ERROR unions may not contain fields that need dropping
2019-09-20T13:09:14.8889941Z    |
2019-09-20T13:09:14.8890005Z note: `std::mem::ManuallyDrop` can be used to wrap the type
2019-09-20T13:09:14.8890298Z   --> /checkout/src/test/ui/union/union-custom-drop.rs:7:5
2019-09-20T13:09:14.8890347Z    |
2019-09-20T13:09:14.8890347Z    |
2019-09-20T13:09:14.8890414Z LL |     bar: Bar, //~ ERROR unions may not contain fields that need dropping
2019-09-20T13:09:14.8890490Z 
2019-09-20T13:09:14.8890542Z error: aborting due to previous error
2019-09-20T13:09:14.8890589Z 
2019-09-20T13:09:14.8890840Z For more information about this error, try `rustc --explain E0740`.
---
2019-09-20T13:09:14.8930641Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-20T13:09:14.8930730Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T13:09:14.8947736Z 
2019-09-20T13:09:14.8947848Z 
2019-09-20T13:09:14.8950123Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-20T13:09:14.9023062Z 
2019-09-20T13:09:14.9023093Z 
2019-09-20T13:09:14.9075478Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-20T13:09:14.9075590Z Build completed unsuccessfully in 1:07:32
2019-09-20T13:09:14.9075590Z Build completed unsuccessfully in 1:07:32
2019-09-20T13:09:14.9075643Z == clock drift check ==
2019-09-20T13:09:14.9075690Z   local time: Fri Sep 20 13:09:14 UTC 2019
2019-09-20T13:09:15.0543941Z   network time: Fri, 20 Sep 2019 13:09:15 GMT
2019-09-20T13:09:15.0553479Z == end clock drift check ==
2019-09-20T13:09:15.8862025Z ##[error]Bash exited with code '1'.
2019-09-20T13:09:15.8902154Z ##[section]Starting: Checkout
2019-09-20T13:09:15.8903982Z ==============================================================================
2019-09-20T13:09:15.8904032Z Task         : Get sources
2019-09-20T13:09:15.8904075Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
