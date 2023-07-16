plain
2019-12-23T16:33:55.7709615Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T16:33:55.7893101Z ##[command]git config gc.auto 0
2019-12-23T16:33:55.7958134Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T16:33:55.8003961Z ##[command]git config --get-all http.proxy
2019-12-23T16:33:55.8141223Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67327/merge:refs/remotes/pull/67327/merge
---
2019-12-23T17:34:30.8423923Z .................................................................................................... 1600/9427
2019-12-23T17:34:35.4878233Z .................................................................................................... 1700/9427
2019-12-23T17:34:46.1623434Z ...................................................F.................................i.............. 1800/9427
2019-12-23T17:34:53.8057770Z .................................................................................................... 1900/9427
2019-12-23T17:35:01.1617258Z ......................................................................iiiii......................... 2000/9427
2019-12-23T17:35:21.6466894Z .................................................................................................... 2200/9427
2019-12-23T17:35:24.0391725Z .................................................................................................... 2300/9427
2019-12-23T17:35:26.7675645Z .................................................................................................... 2400/9427
2019-12-23T17:35:39.9501862Z .................................................................................................... 2500/9427
---
2019-12-23T17:38:30.7342096Z .i...............i.................................................................................. 4900/9427
2019-12-23T17:38:40.6057452Z .................................................................................................... 5000/9427
2019-12-23T17:38:45.4932286Z .............................................i...................................................... 5100/9427
2019-12-23T17:38:55.2227973Z .................................................................................................... 5200/9427
2019-12-23T17:39:01.0498064Z ............ii.ii...........i....................................................................... 5300/9427
2019-12-23T17:39:10.2094398Z .................................................................................................... 5500/9427
2019-12-23T17:39:21.4029232Z ..............................................................................................i..... 5600/9427
2019-12-23T17:39:29.9663468Z .................................................................................................... 5700/9427
2019-12-23T17:39:35.1442823Z .................................................................................................... 5800/9427
2019-12-23T17:39:35.1442823Z .................................................................................................... 5800/9427
2019-12-23T17:39:44.8358977Z ..................................................................................ii...i..ii........ 5900/9427
2019-12-23T17:40:07.2813424Z .................................................................................................... 6100/9427
2019-12-23T17:40:14.8049293Z .................................................................................................... 6200/9427
2019-12-23T17:40:22.7599262Z .................................................................................................... 6300/9427
2019-12-23T17:40:22.7599262Z .................................................................................................... 6300/9427
2019-12-23T17:40:43.0326875Z .........i..ii...................................................................................... 6400/9427
2019-12-23T17:41:02.0919760Z .....................................................................................i.............. 6600/9427
2019-12-23T17:41:04.2141699Z .................................................................................................... 6700/9427
2019-12-23T17:41:06.5048898Z .....................................................................................i.............. 6800/9427
2019-12-23T17:41:09.1932144Z .................................................................................................... 6900/9427
---
2019-12-23T17:45:53.3599488Z 
2019-12-23T17:45:53.3600477Z ---- [ui] ui/consts/uninhabited-const-issue-61744.rs stdout ----
2019-12-23T17:45:53.3604636Z diff of stderr:
2019-12-23T17:45:53.3605813Z 
2019-12-23T17:45:53.3607243Z - error[E0391]: cycle detected when const-evaluating `hint_unreachable`
2019-12-23T17:45:53.3607814Z + error[E0080]: evaluation of constant value failed
2019-12-23T17:45:53.3608421Z 2   --> $DIR/uninhabited-const-issue-61744.rs:8:5
2019-12-23T17:45:53.3609752Z + LL |     hint_unreachable()
2019-12-23T17:45:53.3610820Z +    |     ------------------
2019-12-23T17:45:53.3611795Z +    |     |
2019-12-23T17:45:53.3611795Z +    |     |
2019-12-23T17:45:53.3613033Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3613715Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3614327Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3614861Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3616404Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3621144Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3623930Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3625215Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3626642Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3627797Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3628694Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3630077Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3630779Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3632643Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3633774Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3634346Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3634889Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3635414Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3635959Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3637224Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3637754Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3638768Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3639412Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3640519Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3641174Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3641724Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3642658Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3643473Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3646238Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3646945Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3647496Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3648705Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3649758Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3650414Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3651530Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3654093Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3654453Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3654763Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3655814Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3656170Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3656526Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3656833Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3658223Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3659014Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3659486Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3659881Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3660188Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3660493Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3660872Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3661179Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3661482Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3662954Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3663310Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3663622Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3664729Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3665060Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3665407Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3665754Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3666060Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3666401Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3666709Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3667023Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3667348Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3667656Z +    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3667715Z + ...
2019-12-23T17:45:53.3667781Z 4 LL |     fake_type()
2019-12-23T17:45:53.3667878Z +    |     |
2019-12-23T17:45:53.3667933Z +    |     reached the configured maximum number of stack frames
2019-12-23T17:45:53.3667933Z +    |     reached the configured maximum number of stack frames
2019-12-23T17:45:53.3668276Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-12-23T17:45:53.3668393Z + error: any use of this value will cause an error
2019-12-23T17:45:53.3668678Z +   --> $DIR/uninhabited-const-issue-61744.rs:12:36
2019-12-23T17:45:53.3668731Z 6    |
2019-12-23T17:45:53.3668731Z 6    |
2019-12-23T17:45:53.3668997Z - note: ...which requires const-evaluating `fake_type`...
2019-12-23T17:45:53.3669283Z -   --> $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3669343Z + LL |     const CONSTANT: i32 = unsafe { fake_type() };
2019-12-23T17:45:53.3669684Z +    |                                    |
2019-12-23T17:45:53.3669740Z +    |                                    referenced constant has errors
2019-12-23T17:45:53.3669788Z 9    |
2019-12-23T17:45:53.3670018Z - LL |     hint_unreachable()
2019-12-23T17:45:53.3670018Z - LL |     hint_unreachable()
2019-12-23T17:45:53.3670271Z -    |     ^^^^^^^^^^^^^^^^^^
2019-12-23T17:45:53.3670573Z -    = note: ...which again requires const-evaluating `hint_unreachable`, completing the cycle
2019-12-23T17:45:53.3670840Z - note: cycle used when const-evaluating `fake_type`
2019-12-23T17:45:53.3671342Z -   --> $DIR/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3671449Z + 
2019-12-23T17:45:53.3671515Z + error[E0080]: erroneous constant used
2019-12-23T17:45:53.3671873Z +   --> $DIR/uninhabited-const-issue-61744.rs:18:10
2019-12-23T17:45:53.3671938Z 15    |
2019-12-23T17:45:53.3671938Z 15    |
2019-12-23T17:45:53.3672221Z - LL |     hint_unreachable()
2019-12-23T17:45:53.3672532Z -    |     ^^^^^^^^^^^^^^^^^^
2019-12-23T17:45:53.3672588Z + LL |     dbg!(i32::CONSTANT);
2019-12-23T17:45:53.3672665Z +    |          ^^^^^^^^^^^^^ referenced constant has errors
2019-12-23T17:45:53.3673008Z - error: aborting due to previous error
2019-12-23T17:45:53.3673068Z + error: aborting due to 3 previous errors
2019-12-23T17:45:53.3673134Z 20 
2019-12-23T17:45:53.3673447Z - For more information about this error, try `rustc --explain E0391`.
2019-12-23T17:45:53.3673447Z - For more information about this error, try `rustc --explain E0391`.
2019-12-23T17:45:53.3673772Z + For more information about this error, try `rustc --explain E0080`.
2019-12-23T17:45:53.3673846Z 22 
2019-12-23T17:45:53.3673880Z 
2019-12-23T17:45:53.3673913Z 
2019-12-23T17:45:53.3673967Z The actual stderr differed from the expected stderr.
2019-12-23T17:45:53.3674398Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/uninhabited-const-issue-61744.stderr
2019-12-23T17:45:53.3674714Z To update references, rerun the tests and pass the `--bless` flag
2019-12-23T17:45:53.3675058Z To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`
2019-12-23T17:45:53.3675171Z error: 1 errors occurred comparing output.
2019-12-23T17:45:53.3675223Z status: exit code: 1
2019-12-23T17:45:53.3675223Z status: exit code: 1
2019-12-23T17:45:53.3676248Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary" "-A" "unused"
2019-12-23T17:45:53.3676662Z ------------------------------------------
2019-12-23T17:45:53.3676704Z 
2019-12-23T17:45:53.3676971Z ------------------------------------------
2019-12-23T17:45:53.3677042Z stderr:
---
2019-12-23T17:45:53.3716530Z    |     inside call to `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3716956Z    |     inside call to `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3717231Z    |     inside call to `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-12-23T17:45:53.3717280Z ...
2019-12-23T17:45:53.3717896Z LL |     fake_type() //~ ERROR cycle detected when const-evaluating `hint_unreachable` [E0391]
2019-12-23T17:45:53.3717990Z    |     |
2019-12-23T17:45:53.3718052Z    |     reached the configured maximum number of stack frames
2019-12-23T17:45:53.3718052Z    |     reached the configured maximum number of stack frames
2019-12-23T17:45:53.3718426Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-12-23T17:45:53.3718517Z error: any use of this value will cause an error
2019-12-23T17:45:53.3718821Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:12:36
2019-12-23T17:45:53.3718874Z    |
2019-12-23T17:45:53.3718874Z    |
2019-12-23T17:45:53.3718921Z LL |     const CONSTANT: i32 = unsafe { fake_type() };
2019-12-23T17:45:53.3719248Z    |                                    |
2019-12-23T17:45:53.3719298Z    |                                    referenced constant has errors
2019-12-23T17:45:53.3719369Z    |
2019-12-23T17:45:53.3719414Z    = note: `#[deny(const_err)]` on by default
---
2019-12-23T17:45:53.3721323Z test result: FAILED. 9379 passed; 1 failed; 47 ignored; 0 measured; 0 filtered out
2019-12-23T17:45:53.3721359Z 
2019-12-23T17:45:53.3721441Z 
2019-12-23T17:45:53.3721469Z 
2019-12-23T17:45:53.3724236Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T17:45:53.3724505Z 
2019-12-23T17:45:53.3724551Z 
2019-12-23T17:45:53.3724598Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T17:45:53.3724648Z Build completed unsuccessfully in 1:05:28
2019-12-23T17:45:53.3724648Z Build completed unsuccessfully in 1:05:28
2019-12-23T17:45:53.3725010Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-23T17:45:53.3725221Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T17:45:53.3725271Z == clock drift check ==
2019-12-23T17:45:53.3725333Z   local time: Mon Dec 23 17:45:53 UTC 2019
2019-12-23T17:45:53.6565004Z   network time: Mon, 23 Dec 2019 17:45:53 GMT
2019-12-23T17:45:53.6574376Z == end clock drift check ==
2019-12-23T17:45:54.5834359Z 
2019-12-23T17:45:54.5934932Z ##[error]Bash exited with code '1'.
2019-12-23T17:45:54.5984177Z ##[section]Starting: Checkout
2019-12-23T17:45:54.5985920Z ==============================================================================
2019-12-23T17:45:54.5985977Z Task         : Get sources
2019-12-23T17:45:54.5986026Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
