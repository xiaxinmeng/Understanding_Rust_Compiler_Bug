plain
2019-09-10T21:55:00.8177242Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T21:55:00.8356945Z ##[command]git config gc.auto 0
2019-09-10T21:55:00.8436220Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T21:55:00.8483804Z ##[command]git config --get-all http.proxy
2019-09-10T21:55:00.8621741Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64360/merge:refs/remotes/pull/64360/merge
---
2019-09-10T22:58:10.3023963Z .................................................................................................... 1500/9009
2019-09-10T22:58:16.4525395Z .................................................................................................... 1600/9009
2019-09-10T22:58:29.3711023Z ..........................................................i...............i......................... 1700/9009
2019-09-10T22:58:37.3959944Z .................................................................................................... 1800/9009
2019-09-10T22:58:52.3613315Z .................................................iiiii.............................................. 1900/9009
2019-09-10T22:59:03.9349382Z .......................................F............................................................ 2100/9009
2019-09-10T22:59:06.6550979Z .................................................................................................... 2200/9009
2019-09-10T22:59:10.3976993Z .................................................................................................... 2300/9009
2019-09-10T22:59:18.7994653Z .................................................................................................... 2400/9009
---
2019-09-10T23:02:22.9364567Z ....................................i...............i............................................... 4700/9009
2019-09-10T23:02:34.5522528Z .................................................................................................... 4800/9009
2019-09-10T23:02:41.3692154Z .................................................................................................... 4900/9009
2019-09-10T23:02:52.4602098Z .................................................................................................... 5000/9009
2019-09-10T23:02:58.9370770Z ...................ii.ii............................................................................ 5100/9009
2019-09-10T23:03:09.9364336Z .................................................................................................... 5300/9009
2019-09-10T23:03:20.3584987Z ..................................................................................i................. 5400/9009
2019-09-10T23:03:28.5950236Z .................................................................................................... 5500/9009
2019-09-10T23:03:34.4838620Z .................................................................................................... 5600/9009
2019-09-10T23:03:34.4838620Z .................................................................................................... 5600/9009
2019-09-10T23:03:45.2544923Z ............................................................................ii...i..ii...........i.. 5700/9009
2019-09-10T23:04:11.5973477Z .................................................................................................... 5900/9009
2019-09-10T23:04:21.7255395Z .................................................................................................... 6000/9009
2019-09-10T23:04:21.7255395Z .................................................................................................... 6000/9009
2019-09-10T23:04:31.1086365Z ..............................................................................i..ii................. 6100/9009
2019-09-10T23:05:01.5348056Z .................................................................................................... 6300/9009
2019-09-10T23:05:03.9025091Z .....................................i.............................................................. 6400/9009
2019-09-10T23:05:06.2218242Z .................................................................................................... 6500/9009
2019-09-10T23:05:08.9799050Z .........i.......................................................................................... 6600/9009
---
2019-09-10T23:09:20.1889108Z 
2019-09-10T23:09:20.1890050Z ---- [ui] ui/const-generics/foreign-item-const-parameter.rs stdout ----
2019-09-10T23:09:20.1890393Z diff of stderr:
2019-09-10T23:09:20.1890644Z 
2019-09-10T23:09:20.1890872Z 12 LL |     fn foo<const X: usize>();
2019-09-10T23:09:20.1892726Z 14    |
2019-09-10T23:09:20.1893339Z -    = help: replace the const parameters with concrete consts
2019-09-10T23:09:20.1893673Z +    = help: replace the const parameters with concreteconsts
2019-09-10T23:09:20.1893934Z 16 
2019-09-10T23:09:20.1893934Z 16 
2019-09-10T23:09:20.1894158Z 17 error[E0044]: foreign items may not have type or const parameters
2019-09-10T23:09:20.1894646Z 18   --> $DIR/foreign-item-const-parameter.rs:7:5
2019-09-10T23:09:20.1894947Z 
2019-09-10T23:09:20.1895410Z 20 LL |     fn bar<T, const X: usize>(_: T);
2019-09-10T23:09:20.1896399Z 22    |
2019-09-10T23:09:20.1896912Z -    = help: replace the type or const parameters with concrete types or consts
2019-09-10T23:09:20.1897220Z +    = help: replace the type or const parameters with concretetypes or consts
2019-09-10T23:09:20.1897475Z 24 
2019-09-10T23:09:20.1897475Z 24 
2019-09-10T23:09:20.1898077Z 25 error: aborting due to 2 previous errors
2019-09-10T23:09:20.1898440Z 26 
2019-09-10T23:09:20.1898634Z 
2019-09-10T23:09:20.1898819Z 
2019-09-10T23:09:20.1899143Z The actual stderr differed from the expected stderr.
2019-09-10T23:09:20.1899768Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/foreign-item-const-parameter/foreign-item-const-parameter.stderr
2019-09-10T23:09:20.1900714Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T23:09:20.1901356Z To only update this specific test, also pass `--test-args const-generics/foreign-item-const-parameter.rs`
2019-09-10T23:09:20.1902267Z error: 1 errors occurred comparing output.
2019-09-10T23:09:20.1902516Z status: exit code: 1
2019-09-10T23:09:20.1902516Z status: exit code: 1
2019-09-10T23:09:20.1903612Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/foreign-item-const-parameter.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/foreign-item-const-parameter" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/foreign-item-const-parameter/auxiliary" "-A" "unused"
2019-09-10T23:09:20.1905783Z ------------------------------------------
2019-09-10T23:09:20.1905990Z 
2019-09-10T23:09:20.1906522Z ------------------------------------------
2019-09-10T23:09:20.1906712Z stderr:
---
2019-09-10T23:09:20.1908765Z 
2019-09-10T23:09:20.1908904Z error[E0044]: foreign items may not have const parameters
2019-09-10T23:09:20.1909524Z   --> /checkout/src/test/ui/const-generics/foreign-item-const-parameter.rs:5:5
2019-09-10T23:09:20.1909833Z    |
2019-09-10T23:09:20.1910210Z LL |     fn foo<const X: usize>(); //~ ERROR foreign items may not have const parameters
2019-09-10T23:09:20.1910826Z    |
2019-09-10T23:09:20.1910972Z    = help: replace the const parameters with concreteconsts
2019-09-10T23:09:20.1912019Z 
2019-09-10T23:09:20.1912202Z error[E0044]: foreign items may not have type or const parameters
2019-09-10T23:09:20.1912202Z error[E0044]: foreign items may not have type or const parameters
2019-09-10T23:09:20.1912703Z   --> /checkout/src/test/ui/const-generics/foreign-item-const-parameter.rs:7:5
2019-09-10T23:09:20.1912940Z    |
2019-09-10T23:09:20.1913089Z LL |     fn bar<T, const X: usize>(_: T); //~ ERROR foreign items may not have type or const parameters
2019-09-10T23:09:20.1913725Z    |
2019-09-10T23:09:20.1913872Z    = help: replace the type or const parameters with concretetypes or consts
2019-09-10T23:09:20.1913999Z 
2019-09-10T23:09:20.1914153Z error: aborting due to 2 previous errors
---
2019-09-10T23:09:20.1915821Z 
2019-09-10T23:09:20.1916165Z ---- [ui] ui/error-codes/E0044.rs stdout ----
2019-09-10T23:09:20.1916338Z diff of stderr:
2019-09-10T23:09:20.1916443Z 
2019-09-10T23:09:20.1916775Z 4 LL |     fn sqrt<T>(f: T) -> T;
2019-09-10T23:09:20.1917514Z 6    |
2019-09-10T23:09:20.1917874Z -    = help: replace the type parameters with concrete types like `u32`
2019-09-10T23:09:20.1918054Z +    = help: replace the type parameters with concretetypes like `u32`
2019-09-10T23:09:20.1918177Z 8 
2019-09-10T23:09:20.1918177Z 8 
2019-09-10T23:09:20.1918366Z 9 error: aborting due to previous error
2019-09-10T23:09:20.1918502Z 10 
2019-09-10T23:09:20.1918603Z 
2019-09-10T23:09:20.1918705Z 
2019-09-10T23:09:20.1918858Z The actual stderr differed from the expected stderr.
2019-09-10T23:09:20.1919268Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0044/E0044.stderr
2019-09-10T23:09:20.1919668Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T23:09:20.1920098Z To only update this specific test, also pass `--test-args error-codes/E0044.rs`
2019-09-10T23:09:20.1920412Z error: 1 errors occurred comparing output.
2019-09-10T23:09:20.1920561Z status: exit code: 1
2019-09-10T23:09:20.1920561Z status: exit code: 1
2019-09-10T23:09:20.1922224Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0044.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0044" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0044/auxiliary" "-A" "unused"
2019-09-10T23:09:20.1923008Z ------------------------------------------
2019-09-10T23:09:20.1923211Z 
2019-09-10T23:09:20.1923780Z ------------------------------------------
2019-09-10T23:09:20.1924430Z stderr:
2019-09-10T23:09:20.1924430Z stderr:
2019-09-10T23:09:20.1924941Z ------------------------------------------
2019-09-10T23:09:20.1925145Z error[E0044]: foreign items may not have type parameters
2019-09-10T23:09:20.1925667Z   --> /checkout/src/test/ui/error-codes/E0044.rs:2:5
2019-09-10T23:09:20.1925843Z    |
2019-09-10T23:09:20.1926292Z LL |     fn sqrt<T>(f: T) -> T;
2019-09-10T23:09:20.1929324Z    |
2019-09-10T23:09:20.1929634Z    = help: replace the type parameters with concretetypes like `u32`
2019-09-10T23:09:20.1929776Z 
2019-09-10T23:09:20.1930013Z error: aborting due to previous error
---
2019-09-10T23:09:20.1934726Z 10 
2019-09-10T23:09:20.1934849Z 
2019-09-10T23:09:20.1934967Z 
2019-09-10T23:09:20.1935386Z The actual stderr differed from the expected stderr.
2019-09-10T23:09:20.1935901Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-extern/generic-extern.stderr
2019-09-10T23:09:20.1936324Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T23:09:20.1936748Z To only update this specific test, also pass `--test-args generic/generic-extern.rs`
2019-09-10T23:09:20.1937045Z error: 1 errors occurred comparing output.
2019-09-10T23:09:20.1937355Z status: exit code: 1
2019-09-10T23:09:20.1937355Z status: exit code: 1
2019-09-10T23:09:20.1938173Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic/generic-extern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-extern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-extern/auxiliary" "-A" "unused"
2019-09-10T23:09:20.1938898Z ------------------------------------------
2019-09-10T23:09:20.1939080Z 
2019-09-10T23:09:20.1939426Z ------------------------------------------
2019-09-10T23:09:20.1939729Z stderr:
2019-09-10T23:09:20.1939729Z stderr:
2019-09-10T23:09:20.1940084Z ------------------------------------------
2019-09-10T23:09:20.1940317Z error[E0044]: foreign items may not have type parameters
2019-09-10T23:09:20.1940702Z   --> /checkout/src/test/ui/generic/generic-extern.rs:2:5
2019-09-10T23:09:20.1940918Z    |
2019-09-10T23:09:20.1941061Z LL |     fn foo<T>(); //~ ERROR foreign items may not have type parameters
2019-09-10T23:09:20.1942128Z    |
2019-09-10T23:09:20.1942292Z    = help: replace the type parameters with concretetypes like `u32`
2019-09-10T23:09:20.1942492Z 
2019-09-10T23:09:20.1942691Z error: aborting due to previous error
---
2019-09-10T23:09:20.1950912Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-10T23:09:20.1950966Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-10T23:09:20.1950994Z 
2019-09-10T23:09:20.1951026Z 
2019-09-10T23:09:20.1953303Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-10T23:09:20.1953589Z 
2019-09-10T23:09:20.1953724Z 
2019-09-10T23:09:20.1958141Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-10T23:09:20.1958215Z Build completed unsuccessfully in 1:07:21
2019-09-10T23:09:20.1958215Z Build completed unsuccessfully in 1:07:21
2019-09-10T23:09:20.2014802Z == clock drift check ==
2019-09-10T23:09:20.2035442Z   local time: Tue Sep 10 23:09:20 UTC 2019
2019-09-10T23:09:20.3593247Z   network time: Tue, 10 Sep 2019 23:09:20 GMT
2019-09-10T23:09:20.3594464Z == end clock drift check ==
2019-09-10T23:09:21.2793254Z ##[error]Bash exited with code '1'.
2019-09-10T23:09:21.2836234Z ##[section]Starting: Checkout
2019-09-10T23:09:21.2837967Z ==============================================================================
2019-09-10T23:09:21.2838033Z Task         : Get sources
2019-09-10T23:09:21.2838073Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
