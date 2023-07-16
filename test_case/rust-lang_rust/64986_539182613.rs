plain
2019-10-07T18:56:30.4048938Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T18:56:31.2985241Z ##[command]git config gc.auto 0
2019-10-07T18:56:31.2988730Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T18:56:31.2990661Z ##[command]git config --get-all http.proxy
2019-10-07T18:56:31.2993066Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64986/merge:refs/remotes/pull/64986/merge
---
2019-10-07T19:57:58.0118396Z .................................................................................................... 1600/9128
2019-10-07T19:58:05.1966443Z .................................................................................................... 1700/9128
2019-10-07T19:58:17.1078176Z ..................i...............i................................................................. 1800/9128
2019-10-07T19:58:24.3752931Z .................................................................................................... 1900/9128
2019-10-07T19:58:40.3513344Z .........iiiii...................................................................................... 2000/9128
2019-10-07T19:58:49.3546736Z .................................................................................................... 2200/9128
2019-10-07T19:58:51.9379713Z .................................................................................................... 2300/9128
2019-10-07T19:58:57.6647035Z .................................................................................................... 2400/9128
2019-10-07T19:59:03.7882461Z ........................................................................................F........... 2500/9128
---
2019-10-07T20:01:58.6028110Z ...................................................................................................i 4700/9128
2019-10-07T20:02:05.9035538Z ...............i.................................................................................... 4800/9128
2019-10-07T20:02:17.3935828Z .................................................................................................... 4900/9128
2019-10-07T20:02:22.9875060Z .................................................................................................... 5000/9128
2019-10-07T20:02:34.6755858Z .............................................................................................ii.ii.. 5100/9128
2019-10-07T20:02:44.9394513Z .................................................................................................... 5300/9128
2019-10-07T20:02:54.6744252Z .................................................................................................... 5400/9128
2019-10-07T20:03:01.5821907Z ...........................................................i........................................ 5500/9128
2019-10-07T20:03:08.7471320Z .................................................................................................... 5600/9128
2019-10-07T20:03:08.7471320Z .................................................................................................... 5600/9128
2019-10-07T20:03:16.6980746Z .................................................................................................... 5700/9128
2019-10-07T20:03:26.5971995Z ........................................................ii...i..ii...........i...................... 5800/9128
2019-10-07T20:03:52.8685261Z .................................................................................................... 6000/9128
2019-10-07T20:04:02.3069950Z .................................................................................................... 6100/9128
2019-10-07T20:04:02.3069950Z .................................................................................................... 6100/9128
2019-10-07T20:04:09.5178853Z ..............................................................i..ii................................. 6200/9128
2019-10-07T20:04:38.5466954Z .................................................................................................... 6400/9128
2019-10-07T20:04:40.6958864Z ......................i............................................................................. 6500/9128
2019-10-07T20:04:42.9191247Z ...............................................................................................i.... 6600/9128
2019-10-07T20:04:45.6572852Z .................................................................................................... 6700/9128
---
2019-10-07T20:08:57.2899327Z 
2019-10-07T20:08:57.2900296Z ---- [ui] ui/feature-gates/feature-gate-const_generics-ptr.rs stdout ----
2019-10-07T20:08:57.2900647Z diff of stderr:
2019-10-07T20:08:57.2901379Z 
2019-10-07T20:08:57.2902125Z 16    = note: for more information, see ***/issues/44580
2019-10-07T20:08:57.2902504Z 17    = help: add `#![feature(const_generics)]` to the crate attributes to enable
2019-10-07T20:08:57.2903301Z - error[E0658]: use of function pointers as const generic arguments are unstable
2019-10-07T20:08:57.2903649Z + error[E0658]: using function pointers as const generic parameters is unstable
2019-10-07T20:08:57.2904323Z 20   --> $DIR/feature-gate-const_generics-ptr.rs:1:25
2019-10-07T20:08:57.2904763Z 21    |
2019-10-07T20:08:57.2904763Z 21    |
2019-10-07T20:08:57.2905011Z 22 LL | struct ConstFn<const F: fn()>;
2019-10-07T20:08:57.2905199Z 
2019-10-07T20:08:57.2905762Z 25    = note: for more information, see ***/issues/53020
2019-10-07T20:08:57.2906076Z 26    = help: add `#![feature(const_compare_raw_pointers)]` to the crate attributes to enable
2019-10-07T20:08:57.2906788Z - error[E0658]: use of raw pointers as const generic arguments are unstable
2019-10-07T20:08:57.2907087Z + error[E0658]: using raw pointers as const generic parameters is unstable
2019-10-07T20:08:57.2907557Z 29   --> $DIR/feature-gate-const_generics-ptr.rs:5:26
2019-10-07T20:08:57.2907862Z 30    |
2019-10-07T20:08:57.2907862Z 30    |
2019-10-07T20:08:57.2908088Z 31 LL | struct ConstPtr<const P: *const u32>;
2019-10-07T20:08:57.2908482Z 
2019-10-07T20:08:57.2908698Z The actual stderr differed from the expected stderr.
2019-10-07T20:08:57.2908698Z The actual stderr differed from the expected stderr.
2019-10-07T20:08:57.2909237Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-ptr/feature-gate-const_generics-ptr.stderr
2019-10-07T20:08:57.2909795Z To update references, rerun the tests and pass the `--bless` flag
2019-10-07T20:08:57.2910695Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-const_generics-ptr.rs`
2019-10-07T20:08:57.2912179Z error: 1 errors occurred comparing output.
2019-10-07T20:08:57.2912421Z status: exit code: 1
2019-10-07T20:08:57.2912421Z status: exit code: 1
2019-10-07T20:08:57.2913527Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_generics-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-ptr/auxiliary" "-A" "unused"
2019-10-07T20:08:57.2914757Z ------------------------------------------
2019-10-07T20:08:57.2915228Z 
2019-10-07T20:08:57.2915689Z ------------------------------------------
2019-10-07T20:08:57.2915993Z stderr:
2019-10-07T20:08:57.2915993Z stderr:
2019-10-07T20:08:57.2916415Z ------------------------------------------
2019-10-07T20:08:57.2916689Z error[E0658]: const generics are unstable
2019-10-07T20:08:57.2917174Z   --> /checkout/src/test/ui/feature-gates/feature-gate-const_generics-ptr.rs:1:22
2019-10-07T20:08:57.2917462Z    |
2019-10-07T20:08:57.2917700Z LL | struct ConstFn<const F: fn()>;
2019-10-07T20:08:57.2918117Z    |
2019-10-07T20:08:57.2918117Z    |
2019-10-07T20:08:57.2918627Z    = note: for more information, see ***/issues/44580
2019-10-07T20:08:57.2918944Z    = help: add `#![feature(const_generics)]` to the crate attributes to enable
2019-10-07T20:08:57.2919380Z error[E0658]: const generics are unstable
2019-10-07T20:08:57.2919890Z   --> /checkout/src/test/ui/feature-gates/feature-gate-const_generics-ptr.rs:5:23
2019-10-07T20:08:57.2920188Z    |
2019-10-07T20:08:57.2920188Z    |
2019-10-07T20:08:57.2920433Z LL | struct ConstPtr<const P: *const u32>;
2019-10-07T20:08:57.2921301Z    |
2019-10-07T20:08:57.2921301Z    |
2019-10-07T20:08:57.2921913Z    = note: for more information, see ***/issues/44580
2019-10-07T20:08:57.2922256Z    = help: add `#![feature(const_generics)]` to the crate attributes to enable
2019-10-07T20:08:57.2922731Z error[E0658]: using function pointers as const generic parameters is unstable
2019-10-07T20:08:57.2923259Z   --> /checkout/src/test/ui/feature-gates/feature-gate-const_generics-ptr.rs:1:25
2019-10-07T20:08:57.2923613Z    |
2019-10-07T20:08:57.2923613Z    |
2019-10-07T20:08:57.2923856Z LL | struct ConstFn<const F: fn()>;
2019-10-07T20:08:57.2924342Z    |
2019-10-07T20:08:57.2924342Z    |
2019-10-07T20:08:57.2925241Z    = note: for more information, see ***/issues/53020
2019-10-07T20:08:57.2926081Z    = help: add `#![feature(const_compare_raw_pointers)]` to the crate attributes to enable
2019-10-07T20:08:57.2927063Z error[E0658]: using raw pointers as const generic parameters is unstable
2019-10-07T20:08:57.2927607Z   --> /checkout/src/test/ui/feature-gates/feature-gate-const_generics-ptr.rs:5:26
2019-10-07T20:08:57.2927968Z    |
2019-10-07T20:08:57.2927968Z    |
2019-10-07T20:08:57.2928125Z LL | struct ConstPtr<const P: *const u32>;
2019-10-07T20:08:57.2928391Z    |
2019-10-07T20:08:57.2928391Z    |
2019-10-07T20:08:57.2928803Z    = note: for more information, see ***/issues/53020
2019-10-07T20:08:57.2928994Z    = help: add `#![feature(const_compare_raw_pointers)]` to the crate attributes to enable
2019-10-07T20:08:57.2931525Z error: aborting due to 4 previous errors
2019-10-07T20:08:57.2931808Z 
2019-10-07T20:08:57.2932348Z For more information about this error, try `rustc --explain E0658`.
2019-10-07T20:08:57.2932567Z 
---
2019-10-07T20:08:57.2936379Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-07T20:08:57.2936727Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-07T20:08:57.2952525Z 
2019-10-07T20:08:57.2953084Z 
2019-10-07T20:08:57.2955425Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-07T20:08:57.2956068Z 
2019-10-07T20:08:57.2956188Z 
2019-10-07T20:08:57.3009161Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-07T20:08:57.3009271Z Build completed unsuccessfully in 1:05:18
2019-10-07T20:08:57.3009271Z Build completed unsuccessfully in 1:05:18
2019-10-07T20:08:57.3019320Z == clock drift check ==
2019-10-07T20:08:57.3041444Z   local time: Mon Oct  7 20:08:57 UTC 2019
2019-10-07T20:08:57.4005292Z   network time: Mon, 07 Oct 2019 20:08:57 GMT
2019-10-07T20:08:57.4010700Z == end clock drift check ==
2019-10-07T20:08:58.6875219Z ##[error]Bash exited with code '1'.
2019-10-07T20:08:58.6922132Z ##[section]Starting: Checkout
2019-10-07T20:08:58.6924074Z ==============================================================================
2019-10-07T20:08:58.6924137Z Task         : Get sources
2019-10-07T20:08:58.6924190Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
