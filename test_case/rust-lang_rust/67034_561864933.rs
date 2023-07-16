plain
2019-12-04T21:07:44.0818487Z ========================== Starting Command Output ===========================
2019-12-04T21:07:44.0841161Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/be3dde5d-94f8-4261-a108-809a62813220.sh
2019-12-04T21:07:44.3171037Z 
2019-12-04T21:07:44.3237884Z ##[section]Finishing: Disable git automatic line ending conversion
2019-12-04T21:07:44.3244011Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67034/merge to s
2019-12-04T21:07:44.3245762Z Task         : Get sources
2019-12-04T21:07:44.3245796Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-04T21:07:44.3245879Z Version      : 1.0.0
2019-12-04T21:07:44.3245912Z Author       : Microsoft
---
2019-12-04T21:07:46.8078268Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-04T21:07:46.8093704Z ##[command]git config gc.auto 0
2019-12-04T21:07:46.8096468Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-04T21:07:46.8100180Z ##[command]git config --get-all http.proxy
2019-12-04T21:07:46.8233721Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67034/merge:refs/remotes/pull/67034/merge
---
2019-12-04T22:06:35.0948920Z ......................................................................................F............. 1600/9323
2019-12-04T22:06:39.6962652Z .....F......................................................F....................................... 1700/9323
2019-12-04T22:06:52.2735290Z ..........................................i......................................................... 1800/9323
2019-12-04T22:06:59.4771679Z .................................................................................................... 1900/9323
2019-12-04T22:07:13.2620748Z ...........................iiiii.................................................................... 2000/9323
2019-12-04T22:07:23.1424447Z .................................................................................................... 2200/9323
2019-12-04T22:07:25.6619659Z .................................................................................................... 2300/9323
2019-12-04T22:07:30.2845585Z .................................................................................................... 2400/9323
2019-12-04T22:07:51.5499226Z .................................................................................................... 2500/9323
---
2019-12-04T22:10:28.0704535Z ............................i...............i....................................................... 4800/9323
2019-12-04T22:10:38.0563595Z .................................................................................................... 4900/9323
2019-12-04T22:10:44.5118454Z .................................................................................................... 5000/9323
2019-12-04T22:10:52.2580126Z .................................................................................................... 5100/9323
2019-12-04T22:10:59.7851767Z ..................................ii.ii...........i................................................. 5200/9323
2019-12-04T22:11:09.0843362Z .................................................................................................... 5400/9323
2019-12-04T22:11:18.7033807Z .................................................................................................... 5500/9323
2019-12-04T22:11:25.8619275Z ................i................................................................................... 5600/9323
2019-12-04T22:11:32.0941308Z .................................................................................................... 5700/9323
2019-12-04T22:11:32.0941308Z .................................................................................................... 5700/9323
2019-12-04T22:11:43.3824927Z .................................................................................................... 5800/9323
2019-12-04T22:11:54.9719981Z ..ii...i..ii...........i............................................................................ 5900/9323
2019-12-04T22:12:12.7269449Z .................................................................................................... 6100/9323
2019-12-04T22:12:20.0959944Z .................................................................................................... 6200/9323
2019-12-04T22:12:20.0959944Z .................................................................................................... 6200/9323
2019-12-04T22:12:36.7463448Z .........................i..ii...................................................................... 6300/9323
2019-12-04T22:12:55.9372986Z .................................................................................................i.. 6500/9323
2019-12-04T22:12:58.1737302Z .................................................................................................... 6600/9323
2019-12-04T22:13:00.3775886Z ........................................................................................i........... 6700/9323
2019-12-04T22:13:03.0379097Z .................................................................................................... 6800/9323
---
2019-12-04T22:14:39.8894889Z .................................................................................................... 7400/9323
2019-12-04T22:14:45.3922584Z .................................................................................................... 7500/9323
2019-12-04T22:14:51.4490248Z .................................................................................................... 7600/9323
2019-12-04T22:15:02.2298113Z .................................................................................................... 7700/9323
2019-12-04T22:15:08.7295118Z .iiii............................................................................................... 7800/9323
2019-12-04T22:15:22.8914596Z .................................................................................................... 8000/9323
2019-12-04T22:15:34.4743136Z .................................................................................................... 8100/9323
2019-12-04T22:15:46.4527573Z .................................................................................................... 8200/9323
2019-12-04T22:15:52.6617667Z .................................................................................................... 8300/9323
---
2019-12-04T22:17:44.9001308Z failures:
2019-12-04T22:17:44.9054523Z 
2019-12-04T22:17:44.9055384Z ---- [ui] ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-completion.rs stdout ----
2019-12-04T22:17:44.9055698Z 
2019-12-04T22:17:44.9056296Z error: error pattern ' thread 'main' panicked at '`async fn` resumed after completion'' not found!
2019-12-04T22:17:44.9057175Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-completion/a"
2019-12-04T22:17:44.9058337Z stdout:
2019-12-04T22:17:44.9059804Z ------------------------------------------
2019-12-04T22:17:44.9061528Z 
2019-12-04T22:17:44.9061528Z 
2019-12-04T22:17:44.9063594Z ------------------------------------------
2019-12-04T22:17:44.9064850Z stderr:
2019-12-04T22:17:44.9065741Z ------------------------------------------
2019-12-04T22:17:44.9067358Z thread 'main' panicked at /checkout/src/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-completion.rs:12:16, '`async fn` resumed after completion'
2019-12-04T22:17:44.9068037Z 
2019-12-04T22:17:44.9068508Z ------------------------------------------
2019-12-04T22:17:44.9073596Z 
2019-12-04T22:17:44.9074255Z 
2019-12-04T22:17:44.9074255Z 
2019-12-04T22:17:44.9075800Z ---- [ui] ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic.rs stdout ----
2019-12-04T22:17:44.9076088Z 
2019-12-04T22:17:44.9076661Z error: error pattern ' thread 'main' panicked at '`async fn` resumed after panicking'' not found!
2019-12-04T22:17:44.9077463Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic/a"
2019-12-04T22:17:44.9077676Z stdout:
2019-12-04T22:17:44.9078110Z ------------------------------------------
2019-12-04T22:17:44.9078285Z 
2019-12-04T22:17:44.9078285Z 
2019-12-04T22:17:44.9078675Z ------------------------------------------
2019-12-04T22:17:44.9078911Z stderr:
2019-12-04T22:17:44.9079306Z ------------------------------------------
2019-12-04T22:17:44.9079871Z thread 'main' panicked at /checkout/src/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic.rs:15:5, 'explicit panic'
2019-12-04T22:17:44.9080113Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-04T22:17:44.9080654Z thread 'main' panicked at /checkout/src/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic.rs:14:16, '`async fn` resumed after panicking'
2019-12-04T22:17:44.9081253Z ------------------------------------------
2019-12-04T22:17:44.9081426Z 
2019-12-04T22:17:44.9081559Z 
2019-12-04T22:17:44.9081979Z ---- [ui] ui/consts/const-eval/const_panic.rs stdout ----
2019-12-04T22:17:44.9081979Z ---- [ui] ui/consts/const-eval/const_panic.rs stdout ----
2019-12-04T22:17:44.9082177Z diff of stderr:
2019-12-04T22:17:44.9082310Z 
2019-12-04T22:17:44.9082480Z 4 LL | pub const Z: () = panic!("cheese");
2019-12-04T22:17:44.9083644Z 6    |                   |
2019-12-04T22:17:44.9083644Z 6    |                   |
2019-12-04T22:17:44.9084347Z -    |                   the evaluated program panicked at 'cheese', $DIR/const_panic.rs:4:19
2019-12-04T22:17:44.9084941Z +    |                   the evaluated program panicked at $DIR/const_panic.rs:4:19, 'cheese'
2019-12-04T22:17:44.9085330Z 9    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9085846Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9086033Z 
2019-12-04T22:17:44.9086033Z 
2019-12-04T22:17:44.9086217Z 15 LL | pub const Y: () = unreachable!();
2019-12-04T22:17:44.9086802Z 17    |                   |
2019-12-04T22:17:44.9087316Z -    |                   the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic.rs:7:19
2019-12-04T22:17:44.9087316Z -    |                   the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic.rs:7:19
2019-12-04T22:17:44.9088077Z +    |                   the evaluated program panicked at $DIR/const_panic.rs:7:19, 'internal error: entered unreachable code'
2019-12-04T22:17:44.9088832Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9089055Z 21 
2019-12-04T22:17:44.9089220Z 
2019-12-04T22:17:44.9089385Z 25 LL | pub const X: () = unimplemented!();
2019-12-04T22:17:44.9089385Z 25 LL | pub const X: () = unimplemented!();
2019-12-04T22:17:44.9089827Z 26    | ------------------^^^^^^^^^^^^^^^^-
2019-12-04T22:17:44.9090059Z 27    |                   |
2019-12-04T22:17:44.9090567Z -    |                   the evaluated program panicked at 'not yet implemented', $DIR/const_panic.rs:10:19
2019-12-04T22:17:44.9091125Z +    |                   the evaluated program panicked at $DIR/const_panic.rs:10:19, 'not yet implemented'
2019-12-04T22:17:44.9091878Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9092132Z 31 
2019-12-04T22:17:44.9092283Z 
2019-12-04T22:17:44.9092424Z 
2019-12-04T22:17:44.9092424Z 
2019-12-04T22:17:44.9092592Z The actual stderr differed from the expected stderr.
2019-12-04T22:17:44.9093613Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/const_panic.stderr
2019-12-04T22:17:44.9094113Z To update references, rerun the tests and pass the `--bless` flag
2019-12-04T22:17:44.9094619Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic.rs`
2019-12-04T22:17:44.9094967Z error: 1 errors occurred comparing output.
2019-12-04T22:17:44.9095136Z status: exit code: 1
2019-12-04T22:17:44.9095136Z status: exit code: 1
2019-12-04T22:17:44.9096308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/auxiliary" "-A" "unused"
2019-12-04T22:17:44.9097001Z ------------------------------------------
2019-12-04T22:17:44.9097178Z 
2019-12-04T22:17:44.9097600Z ------------------------------------------
2019-12-04T22:17:44.9097792Z stderr:
2019-12-04T22:17:44.9097792Z stderr:
2019-12-04T22:17:44.9098205Z ------------------------------------------
2019-12-04T22:17:44.9098427Z error: any use of this value will cause an error
2019-12-04T22:17:44.9098845Z   --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:4:19
2019-12-04T22:17:44.9099063Z    |
2019-12-04T22:17:44.9099221Z LL | pub const Z: () = panic!("cheese");
2019-12-04T22:17:44.9099844Z    |                   |
2019-12-04T22:17:44.9099844Z    |                   |
2019-12-04T22:17:44.9100461Z    |                   the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic.rs:4:19, 'cheese'
2019-12-04T22:17:44.9100875Z    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9101474Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9101669Z 
2019-12-04T22:17:44.9101863Z error: any use of this value will cause an error
2019-12-04T22:17:44.9101863Z error: any use of this value will cause an error
2019-12-04T22:17:44.9102291Z   --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:7:19
2019-12-04T22:17:44.9102494Z    |
2019-12-04T22:17:44.9102666Z LL | pub const Y: () = unreachable!();
2019-12-04T22:17:44.9103812Z    |                   |
2019-12-04T22:17:44.9103812Z    |                   |
2019-12-04T22:17:44.9104368Z    |                   the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic.rs:7:19, 'internal error: entered unreachable code'
2019-12-04T22:17:44.9105339Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9105535Z 
2019-12-04T22:17:44.9105722Z error: any use of this value will cause an error
2019-12-04T22:17:44.9106164Z   --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:10:19
2019-12-04T22:17:44.9106164Z   --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:10:19
2019-12-04T22:17:44.9106368Z    |
2019-12-04T22:17:44.9106521Z LL | pub const X: () = unimplemented!();
2019-12-04T22:17:44.9106930Z    | ------------------^^^^^^^^^^^^^^^^-
2019-12-04T22:17:44.9107128Z    |                   |
2019-12-04T22:17:44.9107666Z    |                   the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic.rs:10:19, 'not yet implemented'
2019-12-04T22:17:44.9108384Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9108636Z 
2019-12-04T22:17:44.9108806Z error: aborting due to 3 previous errors
2019-12-04T22:17:44.9108968Z 
2019-12-04T22:17:44.9108968Z 
2019-12-04T22:17:44.9109107Z 
2019-12-04T22:17:44.9109555Z ------------------------------------------
2019-12-04T22:17:44.9110015Z 
2019-12-04T22:17:44.9110501Z 
2019-12-04T22:17:44.9111045Z ---- [ui] ui/consts/const-eval/const_panic_libcore.rs stdout ----
2019-12-04T22:17:44.9111270Z diff of stderr:
2019-12-04T22:17:44.9111429Z 
2019-12-04T22:17:44.9111588Z 4 LL | const Z: () = panic!("cheese");
2019-12-04T22:17:44.9112231Z 6    |               |
2019-12-04T22:17:44.9112231Z 6    |               |
2019-12-04T22:17:44.9113135Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore.rs:5:15
2019-12-04T22:17:44.9113822Z +    |               the evaluated program panicked at $DIR/const_panic_libcore.rs:5:15, 'cheese'
2019-12-04T22:17:44.9114214Z 9    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9114793Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9114992Z 
2019-12-04T22:17:44.9114992Z 
2019-12-04T22:17:44.9115150Z 15 LL | const Y: () = unreachable!();
2019-12-04T22:17:44.9115745Z 17    |               |
2019-12-04T22:17:44.9116243Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore.rs:8:15
2019-12-04T22:17:44.9116243Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore.rs:8:15
2019-12-04T22:17:44.9116849Z +    |               the evaluated program panicked at $DIR/const_panic_libcore.rs:8:15, 'internal error: entered unreachable code'
2019-12-04T22:17:44.9117568Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9117791Z 21 
2019-12-04T22:17:44.9117926Z 
2019-12-04T22:17:44.9118258Z 25 LL | const X: () = unimplemented!();
2019-12-04T22:17:44.9118258Z 25 LL | const X: () = unimplemented!();
2019-12-04T22:17:44.9118781Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-12-04T22:17:44.9119028Z 27    |               |
2019-12-04T22:17:44.9120135Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore.rs:11:15
2019-12-04T22:17:44.9120791Z +    |               the evaluated program panicked at $DIR/const_panic_libcore.rs:11:15, 'not yet implemented'
2019-12-04T22:17:44.9123252Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9123872Z 31 
2019-12-04T22:17:44.9124035Z 
2019-12-04T22:17:44.9124477Z 
2019-12-04T22:17:44.9124477Z 
2019-12-04T22:17:44.9124614Z The actual stderr differed from the expected stderr.
2019-12-04T22:17:44.9125186Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/const_panic_libcore.stderr
2019-12-04T22:17:44.9147396Z To update references, rerun the tests and pass the `--bless` flag
2019-12-04T22:17:44.9147993Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore.rs`
2019-12-04T22:17:44.9148115Z error: 1 errors occurred comparing output.
2019-12-04T22:17:44.9148163Z status: exit code: 1
2019-12-04T22:17:44.9148163Z status: exit code: 1
2019-12-04T22:17:44.9149030Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/auxiliary" "-A" "unused"
2019-12-04T22:17:44.9149426Z ------------------------------------------
2019-12-04T22:17:44.9149465Z 
2019-12-04T22:17:44.9150390Z ------------------------------------------
2019-12-04T22:17:44.9150470Z stderr:
2019-12-04T22:17:44.9150470Z stderr:
2019-12-04T22:17:44.9150747Z ------------------------------------------
2019-12-04T22:17:44.9150801Z error: any use of this value will cause an error
2019-12-04T22:17:44.9151091Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:5:15
2019-12-04T22:17:44.9151168Z    |
2019-12-04T22:17:44.9151218Z LL | const Z: () = panic!("cheese");
2019-12-04T22:17:44.9151524Z    |               |
2019-12-04T22:17:44.9151524Z    |               |
2019-12-04T22:17:44.9151854Z    |               the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:5:15, 'cheese'
2019-12-04T22:17:44.9151995Z    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9152352Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9152398Z 
2019-12-04T22:17:44.9152466Z error: any use of this value will cause an error
2019-12-04T22:17:44.9152466Z error: any use of this value will cause an error
2019-12-04T22:17:44.9157312Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:8:15
2019-12-04T22:17:44.9157390Z    |
2019-12-04T22:17:44.9157463Z LL | const Y: () = unreachable!();
2019-12-04T22:17:44.9158477Z    |               |
2019-12-04T22:17:44.9158477Z    |               |
2019-12-04T22:17:44.9158931Z    |               the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:8:15, 'internal error: entered unreachable code'
2019-12-04T22:17:44.9159532Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9159629Z 
2019-12-04T22:17:44.9159680Z error: any use of this value will cause an error
2019-12-04T22:17:44.9160276Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-12-04T22:17:44.9160276Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-12-04T22:17:44.9160376Z    |
2019-12-04T22:17:44.9160419Z LL | const X: () = unimplemented!();
2019-12-04T22:17:44.9160712Z    | --------------^^^^^^^^^^^^^^^^-
2019-12-04T22:17:44.9160954Z    |               |
2019-12-04T22:17:44.9161351Z    |               the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15, 'not yet implemented'
2019-12-04T22:17:44.9161929Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9162005Z 
2019-12-04T22:17:44.9162053Z error: aborting due to 3 previous errors
2019-12-04T22:17:44.9162085Z 
2019-12-04T22:17:44.9162085Z 
2019-12-04T22:17:44.9162111Z 
2019-12-04T22:17:44.9162764Z ------------------------------------------
2019-12-04T22:17:44.9162810Z 
2019-12-04T22:17:44.9162838Z 
2019-12-04T22:17:44.9164299Z ---- [ui] ui/consts/const-eval/const_panic_libcore_main.rs stdout ----
2019-12-04T22:17:44.9164386Z diff of stderr:
2019-12-04T22:17:44.9164415Z 
2019-12-04T22:17:44.9164458Z 4 LL | const Z: () = panic!("cheese");
2019-12-04T22:17:44.9164760Z 6    |               |
2019-12-04T22:17:44.9164760Z 6    |               |
2019-12-04T22:17:44.9165043Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore_main.rs:9:15
2019-12-04T22:17:44.9165330Z +    |               the evaluated program panicked at $DIR/const_panic_libcore_main.rs:9:15, 'cheese'
2019-12-04T22:17:44.9165443Z 9    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9165826Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9165899Z 
2019-12-04T22:17:44.9165899Z 
2019-12-04T22:17:44.9165948Z 15 LL | const Y: () = unreachable!();
2019-12-04T22:17:44.9166246Z 17    |               |
2019-12-04T22:17:44.9166559Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_main.rs:12:15
2019-12-04T22:17:44.9166559Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_main.rs:12:15
2019-12-04T22:17:44.9166886Z +    |               the evaluated program panicked at $DIR/const_panic_libcore_main.rs:12:15, 'internal error: entered unreachable code'
2019-12-04T22:17:44.9168194Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9168269Z 21 
2019-12-04T22:17:44.9168298Z 
2019-12-04T22:17:44.9168365Z 25 LL | const X: () = unimplemented!();
2019-12-04T22:17:44.9168365Z 25 LL | const X: () = unimplemented!();
2019-12-04T22:17:44.9168652Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-12-04T22:17:44.9168700Z 27    |               |
2019-12-04T22:17:44.9169034Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore_main.rs:15:15
2019-12-04T22:17:44.9169343Z +    |               the evaluated program panicked at $DIR/const_panic_libcore_main.rs:15:15, 'not yet implemented'
2019-12-04T22:17:44.9169732Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9169786Z 31 
2019-12-04T22:17:44.9169812Z 
2019-12-04T22:17:44.9169837Z 
2019-12-04T22:17:44.9169837Z 
2019-12-04T22:17:44.9169901Z The actual stderr differed from the expected stderr.
2019-12-04T22:17:44.9170232Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/const_panic_libcore_main.stderr
2019-12-04T22:17:44.9170481Z To update references, rerun the tests and pass the `--bless` flag
2019-12-04T22:17:44.9170781Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_main.rs`
2019-12-04T22:17:44.9170997Z error: 1 errors occurred comparing output.
2019-12-04T22:17:44.9171074Z status: exit code: 1
2019-12-04T22:17:44.9171074Z status: exit code: 1
2019-12-04T22:17:44.9171911Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary" "-A" "unused"
2019-12-04T22:17:44.9172250Z ------------------------------------------
2019-12-04T22:17:44.9172284Z 
2019-12-04T22:17:44.9172643Z ------------------------------------------
2019-12-04T22:17:44.9172690Z stderr:
2019-12-04T22:17:44.9172690Z stderr:
2019-12-04T22:17:44.9173227Z ------------------------------------------
2019-12-04T22:17:44.9173286Z error: any use of this value will cause an error
2019-12-04T22:17:44.9173575Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:9:15
2019-12-04T22:17:44.9173626Z    |
2019-12-04T22:17:44.9173672Z LL | const Z: () = panic!("cheese");
2019-12-04T22:17:44.9173951Z    |               |
2019-12-04T22:17:44.9173951Z    |               |
2019-12-04T22:17:44.9174258Z    |               the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:9:15, 'cheese'
2019-12-04T22:17:44.9174375Z    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9174696Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9174810Z 
2019-12-04T22:17:44.9174856Z error: any use of this value will cause an error
2019-12-04T22:17:44.9174856Z error: any use of this value will cause an error
2019-12-04T22:17:44.9175131Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:12:15
2019-12-04T22:17:44.9175198Z    |
2019-12-04T22:17:44.9175241Z LL | const Y: () = unreachable!();
2019-12-04T22:17:44.9175516Z    |               |
2019-12-04T22:17:44.9175516Z    |               |
2019-12-04T22:17:44.9175858Z    |               the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:12:15, 'internal error: entered unreachable code'
2019-12-04T22:17:44.9176248Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9176290Z 
2019-12-04T22:17:44.9176335Z error: any use of this value will cause an error
2019-12-04T22:17:44.9176612Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-12-04T22:17:44.9176612Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-12-04T22:17:44.9176670Z    |
2019-12-04T22:17:44.9176713Z LL | const X: () = unimplemented!();
2019-12-04T22:17:44.9176951Z    | --------------^^^^^^^^^^^^^^^^-
2019-12-04T22:17:44.9176996Z    |               |
2019-12-04T22:17:44.9177319Z    |               the evaluated program panicked at /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15, 'not yet implemented'
2019-12-04T22:17:44.9178459Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9178502Z 
2019-12-04T22:17:44.9178546Z error: aborting due to 3 previous errors
2019-12-04T22:17:44.9178599Z 
---
2019-12-04T22:17:44.9179252Z 
2019-12-04T22:17:44.9179311Z 4 LL | const _: () = assert!(false);
2019-12-04T22:17:44.9179678Z 5    | --------------^^^^^^^^^^^^^^-
2019-12-04T22:17:44.9179738Z 6    |               |
2019-12-04T22:17:44.9180072Z -    |               the evaluated program panicked at 'assertion failed: false', $DIR/assert.rs:12:15
2019-12-04T22:17:44.9180361Z +    |               the evaluated program panicked at $DIR/assert.rs:12:15, 'assertion failed: false'
2019-12-04T22:17:44.9180457Z 9    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9180800Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9180841Z 
2019-12-04T22:17:44.9180867Z 
2019-12-04T22:17:44.9180867Z 
2019-12-04T22:17:44.9180928Z The actual stderr differed from the expected stderr.
2019-12-04T22:17:44.9181233Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert.both/assert.both.stderr
2019-12-04T22:17:44.9181605Z To update references, rerun the tests and pass the `--bless` flag
2019-12-04T22:17:44.9181900Z To only update this specific test, also pass `--test-args consts/control-flow/assert.rs`
2019-12-04T22:17:44.9181936Z 
2019-12-04T22:17:44.9181983Z error in revision `both`: 1 errors occurred comparing output.
2019-12-04T22:17:44.9182027Z status: exit code: 1
2019-12-04T22:17:44.9182819Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/assert.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "both" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert.both" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert.both/auxiliary" "-A" "unused"
2019-12-04T22:17:44.9183510Z ------------------------------------------
2019-12-04T22:17:44.9183548Z 
2019-12-04T22:17:44.9183774Z ------------------------------------------
2019-12-04T22:17:44.9183837Z stderr:
2019-12-04T22:17:44.9183837Z stderr:
2019-12-04T22:17:44.9184055Z ------------------------------------------
2019-12-04T22:17:44.9184105Z error: any use of this value will cause an error
2019-12-04T22:17:44.9184364Z   --> /checkout/src/test/ui/consts/control-flow/assert.rs:12:15
2019-12-04T22:17:44.9184413Z    |
2019-12-04T22:17:44.9184459Z LL | const _: () = assert!(false);
2019-12-04T22:17:44.9184689Z    | --------------^^^^^^^^^^^^^^-
2019-12-04T22:17:44.9184735Z    |               |
2019-12-04T22:17:44.9185050Z    |               the evaluated program panicked at /checkout/src/test/ui/consts/control-flow/assert.rs:12:15, 'assertion failed: false'
2019-12-04T22:17:44.9185166Z    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9185509Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9185549Z 
2019-12-04T22:17:44.9185610Z error: aborting due to previous error
---
2019-12-04T22:17:44.9186592Z 
2019-12-04T22:17:44.9186938Z ---- [ui] ui/consts/control-flow/short-circuit.rs#stock stdout ----
2019-12-04T22:17:44.9186990Z diff of stderr:
2019-12-04T22:17:44.9187035Z 
2019-12-04T22:17:44.9187077Z 4 LL | const _: bool = true || panic!();
2019-12-04T22:17:44.9187348Z 6    |                         |
2019-12-04T22:17:44.9187659Z -    |                         the evaluated program panicked at 'explicit panic', $DIR/short-circuit.rs:10:25
2019-12-04T22:17:44.9187659Z -    |                         the evaluated program panicked at 'explicit panic', $DIR/short-circuit.rs:10:25
2019-12-04T22:17:44.9187953Z +    |                         the evaluated program panicked at $DIR/short-circuit.rs:10:25, 'explicit panic'
2019-12-04T22:17:44.9188199Z 9    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9188569Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9188611Z 
2019-12-04T22:17:44.9188611Z 
2019-12-04T22:17:44.9188673Z 15 LL | const _: bool = false && panic!();
2019-12-04T22:17:44.9188941Z 17    |                          |
2019-12-04T22:17:44.9189243Z -    |                          the evaluated program panicked at 'explicit panic', $DIR/short-circuit.rs:11:26
2019-12-04T22:17:44.9189243Z -    |                          the evaluated program panicked at 'explicit panic', $DIR/short-circuit.rs:11:26
2019-12-04T22:17:44.9189538Z +    |                          the evaluated program panicked at $DIR/short-circuit.rs:11:26, 'explicit panic'
2019-12-04T22:17:44.9190134Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9190299Z 21 
2019-12-04T22:17:44.9190335Z 
2019-12-04T22:17:44.9190361Z 
2019-12-04T22:17:44.9190361Z 
2019-12-04T22:17:44.9190423Z The actual stderr differed from the expected stderr.
2019-12-04T22:17:44.9190779Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/short-circuit.stock/short-circuit.stock.stderr
2019-12-04T22:17:44.9191030Z To update references, rerun the tests and pass the `--bless` flag
2019-12-04T22:17:44.9191345Z To only update this specific test, also pass `--test-args consts/control-flow/short-circuit.rs`
2019-12-04T22:17:44.9191385Z 
2019-12-04T22:17:44.9191433Z error in revision `stock`: 1 errors occurred comparing output.
2019-12-04T22:17:44.9191481Z status: exit code: 1
2019-12-04T22:17:44.9192331Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/short-circuit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/short-circuit.stock" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/short-circuit.stock/auxiliary" "-A" "unused"
2019-12-04T22:17:44.9192700Z ------------------------------------------
2019-12-04T22:17:44.9192737Z 
2019-12-04T22:17:44.9198013Z ------------------------------------------
2019-12-04T22:17:44.9198100Z stderr:
2019-12-04T22:17:44.9198100Z stderr:
2019-12-04T22:17:44.9198355Z ------------------------------------------
2019-12-04T22:17:44.9198405Z error: any use of this value will cause an error
2019-12-04T22:17:44.9198679Z   --> /checkout/src/test/ui/consts/control-flow/short-circuit.rs:10:25
2019-12-04T22:17:44.9198731Z    |
2019-12-04T22:17:44.9198799Z LL | const _: bool = true || panic!();  //[stock]~ ERROR any use of this value will cause an error
2019-12-04T22:17:44.9202676Z    |                         |
2019-12-04T22:17:44.9203492Z    |                         the evaluated program panicked at /checkout/src/test/ui/consts/control-flow/short-circuit.rs:10:25, 'explicit panic'
2019-12-04T22:17:44.9203584Z    |
2019-12-04T22:17:44.9203632Z    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9203632Z    = note: `#[deny(const_err)]` on by default
2019-12-04T22:17:44.9203982Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-04T22:17:44.9204025Z 
2019-12-04T22:17:44.9204069Z error: any use of this value will cause an error
2019-12-04T22:17:44.9204320Z   --> /checkout/src/test/ui/consts/control-flow/short-circuit.rs:11:26
2019-12-04T22:17:44.9204384Z    |
2019-12-04T22:17:44.9204434Z LL | const _: bool = false && panic!(); //[stock]~ ERROR any use of this value will cause an error
2019-12-04T22:17:44.9204874Z    |                          |
2019-12-04T22:17:44.9205243Z    |                          the evaluated program panicked at /checkout/src/test/ui/consts/control-flow/short-circuit.rs:11:26, 'explicit panic'
2019-12-04T22:17:44.9205297Z    |
2019-12-04T22:17:44.9205634Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
---
2019-12-04T22:17:44.9206402Z 
2019-12-04T22:17:44.9206529Z 4 LL |     x: &UnsafeCell::new(42),
2019-12-04T22:17:44.9206593Z 5    |        ^^^^^^^^^^^^^^^^^^^^
2019-12-04T22:17:44.9206640Z 6 
2019-12-04T22:17:44.9206907Z - thread 'rustc' panicked at 'assertion failed: `(left != right)`
2019-12-04T22:17:44.9207218Z + thread 'rustc' panicked at src/librustc_mir/interpret/intern.rs:LL:CC, 'assertion failed: `(left != right)`
2019-12-04T22:17:44.9207272Z 8   left: `Const`,
2019-12-04T22:17:44.9207762Z -  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', src/librustc_mir/interpret/intern.rs:LL:CC
2019-12-04T22:17:44.9208255Z +  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.'
2019-12-04T22:17:44.9208415Z 11 
2019-12-04T22:17:44.9208461Z 12 error: internal compiler error: unexpected panic
2019-12-04T22:17:44.9208492Z 
2019-12-04T22:17:44.9208516Z 
2019-12-04T22:17:44.9208516Z 
2019-12-04T22:17:44.9208578Z The actual stderr differed from the expected stderr.
2019-12-04T22:17:44.9208913Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/mutable_references_ice.stderr
2019-12-04T22:17:44.9209163Z To update references, rerun the tests and pass the `--bless` flag
2019-12-04T22:17:44.9209465Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references_ice.rs`
2019-12-04T22:17:44.9209544Z error: 1 errors occurred comparing output.
2019-12-04T22:17:44.9209605Z status: exit code: 101
2019-12-04T22:17:44.9209605Z status: exit code: 101
2019-12-04T22:17:44.9210453Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary" "-A" "unused"
2019-12-04T22:17:44.9210798Z ------------------------------------------
2019-12-04T22:17:44.9210832Z 
2019-12-04T22:17:44.9211065Z ------------------------------------------
2019-12-04T22:17:44.9211111Z stderr:
2019-12-04T22:17:44.9211111Z stderr:
2019-12-04T22:17:44.9211326Z ------------------------------------------
2019-12-04T22:17:44.9211373Z warning: skipping const checks
2019-12-04T22:17:44.9211657Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:22:8
2019-12-04T22:17:44.9211787Z    |
2019-12-04T22:17:44.9211844Z LL |     x: &UnsafeCell::new(42), //~ WARN: skipping const checks
2019-12-04T22:17:44.9211939Z 
2019-12-04T22:17:44.9211939Z 
2019-12-04T22:17:44.9212252Z thread 'rustc' panicked at src/librustc_mir/interpret/intern.rs:183:17, 'assertion failed: `(left != right)`
2019-12-04T22:17:44.9212326Z   left: `Const`,
2019-12-04T22:17:44.9212767Z  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.'
2019-12-04T22:17:44.9213213Z 
2019-12-04T22:17:44.9213256Z error: internal compiler error: unexpected panic
2019-12-04T22:17:44.9213403Z 
2019-12-04T22:17:44.9213464Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-04T22:17:44.9213464Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-04T22:17:44.9213502Z 
2019-12-04T22:17:44.9213995Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-04T22:17:44.9214318Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-12-04T22:17:44.9214351Z 
2019-12-04T22:17:44.9214351Z 
2019-12-04T22:17:44.9214669Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-04T22:17:44.9214754Z 
2019-12-04T22:17:44.9214974Z ------------------------------------------
2019-12-04T22:17:44.9215005Z 
2019-12-04T22:17:44.9215031Z 
---
2019-12-04T22:17:44.9216006Z 
2019-12-04T22:17:44.9216220Z ------------------------------------------
2019-12-04T22:17:44.9216264Z stderr:
2019-12-04T22:17:44.9216475Z ------------------------------------------
2019-12-04T22:17:44.9216773Z thread 'main' panicked at /checkout/src/test/ui/issues/issue-24313.rs:23:9, 'bad failure message:
2019-12-04T22:17:44.9217056Z thread '<unnamed>' panicked at /checkout/src/test/ui/issues/issue-24313.rs:13:26, 'explicit panic'
2019-12-04T22:17:44.9217185Z fatal runtime error: failed to initiate panic, error 5
2019-12-04T22:17:44.9217215Z 
2019-12-04T22:17:44.9217390Z '
2019-12-04T22:17:44.9217458Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-04T22:17:44.9217458Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-04T22:17:44.9217497Z 
2019-12-04T22:17:44.9217714Z ------------------------------------------
2019-12-04T22:17:44.9217746Z 
2019-12-04T22:17:44.9217779Z 
2019-12-04T22:17:44.9218020Z ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
2019-12-04T22:17:44.9218067Z diff of stderr:
2019-12-04T22:17:44.9218095Z 
2019-12-04T22:17:44.9218417Z - thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:LL:CC
2019-12-04T22:17:44.9218737Z + thread 'rustc' panicked at src/librustc_mir/hair/pattern/_match.rs:LL:CC, 'assertion failed: rows.iter().all(|r| r.len() == v.len())'
2019-12-04T22:17:44.9218860Z 3 
2019-12-04T22:17:44.9218904Z 4 error: internal compiler error: unexpected panic
2019-12-04T22:17:44.9218935Z 
2019-12-04T22:17:44.9218960Z 
2019-12-04T22:17:44.9218960Z 
2019-12-04T22:17:44.9219021Z The actual stderr differed from the expected stderr.
2019-12-04T22:17:44.9219331Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
2019-12-04T22:17:44.9219666Z To update references, rerun the tests and pass the `--bless` flag
2019-12-04T22:17:44.9220012Z To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
2019-12-04T22:17:44.9220099Z error: 1 errors occurred comparing output.
2019-12-04T22:17:44.9220146Z status: exit code: 101
2019-12-04T22:17:44.9220146Z status: exit code: 101
2019-12-04T22:17:44.9220926Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
2019-12-04T22:17:44.9221403Z ------------------------------------------
2019-12-04T22:17:44.9221442Z 
2019-12-04T22:17:44.9221684Z ------------------------------------------
2019-12-04T22:17:44.9221750Z stderr:
2019-12-04T22:17:44.9221750Z stderr:
2019-12-04T22:17:44.9221987Z ------------------------------------------
2019-12-04T22:17:44.9222319Z thread 'rustc' panicked at src/librustc_mir/hair/pattern/_match.rs:1625:5, 'assertion failed: rows.iter().all(|r| r.len() == v.len())'
2019-12-04T22:17:44.9222438Z 
2019-12-04T22:17:44.9222484Z error: internal compiler error: unexpected panic
2019-12-04T22:17:44.9222516Z 
2019-12-04T22:17:44.9222581Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-04T22:17:44.9222581Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-04T22:17:44.9222612Z 
2019-12-04T22:17:44.9223218Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-04T22:17:44.9223579Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-12-04T22:17:44.9223622Z 
2019-12-04T22:17:44.9223622Z 
2019-12-04T22:17:44.9223911Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-04T22:17:44.9223990Z 
2019-12-04T22:17:44.9224210Z ------------------------------------------
2019-12-04T22:17:44.9224242Z 
2019-12-04T22:17:44.9224266Z 
2019-12-04T22:17:44.9224266Z 
2019-12-04T22:17:44.9224503Z ---- [ui] ui/test-panic-abort.rs stdout ----
2019-12-04T22:17:44.9224552Z diff of run.stdout:
2019-12-04T22:17:44.9224580Z 
2019-12-04T22:17:44.9224620Z 15 testing123
2019-12-04T22:17:44.9224846Z 16 ---- it_fails stderr ----
2019-12-04T22:17:44.9224892Z 17 testing321
2019-12-04T22:17:44.9225128Z - thread 'main' panicked at 'assertion failed: `(left == right)`
2019-12-04T22:17:44.9225423Z + thread 'main' panicked at $DIR/test-panic-abort.rs:31:5, 'assertion failed: `(left == right)`
2019-12-04T22:17:44.9225708Z -  right: `5`', $DIR/test-panic-abort.rs:31:5
2019-12-04T22:17:44.9225923Z +  right: `5`'
2019-12-04T22:17:44.9225983Z 21 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-04T22:17:44.9226028Z 22 
2019-12-04T22:17:44.9226028Z 22 
2019-12-04T22:17:44.9226085Z 23 
2019-12-04T22:17:44.9226111Z 
2019-12-04T22:17:44.9226136Z 
2019-12-04T22:17:44.9226181Z The actual run.stdout differed from the expected run.stdout.
2019-12-04T22:17:44.9226503Z Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/test-panic-abort.run.stdout
2019-12-04T22:17:44.9226585Z error: 1 errors occured comparing run output.
2019-12-04T22:17:44.9226629Z status: exit code: 101
2019-12-04T22:17:44.9226629Z status: exit code: 101
2019-12-04T22:17:44.9226925Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a" "--test-threads=1"
2019-12-04T22:17:44.9227191Z ------------------------------------------
2019-12-04T22:17:44.9227250Z 
2019-12-04T22:17:44.9227290Z running 4 tests
2019-12-04T22:17:44.9227331Z test it_exits ... FAILED
---
2019-12-04T22:17:44.9228447Z hello, world
2019-12-04T22:17:44.9228488Z testing123
2019-12-04T22:17:44.9228704Z ---- it_fails stderr ----
2019-12-04T22:17:44.9228748Z testing321
2019-12-04T22:17:44.9229030Z thread 'main' panicked at /checkout/src/test/ui/test-panic-abort.rs:31:5, 'assertion failed: `(left == right)`
2019-12-04T22:17:44.9229283Z  right: `5`'
2019-12-04T22:17:44.9229419Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-04T22:17:44.9229452Z 
2019-12-04T22:17:44.9229504Z 
---
2019-12-04T22:17:44.9234407Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-04T22:17:44.9234484Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-04T22:17:44.9234533Z 
2019-12-04T22:17:44.9234559Z 
2019-12-04T22:17:44.9236255Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-04T22:17:44.9236520Z 
2019-12-04T22:17:44.9236549Z 
2019-12-04T22:17:44.9236595Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-04T22:17:44.9236662Z Build completed unsuccessfully in 1:04:07
2019-12-04T22:17:44.9236662Z Build completed unsuccessfully in 1:04:07
2019-12-04T22:17:44.9236707Z == clock drift check ==
2019-12-04T22:17:44.9236793Z   local time: Wed Dec  4 22:17:44 UTC 2019
2019-12-04T22:17:45.0100860Z   network time: Wed, 04 Dec 2019 22:17:45 GMT
2019-12-04T22:17:45.0101655Z == end clock drift check ==
2019-12-04T22:17:45.6903913Z 
2019-12-04T22:17:45.7043890Z ##[error]Bash exited with code '1'.
2019-12-04T22:17:45.7057574Z ##[section]Finishing: Run build
2019-12-04T22:17:45.7080335Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67034/merge to s
2019-12-04T22:17:45.7082461Z Task         : Get sources
2019-12-04T22:17:45.7082534Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-04T22:17:45.7082585Z Version      : 1.0.0
2019-12-04T22:17:45.7082632Z Author       : Microsoft
2019-12-04T22:17:45.7082632Z Author       : Microsoft
2019-12-04T22:17:45.7082703Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2019-12-04T22:17:45.7082758Z ==============================================================================
2019-12-04T22:17:46.1808907Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2019-12-04T22:17:46.1852046Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67034/merge to s
2019-12-04T22:17:46.1958918Z Start cleaning up orphan processes.
2019-12-04T22:17:46.2069014Z Terminate orphan process: pid (3638) (python)
2019-12-04T22:17:46.2375380Z ##[section]Finishing: Finalize Job
2019-12-04T22:17:46.2429963Z ##[section]Finishing: Linux x86_64-gnu-llvm-7
