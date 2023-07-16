plain
2019-09-09T18:59:29.6831564Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T18:59:29.7001962Z ##[command]git config gc.auto 0
2019-09-09T18:59:29.7055287Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T18:59:29.7117325Z ##[command]git config --get-all http.proxy
2019-09-09T18:59:29.7239800Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-09T19:59:58.4829728Z .................................................................................................... 1500/9003
2019-09-09T20:00:04.0246275Z .................................................................................................... 1600/9003
2019-09-09T20:00:16.3395555Z .....................................................i...............i.............................. 1700/9003
2019-09-09T20:00:23.8143004Z .................................................................................................... 1800/9003
2019-09-09T20:00:37.8552863Z ............................................iiiii................................................... 1900/9003
2019-09-09T20:00:48.2403187Z .................................................................................................... 2100/9003
2019-09-09T20:00:50.7263018Z .................................................................................................... 2200/9003
2019-09-09T20:00:54.3465173Z .................................................................................................... 2300/9003
2019-09-09T20:01:01.7130879Z .................................................................................................... 2400/9003
---
2019-09-09T20:03:56.8473375Z ...............................i...............i.................................................... 4700/9003
2019-09-09T20:04:08.2893420Z .................................................................................................... 4800/9003
2019-09-09T20:04:14.3198703Z .................................................................................................... 4900/9003
2019-09-09T20:04:24.7990983Z .................................................................................................... 5000/9003
2019-09-09T20:04:30.4467986Z .............ii.ii.................................................................................. 5100/9003
2019-09-09T20:04:40.6653577Z .................................................................................................... 5300/9003
2019-09-09T20:04:50.2611189Z ............................................................................i....................... 5400/9003
2019-09-09T20:04:57.3395051Z .................................................................................................... 5500/9003
2019-09-09T20:05:03.2370213Z .................................................................................................... 5600/9003
2019-09-09T20:05:03.2370213Z .................................................................................................... 5600/9003
2019-09-09T20:05:13.4073873Z ......................................................................ii...i..ii...........i........ 5700/9003
2019-09-09T20:05:36.5767976Z .................................................................................................... 5900/9003
2019-09-09T20:05:45.4106109Z .................................................................................................... 6000/9003
2019-09-09T20:05:45.4106109Z .................................................................................................... 6000/9003
2019-09-09T20:05:51.1744483Z ........................................................................i..ii....................... 6100/9003
2019-09-09T20:06:18.4916697Z .................................................................................................... 6300/9003
2019-09-09T20:06:20.3393123Z ...............................i.................................................................... 6400/9003
2019-09-09T20:06:22.2001819Z .................................................................................................... 6500/9003
2019-09-09T20:06:24.5716783Z ...i................................................................................................ 6600/9003
---
2019-09-09T20:10:55.6195819Z  finished in 18.510
2019-09-09T20:10:55.6202618Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T20:10:55.7742845Z 
2019-09-09T20:10:55.7743555Z running 150 tests
2019-09-09T20:10:59.6237344Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T20:11:00.8483102Z ..iiii..............i.........iii.i.......ii......
2019-09-09T20:11:00.8503782Z 
2019-09-09T20:11:00.8506877Z  finished in 5.230
2019-09-09T20:11:00.8708839Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T20:11:01.0165922Z 
---
2019-09-09T20:11:02.9886543Z  finished in 2.118
2019-09-09T20:11:03.0053001Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T20:11:03.1721609Z 
2019-09-09T20:11:03.1721823Z running 9 tests
2019-09-09T20:11:03.1722725Z iiiiiiiii
2019-09-09T20:11:03.1724020Z 
2019-09-09T20:11:03.1726542Z  finished in 0.166
2019-09-09T20:11:03.1887686Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T20:11:03.3685647Z 
---
2019-09-09T20:11:20.7417864Z  finished in 17.553
2019-09-09T20:11:20.7581696Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T20:11:20.9045061Z 
2019-09-09T20:11:20.9045258Z running 123 tests
2019-09-09T20:11:43.3671235Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-09T20:11:47.7987819Z i.i.i......iii.i.....ii
2019-09-09T20:11:47.7989807Z 
2019-09-09T20:11:47.7994354Z  finished in 27.040
2019-09-09T20:11:47.8000393Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T20:11:47.8001699Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-09T20:12:48.4453246Z 
2019-09-09T20:12:48.4453766Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2019-09-09T20:12:48.4453974Z diff of stderr:
2019-09-09T20:12:48.4454133Z 
2019-09-09T20:12:48.4454694Z 43    = note: for more information, see ***/issues/27812
2019-09-09T20:12:48.4454923Z 44    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T20:12:48.4455601Z - error: aborting due to 6 previous errors
2019-09-09T20:12:48.4455952Z + error: aborting due to 5 previous errors
2019-09-09T20:12:48.4457954Z 47 
2019-09-09T20:12:48.4458511Z - Some errors have detailed explanations: E0601, E0658.
2019-09-09T20:12:48.4458511Z - Some errors have detailed explanations: E0601, E0658.
2019-09-09T20:12:48.4458903Z - For more information about an error, try `rustc --explain E0601`.
2019-09-09T20:12:48.4459243Z + For more information about this error, try `rustc --explain E0658`.
2019-09-09T20:12:48.4459389Z 50 
2019-09-09T20:12:48.4459512Z 
2019-09-09T20:12:48.4459604Z 
2019-09-09T20:12:48.4459718Z The actual stderr differed from the expected stderr.
2019-09-09T20:12:48.4460135Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-09-09T20:12:48.4460641Z To update references, rerun the tests and pass the `--bless` flag
2019-09-09T20:12:48.4461136Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2019-09-09T20:12:48.4461412Z error: 1 errors occurred comparing output.
2019-09-09T20:12:48.4461634Z status: exit code: 1
2019-09-09T20:12:48.4461634Z status: exit code: 1
2019-09-09T20:12:48.4462740Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-09-09T20:12:48.4463348Z ------------------------------------------
2019-09-09T20:12:48.4463519Z 
2019-09-09T20:12:48.4463899Z ------------------------------------------
2019-09-09T20:12:48.4464072Z stderr:
2019-09-09T20:12:48.4464072Z stderr:
2019-09-09T20:12:48.4464412Z ------------------------------------------
2019-09-09T20:12:48.4464972Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T20:12:48.4465938Z    |
2019-09-09T20:12:48.4466051Z LL | extern crate rustc_data_structures;
2019-09-09T20:12:48.4466161Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T20:12:48.4466289Z    |
2019-09-09T20:12:48.4466289Z    |
2019-09-09T20:12:48.4466655Z    = note: for more information, see ***/issues/27812
2019-09-09T20:12:48.4466845Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T20:12:48.4466958Z 
2019-09-09T20:12:48.4467408Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T20:12:48.4468087Z    |
2019-09-09T20:12:48.4468206Z LL | extern crate rustc;
2019-09-09T20:12:48.4468347Z    | ^^^^^^^^^^^^^^^^^^^
2019-09-09T20:12:48.4468460Z    |
2019-09-09T20:12:48.4468460Z    |
2019-09-09T20:12:48.4468873Z    = note: for more information, see ***/issues/27812
2019-09-09T20:12:48.4469043Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T20:12:48.4469150Z 
2019-09-09T20:12:48.4470592Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T20:12:48.4472071Z    |
2019-09-09T20:12:48.4472287Z LL | extern crate rustc_macros;
2019-09-09T20:12:48.4472431Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T20:12:48.4472567Z    |
2019-09-09T20:12:48.4472567Z    |
2019-09-09T20:12:48.4473081Z    = note: for more information, see ***/issues/27812
2019-09-09T20:12:48.4473285Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T20:12:48.4473458Z 
2019-09-09T20:12:48.4473984Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T20:12:48.4474604Z    |
2019-09-09T20:12:48.4474745Z LL | use rustc_macros::HashStable;
2019-09-09T20:12:48.4474884Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T20:12:48.4475195Z    |
2019-09-09T20:12:48.4475195Z    |
2019-09-09T20:12:48.4475869Z    = note: for more information, see ***/issues/27812
2019-09-09T20:12:48.4477051Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T20:12:48.4477098Z 
2019-09-09T20:12:48.4477510Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T20:12:48.4477806Z    |
2019-09-09T20:12:48.4477842Z LL | #[derive(HashStable)]
2019-09-09T20:12:48.4477897Z    |          ^^^^^^^^^^
2019-09-09T20:12:48.4477932Z    |
2019-09-09T20:12:48.4477932Z    |
2019-09-09T20:12:48.4478183Z    = note: for more information, see ***/issues/27812
2019-09-09T20:12:48.4478253Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T20:12:48.4478323Z error: aborting due to 5 previous errors
2019-09-09T20:12:48.4478350Z 
2019-09-09T20:12:48.4478609Z For more information about this error, try `rustc --explain E0658`.
2019-09-09T20:12:48.4478650Z 
---
2019-09-09T20:12:48.4479740Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T20:12:48.4479808Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T20:12:48.4479837Z 
2019-09-09T20:12:48.4479860Z 
2019-09-09T20:12:48.4481441Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T20:12:48.4482544Z 
2019-09-09T20:12:48.4482575Z 
2019-09-09T20:12:48.4482645Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T20:12:48.4482707Z Build completed unsuccessfully in 1:06:27
2019-09-09T20:12:48.4482707Z Build completed unsuccessfully in 1:06:27
2019-09-09T20:12:48.4512682Z == clock drift check ==
2019-09-09T20:12:48.4528885Z   local time: Mon Sep  9 20:12:48 UTC 2019
2019-09-09T20:12:48.5469144Z   network time: Mon, 09 Sep 2019 20:12:48 GMT
2019-09-09T20:12:48.5473476Z == end clock drift check ==
2019-09-09T20:12:49.4690704Z ##[error]Bash exited with code '1'.
2019-09-09T20:12:49.4731351Z ##[section]Starting: Checkout
2019-09-09T20:12:49.4733422Z ==============================================================================
2019-09-09T20:12:49.4733497Z Task         : Get sources
2019-09-09T20:12:49.4733546Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
