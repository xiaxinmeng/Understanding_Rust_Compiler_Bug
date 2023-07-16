plain
2019-12-26T19:22:26.4774473Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T19:22:27.3755736Z ##[command]git config gc.auto 0
2019-12-26T19:22:27.3761106Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T19:22:27.3765658Z ##[command]git config --get-all http.proxy
2019-12-26T19:22:27.3770582Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67642/merge:refs/remotes/pull/67642/merge
---
2019-12-26T20:24:12.1786334Z .................................................................................................... 1600/9454
2019-12-26T20:24:17.1051736Z .................................................................................................... 1700/9454
2019-12-26T20:24:26.9343551Z ..............................................................................................i..... 1800/9454
2019-12-26T20:24:35.2669281Z .................................................................................................... 1900/9454
2019-12-26T20:24:42.0488140Z ...............................................................................iiiii................ 2000/9454
2019-12-26T20:25:04.1386033Z .................................................................................................... 2200/9454
2019-12-26T20:25:06.6015517Z .................................................................................................... 2300/9454
2019-12-26T20:25:09.2421595Z .................................................................................................... 2400/9454
2019-12-26T20:25:15.7761896Z .................................................................................................... 2500/9454
---
2019-12-26T20:28:19.0324147Z ..........i...............i......................................................................... 4900/9454
2019-12-26T20:28:29.3874131Z .................................................................................................... 5000/9454
2019-12-26T20:28:34.5683745Z ......................................................i............................................. 5100/9454
2019-12-26T20:28:44.1682390Z .................................................................................................... 5200/9454
2019-12-26T20:28:50.6276057Z .....................ii.ii...........i.............................................................. 5300/9454
2019-12-26T20:28:59.8471321Z .................................................................................................... 5500/9454
2019-12-26T20:29:11.0168981Z .................................................................................................... 5600/9454
2019-12-26T20:29:18.5464324Z ...i................................................................................................ 5700/9454
2019-12-26T20:29:24.3990269Z .................................................................................................... 5800/9454
2019-12-26T20:29:24.3990269Z .................................................................................................... 5800/9454
2019-12-26T20:29:34.6352588Z ...........................................................................................ii...i..i 5900/9454
2019-12-26T20:29:47.4560925Z i............i...................................................................................... 6000/9454
2019-12-26T20:30:06.5475646Z .................................................................................................... 6200/9454
2019-12-26T20:30:13.3618270Z .................................................................................................... 6300/9454
2019-12-26T20:30:13.3618270Z .................................................................................................... 6300/9454
2019-12-26T20:30:28.4376280Z ..................i..ii............................................................................. 6400/9454
2019-12-26T20:30:48.6576270Z ..............................................................................................i..... 6600/9454
2019-12-26T20:30:50.8969523Z .................................................................................................... 6700/9454
2019-12-26T20:30:53.2346395Z ..............................................................................................i..... 6800/9454
2019-12-26T20:30:55.9926247Z .................................................................................................... 6900/9454
---
2019-12-26T20:32:36.6287595Z .................................................................................................... 7500/9454
2019-12-26T20:32:41.6488113Z .................................................................................................... 7600/9454
2019-12-26T20:32:48.4833538Z .................................................................................................... 7700/9454
2019-12-26T20:32:59.2233178Z .................................................................................................... 7800/9454
2019-12-26T20:33:05.9032876Z .........................iiii....................................................................... 7900/9454
2019-12-26T20:33:20.6807012Z .................................................................................................... 8100/9454
2019-12-26T20:33:30.5881883Z .................................................................................................... 8200/9454
2019-12-26T20:33:44.5425714Z .................................................................................................... 8300/9454
2019-12-26T20:33:51.9586164Z .................................................................................................... 8400/9454
---
2019-12-26T20:35:49.1250313Z ---- [ui] ui/const-generics/array-impls/core-traits-no-impls-length-33.rs stdout ----
2019-12-26T20:35:49.1250640Z diff of stderr:
2019-12-26T20:35:49.1250786Z 
2019-12-26T20:35:49.1250935Z 15    |
2019-12-26T20:35:49.1251118Z 16    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-26T20:35:49.1252038Z - error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-26T20:35:49.1252569Z -   --> $DIR/core-traits-no-impls-length-33.rs:8:19
2019-12-26T20:35:49.1252925Z -    |
2019-12-26T20:35:49.1253320Z - LL |     let mut set = HashSet::new();
2019-12-26T20:35:49.1253320Z - LL |     let mut set = HashSet::new();
2019-12-26T20:35:49.1254177Z -    |                   ^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-26T20:35:49.1255571Z -    |
2019-12-26T20:35:49.1256050Z -    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-26T20:35:49.1256472Z -    = note: required by `std::collections::HashSet::<T>::new`
2019-12-26T20:35:49.1257899Z 27 error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-26T20:35:49.1258418Z 28   --> $DIR/core-traits-no-impls-length-33.rs:15:19
2019-12-26T20:35:49.1258584Z 29    |
2019-12-26T20:35:49.1260429Z 
2019-12-26T20:35:49.1260429Z 
2019-12-26T20:35:49.1261219Z 57              <&'a mut [T] as std::iter::IntoIterator>
2019-12-26T20:35:49.1262246Z 59 
2019-12-26T20:35:49.1262670Z - error: aborting due to 6 previous errors
2019-12-26T20:35:49.1262861Z + error: aborting due to 5 previous errors
2019-12-26T20:35:49.1263030Z 61 
2019-12-26T20:35:49.1263030Z 61 
2019-12-26T20:35:49.1263182Z 62 Some errors have detailed explanations: E0277, E0369.
2019-12-26T20:35:49.1263588Z 63 For more information about an error, try `rustc --explain E0277`.
2019-12-26T20:35:49.1263754Z 
2019-12-26T20:35:49.1263900Z 
2019-12-26T20:35:49.1264051Z The actual stderr differed from the expected stderr.
2019-12-26T20:35:49.1264571Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2019-12-26T20:35:49.1265033Z To update references, rerun the tests and pass the `--bless` flag
2019-12-26T20:35:49.1265552Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2019-12-26T20:35:49.1265875Z error: 1 errors occurred comparing output.
2019-12-26T20:35:49.1266206Z status: exit code: 1
2019-12-26T20:35:49.1266206Z status: exit code: 1
2019-12-26T20:35:49.1267899Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary" "-A" "unused"
2019-12-26T20:35:49.1269638Z ------------------------------------------
2019-12-26T20:35:49.1269827Z 
2019-12-26T20:35:49.1270173Z ------------------------------------------
2019-12-26T20:35:49.1270343Z stderr:
2019-12-26T20:35:49.1270343Z stderr:
2019-12-26T20:35:49.1270699Z ------------------------------------------
2019-12-26T20:35:49.1274739Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-26T20:35:49.1277092Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2019-12-26T20:35:49.1277332Z    |
2019-12-26T20:35:49.1277680Z LL |     println!("{:?}", [0_usize; 33]);
2019-12-26T20:35:49.1277871Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-26T20:35:49.1278174Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2019-12-26T20:35:49.1278337Z    = note: required by `std::fmt::Debug::fmt`
2019-12-26T20:35:49.1278468Z 
2019-12-26T20:35:49.1278800Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-26T20:35:49.1278800Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-26T20:35:49.1279601Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:10:16
2019-12-26T20:35:49.1279791Z    |
2019-12-26T20:35:49.1279960Z LL |     set.insert([0_usize; 33]);
2019-12-26T20:35:49.1280120Z    |                ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-26T20:35:49.1280280Z    |
2019-12-26T20:35:49.1280522Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-26T20:35:49.1281207Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-26T20:35:49.1282253Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:15:19
2019-12-26T20:35:49.1284309Z    |
2019-12-26T20:35:49.1284309Z    |
2019-12-26T20:35:49.1284638Z LL |     [0_usize; 33] == [1_usize; 33]
2019-12-26T20:35:49.1285039Z    |     ------------- ^^ ------------- [usize; 33]
2019-12-26T20:35:49.1285134Z    |     [usize; 33]
2019-12-26T20:35:49.1285196Z    |
2019-12-26T20:35:49.1285246Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2019-12-26T20:35:49.1285281Z 
2019-12-26T20:35:49.1285281Z 
2019-12-26T20:35:49.1285329Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-12-26T20:35:49.1285631Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:20:19
2019-12-26T20:35:49.1285690Z    |
2019-12-26T20:35:49.1285733Z LL |     [0_usize; 33] < [1_usize; 33]
2019-12-26T20:35:49.1285975Z    |     ------------- ^ ------------- [usize; 33]
2019-12-26T20:35:49.1286064Z    |     [usize; 33]
2019-12-26T20:35:49.1286284Z    |
2019-12-26T20:35:49.1286284Z    |
2019-12-26T20:35:49.1286334Z    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2019-12-26T20:35:49.1286531Z 
2019-12-26T20:35:49.1286584Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-12-26T20:35:49.1287027Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:25:14
2019-12-26T20:35:49.1287072Z    |
2019-12-26T20:35:49.1287111Z LL |     for _ in &[0_usize; 33] {
2019-12-26T20:35:49.1287339Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2019-12-26T20:35:49.1287598Z    = help: the following implementations were found:
2019-12-26T20:35:49.1287598Z    = help: the following implementations were found:
2019-12-26T20:35:49.1288004Z              <&'a [T; _] as std::iter::IntoIterator>
2019-12-26T20:35:49.1288202Z              <&'a [T] as std::iter::IntoIterator>
2019-12-26T20:35:49.1288406Z              <&'a mut [T; _] as std::iter::IntoIterator>
2019-12-26T20:35:49.1288623Z              <&'a mut [T] as std::iter::IntoIterator>
2019-12-26T20:35:49.1288698Z 
2019-12-26T20:35:49.1288810Z error: aborting due to 5 previous errors
2019-12-26T20:35:49.1288861Z 
2019-12-26T20:35:49.1288902Z Some errors have detailed explanations: E0277, E0369.
---
2019-12-26T20:35:49.1290000Z test result: FAILED. 9406 passed; 1 failed; 47 ignored; 0 measured; 0 filtered out
2019-12-26T20:35:49.1290049Z 
2019-12-26T20:35:49.1290072Z 
2019-12-26T20:35:49.1290097Z 
2019-12-26T20:35:49.1292240Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-26T20:35:49.1292497Z 
2019-12-26T20:35:49.1292528Z 
2019-12-26T20:35:49.1292855Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-26T20:35:49.1292917Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-26T20:35:49.1292917Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-26T20:35:49.1294574Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-26T20:35:49.1294659Z Build completed unsuccessfully in 1:06:20
2019-12-26T20:35:49.1352411Z == clock drift check ==
2019-12-26T20:35:49.1377714Z   local time: Thu Dec 26 20:35:49 UTC 2019
2019-12-26T20:35:49.2451433Z   network time: Thu, 26 Dec 2019 20:35:49 GMT
2019-12-26T20:35:49.2453727Z == end clock drift check ==
2019-12-26T20:35:50.3076421Z 
2019-12-26T20:35:50.3187034Z ##[error]Bash exited with code '1'.
2019-12-26T20:35:50.3233344Z ##[section]Starting: Checkout
2019-12-26T20:35:50.3235144Z ==============================================================================
2019-12-26T20:35:50.3235221Z Task         : Get sources
2019-12-26T20:35:50.3235274Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
