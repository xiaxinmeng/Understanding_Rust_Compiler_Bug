plain
2019-09-21T09:12:14.6919158Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T09:12:14.7110826Z ##[command]git config gc.auto 0
2019-09-21T09:12:14.7185403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T09:12:14.7249730Z ##[command]git config --get-all http.proxy
2019-09-21T09:12:14.7421093Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64582/merge:refs/remotes/pull/64582/merge
---
2019-09-21T10:15:45.0267080Z .................................................................................................... 1500/9029
2019-09-21T10:15:51.0622017Z .................................................................................................... 1600/9029
2019-09-21T10:16:03.9656906Z .....................................................................i...............i.............. 1700/9029
2019-09-21T10:16:10.9618060Z .................................................................................................... 1800/9029
2019-09-21T10:16:26.8384908Z ............................................................iiiii................................... 1900/9029
2019-09-21T10:16:39.1523314Z .................................................................................................... 2100/9029
2019-09-21T10:16:41.7428185Z .................................................................................................... 2200/9029
2019-09-21T10:16:45.0847162Z .................................................................................................... 2300/9029
2019-09-21T10:16:54.0074390Z .................................................................................................... 2400/9029
---
2019-09-21T10:19:57.2440908Z ................................................i...............i................................... 4700/9029
2019-09-21T10:20:07.0534308Z .................................................................................................... 4800/9029
2019-09-21T10:20:15.2767170Z .................................................................................................... 4900/9029
2019-09-21T10:20:25.1487273Z .................................................................................................... 5000/9029
2019-09-21T10:20:33.2894910Z ................................ii.ii............................................................... 5100/9029
2019-09-21T10:20:43.1014920Z .................................................................................................... 5300/9029
2019-09-21T10:20:53.9858015Z ................................................................................................i... 5400/9029
2019-09-21T10:21:02.8943880Z .................................................................................................... 5500/9029
2019-09-21T10:21:07.8952839Z .................................................................................................... 5600/9029
2019-09-21T10:21:07.8952839Z .................................................................................................... 5600/9029
2019-09-21T10:21:19.0394169Z ...........................................................................................ii...i..i 5700/9029
2019-09-21T10:21:34.0961825Z i...........i....................................................................................... 5800/9029
2019-09-21T10:21:55.3817260Z .................................................................................................... 6000/9029
2019-09-21T10:21:55.3817260Z .................................................................................................... 6000/9029
2019-09-21T10:22:03.8386368Z .............................................................................................i..ii.. 6100/9029
2019-09-21T10:22:33.6494645Z .................................................................................................... 6300/9029
2019-09-21T10:22:38.4095361Z ....................................................i............................................... 6400/9029
2019-09-21T10:22:40.7428341Z ...................................................F................................................ 6500/9029
2019-09-21T10:22:43.3438646Z ........................i........................................................................... 6600/9029
---
2019-09-21T10:27:02.7843636Z 
2019-09-21T10:27:02.7844674Z ---- [ui] ui/codemap_tests/unicode.rs stdout ----
2019-09-21T10:27:02.7844821Z diff of stderr:
2019-09-21T10:27:02.7844858Z 
2019-09-21T10:27:02.7845175Z 4 LL | extern "路濫狼á́́" fn foo() {}
2019-09-21T10:27:02.7845235Z 5    |        ^^^^^^^^^ invalid ABI
2019-09-21T10:27:02.7845304Z 6    |
2019-09-21T10:27:02.7845791Z -    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-21T10:27:02.7846274Z +    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-21T10:27:02.7846439Z 9 error: aborting due to previous error
2019-09-21T10:27:02.7846507Z 10 
2019-09-21T10:27:02.7846540Z 
2019-09-21T10:27:02.7846570Z 
2019-09-21T10:27:02.7846570Z 
2019-09-21T10:27:02.7846622Z The actual stderr differed from the expected stderr.
2019-09-21T10:27:02.7846974Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/unicode.stderr
2019-09-21T10:27:02.7847253Z To update references, rerun the tests and pass the `--bless` flag
2019-09-21T10:27:02.7848071Z To only update this specific test, also pass `--test-args codemap_tests/unicode.rs`
2019-09-21T10:27:02.7848312Z error: 1 errors occurred comparing output.
2019-09-21T10:27:02.7848387Z status: exit code: 1
2019-09-21T10:27:02.7848387Z status: exit code: 1
2019-09-21T10:27:02.7849217Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/unicode.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/auxiliary" "-A" "unused"
2019-09-21T10:27:02.7849599Z ------------------------------------------
2019-09-21T10:27:02.7849636Z 
2019-09-21T10:27:02.7849876Z ------------------------------------------
2019-09-21T10:27:02.7849948Z stderr:
2019-09-21T10:27:02.7849948Z stderr:
2019-09-21T10:27:02.7850196Z ------------------------------------------
2019-09-21T10:27:02.7850455Z error[E0703]: invalid ABI: found `路濫狼á́́`
2019-09-21T10:27:02.7850794Z    |
2019-09-21T10:27:02.7850794Z    |
2019-09-21T10:27:02.7851073Z LL | extern "路濫狼á́́" fn foo() {} //~ ERROR invalid ABI
2019-09-21T10:27:02.7851175Z    |        ^^^^^^^^^ invalid ABI
2019-09-21T10:27:02.7851480Z    |
2019-09-21T10:27:02.7851992Z    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-21T10:27:02.7852125Z error: aborting due to previous error
2019-09-21T10:27:02.7852158Z 
2019-09-21T10:27:02.7852188Z 
2019-09-21T10:27:02.7852449Z ------------------------------------------
2019-09-21T10:27:02.7852449Z ------------------------------------------
2019-09-21T10:27:02.7852485Z 
2019-09-21T10:27:02.7852514Z 
2019-09-21T10:27:02.7852756Z ---- [ui] ui/parser/issue-8537.rs stdout ----
2019-09-21T10:27:02.7852961Z diff of stderr:
2019-09-21T10:27:02.7853001Z 
2019-09-21T10:27:02.7853266Z 4 LL |   "invalid-ab_isize"
2019-09-21T10:27:02.7853322Z 5    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
2019-09-21T10:27:02.7853392Z 6    |
2019-09-21T10:27:02.7853835Z -    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-21T10:27:02.7854318Z +    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-21T10:27:02.7854462Z 9 error: aborting due to previous error
2019-09-21T10:27:02.7854529Z 10 
2019-09-21T10:27:02.7854564Z 
2019-09-21T10:27:02.7854594Z 
2019-09-21T10:27:02.7854594Z 
2019-09-21T10:27:02.7854646Z The actual stderr differed from the expected stderr.
2019-09-21T10:27:02.7854999Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/issue-8537.stderr
2019-09-21T10:27:02.7855279Z To update references, rerun the tests and pass the `--bless` flag
2019-09-21T10:27:02.7855569Z To only update this specific test, also pass `--test-args parser/issue-8537.rs`
2019-09-21T10:27:02.7855686Z error: 1 errors occurred comparing output.
2019-09-21T10:27:02.7855736Z status: exit code: 1
2019-09-21T10:27:02.7855736Z status: exit code: 1
2019-09-21T10:27:02.7856552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-8537.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/auxiliary" "-A" "unused"
2019-09-21T10:27:02.7856920Z ------------------------------------------
2019-09-21T10:27:02.7856958Z 
2019-09-21T10:27:02.7857198Z ------------------------------------------
2019-09-21T10:27:02.7857248Z stderr:
2019-09-21T10:27:02.7857248Z stderr:
2019-09-21T10:27:02.7857919Z ------------------------------------------
2019-09-21T10:27:02.7858241Z error[E0703]: invalid ABI: found `invalid-ab_isize`
2019-09-21T10:27:02.7858499Z   --> /checkout/src/test/ui/parser/issue-8537.rs:2:3
2019-09-21T10:27:02.7858574Z    |
2019-09-21T10:27:02.7858850Z LL |   "invalid-ab_isize" //~ ERROR invalid ABI
2019-09-21T10:27:02.7858912Z    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
2019-09-21T10:27:02.7858963Z    |
2019-09-21T10:27:02.7859810Z    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-21T10:27:02.7859978Z error: aborting due to previous error
2019-09-21T10:27:02.7860014Z 
2019-09-21T10:27:02.7860046Z 
2019-09-21T10:27:02.7860387Z ------------------------------------------
2019-09-21T10:27:02.7860387Z ------------------------------------------
2019-09-21T10:27:02.7860427Z 
2019-09-21T10:27:02.7860477Z 
2019-09-21T10:27:02.7860744Z ---- [ui] ui/reify-intrinsic.rs stdout ----
2019-09-21T10:27:02.7860981Z diff of stderr:
2019-09-21T10:27:02.7861015Z 
2019-09-21T10:27:02.7861101Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-21T10:27:02.7861162Z 9    |
2019-09-21T10:27:02.7861519Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-21T10:27:02.7861878Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-21T10:27:02.7862207Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-21T10:27:02.7862377Z 12 
2019-09-21T10:27:02.7862850Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-21T10:27:02.7863265Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-21T10:27:02.7863554Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-21T10:27:02.7863629Z 15    |
2019-09-21T10:27:02.7863945Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-21T10:27:02.7864021Z 
2019-09-21T10:27:02.7864096Z The actual stderr differed from the expected stderr.
2019-09-21T10:27:02.7864438Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-21T10:27:02.7864438Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-21T10:27:02.7864731Z To update references, rerun the tests and pass the `--bless` flag
2019-09-21T10:27:02.7865064Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-21T10:27:02.7865161Z error: 1 errors occurred comparing output.
2019-09-21T10:27:02.7865234Z status: exit code: 1
2019-09-21T10:27:02.7865234Z status: exit code: 1
2019-09-21T10:27:02.7866037Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-21T10:27:02.7866436Z ------------------------------------------
2019-09-21T10:27:02.7866476Z 
2019-09-21T10:27:02.7866755Z ------------------------------------------
2019-09-21T10:27:02.7866830Z stderr:
2019-09-21T10:27:02.7866830Z stderr:
2019-09-21T10:27:02.7867092Z ------------------------------------------
2019-09-21T10:27:02.7867152Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-21T10:27:02.7867750Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-09-21T10:27:02.7867824Z    |
2019-09-21T10:27:02.7868172Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-21T10:27:02.7868343Z    |                                                                |
2019-09-21T10:27:02.7868407Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-21T10:27:02.7868502Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-21T10:27:02.7868562Z    |
2019-09-21T10:27:02.7868562Z    |
2019-09-21T10:27:02.7868877Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-21T10:27:02.7869185Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-21T10:27:02.7869228Z 
2019-09-21T10:27:02.7869600Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-21T10:27:02.7870170Z    |
2019-09-21T10:27:02.7870170Z    |
2019-09-21T10:27:02.7870458Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-21T10:27:02.7870582Z 
2019-09-21T10:27:02.7870631Z error: aborting due to 2 previous errors
2019-09-21T10:27:02.7870682Z 
2019-09-21T10:27:02.7870733Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-21T10:27:02.7890265Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-21T10:27:02.7890400Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-21T10:27:02.7917094Z 
2019-09-21T10:27:02.7919002Z 
2019-09-21T10:27:02.7923168Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-21T10:27:02.7925045Z 
2019-09-21T10:27:02.7925426Z 
2019-09-21T10:27:02.7925857Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-21T10:27:02.7926267Z Build completed unsuccessfully in 1:07:40
2019-09-21T10:27:02.7926267Z Build completed unsuccessfully in 1:07:40
2019-09-21T10:27:02.7981978Z == clock drift check ==
2019-09-21T10:27:02.8014825Z   local time: Sat Sep 21 10:27:02 UTC 2019
2019-09-21T10:27:02.9506965Z   network time: Sat, 21 Sep 2019 10:27:02 GMT
2019-09-21T10:27:02.9514792Z == end clock drift check ==
2019-09-21T10:27:04.1051814Z ##[error]Bash exited with code '1'.
2019-09-21T10:27:04.1120985Z ##[section]Starting: Checkout
2019-09-21T10:27:04.1123190Z ==============================================================================
2019-09-21T10:27:04.1123244Z Task         : Get sources
2019-09-21T10:27:04.1123288Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
