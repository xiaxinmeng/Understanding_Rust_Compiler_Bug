plain
2019-08-12T13:05:15.1176228Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T13:05:15.1376462Z ##[command]git config gc.auto 0
2019-08-12T13:05:15.1440652Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T13:05:15.1503096Z ##[command]git config --get-all http.proxy
2019-08-12T13:05:15.1652792Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63492/merge:refs/remotes/pull/63492/merge
---
2019-08-12T13:05:51.0874513Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T13:05:51.0874538Z 
2019-08-12T13:05:51.0874709Z   git checkout -b <new-branch-name>
2019-08-12T13:05:51.0874733Z 
2019-08-12T13:05:51.0874768Z HEAD is now at e8cf4572c Merge 7e57d8f396278e2c648e59e5c7b7dc9d2fb1028d into c01be67ea40266d6a4c3289654a07ddd7ce2a172
2019-08-12T13:05:51.1009929Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T13:05:51.1013424Z ==============================================================================
2019-08-12T13:05:51.1013483Z Task         : Bash
2019-08-12T13:05:51.1013531Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-12T14:07:36.2872190Z .................................................................................................... 1300/8876
2019-08-12T14:07:42.9184971Z .................................................................................................... 1400/8876
2019-08-12T14:07:49.2716849Z .................................................................................................... 1500/8876
2019-08-12T14:08:00.2197534Z .....................................................................................i.............. 1600/8876
2019-08-12T14:08:07.9420628Z .i.................................................................................................. 1700/8876
2019-08-12T14:08:14.8875195Z ............................................................................iiiii................... 1800/8876
2019-08-12T14:08:37.4366973Z .................................................................................................... 2000/8876
2019-08-12T14:08:39.9393178Z .................................................................................................... 2100/8876
2019-08-12T14:08:42.7105612Z .................................................................................................... 2200/8876
2019-08-12T14:08:50.6923324Z .................................................................................................... 2300/8876
---
2019-08-12T14:12:52.1833922Z .................................................................................................... 5300/8876
2019-08-12T14:12:59.8241492Z .........i.......................................................................................... 5400/8876
2019-08-12T14:13:05.5175965Z .................................................................................................... 5500/8876
2019-08-12T14:13:18.0391983Z .................................................................................................... 5600/8876
2019-08-12T14:13:32.3676642Z ....ii...i..ii...........i.......................................................................... 5700/8876
2019-08-12T14:13:47.9121017Z .................................................................................................... 5900/8876
2019-08-12T14:13:52.7684280Z .................................................................................................... 6000/8876
2019-08-12T14:13:52.7684280Z .................................................................................................... 6000/8876
2019-08-12T14:14:07.4242616Z .....i..ii.......................................................................................... 6100/8876
2019-08-12T14:14:26.6803622Z ................................................i................................................... 6300/8876
2019-08-12T14:14:28.8854664Z .................................................................................................... 6400/8876
2019-08-12T14:14:31.4060363Z ....................i............................................................................... 6500/8876
2019-08-12T14:14:36.0640465Z .................................................................................................... 6600/8876
---
2019-08-12T14:19:25.8346361Z  finished in 23.298
2019-08-12T14:19:25.8539270Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-12T14:19:26.0162670Z 
2019-08-12T14:19:26.0163631Z running 146 tests
2019-08-12T14:19:29.3558268Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-08-12T14:19:31.2911312Z iii..............i.........iii.i......ii......
2019-08-12T14:19:31.2913484Z 
2019-08-12T14:19:31.2917937Z  finished in 5.438
2019-08-12T14:19:31.3101651Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-12T14:19:32.0998783Z 
---
2019-08-12T14:19:33.6061119Z  finished in 2.295
2019-08-12T14:19:33.6244681Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-12T14:19:33.7855225Z 
2019-08-12T14:19:33.7856486Z running 9 tests
2019-08-12T14:19:33.7858259Z iiiiiiiii
2019-08-12T14:19:33.7859408Z 
2019-08-12T14:19:33.7859593Z  finished in 0.161
2019-08-12T14:19:33.8051906Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-12T14:19:34.5968977Z 
---
2019-08-12T14:19:52.8186443Z  finished in 19.013
2019-08-12T14:19:52.8367806Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-12T14:19:52.9963852Z 
2019-08-12T14:19:52.9964105Z running 122 tests
2019-08-12T14:20:17.6455065Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-12T14:20:22.5924727Z .i.i......iii.i.....ii
2019-08-12T14:20:22.5926336Z 
2019-08-12T14:20:22.5926596Z  finished in 29.755
2019-08-12T14:20:22.5936909Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-12T14:20:22.5937480Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-12T14:21:13.0604357Z failures:
2019-08-12T14:21:13.0606859Z 
2019-08-12T14:21:13.0610699Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-08-12T14:21:13.0610946Z 
2019-08-12T14:21:13.0611426Z error: test compilation failed although it shouldn't!
2019-08-12T14:21:13.0611639Z status: exit code: 1
2019-08-12T14:21:13.0612881Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary" "-A" "unused"
2019-08-12T14:21:13.0613478Z ------------------------------------------
2019-08-12T14:21:13.0613624Z 
2019-08-12T14:21:13.0613945Z ------------------------------------------
2019-08-12T14:21:13.0614128Z stderr:
2019-08-12T14:21:13.0614128Z stderr:
2019-08-12T14:21:13.0614783Z ------------------------------------------
2019-08-12T14:21:13.0615145Z error[E0560]: struct `syntax::ast::FnDecl` has no field named `c_variadic`
2019-08-12T14:21:13.0615707Z    |
2019-08-12T14:21:13.0615707Z    |
2019-08-12T14:21:13.0616003Z LL |                     c_variadic: false,
2019-08-12T14:21:13.0616144Z    |                     ^^^^^^^^^^ `syntax::ast::FnDecl` does not have this field
2019-08-12T14:21:13.0616257Z    |
2019-08-12T14:21:13.0616384Z    = note: available fields are: `inputs`, `output`
2019-08-12T14:21:13.0616771Z error: aborting due to previous error
2019-08-12T14:21:13.0616864Z 
2019-08-12T14:21:13.0617191Z For more information about this error, try `rustc --explain E0560`.
2019-08-12T14:21:13.0617335Z 
---
2019-08-12T14:21:13.0624066Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-12T14:21:13.0624459Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-12T14:21:13.0633025Z 
2019-08-12T14:21:13.0633233Z 
2019-08-12T14:21:13.0635287Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-12T14:21:13.0635932Z 
2019-08-12T14:21:13.0636039Z 
2019-08-12T14:21:13.0643605Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-12T14:21:13.0643726Z Build completed unsuccessfully in 1:09:23
2019-08-12T14:21:13.0643726Z Build completed unsuccessfully in 1:09:23
2019-08-12T14:21:14.2607734Z ##[error]Bash exited with code '1'.
2019-08-12T14:21:14.2664168Z ##[section]Starting: Checkout
2019-08-12T14:21:14.2665734Z ==============================================================================
2019-08-12T14:21:14.2665779Z Task         : Get sources
2019-08-12T14:21:14.2665821Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
