plain
2019-07-24T23:00:04.8669121Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T23:00:04.8845170Z ##[command]git config gc.auto 0
2019-07-24T23:00:04.8912247Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T23:00:04.8956519Z ##[command]git config --get-all http.proxy
2019-07-24T23:00:04.9087094Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62959/merge:refs/remotes/pull/62959/merge
---
2019-07-24T23:00:43.2903968Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T23:00:43.2904004Z 
2019-07-24T23:00:43.2904255Z   git checkout -b <new-branch-name>
2019-07-24T23:00:43.2904288Z 
2019-07-24T23:00:43.2904362Z HEAD is now at 548e199df Merge ac96d7734cd80556616f3c0d8deec4ee586caef6 into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-24T23:00:43.3048598Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T23:00:43.3051546Z ==============================================================================
2019-07-24T23:00:43.3051617Z Task         : Bash
2019-07-24T23:00:43.3051659Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T00:01:19.7826315Z .................................................................................................... 200/5851
2019-07-25T00:01:24.7180990Z .................................................................................................... 300/5851
2019-07-25T00:01:28.2207641Z .................................................................................................... 400/5851
2019-07-25T00:01:32.1469397Z .................................................................................................... 500/5851
2019-07-25T00:01:36.2358319Z ........................................................................i........................... 600/5851
2019-07-25T00:01:45.5572163Z ...........................................F........................................................ 800/5851
2019-07-25T00:01:51.4719580Z .................................................................................................... 900/5851
2019-07-25T00:01:56.7068599Z ...................................................................................................i 1000/5851
2019-07-25T00:01:56.7068599Z ...................................................................................................i 1000/5851
2019-07-25T00:02:02.4184061Z ...........i........................................................................................ 1100/5851
2019-07-25T00:02:06.4667817Z .............................iiiii.................................................................. 1200/5851
2019-07-25T00:02:12.7613154Z .................................................................................................... 1400/5851
2019-07-25T00:02:15.6152105Z .................................................................................................... 1500/5851
2019-07-25T00:02:19.5824992Z .................................................................................................... 1600/5851
2019-07-25T00:02:22.3745405Z .................................................................................................... 1700/5851
2019-07-25T00:02:22.3745405Z .................................................................................................... 1700/5851
2019-07-25T00:02:25.9643423Z .....................................................................i.............................. 1800/5851
2019-07-25T00:02:34.9827257Z .................................................................................................... 2000/5851
2019-07-25T00:02:39.4125210Z .................................................................................................... 2100/5851
2019-07-25T00:02:43.3146414Z .................................................................................................... 2200/5851
2019-07-25T00:02:43.3146414Z .................................................................................................... 2200/5851
2019-07-25T00:02:47.4502815Z ......................................................i............................................. 2300/5851
2019-07-25T00:02:57.9133905Z .................................................................................................... 2500/5851
2019-07-25T00:03:02.2646221Z .................................................................................................... 2600/5851
2019-07-25T00:03:07.6408917Z .................................................................................................... 2700/5851
2019-07-25T00:03:11.7289446Z .................................................................................................... 2800/5851
2019-07-25T00:03:11.7289446Z .................................................................................................... 2800/5851
2019-07-25T00:03:16.3968020Z .................................................................................................... 2900/5851
2019-07-25T00:03:21.8427343Z .................................................................................................... 3000/5851
2019-07-25T00:03:26.5173068Z .................................................................................................... 3100/5851
2019-07-25T00:03:31.9727729Z .................................................................................................... 3200/5851
2019-07-25T00:03:35.7765238Z .....................................................FF............................................. 3300/5851
2019-07-25T00:03:39.6855007Z .................................................................................................... 3400/5851
2019-07-25T00:03:45.0665044Z .................................................................................................... 3500/5851
2019-07-25T00:03:49.1103849Z .....................i.............................................................................. 3600/5851
2019-07-25T00:03:53.4731084Z ..............................................................................................ii...i 3700/5851
2019-07-25T00:03:57.4755746Z ..ii................................................................................................ 3800/5851
2019-07-25T00:04:06.5772059Z .................................................................................................... 4000/5851
2019-07-25T00:04:06.5772059Z .................................................................................................... 4000/5851
2019-07-25T00:04:10.6217374Z ........ii.......................................................................................... 4100/5851
2019-07-25T00:04:12.7883745Z .............................i...................................................................... 4200/5851
2019-07-25T00:04:14.9782260Z ................................................................................................i... 4300/5851
2019-07-25T00:04:21.9510950Z .................................................................................................... 4500/5851
2019-07-25T00:04:40.4148610Z .................................................................................................... 4600/5851
2019-07-25T00:04:44.1582520Z .................................................................................................... 4700/5851
2019-07-25T00:04:48.0403249Z .................................................................................................... 4800/5851
---
2019-07-25T00:05:23.1189456Z .................................................................................................... 5400/5851
2019-07-25T00:05:27.2253336Z .................................................................................................... 5500/5851
2019-07-25T00:05:31.5300712Z .................................................................................................... 5600/5851
2019-07-25T00:05:34.8034572Z .................................................................................................... 5700/5851
2019-07-25T00:05:37.9490051Z ...........................................................................................i........ 5800/5851
2019-07-25T00:05:39.8571322Z failures:
2019-07-25T00:05:39.8628391Z 
2019-07-25T00:05:39.8629151Z ---- [ui] ui/const-generics/array-impls/core-traits-no-impls-length-33.rs stdout ----
2019-07-25T00:05:39.8629225Z diff of stderr:
2019-07-25T00:05:39.8629225Z diff of stderr:
2019-07-25T00:05:39.8629264Z 
2019-07-25T00:05:39.8629590Z 46              <&'a [T] as std::iter::IntoIterator>
2019-07-25T00:05:39.8629885Z 47              <&'a mut [T; _] as std::iter::IntoIterator>
2019-07-25T00:05:39.8630236Z 48              <&'a mut [T] as std::iter::IntoIterator>
2019-07-25T00:05:39.8630313Z +              <[T; _] as std::iter::IntoIterator>
2019-07-25T00:05:39.8630418Z 50 
2019-07-25T00:05:39.8630506Z 51 error: aborting due to 5 previous errors
2019-07-25T00:05:39.8630543Z 
2019-07-25T00:05:39.8630571Z 
2019-07-25T00:05:39.8630571Z 
2019-07-25T00:05:39.8630622Z The actual stderr differed from the expected stderr.
2019-07-25T00:05:39.8631053Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2019-07-25T00:05:39.8631619Z To update references, rerun the tests and pass the `--bless` flag
2019-07-25T00:05:39.8632399Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2019-07-25T00:05:39.8632545Z error: 1 errors occurred comparing output.
2019-07-25T00:05:39.8632596Z status: exit code: 1
2019-07-25T00:05:39.8632596Z status: exit code: 1
2019-07-25T00:05:39.8633590Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary" "-A" "unused"
2019-07-25T00:05:39.8634013Z ------------------------------------------
2019-07-25T00:05:39.8634053Z 
2019-07-25T00:05:39.8634311Z ------------------------------------------
2019-07-25T00:05:39.8634362Z stderr:
2019-07-25T00:05:39.8634362Z stderr:
2019-07-25T00:05:39.8634630Z ------------------------------------------
2019-07-25T00:05:39.8634691Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-07-25T00:05:39.8635010Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2019-07-25T00:05:39.8635086Z    |
2019-07-25T00:05:39.8635144Z LL |     println!("{:?}", [0_usize; 33]);
2019-07-25T00:05:39.8635207Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-07-25T00:05:39.8635280Z    |
2019-07-25T00:05:39.8635338Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2019-07-25T00:05:39.8635406Z    = note: required by `std::fmt::Debug::fmt`
2019-07-25T00:05:39.8635456Z 
2019-07-25T00:05:39.8635509Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-07-25T00:05:39.8635833Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:9:9
2019-07-25T00:05:39.8635905Z    |
2019-07-25T00:05:39.8635954Z LL |     set.insert([0_usize; 33]);
2019-07-25T00:05:39.8636012Z    |         ^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-07-25T00:05:39.8636064Z    |
2019-07-25T00:05:39.8636275Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-07-25T00:05:39.8636323Z 
2019-07-25T00:05:39.8636467Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-07-25T00:05:39.8636847Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:14:19
2019-07-25T00:05:39.8636915Z    |
2019-07-25T00:05:39.8636980Z LL |     [0_usize; 33] == [1_usize; 33]
2019-07-25T00:05:39.8637256Z    |     ------------- ^^ ------------- [usize; 33]
2019-07-25T00:05:39.8637357Z    |     [usize; 33]
2019-07-25T00:05:39.8637420Z    |
2019-07-25T00:05:39.8637420Z    |
2019-07-25T00:05:39.8637474Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2019-07-25T00:05:39.8637663Z 
2019-07-25T00:05:39.8637732Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-07-25T00:05:39.8638084Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:19:19
2019-07-25T00:05:39.8638222Z    |
2019-07-25T00:05:39.8638287Z LL |     [0_usize; 33] < [1_usize; 33]
2019-07-25T00:05:39.8638557Z    |     ------------- ^ ------------- [usize; 33]
2019-07-25T00:05:39.8638654Z    |     [usize; 33]
2019-07-25T00:05:39.8638715Z    |
2019-07-25T00:05:39.8638715Z    |
2019-07-25T00:05:39.8638896Z    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2019-07-25T00:05:39.8638933Z 
2019-07-25T00:05:39.8639002Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-07-25T00:05:39.8639360Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:24:14
2019-07-25T00:05:39.8639416Z    |
2019-07-25T00:05:39.8639481Z LL |     for _ in &[0_usize; 33] {
2019-07-25T00:05:39.8639541Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2019-07-25T00:05:39.8639643Z    = help: the following implementations were found:
2019-07-25T00:05:39.8639643Z    = help: the following implementations were found:
2019-07-25T00:05:39.8639942Z              <&'a [T; _] as std::iter::IntoIterator>
2019-07-25T00:05:39.8640212Z              <&'a [T] as std::iter::IntoIterator>
2019-07-25T00:05:39.8640485Z              <&'a mut [T; _] as std::iter::IntoIterator>
2019-07-25T00:05:39.8640778Z              <&'a mut [T] as std::iter::IntoIterator>
2019-07-25T00:05:39.8640845Z              <[T; _] as std::iter::IntoIterator>
2019-07-25T00:05:39.8640951Z 
2019-07-25T00:05:39.8640999Z error: aborting due to 5 previous errors
2019-07-25T00:05:39.8641031Z 
2019-07-25T00:05:39.8641081Z Some errors have detailed explanations: E0277, E0369.
---
2019-07-25T00:05:39.8642690Z ---- [ui] ui/iterators/array.rs stdout ----
2019-07-25T00:05:39.8642738Z 
2019-07-25T00:05:39.8642788Z error: ui test compiled successfully!
2019-07-25T00:05:39.8642858Z status: exit code: 0
2019-07-25T00:05:39.8643657Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array/auxiliary" "-A" "unused"
2019-07-25T00:05:39.8644047Z ------------------------------------------
2019-07-25T00:05:39.8644085Z 
2019-07-25T00:05:39.8644359Z ------------------------------------------
2019-07-25T00:05:39.8644411Z stderr:
---
2019-07-25T00:05:39.8645478Z ---- [ui] ui/iterators/array-of-ranges.rs stdout ----
2019-07-25T00:05:39.8645515Z 
2019-07-25T00:05:39.8645591Z error: ui test compiled successfully!
2019-07-25T00:05:39.8645640Z status: exit code: 0
2019-07-25T00:05:39.8646451Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/array-of-ranges.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array-of-ranges" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array-of-ranges/auxiliary" "-A" "unused"
2019-07-25T00:05:39.8646855Z ------------------------------------------
2019-07-25T00:05:39.8646894Z 
2019-07-25T00:05:39.8647149Z ------------------------------------------
2019-07-25T00:05:39.8647200Z stderr:
---
2019-07-25T00:05:39.8656140Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-25T00:05:39.8656446Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T00:05:39.8669917Z 
2019-07-25T00:05:39.8670076Z 
2019-07-25T00:05:39.8675765Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-25T00:05:39.8676480Z 
2019-07-25T00:05:39.8676691Z 
2019-07-25T00:05:39.8688639Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-25T00:05:39.8688798Z Build completed unsuccessfully in 0:58:32
2019-07-25T00:05:39.8688798Z Build completed unsuccessfully in 0:58:32
2019-07-25T00:05:40.8880475Z ##[error]Bash exited with code '1'.
2019-07-25T00:05:40.8932100Z ##[section]Starting: Checkout
2019-07-25T00:05:40.8935486Z ==============================================================================
2019-07-25T00:05:40.8935561Z Task         : Get sources
2019-07-25T00:05:40.8935612Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
