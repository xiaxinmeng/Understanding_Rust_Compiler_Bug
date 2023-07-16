plain
2019-10-02T08:08:53.7033839Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-02T08:08:53.7259802Z ##[command]git config gc.auto 0
2019-10-02T08:08:53.7355153Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-02T08:08:53.7429159Z ##[command]git config --get-all http.proxy
2019-10-02T08:08:53.7587581Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64603/merge:refs/remotes/pull/64603/merge
---
2019-10-02T09:14:23.3206947Z .................................................................................................... 1400/9086
2019-10-02T09:14:30.2462474Z .................................................................................................... 1500/9086
2019-10-02T09:14:37.2552180Z .................................................................................................... 1600/9086
2019-10-02T09:14:47.5378607Z .................................................................................................... 1700/9086
2019-10-02T09:14:56.4664080Z i...............i................................................................................... 1800/9086
2019-10-02T09:15:03.7512392Z ...........................................................................................iiiii.... 1900/9086
2019-10-02T09:15:27.4552606Z .................................................................................................... 2100/9086
2019-10-02T09:15:30.0225460Z .................................................................................................... 2200/9086
2019-10-02T09:15:32.7929710Z .................................................................................................... 2300/9086
2019-10-02T09:15:39.6701636Z .................................................................................................... 2400/9086
---
2019-10-02T09:18:46.5013079Z ..............................................................................i...............i..... 4700/9086
2019-10-02T09:18:55.0446872Z .................................................................................................... 4800/9086
2019-10-02T09:19:05.5691500Z .................................................................................................... 4900/9086
2019-10-02T09:19:11.9392917Z .................................................................................................... 5000/9086
2019-10-02T09:19:23.5093253Z .....................................................................ii.ii.......................... 5100/9086
2019-10-02T09:19:33.9196282Z .................................................................................................... 5300/9086
2019-10-02T09:19:43.9389363Z .................................................................................................... 5400/9086
2019-10-02T09:19:51.7508654Z ...................................i................................................................ 5500/9086
2019-10-02T09:19:58.4803338Z .................................................................................................... 5600/9086
2019-10-02T09:19:58.4803338Z .................................................................................................... 5600/9086
2019-10-02T09:20:10.9353194Z .................................................................................................... 5700/9086
2019-10-02T09:20:21.7459312Z ...............................ii...i..ii...........i............................................... 5800/9086
2019-10-02T09:20:44.7685000Z .................................................................................................... 6000/9086
2019-10-02T09:20:54.3332571Z .................................................................................................... 6100/9086
2019-10-02T09:20:54.3332571Z .................................................................................................... 6100/9086
2019-10-02T09:21:11.3428996Z ..................................i..ii............................................................. 6200/9086
2019-10-02T09:21:32.0909594Z ..............................................................................................i..... 6400/9086
2019-10-02T09:21:34.5254006Z .................................................................................................... 6500/9086
2019-10-02T09:21:36.9506412Z ..................................................................i................................. 6600/9086
2019-10-02T09:21:40.1182973Z .................................................................................................... 6700/9086
---
2019-10-02T09:25:57.0618190Z - error: aborting due to previous error
2019-10-02T09:25:57.0618905Z + error: lifetime parameter `'_` only used once
2019-10-02T09:25:57.0619797Z +   --> $DIR/one-use-in-inherent-impl-header.rs:17:10
2019-10-02T09:25:57.0619896Z +    |
2019-10-02T09:25:57.0620378Z + LL | impl Foo<'_> {
2019-10-02T09:25:57.0620455Z +    |          ^^ this lifetime is only used here
2019-10-02T09:25:57.0620572Z + error: aborting due to 2 previous errors
2019-10-02T09:25:57.0620618Z 16 
2019-10-02T09:25:57.0620678Z 17 
2019-10-02T09:25:57.0620710Z 
2019-10-02T09:25:57.0620710Z 
2019-10-02T09:25:57.0620739Z 
2019-10-02T09:25:57.0620788Z The actual stderr differed from the expected stderr.
2019-10-02T09:25:57.0621241Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-inherent-impl-header/one-use-in-inherent-impl-header.stderr
2019-10-02T09:25:57.0621530Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T09:25:57.0622098Z To only update this specific test, also pass `--test-args single-use-lifetime/one-use-in-inherent-impl-header.rs`
2019-10-02T09:25:57.0622216Z error: 1 errors occurred comparing output.
2019-10-02T09:25:57.0622268Z status: exit code: 1
2019-10-02T09:25:57.0622268Z status: exit code: 1
2019-10-02T09:25:57.0623268Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-impl-header.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-inherent-impl-header" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-inherent-impl-header/auxiliary" "-A" "unused"
2019-10-02T09:25:57.0623676Z ------------------------------------------
2019-10-02T09:25:57.0623717Z 
2019-10-02T09:25:57.0623979Z ------------------------------------------
2019-10-02T09:25:57.0624032Z stderr:
2019-10-02T09:25:57.0624032Z stderr:
2019-10-02T09:25:57.0624307Z ------------------------------------------
2019-10-02T09:25:57.0624580Z error: lifetime parameter `'f` only used once
2019-10-02T09:25:57.0624897Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-impl-header.rs:12:6
2019-10-02T09:25:57.0624973Z    |
2019-10-02T09:25:57.0625252Z LL | impl<'f> Foo<'f> { //~ ERROR `'f` only used once
2019-10-02T09:25:57.0625511Z    |      ^^      -- ...is used only here
2019-10-02T09:25:57.0625634Z    |      this lifetime...
2019-10-02T09:25:57.0625682Z    |
2019-10-02T09:25:57.0625746Z note: lint level defined here
2019-10-02T09:25:57.0626055Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-impl-header.rs:1:9
2019-10-02T09:25:57.0626055Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-impl-header.rs:1:9
2019-10-02T09:25:57.0626568Z    |
2019-10-02T09:25:57.0626622Z LL | #![deny(single_use_lifetimes)]
2019-10-02T09:25:57.0626700Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-02T09:25:57.0626735Z 
2019-10-02T09:25:57.0627078Z error: lifetime parameter `'_` only used once
2019-10-02T09:25:57.0627409Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-impl-header.rs:17:10
2019-10-02T09:25:57.0627481Z    |
2019-10-02T09:25:57.0627718Z LL | impl Foo<'_> {
2019-10-02T09:25:57.0627791Z    |          ^^ this lifetime is only used here
2019-10-02T09:25:57.0627878Z error: aborting due to 2 previous errors
2019-10-02T09:25:57.0627911Z 
2019-10-02T09:25:57.0627941Z 
2019-10-02T09:25:57.0628215Z ------------------------------------------
---
2019-10-02T09:25:57.0677948Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-02T09:25:57.0678034Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-02T09:25:57.0684750Z 
2019-10-02T09:25:57.0684840Z 
2019-10-02T09:25:57.0686285Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-02T09:25:57.0686651Z 
2019-10-02T09:25:57.0686678Z 
2019-10-02T09:25:57.0692917Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-02T09:25:57.0693008Z Build completed unsuccessfully in 1:09:27
2019-10-02T09:25:57.0693008Z Build completed unsuccessfully in 1:09:27
2019-10-02T09:25:57.0748518Z == clock drift check ==
2019-10-02T09:25:57.0764808Z   local time: Wed Oct  2 09:25:57 UTC 2019
2019-10-02T09:25:57.1631682Z   network time: Wed, 02 Oct 2019 09:25:57 GMT
2019-10-02T09:25:57.1639960Z == end clock drift check ==
2019-10-02T09:25:58.1717462Z ##[error]Bash exited with code '1'.
2019-10-02T09:25:58.1764596Z ##[section]Starting: Checkout
2019-10-02T09:25:58.1766330Z ==============================================================================
2019-10-02T09:25:58.1766395Z Task         : Get sources
2019-10-02T09:25:58.1766437Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
