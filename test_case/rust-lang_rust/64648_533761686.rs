plain
2019-09-21T01:38:54.4244651Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T01:38:55.1301136Z ##[command]git config gc.auto 0
2019-09-21T01:38:55.1303905Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T01:38:55.1305735Z ##[command]git config --get-all http.proxy
2019-09-21T01:38:55.1307984Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64648/merge:refs/remotes/pull/64648/merge
---
2019-09-21T02:39:59.2029619Z .................................................................................................... 1500/9029
2019-09-21T02:40:04.9502239Z .................................................................................................... 1600/9029
2019-09-21T02:40:17.5150404Z .....................................................................i...............i.............. 1700/9029
2019-09-21T02:40:24.3172143Z .................................................................................................... 1800/9029
2019-09-21T02:40:39.5759472Z ............................................................iiiii................................... 1900/9029
2019-09-21T02:40:51.6288339Z .................................................................................................... 2100/9029
2019-09-21T02:40:54.0875654Z .................................................................................................... 2200/9029
2019-09-21T02:40:57.2948286Z .................................................................................................... 2300/9029
2019-09-21T02:41:05.7053152Z .................................................................................................... 2400/9029
---
2019-09-21T02:44:02.1226768Z ................................................i...............i................................... 4700/9029
2019-09-21T02:44:11.5066028Z .................................................................................................... 4800/9029
2019-09-21T02:44:19.5958179Z .................................................................................................... 4900/9029
2019-09-21T02:44:29.0643651Z .................................................................................................... 5000/9029
2019-09-21T02:44:36.8818683Z ................................ii.ii............................................................... 5100/9029
2019-09-21T02:44:46.3814708Z .................................................................................................... 5300/9029
2019-09-21T02:44:56.9973710Z ................................................................................................i... 5400/9029
2019-09-21T02:45:05.2793905Z .................................................................................................... 5500/9029
2019-09-21T02:45:09.9570041Z .................................................................................................... 5600/9029
2019-09-21T02:45:09.9570041Z .................................................................................................... 5600/9029
2019-09-21T02:45:20.6059538Z ...........................................................................................ii...i..i 5700/9029
2019-09-21T02:45:34.9343938Z i...........i....................................................................................... 5800/9029
2019-09-21T02:45:55.7671129Z .................................................................................................... 6000/9029
2019-09-21T02:45:55.7671129Z .................................................................................................... 6000/9029
2019-09-21T02:46:02.7763219Z .............................................................................................i..ii.. 6100/9029
2019-09-21T02:46:30.6401446Z .................................................................................................... 6300/9029
2019-09-21T02:46:35.0977626Z ....................................................i............................................... 6400/9029
2019-09-21T02:46:37.2881648Z .................................................................................................... 6500/9029
2019-09-21T02:46:39.6351980Z ........................i........................................................................... 6600/9029
---
2019-09-21T02:50:41.5373554Z diff of stderr:
2019-09-21T02:50:41.5373670Z 
2019-09-21T02:50:41.5373795Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-21T02:50:41.5373937Z 9    |
2019-09-21T02:50:41.5374269Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-21T02:50:41.5374645Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-21T02:50:41.5374981Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-21T02:50:41.5375370Z 12 
2019-09-21T02:50:41.5375978Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-21T02:50:41.5376976Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-21T02:50:41.5377501Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-21T02:50:41.5377681Z 15    |
2019-09-21T02:50:41.5378098Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-21T02:50:41.5378375Z 
2019-09-21T02:50:41.5378540Z The actual stderr differed from the expected stderr.
2019-09-21T02:50:41.5378980Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-21T02:50:41.5378980Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-21T02:50:41.5379376Z To update references, rerun the tests and pass the `--bless` flag
2019-09-21T02:50:41.5379865Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-21T02:50:41.5380505Z error: 1 errors occurred comparing output.
2019-09-21T02:50:41.5380794Z status: exit code: 1
2019-09-21T02:50:41.5380794Z status: exit code: 1
2019-09-21T02:50:41.5381593Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-21T02:50:41.5382135Z ------------------------------------------
2019-09-21T02:50:41.5382262Z 
2019-09-21T02:50:41.5382554Z ------------------------------------------
2019-09-21T02:50:41.5382705Z stderr:
2019-09-21T02:50:41.5382705Z stderr:
2019-09-21T02:50:41.5382976Z ------------------------------------------
2019-09-21T02:50:41.5383134Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-21T02:50:41.5383432Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-09-21T02:50:41.5383573Z    |
2019-09-21T02:50:41.5383900Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-21T02:50:41.5384186Z    |                                                                |
2019-09-21T02:50:41.5384304Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-21T02:50:41.5384449Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-21T02:50:41.5384564Z    |
2019-09-21T02:50:41.5384564Z    |
2019-09-21T02:50:41.5384878Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-21T02:50:41.5385251Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-21T02:50:41.5385371Z 
2019-09-21T02:50:41.5385736Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-21T02:50:41.5386350Z    |
2019-09-21T02:50:41.5386350Z    |
2019-09-21T02:50:41.5387038Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-21T02:50:41.5387399Z 
2019-09-21T02:50:41.5387555Z error: aborting due to 2 previous errors
2019-09-21T02:50:41.5387679Z 
2019-09-21T02:50:41.5387822Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-21T02:50:41.5410631Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-21T02:50:41.5410871Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-21T02:50:41.5428356Z 
2019-09-21T02:50:41.5428552Z 
2019-09-21T02:50:41.5430800Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-21T02:50:41.5431350Z 
2019-09-21T02:50:41.5431380Z 
2019-09-21T02:50:41.5435968Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-21T02:50:41.5436066Z Build completed unsuccessfully in 1:04:54
2019-09-21T02:50:41.5436066Z Build completed unsuccessfully in 1:04:54
2019-09-21T02:50:41.5489070Z == clock drift check ==
2019-09-21T02:50:41.5513036Z   local time: Sat Sep 21 02:50:41 UTC 2019
2019-09-21T02:50:41.6364042Z   network time: Sat, 21 Sep 2019 02:50:41 GMT
2019-09-21T02:50:41.6364196Z == end clock drift check ==
2019-09-21T02:50:42.5748642Z ##[error]Bash exited with code '1'.
2019-09-21T02:50:42.5828004Z ##[section]Starting: Checkout
2019-09-21T02:50:42.5829865Z ==============================================================================
2019-09-21T02:50:42.5829922Z Task         : Get sources
2019-09-21T02:50:42.5830006Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
