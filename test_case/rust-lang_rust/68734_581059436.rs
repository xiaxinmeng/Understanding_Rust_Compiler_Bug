plain
2020-02-01T17:42:32.2561421Z ========================== Starting Command Output ===========================
2020-02-01T17:42:32.2564885Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9fa36c1f-a6ab-4c1c-b1e6-d18e8f4d6e43.sh
2020-02-01T17:42:32.2565144Z 
2020-02-01T17:42:32.2575388Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T17:42:32.2587373Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68734/merge to s
2020-02-01T17:42:32.2590384Z Task         : Get sources
2020-02-01T17:42:32.2590418Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T17:42:32.2590499Z Version      : 1.0.0
2020-02-01T17:42:32.2590531Z Author       : Microsoft
---
2020-02-01T17:42:33.3178028Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T17:42:33.3188880Z ##[command]git config gc.auto 0
2020-02-01T17:42:33.3191102Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T17:42:33.3193232Z ##[command]git config --get-all http.proxy
2020-02-01T17:42:33.3201420Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68734/merge:refs/remotes/pull/68734/merge
---
2020-02-01T18:40:30.1453362Z .................................................................................................... 1700/9561
2020-02-01T18:40:35.2617009Z .................................................................................................... 1800/9561
2020-02-01T18:40:48.2116936Z .........................i.......................................................................... 1900/9561
2020-02-01T18:40:55.3249740Z .................................................................................................... 2000/9561
2020-02-01T18:41:10.1823338Z ...............iiiii................................................................................ 2100/9561
2020-02-01T18:41:20.1880717Z .................................................................................................... 2300/9561
2020-02-01T18:41:22.6558052Z .................................................................................................... 2400/9561
2020-02-01T18:41:27.8450280Z .................................................................................................... 2500/9561
2020-02-01T18:41:49.4533916Z .................................................................................................... 2600/9561
---
2020-02-01T18:44:29.0990205Z .................................................................................................... 4800/9561
2020-02-01T18:44:34.5888206Z ..........................................................i...............i......................... 4900/9561
2020-02-01T18:44:42.6495407Z .................................................................................................... 5000/9561
2020-02-01T18:44:51.0492747Z .................................................................................................... 5100/9561
2020-02-01T18:44:56.0682177Z .i.................................................................................................. 5200/9561
2020-02-01T18:45:07.3206764Z ...........................................................................ii.ii........i...i....... 5300/9561
2020-02-01T18:45:16.0135357Z .............i...................................................................................... 5500/9561
2020-02-01T18:45:26.2151673Z .................................................................................................... 5600/9561
2020-02-01T18:45:32.8356966Z ..............................................................i..................................... 5700/9561
2020-02-01T18:45:40.2445735Z .................................................................................................... 5800/9561
2020-02-01T18:45:40.2445735Z .................................................................................................... 5800/9561
2020-02-01T18:45:48.2517752Z .................................................................................................... 5900/9561
2020-02-01T18:45:57.4389601Z .....................................................ii...i..ii...........i......................... 6000/9561
2020-02-01T18:46:19.6310295Z .................................................................................................... 6200/9561
2020-02-01T18:46:27.4215899Z .................................................................................................... 6300/9561
2020-02-01T18:46:27.4215899Z .................................................................................................... 6300/9561
2020-02-01T18:46:36.1598811Z .................................................................................i..ii.............. 6400/9561
2020-02-01T18:47:10.1876008Z .................................................................................................... 6600/9561
2020-02-01T18:47:15.8368112Z .........................................................i.......................................... 6700/9561
2020-02-01T18:47:18.0704215Z .................................................................................................... 6800/9561
2020-02-01T18:47:20.4473154Z ..........................................................i......................................... 6900/9561
---
2020-02-01T18:49:06.0561899Z .................................................................................................... 7600/9561
2020-02-01T18:49:11.6825144Z .................................................................................................... 7700/9561
2020-02-01T18:49:18.7399929Z .................................................................................................... 7800/9561
2020-02-01T18:49:30.0540919Z .................................................................................................... 7900/9561
2020-02-01T18:49:36.4295217Z ..............iiiiiii.i............................................................................. 8000/9561
2020-02-01T18:49:51.5142621Z .................................................................................................... 8200/9561
2020-02-01T18:50:02.2235548Z .................................................................................................... 8300/9561
2020-02-01T18:50:15.9894081Z .................................................................................................... 8400/9561
2020-02-01T18:50:23.1773884Z .................................................................................................... 8500/9561
---
2020-02-01T18:52:47.7014085Z  finished in 7.402
2020-02-01T18:52:47.7209024Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T18:52:47.8815223Z 
2020-02-01T18:52:47.8816595Z running 169 tests
2020-02-01T18:52:51.0016150Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-02-01T18:52:53.2446601Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-02-01T18:52:53.2447820Z 
2020-02-01T18:52:53.2450373Z  finished in 5.524
2020-02-01T18:52:53.2666535Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T18:52:53.4236992Z 
---
2020-02-01T18:52:56.3207957Z  finished in 2.219
2020-02-01T18:52:56.3208254Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T18:52:56.3208316Z 
2020-02-01T18:52:56.3208359Z running 9 tests
2020-02-01T18:52:56.3208647Z iiiiiiiii
2020-02-01T18:52:56.3208967Z 
2020-02-01T18:52:56.3209008Z  finished in 0.158
2020-02-01T18:52:56.3209288Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T18:52:56.3209649Z 
---
2020-02-01T18:53:16.6416954Z  finished in 20.958
2020-02-01T18:53:16.6625072Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T18:53:16.8413097Z 
2020-02-01T18:53:16.8413818Z running 116 tests
2020-02-01T18:53:31.5232939Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-01T18:53:33.6550467Z ....iiii.....ii.
2020-02-01T18:53:33.6552290Z 
2020-02-01T18:53:33.6553164Z  finished in 16.992
2020-02-01T18:53:33.6558195Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T18:53:33.6558558Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-01T19:00:56.7872838Z ..................................................................................i................. 300/338
2020-02-01T19:01:19.4197075Z ......................................
2020-02-01T19:01:19.4198076Z failures:
2020-02-01T19:01:19.4200641Z 
2020-02-01T19:01:19.4201368Z ---- [rustdoc] rustdoc/copy-local-img.rs stdout ----
2020-02-01T19:01:19.4201745Z 
2020-02-01T19:01:19.4202008Z error: htmldocck failed!
2020-02-01T19:01:19.4202252Z status: exit code: 1
2020-02-01T19:01:19.4202945Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/copy-local-img" "/checkout/src/test/rustdoc/copy-local-img.rs"
2020-02-01T19:01:19.4203903Z ------------------------------------------
2020-02-01T19:01:19.4204196Z 
2020-02-01T19:01:19.4204676Z ------------------------------------------
2020-02-01T19:01:19.4205003Z stderr:
2020-02-01T19:01:19.4205003Z stderr:
2020-02-01T19:01:19.4205483Z ------------------------------------------
2020-02-01T19:01:19.4205797Z 3: @has check failed
2020-02-01T19:01:19.4206303Z  File does not exist 'static/d3212ae9b701da87'
2020-02-01T19:01:19.4206616Z  // @has static/d3212ae9b701da87
2020-02-01T19:01:19.4207103Z Encountered 1 errors
2020-02-01T19:01:19.4207326Z 
2020-02-01T19:01:19.4207817Z ------------------------------------------
2020-02-01T19:01:19.4208099Z 
2020-02-01T19:01:19.4208099Z 
2020-02-01T19:01:19.4208346Z 
2020-02-01T19:01:19.4208557Z 
2020-02-01T19:01:19.4208788Z failures:
2020-02-01T19:01:19.4209309Z     [rustdoc] rustdoc/copy-local-img.rs
2020-02-01T19:01:19.4212270Z test result: FAILED. 334 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
2020-02-01T19:01:19.4212683Z 
2020-02-01T19:01:19.4215145Z 
2020-02-01T19:01:19.4215458Z 
2020-02-01T19:01:19.4215458Z 
2020-02-01T19:01:19.4217520Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-01T19:01:19.4220790Z 
2020-02-01T19:01:19.4221031Z 
2020-02-01T19:01:19.4291727Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-01T19:01:19.4293900Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-01T19:01:19.4293900Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-01T19:01:19.4297477Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-01T19:01:19.4298851Z Build completed unsuccessfully in 1:12:55
2020-02-01T19:01:19.4301584Z == clock drift check ==
2020-02-01T19:01:19.4322630Z   local time: Sat Feb  1 19:01:19 UTC 2020
2020-02-01T19:01:19.7271584Z   network time: Sat, 01 Feb 2020 19:01:19 GMT
2020-02-01T19:01:19.7277193Z == end clock drift check ==
2020-02-01T19:01:20.8724113Z 
2020-02-01T19:01:20.8829788Z ##[error]Bash exited with code '1'.
2020-02-01T19:01:20.8842280Z ##[section]Finishing: Run build
2020-02-01T19:01:20.8878811Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68734/merge to s
2020-02-01T19:01:20.8880637Z Task         : Get sources
2020-02-01T19:01:20.8880685Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T19:01:20.8880731Z Version      : 1.0.0
2020-02-01T19:01:20.8880779Z Author       : Microsoft
2020-02-01T19:01:20.8880779Z Author       : Microsoft
2020-02-01T19:01:20.8880825Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T19:01:20.8880874Z ==============================================================================
2020-02-01T19:01:21.3762913Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T19:01:21.3816539Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68734/merge to s
2020-02-01T19:01:21.3943150Z Cleaning up task key
2020-02-01T19:01:21.3944084Z Start cleaning up orphan processes.
2020-02-01T19:01:21.4067234Z Terminate orphan process: pid (5114) (python)
2020-02-01T19:01:21.4311127Z ##[section]Finishing: Finalize Job
