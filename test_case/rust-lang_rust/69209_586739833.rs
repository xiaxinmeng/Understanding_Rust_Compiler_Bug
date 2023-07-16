plain
2020-02-16T17:45:41.5334734Z ========================== Starting Command Output ===========================
2020-02-16T17:45:41.5337064Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a2f64065-3cd7-44de-b256-f9fc36f29bc3.sh
2020-02-16T17:45:41.5337099Z 
2020-02-16T17:45:41.5340877Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T17:45:41.5395419Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69209/merge to s
2020-02-16T17:45:41.5397225Z Task         : Get sources
2020-02-16T17:45:41.5397259Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T17:45:41.5397286Z Version      : 1.0.0
2020-02-16T17:45:41.5397313Z Author       : Microsoft
---
2020-02-16T17:45:42.3775202Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T17:45:42.3881772Z ##[command]git config gc.auto 0
2020-02-16T17:45:42.3969441Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T17:45:42.4027291Z ##[command]git config --get-all http.proxy
2020-02-16T17:45:42.4175207Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69209/merge:refs/remotes/pull/69209/merge
---
2020-02-16T18:42:02.6742589Z .................................................................................................... 1700/9650
2020-02-16T18:42:06.9280411Z .................................................................................................... 1800/9650
2020-02-16T18:42:18.1337179Z ..................................i................................................................. 1900/9650
2020-02-16T18:42:25.2778368Z .................................................................................................... 2000/9650
2020-02-16T18:42:38.7339365Z ........................iiiii....................................................................... 2100/9650
2020-02-16T18:42:47.2985188Z .................................................................................................... 2300/9650
2020-02-16T18:42:49.2768037Z .................................................................................................... 2400/9650
2020-02-16T18:42:53.1759410Z .................................................................................................... 2500/9650
2020-02-16T18:43:12.6178961Z .................................................................................................... 2600/9650
---
2020-02-16T18:46:30.9622956Z .................................................................................................... 5600/9650
2020-02-16T18:46:41.3109913Z .......................................................................................i............ 5700/9650
2020-02-16T18:46:49.2442722Z .................................................................................................... 5800/9650
2020-02-16T18:46:54.4860159Z .....................................................................................i.............. 5900/9650
2020-02-16T18:47:03.8359600Z ...............................................................................ii...i..ii........... 6000/9650
2020-02-16T18:47:15.7636936Z i................................................................................................... 6100/9650
2020-02-16T18:47:30.7808567Z .................................................................................................... 6300/9650
2020-02-16T18:47:37.7295746Z .................................................................................................... 6400/9650
2020-02-16T18:47:37.7295746Z .................................................................................................... 6400/9650
2020-02-16T18:47:52.2146114Z .......i..ii........................................................................................ 6500/9650
2020-02-16T18:48:09.9530626Z ...............................................................................................i.... 6700/9650
2020-02-16T18:48:11.8192869Z .................................................................................................... 6800/9650
2020-02-16T18:48:13.7782881Z .................................................................................................... 6900/9650
2020-02-16T18:48:15.9059258Z .....i.............................................................................................. 7000/9650
---
2020-02-16T18:49:46.1015522Z .................................................................................................... 7600/9650
2020-02-16T18:49:50.2786109Z .................................................................................................... 7700/9650
2020-02-16T18:49:55.6452303Z .................................................................................................... 7800/9650
2020-02-16T18:50:02.1435839Z .................................................................................................... 7900/9650
2020-02-16T18:50:11.6776975Z .......................................................................................iiiiiii.i.... 8000/9650
2020-02-16T18:50:27.4001256Z ...........................i......i................................................................. 8200/9650
2020-02-16T18:50:32.0888555Z .................................................................................................... 8300/9650
2020-02-16T18:50:42.9938984Z .................................................................................................... 8400/9650
2020-02-16T18:50:54.6845164Z .................................................................................................... 8500/9650
---
2020-02-16T18:52:48.2538714Z 
2020-02-16T18:52:48.2539247Z ---- [ui] ui/fmt/send-sync.rs stdout ----
2020-02-16T18:52:48.2539305Z diff of stderr:
2020-02-16T18:52:48.2539568Z 
2020-02-16T18:52:48.2539637Z 23 LL |     sync(format_args!("{:?}", c));
2020-02-16T18:52:48.2539699Z 24    |     ^^^^ `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T18:52:48.2539763Z 25    |
2020-02-16T18:52:48.2540077Z -    = help: within `[std::fmt::ArgumentV1<'_>]`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
2020-02-16T18:52:48.2540369Z +    = help: within `std::fmt::Arguments<'_>`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
2020-02-16T18:52:48.2540447Z 27    = note: required because it appears within the type `&core::fmt::Opaque`
2020-02-16T18:52:48.2540700Z 28    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T18:52:48.2541124Z 29    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
2020-02-16T18:52:48.2541208Z 
2020-02-16T18:52:48.2541252Z The actual stderr differed from the expected stderr.
2020-02-16T18:52:48.2541726Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync/send-sync.stderr
2020-02-16T18:52:48.2541726Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync/send-sync.stderr
2020-02-16T18:52:48.2542314Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T18:52:48.2542726Z To only update this specific test, also pass `--test-args fmt/send-sync.rs`
2020-02-16T18:52:48.2543227Z error: 1 errors occurred comparing output.
2020-02-16T18:52:48.2543269Z status: exit code: 1
2020-02-16T18:52:48.2543269Z status: exit code: 1
2020-02-16T18:52:48.2544333Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/send-sync.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync/auxiliary" "-A" "unused"
2020-02-16T18:52:48.2544652Z ------------------------------------------
2020-02-16T18:52:48.2544700Z 
2020-02-16T18:52:48.2544905Z ------------------------------------------
2020-02-16T18:52:48.2544947Z stderr:
2020-02-16T18:52:48.2544947Z stderr:
2020-02-16T18:52:48.2545161Z ------------------------------------------
2020-02-16T18:52:48.2545211Z error[E0277]: `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T18:52:48.2545430Z   --> /checkout/src/test/ui/fmt/send-sync.rs:8:5
2020-02-16T18:52:48.2545489Z    |
2020-02-16T18:52:48.2545530Z LL | fn send<T: Send>(_: T) {}
2020-02-16T18:52:48.2545939Z ...
2020-02-16T18:52:48.2545939Z ...
2020-02-16T18:52:48.2546239Z LL |     send(format_args!("{:?}", c)); //~ ERROR E0277
2020-02-16T18:52:48.2546284Z    |     ^^^^ `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T18:52:48.2546339Z    |
2020-02-16T18:52:48.2546619Z    = help: within `[std::fmt::ArgumentV1<'_>]`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
2020-02-16T18:52:48.2546680Z    = note: required because it appears within the type `&core::fmt::Opaque`
2020-02-16T18:52:48.2546929Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T18:52:48.2547160Z    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
2020-02-16T18:52:48.2547418Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&[std::fmt::ArgumentV1<'_>]`
2020-02-16T18:52:48.2547839Z    = note: required because it appears within the type `std::fmt::Arguments<'_>`
2020-02-16T18:52:48.2547914Z error[E0277]: `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T18:52:48.2548321Z   --> /checkout/src/test/ui/fmt/send-sync.rs:9:5
2020-02-16T18:52:48.2548364Z    |
2020-02-16T18:52:48.2548364Z    |
2020-02-16T18:52:48.2548404Z LL | fn sync<T: Sync>(_: T) {}
2020-02-16T18:52:48.2548690Z ...
2020-02-16T18:52:48.2548690Z ...
2020-02-16T18:52:48.2548734Z LL |     sync(format_args!("{:?}", c)); //~ ERROR E0277
2020-02-16T18:52:48.2548781Z    |     ^^^^ `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T18:52:48.2549113Z    = help: within `std::fmt::Arguments<'_>`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
2020-02-16T18:52:48.2549169Z    = note: required because it appears within the type `&core::fmt::Opaque`
2020-02-16T18:52:48.2549427Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T18:52:48.2549427Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T18:52:48.2549671Z    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
2020-02-16T18:52:48.2549915Z    = note: required because it appears within the type `&[std::fmt::ArgumentV1<'_>]`
2020-02-16T18:52:48.2550172Z    = note: required because it appears within the type `std::fmt::Arguments<'_>`
2020-02-16T18:52:48.2550328Z error: aborting due to 2 previous errors
2020-02-16T18:52:48.2550410Z 
2020-02-16T18:52:48.2550678Z For more information about this error, try `rustc --explain E0277`.
2020-02-16T18:52:48.2550710Z 
---
2020-02-16T18:52:48.2598765Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T18:52:48.2598853Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T18:52:48.2606862Z 
2020-02-16T18:52:48.2606948Z 
2020-02-16T18:52:48.2608735Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T18:52:48.2608985Z 
2020-02-16T18:52:48.2609013Z 
2020-02-16T18:52:48.2615146Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T18:52:48.2615404Z Build completed unsuccessfully in 1:01:11
2020-02-16T18:52:48.2615404Z Build completed unsuccessfully in 1:01:11
2020-02-16T18:52:48.2667173Z == clock drift check ==
2020-02-16T18:52:48.2684510Z   local time: Sun Feb 16 18:52:48 UTC 2020
2020-02-16T18:52:48.5526005Z   network time: Sun, 16 Feb 2020 18:52:48 GMT
2020-02-16T18:52:48.5530000Z == end clock drift check ==
2020-02-16T18:52:48.9053969Z 
2020-02-16T18:52:48.9181409Z ##[error]Bash exited with code '1'.
2020-02-16T18:52:48.9194410Z ##[section]Finishing: Run build
2020-02-16T18:52:48.9233732Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69209/merge to s
2020-02-16T18:52:48.9235883Z Task         : Get sources
2020-02-16T18:52:48.9235931Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T18:52:48.9235997Z Version      : 1.0.0
2020-02-16T18:52:48.9236039Z Author       : Microsoft
2020-02-16T18:52:48.9236039Z Author       : Microsoft
2020-02-16T18:52:48.9236085Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T18:52:48.9236154Z ==============================================================================
2020-02-16T18:52:49.2906317Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T18:52:49.2982959Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69209/merge to s
2020-02-16T18:52:49.3099025Z Cleaning up task key
2020-02-16T18:52:49.3099832Z Start cleaning up orphan processes.
2020-02-16T18:52:49.3224681Z Terminate orphan process: pid (4466) (python)
2020-02-16T18:52:49.3501282Z ##[section]Finishing: Finalize Job
