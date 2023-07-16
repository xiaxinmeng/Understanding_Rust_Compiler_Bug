plain
2019-08-01T01:21:31.6088024Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T01:21:31.6289533Z ##[command]git config gc.auto 0
2019-08-01T01:21:31.6363465Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T01:21:31.6421099Z ##[command]git config --get-all http.proxy
2019-08-01T01:21:32.1828638Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63175/merge:refs/remotes/pull/63175/merge
---
2019-08-01T01:22:06.7915032Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T01:22:06.7915066Z 
2019-08-01T01:22:06.7915292Z   git checkout -b <new-branch-name>
2019-08-01T01:22:06.7915344Z 
2019-08-01T01:22:06.7915399Z HEAD is now at bcc209efb Merge c46e0a425d5c57e242c77ca8bfc43eaf0fe5843f into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-08-01T01:22:06.8092778Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T01:22:06.8095924Z ==============================================================================
2019-08-01T01:22:06.8096007Z Task         : Bash
2019-08-01T01:22:06.8096059Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T02:24:36.3649874Z .................................................................................................... 1400/8820
2019-08-01T02:24:42.6015099Z .................................................................................................... 1500/8820
2019-08-01T02:24:55.6139378Z ..................................................................i...............i................. 1600/8820
2019-08-01T02:25:03.3192117Z .................................................................................................... 1700/8820
2019-08-01T02:25:19.0101227Z ....................................................iiiii........................................... 1800/8820
2019-08-01T02:25:30.5298418Z .................................................................................................... 2000/8820
2019-08-01T02:25:33.1730548Z .................................................................................................... 2100/8820
2019-08-01T02:25:36.8757734Z .................................................................................................... 2200/8820
2019-08-01T02:25:43.5720215Z .................................................................................................... 2300/8820
---
2019-08-01T02:29:44.7322948Z .................................................................................................... 5300/8820
2019-08-01T02:29:52.5081301Z ...............i.................................................................................... 5400/8820
2019-08-01T02:29:58.2691244Z .................................................................................................... 5500/8820
2019-08-01T02:30:10.8739413Z .................................................................................................... 5600/8820
2019-08-01T02:30:24.5827135Z .........ii...i..ii...........i..................................................................... 5700/8820
2019-08-01T02:30:42.6148486Z .................................................................................................... 5900/8820
2019-08-01T02:30:47.6624464Z .................................................................................................... 6000/8820
2019-08-01T02:30:47.6624464Z .................................................................................................... 6000/8820
2019-08-01T02:31:01.9743591Z .........i..ii...................................................................................... 6100/8820
2019-08-01T02:31:21.6219448Z ....................................................i............................................... 6300/8820
2019-08-01T02:31:23.8200412Z .................................................................................................... 6400/8820
2019-08-01T02:31:26.3388541Z ......................i............................................................................. 6500/8820
2019-08-01T02:31:31.0030694Z .................................................................................................... 6600/8820
---
2019-08-01T02:35:32.7029176Z failures:
2019-08-01T02:35:32.7068847Z 
2019-08-01T02:35:32.7069352Z ---- [ui] ui/commandline-argfile.rs stdout ----
2019-08-01T02:35:32.7069421Z 
2019-08-01T02:35:32.7069669Z error: test compilation failed although it shouldn't!
2019-08-01T02:35:32.7069722Z status: exit code: 1
2019-08-01T02:35:32.7070532Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/commandline-argfile.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "cmdline_set" "@src/test/ui/commandline-argfile.args" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile/auxiliary" "-A" "unused"
2019-08-01T02:35:32.7071118Z ------------------------------------------
2019-08-01T02:35:32.7071154Z 
2019-08-01T02:35:32.7071369Z ------------------------------------------
2019-08-01T02:35:32.7071433Z stderr:
2019-08-01T02:35:32.7071433Z stderr:
2019-08-01T02:35:32.7071645Z ------------------------------------------
2019-08-01T02:35:32.7071937Z Warning: Failed to open src/test/ui/commandline-argfile.args: No such file or directory (os error 2) (cwd: /checkout/obj)
2019-08-01T02:35:32.7072431Z error: multiple input filenames provided (first two filenames are `/checkout/src/test/ui/commandline-argfile.rs` and `@src/test/ui/commandline-argfile.args`)
2019-08-01T02:35:32.7072519Z 
2019-08-01T02:35:32.7073316Z ------------------------------------------
2019-08-01T02:35:32.7073382Z 
2019-08-01T02:35:32.7073407Z 
---
2019-08-01T02:35:32.7123171Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-08-01T02:35:32.7123262Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-01T02:35:32.7139680Z 
2019-08-01T02:35:32.7140384Z 
2019-08-01T02:35:32.7148538Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-01T02:35:32.7149674Z 
2019-08-01T02:35:32.7149736Z 
2019-08-01T02:35:32.7156683Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-01T02:35:32.7157076Z Build completed unsuccessfully in 1:06:53
2019-08-01T02:35:32.7157076Z Build completed unsuccessfully in 1:06:53
2019-08-01T02:35:33.5089571Z ##[error]Bash exited with code '1'.
2019-08-01T02:35:33.5153681Z ##[section]Starting: Checkout
2019-08-01T02:35:33.5156115Z ==============================================================================
2019-08-01T02:35:33.5156203Z Task         : Get sources
2019-08-01T02:35:33.5156251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
