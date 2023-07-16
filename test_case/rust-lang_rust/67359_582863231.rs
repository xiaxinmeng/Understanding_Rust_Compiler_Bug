plain
2020-02-06T10:18:11.6653638Z ========================== Starting Command Output ===========================
2020-02-06T10:18:11.6655816Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/882af255-a31c-4a23-8a1f-efc35a1e55de.sh
2020-02-06T10:18:11.6655852Z 
2020-02-06T10:18:11.6658292Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-06T10:18:11.6663741Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67359/merge to s
2020-02-06T10:18:11.6665238Z Task         : Get sources
2020-02-06T10:18:11.6665295Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T10:18:11.6665325Z Version      : 1.0.0
2020-02-06T10:18:11.6665353Z Author       : Microsoft
---
2020-02-06T10:18:12.5458386Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-06T10:18:12.5557798Z ##[command]git config gc.auto 0
2020-02-06T10:18:12.5629985Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-06T10:18:12.5689406Z ##[command]git config --get-all http.proxy
2020-02-06T10:18:12.5847548Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67359/merge:refs/remotes/pull/67359/merge
---
2020-02-06T11:18:32.3913752Z .................................................................................................... 1700/9590
2020-02-06T11:18:37.7020777Z .................................................................................................... 1800/9590
2020-02-06T11:18:50.7340056Z .............................i...................................................................... 1900/9590
2020-02-06T11:18:58.3151528Z .................................................................................................... 2000/9590
2020-02-06T11:19:13.0977882Z ...................iiiii............................................................................ 2100/9590
2020-02-06T11:19:23.7541362Z .................................................................................................... 2300/9590
2020-02-06T11:19:26.4536044Z .................................................................................................... 2400/9590
2020-02-06T11:19:31.7412634Z .................................................................................................... 2500/9590
2020-02-06T11:19:53.1599317Z .................................................................................................... 2600/9590
---
2020-02-06T11:22:35.6971976Z ..............................................................i...............i..................... 4900/9590
2020-02-06T11:22:43.3970822Z .................................................................................................... 5000/9590
2020-02-06T11:22:51.6507304Z .................................................................................................... 5100/9590
2020-02-06T11:22:56.3381808Z .....i.............................................................................................. 5200/9590
2020-02-06T11:23:07.7973119Z ...............................................................................ii.ii........i...i... 5300/9590
2020-02-06T11:23:16.4664440Z .................i.................................................................................. 5500/9590
2020-02-06T11:23:25.9470076Z .................................................................................................... 5600/9590
2020-02-06T11:23:32.8673726Z ...................................................................i................................ 5700/9590
2020-02-06T11:23:40.1744852Z .................................................................................................... 5800/9590
2020-02-06T11:23:40.1744852Z .................................................................................................... 5800/9590
2020-02-06T11:23:47.4655943Z .................................................................................................... 5900/9590
2020-02-06T11:23:57.2379277Z ..........................................................ii...i..ii...........i.................... 6000/9590
2020-02-06T11:24:19.2811223Z .................................................................................................... 6200/9590
2020-02-06T11:24:27.1369063Z .................................................................................................... 6300/9590
2020-02-06T11:24:27.1369063Z .................................................................................................... 6300/9590
2020-02-06T11:24:35.3302379Z ......................................................................................i..ii......... 6400/9590
2020-02-06T11:25:00.0159369Z .................................................................................................... 6600/9590
2020-02-06T11:25:09.9457569Z ........................................................................i........................... 6700/9590
2020-02-06T11:25:12.3005559Z .................................................................................................... 6800/9590
2020-02-06T11:25:14.7661133Z ..........................................................................i......................... 6900/9590
---
2020-02-06T11:27:00.8408172Z .................................................................................................... 7600/9590
2020-02-06T11:27:05.8959011Z .................................................................................................... 7700/9590
2020-02-06T11:27:13.2207097Z .................................................................................................... 7800/9590
2020-02-06T11:27:22.4399433Z .................................................................................................... 7900/9590
2020-02-06T11:27:30.2901968Z .....................................iiiiiii.i...................................................... 8000/9590
2020-02-06T11:27:45.9248137Z .................................................................................................... 8200/9590
2020-02-06T11:27:54.2642310Z .................................................................................................... 8300/9590
2020-02-06T11:28:08.7016325Z .................................................................................................... 8400/9590
2020-02-06T11:28:16.0550799Z .................................................................................................... 8500/9590
---
2020-02-06T11:30:48.1351670Z  finished in 7.909
2020-02-06T11:30:48.1423831Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T11:30:48.3533855Z 
2020-02-06T11:30:48.3534296Z running 172 tests
2020-02-06T11:30:52.1311532Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/172
2020-02-06T11:30:54.1010753Z ...i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-02-06T11:30:54.1012400Z 
2020-02-06T11:30:54.1018856Z  finished in 5.959
2020-02-06T11:30:54.1213666Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T11:30:54.3014946Z 
---
2020-02-06T11:30:56.4533062Z  finished in 2.332
2020-02-06T11:30:56.4739906Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T11:30:56.6386682Z 
2020-02-06T11:30:56.6386808Z running 9 tests
2020-02-06T11:30:56.6389141Z iiiiiiiii
2020-02-06T11:30:56.6390491Z 
2020-02-06T11:30:56.6391052Z  finished in 0.165
2020-02-06T11:30:56.6597743Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T11:30:56.8515930Z 
---
2020-02-06T11:31:18.5486539Z  finished in 21.889
2020-02-06T11:31:18.5719439Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T11:31:18.7650902Z 
2020-02-06T11:31:18.7651118Z running 116 tests
2020-02-06T11:31:33.1751085Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-06T11:31:35.1218271Z ....iiii.....ii.
2020-02-06T11:31:35.1219258Z 
2020-02-06T11:31:35.1224019Z  finished in 16.550
2020-02-06T11:31:35.1230202Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T11:31:35.1230573Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-06T11:32:16.5443825Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2020-02-06T11:32:16.5444026Z diff of stderr:
2020-02-06T11:32:16.5444150Z 
2020-02-06T11:32:16.5444388Z 42    |
2020-02-06T11:32:16.5445269Z 43    = note: for more information, see ***/issues/27812
2020-02-06T11:32:16.5445472Z 44    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-02-06T11:32:16.5445921Z -    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T11:32:16.5446377Z +    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T11:32:16.5446727Z 47 error: aborting due to 5 previous errors
2020-02-06T11:32:16.5446867Z 48 
2020-02-06T11:32:16.5446989Z 
2020-02-06T11:32:16.5447126Z 
2020-02-06T11:32:16.5447126Z 
2020-02-06T11:32:16.5447270Z The actual stderr differed from the expected stderr.
2020-02-06T11:32:16.5447720Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2020-02-06T11:32:16.5448243Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T11:32:16.5448674Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2020-02-06T11:32:16.5448995Z error: 1 errors occurred comparing output.
2020-02-06T11:32:16.5449137Z status: exit code: 1
2020-02-06T11:32:16.5449137Z status: exit code: 1
2020-02-06T11:32:16.5450309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2020-02-06T11:32:16.5453240Z ------------------------------------------
2020-02-06T11:32:16.5453429Z 
2020-02-06T11:32:16.5453937Z ------------------------------------------
2020-02-06T11:32:16.5454342Z stderr:
2020-02-06T11:32:16.5454342Z stderr:
2020-02-06T11:32:16.5454718Z ------------------------------------------
2020-02-06T11:32:16.5455248Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-02-06T11:32:16.5455849Z    |
2020-02-06T11:32:16.5456012Z LL | extern crate rustc_data_structures;
2020-02-06T11:32:16.5456154Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-06T11:32:16.5456294Z    |
2020-02-06T11:32:16.5456294Z    |
2020-02-06T11:32:16.5456749Z    = note: for more information, see ***/issues/27812
2020-02-06T11:32:16.5456972Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-02-06T11:32:16.5457132Z 
2020-02-06T11:32:16.5457649Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-02-06T11:32:16.5458244Z    |
2020-02-06T11:32:16.5458384Z LL | extern crate rustc;
2020-02-06T11:32:16.5458521Z    | ^^^^^^^^^^^^^^^^^^^
2020-02-06T11:32:16.5458678Z    |
2020-02-06T11:32:16.5458678Z    |
2020-02-06T11:32:16.5459092Z    = note: for more information, see ***/issues/27812
2020-02-06T11:32:16.5459321Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-02-06T11:32:16.5459462Z 
2020-02-06T11:32:16.5460026Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-02-06T11:32:16.5460701Z    |
2020-02-06T11:32:16.5460851Z LL | extern crate rustc_macros;
2020-02-06T11:32:16.5461028Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-06T11:32:16.5461176Z    |
2020-02-06T11:32:16.5461176Z    |
2020-02-06T11:32:16.5461643Z    = note: for more information, see ***/issues/27812
2020-02-06T11:32:16.5461841Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-02-06T11:32:16.5461978Z 
2020-02-06T11:32:16.5462839Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-02-06T11:32:16.5463482Z    |
2020-02-06T11:32:16.5463654Z LL | use rustc_macros::HashStable;
2020-02-06T11:32:16.5463804Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-06T11:32:16.5463951Z    |
2020-02-06T11:32:16.5463951Z    |
2020-02-06T11:32:16.5464432Z    = note: for more information, see ***/issues/27812
2020-02-06T11:32:16.5464639Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-02-06T11:32:16.5464798Z 
2020-02-06T11:32:16.5465450Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-02-06T11:32:16.5466784Z    |
2020-02-06T11:32:16.5466832Z LL | #[derive(HashStable)]
2020-02-06T11:32:16.5466879Z    |          ^^^^^^^^^^
2020-02-06T11:32:16.5466945Z    |
2020-02-06T11:32:16.5466945Z    |
2020-02-06T11:32:16.5467273Z    = note: for more information, see ***/issues/27812
2020-02-06T11:32:16.5467333Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-02-06T11:32:16.5467811Z    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T11:32:16.5467979Z error: aborting due to 5 previous errors
2020-02-06T11:32:16.5468034Z 
2020-02-06T11:32:16.5468337Z For more information about this error, try `rustc --explain E0658`.
2020-02-06T11:32:16.5468375Z 
---
2020-02-06T11:32:16.5469545Z test result: FAILED. 62 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-06T11:32:16.5469582Z 
2020-02-06T11:32:16.5469606Z 
2020-02-06T11:32:16.5469631Z 
2020-02-06T11:32:16.5471486Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-06T11:32:16.5471741Z 
2020-02-06T11:32:16.5471770Z 
2020-02-06T11:32:16.5471816Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-06T11:32:16.5471865Z Build completed unsuccessfully in 1:07:32
2020-02-06T11:32:16.5471865Z Build completed unsuccessfully in 1:07:32
2020-02-06T11:32:16.5510174Z == clock drift check ==
2020-02-06T11:32:16.5531246Z   local time: Thu Feb  6 11:32:16 UTC 2020
2020-02-06T11:32:17.1139981Z   network time: Thu, 06 Feb 2020 11:32:17 GMT
2020-02-06T11:32:17.1140087Z == end clock drift check ==
2020-02-06T11:32:18.1078282Z 
2020-02-06T11:32:18.1236027Z ##[error]Bash exited with code '1'.
2020-02-06T11:32:18.1248938Z ##[section]Finishing: Run build
2020-02-06T11:32:18.1275331Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67359/merge to s
2020-02-06T11:32:18.1277183Z Task         : Get sources
2020-02-06T11:32:18.1277247Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T11:32:18.1277318Z Version      : 1.0.0
2020-02-06T11:32:18.1277361Z Author       : Microsoft
2020-02-06T11:32:18.1277361Z Author       : Microsoft
2020-02-06T11:32:18.1277407Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-06T11:32:18.1277476Z ==============================================================================
2020-02-06T11:32:18.5781482Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-06T11:32:18.5827803Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67359/merge to s
2020-02-06T11:32:18.5950346Z Cleaning up task key
2020-02-06T11:32:18.5951134Z Start cleaning up orphan processes.
2020-02-06T11:32:18.6311652Z Terminate orphan process: pid (4784) (python)
2020-02-06T11:32:18.6348951Z ##[section]Finishing: Finalize Job
