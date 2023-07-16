plain
2020-01-06T16:46:01.5119546Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T16:46:02.1772066Z ##[command]git config gc.auto 0
2020-01-06T16:46:02.1777784Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T16:46:02.1781995Z ##[command]git config --get-all http.proxy
2020-01-06T16:46:02.1786678Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67930/merge:refs/remotes/pull/67930/merge
---
2020-01-06T17:38:47.5945443Z .................................................................................................... 1500/9477
2020-01-06T17:38:53.5117158Z .................................................................................................... 1600/9477
2020-01-06T17:38:58.5105996Z .................................................................................................... 1700/9477
2020-01-06T17:39:07.8392606Z .................................................................................................... 1800/9477
2020-01-06T17:39:15.9965253Z .i.................................................................................................. 1900/9477
2020-01-06T17:39:22.6633056Z .........................................................................................iiiii...... 2000/9477
2020-01-06T17:39:44.9860313Z .................................................................................................... 2200/9477
2020-01-06T17:39:47.2976912Z .................................................................................................... 2300/9477
2020-01-06T17:39:49.7160888Z .................................................................................................... 2400/9477
2020-01-06T17:39:55.8628734Z .................................................................................................... 2500/9477
---
2020-01-06T17:42:30.0920971Z .................................................................................................... 4500/9477
2020-01-06T17:42:35.9595317Z .................................................................................................... 4600/9477
2020-01-06T17:42:41.6611969Z .................................................................................................... 4700/9477
2020-01-06T17:42:49.2056421Z .................................................................................................... 4800/9477
2020-01-06T17:42:54.6087686Z .....................i...............i........................F.FF.F................................ 4900/9477
2020-01-06T17:43:10.4594041Z ..................................................................i................................. 5100/9477
2020-01-06T17:43:18.4396828Z .................................................................................................... 5200/9477
2020-01-06T17:43:18.4396828Z .................................................................................................... 5200/9477
2020-01-06T17:43:26.1780912Z .................................ii.ii...........i.................................................. 5300/9477
2020-01-06T17:43:35.9098780Z .................................................................................................... 5500/9477
2020-01-06T17:43:45.7963263Z .................................................................................................... 5600/9477
2020-01-06T17:43:53.4659284Z .................i.................................................................................. 5700/9477
2020-01-06T17:44:00.0541006Z .................................................................................................... 5800/9477
2020-01-06T17:44:00.0541006Z .................................................................................................... 5800/9477
2020-01-06T17:44:11.5417313Z .................................................................................................... 5900/9477
2020-01-06T17:44:23.5923042Z .......ii...i..ii...........i....................................................................... 6000/9477
2020-01-06T17:44:41.5515810Z .................................................................................................... 6200/9477
2020-01-06T17:44:46.0879462Z .................................................................................................... 6300/9477
2020-01-06T17:44:46.0879462Z .................................................................................................... 6300/9477
2020-01-06T17:44:59.2703652Z ..................................i..ii............................................................. 6400/9477
2020-01-06T17:45:20.0161692Z .................................................................................................... 6600/9477
2020-01-06T17:45:22.1532913Z .........i.......................................................................................... 6700/9477
2020-01-06T17:45:24.5259271Z .................................................................................................... 6800/9477
2020-01-06T17:45:27.0893265Z .........i.......................................................................................... 6900/9477
---
2020-01-06T17:47:03.9006913Z .................................................................................................... 7500/9477
2020-01-06T17:47:08.0189834Z .................................................................................................... 7600/9477
2020-01-06T17:47:13.3379974Z .................................................................................................... 7700/9477
2020-01-06T17:47:24.1615561Z .................................................................................................... 7800/9477
2020-01-06T17:47:32.4595780Z .............................................iiii................................................... 7900/9477
2020-01-06T17:47:47.6368256Z .................................................................................................... 8100/9477
2020-01-06T17:47:54.4754273Z .................................................................................................... 8200/9477
2020-01-06T17:48:10.1906408Z .................................................................................................... 8300/9477
2020-01-06T17:48:17.9642637Z .................................................................................................... 8400/9477
---
2020-01-06T17:50:15.3612166Z diff of stderr:
2020-01-06T17:50:15.3612356Z 
2020-01-06T17:50:15.3612971Z 2   --> $DIR/result-as_deref_err.rs:4:28
2020-01-06T17:50:15.3613406Z 3    |
2020-01-06T17:50:15.3613629Z 4 LL |     let _result = &Err(41).as_deref_err();
2020-01-06T17:50:15.3615290Z -    |                            ^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_ok`
2020-01-06T17:50:15.3615521Z 6    |
2020-01-06T17:50:15.3615595Z 7    = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3615595Z 7    = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3615666Z 8            `{integer} : std::ops::Deref`
2020-01-06T17:50:15.3615730Z 
2020-01-06T17:50:15.3615795Z The actual stderr differed from the expected stderr.
2020-01-06T17:50:15.3616166Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err/result-as_deref_err.stderr
2020-01-06T17:50:15.3616166Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err/result-as_deref_err.stderr
2020-01-06T17:50:15.3616444Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T17:50:15.3616775Z To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_err.rs`
2020-01-06T17:50:15.3616864Z error: 1 errors occurred comparing output.
2020-01-06T17:50:15.3616929Z status: exit code: 1
2020-01-06T17:50:15.3616929Z status: exit code: 1
2020-01-06T17:50:15.3618066Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err/auxiliary" "-A" "unused"
2020-01-06T17:50:15.3618546Z ------------------------------------------
2020-01-06T17:50:15.3618582Z 
2020-01-06T17:50:15.3618823Z ------------------------------------------
2020-01-06T17:50:15.3618871Z stderr:
2020-01-06T17:50:15.3618871Z stderr:
2020-01-06T17:50:15.3619091Z ------------------------------------------
2020-01-06T17:50:15.3619168Z error[E0599]: no method named `as_deref_err` found for type `std::result::Result<_, {integer}>` in the current scope
2020-01-06T17:50:15.3619526Z    |
2020-01-06T17:50:15.3619526Z    |
2020-01-06T17:50:15.3619592Z LL |     let _result = &Err(41).as_deref_err();
2020-01-06T17:50:15.3619863Z    |
2020-01-06T17:50:15.3619939Z    = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3619939Z    = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3619995Z            `{integer} : std::ops::Deref`
2020-01-06T17:50:15.3620074Z error: aborting due to previous error
2020-01-06T17:50:15.3620119Z 
2020-01-06T17:50:15.3620380Z For more information about this error, try `rustc --explain E0599`.
2020-01-06T17:50:15.3620417Z 
2020-01-06T17:50:15.3620417Z 
2020-01-06T17:50:15.3620636Z ------------------------------------------
2020-01-06T17:50:15.3620686Z 
2020-01-06T17:50:15.3620713Z 
2020-01-06T17:50:15.3620993Z ---- [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok.rs stdout ----
2020-01-06T17:50:15.3621047Z diff of stderr:
2020-01-06T17:50:15.3621093Z 
2020-01-06T17:50:15.3621135Z 3    |
2020-01-06T17:50:15.3621186Z 4 LL |     let _result = &mut Ok(42).as_deref_mut_ok();
2020-01-06T17:50:15.3621471Z -    |
2020-01-06T17:50:15.3621759Z -    = note: the method `as_deref_mut_ok` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3621759Z -    = note: the method `as_deref_mut_ok` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3622010Z -            `{integer} : std::ops::DerefMut`
2020-01-06T17:50:15.3622108Z 10 error: aborting due to previous error
2020-01-06T17:50:15.3622155Z 11 
2020-01-06T17:50:15.3622199Z 
2020-01-06T17:50:15.3622227Z 
2020-01-06T17:50:15.3622227Z 
2020-01-06T17:50:15.3622275Z The actual stderr differed from the expected stderr.
2020-01-06T17:50:15.3622645Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok/result-as_deref_mut_ok.stderr
2020-01-06T17:50:15.3622932Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T17:50:15.3623248Z To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok.rs`
2020-01-06T17:50:15.3623352Z error: 1 errors occurred comparing output.
2020-01-06T17:50:15.3623409Z status: exit code: 1
2020-01-06T17:50:15.3623409Z status: exit code: 1
2020-01-06T17:50:15.3624470Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok/auxiliary" "-A" "unused"
2020-01-06T17:50:15.3624833Z ------------------------------------------
2020-01-06T17:50:15.3624868Z 
2020-01-06T17:50:15.3625157Z ------------------------------------------
2020-01-06T17:50:15.3625205Z stderr:
2020-01-06T17:50:15.3625205Z stderr:
2020-01-06T17:50:15.3625446Z ------------------------------------------
2020-01-06T17:50:15.3625509Z error[E0599]: no method named `as_deref_mut_ok` found for type `std::result::Result<{integer}, _>` in the current scope
2020-01-06T17:50:15.3625875Z    |
2020-01-06T17:50:15.3625875Z    |
2020-01-06T17:50:15.3625927Z LL |     let _result = &mut Ok(42).as_deref_mut_ok();
2020-01-06T17:50:15.3626052Z 
2020-01-06T17:50:15.3626097Z error: aborting due to previous error
2020-01-06T17:50:15.3626130Z 
2020-01-06T17:50:15.3626387Z For more information about this error, try `rustc --explain E0599`.
---
2020-01-06T17:50:15.3627445Z 4 LL |     let _result = &mut Err(41).as_deref_mut_err();
2020-01-06T17:50:15.3627752Z -    |                                ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_mut_ok`
2020-01-06T17:50:15.3627844Z +    |                                ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_mut`
2020-01-06T17:50:15.3627895Z 6    |
2020-01-06T17:50:15.3627950Z 7    = note: the method `as_deref_mut_err` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3628023Z 8            `{integer} : std::ops::DerefMut`
2020-01-06T17:50:15.3628090Z 
2020-01-06T17:50:15.3628139Z The actual stderr differed from the expected stderr.
2020-01-06T17:50:15.3628518Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err/result-as_deref_mut_err.stderr
2020-01-06T17:50:15.3628518Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err/result-as_deref_mut_err.stderr
2020-01-06T17:50:15.3628782Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T17:50:15.3629113Z To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_mut_err.rs`
2020-01-06T17:50:15.3629209Z error: 1 errors occurred comparing output.
2020-01-06T17:50:15.3629259Z status: exit code: 1
2020-01-06T17:50:15.3629259Z status: exit code: 1
2020-01-06T17:50:15.3630399Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err/auxiliary" "-A" "unused"
2020-01-06T17:50:15.3630758Z ------------------------------------------
2020-01-06T17:50:15.3630793Z 
2020-01-06T17:50:15.3631089Z ------------------------------------------
2020-01-06T17:50:15.3631143Z stderr:
2020-01-06T17:50:15.3631143Z stderr:
2020-01-06T17:50:15.3631376Z ------------------------------------------
2020-01-06T17:50:15.3631437Z error[E0599]: no method named `as_deref_mut_err` found for type `std::result::Result<_, {integer}>` in the current scope
2020-01-06T17:50:15.3631859Z    |
2020-01-06T17:50:15.3631908Z LL |     let _result = &mut Err(41).as_deref_mut_err();
2020-01-06T17:50:15.3631986Z    |                                ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_mut`
2020-01-06T17:50:15.3632037Z    |
2020-01-06T17:50:15.3632037Z    |
2020-01-06T17:50:15.3632092Z    = note: the method `as_deref_mut_err` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3632163Z            `{integer} : std::ops::DerefMut`
2020-01-06T17:50:15.3632249Z error: aborting due to previous error
2020-01-06T17:50:15.3632280Z 
2020-01-06T17:50:15.3632557Z For more information about this error, try `rustc --explain E0599`.
2020-01-06T17:50:15.3632593Z 
2020-01-06T17:50:15.3632593Z 
2020-01-06T17:50:15.3632815Z ------------------------------------------
2020-01-06T17:50:15.3632847Z 
2020-01-06T17:50:15.3632890Z 
2020-01-06T17:50:15.3633157Z ---- [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs stdout ----
2020-01-06T17:50:15.3633219Z diff of stderr:
2020-01-06T17:50:15.3633249Z 
2020-01-06T17:50:15.3633307Z 3    |
2020-01-06T17:50:15.3633356Z 4 LL |     let _result = &Ok(42).as_deref_ok();
2020-01-06T17:50:15.3633624Z -    |
2020-01-06T17:50:15.3633901Z -    = note: the method `as_deref_ok` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3633901Z -    = note: the method `as_deref_ok` exists but the following trait bounds were not satisfied:
2020-01-06T17:50:15.3634132Z -            `{integer} : std::ops::Deref`
2020-01-06T17:50:15.3634249Z 10 error: aborting due to previous error
2020-01-06T17:50:15.3634295Z 11 
2020-01-06T17:50:15.3634325Z 
2020-01-06T17:50:15.3634367Z 
2020-01-06T17:50:15.3634367Z 
2020-01-06T17:50:15.3634417Z The actual stderr differed from the expected stderr.
2020-01-06T17:50:15.3634772Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok/result-as_deref_ok.stderr
2020-01-06T17:50:15.3635059Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T17:50:15.3635371Z To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs`
2020-01-06T17:50:15.3635473Z error: 1 errors occurred comparing output.
2020-01-06T17:50:15.3635522Z status: exit code: 1
2020-01-06T17:50:15.3635522Z status: exit code: 1
2020-01-06T17:50:15.3636499Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok/auxiliary" "-A" "unused"
2020-01-06T17:50:15.3636853Z ------------------------------------------
2020-01-06T17:50:15.3636904Z 
2020-01-06T17:50:15.3637128Z ------------------------------------------
2020-01-06T17:50:15.3637181Z stderr:
2020-01-06T17:50:15.3637181Z stderr:
2020-01-06T17:50:15.3637417Z ------------------------------------------
2020-01-06T17:50:15.3637480Z error[E0599]: no method named `as_deref_ok` found for type `std::result::Result<{integer}, _>` in the current scope
2020-01-06T17:50:15.3637896Z    |
2020-01-06T17:50:15.3637896Z    |
2020-01-06T17:50:15.3637945Z LL |     let _result = &Ok(42).as_deref_ok();
2020-01-06T17:50:15.3638041Z 
2020-01-06T17:50:15.3638150Z error: aborting due to previous error
2020-01-06T17:50:15.3638182Z 
2020-01-06T17:50:15.3638445Z For more information about this error, try `rustc --explain E0599`.
---
2020-01-06T17:50:15.3640725Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-06T17:50:15.3640795Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-06T17:50:15.3646265Z 
2020-01-06T17:50:15.3646371Z 
2020-01-06T17:50:15.3649206Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-06T17:50:15.3649497Z 
2020-01-06T17:50:15.3649528Z 
2020-01-06T17:50:15.3649579Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-06T17:50:15.3650259Z Build completed unsuccessfully in 0:57:29
2020-01-06T17:50:15.3650259Z Build completed unsuccessfully in 0:57:29
2020-01-06T17:50:15.3696823Z == clock drift check ==
2020-01-06T17:50:15.3714550Z   local time: Mon Jan  6 17:50:15 UTC 2020
2020-01-06T17:50:15.9189449Z   network time: Mon, 06 Jan 2020 17:50:15 GMT
2020-01-06T17:50:15.9191174Z == end clock drift check ==
2020-01-06T17:50:16.8214720Z 
2020-01-06T17:50:16.8348706Z ##[error]Bash exited with code '1'.
2020-01-06T17:50:16.8399500Z ##[section]Starting: Checkout
2020-01-06T17:50:16.8401811Z ==============================================================================
2020-01-06T17:50:16.8401872Z Task         : Get sources
2020-01-06T17:50:16.8401944Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
