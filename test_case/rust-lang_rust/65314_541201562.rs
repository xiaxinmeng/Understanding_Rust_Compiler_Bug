plain
2019-10-11T18:02:50.3561203Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T18:02:50.3648653Z ##[command]git config gc.auto 0
2019-10-11T18:02:50.3728474Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T18:02:50.3784007Z ##[command]git config --get-all http.proxy
2019-10-11T18:02:50.3919301Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65314/merge:refs/remotes/pull/65314/merge
---
2019-10-11T19:01:26.3791443Z .................................................................................................... 1600/9146
2019-10-11T19:01:33.5137157Z .................................................................................................... 1700/9146
2019-10-11T19:01:44.6191955Z .................i...............i.................................................................. 1800/9146
2019-10-11T19:01:51.5649302Z .................................................................................................... 1900/9146
2019-10-11T19:02:06.0758927Z ........iiiii....................................................................................... 2000/9146
2019-10-11T19:02:15.5925655Z .................................................................................................... 2200/9146
2019-10-11T19:02:18.1150504Z .................................................................................................... 2300/9146
2019-10-11T19:02:23.6274596Z .................................................................................................... 2400/9146
2019-10-11T19:02:29.4994002Z .................................................................................................... 2500/9146
---
2019-10-11T19:05:14.6488193Z .................................................................................................... 4700/9146
2019-10-11T19:05:21.5765429Z .i...............i.................................................................................. 4800/9146
2019-10-11T19:05:32.3784603Z .................................................................................................... 4900/9146
2019-10-11T19:05:37.6243290Z .................................................................................................... 5000/9146
2019-10-11T19:05:48.3503167Z ...............................................................................................ii.ii 5100/9146
2019-10-11T19:05:58.2912230Z .................................................................................................... 5300/9146
2019-10-11T19:06:07.5410503Z .................................................................................................... 5400/9146
2019-10-11T19:06:14.1113410Z .............................................................i...................................... 5500/9146
2019-10-11T19:06:21.1855575Z .................................................................................................... 5600/9146
2019-10-11T19:06:21.1855575Z .................................................................................................... 5600/9146
2019-10-11T19:06:28.2673168Z .................................................................................................... 5700/9146
2019-10-11T19:06:37.9440900Z ..........................................................ii...i..ii...........i.................... 5800/9146
2019-10-11T19:07:02.7120178Z .................................................................................................... 6000/9146
2019-10-11T19:07:11.5169203Z .................................................................................................... 6100/9146
2019-10-11T19:07:11.5169203Z .................................................................................................... 6100/9146
2019-10-11T19:07:18.3222530Z ................................................................i..ii............................... 6200/9146
2019-10-11T19:07:46.6868160Z .................................................................................................... 6400/9146
2019-10-11T19:07:48.8461004Z ........................i........................................................................... 6500/9146
2019-10-11T19:07:50.9494459Z .................................................................................................i.. 6600/9146
2019-10-11T19:07:53.4776146Z .................................................................................................... 6700/9146
---
2019-10-11T19:12:21.4033954Z  finished in 5.479
2019-10-11T19:12:21.4197269Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T19:12:21.6118579Z 
2019-10-11T19:12:21.6121631Z running 153 tests
2019-10-11T19:12:24.8391907Z i....iii......iii..iiii....i.............................i..i..................i....i............ii. 100/153
2019-10-11T19:12:26.7596984Z i.i..iiii..............i.........iii.i.......ii......
2019-10-11T19:12:26.7602091Z 
2019-10-11T19:12:26.7605572Z  finished in 5.340
2019-10-11T19:12:26.7775038Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T19:12:26.9305330Z 
---
2019-10-11T19:12:28.8655489Z  finished in 2.087
2019-10-11T19:12:28.8818264Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T19:12:29.0371658Z 
2019-10-11T19:12:29.0371807Z running 9 tests
2019-10-11T19:12:29.0372684Z iiiiiiiii
2019-10-11T19:12:29.0373496Z 
2019-10-11T19:12:29.0373537Z  finished in 0.155
2019-10-11T19:12:29.0570373Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T19:12:29.2443271Z 
---
2019-10-11T19:12:46.5128464Z  finished in 17.455
2019-10-11T19:12:46.5363667Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T19:12:46.7280060Z 
2019-10-11T19:12:46.7280237Z running 123 tests
2019-10-11T19:13:09.5757266Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-11T19:13:13.9514750Z i.i.i......iii.i.....ii
2019-10-11T19:13:13.9515895Z 
2019-10-11T19:13:13.9519736Z  finished in 27.415
2019-10-11T19:13:13.9528987Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T19:13:13.9531270Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-11T19:25:51.6118303Z 
2019-10-11T19:25:51.6119090Z    Doc-tests core
2019-10-11T19:25:56.6258227Z 
2019-10-11T19:25:56.6260218Z running 2405 tests
2019-10-11T19:26:07.4023466Z ......iiiii......................................................................................... 100/2405
2019-10-11T19:26:17.7059477Z ...............................................................................ii................... 200/2405
2019-10-11T19:26:42.4268968Z .i.................................................................................................. 400/2405
2019-10-11T19:26:42.4268968Z .i.................................................................................................. 400/2405
2019-10-11T19:26:52.7840403Z ................................................i..i.................iiii........................... 500/2405
2019-10-11T19:27:12.4204430Z .................................................................................................... 700/2405
2019-10-11T19:27:22.4147481Z .................................................................................................... 800/2405
2019-10-11T19:27:32.4885973Z .................................................................................................... 900/2405
2019-10-11T19:27:42.5075068Z .................................................................................................... 1000/2405
---
2019-10-11T19:31:38.2000554Z .................................................................................................... 500/763
2019-10-11T19:31:38.2243168Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-11T19:31:38.2277054Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"thread '<unnamed>' panicked at '', src/libcore/result.rs:1165:5
2019-10-11T19:31:38.2280869Z called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-11T19:31:38.2318296Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-11T19:31:38.5011714Z ..........................................thread '.<unnamed>..' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError.', src/libcore/result.rs:1165:5
2019-10-11T19:31:38.5032476Z thread '<unnamed>.' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)".', src/libcore/result.rs:1165:5.
2019-10-11T19:31:38.5043306Z ..thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-11T19:31:38.5051911Z ..thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-11T19:31:40.5464061Z ......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-10-11T19:31:40.5470467Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-10-11T19:31:40.5481201Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-10-11T19:31:40.5482360Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-10-11T19:31:49.8577901Z 
2019-10-11T19:31:49.8578408Z running 994 tests
2019-10-11T19:32:11.0339344Z i................................................................................................... 100/994
2019-10-11T19:32:22.5062928Z .................................................................................................... 200/994
2019-10-11T19:32:30.8580657Z ...................iii......i......i...i......i..................................................... 300/994
2019-10-11T19:32:36.6865613Z .................................................................................................... 400/994
2019-10-11T19:32:44.4670561Z .....................................i..i.................................ii........................ 500/994
2019-10-11T19:32:59.4966809Z .................................................................................................... 700/994
2019-10-11T19:32:59.4966809Z .................................................................................................... 700/994
2019-10-11T19:33:07.8140551Z ....................iiii............................................................................ 800/994
2019-10-11T19:33:23.3735183Z .................................................................................................... 900/994
2019-10-11T19:33:31.1995104Z ..........................................iiii................................................
2019-10-11T19:33:31.1996429Z 
2019-10-11T19:33:31.2092186Z  finished in 194.068
2019-10-11T19:33:31.2110303Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T19:33:31.4205489Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-10-11T19:49:14.0429776Z  finished in 39.723
2019-10-11T19:49:14.0779116Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T19:49:14.2994298Z 
2019-10-11T19:49:14.2995450Z running 202 tests
2019-10-11T19:49:49.4842136Z ....................i...ii.................................................................i........ 100/202
2019-10-11T19:50:40.1944918Z ................................iiii.......i...........iiii.iii....................................i 200/202
2019-10-11T19:50:43.0061957Z test result: ok. 185 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out
2019-10-11T19:50:43.0062329Z 
2019-10-11T19:50:43.0067198Z  finished in 88.929
2019-10-11T19:50:43.0078483Z doc tests for: /checkout/src/doc/rustdoc/src/command-line-arguments.md
---
2019-10-11T19:51:12.8675569Z ---- [ui] rustdoc-ui/failed-doctest-missing-codes.rs stdout ----
2019-10-11T19:51:12.8675616Z diff of stdout:
2019-10-11T19:51:12.8675646Z 
2019-10-11T19:51:12.8675699Z 6 
2019-10-11T19:51:12.8675918Z 7 ---- $DIR/failed-doctest-missing-codes.rs - Foo (line 8) stdout ----
2019-10-11T19:51:12.8676189Z -  --> $DIR/failed-doctest-missing-codes.rs:9:13
2019-10-11T19:51:12.8676534Z -   |
2019-10-11T19:51:12.8676534Z -   |
2019-10-11T19:51:12.8676704Z - 3 | let x: () = 5i32;
2019-10-11T19:51:12.8676910Z -   |             ^^^^ expected (), found i32
2019-10-11T19:51:12.8677776Z -   = note: expected type `()`
2019-10-11T19:51:12.8678086Z -              found type `i32`
2019-10-11T19:51:12.8678329Z +   --> $DIR/failed-doctest-missing-codes.rs:9:13
2019-10-11T19:51:12.8678379Z +    |
---
2019-10-11T19:51:12.8678773Z 18 
2019-10-11T19:51:12.8678819Z 
2019-10-11T19:51:12.8678845Z 
2019-10-11T19:51:12.8678891Z The actual stdout differed from the expected stdout.
2019-10-11T19:51:12.8679553Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-missing-codes/failed-doctest-missing-codes.stdout
2019-10-11T19:51:12.8679974Z To update references, rerun the tests and pass the `--bless` flag
2019-10-11T19:51:12.8680337Z To only update this specific test, also pass `--test-args failed-doctest-missing-codes.rs`
2019-10-11T19:51:12.8680446Z error: 1 errors occurred comparing output.
2019-10-11T19:51:12.8680495Z status: exit code: 101
2019-10-11T19:51:12.8680495Z status: exit code: 101
2019-10-11T19:51:12.8681600Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-missing-codes" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-missing-codes/auxiliary"
2019-10-11T19:51:12.8682638Z ------------------------------------------
2019-10-11T19:51:12.8682691Z 
2019-10-11T19:51:12.8682745Z running 1 test
2019-10-11T19:51:12.8683013Z test /checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs - Foo (line 8) ... FAILED
---
2019-10-11T19:51:12.8684303Z 
2019-10-11T19:51:12.8684341Z error: aborting due to previous error
2019-10-11T19:51:12.8684367Z 
2019-10-11T19:51:12.8684774Z For more information about this error, try `rustc --explain E0308`.
2019-10-11T19:51:12.8684821Z Some expected error codes were not found: ["E0004"]
2019-10-11T19:51:12.8684897Z failures:
2019-10-11T19:51:12.8685121Z     /checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs - Foo (line 8)
2019-10-11T19:51:12.8685152Z 
2019-10-11T19:51:12.8685194Z test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---
2019-10-11T19:51:12.8686324Z ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
2019-10-11T19:51:12.8686366Z diff of stdout:
2019-10-11T19:51:12.8686390Z 
2019-10-11T19:51:12.8686445Z 7 
2019-10-11T19:51:12.8686658Z 8 ---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-10-11T19:51:12.8686703Z 9 error[E0425]: cannot find value `no` in this scope
2019-10-11T19:51:12.8687065Z -   |
2019-10-11T19:51:12.8687221Z - 3 | no
2019-10-11T19:51:12.8687394Z -   | ^^ not found in this scope
2019-10-11T19:51:12.8687973Z +   --> $DIR/failed-doctest-output.rs:22:1
---
2019-10-11T19:51:12.8688348Z 16 
2019-10-11T19:51:12.8688552Z 
2019-10-11T19:51:12.8688579Z 
2019-10-11T19:51:12.8688626Z The actual stdout differed from the expected stdout.
2019-10-11T19:51:12.8689099Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
2019-10-11T19:51:12.8689422Z To update references, rerun the tests and pass the `--bless` flag
2019-10-11T19:51:12.8689697Z To only update this specific test, also pass `--test-args failed-doctest-output.rs`
2019-10-11T19:51:12.8689797Z error: 1 errors occurred comparing output.
2019-10-11T19:51:12.8689844Z status: exit code: 101
2019-10-11T19:51:12.8689844Z status: exit code: 101
2019-10-11T19:51:12.8690555Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
2019-10-11T19:51:12.8690909Z ------------------------------------------
2019-10-11T19:51:12.8690960Z 
2019-10-11T19:51:12.8691005Z running 2 tests
2019-10-11T19:51:12.8691594Z test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
---
2019-10-11T19:51:12.8693897Z 
2019-10-11T19:51:12.8693948Z stderr:
2019-10-11T19:51:12.8693981Z stderr 1
2019-10-11T19:51:12.8694014Z stderr 2
2019-10-11T19:51:12.8694435Z thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:7:1
2019-10-11T19:51:12.8694522Z 
2019-10-11T19:51:12.8694544Z 
2019-10-11T19:51:12.8694566Z 
2019-10-11T19:51:12.8694619Z failures:
---
2019-10-11T19:51:12.8696429Z ---- [ui] rustdoc-ui/unparseable-doc-test.rs stdout ----
2019-10-11T19:51:12.8696590Z diff of stdout:
2019-10-11T19:51:12.8696615Z 
2019-10-11T19:51:12.8696650Z 6 
2019-10-11T19:51:12.8696968Z 7 ---- $DIR/unparseable-doc-test.rs - foo (line 6) stdout ----
2019-10-11T19:51:12.8697042Z 8 error: unterminated double quote string
2019-10-11T19:51:12.8697267Z -  --> $DIR/unparseable-doc-test.rs:8:1
2019-10-11T19:51:12.8697873Z -   |
2019-10-11T19:51:12.8698140Z - 2 | "unterminated
2019-10-11T19:51:12.8698576Z +   --> $DIR/unparseable-doc-test.rs:8:1
2019-10-11T19:51:12.8698622Z +    |
2019-10-11T19:51:12.8698682Z + LL | "unterminated
2019-10-11T19:51:12.8698726Z +    | ^^^^^^^^^^^^^
2019-10-11T19:51:12.8698726Z +    | ^^^^^^^^^^^^^
2019-10-11T19:51:12.8698768Z 13 
2019-10-11T19:51:12.8698830Z 14 error: aborting due to previous error
2019-10-11T19:51:12.8698871Z 15 
2019-10-11T19:51:12.8698900Z 
2019-10-11T19:51:12.8698925Z 
2019-10-11T19:51:12.8698991Z The actual stdout differed from the expected stdout.
2019-10-11T19:51:12.8699323Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test/unparseable-doc-test.stdout
2019-10-11T19:51:12.8699585Z To update references, rerun the tests and pass the `--bless` flag
2019-10-11T19:51:12.8699915Z To only update this specific test, also pass `--test-args unparseable-doc-test.rs`
2019-10-11T19:51:12.8700004Z error: 1 errors occurred comparing output.
2019-10-11T19:51:12.8700054Z status: exit code: 101
2019-10-11T19:51:12.8700054Z status: exit code: 101
2019-10-11T19:51:12.8700799Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unparseable-doc-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test/auxiliary"
2019-10-11T19:51:12.8701452Z ------------------------------------------
2019-10-11T19:51:12.8701481Z 
2019-10-11T19:51:12.8701518Z running 1 test
2019-10-11T19:51:12.8701762Z test /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - foo (line 6) ... FAILED
---
2019-10-11T19:51:12.8704928Z test result: FAILED. 28 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-11T19:51:12.8704977Z 
2019-10-11T19:51:12.8704998Z 
2019-10-11T19:51:12.8705019Z 
2019-10-11T19:51:12.8706796Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-11T19:51:12.8707044Z 
2019-10-11T19:51:12.8707070Z 
2019-10-11T19:51:12.8707357Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-11T19:51:12.8707409Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-11T19:51:12.8707409Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-11T19:51:12.8723521Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-11T19:51:12.8723637Z Build completed unsuccessfully in 1:42:13
2019-10-11T19:51:12.8774240Z == clock drift check ==
2019-10-11T19:51:12.8785351Z   local time: Fri Oct 11 19:51:12 UTC 2019
2019-10-11T19:51:12.9628557Z   network time: Fri, 11 Oct 2019 19:51:12 GMT
2019-10-11T19:51:12.9630788Z == end clock drift check ==
2019-10-11T19:51:13.4955697Z ##[error]Bash exited with code '1'.
2019-10-11T19:51:13.5013090Z ##[section]Starting: Checkout
2019-10-11T19:51:13.5015282Z ==============================================================================
2019-10-11T19:51:13.5015360Z Task         : Get sources
2019-10-11T19:51:13.5015404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
