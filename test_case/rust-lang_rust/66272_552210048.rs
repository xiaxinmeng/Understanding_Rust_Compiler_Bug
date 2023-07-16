plain
2019-11-10T15:16:23.6866700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-10T15:16:23.7005551Z ##[command]git config gc.auto 0
2019-11-10T15:16:23.7078955Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-10T15:16:23.7133385Z ##[command]git config --get-all http.proxy
2019-11-10T15:16:23.7271338Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66272/merge:refs/remotes/pull/66272/merge
---
2019-11-10T16:13:28.1333928Z .................................................................................................... 1500/9220
2019-11-10T16:13:34.5726699Z .................................................................................................... 1600/9220
2019-11-10T16:13:44.3965797Z .................................................................................................i.. 1700/9220
2019-11-10T16:13:52.9977139Z .................................................................................................... 1800/9220
2019-11-10T16:13:59.8755593Z .................................................................................iiiii.............. 1900/9220
2019-11-10T16:14:20.9295979Z .................................................................................................... 2100/9220
2019-11-10T16:14:23.3255776Z .................................................................................................... 2200/9220
2019-11-10T16:14:25.9229507Z .................................................................................................... 2300/9220
2019-11-10T16:14:39.4567175Z .................................................................................................... 2400/9220
---
2019-11-10T16:17:27.1336166Z ...........................................................................i...............i........ 4700/9220
2019-11-10T16:17:34.8479710Z .................................................................................................... 4800/9220
2019-11-10T16:17:43.5785516Z .................................................................................................... 4900/9220
2019-11-10T16:17:48.7919163Z .................................................................................................... 5000/9220
2019-11-10T16:17:59.9415272Z .............................................................................ii.ii...........i...... 5100/9220
2019-11-10T16:18:08.5916219Z ............i....................................................................................... 5300/9220
2019-11-10T16:18:18.8148988Z .................................................................................................... 5400/9220
2019-11-10T16:18:25.3970796Z ...........................................................i........................................ 5500/9220
2019-11-10T16:18:32.4790631Z .................................................................................................... 5600/9220
2019-11-10T16:18:32.4790631Z .................................................................................................... 5600/9220
2019-11-10T16:18:42.2936979Z .................................................................................................... 5700/9220
2019-11-10T16:18:49.7538129Z ............................................ii...i..ii...........i.................................. 5800/9220
2019-11-10T16:19:12.3137500Z .................................................................................................... 6000/9220
2019-11-10T16:19:20.6074281Z .................................................................................................... 6100/9220
2019-11-10T16:19:20.6074281Z .................................................................................................... 6100/9220
2019-11-10T16:19:27.0243390Z ...............................................................i..ii................................ 6200/9220
2019-11-10T16:19:55.3373685Z .................................................................................................... 6400/9220
2019-11-10T16:19:57.3678127Z ...............................i.................................................................... 6500/9220
2019-11-10T16:19:59.6466665Z .................................................................................................... 6600/9220
2019-11-10T16:20:02.0438704Z ...............i.................................................................................... 6700/9220
---
2019-11-10T16:24:44.4063584Z normalized stderr:
2019-11-10T16:24:44.4063623Z warning: unused variable: `arg`
2019-11-10T16:24:44.4063817Z   --> $DIR/typeid-intrinsic.rs:106:23
2019-11-10T16:24:44.4063876Z    |
2019-11-10T16:24:44.4064063Z LL |     fn non_static<'a>(arg: &'a str) {
2019-11-10T16:24:44.4064111Z    |                       ^^^ help: consider prefixing with an underscore: `_arg`
2019-11-10T16:24:44.4064209Z    = note: `#[warn(unused_variables)]` on by default
2019-11-10T16:24:44.4064236Z 
2019-11-10T16:24:44.4064273Z warning: unnecessary `unsafe` block
2019-11-10T16:24:44.4064481Z   --> $DIR/typeid-intrinsic.rs:107:20
2019-11-10T16:24:44.4064481Z   --> $DIR/typeid-intrinsic.rs:107:20
2019-11-10T16:24:44.4064521Z    |
2019-11-10T16:24:44.4064732Z LL |         assert_eq!(unsafe { type_id::<ContainsRef<'a>>() },
2019-11-10T16:24:44.4064836Z    |
2019-11-10T16:24:44.4065131Z    = note: `#[warn(unused_unsafe)]` on by default
2019-11-10T16:24:44.4065158Z 
2019-11-10T16:24:44.4065212Z warning: unnecessary `unsafe` block
2019-11-10T16:24:44.4065212Z warning: unnecessary `unsafe` block
2019-11-10T16:24:44.4065436Z   --> $DIR/typeid-intrinsic.rs:108:20
2019-11-10T16:24:44.4065475Z    |
2019-11-10T16:24:44.4065708Z LL |                    unsafe { type_id::<ContainsRef<'static>>() });
2019-11-10T16:24:44.4065781Z 
2019-11-10T16:24:44.4065817Z warning: field is never used: `field`
2019-11-10T16:24:44.4066019Z   --> $DIR/typeid-intrinsic.rs:19:5
2019-11-10T16:24:44.4066058Z    |
---
2019-11-10T16:24:44.4066549Z 
2019-11-10T16:24:44.4066571Z 
2019-11-10T16:24:44.4066593Z 
2019-11-10T16:24:44.4066632Z The actual stderr differed from the expected stderr.
2019-11-10T16:24:44.4066943Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeid-intrinsic/typeid-intrinsic.stderr
2019-11-10T16:24:44.4068006Z To update references, rerun the tests and pass the `--bless` flag
2019-11-10T16:24:44.4068302Z To only update this specific test, also pass `--test-args typeid-intrinsic.rs`
2019-11-10T16:24:44.4068406Z error: 1 errors occurred comparing output.
2019-11-10T16:24:44.4068452Z status: exit code: 0
2019-11-10T16:24:44.4068452Z status: exit code: 0
2019-11-10T16:24:44.4069509Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeid-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeid-intrinsic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeid-intrinsic/auxiliary"
2019-11-10T16:24:44.4069885Z ------------------------------------------
2019-11-10T16:24:44.4069939Z 
2019-11-10T16:24:44.4070164Z ------------------------------------------
2019-11-10T16:24:44.4070210Z stderr:
2019-11-10T16:24:44.4070210Z stderr:
2019-11-10T16:24:44.4070427Z ------------------------------------------
2019-11-10T16:24:44.4070495Z warning: unused variable: `arg`
2019-11-10T16:24:44.4070735Z   --> /checkout/src/test/ui/typeid-intrinsic.rs:106:23
2019-11-10T16:24:44.4070785Z    |
2019-11-10T16:24:44.4071761Z LL |     fn non_static<'a>(arg: &'a str) {
2019-11-10T16:24:44.4071818Z    |                       ^^^ help: consider prefixing with an underscore: `_arg`
2019-11-10T16:24:44.4071947Z    = note: `#[warn(unused_variables)]` on by default
2019-11-10T16:24:44.4071979Z 
2019-11-10T16:24:44.4072022Z warning: unnecessary `unsafe` block
2019-11-10T16:24:44.4072824Z   --> /checkout/src/test/ui/typeid-intrinsic.rs:107:20
2019-11-10T16:24:44.4072824Z   --> /checkout/src/test/ui/typeid-intrinsic.rs:107:20
2019-11-10T16:24:44.4072910Z    |
2019-11-10T16:24:44.4073156Z LL |         assert_eq!(unsafe { type_id::<ContainsRef<'a>>() },
2019-11-10T16:24:44.4073273Z    |
2019-11-10T16:24:44.4073319Z    = note: `#[warn(unused_unsafe)]` on by default
2019-11-10T16:24:44.4073351Z 
2019-11-10T16:24:44.4073413Z warning: unnecessary `unsafe` block
2019-11-10T16:24:44.4073413Z warning: unnecessary `unsafe` block
2019-11-10T16:24:44.4073653Z   --> /checkout/src/test/ui/typeid-intrinsic.rs:108:20
2019-11-10T16:24:44.4073700Z    |
2019-11-10T16:24:44.4073945Z LL |                    unsafe { type_id::<ContainsRef<'static>>() });
2019-11-10T16:24:44.4074051Z 
2019-11-10T16:24:44.4074101Z warning: field is never used: `field`
2019-11-10T16:24:44.4074356Z   --> /checkout/src/test/ui/typeid-intrinsic.rs:19:5
2019-11-10T16:24:44.4074403Z    |
---
2019-11-10T16:24:44.4076203Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-10T16:24:44.4076349Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-10T16:24:44.4076390Z 
2019-11-10T16:24:44.4076434Z 
2019-11-10T16:24:44.4078675Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-10T16:24:44.4078951Z 
2019-11-10T16:24:44.4078983Z 
2019-11-10T16:24:44.4079030Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-10T16:24:44.4079081Z Build completed unsuccessfully in 1:02:04
2019-11-10T16:24:44.4079081Z Build completed unsuccessfully in 1:02:04
2019-11-10T16:24:44.4079147Z == clock drift check ==
2019-11-10T16:24:44.4079194Z   local time: Sun Nov 10 16:24:44 UTC 2019
2019-11-10T16:24:44.4079241Z   network time: Sun, 10 Nov 2019 16:24:44 GMT
2019-11-10T16:24:44.4079305Z == end clock drift check ==
2019-11-10T16:24:45.0897450Z 
2019-11-10T16:24:45.0995974Z ##[error]Bash exited with code '1'.
2019-11-10T16:24:45.1031620Z ##[section]Starting: Checkout
2019-11-10T16:24:45.1033314Z ==============================================================================
2019-11-10T16:24:45.1033377Z Task         : Get sources
2019-11-10T16:24:45.1033442Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
