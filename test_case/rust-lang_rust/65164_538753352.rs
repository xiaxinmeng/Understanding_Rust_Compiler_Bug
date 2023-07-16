plain
2019-10-06T13:24:52.5209187Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T13:24:52.5430690Z ##[command]git config gc.auto 0
2019-10-06T13:24:52.5535013Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T13:24:52.5595577Z ##[command]git config --get-all http.proxy
2019-10-06T13:24:52.5763632Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65164/merge:refs/remotes/pull/65164/merge
---
2019-10-06T14:27:01.4042361Z .................................................................................................... 1500/9109
2019-10-06T14:27:08.4262558Z .................................................................................................... 1600/9109
2019-10-06T14:27:17.5479517Z .................................................................................................... 1700/9109
2019-10-06T14:27:27.0175821Z ........i...............i........................................................................... 1800/9109
2019-10-06T14:27:34.3096936Z ...................................................................................................i 1900/9109
2019-10-06T14:27:50.7456997Z iiii................................................................................................ 2000/9109
2019-10-06T14:27:59.8710657Z .................................................................................................... 2200/9109
2019-10-06T14:28:02.6216327Z .................................................................................................... 2300/9109
2019-10-06T14:28:09.0965807Z .................................................................................................... 2400/9109
2019-10-06T14:28:14.9170217Z .................................................................................................... 2500/9109
---
2019-10-06T14:31:11.0024619Z ........................................................................................i........... 4700/9109
2019-10-06T14:31:18.7840102Z ....i............................................................................................... 4800/9109
2019-10-06T14:31:29.4311830Z .................................................................................................... 4900/9109
2019-10-06T14:31:35.1885128Z .................................................................................................... 5000/9109
2019-10-06T14:31:47.5647456Z ..................................................................................ii.ii............. 5100/9109
2019-10-06T14:31:57.0573500Z .................................................................................................... 5300/9109
2019-10-06T14:32:07.1839092Z .................................................................................................... 5400/9109
2019-10-06T14:32:14.1078775Z ................................................i................................................... 5500/9109
2019-10-06T14:32:21.1745542Z .................................................................................................... 5600/9109
2019-10-06T14:32:21.1745542Z .................................................................................................... 5600/9109
2019-10-06T14:32:31.6913884Z .................................................................................................... 5700/9109
2019-10-06T14:32:39.7593189Z .............................................ii...i..ii...........i................................. 5800/9109
2019-10-06T14:33:05.6189439Z .................................................................................................... 6000/9109
2019-10-06T14:33:14.9741081Z .................................................................................................... 6100/9109
2019-10-06T14:33:14.9741081Z .................................................................................................... 6100/9109
2019-10-06T14:33:27.8577680Z ...................................................i..ii............................................ 6200/9109
2019-10-06T14:33:52.6409156Z .................................................................................................... 6400/9109
2019-10-06T14:33:54.9087696Z ...........i........................................................................................ 6500/9109
2019-10-06T14:33:57.6313162Z ....................................................................................i............... 6600/9109
2019-10-06T14:33:59.8769934Z .................................................................................................... 6700/9109
---
2019-10-06T14:38:11.6507327Z 
2019-10-06T14:38:11.6508748Z 
2019-10-06T14:38:11.6509047Z The actual stderr differed from the expected stderr.
2019-10-06T14:38:11.6509798Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conflicting-repr-hints/conflicting-repr-hints.stderr
2019-10-06T14:38:11.6510449Z To update references, rerun the tests and pass the `--bless` flag
2019-10-06T14:38:11.6511158Z To only update this specific test, also pass `--test-args conflicting-repr-hints.rs`
2019-10-06T14:38:11.6511759Z error: 1 errors occurred comparing output.
2019-10-06T14:38:11.6512027Z status: exit code: 1
2019-10-06T14:38:11.6512027Z status: exit code: 1
2019-10-06T14:38:11.6513196Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conflicting-repr-hints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conflicting-repr-hints" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conflicting-repr-hints/auxiliary" "-A" "unused"
2019-10-06T14:38:11.6514180Z ------------------------------------------
2019-10-06T14:38:11.6514494Z 
2019-10-06T14:38:11.6519517Z ------------------------------------------
2019-10-06T14:38:11.6519713Z stderr:
2019-10-06T14:38:11.6519713Z stderr:
2019-10-06T14:38:11.6519987Z ------------------------------------------
2019-10-06T14:38:11.6520044Z warning[E0566]: conflicting representation hints
2019-10-06T14:38:11.6520311Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:9:8
2019-10-06T14:38:11.6520395Z    |
2019-10-06T14:38:11.6520452Z LL | #[repr(C, u64)] //~ WARNING conflicting representation hints
2019-10-06T14:38:11.6520555Z 
2019-10-06T14:38:11.6520622Z warning[E0566]: conflicting representation hints
2019-10-06T14:38:11.6520896Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:12:8
2019-10-06T14:38:11.6520977Z    |
2019-10-06T14:38:11.6520977Z    |
2019-10-06T14:38:11.6521029Z LL | #[repr(u32, u64)] //~ WARNING conflicting representation hints
2019-10-06T14:38:11.6521088Z    |        ^^^  ^^^
2019-10-06T14:38:11.6521189Z error[E0587]: type has conflicting packed and align representation hints
2019-10-06T14:38:11.6521458Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:19:1
2019-10-06T14:38:11.6521511Z    |
2019-10-06T14:38:11.6521511Z    |
2019-10-06T14:38:11.6521583Z LL | struct F(i32); //~ ERROR type has conflicting packed and align representation hints
2019-10-06T14:38:11.6521673Z 
2019-10-06T14:38:11.6521724Z error[E0587]: type has conflicting packed and align representation hints
2019-10-06T14:38:11.6522007Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:23:1
2019-10-06T14:38:11.6522057Z    |
2019-10-06T14:38:11.6522057Z    |
2019-10-06T14:38:11.6522324Z LL | struct G(i32); //~ ERROR type has conflicting packed and align representation hints
2019-10-06T14:38:11.6522461Z 
2019-10-06T14:38:11.6522513Z error[E0587]: type has conflicting packed and align representation hints
2019-10-06T14:38:11.6522857Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:27:1
2019-10-06T14:38:11.6522908Z    |
2019-10-06T14:38:11.6522908Z    |
2019-10-06T14:38:11.6522962Z LL | struct H(i32); //~ ERROR type has conflicting packed and align representation hints
2019-10-06T14:38:11.6523060Z 
2019-10-06T14:38:11.6523110Z error[E0634]: type has conflicting packed representation hints
2019-10-06T14:38:11.6523375Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:30:1
2019-10-06T14:38:11.6523443Z    |
2019-10-06T14:38:11.6523443Z    |
2019-10-06T14:38:11.6523496Z LL | struct I(i32); //~ ERROR type has conflicting packed representation hints
2019-10-06T14:38:11.6523579Z 
2019-10-06T14:38:11.6523656Z error[E0634]: type has conflicting packed representation hints
2019-10-06T14:38:11.6523925Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:34:1
2019-10-06T14:38:11.6524097Z    |
2019-10-06T14:38:11.6524097Z    |
2019-10-06T14:38:11.6524169Z LL | struct J(i32); //~ ERROR type has conflicting packed representation hints
2019-10-06T14:38:11.6524252Z 
2019-10-06T14:38:11.6524303Z error[E0587]: type has conflicting packed and align representation hints
2019-10-06T14:38:11.6524623Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:40:1
2019-10-06T14:38:11.6524674Z    |
2019-10-06T14:38:11.6524674Z    |
2019-10-06T14:38:11.6524727Z LL | / union X { //~ ERROR type has conflicting packed and align representation hints
2019-10-06T14:38:11.6524801Z LL | |     i: i32
2019-10-06T14:38:11.6524891Z    | |_^
2019-10-06T14:38:11.6524920Z 
2019-10-06T14:38:11.6524988Z error[E0587]: type has conflicting packed and align representation hints
2019-10-06T14:38:11.6525266Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:46:1
2019-10-06T14:38:11.6525266Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:46:1
2019-10-06T14:38:11.6525317Z    |
2019-10-06T14:38:11.6525388Z LL | / union Y { //~ ERROR type has conflicting packed and align representation hints
2019-10-06T14:38:11.6525448Z LL | |     i: i32
2019-10-06T14:38:11.6525552Z    | |_^
2019-10-06T14:38:11.6525581Z 
2019-10-06T14:38:11.6525633Z error[E0587]: type has conflicting packed and align representation hints
2019-10-06T14:38:11.6525904Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:52:1
2019-10-06T14:38:11.6525904Z   --> /checkout/src/test/ui/conflicting-repr-hints.rs:52:1
2019-10-06T14:38:11.6525973Z    |
2019-10-06T14:38:11.6526026Z LL | / union Z { //~ ERROR type has conflicting packed and align representation hints
2019-10-06T14:38:11.6526077Z LL | |     i: i32
2019-10-06T14:38:11.6526182Z    | |_^
2019-10-06T14:38:11.6526211Z 
2019-10-06T14:38:11.6526258Z error: aborting due to 8 previous errors
2019-10-06T14:38:11.6526307Z 
---
2019-10-06T14:38:11.6528472Z 30 
2019-10-06T14:38:11.6528501Z 
2019-10-06T14:38:11.6528530Z 
2019-10-06T14:38:11.6528579Z The actual stderr differed from the expected stderr.
2019-10-06T14:38:11.6529200Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-repr-simd/feature-gate-repr-simd.stderr
2019-10-06T14:38:11.6529541Z To update references, rerun the tests and pass the `--bless` flag
2019-10-06T14:38:11.6529897Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-repr-simd.rs`
2019-10-06T14:38:11.6530011Z error: 1 errors occurred comparing output.
2019-10-06T14:38:11.6530063Z status: exit code: 1
2019-10-06T14:38:11.6530063Z status: exit code: 1
2019-10-06T14:38:11.6530959Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-repr-simd.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-repr-simd" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-repr-simd/auxiliary" "-A" "unused"
2019-10-06T14:38:11.6531359Z ------------------------------------------
2019-10-06T14:38:11.6531562Z 
2019-10-06T14:38:11.6531864Z ------------------------------------------
2019-10-06T14:38:11.6531936Z stderr:
2019-10-06T14:38:11.6531936Z stderr:
2019-10-06T14:38:11.6532202Z ------------------------------------------
2019-10-06T14:38:11.6532262Z error[E0658]: SIMD types are experimental and possibly buggy
2019-10-06T14:38:11.6532565Z   --> /checkout/src/test/ui/feature-gates/feature-gate-repr-simd.rs:1:1
2019-10-06T14:38:11.6532640Z    |
2019-10-06T14:38:11.6532693Z LL | #[repr(simd)] //~ error: SIMD types are experimental
2019-10-06T14:38:11.6532809Z    |
2019-10-06T14:38:11.6532809Z    |
2019-10-06T14:38:11.6533263Z    = note: for more information, see ***/issues/27731
2019-10-06T14:38:11.6533329Z    = help: add `#![feature(repr_simd)]` to the crate attributes to enable
2019-10-06T14:38:11.6533447Z error[E0658]: SIMD types are experimental and possibly buggy
2019-10-06T14:38:11.6533772Z   --> /checkout/src/test/ui/feature-gates/feature-gate-repr-simd.rs:5:1
2019-10-06T14:38:11.6533856Z    |
2019-10-06T14:38:11.6533856Z    |
2019-10-06T14:38:11.6533909Z LL | #[repr(simd)] //~ error: SIMD types are experimental
2019-10-06T14:38:11.6534024Z    |
2019-10-06T14:38:11.6534024Z    |
2019-10-06T14:38:11.6534371Z    = note: for more information, see ***/issues/27731
2019-10-06T14:38:11.6534437Z    = help: add `#![feature(repr_simd)]` to the crate attributes to enable
2019-10-06T14:38:11.6534541Z warning[E0566]: conflicting representation hints
2019-10-06T14:38:11.6534853Z   --> /checkout/src/test/ui/feature-gates/feature-gate-repr-simd.rs:4:8
2019-10-06T14:38:11.6534906Z    |
2019-10-06T14:38:11.6534906Z    |
2019-10-06T14:38:11.6534976Z LL | #[repr(C)] //~ warn: conflicting representation hints
2019-10-06T14:38:11.6535028Z    |        ^
2019-10-06T14:38:11.6535083Z LL | #[repr(simd)] //~ error: SIMD types are experimental
2019-10-06T14:38:11.6535193Z 
2019-10-06T14:38:11.6535241Z error: aborting due to 2 previous errors
2019-10-06T14:38:11.6535282Z 
2019-10-06T14:38:11.6535349Z Some errors have detailed explanations: E0566, E0658.
---
2019-10-06T14:38:11.6544143Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-06T14:38:11.6544405Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-06T14:38:11.6565333Z 
2019-10-06T14:38:11.6565460Z 
2019-10-06T14:38:11.6567160Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-06T14:38:11.6568168Z 
2019-10-06T14:38:11.6568198Z 
2019-10-06T14:38:11.6575401Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-06T14:38:11.6575495Z Build completed unsuccessfully in 1:06:12
2019-10-06T14:38:11.6575495Z Build completed unsuccessfully in 1:06:12
2019-10-06T14:38:11.6631575Z == clock drift check ==
2019-10-06T14:38:11.6645293Z   local time: Sun Oct  6 14:38:11 UTC 2019
2019-10-06T14:38:11.8279763Z   network time: Sun, 06 Oct 2019 14:38:11 GMT
2019-10-06T14:38:11.8282800Z == end clock drift check ==
2019-10-06T14:38:12.8702692Z ##[error]Bash exited with code '1'.
2019-10-06T14:38:12.8745130Z ##[section]Starting: Checkout
2019-10-06T14:38:12.8747041Z ==============================================================================
2019-10-06T14:38:12.8747216Z Task         : Get sources
2019-10-06T14:38:12.8747277Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
