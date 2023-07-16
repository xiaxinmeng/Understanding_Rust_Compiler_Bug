plain
2019-09-10T17:47:47.1539907Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T17:47:47.9539758Z ##[command]git config gc.auto 0
2019-09-10T17:47:47.9550479Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T17:47:47.9555394Z ##[command]git config --get-all http.proxy
2019-09-10T17:47:47.9560836Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64360/merge:refs/remotes/pull/64360/merge
---
2019-09-10T18:54:39.1701546Z .................................................................................................... 1500/9007
2019-09-10T18:54:45.5612829Z .................................................................................................... 1600/9007
2019-09-10T18:54:59.1395175Z ........................................................i...............i........................... 1700/9007
2019-09-10T18:55:07.4794298Z .................................................................................................... 1800/9007
2019-09-10T18:55:23.3063626Z ...............................................iiiii................................................ 1900/9007
2019-09-10T18:55:35.1349692Z ....................................F............................................................... 2100/9007
2019-09-10T18:55:37.8584469Z .................................................................................................... 2200/9007
2019-09-10T18:55:41.9643281Z .................................................................................................... 2300/9007
2019-09-10T18:55:50.3291853Z .................................................................................................... 2400/9007
---
2019-09-10T18:59:01.0905679Z ...................................i..............i................................................. 4700/9007
2019-09-10T18:59:13.4950386Z .................................................................................................... 4800/9007
2019-09-10T18:59:20.9061898Z .................................................................................................... 4900/9007
2019-09-10T18:59:32.3924046Z .................................................................................................... 5000/9007
2019-09-10T18:59:38.9105046Z .................ii.ii.............................................................................. 5100/9007
2019-09-10T18:59:50.3754747Z .................................................................................................... 5300/9007
2019-09-10T19:00:01.2214441Z ................................................................................i................... 5400/9007
2019-09-10T19:00:09.5667004Z .................................................................................................... 5500/9007
2019-09-10T19:00:16.0167360Z .................................................................................................... 5600/9007
2019-09-10T19:00:16.0167360Z .................................................................................................... 5600/9007
2019-09-10T19:00:27.7405057Z ..........................................................................ii...i..ii...........i.... 5700/9007
2019-09-10T19:00:53.7630681Z .................................................................................................... 5900/9007
2019-09-10T19:01:03.9124823Z .................................................................................................... 6000/9007
2019-09-10T19:01:03.9124823Z .................................................................................................... 6000/9007
2019-09-10T19:01:09.5452768Z ............................................................................i..ii................... 6100/9007
2019-09-10T19:01:41.5870377Z .................................................................................................... 6300/9007
2019-09-10T19:01:43.8916646Z ...................................i................................................................ 6400/9007
2019-09-10T19:01:46.2143576Z .................................................................................................... 6500/9007
2019-09-10T19:01:49.0112772Z .......i............................................................................................ 6600/9007
---
2019-09-10T19:06:12.5469377Z 
2019-09-10T19:06:12.5472859Z ---- [ui] ui/error-codes/E0044.rs stdout ----
2019-09-10T19:06:12.5472951Z diff of stderr:
2019-09-10T19:06:12.5472983Z 
2019-09-10T19:06:12.5473215Z 4 LL |     fn sqrt<T>(f: T) -> T;
2019-09-10T19:06:12.5473530Z 6    |
2019-09-10T19:06:12.5473530Z 6    |
2019-09-10T19:06:12.5473813Z -    = help: use specialization instead of type parameters by replacing them with concrete types like `u32`
2019-09-10T19:06:12.5473905Z +    = help: use specialization instead of type parameters by replacing them with concrete typelike `u32`
2019-09-10T19:06:12.5474032Z 9 error: aborting due to previous error
2019-09-10T19:06:12.5474075Z 10 
2019-09-10T19:06:12.5474122Z 
2019-09-10T19:06:12.5474148Z 
2019-09-10T19:06:12.5474148Z 
2019-09-10T19:06:12.5474195Z The actual stderr differed from the expected stderr.
2019-09-10T19:06:12.5474492Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0044/E0044.stderr
2019-09-10T19:06:12.5474770Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T19:06:12.5475029Z To only update this specific test, also pass `--test-args error-codes/E0044.rs`
2019-09-10T19:06:12.5475126Z error: 1 errors occurred comparing output.
2019-09-10T19:06:12.5475172Z status: exit code: 1
2019-09-10T19:06:12.5475172Z status: exit code: 1
2019-09-10T19:06:12.5475908Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0044.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0044" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0044/auxiliary" "-A" "unused"
2019-09-10T19:06:12.5476254Z ------------------------------------------
2019-09-10T19:06:12.5476291Z 
2019-09-10T19:06:12.5476524Z ------------------------------------------
2019-09-10T19:06:12.5476572Z stderr:
2019-09-10T19:06:12.5476572Z stderr:
2019-09-10T19:06:12.5477082Z ------------------------------------------
2019-09-10T19:06:12.5477163Z error[E0044]: foreign items may not have type parameters
2019-09-10T19:06:12.5477401Z   --> /checkout/src/test/ui/error-codes/E0044.rs:2:5
2019-09-10T19:06:12.5477457Z    |
2019-09-10T19:06:12.5477710Z LL |     fn sqrt<T>(f: T) -> T;
2019-09-10T19:06:12.5478271Z    |
2019-09-10T19:06:12.5478271Z    |
2019-09-10T19:06:12.5478346Z    = help: use specialization instead of type parameters by replacing them with concrete typelike `u32`
2019-09-10T19:06:12.5478437Z error: aborting due to previous error
2019-09-10T19:06:12.5478468Z 
2019-09-10T19:06:12.5478809Z For more information about this error, try `rustc --explain E0044`.
2019-09-10T19:06:12.5478996Z 
---
2019-09-10T19:06:12.5479668Z 
2019-09-10T19:06:12.5479728Z 4 LL |     fn foo<T>();
2019-09-10T19:06:12.5479977Z 5    |     ^^^^^^^^^^^^ can't have type parameters
2019-09-10T19:06:12.5480028Z 6    |
2019-09-10T19:06:12.5480353Z -    = help: use specialization instead of type parameters by replacing them with concrete types like `u32`
2019-09-10T19:06:12.5480420Z +    = help: use specialization instead of type parameters by replacing them with concrete typelike `u32`
2019-09-10T19:06:12.5480516Z 9 error: aborting due to previous error
2019-09-10T19:06:12.5480577Z 10 
2019-09-10T19:06:12.5480606Z 
2019-09-10T19:06:12.5480634Z 
2019-09-10T19:06:12.5480634Z 
2019-09-10T19:06:12.5480682Z The actual stderr differed from the expected stderr.
2019-09-10T19:06:12.5481031Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-extern/generic-extern.stderr
2019-09-10T19:06:12.5481299Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T19:06:12.5481581Z To only update this specific test, also pass `--test-args generic/generic-extern.rs`
2019-09-10T19:06:12.5481684Z error: 1 errors occurred comparing output.
2019-09-10T19:06:12.5481731Z status: exit code: 1
2019-09-10T19:06:12.5481731Z status: exit code: 1
2019-09-10T19:06:12.5482501Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic/generic-extern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-extern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-extern/auxiliary" "-A" "unused"
2019-09-10T19:06:12.5482858Z ------------------------------------------
2019-09-10T19:06:12.5482897Z 
2019-09-10T19:06:12.5483136Z ------------------------------------------
2019-09-10T19:06:12.5483206Z stderr:
2019-09-10T19:06:12.5483206Z stderr:
2019-09-10T19:06:12.5483440Z ------------------------------------------
2019-09-10T19:06:12.5483497Z error[E0044]: foreign items may not have type parameters
2019-09-10T19:06:12.5483757Z   --> /checkout/src/test/ui/generic/generic-extern.rs:2:5
2019-09-10T19:06:12.5483829Z    |
2019-09-10T19:06:12.5483881Z LL |     fn foo<T>(); //~ ERROR foreign items may not have type parameters
2019-09-10T19:06:12.5484201Z    |
2019-09-10T19:06:12.5484201Z    |
2019-09-10T19:06:12.5484254Z    = help: use specialization instead of type parameters by replacing them with concrete typelike `u32`
2019-09-10T19:06:12.5484369Z error: aborting due to previous error
2019-09-10T19:06:12.5484400Z 
2019-09-10T19:06:12.5484662Z For more information about this error, try `rustc --explain E0044`.
2019-09-10T19:06:12.5484699Z 
---
2019-09-10T19:06:12.5500374Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-10T19:06:12.5500502Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-10T19:06:12.5514422Z 
2019-09-10T19:06:12.5514540Z 
2019-09-10T19:06:12.5516986Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-10T19:06:12.5517275Z 
2019-09-10T19:06:12.5517306Z 
2019-09-10T19:06:12.5523088Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-10T19:06:12.5523167Z Build completed unsuccessfully in 1:10:44
2019-09-10T19:06:12.5523167Z Build completed unsuccessfully in 1:10:44
2019-09-10T19:06:12.5575550Z == clock drift check ==
2019-09-10T19:06:12.5591375Z   local time: Tue Sep 10 19:06:12 UTC 2019
2019-09-10T19:06:12.6457982Z   network time: Tue, 10 Sep 2019 19:06:12 GMT
2019-09-10T19:06:12.6459443Z == end clock drift check ==
2019-09-10T19:06:13.4989379Z ##[error]Bash exited with code '1'.
2019-09-10T19:06:13.5028220Z ##[section]Starting: Checkout
2019-09-10T19:06:13.5030174Z ==============================================================================
2019-09-10T19:06:13.5030228Z Task         : Get sources
2019-09-10T19:06:13.5030292Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
