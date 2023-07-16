plain
2019-08-01T00:04:18.9735769Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T00:04:18.9957847Z ##[command]git config gc.auto 0
2019-08-01T00:04:19.0039949Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T00:04:19.0103492Z ##[command]git config --get-all http.proxy
2019-08-01T00:04:19.0249955Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63175/merge:refs/remotes/pull/63175/merge
---
2019-08-01T00:04:55.0434035Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T00:04:55.0434868Z 
2019-08-01T00:04:55.0435567Z   git checkout -b <new-branch-name>
2019-08-01T00:04:55.0435723Z 
2019-08-01T00:04:55.0435859Z HEAD is now at 972f666ee Merge 94f3d582447a100ad6d263f783711ed5eb6fd20d into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-08-01T00:04:55.0594681Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T00:04:55.0597736Z ==============================================================================
2019-08-01T00:04:55.0597803Z Task         : Bash
2019-08-01T00:04:55.0597841Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T01:05:24.3546057Z .................................................................................................... 1400/8820
2019-08-01T01:05:30.3618493Z .................................................................................................... 1500/8820
2019-08-01T01:05:43.0373725Z ..................................................................i...............i................. 1600/8820
2019-08-01T01:05:50.6920553Z .................................................................................................... 1700/8820
2019-08-01T01:06:05.6569129Z ....................................................iiiii........................................... 1800/8820
2019-08-01T01:06:17.1320088Z .................................................................................................... 2000/8820
2019-08-01T01:06:19.7035573Z .................................................................................................... 2100/8820
2019-08-01T01:06:23.3414523Z .................................................................................................... 2200/8820
2019-08-01T01:06:30.0341411Z .................................................................................................... 2300/8820
---
2019-08-01T01:10:28.9691799Z .................................................................................................... 5300/8820
2019-08-01T01:10:36.8113532Z ...............i.................................................................................... 5400/8820
2019-08-01T01:10:42.4991372Z .................................................................................................... 5500/8820
2019-08-01T01:10:55.0037881Z .................................................................................................... 5600/8820
2019-08-01T01:11:08.6460303Z .........ii...i..ii...........i..................................................................... 5700/8820
2019-08-01T01:11:26.3030927Z .................................................................................................... 5900/8820
2019-08-01T01:11:31.2592605Z .................................................................................................... 6000/8820
2019-08-01T01:11:31.2592605Z .................................................................................................... 6000/8820
2019-08-01T01:11:45.2277197Z .........i..ii...................................................................................... 6100/8820
2019-08-01T01:12:04.5741858Z ....................................................i............................................... 6300/8820
2019-08-01T01:12:06.8047406Z .................................................................................................... 6400/8820
2019-08-01T01:12:09.2735322Z ......................i............................................................................. 6500/8820
2019-08-01T01:12:13.9382428Z .................................................................................................... 6600/8820
---
2019-08-01T01:16:12.9798247Z failures:
2019-08-01T01:16:12.9830862Z 
2019-08-01T01:16:12.9831365Z ---- [ui] ui/commandline-argfile.rs stdout ----
2019-08-01T01:16:12.9831428Z 
2019-08-01T01:16:12.9831809Z error: test compilation failed although it shouldn't!
2019-08-01T01:16:12.9831858Z status: exit code: 1
2019-08-01T01:16:12.9832808Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/commandline-argfile.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "cmdline_set" "@$DIR/commandline-argfile.args" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile/auxiliary" "-A" "unused"
2019-08-01T01:16:12.9833692Z ------------------------------------------
2019-08-01T01:16:12.9833728Z 
2019-08-01T01:16:12.9833941Z ------------------------------------------
2019-08-01T01:16:12.9833987Z stderr:
2019-08-01T01:16:12.9833987Z stderr:
2019-08-01T01:16:12.9834215Z ------------------------------------------
2019-08-01T01:16:12.9834480Z Warning: Failed to open $DIR/commandline-argfile.args: No such file or directory (os error 2)
2019-08-01T01:16:12.9835036Z error: multiple input filenames provided (first two filenames are `/checkout/src/test/ui/commandline-argfile.rs` and `@$DIR/commandline-argfile.args`)
2019-08-01T01:16:12.9835136Z 
2019-08-01T01:16:12.9835351Z ------------------------------------------
2019-08-01T01:16:12.9835382Z 
2019-08-01T01:16:12.9835424Z 
---
2019-08-01T01:16:12.9871295Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-08-01T01:16:12.9871404Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-01T01:16:12.9885121Z 
2019-08-01T01:16:12.9885246Z 
2019-08-01T01:16:12.9890592Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-01T01:16:12.9890895Z 
2019-08-01T01:16:12.9890941Z 
2019-08-01T01:16:12.9904256Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-01T01:16:12.9904604Z Build completed unsuccessfully in 1:05:02
2019-08-01T01:16:12.9904604Z Build completed unsuccessfully in 1:05:02
2019-08-01T01:16:13.7548722Z ##[error]Bash exited with code '1'.
2019-08-01T01:16:13.7588057Z ##[section]Starting: Checkout
2019-08-01T01:16:13.7589772Z ==============================================================================
2019-08-01T01:16:13.7589823Z Task         : Get sources
2019-08-01T01:16:13.7589869Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
