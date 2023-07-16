plain
2019-08-01T13:59:32.3554068Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T13:59:32.3734597Z ##[command]git config gc.auto 0
2019-08-01T13:59:32.3803892Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T13:59:32.3852640Z ##[command]git config --get-all http.proxy
2019-08-01T13:59:32.3994550Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61041/merge:refs/remotes/pull/61041/merge
---
2019-08-01T14:00:08.6181512Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T14:00:08.6181544Z 
2019-08-01T14:00:08.6181752Z   git checkout -b <new-branch-name>
2019-08-01T14:00:08.6181782Z 
2019-08-01T14:00:08.6181829Z HEAD is now at fc0affb95 Merge 754c14c858c57388644565cfe2b4e95df4168d81 into a17951c4f80eb5208030f91fdb4ae93919fa6b12
2019-08-01T14:00:08.6353059Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T14:00:08.6355740Z ==============================================================================
2019-08-01T14:00:08.6355796Z Task         : Bash
2019-08-01T14:00:08.6355860Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T15:00:20.4553263Z .................................................................................................... 1400/8820
2019-08-01T15:00:26.2535202Z .................................................................................................... 1500/8820
2019-08-01T15:00:38.6401331Z ..................................................................i...............i................. 1600/8820
2019-08-01T15:00:45.9971996Z .................................................................................................... 1700/8820
2019-08-01T15:01:00.5774167Z ....................................................iiiii........................................... 1800/8820
2019-08-01T15:01:11.7242265Z .................................................................................................... 2000/8820
2019-08-01T15:01:14.1383276Z .................................................................................................... 2100/8820
2019-08-01T15:01:17.6677039Z .................................................................................................... 2200/8820
2019-08-01T15:01:24.0008251Z .................................................................................................... 2300/8820
---
2019-08-01T15:05:16.7654890Z .................................................................................................... 5300/8820
2019-08-01T15:05:23.8835147Z ...............i.................................................................................... 5400/8820
2019-08-01T15:05:29.4279494Z .................................................................................................... 5500/8820
2019-08-01T15:05:41.6903447Z .................................................................................................... 5600/8820
2019-08-01T15:05:55.0083041Z .........ii...i..ii...........i..................................................................... 5700/8820
2019-08-01T15:06:11.2218598Z .................................................................................................... 5900/8820
2019-08-01T15:06:15.9905394Z .................................................................................................... 6000/8820
2019-08-01T15:06:15.9905394Z .................................................................................................... 6000/8820
2019-08-01T15:06:29.9611999Z .........i..ii...................................................................................... 6100/8820
2019-08-01T15:06:49.0469453Z ....................................................i............................................... 6300/8820
2019-08-01T15:06:51.1197507Z .................................................................................................... 6400/8820
2019-08-01T15:06:53.4508435Z ......................i............................................................................. 6500/8820
2019-08-01T15:06:57.9653162Z .................................................................................................... 6600/8820
---
2019-08-01T15:11:32.6357108Z  finished in 20.436
2019-08-01T15:11:32.6524506Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T15:11:32.8174622Z 
2019-08-01T15:11:32.8175519Z running 146 tests
2019-08-01T15:11:36.0339292Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-08-01T15:11:37.8410860Z iii..............i.........iii.i......ii......
2019-08-01T15:11:37.8414369Z 
2019-08-01T15:11:37.8418037Z  finished in 5.189
2019-08-01T15:11:37.8596701Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T15:11:38.0166884Z 
---
2019-08-01T15:11:40.0815229Z  finished in 2.222
2019-08-01T15:11:40.1011057Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T15:11:40.2617866Z 
2019-08-01T15:11:40.2618253Z running 9 tests
2019-08-01T15:11:40.2619270Z iiiiiiiii
2019-08-01T15:11:40.2619975Z 
2019-08-01T15:11:40.2620225Z  finished in 0.160
2019-08-01T15:11:40.2810059Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T15:11:40.4469040Z 
2019-08-01T15:11:40.4469040Z 
2019-08-01T15:11:40.4469288Z running 105 tests
2019-08-01T15:11:57.8735185Z .........................................................................F.......................... 100/105
2019-08-01T15:11:58.6314042Z .....
2019-08-01T15:11:58.6314236Z failures:
2019-08-01T15:11:58.6314270Z 
2019-08-01T15:11:58.6314594Z ---- [incremental] incremental/no_mangle2.rs stdout ----
2019-08-01T15:11:58.6314990Z thread '[incremental] incremental/no_mangle2.rs' panicked at '`check-pass` header is only supported in `cfail` incremental tests', src/tools/compiletest/src/header.rs:585:17
2019-08-01T15:11:58.6315121Z 
2019-08-01T15:11:58.6315148Z 
2019-08-01T15:11:58.6315201Z failures:
2019-08-01T15:11:58.6315248Z     [incremental] incremental/no_mangle2.rs
2019-08-01T15:11:58.6315248Z     [incremental] incremental/no_mangle2.rs
2019-08-01T15:11:58.6315294Z 
2019-08-01T15:11:58.6315570Z test result: FAILED. 104 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-08-01T15:11:58.6315606Z 
2019-08-01T15:11:58.6316011Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-08-01T15:11:58.6322293Z 
2019-08-01T15:11:58.6322358Z 
2019-08-01T15:11:58.6327961Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-01T15:11:58.6328440Z 
2019-08-01T15:11:58.6328469Z 
2019-08-01T15:11:58.6337450Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-01T15:11:58.6338329Z Build completed unsuccessfully in 1:05:34
2019-08-01T15:11:58.6338329Z Build completed unsuccessfully in 1:05:34
2019-08-01T15:12:02.4089305Z ##[error]Bash exited with code '1'.
2019-08-01T15:12:02.4148838Z ##[section]Starting: Checkout
2019-08-01T15:12:02.4150614Z ==============================================================================
2019-08-01T15:12:02.4150668Z Task         : Get sources
2019-08-01T15:12:02.4150715Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
