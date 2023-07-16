plain
2019-08-29T08:15:32.8469056Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-29T08:15:32.8647137Z ##[command]git config gc.auto 0
2019-08-29T08:15:32.8733136Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-29T08:15:32.8783132Z ##[command]git config --get-all http.proxy
2019-08-29T08:15:32.8929083Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63991/merge:refs/remotes/pull/63991/merge
---
2019-08-29T09:20:33.6748050Z .................................................................................................... 1500/8972
2019-08-29T09:20:39.4897726Z .................................................................................................... 1600/8972
2019-08-29T09:20:52.6060788Z ................................................i...............i................................... 1700/8972
2019-08-29T09:21:01.1055888Z .................................................................................................... 1800/8972
2019-08-29T09:21:15.9495709Z .......................................iiiii........................................................ 1900/8972
2019-08-29T09:21:27.1852740Z .................................................................................................... 2100/8972
2019-08-29T09:21:29.8715328Z .................................................................................................... 2200/8972
2019-08-29T09:21:34.0846877Z .................................................................................................... 2300/8972
2019-08-29T09:21:41.9864662Z .................................................................................................... 2400/8972
---
2019-08-29T09:24:46.7081280Z ..........................i...............i......................................................... 4700/8972
2019-08-29T09:24:58.8807629Z .................................................................................................... 4800/8972
2019-08-29T09:25:05.2548936Z .................................................................................................... 4900/8972
2019-08-29T09:25:16.5272826Z .................................................................................................... 5000/8972
2019-08-29T09:25:22.3870170Z .......ii.ii........................................................................................ 5100/8972
2019-08-29T09:25:35.9127663Z .................................................................................................... 5300/8972
2019-08-29T09:25:44.4045970Z ......................................................................i............................. 5400/8972
2019-08-29T09:25:52.0089286Z .................................................................................................... 5500/8972
2019-08-29T09:25:59.1660088Z .................................................................................................... 5600/8972
2019-08-29T09:25:59.1660088Z .................................................................................................... 5600/8972
2019-08-29T09:26:10.0886595Z ................................................................ii...i..ii...........i.............. 5700/8972
2019-08-29T09:26:36.3410833Z .................................................................................................... 5900/8972
2019-08-29T09:26:46.1511124Z .................................................................................................... 6000/8972
2019-08-29T09:26:46.1511124Z .................................................................................................... 6000/8972
2019-08-29T09:26:56.0436016Z .................................................................i..ii.............................. 6100/8972
2019-08-29T09:27:25.9282563Z .................................................................................................... 6300/8972
2019-08-29T09:27:28.0897186Z ....................i............................................................................... 6400/8972
2019-08-29T09:27:30.1729513Z ............................................................................................i....... 6500/8972
2019-08-29T09:27:32.8932120Z .................................................................................................... 6600/8972
---
2019-08-29T09:31:40.1473426Z diff of stderr:
2019-08-29T09:31:40.1473573Z 
2019-08-29T09:31:40.1473708Z 43    | ^^^^^^^^^^^^^^^^^^
2019-08-29T09:31:40.1473857Z 44 
2019-08-29T09:31:40.1474330Z 45 error: `extern` block uses type `std::option::Option<std::ptr::Unique<u8>>` which is not FFI-safe: enum has no representation hint
2019-08-29T09:31:40.1476167Z +   --> $DIR/lint-ctypes-enum.rs:48:17
2019-08-29T09:31:40.1476459Z 47    |
2019-08-29T09:31:40.1476459Z 47    |
2019-08-29T09:31:40.1476628Z 48 LL |    fn unique(x: Option<std::ptr::Unique<u8>>);
2019-08-29T09:31:40.1477219Z +    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-29T09:31:40.1477375Z 50    |
2019-08-29T09:31:40.1477375Z 50    |
2019-08-29T09:31:40.1477528Z 51    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T09:31:40.1477801Z 
2019-08-29T09:31:40.1478269Z 
2019-08-29T09:31:40.1478493Z The actual stderr differed from the expected stderr.
2019-08-29T09:31:40.1479020Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/lint-ctypes-enum.stderr
2019-08-29T09:31:40.1479020Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/lint-ctypes-enum.stderr
2019-08-29T09:31:40.1479470Z To update references, rerun the tests and pass the `--bless` flag
2019-08-29T09:31:40.1479918Z To only update this specific test, also pass `--test-args lint/lint-ctypes-enum.rs`
2019-08-29T09:31:40.1480251Z error: 1 errors occurred comparing output.
2019-08-29T09:31:40.1480413Z status: exit code: 1
2019-08-29T09:31:40.1480413Z status: exit code: 1
2019-08-29T09:31:40.1481652Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/auxiliary" "-A" "unused"
2019-08-29T09:31:40.1482333Z ------------------------------------------
2019-08-29T09:31:40.1482495Z 
2019-08-29T09:31:40.1482876Z ------------------------------------------
2019-08-29T09:31:40.1483126Z stderr:
2019-08-29T09:31:40.1483126Z stderr:
2019-08-29T09:31:40.1483801Z ------------------------------------------
2019-08-29T09:31:40.1485162Z error: `extern` block uses type `U` which is not FFI-safe: enum has no representation hint
2019-08-29T09:31:40.1485689Z    |
2019-08-29T09:31:40.1485689Z    |
2019-08-29T09:31:40.1485737Z LL |    fn uf(x: U); //~ ERROR enum has no representation hint
2019-08-29T09:31:40.1485843Z    |
2019-08-29T09:31:40.1485896Z note: lint level defined here
2019-08-29T09:31:40.1486388Z   --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:3:9
2019-08-29T09:31:40.1486435Z    |
2019-08-29T09:31:40.1486435Z    |
2019-08-29T09:31:40.1486494Z LL | #![deny(improper_ctypes)]
2019-08-29T09:31:40.1486536Z    |         ^^^^^^^^^^^^^^^
2019-08-29T09:31:40.1486589Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T09:31:40.1487149Z   --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:9:1
2019-08-29T09:31:40.1487194Z    |
2019-08-29T09:31:40.1487194Z    |
2019-08-29T09:31:40.1487252Z LL | enum U { A }
2019-08-29T09:31:40.1487332Z 
2019-08-29T09:31:40.1487332Z 
2019-08-29T09:31:40.1487599Z error: `extern` block uses type `B` which is not FFI-safe: enum has no representation hint
2019-08-29T09:31:40.1488645Z    |
2019-08-29T09:31:40.1488645Z    |
2019-08-29T09:31:40.1488748Z LL |    fn bf(x: B); //~ ERROR enum has no representation hint
2019-08-29T09:31:40.1488857Z    |
2019-08-29T09:31:40.1488857Z    |
2019-08-29T09:31:40.1488912Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T09:31:40.1489278Z   --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:10:1
2019-08-29T09:31:40.1489327Z    |
2019-08-29T09:31:40.1489327Z    |
2019-08-29T09:31:40.1489387Z LL | enum B { C, D }
2019-08-29T09:31:40.1489463Z 
2019-08-29T09:31:40.1489463Z 
2019-08-29T09:31:40.1489739Z error: `extern` block uses type `T` which is not FFI-safe: enum has no representation hint
2019-08-29T09:31:40.1490070Z    |
2019-08-29T09:31:40.1490070Z    |
2019-08-29T09:31:40.1490118Z LL |    fn tf(x: T); //~ ERROR enum has no representation hint
2019-08-29T09:31:40.1490223Z    |
2019-08-29T09:31:40.1490223Z    |
2019-08-29T09:31:40.1490285Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T09:31:40.1490607Z   --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:11:1
2019-08-29T09:31:40.1490655Z    |
2019-08-29T09:31:40.1490655Z    |
2019-08-29T09:31:40.1490697Z LL | enum T { E, F, G }
2019-08-29T09:31:40.1490787Z 
2019-08-29T09:31:40.1490787Z 
2019-08-29T09:31:40.1491103Z error: `extern` block uses type `std::option::Option<std::ptr::Unique<u8>>` which is not FFI-safe: enum has no representation hint
2019-08-29T09:31:40.1491437Z    |
2019-08-29T09:31:40.1491437Z    |
2019-08-29T09:31:40.1491488Z LL |    fn unique(x: Option<std::ptr::Unique<u8>>); //~ ERROR enum has no representation hint
2019-08-29T09:31:40.1491604Z    |
2019-08-29T09:31:40.1491604Z    |
2019-08-29T09:31:40.1492258Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T09:31:40.1493386Z 
2019-08-29T09:31:40.1493839Z error: `extern` block uses type `u128` which is not FFI-safe: 128-bit integers don't currently have a known stable ABI
2019-08-29T09:31:40.1494132Z    |
2019-08-29T09:31:40.1494132Z    |
2019-08-29T09:31:40.1494192Z LL |    fn nonzero_u128(x: Option<num::NonZeroU128>);
2019-08-29T09:31:40.1494271Z 
2019-08-29T09:31:40.1494271Z 
2019-08-29T09:31:40.1494557Z error: `extern` block uses type `i128` which is not FFI-safe: 128-bit integers don't currently have a known stable ABI
2019-08-29T09:31:40.1495217Z    |
2019-08-29T09:31:40.1495217Z    |
2019-08-29T09:31:40.1495262Z LL |    fn nonzero_i128(x: Option<num::NonZeroI128>);
2019-08-29T09:31:40.1495357Z 
2019-08-29T09:31:40.1495357Z 
2019-08-29T09:31:40.1495692Z error: `extern` block uses type `std::option::Option<TransparentUnion<std::num::NonZeroU8>>` which is not FFI-safe: enum has no representation hint
2019-08-29T09:31:40.1496023Z    |
2019-08-29T09:31:40.1496023Z    |
2019-08-29T09:31:40.1496072Z LL |    fn transparent_union(x: Option<TransparentUnion<num::NonZeroU8>>);
2019-08-29T09:31:40.1496185Z    |
2019-08-29T09:31:40.1496185Z    |
2019-08-29T09:31:40.1496237Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T09:31:40.1496281Z 
2019-08-29T09:31:40.1496613Z error: `extern` block uses type `std::option::Option<Rust<std::num::NonZeroU8>>` which is not FFI-safe: enum has no representation hint
2019-08-29T09:31:40.1500443Z    |
2019-08-29T09:31:40.1500443Z    |
2019-08-29T09:31:40.1500539Z LL |    fn repr_rust(x: Option<Rust<num::NonZeroU8>>); //~ ERROR enum has no representation hint
2019-08-29T09:31:40.1500640Z    |
2019-08-29T09:31:40.1500640Z    |
2019-08-29T09:31:40.1500713Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T09:31:40.1500750Z 
2019-08-29T09:31:40.1501181Z error: `extern` block uses type `std::result::Result<(), std::num::NonZeroI32>` which is not FFI-safe: enum has no representation hint
2019-08-29T09:31:40.1501519Z    |
2019-08-29T09:31:40.1501519Z    |
2019-08-29T09:31:40.1501580Z LL |    fn no_result(x: Result<(), num::NonZeroI32>); //~ ERROR enum has no representation hint
2019-08-29T09:31:40.1501853Z    |
2019-08-29T09:31:40.1501853Z    |
2019-08-29T09:31:40.1501912Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T09:31:40.1502008Z error: aborting due to 9 previous errors
2019-08-29T09:31:40.1502037Z 
2019-08-29T09:31:40.1502063Z 
2019-08-29T09:31:40.1502296Z ------------------------------------------
---
2019-08-29T09:31:40.1504130Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-29T09:31:40.1504197Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-29T09:31:40.1516087Z 
2019-08-29T09:31:40.1516173Z 
2019-08-29T09:31:40.1518898Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-29T09:31:40.1519289Z 
2019-08-29T09:31:40.1519338Z 
2019-08-29T09:31:40.1523673Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-29T09:31:40.1523744Z Build completed unsuccessfully in 1:08:40
2019-08-29T09:31:40.1523744Z Build completed unsuccessfully in 1:08:40
2019-08-29T09:31:40.1573328Z == clock drift check ==
2019-08-29T09:31:40.1589474Z   local time: Thu Aug 29 09:31:40 UTC 2019
2019-08-29T09:31:40.4352406Z   network time: Thu, 29 Aug 2019 09:31:40 GMT
2019-08-29T09:31:40.4355659Z == end clock drift check ==
2019-08-29T09:31:41.1364059Z ##[error]Bash exited with code '1'.
2019-08-29T09:31:41.1405061Z ##[section]Starting: Checkout
2019-08-29T09:31:41.1406732Z ==============================================================================
2019-08-29T09:31:41.1406802Z Task         : Get sources
2019-08-29T09:31:41.1406847Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
