plain
2019-12-26T22:50:33.8714030Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T22:50:33.8731087Z ##[command]git config gc.auto 0
2019-12-26T22:50:33.8733928Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T22:50:33.8736858Z ##[command]git config --get-all http.proxy
2019-12-26T22:50:33.8739306Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67642/merge:refs/remotes/pull/67642/merge
---
2019-12-26T23:53:04.7459203Z .................................................................................................... 1600/9455
2019-12-26T23:53:09.9326183Z .................................................................................................... 1700/9455
2019-12-26T23:53:19.9258673Z .............................................................................................i...... 1800/9455
2019-12-26T23:53:28.5551899Z .................................................................................................... 1900/9455
2019-12-26T23:53:35.6423539Z ...............................................................................iiiii................ 2000/9455
2019-12-26T23:53:58.4831435Z .................................................................................................... 2200/9455
2019-12-26T23:54:00.9873085Z .................................................................................................... 2300/9455
2019-12-26T23:54:03.6722711Z .................................................................................................... 2400/9455
2019-12-26T23:54:10.3211033Z .................................................................................................... 2500/9455
---
2019-12-26T23:57:17.3139580Z ..........i...............i......................................................................... 4900/9455
2019-12-26T23:57:27.7274689Z .................................................................................................... 5000/9455
2019-12-26T23:57:32.9979015Z ......................................................i............................................. 5100/9455
2019-12-26T23:57:42.8918905Z .................................................................................................... 5200/9455
2019-12-26T23:57:49.5091380Z .....................ii.ii...........i.............................................................. 5300/9455
2019-12-26T23:57:59.0279142Z .................................................................................................... 5500/9455
2019-12-26T23:58:10.4486694Z .................................................................................................... 5600/9455
2019-12-26T23:58:18.0421934Z ...i................................................................................................ 5700/9455
2019-12-26T23:58:23.9943519Z .................................................................................................... 5800/9455
2019-12-26T23:58:23.9943519Z .................................................................................................... 5800/9455
2019-12-26T23:58:34.6370576Z ...........................................................................................ii...i..i 5900/9455
2019-12-26T23:58:47.8313982Z i...........i....................................................................................... 6000/9455
2019-12-26T23:59:06.2283898Z .................................................................................................... 6200/9455
2019-12-26T23:59:13.8599721Z .................................................................................................... 6300/9455
2019-12-26T23:59:13.8599721Z .................................................................................................... 6300/9455
2019-12-26T23:59:31.2246975Z ..................i..ii............................................................................. 6400/9455
2019-12-26T23:59:51.9843990Z ...............................................................................................i.... 6600/9455
2019-12-26T23:59:54.2539992Z .................................................................................................... 6700/9455
2019-12-26T23:59:56.6400083Z ...............................................................................................i.... 6800/9455
2019-12-26T23:59:59.4505121Z .................................................................................................... 6900/9455
---
2019-12-27T00:01:42.8250873Z .................................................................................................... 7500/9455
2019-12-27T00:01:47.9038732Z .................................................................................................... 7600/9455
2019-12-27T00:01:54.9540174Z .................................................................................................... 7700/9455
2019-12-27T00:02:06.1131880Z .................................................................................................... 7800/9455
2019-12-27T00:02:13.0664847Z ..........................iiii...................................................................... 7900/9455
2019-12-27T00:02:28.4860979Z .................................................................................................... 8100/9455
2019-12-27T00:02:38.5788464Z .................................................................................................... 8200/9455
2019-12-27T00:02:52.9164876Z .................................................................................................... 8300/9455
2019-12-27T00:03:00.5837149Z .................................................................................................... 8400/9455
---
2019-12-27T00:05:02.4815984Z 10 error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-27T00:05:02.4816421Z -   --> $DIR/core-traits-no-impls-length-33.rs:10:16
2019-12-27T00:05:02.4817130Z +   --> $DIR/core-traits-no-impls-length-33.rs:9:16
2019-12-27T00:05:02.4817322Z 12    |
2019-12-27T00:05:02.4817714Z 13 LL |     set.insert([0_usize; 33]);
2019-12-27T00:05:02.4817924Z 14    |                ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-27T00:05:02.4818062Z 
2019-12-27T00:05:02.4818270Z 16    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-27T00:05:02.4818580Z 18 error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-27T00:05:02.4819350Z -   --> $DIR/core-traits-no-impls-length-33.rs:15:19
2019-12-27T00:05:02.4820183Z +   --> $DIR/core-traits-no-impls-length-33.rs:14:19
2019-12-27T00:05:02.4820641Z 20    |
2019-12-27T00:05:02.4820641Z 20    |
2019-12-27T00:05:02.4820798Z 21 LL |     [0_usize; 33] == [1_usize; 33]
2019-12-27T00:05:02.4821210Z 22    |     ------------- ^^ ------------- [usize; 33]
2019-12-27T00:05:02.4821578Z 26    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2019-12-27T00:05:02.4821723Z 27 
2019-12-27T00:05:02.4821723Z 27 
2019-12-27T00:05:02.4821896Z 28 error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-12-27T00:05:02.4822488Z -   --> $DIR/core-traits-no-impls-length-33.rs:20:19
2019-12-27T00:05:02.4826019Z +   --> $DIR/core-traits-no-impls-length-33.rs:19:19
2019-12-27T00:05:02.4826306Z 30    |
2019-12-27T00:05:02.4826455Z 31 LL |     [0_usize; 33] < [1_usize; 33]
2019-12-27T00:05:02.4827017Z 32    |     ------------- ^ ------------- [usize; 33]
2019-12-27T00:05:02.4827414Z 
2019-12-27T00:05:02.4827738Z 36    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2019-12-27T00:05:02.4827907Z 37 
2019-12-27T00:05:02.4828059Z 38 error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-12-27T00:05:02.4829118Z -   --> $DIR/core-traits-no-impls-length-33.rs:25:14
2019-12-27T00:05:02.4829751Z +   --> $DIR/core-traits-no-impls-length-33.rs:24:14
2019-12-27T00:05:02.4829943Z 40    |
2019-12-27T00:05:02.4830088Z 41 LL |     for _ in &[0_usize; 33] {
2019-12-27T00:05:02.4830274Z 42    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2019-12-27T00:05:02.4830523Z 
2019-12-27T00:05:02.4830685Z The actual stderr differed from the expected stderr.
2019-12-27T00:05:02.4830685Z The actual stderr differed from the expected stderr.
2019-12-27T00:05:02.4831194Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2019-12-27T00:05:02.4831626Z To update references, rerun the tests and pass the `--bless` flag
2019-12-27T00:05:02.4832145Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2019-12-27T00:05:02.4832461Z error: 1 errors occurred comparing output.
2019-12-27T00:05:02.4832924Z status: exit code: 1
2019-12-27T00:05:02.4832924Z status: exit code: 1
2019-12-27T00:05:02.4834024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary" "-A" "unused"
2019-12-27T00:05:02.4834847Z ------------------------------------------
2019-12-27T00:05:02.4835171Z 
2019-12-27T00:05:02.4835827Z ------------------------------------------
2019-12-27T00:05:02.4836217Z stderr:
2019-12-27T00:05:02.4836217Z stderr:
2019-12-27T00:05:02.4836933Z ------------------------------------------
2019-12-27T00:05:02.4837228Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-27T00:05:02.4837972Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2019-12-27T00:05:02.4841909Z    |
2019-12-27T00:05:02.4845291Z LL |     println!("{:?}", [0_usize; 33]);
2019-12-27T00:05:02.4851033Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-27T00:05:02.4852368Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2019-12-27T00:05:02.4852583Z    = note: required by `std::fmt::Debug::fmt`
2019-12-27T00:05:02.4852713Z 
2019-12-27T00:05:02.4852974Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-27T00:05:02.4852974Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-27T00:05:02.4853587Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:9:16
2019-12-27T00:05:02.4853819Z    |
2019-12-27T00:05:02.4854068Z LL |     set.insert([0_usize; 33]);
2019-12-27T00:05:02.4854318Z    |                ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-27T00:05:02.4854588Z    |
2019-12-27T00:05:02.4854834Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-27T00:05:02.4855289Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-27T00:05:02.4855860Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:14:19
2019-12-27T00:05:02.4856773Z    |
2019-12-27T00:05:02.4856773Z    |
2019-12-27T00:05:02.4856844Z LL |     [0_usize; 33] == [1_usize; 33]
2019-12-27T00:05:02.4857161Z    |     ------------- ^^ ------------- [usize; 33]
2019-12-27T00:05:02.4857275Z    |     [usize; 33]
2019-12-27T00:05:02.4857316Z    |
2019-12-27T00:05:02.4857378Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2019-12-27T00:05:02.4857411Z 
2019-12-27T00:05:02.4857411Z 
2019-12-27T00:05:02.4857485Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-12-27T00:05:02.4857766Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:19:19
2019-12-27T00:05:02.4857816Z    |
2019-12-27T00:05:02.4857879Z LL |     [0_usize; 33] < [1_usize; 33]
2019-12-27T00:05:02.4858103Z    |     ------------- ^ ------------- [usize; 33]
2019-12-27T00:05:02.4858213Z    |     [usize; 33]
2019-12-27T00:05:02.4858254Z    |
2019-12-27T00:05:02.4858254Z    |
2019-12-27T00:05:02.4858304Z    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2019-12-27T00:05:02.4858338Z 
2019-12-27T00:05:02.4858406Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-12-27T00:05:02.4858681Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:24:14
2019-12-27T00:05:02.4858739Z    |
2019-12-27T00:05:02.4858805Z LL |     for _ in &[0_usize; 33] {
2019-12-27T00:05:02.4858866Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2019-12-27T00:05:02.4859330Z    = help: the following implementations were found:
2019-12-27T00:05:02.4859330Z    = help: the following implementations were found:
2019-12-27T00:05:02.4859606Z              <&'a [T; _] as std::iter::IntoIterator>
2019-12-27T00:05:02.4859830Z              <&'a [T] as std::iter::IntoIterator>
2019-12-27T00:05:02.4860080Z              <&'a mut [T; _] as std::iter::IntoIterator>
2019-12-27T00:05:02.4860310Z              <&'a mut [T] as std::iter::IntoIterator>
2019-12-27T00:05:02.4860394Z 
2019-12-27T00:05:02.4860456Z error: aborting due to 5 previous errors
2019-12-27T00:05:02.4860485Z 
2019-12-27T00:05:02.4860529Z Some errors have detailed explanations: E0277, E0369.
---
2019-12-27T00:05:02.4862409Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-27T00:05:02.4862486Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-27T00:05:02.4862520Z 
2019-12-27T00:05:02.4862545Z 
2019-12-27T00:05:02.4864314Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-27T00:05:02.4864743Z 
2019-12-27T00:05:02.4864772Z 
2019-12-27T00:05:02.4867965Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-27T00:05:02.4868038Z Build completed unsuccessfully in 1:07:42
2019-12-27T00:05:02.4868038Z Build completed unsuccessfully in 1:07:42
2019-12-27T00:05:02.4905930Z == clock drift check ==
2019-12-27T00:05:02.4922803Z   local time: Fri Dec 27 00:05:02 UTC 2019
2019-12-27T00:05:02.6557093Z   network time: Fri, 27 Dec 2019 00:05:02 GMT
2019-12-27T00:05:02.6564415Z == end clock drift check ==
2019-12-27T00:05:03.8093606Z 
2019-12-27T00:05:03.8241560Z ##[error]Bash exited with code '1'.
2019-12-27T00:05:03.8288174Z ##[section]Starting: Checkout
2019-12-27T00:05:03.8290895Z ==============================================================================
2019-12-27T00:05:03.8290970Z Task         : Get sources
2019-12-27T00:05:03.8291019Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
