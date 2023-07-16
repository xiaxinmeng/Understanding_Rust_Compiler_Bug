plain
2019-12-05T01:32:58.3616945Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-05T01:32:58.8968607Z ##[command]git config gc.auto 0
2019-12-05T01:32:58.8973403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-05T01:32:58.8978828Z ##[command]git config --get-all http.proxy
2019-12-05T01:32:58.8984042Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67042/merge:refs/remotes/pull/67042/merge
---
2019-12-05T02:33:54.3393372Z .................................................................................................... 1600/9325
2019-12-05T02:33:59.0361821Z .................................................................................................... 1700/9325
2019-12-05T02:34:11.6170099Z ..........................................i......................................................... 1800/9325
2019-12-05T02:34:19.5142272Z .................................................................................................... 1900/9325
2019-12-05T02:34:33.7234478Z ...........................iiiii.................................................................... 2000/9325
2019-12-05T02:34:43.9305250Z .................................................................................................... 2200/9325
2019-12-05T02:34:46.5261229Z .................................................................................................... 2300/9325
2019-12-05T02:34:51.1265820Z .................................................................................................... 2400/9325
2019-12-05T02:35:13.0140976Z .................................................................................................... 2500/9325
---
2019-12-05T02:37:52.6165837Z ............................i...............i....................................................... 4800/9325
2019-12-05T02:38:02.8178831Z .................................................................................................... 4900/9325
2019-12-05T02:38:09.2385757Z .................................................................................................... 5000/9325
2019-12-05T02:38:16.9670967Z ..................................F................................................................. 5100/9325
2019-12-05T02:38:24.6369256Z ....................................ii.ii...........i............................................... 5200/9325
2019-12-05T02:38:34.2678142Z .................................................................................................... 5400/9325
2019-12-05T02:38:44.1377537Z .................................................................................................... 5500/9325
2019-12-05T02:38:51.7589682Z ..................i................................................................................. 5600/9325
2019-12-05T02:38:58.0387443Z .................................................................................................... 5700/9325
2019-12-05T02:38:58.0387443Z .................................................................................................... 5700/9325
2019-12-05T02:39:10.2565068Z .................................................................................................... 5800/9325
2019-12-05T02:39:22.3760358Z ....ii...i..ii...........i.......................................................................... 5900/9325
2019-12-05T02:39:40.5397097Z .................................................................................................... 6100/9325
2019-12-05T02:39:47.9037882Z .................................................................................................... 6200/9325
2019-12-05T02:39:47.9037882Z .................................................................................................... 6200/9325
2019-12-05T02:40:06.1449157Z ...........................i..ii.................................................................... 6300/9325
2019-12-05T02:40:26.0532251Z ...................................................................................................i 6500/9325
2019-12-05T02:40:28.2743438Z .................................................................................................... 6600/9325
2019-12-05T02:40:30.5052457Z ..........................................................................................i......... 6700/9325
2019-12-05T02:40:33.2649621Z .................................................................................................... 6800/9325
---
2019-12-05T02:42:12.7976142Z .................................................................................................... 7400/9325
2019-12-05T02:42:18.2928786Z .................................................................................................... 7500/9325
2019-12-05T02:42:25.0766411Z .................................................................................................... 7600/9325
2019-12-05T02:42:36.3595451Z .................................................................................................... 7700/9325
2019-12-05T02:42:42.9505946Z ...iiii............................................................................................. 7800/9325
2019-12-05T02:42:57.7406046Z .................................................................................................... 8000/9325
2019-12-05T02:43:09.5381506Z .................................................................................................... 8100/9325
2019-12-05T02:43:21.8781317Z .................................................................................................... 8200/9325
2019-12-05T02:43:28.2990663Z .................................................................................................... 8300/9325
---
2019-12-05T02:45:24.9246150Z ---- [ui] ui/const-generics/array-impls/core-traits-no-impls-length-33.rs stdout ----
2019-12-05T02:45:24.9246359Z diff of stderr:
2019-12-05T02:45:24.9246398Z 
2019-12-05T02:45:24.9246463Z 15    |
2019-12-05T02:45:24.9246524Z 16    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-05T02:45:24.9246692Z + error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-05T02:45:24.9247024Z +   --> $DIR/core-traits-no-impls-length-33.rs:8:19
2019-12-05T02:45:24.9247080Z +    |
2019-12-05T02:45:24.9247128Z + LL |     let mut set = HashSet::new();
2019-12-05T02:45:24.9247128Z + LL |     let mut set = HashSet::new();
2019-12-05T02:45:24.9247207Z +    |                   ^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-05T02:45:24.9247263Z +    |
2019-12-05T02:45:24.9247318Z +    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-05T02:45:24.9247394Z +    = note: required by `std::collections::HashSet::<T>::new`
2019-12-05T02:45:24.9247496Z 18 error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-05T02:45:24.9247791Z 19   --> $DIR/core-traits-no-impls-length-33.rs:14:19
2019-12-05T02:45:24.9247856Z 20    |
2019-12-05T02:45:24.9247886Z 
2019-12-05T02:45:24.9247886Z 
2019-12-05T02:45:24.9248342Z 48              <&'a mut [T] as std::iter::IntoIterator>
2019-12-05T02:45:24.9248482Z 50 
2019-12-05T02:45:24.9248765Z - error: aborting due to 5 previous errors
2019-12-05T02:45:24.9248838Z + error: aborting due to 6 previous errors
2019-12-05T02:45:24.9248884Z 52 
2019-12-05T02:45:24.9248884Z 52 
2019-12-05T02:45:24.9248937Z 53 Some errors have detailed explanations: E0277, E0369.
2019-12-05T02:45:24.9249236Z 54 For more information about an error, try `rustc --explain E0277`.
2019-12-05T02:45:24.9249276Z 
2019-12-05T02:45:24.9249305Z 
2019-12-05T02:45:24.9249355Z The actual stderr differed from the expected stderr.
2019-12-05T02:45:24.9249776Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2019-12-05T02:45:24.9250070Z To update references, rerun the tests and pass the `--bless` flag
2019-12-05T02:45:24.9250597Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2019-12-05T02:45:24.9250709Z error: 1 errors occurred comparing output.
2019-12-05T02:45:24.9250757Z status: exit code: 1
2019-12-05T02:45:24.9250757Z status: exit code: 1
2019-12-05T02:45:24.9251730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary" "-A" "unused"
2019-12-05T02:45:24.9252123Z ------------------------------------------
2019-12-05T02:45:24.9252162Z 
2019-12-05T02:45:24.9252408Z ------------------------------------------
2019-12-05T02:45:24.9252475Z stderr:
2019-12-05T02:45:24.9252475Z stderr:
2019-12-05T02:45:24.9252715Z ------------------------------------------
2019-12-05T02:45:24.9252774Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-05T02:45:24.9253101Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2019-12-05T02:45:24.9253211Z    |
2019-12-05T02:45:24.9253258Z LL |     println!("{:?}", [0_usize; 33]);
2019-12-05T02:45:24.9253335Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-05T02:45:24.9253447Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2019-12-05T02:45:24.9253520Z    = note: required by `std::fmt::Debug::fmt`
2019-12-05T02:45:24.9253561Z 
2019-12-05T02:45:24.9253619Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-05T02:45:24.9253619Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-05T02:45:24.9254254Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:9:16
2019-12-05T02:45:24.9254343Z    |
2019-12-05T02:45:24.9254390Z LL |     set.insert([0_usize; 33]);
2019-12-05T02:45:24.9254452Z    |                ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-05T02:45:24.9254524Z    |
2019-12-05T02:45:24.9254580Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-05T02:45:24.9254685Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-05T02:45:24.9255054Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:8:19
2019-12-05T02:45:24.9255109Z    |
2019-12-05T02:45:24.9255172Z LL |     let mut set = HashSet::new();
2019-12-05T02:45:24.9255172Z LL |     let mut set = HashSet::new();
2019-12-05T02:45:24.9255360Z    |                   ^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-05T02:45:24.9255423Z    |
2019-12-05T02:45:24.9255494Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-05T02:45:24.9255555Z    = note: required by `std::collections::HashSet::<T>::new`
2019-12-05T02:45:24.9255643Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-05T02:45:24.9256000Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:14:19
2019-12-05T02:45:24.9256056Z    |
2019-12-05T02:45:24.9256056Z    |
2019-12-05T02:45:24.9256104Z LL |     [0_usize; 33] == [1_usize; 33]
2019-12-05T02:45:24.9256374Z    |     ------------- ^^ ------------- [usize; 33]
2019-12-05T02:45:24.9256475Z    |     [usize; 33]
2019-12-05T02:45:24.9256535Z    |
2019-12-05T02:45:24.9256590Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2019-12-05T02:45:24.9256720Z 
2019-12-05T02:45:24.9256720Z 
2019-12-05T02:45:24.9256773Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-12-05T02:45:24.9257123Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:19:19
2019-12-05T02:45:24.9257178Z    |
2019-12-05T02:45:24.9257225Z LL |     [0_usize; 33] < [1_usize; 33]
2019-12-05T02:45:24.9257495Z    |     ------------- ^ ------------- [usize; 33]
2019-12-05T02:45:24.9257593Z    |     [usize; 33]
2019-12-05T02:45:24.9257637Z    |
2019-12-05T02:45:24.9257637Z    |
2019-12-05T02:45:24.9257707Z    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2019-12-05T02:45:24.9257744Z 
2019-12-05T02:45:24.9257797Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-12-05T02:45:24.9258123Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:24:14
2019-12-05T02:45:24.9258188Z    |
2019-12-05T02:45:24.9258242Z LL |     for _ in &[0_usize; 33] {
2019-12-05T02:45:24.9258317Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2019-12-05T02:45:24.9258417Z    = help: the following implementations were found:
2019-12-05T02:45:24.9258417Z    = help: the following implementations were found:
2019-12-05T02:45:24.9258696Z              <&'a [T; _] as std::iter::IntoIterator>
2019-12-05T02:45:24.9258954Z              <&'a [T] as std::iter::IntoIterator>
2019-12-05T02:45:24.9259215Z              <&'a mut [T; _] as std::iter::IntoIterator>
2019-12-05T02:45:24.9259490Z              <&'a mut [T] as std::iter::IntoIterator>
2019-12-05T02:45:24.9259599Z 
2019-12-05T02:45:24.9259646Z error: aborting due to 6 previous errors
2019-12-05T02:45:24.9259694Z 
2019-12-05T02:45:24.9259780Z Some errors have detailed explanations: E0277, E0369.
---
2019-12-05T02:45:24.9260433Z 
2019-12-05T02:45:24.9260712Z ---- [ui] ui/issues/issue-66923-show-error-for-correct-call.rs stdout ----
2019-12-05T02:45:24.9260784Z diff of stderr:
2019-12-05T02:45:24.9260815Z 
2019-12-05T02:45:24.9261148Z - error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-05T02:45:24.9261219Z + error[E0277]: a value of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-05T02:45:24.9261514Z 2   --> $DIR/issue-66923-show-error-for-correct-call.rs:8:39
2019-12-05T02:45:24.9261576Z 3    |
2019-12-05T02:45:24.9261631Z 4 LL |     let x2: Vec<f64> = x1.into_iter().collect();
2019-12-05T02:45:24.9261686Z 
2019-12-05T02:45:24.9268948Z -    |                                       ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-05T02:45:24.9269215Z +    |                                       ^^^^^^^ value of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-05T02:45:24.9269319Z 6    |
2019-12-05T02:45:24.9269378Z 7    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-05T02:45:24.9269478Z 
2019-12-05T02:45:24.9269478Z 
2019-12-05T02:45:24.9269878Z - error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-05T02:45:24.9269950Z + error[E0277]: a value of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-05T02:45:24.9270485Z 10   --> $DIR/issue-66923-show-error-for-correct-call.rs:12:29
2019-12-05T02:45:24.9270545Z 11    |
2019-12-05T02:45:24.9270596Z 12 LL |     let x3 = x1.into_iter().collect::<Vec<f64>>();
2019-12-05T02:45:24.9270631Z 
2019-12-05T02:45:24.9271151Z -    |                             ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-05T02:45:24.9271227Z +    |                             ^^^^^^^ value of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-05T02:45:24.9271284Z 14    |
2019-12-05T02:45:24.9271412Z 15    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-05T02:45:24.9271496Z 
2019-12-05T02:45:24.9271525Z 
2019-12-05T02:45:24.9271590Z The actual stderr differed from the expected stderr.
2019-12-05T02:45:24.9271590Z The actual stderr differed from the expected stderr.
2019-12-05T02:45:24.9272171Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call/issue-66923-show-error-for-correct-call.stderr
2019-12-05T02:45:24.9272482Z To update references, rerun the tests and pass the `--bless` flag
2019-12-05T02:45:24.9272824Z To only update this specific test, also pass `--test-args issues/issue-66923-show-error-for-correct-call.rs`
2019-12-05T02:45:24.9272932Z error: 1 errors occurred comparing output.
2019-12-05T02:45:24.9272998Z status: exit code: 1
2019-12-05T02:45:24.9272998Z status: exit code: 1
2019-12-05T02:45:24.9273921Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call/auxiliary" "-A" "unused"
2019-12-05T02:45:24.9274621Z ------------------------------------------
2019-12-05T02:45:24.9274661Z 
2019-12-05T02:45:24.9274940Z ------------------------------------------
2019-12-05T02:45:24.9274999Z stderr:
2019-12-05T02:45:24.9274999Z stderr:
2019-12-05T02:45:24.9275239Z ------------------------------------------
2019-12-05T02:45:24.9275316Z error[E0277]: a value of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-05T02:45:24.9275620Z   --> /checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs:8:39
2019-12-05T02:45:24.9275681Z    |
2019-12-05T02:45:24.9275751Z LL |     let x2: Vec<f64> = x1.into_iter().collect();
2019-12-05T02:45:24.9275821Z    |                                       ^^^^^^^ value of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-05T02:45:24.9275879Z    |
2019-12-05T02:45:24.9275953Z    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-05T02:45:24.9275994Z 
2019-12-05T02:45:24.9276053Z error[E0277]: a value of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-05T02:45:24.9276546Z   --> /checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs:12:29
2019-12-05T02:45:24.9276635Z    |
2019-12-05T02:45:24.9276688Z LL |     let x3 = x1.into_iter().collect::<Vec<f64>>();
2019-12-05T02:45:24.9276754Z    |                             ^^^^^^^ value of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-05T02:45:24.9276826Z    |
2019-12-05T02:45:24.9276884Z    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-05T02:45:24.9276987Z error: aborting due to 2 previous errors
2019-12-05T02:45:24.9277022Z 
2019-12-05T02:45:24.9277353Z For more information about this error, try `rustc --explain E0277`.
2019-12-05T02:45:24.9277394Z 
---
2019-12-05T02:45:24.9279369Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-05T02:45:24.9279436Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-05T02:45:24.9297636Z 
2019-12-05T02:45:24.9297741Z 
2019-12-05T02:45:24.9306094Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-05T02:45:24.9319852Z 
2019-12-05T02:45:24.9319897Z 
2019-12-05T02:45:24.9343292Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-05T02:45:24.9343588Z Build completed unsuccessfully in 1:06:07
2019-12-05T02:45:24.9343588Z Build completed unsuccessfully in 1:06:07
2019-12-05T02:45:24.9362227Z == clock drift check ==
2019-12-05T02:45:24.9385348Z   local time: Thu Dec  5 02:45:24 UTC 2019
2019-12-05T02:45:25.2390789Z   network time: Thu, 05 Dec 2019 02:45:25 GMT
2019-12-05T02:45:25.2397073Z == end clock drift check ==
2019-12-05T02:45:26.0105949Z 
2019-12-05T02:45:26.0215551Z ##[error]Bash exited with code '1'.
2019-12-05T02:45:26.0254064Z ##[section]Starting: Checkout
2019-12-05T02:45:26.0256280Z ==============================================================================
2019-12-05T02:45:26.0256355Z Task         : Get sources
2019-12-05T02:45:26.0256404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
