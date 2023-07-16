plain
2020-02-16T16:07:32.4696507Z ========================== Starting Command Output ===========================
2020-02-16T16:07:32.4700452Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c48e38a1-135d-4ff8-9b7c-f7ebe0df3a59.sh
2020-02-16T16:07:32.4700662Z 
2020-02-16T16:07:32.4704229Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T16:07:32.4715696Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69209/merge to s
2020-02-16T16:07:32.4717586Z Task         : Get sources
2020-02-16T16:07:32.4717623Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T16:07:32.4717660Z Version      : 1.0.0
2020-02-16T16:07:32.4717741Z Author       : Microsoft
---
2020-02-16T16:07:33.5961034Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T16:07:33.6212257Z ##[command]git config gc.auto 0
2020-02-16T16:07:33.6305770Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T16:07:33.6365800Z ##[command]git config --get-all http.proxy
2020-02-16T16:07:33.6521443Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69209/merge:refs/remotes/pull/69209/merge
---
2020-02-16T17:06:51.3542474Z .................................................................................................... 1700/9650
2020-02-16T17:06:56.1884370Z .................................................................................................... 1800/9650
2020-02-16T17:07:07.7970018Z ..................................i................................................................. 1900/9650
2020-02-16T17:07:15.7324665Z .................................................................................................... 2000/9650
2020-02-16T17:07:30.0033323Z ........................iiiii....................................................................... 2100/9650
2020-02-16T17:07:39.6624677Z .................................................................................................... 2300/9650
2020-02-16T17:07:42.1175988Z .................................................................................................... 2400/9650
2020-02-16T17:07:46.6249443Z .................................................................................................... 2500/9650
2020-02-16T17:08:07.7976507Z .................................................................................................... 2600/9650
---
2020-02-16T17:11:26.3842336Z .................................................................................................... 5600/9650
2020-02-16T17:11:36.6027782Z .......................................................................................i............ 5700/9650
2020-02-16T17:11:44.2026945Z .................................................................................................... 5800/9650
2020-02-16T17:11:49.3379858Z .....................................................................................i.............. 5900/9650
2020-02-16T17:11:58.7283156Z ...............................................................................ii...i..ii........... 6000/9650
2020-02-16T17:12:10.7492398Z i................................................................................................... 6100/9650
2020-02-16T17:12:26.9194413Z .................................................................................................... 6300/9650
2020-02-16T17:12:34.4222863Z .................................................................................................... 6400/9650
2020-02-16T17:12:34.4222863Z .................................................................................................... 6400/9650
2020-02-16T17:12:46.8856111Z .......i..ii........................................................................................ 6500/9650
2020-02-16T17:13:06.4745319Z ...............................................................................................i.... 6700/9650
2020-02-16T17:13:08.6459253Z .................................................................................................... 6800/9650
2020-02-16T17:13:10.7627649Z .................................................................................................... 6900/9650
2020-02-16T17:13:13.1366542Z .....i.............................................................................................. 7000/9650
---
2020-02-16T17:14:47.1328975Z .................................................................................................... 7600/9650
2020-02-16T17:14:51.8360614Z .................................................................................................... 7700/9650
2020-02-16T17:14:57.9956965Z .................................................................................................... 7800/9650
2020-02-16T17:15:04.6833162Z .................................................................................................... 7900/9650
2020-02-16T17:15:14.5339597Z .......................................................................................iiiiiii.i.... 8000/9650
2020-02-16T17:15:30.4607045Z ...........................i......i................................................................. 8200/9650
2020-02-16T17:15:35.4795353Z .................................................................................................... 8300/9650
2020-02-16T17:15:47.1723520Z .................................................................................................... 8400/9650
2020-02-16T17:15:59.1339575Z .................................................................................................... 8500/9650
---
2020-02-16T17:17:56.5934226Z 
2020-02-16T17:17:56.5934970Z ---- [ui] ui/fmt/send-sync.rs stdout ----
2020-02-16T17:17:56.5935359Z diff of stderr:
2020-02-16T17:17:56.5935571Z 
2020-02-16T17:17:56.5936117Z - error[E0277]: `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2020-02-16T17:17:56.5936415Z + error[E0277]: `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T17:17:56.5937152Z 3    |
2020-02-16T17:17:56.5937152Z 3    |
2020-02-16T17:17:56.5937365Z 4 LL | fn send<T: Send>(_: T) {}
2020-02-16T17:17:56.5938035Z 5    |    ----    ---- required by this bound in `send`
2020-02-16T17:17:56.5938310Z 6 ...
2020-02-16T17:17:56.5938310Z 6 ...
2020-02-16T17:17:56.5938539Z 7 LL |     send(format_args!("{:?}", c));
2020-02-16T17:17:56.5939049Z -    |     ^^^^ `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2020-02-16T17:17:56.5939334Z +    |     ^^^^ `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T17:17:56.5939538Z 9    |
2020-02-16T17:17:56.5940071Z -    = help: within `[std::fmt::ArgumentV1<'_>]`, the trait `std::marker::Sync` is not implemented for `*mut (dyn std::ops::Fn() + 'static)`
2020-02-16T17:17:56.5940640Z -    = note: required because it appears within the type `std::marker::PhantomData<*mut (dyn std::ops::Fn() + 'static)>`
2020-02-16T17:17:56.5941183Z -    = note: required because it appears within the type `core::fmt::Void`
2020-02-16T17:17:56.5941685Z -    = note: required because it appears within the type `&core::fmt::Void`
2020-02-16T17:17:56.5942270Z +    = help: within `[std::fmt::ArgumentV1<'_>]`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
2020-02-16T17:17:56.5942557Z +    = note: required because it appears within the type `&core::fmt::Opaque`
2020-02-16T17:17:56.5943037Z 14    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T17:17:56.5943584Z 15    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
2020-02-16T17:17:56.5944137Z 16    = note: required because of the requirements on the impl of `std::marker::Send` for `&[std::fmt::ArgumentV1<'_>]`
2020-02-16T17:17:56.5944881Z 17    = note: required because it appears within the type `std::fmt::Arguments<'_>`
2020-02-16T17:17:56.5945147Z 18 
2020-02-16T17:17:56.5945147Z 18 
2020-02-16T17:17:56.5945613Z - error[E0277]: `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2020-02-16T17:17:56.5945917Z + error[E0277]: `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T17:17:56.5946846Z 21    |
2020-02-16T17:17:56.5946846Z 21    |
2020-02-16T17:17:56.5947061Z 22 LL | fn sync<T: Sync>(_: T) {}
2020-02-16T17:17:56.5947920Z 23    |    ----    ---- required by this bound in `sync`
2020-02-16T17:17:56.5948230Z 24 ...
2020-02-16T17:17:56.5948230Z 24 ...
2020-02-16T17:17:56.5948437Z 25 LL |     sync(format_args!("{:?}", c));
2020-02-16T17:17:56.5948923Z -    |     ^^^^ `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2020-02-16T17:17:56.5949206Z +    |     ^^^^ `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T17:17:56.5949409Z 27    |
2020-02-16T17:17:56.5949927Z -    = help: within `std::fmt::Arguments<'_>`, the trait `std::marker::Sync` is not implemented for `*mut (dyn std::ops::Fn() + 'static)`
2020-02-16T17:17:56.5950488Z -    = note: required because it appears within the type `std::marker::PhantomData<*mut (dyn std::ops::Fn() + 'static)>`
2020-02-16T17:17:56.5951011Z -    = note: required because it appears within the type `core::fmt::Void`
2020-02-16T17:17:56.5951547Z -    = note: required because it appears within the type `&core::fmt::Void`
2020-02-16T17:17:56.5952105Z +    = help: within `std::fmt::Arguments<'_>`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
2020-02-16T17:17:56.5952555Z +    = note: required because it appears within the type `&core::fmt::Opaque`
2020-02-16T17:17:56.5953880Z 32    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T17:17:56.5954998Z 33    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
2020-02-16T17:17:56.5955766Z 34    = note: required because it appears within the type `&[std::fmt::ArgumentV1<'_>]`
2020-02-16T17:17:56.5956239Z 
2020-02-16T17:17:56.5956470Z The actual stderr differed from the expected stderr.
2020-02-16T17:17:56.5956966Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync/send-sync.stderr
2020-02-16T17:17:56.5956966Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync/send-sync.stderr
2020-02-16T17:17:56.5957483Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T17:17:56.5958013Z To only update this specific test, also pass `--test-args fmt/send-sync.rs`
2020-02-16T17:17:56.5958489Z error: 1 errors occurred comparing output.
2020-02-16T17:17:56.5958713Z status: exit code: 1
2020-02-16T17:17:56.5958713Z status: exit code: 1
2020-02-16T17:17:56.5959616Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/send-sync.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync/auxiliary" "-A" "unused"
2020-02-16T17:17:56.5960776Z ------------------------------------------
2020-02-16T17:17:56.5962383Z 
2020-02-16T17:17:56.5965450Z ------------------------------------------
2020-02-16T17:17:56.5965561Z stderr:
2020-02-16T17:17:56.5965561Z stderr:
2020-02-16T17:17:56.5965802Z ------------------------------------------
2020-02-16T17:17:56.5965850Z error[E0277]: `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T17:17:56.5966064Z   --> /checkout/src/test/ui/fmt/send-sync.rs:8:5
2020-02-16T17:17:56.5966128Z    |
2020-02-16T17:17:56.5966165Z LL | fn send<T: Send>(_: T) {}
2020-02-16T17:17:56.5966434Z ...
2020-02-16T17:17:56.5966434Z ...
2020-02-16T17:17:56.5966474Z LL |     send(format_args!("{:?}", c)); //~ ERROR E0277
2020-02-16T17:17:56.5966519Z    |     ^^^^ `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T17:17:56.5966576Z    |
2020-02-16T17:17:56.5967285Z    = help: within `[std::fmt::ArgumentV1<'_>]`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
2020-02-16T17:17:56.5967361Z    = note: required because it appears within the type `&core::fmt::Opaque`
2020-02-16T17:17:56.5967662Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T17:17:56.5968015Z    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
2020-02-16T17:17:56.5968276Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&[std::fmt::ArgumentV1<'_>]`
2020-02-16T17:17:56.5968527Z    = note: required because it appears within the type `std::fmt::Arguments<'_>`
2020-02-16T17:17:56.5968600Z error[E0277]: `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T17:17:56.5968818Z   --> /checkout/src/test/ui/fmt/send-sync.rs:9:5
2020-02-16T17:17:56.5968857Z    |
2020-02-16T17:17:56.5968857Z    |
2020-02-16T17:17:56.5968894Z LL | fn sync<T: Sync>(_: T) {}
2020-02-16T17:17:56.5969158Z ...
2020-02-16T17:17:56.5969158Z ...
2020-02-16T17:17:56.5969208Z LL |     sync(format_args!("{:?}", c)); //~ ERROR E0277
2020-02-16T17:17:56.5969252Z    |     ^^^^ `core::fmt::Opaque` cannot be shared between threads safely
2020-02-16T17:17:56.5969578Z    = help: within `std::fmt::Arguments<'_>`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
2020-02-16T17:17:56.5969630Z    = note: required because it appears within the type `&core::fmt::Opaque`
2020-02-16T17:17:56.5969879Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T17:17:56.5969879Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2020-02-16T17:17:56.5970360Z    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
2020-02-16T17:17:56.5970607Z    = note: required because it appears within the type `&[std::fmt::ArgumentV1<'_>]`
2020-02-16T17:17:56.5970861Z    = note: required because it appears within the type `std::fmt::Arguments<'_>`
2020-02-16T17:17:56.5970930Z error: aborting due to 2 previous errors
2020-02-16T17:17:56.5970965Z 
2020-02-16T17:17:56.5971202Z For more information about this error, try `rustc --explain E0277`.
2020-02-16T17:17:56.5971230Z 
---
2020-02-16T17:17:56.5972284Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T17:17:56.5972351Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T17:17:56.5972379Z 
2020-02-16T17:17:56.5972401Z 
2020-02-16T17:17:56.5973805Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T17:17:56.5974069Z 
2020-02-16T17:17:56.5974096Z 
2020-02-16T17:17:56.5976408Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T17:17:56.5977228Z Build completed unsuccessfully in 1:03:34
2020-02-16T17:17:56.5977228Z Build completed unsuccessfully in 1:03:34
2020-02-16T17:17:56.6037008Z == clock drift check ==
2020-02-16T17:17:56.6065488Z   local time: Sun Feb 16 17:17:56 UTC 2020
2020-02-16T17:17:57.1590986Z   network time: Sun, 16 Feb 2020 17:17:57 GMT
2020-02-16T17:17:57.1591546Z == end clock drift check ==
2020-02-16T17:17:57.5849333Z 
2020-02-16T17:17:57.5949305Z ##[error]Bash exited with code '1'.
2020-02-16T17:17:57.5963086Z ##[section]Finishing: Run build
2020-02-16T17:17:57.5984828Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69209/merge to s
2020-02-16T17:17:57.5987090Z Task         : Get sources
2020-02-16T17:17:57.5987143Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T17:17:57.5987218Z Version      : 1.0.0
2020-02-16T17:17:57.5987287Z Author       : Microsoft
2020-02-16T17:17:57.5987287Z Author       : Microsoft
2020-02-16T17:17:57.5987340Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T17:17:57.5987396Z ==============================================================================
2020-02-16T17:17:58.0025176Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T17:17:58.0063266Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69209/merge to s
2020-02-16T17:17:58.0194210Z Cleaning up task key
2020-02-16T17:17:58.0195053Z Start cleaning up orphan processes.
2020-02-16T17:17:58.0314542Z Terminate orphan process: pid (3884) (python)
2020-02-16T17:17:58.0592600Z ##[section]Finishing: Finalize Job
