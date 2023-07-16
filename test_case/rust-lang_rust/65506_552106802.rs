plain
2019-11-09T13:35:28.3872534Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-09T13:35:29.1646442Z ##[command]git config gc.auto 0
2019-11-09T13:35:29.1652403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-09T13:35:29.1654463Z ##[command]git config --get-all http.proxy
2019-11-09T13:35:29.1657109Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65506/merge:refs/remotes/pull/65506/merge
---
2019-11-09T14:34:00.1949147Z .................................................................................................... 1500/9359
2019-11-09T14:34:06.5288520Z .................................................................................................... 1600/9359
2019-11-09T14:34:15.5620539Z .................................................................................................... 1700/9359
2019-11-09T14:34:24.2848410Z ......i............................................................................................. 1800/9359
2019-11-09T14:34:30.9295344Z ..........................................................................................iiiii..... 1900/9359
2019-11-09T14:34:52.2639276Z .................................................................................................... 2100/9359
2019-11-09T14:34:54.6639432Z .................................................................................................... 2200/9359
2019-11-09T14:34:57.1860783Z .................................................................................................... 2300/9359
2019-11-09T14:35:03.3636854Z .................................................................................................... 2400/9359
---
2019-11-09T14:38:06.4544315Z ....i...............i............................................................................... 4800/9359
2019-11-09T14:38:16.4569093Z .................................................................................................... 4900/9359
2019-11-09T14:38:22.3272194Z .................................................................................................... 5000/9359
2019-11-09T14:38:32.4319693Z .................................................................................................... 5100/9359
2019-11-09T14:38:38.3430327Z .........ii.ii...........i.......................................................................... 5200/9359
2019-11-09T14:38:48.7722943Z .................................................................................................... 5400/9359
2019-11-09T14:39:00.0801402Z ..................................................................................................i. 5500/9359
2019-11-09T14:39:08.4809934Z .................................................................................................... 5600/9359
2019-11-09T14:39:14.1983929Z .................................................................................................... 5700/9359
2019-11-09T14:39:14.1983929Z .................................................................................................... 5700/9359
2019-11-09T14:39:26.6428208Z .................................................................................................... 5800/9359
2019-11-09T14:39:38.3678596Z ........ii...i..ii...........i...................................................................... 5900/9359
2019-11-09T14:39:57.0861018Z .................................................................................................... 6100/9359
2019-11-09T14:40:00.9469753Z .................................................................................................... 6200/9359
2019-11-09T14:40:00.9469753Z .................................................................................................... 6200/9359
2019-11-09T14:40:11.1252066Z ............................i..ii................................................................... 6300/9359
2019-11-09T14:40:38.3341257Z .................................................................................................... 6500/9359
2019-11-09T14:40:42.1530862Z ...................................................i................................................ 6600/9359
2019-11-09T14:40:44.3845955Z .................................................................................................... 6700/9359
2019-11-09T14:40:46.9045566Z ...................................i................................................................ 6800/9359
---
2019-11-09T14:45:38.6881425Z failures:
2019-11-09T14:45:38.6881987Z 
2019-11-09T14:45:38.6882690Z ---- [ui] ui/test-attrs/run-unexported-tests.rs stdout ----
2019-11-09T14:45:38.6883341Z 
2019-11-09T14:45:38.6883980Z error: error pattern 'ran an unexported test' not found!
2019-11-09T14:45:38.6884874Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/run-unexported-tests/a"
2019-11-09T14:45:38.6884986Z stdout:
2019-11-09T14:45:38.6885263Z ------------------------------------------
2019-11-09T14:45:38.6885304Z 
2019-11-09T14:45:38.6885304Z 
2019-11-09T14:45:38.6885378Z running 1 test
2019-11-09T14:45:38.6885429Z test m::unexported ... FAILED
2019-11-09T14:45:38.6885462Z 
2019-11-09T14:45:38.6885508Z failures:
2019-11-09T14:45:38.6885558Z 
2019-11-09T14:45:38.6885803Z ---- m::unexported stdout ----
2019-11-09T14:45:38.6886166Z thread 'm::unexported' panicked at 'ran an unexported test', /checkout/src/test/ui/test-attrs/run-unexported-tests.rs:11:9
2019-11-09T14:45:38.6886302Z 
2019-11-09T14:45:38.6886340Z 
2019-11-09T14:45:38.6886385Z failures:
2019-11-09T14:45:38.6886450Z     m::unexported
---
2019-11-09T14:45:38.6887521Z 
2019-11-09T14:45:38.6887549Z 
2019-11-09T14:45:38.6887801Z ---- [ui] ui/test-attrs/test-panic.rs stdout ----
2019-11-09T14:45:38.6887861Z 
2019-11-09T14:45:38.6888272Z error: error pattern 'thread 'test_foo' panicked at' not found!
2019-11-09T14:45:38.6888652Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic/a"
2019-11-09T14:45:38.6888719Z stdout:
2019-11-09T14:45:38.6888994Z ------------------------------------------
2019-11-09T14:45:38.6889031Z 
2019-11-09T14:45:38.6889031Z 
2019-11-09T14:45:38.6889078Z running 1 test
2019-11-09T14:45:38.6889147Z test test_foo ... FAILED
2019-11-09T14:45:38.6889180Z 
2019-11-09T14:45:38.6889223Z failures:
2019-11-09T14:45:38.6889253Z 
2019-11-09T14:45:38.6889484Z ---- test_foo stdout ----
2019-11-09T14:45:38.6889889Z thread 'test_foo' panicked at 'explicit panic', /checkout/src/test/ui/test-attrs/test-panic.rs:9:5
2019-11-09T14:45:38.6889997Z 
2019-11-09T14:45:38.6890047Z 
2019-11-09T14:45:38.6890093Z failures:
2019-11-09T14:45:38.6890140Z     test_foo
---
2019-11-09T14:45:38.6891505Z 
2019-11-09T14:45:38.6891535Z 
2019-11-09T14:45:38.6891812Z ---- [ui] ui/test-attrs/test-should-fail-bad-message.rs stdout ----
2019-11-09T14:45:38.6891853Z 
2019-11-09T14:45:38.6892119Z error: error pattern 'thread 'test_foo' panicked at' not found!
2019-11-09T14:45:38.6892519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-bad-message/a"
2019-11-09T14:45:38.6892577Z stdout:
2019-11-09T14:45:38.6892846Z ------------------------------------------
2019-11-09T14:45:38.6892883Z 
2019-11-09T14:45:38.6892883Z 
2019-11-09T14:45:38.6892939Z running 1 test
2019-11-09T14:45:38.6893007Z test test_foo ... FAILED
2019-11-09T14:45:38.6893039Z 
2019-11-09T14:45:38.6893086Z failures:
2019-11-09T14:45:38.6893115Z 
2019-11-09T14:45:38.6893483Z ---- test_foo stdout ----
2019-11-09T14:45:38.6893840Z thread 'test_foo' panicked at 'blah', /checkout/src/test/ui/test-attrs/test-should-fail-bad-message.rs:10:5
2019-11-09T14:45:38.6893911Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-09T14:45:38.6894175Z note: panic did not include expected string 'foobar'
2019-11-09T14:45:38.6894282Z failures:
2019-11-09T14:45:38.6894329Z     test_foo
2019-11-09T14:45:38.6894359Z 
2019-11-09T14:45:38.6894434Z test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---
2019-11-09T14:45:38.6895416Z 
2019-11-09T14:45:38.6895454Z 
2019-11-09T14:45:38.6895734Z ---- [ui] ui/test-attrs/test-should-panic-bad-message.rs stdout ----
2019-11-09T14:45:38.6895774Z 
2019-11-09T14:45:38.6896046Z error: error pattern 'panicked at 'bar'' not found!
2019-11-09T14:45:38.6896426Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-panic-bad-message/a"
2019-11-09T14:45:38.6896504Z stdout:
2019-11-09T14:45:38.6896750Z ------------------------------------------
2019-11-09T14:45:38.6896788Z 
2019-11-09T14:45:38.6896788Z 
2019-11-09T14:45:38.6896835Z running 1 test
2019-11-09T14:45:38.6896903Z test test_bar ... FAILED
2019-11-09T14:45:38.6896934Z 
2019-11-09T14:45:38.6896979Z failures:
2019-11-09T14:45:38.6897009Z 
2019-11-09T14:45:38.6897258Z ---- test_bar stdout ----
2019-11-09T14:45:38.6897593Z thread 'test_bar' panicked at 'bar', /checkout/src/test/ui/test-attrs/test-should-panic-bad-message.rs:9:5
2019-11-09T14:45:38.6897663Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-09T14:45:38.6897953Z note: panic did not include expected string 'foo'
2019-11-09T14:45:38.6898037Z failures:
2019-11-09T14:45:38.6898083Z     test_bar
2019-11-09T14:45:38.6898134Z 
2019-11-09T14:45:38.6898187Z test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---
2019-11-09T14:45:38.6899160Z 
2019-11-09T14:45:38.6899189Z 
2019-11-09T14:45:38.6899463Z ---- [ui] ui/test-attrs/test-should-panic-no-message.rs stdout ----
2019-11-09T14:45:38.6899588Z 
2019-11-09T14:45:38.6899916Z error: error pattern 'panicked at 'explicit panic'' not found!
2019-11-09T14:45:38.6900309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-panic-no-message/a"
2019-11-09T14:45:38.6900388Z stdout:
2019-11-09T14:45:38.6900635Z ------------------------------------------
2019-11-09T14:45:38.6900671Z 
---
2019-11-09T14:45:38.6900893Z 
2019-11-09T14:45:38.6901147Z ---- test_explicit stdout ----
2019-11-09T14:45:38.6901487Z thread 'test_explicit' panicked at 'explicit panic', /checkout/src/test/ui/test-attrs/test-should-panic-no-message.rs:9:5
2019-11-09T14:45:38.6901557Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-09T14:45:38.6901848Z note: panic did not include expected string 'foo'
2019-11-09T14:45:38.6901932Z failures:
2019-11-09T14:45:38.6901978Z     test_explicit
2019-11-09T14:45:38.6902030Z 
2019-11-09T14:45:38.6902174Z test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---
2019-11-09T14:45:38.6903265Z 
2019-11-09T14:45:38.6903294Z 
2019-11-09T14:45:38.6903541Z ---- [ui] ui/unwind-interleaved.rs stdout ----
2019-11-09T14:45:38.6903599Z 
2019-11-09T14:45:38.6903843Z error: error pattern 'fail' not found!
2019-11-09T14:45:38.6904197Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-interleaved/a"
2019-11-09T14:45:38.6904288Z stdout:
2019-11-09T14:45:38.6904539Z ------------------------------------------
2019-11-09T14:45:38.6904576Z 
---
2019-11-09T14:45:38.6905881Z 
2019-11-09T14:45:38.6905911Z 
2019-11-09T14:45:38.6906155Z ---- [ui] ui/unwind-rec.rs stdout ----
2019-11-09T14:45:38.6906193Z 
2019-11-09T14:45:38.6906430Z error: error pattern 'fail' not found!
2019-11-09T14:45:38.6906792Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-rec/a"
2019-11-09T14:45:38.6906858Z stdout:
2019-11-09T14:45:38.6907126Z ------------------------------------------
2019-11-09T14:45:38.6907164Z 
---
2019-11-09T14:45:38.6908435Z 
2019-11-09T14:45:38.6908464Z 
2019-11-09T14:45:38.6908726Z ---- [ui] ui/unwind-rec2.rs stdout ----
2019-11-09T14:45:38.6908764Z 
2019-11-09T14:45:38.6909006Z error: error pattern 'fail' not found!
2019-11-09T14:45:38.6909370Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-rec2/a"
2019-11-09T14:45:38.6909519Z stdout:
2019-11-09T14:45:38.6909799Z ------------------------------------------
2019-11-09T14:45:38.6909837Z 
---
2019-11-09T14:45:38.6911139Z 
2019-11-09T14:45:38.6911168Z 
2019-11-09T14:45:38.6911496Z ---- [ui] ui/unwind-unique-panic.rs stdout ----
2019-11-09T14:45:38.6911535Z 
2019-11-09T14:45:38.6911796Z error: error pattern 'fail' not found!
2019-11-09T14:45:38.6912154Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-unique-panic/a"
2019-11-09T14:45:38.6912231Z stdout:
2019-11-09T14:45:38.6912477Z ------------------------------------------
2019-11-09T14:45:38.6912514Z 
---
2019-11-09T14:45:38.6917064Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-09T14:45:38.6917130Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-09T14:45:38.6917166Z 
2019-11-09T14:45:38.6917195Z 
2019-11-09T14:45:38.6918980Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-09T14:45:38.6919264Z 
2019-11-09T14:45:38.6919318Z 
2019-11-09T14:45:38.6919372Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-09T14:45:38.6919429Z Build completed unsuccessfully in 1:03:45
2019-11-09T14:45:38.6919429Z Build completed unsuccessfully in 1:03:45
2019-11-09T14:45:38.6919499Z == clock drift check ==
2019-11-09T14:45:38.6919549Z   local time: Sat Nov  9 14:45:38 UTC 2019
2019-11-09T14:45:38.6919603Z   network time: Sat, 09 Nov 2019 14:45:38 GMT
2019-11-09T14:45:38.6919673Z == end clock drift check ==
2019-11-09T14:45:39.1693267Z 
2019-11-09T14:45:39.1812949Z ##[error]Bash exited with code '1'.
2019-11-09T14:45:39.1924177Z ##[section]Starting: Checkout
2019-11-09T14:45:39.1926065Z ==============================================================================
2019-11-09T14:45:39.1926130Z Task         : Get sources
2019-11-09T14:45:39.1926203Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
