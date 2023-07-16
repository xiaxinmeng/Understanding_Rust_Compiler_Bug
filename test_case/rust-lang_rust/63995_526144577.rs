plain
2019-08-29T08:54:16.8610635Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-29T08:54:16.8810332Z ##[command]git config gc.auto 0
2019-08-29T08:54:16.8882638Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-29T08:54:16.8976674Z ##[command]git config --get-all http.proxy
2019-08-29T08:54:16.9065360Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63995/merge:refs/remotes/pull/63995/merge
---
2019-08-29T09:52:33.8895069Z .................................................................................................... 1500/8973
2019-08-29T09:52:39.2820295Z .................................................................................................... 1600/8973
2019-08-29T09:52:51.2585168Z ................................................i...............i................................... 1700/8973
2019-08-29T09:52:58.9669635Z .................................................................................................... 1800/8973
2019-08-29T09:53:12.4071033Z .......................................iiiii........................................................ 1900/8973
2019-08-29T09:53:22.4350069Z .................................................................................................... 2100/8973
2019-08-29T09:53:24.8232518Z .................................................................................................... 2200/8973
2019-08-29T09:53:28.6436236Z .................................................................................................... 2300/8973
2019-08-29T09:53:35.6168048Z .................................................................................................... 2400/8973
---
2019-08-29T09:56:21.9915354Z ..........................i...............i......................................................... 4700/8973
2019-08-29T09:56:33.0070466Z .................................................................................................... 4800/8973
2019-08-29T09:56:38.7875138Z .................................................................................................... 4900/8973
2019-08-29T09:56:48.8862245Z .................................................................................................... 5000/8973
2019-08-29T09:56:54.3508816Z .......ii.ii........................................................................................ 5100/8973
2019-08-29T09:57:07.2251374Z .................................................................................................... 5300/8973
2019-08-29T09:57:15.1765482Z ......................................................................i............................. 5400/8973
2019-08-29T09:57:22.5133143Z .................................................................................................... 5500/8973
2019-08-29T09:57:29.4861419Z .................................................................................................... 5600/8973
2019-08-29T09:57:29.4861419Z .................................................................................................... 5600/8973
2019-08-29T09:57:39.9392887Z ................................................................ii...i..ii...........i.............. 5700/8973
2019-08-29T09:58:05.5084518Z .................................................................................................... 5900/8973
2019-08-29T09:58:14.6649347Z .................................................................................................... 6000/8973
2019-08-29T09:58:14.6649347Z .................................................................................................... 6000/8973
2019-08-29T09:58:23.4406917Z .................................................................i..ii.............................. 6100/8973
2019-08-29T09:58:52.3306197Z .................................................................................................... 6300/8973
2019-08-29T09:58:54.2905299Z ....................i............................................................................... 6400/8973
2019-08-29T09:58:56.2875403Z ............................................................................................i....... 6500/8973
2019-08-29T09:58:58.8183683Z .................................................................................................... 6600/8973
---
2019-08-29T10:02:52.0725134Z diff of stderr:
2019-08-29T10:02:52.0725297Z 
2019-08-29T10:02:52.0725474Z 43    | ^^^^^^^^^^^^^^^^^^
2019-08-29T10:02:52.0725641Z 44 
2019-08-29T10:02:52.0726092Z 45 error: `extern` block uses type `std::option::Option<std::ptr::Unique<u8>>` which is not FFI-safe: enum has no representation hint
2019-08-29T10:02:52.0727519Z +   --> $DIR/lint-ctypes-enum.rs:48:17
2019-08-29T10:02:52.0727849Z 47    |
2019-08-29T10:02:52.0727849Z 47    |
2019-08-29T10:02:52.0728126Z 48 LL |    fn unique(x: Option<std::ptr::Unique<u8>>);
2019-08-29T10:02:52.0728898Z +    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-29T10:02:52.0729135Z 50    |
2019-08-29T10:02:52.0729135Z 50    |
2019-08-29T10:02:52.0729349Z 51    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T10:02:52.0729761Z 
2019-08-29T10:02:52.0729939Z 
2019-08-29T10:02:52.0730141Z The actual stderr differed from the expected stderr.
2019-08-29T10:02:52.0730826Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/lint-ctypes-enum.stderr
2019-08-29T10:02:52.0730826Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/lint-ctypes-enum.stderr
2019-08-29T10:02:52.0731273Z To update references, rerun the tests and pass the `--bless` flag
2019-08-29T10:02:52.0731795Z To only update this specific test, also pass `--test-args lint/lint-ctypes-enum.rs`
2019-08-29T10:02:52.0732537Z error: 1 errors occurred comparing output.
2019-08-29T10:02:52.0732888Z status: exit code: 1
2019-08-29T10:02:52.0732888Z status: exit code: 1
2019-08-29T10:02:52.0733644Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/auxiliary" "-A" "unused"
2019-08-29T10:02:52.0734452Z ------------------------------------------
2019-08-29T10:02:52.0734651Z 
2019-08-29T10:02:52.0735061Z ------------------------------------------
2019-08-29T10:02:52.0735280Z stderr:
2019-08-29T10:02:52.0735280Z stderr:
2019-08-29T10:02:52.0735621Z ------------------------------------------
2019-08-29T10:02:52.0736308Z error: `extern` block uses type `U` which is not FFI-safe: enum has no representation hint
2019-08-29T10:02:52.0736979Z    |
2019-08-29T10:02:52.0736979Z    |
2019-08-29T10:02:52.0737812Z LL |    fn uf(x: U); //~ ERROR enum has no representation hint
2019-08-29T10:02:52.0738264Z    |
2019-08-29T10:02:52.0738478Z note: lint level defined here
2019-08-29T10:02:52.0738962Z   --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:3:9
2019-08-29T10:02:52.0739254Z    |
2019-08-29T10:02:52.0739254Z    |
2019-08-29T10:02:52.0739471Z LL | #![deny(improper_ctypes)]
2019-08-29T10:02:52.0739673Z    |         ^^^^^^^^^^^^^^^
2019-08-29T10:02:52.0739913Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T10:02:52.0741038Z   --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:9:1
2019-08-29T10:02:52.0741462Z    |
2019-08-29T10:02:52.0741462Z    |
2019-08-29T10:02:52.0741817Z LL | enum U { A }
2019-08-29T10:02:52.0742693Z 
2019-08-29T10:02:52.0742693Z 
2019-08-29T10:02:52.0743313Z error: `extern` block uses type `B` which is not FFI-safe: enum has no representation hint
2019-08-29T10:02:52.0744030Z    |
2019-08-29T10:02:52.0744030Z    |
2019-08-29T10:02:52.0744335Z LL |    fn bf(x: B); //~ ERROR enum has no representation hint
2019-08-29T10:02:52.0745798Z    |
2019-08-29T10:02:52.0745798Z    |
2019-08-29T10:02:52.0746806Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T10:02:52.0749089Z   --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:10:1
2019-08-29T10:02:52.0749856Z    |
2019-08-29T10:02:52.0749856Z    |
2019-08-29T10:02:52.0750295Z LL | enum B { C, D }
2019-08-29T10:02:52.0751626Z 
2019-08-29T10:02:52.0751626Z 
2019-08-29T10:02:52.0752053Z error: `extern` block uses type `T` which is not FFI-safe: enum has no representation hint
2019-08-29T10:02:52.0754445Z    |
2019-08-29T10:02:52.0754445Z    |
2019-08-29T10:02:52.0754685Z LL |    fn tf(x: T); //~ ERROR enum has no representation hint
2019-08-29T10:02:52.0757960Z    |
2019-08-29T10:02:52.0757960Z    |
2019-08-29T10:02:52.0758462Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T10:02:52.0761309Z   --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:11:1
2019-08-29T10:02:52.0761682Z    |
2019-08-29T10:02:52.0761682Z    |
2019-08-29T10:02:52.0761809Z LL | enum T { E, F, G }
2019-08-29T10:02:52.0762218Z 
2019-08-29T10:02:52.0762218Z 
2019-08-29T10:02:52.0763015Z error: `extern` block uses type `std::option::Option<std::ptr::Unique<u8>>` which is not FFI-safe: enum has no representation hint
2019-08-29T10:02:52.0763760Z    |
2019-08-29T10:02:52.0763760Z    |
2019-08-29T10:02:52.0764103Z LL |    fn unique(x: Option<std::ptr::Unique<u8>>); //~ ERROR enum has no representation hint
2019-08-29T10:02:52.0764537Z    |
2019-08-29T10:02:52.0764537Z    |
2019-08-29T10:02:52.0764657Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T10:02:52.0764783Z 
2019-08-29T10:02:52.0765157Z error: `extern` block uses type `u128` which is not FFI-safe: 128-bit integers don't currently have a known stable ABI
2019-08-29T10:02:52.0765684Z    |
2019-08-29T10:02:52.0765684Z    |
2019-08-29T10:02:52.0765969Z LL |    fn nonzero_u128(x: Option<num::NonZeroU128>);
2019-08-29T10:02:52.0766212Z 
2019-08-29T10:02:52.0766212Z 
2019-08-29T10:02:52.0766664Z error: `extern` block uses type `i128` which is not FFI-safe: 128-bit integers don't currently have a known stable ABI
2019-08-29T10:02:52.0768456Z    |
2019-08-29T10:02:52.0768456Z    |
2019-08-29T10:02:52.0768654Z LL |    fn nonzero_i128(x: Option<num::NonZeroI128>);
2019-08-29T10:02:52.0769034Z 
2019-08-29T10:02:52.0769034Z 
2019-08-29T10:02:52.0769555Z error: `extern` block uses type `std::option::Option<TransparentUnion<std::num::NonZeroU8>>` which is not FFI-safe: enum has no representation hint
2019-08-29T10:02:52.0770273Z    |
2019-08-29T10:02:52.0770273Z    |
2019-08-29T10:02:52.0770474Z LL |    fn transparent_union(x: Option<TransparentUnion<num::NonZeroU8>>);
2019-08-29T10:02:52.0771424Z    |
2019-08-29T10:02:52.0771424Z    |
2019-08-29T10:02:52.0771608Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T10:02:52.0778743Z 
2019-08-29T10:02:52.0779596Z error: `extern` block uses type `std::option::Option<Rust<std::num::NonZeroU8>>` which is not FFI-safe: enum has no representation hint
2019-08-29T10:02:52.0780030Z    |
2019-08-29T10:02:52.0780030Z    |
2019-08-29T10:02:52.0780084Z LL |    fn repr_rust(x: Option<Rust<num::NonZeroU8>>); //~ ERROR enum has no representation hint
2019-08-29T10:02:52.0780200Z    |
2019-08-29T10:02:52.0780200Z    |
2019-08-29T10:02:52.0780255Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T10:02:52.0780292Z 
2019-08-29T10:02:52.0780783Z error: `extern` block uses type `std::result::Result<(), std::num::NonZeroI32>` which is not FFI-safe: enum has no representation hint
2019-08-29T10:02:52.0781242Z    |
2019-08-29T10:02:52.0781242Z    |
2019-08-29T10:02:52.0781285Z LL |    fn no_result(x: Result<(), num::NonZeroI32>); //~ ERROR enum has no representation hint
2019-08-29T10:02:52.0781386Z    |
2019-08-29T10:02:52.0781386Z    |
2019-08-29T10:02:52.0781434Z    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
2019-08-29T10:02:52.0781690Z error: aborting due to 9 previous errors
2019-08-29T10:02:52.0781715Z 
2019-08-29T10:02:52.0781737Z 
2019-08-29T10:02:52.0782108Z ------------------------------------------
---
2019-08-29T10:02:52.0783297Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-29T10:02:52.0783349Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-29T10:02:52.0783378Z 
2019-08-29T10:02:52.0783400Z 
2019-08-29T10:02:52.0784694Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-29T10:02:52.0785071Z 
2019-08-29T10:02:52.0785114Z 
2019-08-29T10:02:52.0787485Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-29T10:02:52.0788320Z Build completed unsuccessfully in 1:01:55
2019-08-29T10:02:52.0788320Z Build completed unsuccessfully in 1:01:55
2019-08-29T10:02:52.0846694Z == clock drift check ==
2019-08-29T10:02:52.0866382Z   local time: Thu Aug 29 10:02:52 UTC 2019
2019-08-29T10:02:52.2350670Z   network time: Thu, 29 Aug 2019 10:02:52 GMT
2019-08-29T10:02:52.2354938Z == end clock drift check ==
2019-08-29T10:02:53.0256633Z ##[error]Bash exited with code '1'.
2019-08-29T10:02:53.0294878Z ##[section]Starting: Checkout
2019-08-29T10:02:53.0297621Z ==============================================================================
2019-08-29T10:02:53.0297678Z Task         : Get sources
2019-08-29T10:02:53.0297727Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
