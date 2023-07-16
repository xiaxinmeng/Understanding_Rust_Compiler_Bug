plain
2019-12-06T16:20:56.3512939Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-06T16:20:56.3753728Z ##[command]git config gc.auto 0
2019-12-06T16:20:56.3831424Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-06T16:20:56.3892946Z ##[command]git config --get-all http.proxy
2019-12-06T16:20:56.4038053Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66933/merge:refs/remotes/pull/66933/merge
---
2019-12-06T17:19:16.4283773Z .................................................................................................... 1600/9328
2019-12-06T17:19:20.9910183Z .................................................................................................... 1700/9328
2019-12-06T17:19:32.9349815Z ...........................................i........................................................ 1800/9328
2019-12-06T17:19:40.7420937Z .................................................................................................... 1900/9328
2019-12-06T17:19:54.5364761Z ............................iiiii................................................................... 2000/9328
2019-12-06T17:20:04.4871256Z .................................................................................................... 2200/9328
2019-12-06T17:20:06.9705868Z .................................................................................................... 2300/9328
2019-12-06T17:20:11.4170086Z .................................................................................................... 2400/9328
2019-12-06T17:20:32.5355172Z .................................................................................................... 2500/9328
---
2019-12-06T17:23:06.7166547Z .............................i...............i...................................................... 4800/9328
2019-12-06T17:23:16.4137335Z .................................................................................................... 4900/9328
2019-12-06T17:23:22.9091866Z .................................................................................................... 5000/9328
2019-12-06T17:23:28.8504088Z .................................................................................................... 5100/9328
2019-12-06T17:23:37.9361264Z .......................................ii.ii...........i............................................ 5200/9328
2019-12-06T17:23:47.2179464Z .................................................................................................... 5400/9328
2019-12-06T17:23:56.7107604Z .................................................................................................... 5500/9328
2019-12-06T17:24:03.9943794Z .....................i.............................................................................. 5600/9328
2019-12-06T17:24:09.9157589Z .................................................................................................... 5700/9328
2019-12-06T17:24:09.9157589Z .................................................................................................... 5700/9328
2019-12-06T17:24:21.1625575Z .................................................................................................... 5800/9328
2019-12-06T17:24:32.5239240Z .......ii...i..ii...........i....................................................................... 5900/9328
2019-12-06T17:24:49.6929785Z .................................................................................................... 6100/9328
2019-12-06T17:24:56.0555742Z .................................................................................................... 6200/9328
2019-12-06T17:24:56.0555742Z .................................................................................................... 6200/9328
2019-12-06T17:25:09.4130660Z ..............................i..ii................................................................. 6300/9328
2019-12-06T17:25:29.3789271Z .................................................................................................... 6500/9328
2019-12-06T17:25:31.4243881Z ..i................................................................................................. 6600/9328
2019-12-06T17:25:33.5728770Z ..............................................................................................i..... 6700/9328
2019-12-06T17:25:36.1911656Z .................................................................................................... 6800/9328
---
2019-12-06T17:27:11.8880721Z .................................................................................................... 7400/9328
2019-12-06T17:27:17.0730859Z .................................................................................................... 7500/9328
2019-12-06T17:27:23.4368355Z .................................................................................................... 7600/9328
2019-12-06T17:27:34.2004400Z .................................................................................................... 7700/9328
2019-12-06T17:27:40.2503812Z .......iiii......................................................................................... 7800/9328
2019-12-06T17:27:54.9727558Z .................................................................................................... 8000/9328
2019-12-06T17:28:06.0669464Z .................................................................................................... 8100/9328
2019-12-06T17:28:18.1459553Z .................................................................................................... 8200/9328
2019-12-06T17:28:24.4140927Z .................................................................................................... 8300/9328
---
2019-12-06T17:30:16.0417838Z ---- [ui] ui/const-generics/array-impls/core-traits-no-impls-length-33.rs stdout ----
2019-12-06T17:30:16.0418072Z diff of stderr:
2019-12-06T17:30:16.0418194Z 
2019-12-06T17:30:16.0418321Z 15    |
2019-12-06T17:30:16.0418528Z 16    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-06T17:30:16.0418796Z + error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-06T17:30:16.0419183Z +   --> $DIR/core-traits-no-impls-length-33.rs:8:19
2019-12-06T17:30:16.0419339Z +    |
2019-12-06T17:30:16.0419488Z + LL |     let mut set = HashSet::new();
2019-12-06T17:30:16.0419488Z + LL |     let mut set = HashSet::new();
2019-12-06T17:30:16.0419625Z +    |                   ^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-06T17:30:16.0419754Z +    |
2019-12-06T17:30:16.0419901Z +    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-06T17:30:16.0420034Z +    = note: required by `std::collections::HashSet::<T>::new`
2019-12-06T17:30:16.0420302Z 18 error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-06T17:30:16.0420637Z 19   --> $DIR/core-traits-no-impls-length-33.rs:14:19
2019-12-06T17:30:16.0421005Z 20    |
2019-12-06T17:30:16.0421151Z 
2019-12-06T17:30:16.0421151Z 
2019-12-06T17:30:16.0422001Z 48              <&'a mut [T] as std::iter::IntoIterator>
2019-12-06T17:30:16.0422389Z 50 
2019-12-06T17:30:16.0422744Z - error: aborting due to 5 previous errors
2019-12-06T17:30:16.0422918Z + error: aborting due to 6 previous errors
2019-12-06T17:30:16.0423080Z 52 
2019-12-06T17:30:16.0423080Z 52 
2019-12-06T17:30:16.0423221Z 53 Some errors have detailed explanations: E0277, E0369.
2019-12-06T17:30:16.0423621Z 54 For more information about an error, try `rustc --explain E0277`.
2019-12-06T17:30:16.0423780Z 
2019-12-06T17:30:16.0423901Z 
2019-12-06T17:30:16.0424044Z The actual stderr differed from the expected stderr.
2019-12-06T17:30:16.0424552Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2019-12-06T17:30:16.0425276Z To update references, rerun the tests and pass the `--bless` flag
2019-12-06T17:30:16.0425782Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2019-12-06T17:30:16.0426559Z error: 1 errors occurred comparing output.
2019-12-06T17:30:16.0427234Z status: exit code: 1
2019-12-06T17:30:16.0427234Z status: exit code: 1
2019-12-06T17:30:16.0428306Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary" "-A" "unused"
2019-12-06T17:30:16.0428952Z ------------------------------------------
2019-12-06T17:30:16.0429131Z 
2019-12-06T17:30:16.0429494Z ------------------------------------------
2019-12-06T17:30:16.0429661Z stderr:
2019-12-06T17:30:16.0429661Z stderr:
2019-12-06T17:30:16.0430020Z ------------------------------------------
2019-12-06T17:30:16.0430198Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-06T17:30:16.0430785Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2019-12-06T17:30:16.0430972Z    |
2019-12-06T17:30:16.0431095Z LL |     println!("{:?}", [0_usize; 33]);
2019-12-06T17:30:16.0431246Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-06T17:30:16.0431933Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2019-12-06T17:30:16.0432119Z    = note: required by `std::fmt::Debug::fmt`
2019-12-06T17:30:16.0432252Z 
2019-12-06T17:30:16.0432399Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-06T17:30:16.0432399Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-06T17:30:16.0432867Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:9:16
2019-12-06T17:30:16.0433047Z    |
2019-12-06T17:30:16.0433186Z LL |     set.insert([0_usize; 33]);
2019-12-06T17:30:16.0433381Z    |                ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-06T17:30:16.0433524Z    |
2019-12-06T17:30:16.0433689Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-06T17:30:16.0433960Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-06T17:30:16.0434588Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:8:19
2019-12-06T17:30:16.0436686Z    |
2019-12-06T17:30:16.0437109Z LL |     let mut set = HashSet::new();
2019-12-06T17:30:16.0437109Z LL |     let mut set = HashSet::new();
2019-12-06T17:30:16.0437303Z    |                   ^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-06T17:30:16.0437435Z    |
2019-12-06T17:30:16.0437565Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-06T17:30:16.0437714Z    = note: required by `std::collections::HashSet::<T>::new`
2019-12-06T17:30:16.0437947Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-06T17:30:16.0438421Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:14:19
2019-12-06T17:30:16.0438590Z    |
2019-12-06T17:30:16.0438590Z    |
2019-12-06T17:30:16.0438712Z LL |     [0_usize; 33] == [1_usize; 33]
2019-12-06T17:30:16.0439159Z    |     ------------- ^^ ------------- [usize; 33]
2019-12-06T17:30:16.0439458Z    |     [usize; 33]
2019-12-06T17:30:16.0439591Z    |
2019-12-06T17:30:16.0439825Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2019-12-06T17:30:16.0439999Z 
2019-12-06T17:30:16.0439999Z 
2019-12-06T17:30:16.0440130Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-12-06T17:30:16.0440518Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:19:19
2019-12-06T17:30:16.0440695Z    |
2019-12-06T17:30:16.0440816Z LL |     [0_usize; 33] < [1_usize; 33]
2019-12-06T17:30:16.0441849Z    |     ------------- ^ ------------- [usize; 33]
2019-12-06T17:30:16.0442869Z    |     [usize; 33]
2019-12-06T17:30:16.0442911Z    |
2019-12-06T17:30:16.0442911Z    |
2019-12-06T17:30:16.0442979Z    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2019-12-06T17:30:16.0443013Z 
2019-12-06T17:30:16.0443062Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-12-06T17:30:16.0443438Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:24:14
2019-12-06T17:30:16.0443501Z    |
2019-12-06T17:30:16.0443551Z LL |     for _ in &[0_usize; 33] {
2019-12-06T17:30:16.0443603Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2019-12-06T17:30:16.0443712Z    = help: the following implementations were found:
2019-12-06T17:30:16.0443712Z    = help: the following implementations were found:
2019-12-06T17:30:16.0443950Z              <&'a [T; _] as std::iter::IntoIterator>
2019-12-06T17:30:16.0444195Z              <&'a [T] as std::iter::IntoIterator>
2019-12-06T17:30:16.0444428Z              <&'a mut [T; _] as std::iter::IntoIterator>
2019-12-06T17:30:16.0444825Z              <&'a mut [T] as std::iter::IntoIterator>
2019-12-06T17:30:16.0444912Z 
2019-12-06T17:30:16.0444949Z error: aborting due to 6 previous errors
2019-12-06T17:30:16.0444973Z 
2019-12-06T17:30:16.0445028Z Some errors have detailed explanations: E0277, E0369.
---
2019-12-06T17:30:16.0446439Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-06T17:30:16.0446488Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-06T17:30:16.0450311Z 
2019-12-06T17:30:16.0450369Z 
2019-12-06T17:30:16.0453529Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-06T17:30:16.0453871Z 
2019-12-06T17:30:16.0453918Z 
2019-12-06T17:30:16.0550707Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-06T17:30:16.0550800Z Build completed unsuccessfully in 1:03:13
2019-12-06T17:30:16.0550800Z Build completed unsuccessfully in 1:03:13
2019-12-06T17:30:16.0597259Z == clock drift check ==
2019-12-06T17:30:16.0613627Z   local time: Fri Dec  6 17:30:16 UTC 2019
2019-12-06T17:30:16.6150949Z   network time: Fri, 06 Dec 2019 17:30:16 GMT
2019-12-06T17:30:16.6162214Z == end clock drift check ==
2019-12-06T17:30:17.6280691Z 
2019-12-06T17:30:17.6430068Z ##[error]Bash exited with code '1'.
2019-12-06T17:30:17.6463400Z ##[section]Starting: Checkout
2019-12-06T17:30:17.6465474Z ==============================================================================
2019-12-06T17:30:17.6465533Z Task         : Get sources
2019-12-06T17:30:17.6465587Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
