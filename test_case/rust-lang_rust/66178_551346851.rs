plain
2019-11-08T00:33:56.5184175Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T00:33:56.5372782Z ##[command]git config gc.auto 0
2019-11-08T00:33:56.5450065Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T00:33:56.5507542Z ##[command]git config --get-all http.proxy
2019-11-08T00:33:56.5656432Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66178/merge:refs/remotes/pull/66178/merge
---
2019-11-08T01:34:16.2548696Z .................................................................................................... 1600/9290
2019-11-08T01:34:21.8367590Z .................................................................................................... 1700/9290
2019-11-08T01:34:34.2922892Z ................................................................i................................... 1800/9290
2019-11-08T01:34:42.0332986Z .................................................................................................... 1900/9290
2019-11-08T01:34:56.7850254Z ................................................iiiii............................................... 2000/9290
2019-11-08T01:35:07.5998705Z .................................................................................................... 2200/9290
2019-11-08T01:35:10.1793354Z .................................................................................................... 2300/9290
2019-11-08T01:35:13.9360862Z .................................................................................................... 2400/9290
2019-11-08T01:35:37.3197852Z .................................................................................................... 2500/9290
---
2019-11-08T01:38:28.5757985Z ..............................................i...............i..................................... 4800/9290
2019-11-08T01:38:37.7977135Z .................................................................................................... 4900/9290
2019-11-08T01:38:46.5419908Z .................................................................................................... 5000/9290
2019-11-08T01:38:53.4209922Z .................................................................................................... 5100/9290
2019-11-08T01:39:03.5469387Z ................................................ii.ii...........i................................... 5200/9290
2019-11-08T01:39:13.4662333Z .................................................................................................... 5400/9290
2019-11-08T01:39:24.6853654Z .................................................................................................... 5500/9290
2019-11-08T01:39:32.1508127Z ..............................i..................................................................... 5600/9290
2019-11-08T01:39:38.7526366Z .................................................................................................... 5700/9290
2019-11-08T01:39:38.7526366Z .................................................................................................... 5700/9290
2019-11-08T01:39:51.0010498Z .................................................................................................... 5800/9290
2019-11-08T01:40:02.8401880Z ...............ii...i..ii...........i............................................................... 5900/9290
2019-11-08T01:40:23.2372567Z .................................................................................................... 6100/9290
2019-11-08T01:40:31.8409192Z .................................................................................................... 6200/9290
2019-11-08T01:40:31.8409192Z .................................................................................................... 6200/9290
2019-11-08T01:40:46.4860249Z ..................................i..ii............................................................. 6300/9290
2019-11-08T01:41:07.8321481Z .................................................................................................... 6500/9290
2019-11-08T01:41:10.0464963Z ...i................................................................................................ 6600/9290
2019-11-08T01:41:12.3932599Z ......................................................................................i............. 6700/9290
2019-11-08T01:41:15.1158773Z .................................................................................................... 6800/9290
---
2019-11-08T01:46:08.9746532Z ---- [ui] ui/type-alias-impl-trait/issue-58887.rs stdout ----
2019-11-08T01:46:08.9746579Z 
2019-11-08T01:46:08.9751218Z error: ui test compiled successfully!
2019-11-08T01:46:08.9751372Z status: exit code: 0
2019-11-08T01:46:08.9752880Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-58887.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-58887" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-58887/auxiliary" "-A" "unused"
2019-11-08T01:46:08.9755944Z ------------------------------------------
2019-11-08T01:46:08.9756191Z 
2019-11-08T01:46:08.9756749Z ------------------------------------------
2019-11-08T01:46:08.9756870Z stderr:
---
2019-11-08T01:46:08.9758467Z -   --> $DIR/issue-60564.rs:20:49
2019-11-08T01:46:08.9758529Z + error: defining opaque type use does not fully define opaque type
2019-11-08T01:46:08.9758777Z +   --> $DIR/issue-60564.rs:20:5
2019-11-08T01:46:08.9758845Z 3    |
2019-11-08T01:46:08.9759102Z - LL |       fn iter_bits(self, n: u8) -> Self::BitsIter {
2019-11-08T01:46:08.9759369Z -    |  _________________________________________________^
2019-11-08T01:46:08.9759642Z + LL | /     fn iter_bits(self, n: u8) -> Self::BitsIter {
2019-11-08T01:46:08.9759696Z 6 LL | |
2019-11-08T01:46:08.9759745Z 7 LL | |         (0u8..n)
2019-11-08T01:46:08.9759845Z 
2019-11-08T01:46:08.9759874Z 
2019-11-08T01:46:08.9759925Z The actual stderr differed from the expected stderr.
2019-11-08T01:46:08.9760300Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60564/issue-60564.stderr
2019-11-08T01:46:08.9760300Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60564/issue-60564.stderr
2019-11-08T01:46:08.9760572Z To update references, rerun the tests and pass the `--bless` flag
2019-11-08T01:46:08.9760873Z To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-60564.rs`
2019-11-08T01:46:08.9760989Z error: 1 errors occurred comparing output.
2019-11-08T01:46:08.9761041Z status: exit code: 1
2019-11-08T01:46:08.9761041Z status: exit code: 1
2019-11-08T01:46:08.9762334Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-60564.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60564" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60564/auxiliary" "-A" "unused"
2019-11-08T01:46:08.9762710Z ------------------------------------------
2019-11-08T01:46:08.9762747Z 
2019-11-08T01:46:08.9762988Z ------------------------------------------
2019-11-08T01:46:08.9763039Z stderr:
2019-11-08T01:46:08.9763039Z stderr:
2019-11-08T01:46:08.9763289Z ------------------------------------------
2019-11-08T01:46:08.9763506Z error: defining opaque type use does not fully define opaque type
2019-11-08T01:46:08.9763925Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-60564.rs:20:5
2019-11-08T01:46:08.9764005Z    |
2019-11-08T01:46:08.9764262Z LL | /     fn iter_bits(self, n: u8) -> Self::BitsIter {
2019-11-08T01:46:08.9764323Z LL | |     //~^ ERROR type parameter `E` is part of concrete type but not used
2019-11-08T01:46:08.9764395Z LL | |         (0u8..n)
2019-11-08T01:46:08.9764445Z LL | |             .rev()
2019-11-08T01:46:08.9764505Z LL | |             .map(move |shift| ((self >> T::from(shift)) & T::from(1)).try_into().unwrap())
2019-11-08T01:46:08.9764628Z    | |_____^
2019-11-08T01:46:08.9764659Z 
2019-11-08T01:46:08.9764709Z error: could not find defining uses
2019-11-08T01:46:08.9765003Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-60564.rs:8:1
2019-11-08T01:46:08.9765003Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-60564.rs:8:1
2019-11-08T01:46:08.9765058Z    |
2019-11-08T01:46:08.9765122Z LL | type IterBitsIter<T, E, I> = impl std::iter::Iterator<Item = I>;
2019-11-08T01:46:08.9765240Z 
2019-11-08T01:46:08.9765288Z error: aborting due to 2 previous errors
2019-11-08T01:46:08.9765320Z 
2019-11-08T01:46:08.9765365Z 
---
2019-11-08T01:46:08.9805174Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-08T01:46:08.9805321Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-08T01:46:08.9822893Z 
2019-11-08T01:46:08.9824559Z 
2019-11-08T01:46:08.9828894Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-08T01:46:08.9829227Z 
2019-11-08T01:46:08.9829260Z 
2019-11-08T01:46:08.9831267Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-08T01:46:08.9832998Z Build completed unsuccessfully in 1:05:43
2019-11-08T01:46:08.9832998Z Build completed unsuccessfully in 1:05:43
2019-11-08T01:46:08.9884425Z == clock drift check ==
2019-11-08T01:46:08.9901320Z   local time: Fri Nov  8 01:46:08 UTC 2019
2019-11-08T01:46:09.0760547Z   network time: Fri, 08 Nov 2019 01:46:09 GMT
2019-11-08T01:46:09.0760669Z == end clock drift check ==
2019-11-08T01:46:09.8255873Z 
2019-11-08T01:46:09.8387574Z ##[error]Bash exited with code '1'.
2019-11-08T01:46:09.8433694Z ##[section]Starting: Checkout
2019-11-08T01:46:09.8435645Z ==============================================================================
2019-11-08T01:46:09.8435697Z Task         : Get sources
2019-11-08T01:46:09.8435756Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
