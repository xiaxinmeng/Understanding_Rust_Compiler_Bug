plain
2019-12-31T20:51:48.9593765Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T20:51:48.9842602Z ##[command]git config gc.auto 0
2019-12-31T20:51:48.9921481Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T20:51:48.9975875Z ##[command]git config --get-all http.proxy
2019-12-31T20:51:49.0110645Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67000/merge:refs/remotes/pull/67000/merge
---
2019-12-31T21:44:46.7055564Z .................................................................................................... 1100/9463
2019-12-31T21:44:51.7248430Z ..........i......................................................................................... 1200/9463
2019-12-31T21:44:58.2488715Z .................................................................................................... 1300/9463
2019-12-31T21:45:04.0692577Z ..............................................................F..................................... 1400/9463
2019-12-31T21:45:09.8757495Z .......................F.............F.......................FF.........................FF.......... 1500/9463
2019-12-31T21:45:19.5429123Z .................................................................................................... 1700/9463
2019-12-31T21:45:28.2434858Z ............F....................................................................................... 1800/9463
2019-12-31T21:45:35.7870869Z i................................................................................................... 1900/9463
2019-12-31T21:45:35.7870869Z i................................................................................................... 1900/9463
2019-12-31T21:45:41.9543650Z ......................................................................................iiiii......... 2000/9463
2019-12-31T21:46:01.8369804Z .................................................................................................... 2200/9463
2019-12-31T21:46:04.0248228Z .................................................................................................... 2300/9463
2019-12-31T21:46:06.2933684Z .................................................................................................... 2400/9463
2019-12-31T21:46:12.0944583Z .................................................................................................... 2500/9463
---
2019-12-31T21:48:50.9862176Z ................i...............i................................................................... 4900/9463
2019-12-31T21:48:59.6630871Z .................................................................................................... 5000/9463
2019-12-31T21:49:04.7760102Z .............................................................i...................................... 5100/9463
2019-12-31T21:49:12.5258978Z .................................................................................................... 5200/9463
2019-12-31T21:49:19.6861024Z ............................ii.ii...........i....................................................... 5300/9463
2019-12-31T21:49:28.2832255Z .................................................................................................... 5500/9463
2019-12-31T21:49:37.8084271Z .................................................................................................... 5600/9463
2019-12-31T21:49:44.3294007Z ...........i........................................................................................ 5700/9463
2019-12-31T21:49:50.1064567Z .................................................................................................... 5800/9463
2019-12-31T21:49:50.1064567Z .................................................................................................... 5800/9463
2019-12-31T21:49:59.7101626Z ...................................................................................................i 5900/9463
2019-12-31T21:50:10.5127311Z i...i..ii...........i............................................................................... 6000/9463
2019-12-31T21:50:26.6001364Z .................................................................................................... 6200/9463
2019-12-31T21:50:33.2287629Z .................................................................................................... 6300/9463
2019-12-31T21:50:33.2287629Z .................................................................................................... 6300/9463
2019-12-31T21:50:50.0147607Z ..........................i..ii..................................................................... 6400/9463
2019-12-31T21:51:08.8976748Z .................................................................................................... 6600/9463
2019-12-31T21:51:10.7177427Z .i.................................................................................................. 6700/9463
2019-12-31T21:51:12.7686807Z .................................................................................................... 6800/9463
2019-12-31T21:51:15.2221393Z .i.................................................................................................. 6900/9463
---
2019-12-31T21:52:41.5860393Z .................................................................................................... 7500/9463
2019-12-31T21:52:45.9697258Z .................................................................................................... 7600/9463
2019-12-31T21:52:50.8959103Z .................................................................................................... 7700/9463
2019-12-31T21:52:59.7236592Z .................................................................................................... 7800/9463
2019-12-31T21:53:06.4728065Z ................................iiii................................................................ 7900/9463
2019-12-31T21:53:19.5149990Z .................................................................................................... 8100/9463
2019-12-31T21:53:27.3880284Z .................................................................................................... 8200/9463
2019-12-31T21:53:40.3137813Z .................................................................................................... 8300/9463
2019-12-31T21:53:47.0532945Z .................................................................................................... 8400/9463
---
2019-12-31T21:55:28.7553542Z - error: erroneous constant used
2019-12-31T21:55:28.7553914Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7554450Z 18   --> $DIR/array-literal-index-oob.rs:4:5
2019-12-31T21:55:28.7554924Z 19    |
2019-12-31T21:55:28.7555051Z 20 LL |     &{[1, 2, 3][4]};
2019-12-31T21:55:28.7556002Z 22 
2019-12-31T21:55:28.7556140Z 23 error: aborting due to 3 previous errors
2019-12-31T21:55:28.7556327Z 24 
2019-12-31T21:55:28.7556943Z + For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7556943Z + For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7557278Z 25 
2019-12-31T21:55:28.7557584Z 
2019-12-31T21:55:28.7558317Z 
2019-12-31T21:55:28.7558504Z The actual stderr differed from the expected stderr.
2019-12-31T21:55:28.7559129Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/array-literal-index-oob.stderr
2019-12-31T21:55:28.7559772Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T21:55:28.7560854Z To only update this specific test, also pass `--test-args consts/array-literal-index-oob.rs`
2019-12-31T21:55:28.7561327Z error: 1 errors occurred comparing output.
2019-12-31T21:55:28.7561455Z status: exit code: 1
2019-12-31T21:55:28.7561455Z status: exit code: 1
2019-12-31T21:55:28.7562659Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/array-literal-index-oob.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/auxiliary" "-A" "unused"
2019-12-31T21:55:28.7566154Z ------------------------------------------
2019-12-31T21:55:28.7566188Z 
2019-12-31T21:55:28.7566381Z ------------------------------------------
2019-12-31T21:55:28.7566443Z stderr:
2019-12-31T21:55:28.7566443Z stderr:
2019-12-31T21:55:28.7566850Z ------------------------------------------
2019-12-31T21:55:28.7566906Z error: index out of bounds: the len is 3 but the index is 4
2019-12-31T21:55:28.7567149Z   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:4:7
2019-12-31T21:55:28.7567197Z    |
2019-12-31T21:55:28.7567237Z LL |     &{[1, 2, 3][4]};
2019-12-31T21:55:28.7567332Z    |
2019-12-31T21:55:28.7567373Z    = note: `#[deny(const_err)]` on by default
2019-12-31T21:55:28.7567401Z 
2019-12-31T21:55:28.7567528Z error: reaching this expression at runtime will panic or abort
2019-12-31T21:55:28.7567528Z error: reaching this expression at runtime will panic or abort
2019-12-31T21:55:28.7567757Z   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:4:7
2019-12-31T21:55:28.7567801Z    |
2019-12-31T21:55:28.7567840Z LL |     &{[1, 2, 3][4]};
2019-12-31T21:55:28.7568043Z    |     --^^^^^^^^^^^^-
2019-12-31T21:55:28.7568128Z    |       indexing out of bounds: the len is 3 but the index is 4
2019-12-31T21:55:28.7568174Z 
2019-12-31T21:55:28.7568212Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7568594Z   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:4:5
2019-12-31T21:55:28.7568594Z   --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:4:5
2019-12-31T21:55:28.7568636Z    |
2019-12-31T21:55:28.7568690Z LL |     &{[1, 2, 3][4]};
2019-12-31T21:55:28.7568898Z    |     ^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7568978Z error: aborting due to 3 previous errors
2019-12-31T21:55:28.7569002Z 
2019-12-31T21:55:28.7569209Z For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7569417Z 
---
2019-12-31T21:55:28.7572057Z 
2019-12-31T21:55:28.7572079Z 
2019-12-31T21:55:28.7572119Z The actual stderr differed from the expected stderr.
2019-12-31T21:55:28.7572439Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/conditional_array_execution.stderr
2019-12-31T21:55:28.7572690Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T21:55:28.7572947Z To only update this specific test, also pass `--test-args consts/const-eval/conditional_array_execution.rs`
2019-12-31T21:55:28.7573036Z error: 1 errors occurred comparing output.
2019-12-31T21:55:28.7573074Z status: exit code: 1
2019-12-31T21:55:28.7573074Z status: exit code: 1
2019-12-31T21:55:28.7573853Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/auxiliary" "-A" "unused"
2019-12-31T21:55:28.7574178Z ------------------------------------------
2019-12-31T21:55:28.7574208Z 
2019-12-31T21:55:28.7574394Z ------------------------------------------
2019-12-31T21:55:28.7574434Z stderr:
2019-12-31T21:55:28.7574434Z stderr:
2019-12-31T21:55:28.7574636Z ------------------------------------------
2019-12-31T21:55:28.7574680Z warning: any use of this value will cause an error
2019-12-31T21:55:28.7574910Z   --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:7:19
2019-12-31T21:55:28.7574971Z    |
2019-12-31T21:55:28.7575187Z LL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];
2019-12-31T21:55:28.7575446Z    |                   |
2019-12-31T21:55:28.7575487Z    |                   attempt to subtract with overflow
2019-12-31T21:55:28.7575525Z    |
2019-12-31T21:55:28.7575663Z note: lint level defined here
---
2019-12-31T21:55:28.7578485Z 16 
2019-12-31T21:55:28.7578527Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7578716Z +   --> $DIR/const_fn_ptr_fail2.rs:18:5
2019-12-31T21:55:28.7578773Z +    |
2019-12-31T21:55:28.7578817Z + LL |     assert_eq!(Y, 4);
2019-12-31T21:55:28.7578860Z +    |     ^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7579199Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-31T21:55:28.7579253Z + 
2019-12-31T21:55:28.7579313Z 17 error[E0080]: evaluation of constant expression failed
2019-12-31T21:55:28.7579536Z 18   --> $DIR/const_fn_ptr_fail2.rs:20:5
---
2019-12-31T21:55:28.7580244Z - error: aborting due to 2 previous errors
2019-12-31T21:55:28.7580289Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7582629Z +   --> $DIR/const_fn_ptr_fail2.rs:20:5
2019-12-31T21:55:28.7582700Z +    |
2019-12-31T21:55:28.7582737Z + LL |     assert_eq!(Z, 4);
2019-12-31T21:55:28.7582779Z +    |     ^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7583117Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-31T21:55:28.7583166Z + 
2019-12-31T21:55:28.7583205Z + error: aborting due to 4 previous errors
2019-12-31T21:55:28.7583572Z 28 
2019-12-31T21:55:28.7583572Z 28 
2019-12-31T21:55:28.7583902Z 29 For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7583954Z 30 
2019-12-31T21:55:28.7583999Z 
2019-12-31T21:55:28.7584020Z 
2019-12-31T21:55:28.7584059Z The actual stderr differed from the expected stderr.
2019-12-31T21:55:28.7584335Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2/const_fn_ptr_fail2.stderr
2019-12-31T21:55:28.7584575Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T21:55:28.7584813Z To only update this specific test, also pass `--test-args consts/const-eval/const_fn_ptr_fail2.rs`
2019-12-31T21:55:28.7584902Z error: 1 errors occurred comparing output.
2019-12-31T21:55:28.7584940Z status: exit code: 1
2019-12-31T21:55:28.7584940Z status: exit code: 1
2019-12-31T21:55:28.7587248Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2/auxiliary" "-A" "unused"
2019-12-31T21:55:28.7587645Z ------------------------------------------
2019-12-31T21:55:28.7587695Z 
2019-12-31T21:55:28.7587882Z ------------------------------------------
2019-12-31T21:55:28.7587920Z stderr:
2019-12-31T21:55:28.7587920Z stderr:
2019-12-31T21:55:28.7588114Z ------------------------------------------
2019-12-31T21:55:28.7588157Z warning: skipping const checks
2019-12-31T21:55:28.7588370Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:11:5
2019-12-31T21:55:28.7588412Z    |
2019-12-31T21:55:28.7588470Z LL |     x(y) //~ WARN skipping const checks
2019-12-31T21:55:28.7588619Z 
2019-12-31T21:55:28.7588674Z error[E0080]: evaluation of constant expression failed
2019-12-31T21:55:28.7588917Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:18:5
2019-12-31T21:55:28.7588958Z    |
---
2019-12-31T21:55:28.7590405Z 
2019-12-31T21:55:28.7590449Z error[E0080]: evaluation of constant expression failed
2019-12-31T21:55:28.7590665Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:20:5
2019-12-31T21:55:28.7590724Z    |
2019-12-31T21:55:28.7590760Z LL |     assert_eq!(Z, 4);
2019-12-31T21:55:28.7590926Z    |     ^^^^^^^^^^^-^^^^^
2019-12-31T21:55:28.7591022Z    |                referenced constant has errors
2019-12-31T21:55:28.7591058Z    |
2019-12-31T21:55:28.7591325Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-31T21:55:28.7591383Z 
2019-12-31T21:55:28.7591383Z 
2019-12-31T21:55:28.7591420Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7591632Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:20:5
2019-12-31T21:55:28.7591691Z    |
2019-12-31T21:55:28.7591726Z LL |     assert_eq!(Z, 4);
2019-12-31T21:55:28.7591766Z    |     ^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7592098Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-31T21:55:28.7592133Z 
2019-12-31T21:55:28.7592170Z error: aborting due to 4 previous errors
2019-12-31T21:55:28.7592211Z 
---
2019-12-31T21:55:28.7592688Z 
2019-12-31T21:55:28.7592882Z ---- [ui] ui/consts/const-eval/issue-43197.rs stdout ----
2019-12-31T21:55:28.7592931Z diff of stderr:
2019-12-31T21:55:28.7592972Z 
2019-12-31T21:55:28.7593008Z 26 LL |     println!("{} {}", X, Y);
2019-12-31T21:55:28.7593049Z 27    |                       ^ referenced constant has errors
2019-12-31T21:55:28.7593284Z - warning: erroneous constant used
2019-12-31T21:55:28.7593396Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7593604Z 30   --> $DIR/issue-43197.rs:14:23
2019-12-31T21:55:28.7593661Z 31    |
2019-12-31T21:55:28.7593661Z 31    |
2019-12-31T21:55:28.7593699Z 32 LL |     println!("{} {}", X, Y);
2019-12-31T21:55:28.7593724Z 
2019-12-31T21:55:28.7593759Z 38 LL |     println!("{} {}", X, Y);
2019-12-31T21:55:28.7593818Z 39    |                          ^ referenced constant has errors
2019-12-31T21:55:28.7594034Z - warning: erroneous constant used
2019-12-31T21:55:28.7594093Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7594271Z 42   --> $DIR/issue-43197.rs:14:26
2019-12-31T21:55:28.7594382Z 43    |
2019-12-31T21:55:28.7594382Z 43    |
2019-12-31T21:55:28.7594435Z 44 LL |     println!("{} {}", X, Y);
2019-12-31T21:55:28.7594499Z 45    |                          ^ referenced constant has errors
2019-12-31T21:55:28.7594536Z 46 
2019-12-31T21:55:28.7594759Z - error: aborting due to 2 previous errors
2019-12-31T21:55:28.7594809Z + error: aborting due to 4 previous errors
2019-12-31T21:55:28.7594809Z + error: aborting due to 4 previous errors
2019-12-31T21:55:28.7594887Z 48 
2019-12-31T21:55:28.7595126Z 49 For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7595166Z 50 
2019-12-31T21:55:28.7595188Z 
2019-12-31T21:55:28.7595210Z 
2019-12-31T21:55:28.7595268Z The actual stderr differed from the expected stderr.
2019-12-31T21:55:28.7595530Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/issue-43197.stderr
2019-12-31T21:55:28.7595738Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T21:55:28.7595982Z To only update this specific test, also pass `--test-args consts/const-eval/issue-43197.rs`
2019-12-31T21:55:28.7596058Z error: 1 errors occurred comparing output.
2019-12-31T21:55:28.7596095Z status: exit code: 1
2019-12-31T21:55:28.7596095Z status: exit code: 1
2019-12-31T21:55:28.7596836Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-43197.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/auxiliary" "-A" "unused"
2019-12-31T21:55:28.7597120Z ------------------------------------------
2019-12-31T21:55:28.7597148Z 
2019-12-31T21:55:28.7597339Z ------------------------------------------
2019-12-31T21:55:28.7597395Z stderr:
---
2019-12-31T21:55:28.7598779Z 
2019-12-31T21:55:28.7598823Z warning: any use of this value will cause an error
2019-12-31T21:55:28.7599053Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:12:24
2019-12-31T21:55:28.7599093Z    |
2019-12-31T21:55:28.7599267Z LL |     const Y: u32 = foo(0-1);
2019-12-31T21:55:28.7599459Z    |     -------------------^^^--
2019-12-31T21:55:28.7599611Z    |                        attempt to subtract with overflow
2019-12-31T21:55:28.7599658Z 
2019-12-31T21:55:28.7599696Z error[E0080]: evaluation of constant expression failed
2019-12-31T21:55:28.7599928Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:23
2019-12-31T21:55:28.7599928Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:23
2019-12-31T21:55:28.7599969Z    |
2019-12-31T21:55:28.7600021Z LL |     println!("{} {}", X, Y);
2019-12-31T21:55:28.7600061Z    |                       ^ referenced constant has errors
2019-12-31T21:55:28.7600141Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7600349Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:23
2019-12-31T21:55:28.7600457Z    |
2019-12-31T21:55:28.7600457Z    |
2019-12-31T21:55:28.7600492Z LL |     println!("{} {}", X, Y);
2019-12-31T21:55:28.7600548Z    |                       ^ referenced constant has errors
2019-12-31T21:55:28.7600610Z error[E0080]: evaluation of constant expression failed
2019-12-31T21:55:28.7600867Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:26
2019-12-31T21:55:28.7600909Z    |
2019-12-31T21:55:28.7600909Z    |
2019-12-31T21:55:28.7600944Z LL |     println!("{} {}", X, Y);
2019-12-31T21:55:28.7601001Z    |                          ^ referenced constant has errors
2019-12-31T21:55:28.7601063Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7601274Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:26
2019-12-31T21:55:28.7601329Z    |
2019-12-31T21:55:28.7601329Z    |
2019-12-31T21:55:28.7601364Z LL |     println!("{} {}", X, Y);
2019-12-31T21:55:28.7601403Z    |                          ^ referenced constant has errors
2019-12-31T21:55:28.7601488Z error: aborting due to 4 previous errors
2019-12-31T21:55:28.7601512Z 
2019-12-31T21:55:28.7601722Z For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7601751Z 
2019-12-31T21:55:28.7601751Z 
2019-12-31T21:55:28.7601950Z ------------------------------------------
2019-12-31T21:55:28.7601977Z 
2019-12-31T21:55:28.7601998Z 
2019-12-31T21:55:28.7602374Z ---- [ui] ui/consts/const-eval/issue-44578.rs stdout ----
2019-12-31T21:55:28.7602430Z diff of stderr:
2019-12-31T21:55:28.7602453Z 
2019-12-31T21:55:28.7602489Z 4 LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
2019-12-31T21:55:28.7602531Z 5    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7602762Z - error: aborting due to previous error
2019-12-31T21:55:28.7602802Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7602992Z +   --> $DIR/issue-44578.rs:27:20
2019-12-31T21:55:28.7603030Z +    |
2019-12-31T21:55:28.7603030Z +    |
2019-12-31T21:55:28.7603066Z + LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
2019-12-31T21:55:28.7603131Z +    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7603203Z + error: aborting due to 2 previous errors
2019-12-31T21:55:28.7603253Z 8 
2019-12-31T21:55:28.7603463Z 9 For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7603503Z 10 
2019-12-31T21:55:28.7603503Z 10 
2019-12-31T21:55:28.7603524Z 
2019-12-31T21:55:28.7603545Z 
2019-12-31T21:55:28.7603597Z The actual stderr differed from the expected stderr.
2019-12-31T21:55:28.7603851Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578/issue-44578.stderr
2019-12-31T21:55:28.7604293Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T21:55:28.7604527Z To only update this specific test, also pass `--test-args consts/const-eval/issue-44578.rs`
2019-12-31T21:55:28.7604594Z error: 1 errors occurred comparing output.
2019-12-31T21:55:28.7604655Z status: exit code: 1
2019-12-31T21:55:28.7604655Z status: exit code: 1
2019-12-31T21:55:28.7605444Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-44578.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578/auxiliary" "-A" "unused"
2019-12-31T21:55:28.7605909Z ------------------------------------------
2019-12-31T21:55:28.7605939Z 
2019-12-31T21:55:28.7606135Z ------------------------------------------
2019-12-31T21:55:28.7606173Z stderr:
2019-12-31T21:55:28.7606173Z stderr:
2019-12-31T21:55:28.7606343Z ------------------------------------------
2019-12-31T21:55:28.7606483Z error[E0080]: evaluation of constant expression failed
2019-12-31T21:55:28.7606710Z   --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:27:20
2019-12-31T21:55:28.7606752Z    |
2019-12-31T21:55:28.7606807Z LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
2019-12-31T21:55:28.7606856Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7606917Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7607141Z   --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:27:20
2019-12-31T21:55:28.7607180Z    |
2019-12-31T21:55:28.7607180Z    |
2019-12-31T21:55:28.7607217Z LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
2019-12-31T21:55:28.7607275Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7607336Z error: aborting due to 2 previous errors
2019-12-31T21:55:28.7607359Z 
2019-12-31T21:55:28.7607587Z For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7607616Z 
2019-12-31T21:55:28.7607616Z 
2019-12-31T21:55:28.7607788Z ------------------------------------------
2019-12-31T21:55:28.7607814Z 
2019-12-31T21:55:28.7607855Z 
2019-12-31T21:55:28.7608045Z ---- [ui] ui/consts/const-eval/promoted_errors.rs stdout ----
2019-12-31T21:55:28.7608084Z diff of stderr:
2019-12-31T21:55:28.7608114Z 
2019-12-31T21:55:28.7608301Z 22 LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7608342Z 23    |                    ^^^^^^^ dividing by zero
2019-12-31T21:55:28.7608559Z - error: erroneous constant used
2019-12-31T21:55:28.7608599Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7608773Z 26   --> $DIR/promoted_errors.rs:10:20
2019-12-31T21:55:28.7608810Z 27    |
2019-12-31T21:55:28.7608810Z 27    |
2019-12-31T21:55:28.7608997Z 28 LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7609024Z 
2019-12-31T21:55:28.7609060Z 46 LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7609115Z 47    |                    ^^^^^^^^^^^^^^^^ dividing by zero
2019-12-31T21:55:28.7609325Z - error: erroneous constant used
2019-12-31T21:55:28.7609365Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7609556Z 50   --> $DIR/promoted_errors.rs:16:20
2019-12-31T21:55:28.7609593Z 51    |
2019-12-31T21:55:28.7609593Z 51    |
2019-12-31T21:55:28.7609635Z 52 LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7609706Z 60 
2019-12-31T21:55:28.7609742Z 61 error: aborting due to 9 previous errors
2019-12-31T21:55:28.7609775Z 62 
2019-12-31T21:55:28.7609995Z + For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7609995Z + For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7610034Z 63 
2019-12-31T21:55:28.7610055Z 
2019-12-31T21:55:28.7610076Z 
2019-12-31T21:55:28.7610128Z The actual stderr differed from the expected stderr.
2019-12-31T21:55:28.7610386Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/promoted_errors.stderr
2019-12-31T21:55:28.7610938Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T21:55:28.7611363Z To only update this specific test, also pass `--test-args consts/const-eval/promoted_errors.rs`
2019-12-31T21:55:28.7611625Z error: 1 errors occurred comparing output.
2019-12-31T21:55:28.7611683Z status: exit code: 1
2019-12-31T21:55:28.7611683Z status: exit code: 1
2019-12-31T21:55:28.7615379Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/auxiliary" "-A" "unused"
2019-12-31T21:55:28.7615804Z ------------------------------------------
2019-12-31T21:55:28.7615835Z 
2019-12-31T21:55:28.7616202Z ------------------------------------------
2019-12-31T21:55:28.7616241Z stderr:
---
2019-12-31T21:55:28.7619378Z 
2019-12-31T21:55:28.7619415Z error: attempt to divide by zero
2019-12-31T21:55:28.7619637Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:10:20
2019-12-31T21:55:28.7619695Z    |
2019-12-31T21:55:28.7619875Z LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7619964Z 
2019-12-31T21:55:28.7620005Z error: reaching this expression at runtime will panic or abort
2019-12-31T21:55:28.7620225Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:10:20
2019-12-31T21:55:28.7620267Z    |
2019-12-31T21:55:28.7620267Z    |
2019-12-31T21:55:28.7620464Z LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7620508Z    |                    ^^^^^^^ dividing by zero
2019-12-31T21:55:28.7620586Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7620803Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:10:20
2019-12-31T21:55:28.7620844Z    |
2019-12-31T21:55:28.7620844Z    |
2019-12-31T21:55:28.7621020Z LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7621087Z    |                    ^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7621150Z error: attempt to divide by zero
2019-12-31T21:55:28.7621552Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:14:14
2019-12-31T21:55:28.7621592Z    |
2019-12-31T21:55:28.7621592Z    |
2019-12-31T21:55:28.7621764Z LL |     let _x = 1/(1-1);
2019-12-31T21:55:28.7621844Z 
2019-12-31T21:55:28.7621879Z error: attempt to divide by zero
2019-12-31T21:55:28.7622089Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:16:20
2019-12-31T21:55:28.7622146Z    |
2019-12-31T21:55:28.7622146Z    |
2019-12-31T21:55:28.7622183Z LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7622475Z 
2019-12-31T21:55:28.7622532Z error: reaching this expression at runtime will panic or abort
2019-12-31T21:55:28.7623122Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:16:20
2019-12-31T21:55:28.7623171Z    |
2019-12-31T21:55:28.7623171Z    |
2019-12-31T21:55:28.7623225Z LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7623266Z    |                    ^^^^^^^^^^^^^^^^ dividing by zero
2019-12-31T21:55:28.7623328Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7623681Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:16:20
2019-12-31T21:55:28.7623730Z    |
2019-12-31T21:55:28.7623730Z    |
2019-12-31T21:55:28.7623768Z LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7623829Z    |                    ^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7623893Z error: attempt to divide by zero
2019-12-31T21:55:28.7624140Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:20:14
2019-12-31T21:55:28.7624198Z    |
2019-12-31T21:55:28.7624198Z    |
2019-12-31T21:55:28.7624235Z LL |     let _x = 1/(false as u32);
2019-12-31T21:55:28.7624317Z 
2019-12-31T21:55:28.7624354Z error: aborting due to 9 previous errors
2019-12-31T21:55:28.7624458Z 
2019-12-31T21:55:28.7624693Z For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7624693Z For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7624740Z 
2019-12-31T21:55:28.7625103Z ------------------------------------------
2019-12-31T21:55:28.7625132Z 
2019-12-31T21:55:28.7625155Z 
2019-12-31T21:55:28.7626635Z ---- [ui] ui/consts/const-eval/promoted_errors2.rs stdout ----
2019-12-31T21:55:28.7626720Z diff of stderr:
2019-12-31T21:55:28.7626750Z 
2019-12-31T21:55:28.7628656Z 28 LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7628761Z 29    |                    ^^^^^^^ dividing by zero
2019-12-31T21:55:28.7629098Z - error: erroneous constant used
2019-12-31T21:55:28.7629210Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7629450Z 32   --> $DIR/promoted_errors2.rs:11:20
2019-12-31T21:55:28.7629498Z 33    |
2019-12-31T21:55:28.7629498Z 33    |
2019-12-31T21:55:28.7630971Z 34 LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7631187Z 
2019-12-31T21:55:28.7631240Z 52 LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7631452Z 53    |                    ^^^^^^^^^^^^^^^^ dividing by zero
2019-12-31T21:55:28.7631689Z - error: erroneous constant used
2019-12-31T21:55:28.7631730Z + error[E0080]: erroneous constant used
2019-12-31T21:55:28.7631917Z 56   --> $DIR/promoted_errors2.rs:17:20
2019-12-31T21:55:28.7631971Z 57    |
2019-12-31T21:55:28.7631971Z 57    |
2019-12-31T21:55:28.7632008Z 58 LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7632065Z 66 
2019-12-31T21:55:28.7632121Z 67 error: aborting due to 10 previous errors
2019-12-31T21:55:28.7632156Z 68 
2019-12-31T21:55:28.7632530Z + For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7632530Z + For more information about this error, try `rustc --explain E0080`.
2019-12-31T21:55:28.7632587Z 69 
2019-12-31T21:55:28.7632610Z 
2019-12-31T21:55:28.7632633Z 
2019-12-31T21:55:28.7632671Z The actual stderr differed from the expected stderr.
2019-12-31T21:55:28.7634294Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors2/promoted_errors2.stderr
2019-12-31T21:55:28.7634796Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T21:55:28.7635061Z To only update this specific test, also pass `--test-args consts/const-eval/promoted_errors2.rs`
2019-12-31T21:55:28.7635163Z error: 1 errors occurred comparing output.
2019-12-31T21:55:28.7635202Z status: exit code: 1
2019-12-31T21:55:28.7635202Z status: exit code: 1
2019-12-31T21:55:28.7636408Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors2" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors2/auxiliary" "-A" "unused"
2019-12-31T21:55:28.7639799Z ------------------------------------------
2019-12-31T21:55:28.7639848Z 
2019-12-31T21:55:28.7640316Z ------------------------------------------
2019-12-31T21:55:28.7641666Z stderr:
2019-12-31T21:55:28.7641666Z stderr:
2019-12-31T21:55:28.7642281Z ------------------------------------------
2019-12-31T21:55:28.7647113Z error: attempt to subtract with overflow
2019-12-31T21:55:28.7647526Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:7:20
2019-12-31T21:55:28.7647579Z    |
2019-12-31T21:55:28.7647782Z LL |     println!("{}", 0u32 - 1);
2019-12-31T21:55:28.7647880Z    |
2019-12-31T21:55:28.7647918Z note: lint level defined here
2019-12-31T21:55:28.7648139Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:4:9
2019-12-31T21:55:28.7648199Z    |
---
2019-12-31T21:55:28.7649083Z 
2019-12-31T21:55:28.7649121Z error: attempt to divide by zero
2019-12-31T21:55:28.7649360Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:11:20
2019-12-31T21:55:28.7649404Z    |
2019-12-31T21:55:28.7649583Z LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7649667Z 
2019-12-31T21:55:28.7649707Z error: reaching this expression at runtime will panic or abort
2019-12-31T21:55:28.7649928Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:11:20
2019-12-31T21:55:28.7649995Z    |
2019-12-31T21:55:28.7649995Z    |
2019-12-31T21:55:28.7650178Z LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7650222Z    |                    ^^^^^^^ dividing by zero
2019-12-31T21:55:28.7650302Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7650519Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:11:20
2019-12-31T21:55:28.7650582Z    |
2019-12-31T21:55:28.7650582Z    |
2019-12-31T21:55:28.7650763Z LL |     println!("{}", 1/(1-1));
2019-12-31T21:55:28.7650808Z    |                    ^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7650888Z error: attempt to divide by zero
2019-12-31T21:55:28.7651106Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:15:14
2019-12-31T21:55:28.7651148Z    |
2019-12-31T21:55:28.7651148Z    |
2019-12-31T21:55:28.7651334Z LL |     let _x = 1/(1-1);
2019-12-31T21:55:28.7651399Z 
2019-12-31T21:55:28.7651434Z error: attempt to divide by zero
2019-12-31T21:55:28.7651667Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:17:20
2019-12-31T21:55:28.7651717Z    |
2019-12-31T21:55:28.7651717Z    |
2019-12-31T21:55:28.7651754Z LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7651842Z 
2019-12-31T21:55:28.7651881Z error: reaching this expression at runtime will panic or abort
2019-12-31T21:55:28.7652110Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:17:20
2019-12-31T21:55:28.7652169Z    |
2019-12-31T21:55:28.7652169Z    |
2019-12-31T21:55:28.7652207Z LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7652248Z    |                    ^^^^^^^^^^^^^^^^ dividing by zero
2019-12-31T21:55:28.7652328Z error[E0080]: erroneous constant used
2019-12-31T21:55:28.7652549Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:17:20
2019-12-31T21:55:28.7652591Z    |
2019-12-31T21:55:28.7652591Z    |
2019-12-31T21:55:28.7652644Z LL |     println!("{}", 1/(false as u32));
2019-12-31T21:55:28.7652686Z    |                    ^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-31T21:55:28.7652756Z error: attempt to divide by zero
2019-12-31T21:55:28.7652994Z   --> /checkout/src/test/ui/consts/const-eval/promoted_errors2.rs:21:14
2019-12-31T21:55:28.7653036Z    |
2019-12-31T21:55:28.7653036Z    |
2019-12-31T21:55:28.7653073Z LL |     let _x = 1/(false as u32);
2019-12-31T21:55:28.7653240Z 
2019-12-31T21:55:28.7653277Z error: aborting due to 10 previous errors
2019-12-31T21:55:28.7653301Z 
2019-12-31T21:55:28.7653556Z For more information about this error, try `rustc --explain E0080`.
---
2019-12-31T21:55:28.7655589Z 36 
2019-12-31T21:55:28.7655612Z 
2019-12-31T21:55:28.7655634Z 
2019-12-31T21:55:28.7655690Z The actual stderr differed from the expected stderr.
2019-12-31T21:55:28.7655972Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn/non_const_fn.stderr
2019-12-31T21:55:28.7656193Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T21:55:28.7656468Z To only update this specific test, also pass `--test-args consts/miri_unleashed/non_const_fn.rs`
2019-12-31T21:55:28.7656540Z error: 1 errors occurred comparing output.
2019-12-31T21:55:28.7656595Z status: exit code: 1
2019-12-31T21:55:28.7656595Z status: exit code: 1
2019-12-31T21:55:28.7657377Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn/auxiliary" "-A" "unused"
2019-12-31T21:55:28.7657682Z ------------------------------------------
2019-12-31T21:55:28.7657712Z 
2019-12-31T21:55:28.7657925Z ------------------------------------------
2019-12-31T21:55:28.7657964Z stderr:
2019-12-31T21:55:28.7657964Z stderr:
2019-12-31T21:55:28.7658149Z ------------------------------------------
2019-12-31T21:55:28.7658207Z warning: skipping const checks
2019-12-31T21:55:28.7658422Z   --> /checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs:10:15
2019-12-31T21:55:28.7658464Z    |
2019-12-31T21:55:28.7658521Z LL | const C: () = foo(); //~ WARN: skipping const checks
2019-12-31T21:55:28.7658584Z 
2019-12-31T21:55:28.7658622Z warning: any use of this value will cause an error
2019-12-31T21:55:28.7658859Z   --> /checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs:10:15
2019-12-31T21:55:28.7658908Z    |
2019-12-31T21:55:28.7658908Z    |
2019-12-31T21:55:28.7658947Z LL | const C: () = foo(); //~ WARN: skipping const checks
2019-12-31T21:55:28.7659178Z    |               |
2019-12-31T21:55:28.7659444Z    |               calling non-const function `foo`
2019-12-31T21:55:28.7659507Z    |
2019-12-31T21:55:28.7659545Z note: lint level defined here
---
2019-12-31T21:55:28.7663830Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-31T21:55:28.7663896Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-31T21:55:28.7663924Z 
2019-12-31T21:55:28.7663945Z 
2019-12-31T21:55:28.7665396Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-31T21:55:28.7665681Z 
2019-12-31T21:55:28.7665706Z 
2019-12-31T21:55:28.7668870Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-31T21:55:28.7669126Z Build completed unsuccessfully in 0:57:12
2019-12-31T21:55:28.7669126Z Build completed unsuccessfully in 0:57:12
2019-12-31T21:55:28.7669341Z == clock drift check ==
2019-12-31T21:55:28.7669472Z   local time: Tue Dec 31 21:55:28 UTC 2019
2019-12-31T21:55:29.0532344Z   network time: Tue, 31 Dec 2019 21:55:29 GMT
2019-12-31T21:55:29.0535718Z == end clock drift check ==
2019-12-31T21:55:30.2456758Z 
2019-12-31T21:55:30.2538342Z ##[error]Bash exited with code '1'.
2019-12-31T21:55:30.2585924Z ##[section]Starting: Checkout
2019-12-31T21:55:30.2587862Z ==============================================================================
2019-12-31T21:55:30.2587924Z Task         : Get sources
2019-12-31T21:55:30.2587963Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
