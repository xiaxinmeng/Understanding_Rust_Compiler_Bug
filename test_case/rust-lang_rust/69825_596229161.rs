plain
2020-03-08T16:04:43.7433960Z ========================== Starting Command Output ===========================
2020-03-08T16:04:43.7438984Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4c5722bb-a96e-42f5-b99b-c8b793bae3ae.sh
2020-03-08T16:04:43.7439484Z 
2020-03-08T16:04:43.7444414Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-08T16:04:43.7463399Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69825/merge to s
2020-03-08T16:04:43.7467009Z Task         : Get sources
2020-03-08T16:04:43.7467309Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T16:04:43.7467581Z Version      : 1.0.0
2020-03-08T16:04:43.7467767Z Author       : Microsoft
---
2020-03-08T16:04:44.9228406Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-08T16:04:44.9239932Z ##[command]git config gc.auto 0
2020-03-08T16:04:44.9245613Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-08T16:04:44.9254075Z ##[command]git config --get-all http.proxy
2020-03-08T16:04:44.9267235Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69825/merge:refs/remotes/pull/69825/merge
---
2020-03-08T17:09:56.4617553Z .....................................................F.............................................. 1700/9742
2020-03-08T17:10:01.1564430Z .................................................................................................... 1800/9742
2020-03-08T17:10:13.2932378Z ...........................................................i........................................ 1900/9742
2020-03-08T17:10:21.2714772Z .................................................................................................... 2000/9742
2020-03-08T17:10:35.8587196Z .................................................iiiii.............................................. 2100/9742
2020-03-08T17:10:46.4240267Z .................................................................................................... 2300/9742
2020-03-08T17:10:48.6705333Z .................................................................................................... 2400/9742
2020-03-08T17:10:52.1276050Z .................................................................................................... 2500/9742
2020-03-08T17:11:14.7251656Z .................................................................................................... 2600/9742
---
2020-03-08T17:13:58.9726276Z ........i...............i........................................................................... 5000/9742
2020-03-08T17:14:09.0482417Z .................................................................................................... 5100/9742
2020-03-08T17:14:14.1170139Z ...................................................i................................................ 5200/9742
2020-03-08T17:14:22.8206585Z .................................................................................................... 5300/9742
2020-03-08T17:14:30.2692528Z ................................ii.ii........i...i.................................................. 5400/9742
2020-03-08T17:14:38.6360963Z .................................................................................................... 5600/9742
2020-03-08T17:14:48.3721928Z .................................................................................................... 5700/9742
2020-03-08T17:14:55.5608884Z .......................i............................................................................ 5800/9742
2020-03-08T17:15:01.4035314Z .................................................................................................... 5900/9742
2020-03-08T17:15:01.4035314Z .................................................................................................... 5900/9742
2020-03-08T17:15:12.6171606Z .................................................................................................... 6000/9742
2020-03-08T17:15:22.9869223Z ................ii...i..ii...........i.............................................................. 6100/9742
2020-03-08T17:15:40.0289693Z .................................................................................................... 6300/9742
2020-03-08T17:15:47.0982222Z .................................................................................................... 6400/9742
2020-03-08T17:15:47.0982222Z .................................................................................................... 6400/9742
2020-03-08T17:16:12.8293387Z ...............................................i..ii................................................ 6500/9742
2020-03-08T17:16:34.7938135Z .................................................................................................... 6700/9742
2020-03-08T17:16:36.9556424Z .........................................i.......................................................... 6800/9742
2020-03-08T17:16:39.0764625Z .................................................................................................... 6900/9742
2020-03-08T17:16:41.3004243Z ........................................................................i........................... 7000/9742
---
2020-03-08T17:18:25.0099678Z .................................................................................................... 7700/9742
2020-03-08T17:18:29.7007459Z .................................................................................................... 7800/9742
2020-03-08T17:18:35.1972496Z .................................................................................................... 7900/9742
2020-03-08T17:18:43.6047016Z .......................i............................................................................ 8000/9742
2020-03-08T17:18:52.0697512Z ........................................................................iiiiiiiii.i................. 8100/9742
2020-03-08T17:19:08.5423714Z ...............i......i............................................................................. 8300/9742
2020-03-08T17:19:14.0126588Z .................................................................................................... 8400/9742
2020-03-08T17:19:27.4405188Z .................................................................................................... 8500/9742
2020-03-08T17:19:37.6364895Z .................................................................................................... 8600/9742
---
2020-03-08T17:21:38.5699659Z normalized stderr:
2020-03-08T17:21:38.5700020Z warning: field is never read: `a`
2020-03-08T17:21:38.5700649Z   --> $DIR/const_discriminant.rs:11:9
2020-03-08T17:21:38.5701017Z    |
2020-03-08T17:21:38.5701418Z LL |     C { a: u8, b: u8 },
2020-03-08T17:21:38.5702030Z    |
2020-03-08T17:21:38.5702337Z    = note: `#[warn(dead_code)]` on by default
2020-03-08T17:21:38.5702883Z 
2020-03-08T17:21:38.5703195Z warning: field is never read: `b`
2020-03-08T17:21:38.5703195Z warning: field is never read: `b`
2020-03-08T17:21:38.5703850Z   --> $DIR/const_discriminant.rs:11:16
2020-03-08T17:21:38.5704203Z    |
2020-03-08T17:21:38.5704507Z LL |     C { a: u8, b: u8 },
2020-03-08T17:21:38.5705061Z 
2020-03-08T17:21:38.5705389Z warning: variant is never constructed: `Never`
2020-03-08T17:21:38.5706148Z   --> $DIR/const_discriminant.rs:22:5
2020-03-08T17:21:38.5706629Z    |
2020-03-08T17:21:38.5706629Z    |
2020-03-08T17:21:38.5706926Z LL |     Never(Void),
2020-03-08T17:21:38.5707232Z    |     ^^^^^^^^^^^
2020-03-08T17:21:38.5707473Z 
2020-03-08T17:21:38.5707857Z 
2020-03-08T17:21:38.5708125Z 
2020-03-08T17:21:38.5708333Z 
2020-03-08T17:21:38.5708661Z The actual stderr differed from the expected stderr.
2020-03-08T17:21:38.5709804Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_discriminant/const_discriminant.stderr
2020-03-08T17:21:38.5711409Z To update references, rerun the tests and pass the `--bless` flag
2020-03-08T17:21:38.5712043Z To only update this specific test, also pass `--test-args consts/const_discriminant.rs`
2020-03-08T17:21:38.5712465Z error: 1 errors occurred comparing output.
2020-03-08T17:21:38.5712703Z status: exit code: 0
2020-03-08T17:21:38.5712703Z status: exit code: 0
2020-03-08T17:21:38.5714459Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_discriminant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_discriminant/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_discriminant/auxiliary"
2020-03-08T17:21:38.5716051Z ------------------------------------------
2020-03-08T17:21:38.5716349Z 
2020-03-08T17:21:38.5716744Z ------------------------------------------
2020-03-08T17:21:38.5716952Z stderr:
2020-03-08T17:21:38.5716952Z stderr:
2020-03-08T17:21:38.5717335Z ------------------------------------------
2020-03-08T17:21:38.5717603Z warning: field is never read: `a`
2020-03-08T17:21:38.5718139Z   --> /checkout/src/test/ui/consts/const_discriminant.rs:11:9
2020-03-08T17:21:38.5718388Z    |
2020-03-08T17:21:38.5718578Z LL |     C { a: u8, b: u8 },
2020-03-08T17:21:38.5718965Z    |
2020-03-08T17:21:38.5719176Z    = note: `#[warn(dead_code)]` on by default
2020-03-08T17:21:38.5719595Z 
2020-03-08T17:21:38.5719771Z warning: field is never read: `b`
2020-03-08T17:21:38.5719771Z warning: field is never read: `b`
2020-03-08T17:21:38.5720262Z   --> /checkout/src/test/ui/consts/const_discriminant.rs:11:16
2020-03-08T17:21:38.5721344Z    |
2020-03-08T17:21:38.5721544Z LL |     C { a: u8, b: u8 },
2020-03-08T17:21:38.5721913Z 
2020-03-08T17:21:38.5722143Z warning: variant is never constructed: `Never`
2020-03-08T17:21:38.5722802Z   --> /checkout/src/test/ui/consts/const_discriminant.rs:22:5
2020-03-08T17:21:38.5723049Z    |
---
2020-03-08T17:21:38.5733103Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-08T17:21:38.5733688Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-08T17:21:38.5738221Z 
2020-03-08T17:21:38.5738562Z 
2020-03-08T17:21:38.5742674Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-08T17:21:38.5745675Z 
2020-03-08T17:21:38.5745773Z 
2020-03-08T17:21:38.5750185Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-08T17:21:38.5750524Z Build completed unsuccessfully in 1:09:36
2020-03-08T17:21:38.5750524Z Build completed unsuccessfully in 1:09:36
2020-03-08T17:21:38.5809942Z == clock drift check ==
2020-03-08T17:21:38.5829214Z   local time: Sun Mar  8 17:21:38 UTC 2020
2020-03-08T17:21:38.8949591Z   network time: Sun, 08 Mar 2020 17:21:38 GMT
2020-03-08T17:21:38.8951262Z == end clock drift check ==
2020-03-08T17:21:39.4568017Z 
2020-03-08T17:21:39.4653181Z ##[error]Bash exited with code '1'.
2020-03-08T17:21:39.4669022Z ##[section]Finishing: Run build
2020-03-08T17:21:39.4732230Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69825/merge to s
2020-03-08T17:21:39.4738331Z Task         : Get sources
2020-03-08T17:21:39.4738735Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T17:21:39.4739092Z Version      : 1.0.0
2020-03-08T17:21:39.4739340Z Author       : Microsoft
2020-03-08T17:21:39.4739340Z Author       : Microsoft
2020-03-08T17:21:39.4739751Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-08T17:21:39.4740211Z ==============================================================================
2020-03-08T17:21:39.8099800Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-08T17:21:39.8149157Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69825/merge to s
2020-03-08T17:21:39.8240720Z Cleaning up task key
2020-03-08T17:21:39.8242420Z Start cleaning up orphan processes.
2020-03-08T17:21:39.8426019Z Terminate orphan process: pid (3744) (python)
2020-03-08T17:21:39.8601442Z ##[section]Finishing: Finalize Job
