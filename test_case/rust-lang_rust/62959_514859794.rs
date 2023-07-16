plain
2019-07-25T00:29:20.4488943Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T00:29:20.4669657Z ##[command]git config gc.auto 0
2019-07-25T00:29:20.4758416Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T00:29:21.0275565Z ##[command]git config --get-all http.proxy
2019-07-25T00:29:21.0284231Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62959/merge:refs/remotes/pull/62959/merge
---
2019-07-25T00:29:55.0317626Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T00:29:55.0317669Z 
2019-07-25T00:29:55.0317942Z   git checkout -b <new-branch-name>
2019-07-25T00:29:55.0317979Z 
2019-07-25T00:29:55.0318035Z HEAD is now at 1e59da182 Merge ac96d7734cd80556616f3c0d8deec4ee586caef6 into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-25T00:29:55.0467828Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T00:29:55.0471870Z ==============================================================================
2019-07-25T00:29:55.0471938Z Task         : Bash
2019-07-25T00:29:55.0471990Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T01:31:37.3203333Z .................................................................................................... 200/5851
2019-07-25T01:31:41.7257933Z .................................................................................................... 300/5851
2019-07-25T01:31:45.7507633Z .................................................................................................... 400/5851
2019-07-25T01:31:49.8569186Z .................................................................................................... 500/5851
2019-07-25T01:31:53.9387336Z ........................................................................i........................... 600/5851
2019-07-25T01:32:03.2983968Z ...........................................F........................................................ 800/5851
2019-07-25T01:32:09.1665546Z .................................................................................................... 900/5851
2019-07-25T01:32:14.4177823Z ...................................................................................................i 1000/5851
2019-07-25T01:32:14.4177823Z ...................................................................................................i 1000/5851
2019-07-25T01:32:20.2618217Z ...........i........................................................................................ 1100/5851
2019-07-25T01:32:24.4560081Z .............................iiiii.................................................................. 1200/5851
2019-07-25T01:32:30.7476715Z .................................................................................................... 1400/5851
2019-07-25T01:32:33.6326220Z .................................................................................................... 1500/5851
2019-07-25T01:32:37.6983915Z .................................................................................................... 1600/5851
2019-07-25T01:32:40.5306699Z .................................................................................................... 1700/5851
2019-07-25T01:32:40.5306699Z .................................................................................................... 1700/5851
2019-07-25T01:32:44.0894927Z .....................................................................i.............................. 1800/5851
2019-07-25T01:32:53.0826993Z .................................................................................................... 2000/5851
2019-07-25T01:32:57.6491878Z .................................................................................................... 2100/5851
2019-07-25T01:33:01.5495637Z .................................................................................................... 2200/5851
2019-07-25T01:33:01.5495637Z .................................................................................................... 2200/5851
2019-07-25T01:33:05.6271391Z .....................................................i.............................................. 2300/5851
2019-07-25T01:33:16.1170253Z .................................................................................................... 2500/5851
2019-07-25T01:33:20.3911639Z .................................................................................................... 2600/5851
2019-07-25T01:33:25.7524092Z .................................................................................................... 2700/5851
2019-07-25T01:33:29.8749409Z .................................................................................................... 2800/5851
2019-07-25T01:33:29.8749409Z .................................................................................................... 2800/5851
2019-07-25T01:33:34.5740797Z .................................................................................................... 2900/5851
2019-07-25T01:33:40.0203523Z .................................................................................................... 3000/5851
2019-07-25T01:33:44.6333642Z .................................................................................................... 3100/5851
2019-07-25T01:33:50.0971821Z .................................................................................................... 3200/5851
2019-07-25T01:33:53.8461007Z .....................................................F.F............................................ 3300/5851
2019-07-25T01:34:03.0091293Z .................................................................................................... 3500/5851
2019-07-25T01:34:03.0091293Z .................................................................................................... 3500/5851
2019-07-25T01:34:06.8767639Z ....................i............................................................................... 3600/5851
2019-07-25T01:34:11.1701352Z ..............................................................................................ii...i 3700/5851
2019-07-25T01:34:15.1418514Z ..ii................................................................................................ 3800/5851
2019-07-25T01:34:24.2322574Z .................................................................................................... 4000/5851
2019-07-25T01:34:24.2322574Z .................................................................................................... 4000/5851
2019-07-25T01:34:28.3440545Z ........ii.......................................................................................... 4100/5851
2019-07-25T01:34:30.5441262Z .............................i...................................................................... 4200/5851
2019-07-25T01:34:32.7192532Z ................................................................................................i... 4300/5851
2019-07-25T01:34:39.6440542Z .................................................................................................... 4500/5851
2019-07-25T01:34:58.2847968Z .................................................................................................... 4600/5851
2019-07-25T01:35:02.0873676Z .................................................................................................... 4700/5851
2019-07-25T01:35:05.9840980Z .................................................................................................... 4800/5851
---
2019-07-25T01:35:41.6602113Z .................................................................................................... 5400/5851
2019-07-25T01:35:46.0620890Z .................................................................................................... 5500/5851
2019-07-25T01:35:50.1115179Z .................................................................................................... 5600/5851
2019-07-25T01:35:53.2952051Z .................................................................................................... 5700/5851
2019-07-25T01:35:56.4844043Z ...........................................................................................i........ 5800/5851
2019-07-25T01:35:58.3999511Z failures:
2019-07-25T01:35:58.4048260Z 
2019-07-25T01:35:58.4048964Z ---- [ui] ui/const-generics/array-impls/core-traits-no-impls-length-33.rs stdout ----
2019-07-25T01:35:58.4049039Z diff of stderr:
2019-07-25T01:35:58.4049039Z diff of stderr:
2019-07-25T01:35:58.4049068Z 
2019-07-25T01:35:58.4049320Z 46              <&'a [T] as std::iter::IntoIterator>
2019-07-25T01:35:58.4049532Z 47              <&'a mut [T; _] as std::iter::IntoIterator>
2019-07-25T01:35:58.4049738Z 48              <&'a mut [T] as std::iter::IntoIterator>
2019-07-25T01:35:58.4049799Z +              <[T; _] as std::iter::IntoIterator>
2019-07-25T01:35:58.4049881Z 50 
2019-07-25T01:35:58.4049945Z 51 error: aborting due to 5 previous errors
2019-07-25T01:35:58.4049972Z 
2019-07-25T01:35:58.4049994Z 
2019-07-25T01:35:58.4049994Z 
2019-07-25T01:35:58.4050032Z The actual stderr differed from the expected stderr.
2019-07-25T01:35:58.4050360Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2019-07-25T01:35:58.4050586Z To update references, rerun the tests and pass the `--bless` flag
2019-07-25T01:35:58.4050888Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2019-07-25T01:35:58.4050980Z error: 1 errors occurred comparing output.
2019-07-25T01:35:58.4051021Z status: exit code: 1
2019-07-25T01:35:58.4051021Z status: exit code: 1
2019-07-25T01:35:58.4052058Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary" "-A" "unused"
2019-07-25T01:35:58.4052450Z ------------------------------------------
2019-07-25T01:35:58.4052484Z 
2019-07-25T01:35:58.4052691Z ------------------------------------------
2019-07-25T01:35:58.4052750Z stderr:
2019-07-25T01:35:58.4052750Z stderr:
2019-07-25T01:35:58.4052952Z ------------------------------------------
2019-07-25T01:35:58.4053003Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-07-25T01:35:58.4053275Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2019-07-25T01:35:58.4053334Z    |
2019-07-25T01:35:58.4053374Z LL |     println!("{:?}", [0_usize; 33]);
2019-07-25T01:35:58.4053425Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-07-25T01:35:58.4053489Z    |
2019-07-25T01:35:58.4053537Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2019-07-25T01:35:58.4053717Z    = note: required by `std::fmt::Debug::fmt`
2019-07-25T01:35:58.4053745Z 
2019-07-25T01:35:58.4053790Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-07-25T01:35:58.4054084Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:9:9
2019-07-25T01:35:58.4054148Z    |
2019-07-25T01:35:58.4054187Z LL |     set.insert([0_usize; 33]);
2019-07-25T01:35:58.4054237Z    |         ^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-07-25T01:35:58.4054302Z    |
2019-07-25T01:35:58.4054356Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-07-25T01:35:58.4054387Z 
2019-07-25T01:35:58.4054431Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-07-25T01:35:58.4054830Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:14:19
2019-07-25T01:35:58.4054873Z    |
2019-07-25T01:35:58.4054910Z LL |     [0_usize; 33] == [1_usize; 33]
2019-07-25T01:35:58.4055136Z    |     ------------- ^^ ------------- [usize; 33]
2019-07-25T01:35:58.4055214Z    |     [usize; 33]
2019-07-25T01:35:58.4055265Z    |
2019-07-25T01:35:58.4055265Z    |
2019-07-25T01:35:58.4055308Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2019-07-25T01:35:58.4055377Z 
2019-07-25T01:35:58.4055419Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-07-25T01:35:58.4055870Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:19:19
2019-07-25T01:35:58.4055928Z    |
2019-07-25T01:35:58.4055970Z LL |     [0_usize; 33] < [1_usize; 33]
2019-07-25T01:35:58.4056218Z    |     ------------- ^ ------------- [usize; 33]
2019-07-25T01:35:58.4056307Z    |     [usize; 33]
2019-07-25T01:35:58.4056363Z    |
2019-07-25T01:35:58.4056363Z    |
2019-07-25T01:35:58.4056411Z    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2019-07-25T01:35:58.4056451Z 
2019-07-25T01:35:58.4056500Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-07-25T01:35:58.4056798Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:24:14
2019-07-25T01:35:58.4056846Z    |
2019-07-25T01:35:58.4056888Z LL |     for _ in &[0_usize; 33] {
2019-07-25T01:35:58.4056959Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2019-07-25T01:35:58.4057139Z    = help: the following implementations were found:
2019-07-25T01:35:58.4057139Z    = help: the following implementations were found:
2019-07-25T01:35:58.4057431Z              <&'a [T; _] as std::iter::IntoIterator>
2019-07-25T01:35:58.4057661Z              <&'a [T] as std::iter::IntoIterator>
2019-07-25T01:35:58.4057894Z              <&'a mut [T; _] as std::iter::IntoIterator>
2019-07-25T01:35:58.4058143Z              <&'a mut [T] as std::iter::IntoIterator>
2019-07-25T01:35:58.4058194Z              <[T; _] as std::iter::IntoIterator>
2019-07-25T01:35:58.4058284Z 
2019-07-25T01:35:58.4058344Z error: aborting due to 5 previous errors
2019-07-25T01:35:58.4058372Z 
2019-07-25T01:35:58.4058416Z Some errors have detailed explanations: E0277, E0369.
---
2019-07-25T01:35:58.4059285Z ---- [ui] ui/iterators/array.rs stdout ----
2019-07-25T01:35:58.4059313Z 
2019-07-25T01:35:58.4059351Z error: ui test compiled successfully!
2019-07-25T01:35:58.4059388Z status: exit code: 0
2019-07-25T01:35:58.4060007Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array/auxiliary" "-A" "unused"
2019-07-25T01:35:58.4062727Z ------------------------------------------
2019-07-25T01:35:58.4062760Z 
2019-07-25T01:35:58.4062955Z ------------------------------------------
2019-07-25T01:35:58.4063011Z stderr:
---
2019-07-25T01:35:58.4063700Z ---- [ui] ui/iterators/array-of-ranges.rs stdout ----
2019-07-25T01:35:58.4063727Z 
2019-07-25T01:35:58.4063781Z error: ui test compiled successfully!
2019-07-25T01:35:58.4063827Z status: exit code: 0
2019-07-25T01:35:58.4064467Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/array-of-ranges.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array-of-ranges" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array-of-ranges/auxiliary" "-A" "unused"
2019-07-25T01:35:58.4064759Z ------------------------------------------
2019-07-25T01:35:58.4064805Z 
2019-07-25T01:35:58.4064997Z ------------------------------------------
2019-07-25T01:35:58.4065036Z stderr:
---
2019-07-25T01:35:58.4068202Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-25T01:35:58.4068291Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T01:35:58.4074224Z 
2019-07-25T01:35:58.4074339Z 
2019-07-25T01:35:58.4079725Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-25T01:35:58.4080153Z 
2019-07-25T01:35:58.4080182Z 
2019-07-25T01:35:58.4091786Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-25T01:35:58.4091877Z Build completed unsuccessfully in 0:59:10
2019-07-25T01:35:58.4091877Z Build completed unsuccessfully in 0:59:10
2019-07-25T01:35:59.3292957Z ##[error]Bash exited with code '1'.
2019-07-25T01:35:59.3334688Z ##[section]Starting: Checkout
2019-07-25T01:35:59.3337283Z ==============================================================================
2019-07-25T01:35:59.3337340Z Task         : Get sources
2019-07-25T01:35:59.3337404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
