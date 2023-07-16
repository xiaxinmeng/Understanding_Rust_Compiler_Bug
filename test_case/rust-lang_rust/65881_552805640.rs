plain
2019-11-12T08:07:29.7960447Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T08:07:29.8182361Z ##[command]git config gc.auto 0
2019-11-12T08:07:29.8237001Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T08:07:29.8297381Z ##[command]git config --get-all http.proxy
2019-11-12T08:07:29.8444612Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65881/merge:refs/remotes/pull/65881/merge
---
2019-11-12T09:07:14.1072415Z .........................................................................................F.F........ 1400/9229
2019-11-12T09:07:20.9893001Z .................................................................................................... 1500/9229
2019-11-12T09:07:27.5469445Z .................................................................................................... 1600/9229
2019-11-12T09:07:37.6543118Z .................................................................................................... 1700/9229
2019-11-12T09:07:46.2040307Z .i.................................................................................................. 1800/9229
2019-11-12T09:07:53.2010773Z .....................................................................................iiiii.......... 1900/9229
2019-11-12T09:08:15.3086204Z .................................................................................................... 2100/9229
2019-11-12T09:08:17.8065177Z .................................................................................................... 2200/9229
2019-11-12T09:08:20.4888342Z .................................................................................................... 2300/9229
2019-11-12T09:08:31.6315664Z .................................................................................................... 2400/9229
---
2019-11-12T09:11:31.6833541Z .................................................................................i...............i.. 4700/9229
2019-11-12T09:11:39.0659369Z .................................................................................................... 4800/9229
2019-11-12T09:11:48.4119486Z .................................................................................................... 4900/9229
2019-11-12T09:11:53.7937784Z .................................................................................................... 5000/9229
2019-11-12T09:12:05.4694043Z ....................................................................................ii.ii........... 5100/9229
2019-11-12T09:12:09.4543066Z i................................................................................................... 5200/9229
2019-11-12T09:12:24.5657383Z .................................................................................................... 5400/9229
2019-11-12T09:12:31.7734138Z ..................................................................i................................. 5500/9229
2019-11-12T09:12:39.5022445Z .................................................................................................... 5600/9229
2019-11-12T09:12:48.0359782Z .................................................................................................... 5700/9229
2019-11-12T09:12:48.0359782Z .................................................................................................... 5700/9229
2019-11-12T09:12:57.1877320Z ...................................................ii...i..ii...........i........................... 5800/9229
2019-11-12T09:13:20.1539921Z .................................................................................................... 6000/9229
2019-11-12T09:13:28.6652715Z .................................................................................................... 6100/9229
2019-11-12T09:13:28.6652715Z .................................................................................................... 6100/9229
2019-11-12T09:13:35.9018817Z ......................................................................i..ii......................... 6200/9229
2019-11-12T09:14:05.3899668Z .................................................................................................... 6400/9229
2019-11-12T09:14:07.4473526Z ......................................i............................................................. 6500/9229
2019-11-12T09:14:09.6422479Z .................................................................................................... 6600/9229
2019-11-12T09:14:11.9510230Z ......................i............................................................................. 6700/9229
---
2019-11-12T09:18:59.1833664Z 
2019-11-12T09:18:59.1834301Z ---- [ui] ui/consts/const-eval/const_panic_libcore.rs stdout ----
2019-11-12T09:18:59.1834391Z diff of stderr:
2019-11-12T09:18:59.1834429Z 
2019-11-12T09:18:59.1834474Z 4 LL | const Z: () = panic!("cheese");
2019-11-12T09:18:59.1834824Z 6    |               |
2019-11-12T09:18:59.1834824Z 6    |               |
2019-11-12T09:18:59.1835140Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore.rs:5:15
2019-11-12T09:18:59.1835462Z +    |               the evaluated program panicked at 'cheese', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1835567Z 9    = note: `#[deny(const_err)]` on by default
2019-11-12T09:18:59.1836125Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1836173Z 
2019-11-12T09:18:59.1836173Z 
2019-11-12T09:18:59.1836219Z 15 LL | const Y: () = unreachable!();
2019-11-12T09:18:59.1836664Z 17    |               |
2019-11-12T09:18:59.1836664Z 17    |               |
2019-11-12T09:18:59.1836938Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore.rs:8:15
2019-11-12T09:18:59.1837239Z +    |               the evaluated program panicked at 'internal error: entered unreachable code', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1837798Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1837872Z 21 
2019-11-12T09:18:59.1837897Z 
2019-11-12T09:18:59.1837935Z 25 LL | const X: () = unimplemented!();
2019-11-12T09:18:59.1837935Z 25 LL | const X: () = unimplemented!();
2019-11-12T09:18:59.1838179Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-11-12T09:18:59.1838239Z 27    |               |
2019-11-12T09:18:59.1838510Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore.rs:11:15
2019-11-12T09:18:59.1838776Z +    |               the evaluated program panicked at 'not yet implemented', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1839278Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1839324Z 31 
2019-11-12T09:18:59.1839363Z 
2019-11-12T09:18:59.1839386Z 
2019-11-12T09:18:59.1839386Z 
2019-11-12T09:18:59.1839423Z The actual stderr differed from the expected stderr.
2019-11-12T09:18:59.1839862Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/const_panic_libcore.stderr
2019-11-12T09:18:59.1840090Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T09:18:59.1840841Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore.rs`
2019-11-12T09:18:59.1840939Z error: 1 errors occurred comparing output.
2019-11-12T09:18:59.1840977Z status: exit code: 1
2019-11-12T09:18:59.1840977Z status: exit code: 1
2019-11-12T09:18:59.1841678Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/auxiliary" "-A" "unused"
2019-11-12T09:18:59.1842617Z ------------------------------------------
2019-11-12T09:18:59.1842678Z 
2019-11-12T09:18:59.1842941Z ------------------------------------------
2019-11-12T09:18:59.1842990Z stderr:
2019-11-12T09:18:59.1842990Z stderr:
2019-11-12T09:18:59.1843219Z ------------------------------------------
2019-11-12T09:18:59.1843290Z error: any use of this value will cause an error
2019-11-12T09:18:59.1843560Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:5:15
2019-11-12T09:18:59.1843614Z    |
2019-11-12T09:18:59.1843680Z LL | const Z: () = panic!("cheese");
2019-11-12T09:18:59.1843964Z    |               |
2019-11-12T09:18:59.1843964Z    |               |
2019-11-12T09:18:59.1844270Z    |               the evaluated program panicked at 'cheese', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1844375Z    = note: `#[deny(const_err)]` on by default
2019-11-12T09:18:59.1844732Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1844787Z 
2019-11-12T09:18:59.1844843Z error: any use of this value will cause an error
2019-11-12T09:18:59.1844843Z error: any use of this value will cause an error
2019-11-12T09:18:59.1845131Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:8:15
2019-11-12T09:18:59.1845182Z    |
2019-11-12T09:18:59.1845227Z LL | const Y: () = unreachable!();
2019-11-12T09:18:59.1845526Z    |               |
2019-11-12T09:18:59.1845526Z    |               |
2019-11-12T09:18:59.1846066Z    |               the evaluated program panicked at 'internal error: entered unreachable code', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1846423Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1846461Z 
2019-11-12T09:18:59.1846507Z error: any use of this value will cause an error
2019-11-12T09:18:59.1847099Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-11-12T09:18:59.1847099Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-11-12T09:18:59.1847148Z    |
2019-11-12T09:18:59.1847185Z LL | const X: () = unimplemented!();
2019-11-12T09:18:59.1847404Z    | --------------^^^^^^^^^^^^^^^^-
2019-11-12T09:18:59.1847445Z    |               |
2019-11-12T09:18:59.1847879Z    |               the evaluated program panicked at 'not yet implemented', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1848231Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1848266Z 
2019-11-12T09:18:59.1848306Z error: aborting due to 3 previous errors
2019-11-12T09:18:59.1848347Z 
2019-11-12T09:18:59.1848347Z 
2019-11-12T09:18:59.1848371Z 
2019-11-12T09:18:59.1848572Z ------------------------------------------
2019-11-12T09:18:59.1848600Z 
2019-11-12T09:18:59.1848623Z 
2019-11-12T09:18:59.1848862Z ---- [ui] ui/consts/const-eval/const_panic_libcore_main.rs stdout ----
2019-11-12T09:18:59.1848906Z diff of stderr:
2019-11-12T09:18:59.1849039Z 
2019-11-12T09:18:59.1849095Z 4 LL | const Z: () = panic!("cheese");
2019-11-12T09:18:59.1849442Z 6    |               |
2019-11-12T09:18:59.1849442Z 6    |               |
2019-11-12T09:18:59.1849720Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore_main.rs:9:15
2019-11-12T09:18:59.1850001Z +    |               the evaluated program panicked at 'cheese', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1850252Z 9    = note: `#[deny(const_err)]` on by default
2019-11-12T09:18:59.1850559Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1850595Z 
2019-11-12T09:18:59.1850595Z 
2019-11-12T09:18:59.1850632Z 15 LL | const Y: () = unreachable!();
2019-11-12T09:18:59.1851037Z 17    |               |
2019-11-12T09:18:59.1851037Z 17    |               |
2019-11-12T09:18:59.1851303Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_main.rs:12:15
2019-11-12T09:18:59.1851607Z +    |               the evaluated program panicked at 'internal error: entered unreachable code', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1851923Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1851985Z 21 
2019-11-12T09:18:59.1852008Z 
2019-11-12T09:18:59.1852044Z 25 LL | const X: () = unimplemented!();
2019-11-12T09:18:59.1852044Z 25 LL | const X: () = unimplemented!();
2019-11-12T09:18:59.1852230Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-11-12T09:18:59.1852286Z 27    |               |
2019-11-12T09:18:59.1852533Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore_main.rs:15:15
2019-11-12T09:18:59.1853227Z +    |               the evaluated program panicked at 'not yet implemented', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1853663Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1853720Z 31 
2019-11-12T09:18:59.1853765Z 
2019-11-12T09:18:59.1853791Z 
2019-11-12T09:18:59.1853791Z 
2019-11-12T09:18:59.1853837Z The actual stderr differed from the expected stderr.
2019-11-12T09:18:59.1854188Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/const_panic_libcore_main.stderr
2019-11-12T09:18:59.1854472Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T09:18:59.1854788Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_main.rs`
2019-11-12T09:18:59.1854895Z error: 1 errors occurred comparing output.
2019-11-12T09:18:59.1854943Z status: exit code: 1
2019-11-12T09:18:59.1854943Z status: exit code: 1
2019-11-12T09:18:59.1855805Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary" "-A" "unused"
2019-11-12T09:18:59.1856184Z ------------------------------------------
2019-11-12T09:18:59.1856453Z 
2019-11-12T09:18:59.1856652Z ------------------------------------------
2019-11-12T09:18:59.1856692Z stderr:
2019-11-12T09:18:59.1856692Z stderr:
2019-11-12T09:18:59.1856895Z ------------------------------------------
2019-11-12T09:18:59.1856939Z error: any use of this value will cause an error
2019-11-12T09:18:59.1857448Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:9:15
2019-11-12T09:18:59.1857494Z    |
2019-11-12T09:18:59.1857617Z LL | const Z: () = panic!("cheese");
2019-11-12T09:18:59.1857872Z    |               |
2019-11-12T09:18:59.1857872Z    |               |
2019-11-12T09:18:59.1858128Z    |               the evaluated program panicked at 'cheese', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1858210Z    = note: `#[deny(const_err)]` on by default
2019-11-12T09:18:59.1858500Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1858538Z 
2019-11-12T09:18:59.1858579Z error: any use of this value will cause an error
2019-11-12T09:18:59.1858579Z error: any use of this value will cause an error
2019-11-12T09:18:59.1858820Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:12:15
2019-11-12T09:18:59.1858862Z    |
2019-11-12T09:18:59.1858898Z LL | const Y: () = unreachable!();
2019-11-12T09:18:59.1859148Z    |               |
2019-11-12T09:18:59.1859148Z    |               |
2019-11-12T09:18:59.1859418Z    |               the evaluated program panicked at 'internal error: entered unreachable code', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1859746Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1859782Z 
2019-11-12T09:18:59.1859822Z error: any use of this value will cause an error
2019-11-12T09:18:59.1860075Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-11-12T09:18:59.1860075Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-11-12T09:18:59.1860119Z    |
2019-11-12T09:18:59.1860156Z LL | const X: () = unimplemented!();
2019-11-12T09:18:59.1860369Z    | --------------^^^^^^^^^^^^^^^^-
2019-11-12T09:18:59.1860411Z    |               |
2019-11-12T09:18:59.1860668Z    |               the evaluated program panicked at 'not yet implemented', <::core::macros::panic macros>:3:10
2019-11-12T09:18:59.1861032Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T09:18:59.1861069Z 
2019-11-12T09:18:59.1861109Z error: aborting due to 3 previous errors
2019-11-12T09:18:59.1861150Z 
---
2019-11-12T09:18:59.1873204Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-12T09:18:59.1873330Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-12T09:18:59.1890700Z 
2019-11-12T09:18:59.1891544Z 
2019-11-12T09:18:59.1895220Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-12T09:18:59.1895618Z 
2019-11-12T09:18:59.1895670Z 
2019-11-12T09:18:59.1904672Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-12T09:18:59.1904950Z Build completed unsuccessfully in 1:04:52
2019-11-12T09:18:59.1904950Z Build completed unsuccessfully in 1:04:52
2019-11-12T09:18:59.1958045Z == clock drift check ==
2019-11-12T09:18:59.1970916Z   local time: Tue Nov 12 09:18:59 UTC 2019
2019-11-12T09:18:59.4760593Z   network time: Tue, 12 Nov 2019 09:18:59 GMT
2019-11-12T09:18:59.4764351Z == end clock drift check ==
2019-11-12T09:19:00.3172078Z 
2019-11-12T09:19:00.3275264Z ##[error]Bash exited with code '1'.
2019-11-12T09:19:00.3318253Z ##[section]Starting: Checkout
2019-11-12T09:19:00.3319944Z ==============================================================================
2019-11-12T09:19:00.3320010Z Task         : Get sources
2019-11-12T09:19:00.3320051Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
