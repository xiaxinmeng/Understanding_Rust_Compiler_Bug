plain
2019-11-06T20:56:53.6344109Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T20:56:53.6512604Z ##[command]git config gc.auto 0
2019-11-06T20:56:53.6577217Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T20:56:53.6623425Z ##[command]git config --get-all http.proxy
2019-11-06T20:56:53.6760711Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66170/merge:refs/remotes/pull/66170/merge
---
2019-11-06T21:59:31.8214092Z ..................................................................F................................. 1600/9281
2019-11-06T21:59:37.4361731Z .................................................................................................... 1700/9281
2019-11-06T21:59:50.4306667Z ...............................................................i.................................... 1800/9281
2019-11-06T21:59:58.2275319Z .................................................................................................... 1900/9281
2019-11-06T22:00:11.8247591Z ...............................................iiiii................................................ 2000/9281
2019-11-06T22:00:21.5350828Z .................................................................................................... 2200/9281
2019-11-06T22:00:23.9131090Z .................................................................................................... 2300/9281
2019-11-06T22:00:27.5563115Z .................................................................................................... 2400/9281
2019-11-06T22:00:48.8342151Z .................................................................................................... 2500/9281
---
2019-11-06T22:03:23.8313384Z .F...........................................i...............i...................................... 4800/9281
2019-11-06T22:03:33.3648220Z ..F................................................................................................. 4900/9281
2019-11-06T22:03:41.7141687Z .................................................................................................... 5000/9281
2019-11-06T22:03:48.4148410Z .................................................................................................... 5100/9281
2019-11-06T22:03:58.1557722Z ..............................................ii.ii...........i..................................... 5200/9281
2019-11-06T22:04:08.1976144Z .................................................................................................... 5400/9281
2019-11-06T22:04:17.6287336Z .................................................................................................... 5500/9281
2019-11-06T22:04:24.3978385Z ............................i....................................................................... 5600/9281
2019-11-06T22:04:30.4053970Z .................................................................................................... 5700/9281
2019-11-06T22:04:30.4053970Z .................................................................................................... 5700/9281
2019-11-06T22:04:41.3631202Z .................................................................................................... 5800/9281
2019-11-06T22:04:51.9388445Z .............ii...i..ii...........i................................................................. 5900/9281
2019-11-06T22:05:10.7188478Z .................................................................................................... 6100/9281
2019-11-06T22:05:16.0237377Z .................................................................................................... 6200/9281
2019-11-06T22:05:16.0237377Z .................................................................................................... 6200/9281
2019-11-06T22:05:28.7486319Z ................................i..ii............................................................... 6300/9281
2019-11-06T22:05:47.4685232Z ...................................................................................................i 6500/9281
2019-11-06T22:05:49.4644791Z .................................................................................................... 6600/9281
2019-11-06T22:05:51.3986217Z ..............................................................................i..................... 6700/9281
2019-11-06T22:05:53.7843374Z .................................................................................................... 6800/9281
---
2019-11-06T22:07:31.4759209Z .................................................................................................... 7500/9281
2019-11-06T22:07:38.9140565Z .................................................................................................... 7600/9281
2019-11-06T22:07:48.8754767Z .................................................................................................... 7700/9281
2019-11-06T22:07:57.0806745Z .................................................................................................... 7800/9281
2019-11-06T22:08:03.1094919Z .ii......i.......................................................................................... 7900/9281
2019-11-06T22:08:22.4780666Z .................................................................................................... 8100/9281
2019-11-06T22:08:30.3828514Z .................................................................................................... 8200/9281
2019-11-06T22:08:37.8980343Z .................................................................................................... 8300/9281
2019-11-06T22:09:14.0254223Z .................................................................................................... 8400/9281
---
2019-11-06T22:10:18.3992926Z 
2019-11-06T22:10:18.3993539Z ---- [ui] ui/borrowck/issue-64453.rs stdout ----
2019-11-06T22:10:18.3993724Z diff of stderr:
2019-11-06T22:10:18.3993828Z 
2019-11-06T22:10:18.3994163Z - error[E0507]: cannot move out of static item `settings_dir`
2019-11-06T22:10:18.3995140Z -   --> $DIR/issue-64453.rs:15:37
2019-11-06T22:10:18.3995567Z -    |
2019-11-06T22:10:18.3995999Z - LL |     let settings_data = from_string(settings_dir);
2019-11-06T22:10:18.3996519Z -    |                                     ^^^^^^^^^^^^ move occurs because `settings_dir` has type `std::string::String`, which does not implement the `Copy` trait
2019-11-06T22:10:18.3997322Z - error[E0019]: static contains unimplemented expression type
2019-11-06T22:10:18.3997513Z + error[E0744]: match expression is not allowed in a static
2019-11-06T22:10:18.3997908Z 8   --> $DIR/issue-64453.rs:4:31
2019-11-06T22:10:18.3998428Z 9    |
2019-11-06T22:10:18.3998428Z 9    |
2019-11-06T22:10:18.3998540Z 10 LL | static settings_dir: String = format!("");
2019-11-06T22:10:18.3998755Z 12    |
2019-11-06T22:10:18.3999129Z 13    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.3999297Z 14 
2019-11-06T22:10:18.3999615Z - error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-11-06T22:10:18.3999615Z - error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-11-06T22:10:18.3999932Z -   --> $DIR/issue-64453.rs:4:31
2019-11-06T22:10:18.4000208Z -    |
2019-11-06T22:10:18.4000501Z - LL | static settings_dir: String = format!("");
2019-11-06T22:10:18.4001093Z -    |
2019-11-06T22:10:18.4001573Z -    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4001787Z + error: aborting due to previous error
2019-11-06T22:10:18.4001908Z 22 
2019-11-06T22:10:18.4001908Z 22 
2019-11-06T22:10:18.4002279Z - error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-11-06T22:10:18.4002632Z -   --> $DIR/issue-64453.rs:4:31
2019-11-06T22:10:18.4002920Z -    |
2019-11-06T22:10:18.4003251Z - LL | static settings_dir: String = format!("");
2019-11-06T22:10:18.4003861Z -    |
2019-11-06T22:10:18.4004260Z -    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4005004Z - 
2019-11-06T22:10:18.4005439Z - error: aborting due to 4 previous errors
---
2019-11-06T22:10:18.4007440Z 
2019-11-06T22:10:18.4007579Z 
2019-11-06T22:10:18.4007721Z The actual stderr differed from the expected stderr.
2019-11-06T22:10:18.4008470Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453/issue-64453.stderr
2019-11-06T22:10:18.4008811Z To update references, rerun the tests and pass the `--bless` flag
2019-11-06T22:10:18.4009167Z To only update this specific test, also pass `--test-args borrowck/issue-64453.rs`
2019-11-06T22:10:18.4009431Z error: 1 errors occurred comparing output.
2019-11-06T22:10:18.4009542Z status: exit code: 1
2019-11-06T22:10:18.4009542Z status: exit code: 1
2019-11-06T22:10:18.4010223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-64453.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453/auxiliary" "-A" "unused"
2019-11-06T22:10:18.4010723Z ------------------------------------------
2019-11-06T22:10:18.4011173Z 
2019-11-06T22:10:18.4011578Z ------------------------------------------
2019-11-06T22:10:18.4015814Z stderr:
2019-11-06T22:10:18.4015814Z stderr:
2019-11-06T22:10:18.4016243Z ------------------------------------------
2019-11-06T22:10:18.4016329Z error[E0744]: match expression is not allowed in a static
2019-11-06T22:10:18.4016600Z   --> /checkout/src/test/ui/borrowck/issue-64453.rs:4:31
2019-11-06T22:10:18.4016656Z    |
2019-11-06T22:10:18.4016721Z LL | static settings_dir: String = format!("");
2019-11-06T22:10:18.4016830Z    |
2019-11-06T22:10:18.4017370Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4017420Z 
2019-11-06T22:10:18.4017633Z error: aborting due to previous error
---
2019-11-06T22:10:18.4018510Z ---- [ui] ui/consts/const-eval/issue-62272.rs stdout ----
2019-11-06T22:10:18.4018537Z 
2019-11-06T22:10:18.4018722Z error: test compilation failed although it shouldn't!
2019-11-06T22:10:18.4018779Z status: exit code: 1
2019-11-06T22:10:18.4026079Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-62272.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-62272/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-62272/auxiliary"
2019-11-06T22:10:18.4026533Z ------------------------------------------
2019-11-06T22:10:18.4026571Z 
2019-11-06T22:10:18.4026829Z ------------------------------------------
2019-11-06T22:10:18.4026878Z stderr:
2019-11-06T22:10:18.4026878Z stderr:
2019-11-06T22:10:18.4027104Z ------------------------------------------
2019-11-06T22:10:18.4027174Z error[E0744]: loop is not allowed in a const
2019-11-06T22:10:18.4027434Z   --> /checkout/src/test/ui/consts/const-eval/issue-62272.rs:5:17
2019-11-06T22:10:18.4027486Z    |
2019-11-06T22:10:18.4027551Z LL | const FOO: () = loop { break; };
2019-11-06T22:10:18.4027646Z 
2019-11-06T22:10:18.4027690Z error: aborting due to previous error
2019-11-06T22:10:18.4027735Z 
2019-11-06T22:10:18.4028297Z For more information about this error, try `rustc --explain E0744`.
---
2019-11-06T22:10:18.4031507Z ---- [ui] ui/consts/const-labeled-break.rs stdout ----
2019-11-06T22:10:18.4031549Z 
2019-11-06T22:10:18.4031738Z error: test compilation failed although it shouldn't!
2019-11-06T22:10:18.4031802Z status: exit code: 1
2019-11-06T22:10:18.4032361Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-labeled-break.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-labeled-break" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-labeled-break/auxiliary" "-A" "unused"
2019-11-06T22:10:18.4032630Z ------------------------------------------
2019-11-06T22:10:18.4032656Z 
2019-11-06T22:10:18.4032845Z ------------------------------------------
2019-11-06T22:10:18.4032880Z stderr:
2019-11-06T22:10:18.4032880Z stderr:
2019-11-06T22:10:18.4033046Z ------------------------------------------
2019-11-06T22:10:18.4033100Z error[E0744]: while loop is not allowed in a const
2019-11-06T22:10:18.4033286Z   --> /checkout/src/test/ui/consts/const-labeled-break.rs:8:19
2019-11-06T22:10:18.4033324Z    |
2019-11-06T22:10:18.4033509Z LL | const CRASH: () = 'a: while break 'a {};
2019-11-06T22:10:18.4033570Z 
2019-11-06T22:10:18.4033607Z error: aborting due to previous error
2019-11-06T22:10:18.4033645Z 
2019-11-06T22:10:18.4034675Z For more information about this error, try `rustc --explain E0744`.
---
2019-11-06T22:10:18.4035962Z -   --> $DIR/issue-46843.rs:7:26
2019-11-06T22:10:18.4036039Z + error[E0744]: match expression is not allowed in a const
2019-11-06T22:10:18.4036263Z +   --> $DIR/issue-46843.rs:7:20
2019-11-06T22:10:18.4036311Z 3    |
2019-11-06T22:10:18.4036542Z - LL | pub const Q: i32 = match non_const() {
2019-11-06T22:10:18.4036795Z -    |                          ^^^^^^^^^^^
2019-11-06T22:10:18.4036964Z + LL |   pub const Q: i32 = match non_const() {
2019-11-06T22:10:18.4037397Z + LL | |
2019-11-06T22:10:18.4037454Z + LL | |
2019-11-06T22:10:18.4037495Z + LL | |     Thing::This => 1,
2019-11-06T22:10:18.4037495Z + LL | |     Thing::This => 1,
2019-11-06T22:10:18.4037558Z + LL | |     Thing::That => 0
2019-11-06T22:10:18.4037601Z + LL | | };
2019-11-06T22:10:18.4038011Z 6 
2019-11-06T22:10:18.4038918Z - error[E0019]: constant contains unimplemented expression type
2019-11-06T22:10:18.4039109Z -   --> $DIR/issue-46843.rs:7:26
2019-11-06T22:10:18.4039255Z -    |
2019-11-06T22:10:18.4039255Z -    |
2019-11-06T22:10:18.4039451Z - LL | pub const Q: i32 = match non_const() {
2019-11-06T22:10:18.4039663Z + error: aborting due to previous error
2019-11-06T22:10:18.4039711Z 12 
2019-11-06T22:10:18.4039895Z - error[E0019]: constant contains unimplemented expression type
2019-11-06T22:10:18.4040063Z -   --> $DIR/issue-46843.rs:10:5
---
2019-11-06T22:10:18.4041667Z 
2019-11-06T22:10:18.4041703Z 
2019-11-06T22:10:18.4041736Z The actual stderr differed from the expected stderr.
2019-11-06T22:10:18.4041961Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46843/issue-46843.stderr
2019-11-06T22:10:18.4042168Z To update references, rerun the tests and pass the `--bless` flag
2019-11-06T22:10:18.4042396Z To only update this specific test, also pass `--test-args issues/issue-46843.rs`
2019-11-06T22:10:18.4042466Z error: 1 errors occurred comparing output.
2019-11-06T22:10:18.4042516Z status: exit code: 1
2019-11-06T22:10:18.4042516Z status: exit code: 1
2019-11-06T22:10:18.4043063Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46843.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46843" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46843/auxiliary" "-A" "unused"
2019-11-06T22:10:18.4043330Z ------------------------------------------
2019-11-06T22:10:18.4043359Z 
2019-11-06T22:10:18.4043557Z ------------------------------------------
2019-11-06T22:10:18.4043593Z stderr:
2019-11-06T22:10:18.4043593Z stderr:
2019-11-06T22:10:18.4043780Z ------------------------------------------
2019-11-06T22:10:18.4043959Z error[E0744]: match expression is not allowed in a const
2019-11-06T22:10:18.4044194Z   --> /checkout/src/test/ui/issues/issue-46843.rs:7:20
2019-11-06T22:10:18.4044234Z    |
2019-11-06T22:10:18.4044284Z LL |   pub const Q: i32 = match non_const() {
2019-11-06T22:10:18.4044319Z    |  ____________________^
2019-11-06T22:10:18.4044355Z LL | |     //~^ ERROR E0015
2019-11-06T22:10:18.4044391Z LL | |     //~^^ ERROR unimplemented expression type
2019-11-06T22:10:18.4044828Z LL | |     Thing::This => 1, //~ ERROR unimplemented expression type
2019-11-06T22:10:18.4044885Z LL | |     Thing::That => 0
2019-11-06T22:10:18.4044987Z    | |_^
2019-11-06T22:10:18.4045015Z 
2019-11-06T22:10:18.4045057Z error: aborting due to previous error
2019-11-06T22:10:18.4045086Z 
---
2019-11-06T22:10:18.4046229Z 
2019-11-06T22:10:18.4046292Z + error[E0744]: match expression is not allowed in a const
2019-11-06T22:10:18.4046518Z +   --> $DIR/issue-50577.rs:3:16
2019-11-06T22:10:18.4046564Z +    |
2019-11-06T22:10:18.4046610Z + LL |         Drop = assert_eq!(1, 1)
2019-11-06T22:10:18.4046716Z +    |
2019-11-06T22:10:18.4047046Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4047126Z + 
2019-11-06T22:10:18.4047175Z + error[E0744]: if expression is not allowed in a const
2019-11-06T22:10:18.4047175Z + error[E0744]: if expression is not allowed in a const
2019-11-06T22:10:18.4047400Z +   --> $DIR/issue-50577.rs:3:16
2019-11-06T22:10:18.4047472Z +    |
2019-11-06T22:10:18.4047517Z + LL |         Drop = assert_eq!(1, 1)
2019-11-06T22:10:18.4047630Z +    |
2019-11-06T22:10:18.4047959Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4048018Z + 
2019-11-06T22:10:18.4048247Z + error[E0744]: match expression is not allowed in a const
2019-11-06T22:10:18.4048247Z + error[E0744]: match expression is not allowed in a const
2019-11-06T22:10:18.4048595Z +   --> $DIR/issue-50577.rs:3:16
2019-11-06T22:10:18.4048628Z +    |
2019-11-06T22:10:18.4048660Z + LL |         Drop = assert_eq!(1, 1)
2019-11-06T22:10:18.4048740Z +    |
2019-11-06T22:10:18.4048979Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4049036Z + 
2019-11-06T22:10:18.4049074Z 1 error[E0317]: if may be missing an else clause
---
2019-11-06T22:10:18.4050389Z 
2019-11-06T22:10:18.4050409Z 
2019-11-06T22:10:18.4050464Z The actual stderr differed from the expected stderr.
2019-11-06T22:10:18.4050778Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50577/issue-50577.stderr
2019-11-06T22:10:18.4050967Z To update references, rerun the tests and pass the `--bless` flag
2019-11-06T22:10:18.4051181Z To only update this specific test, also pass `--test-args issues/issue-50577.rs`
2019-11-06T22:10:18.4051240Z error: 1 errors occurred comparing output.
2019-11-06T22:10:18.4051273Z status: exit code: 1
2019-11-06T22:10:18.4051273Z status: exit code: 1
2019-11-06T22:10:18.4051881Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50577.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50577" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50577/auxiliary" "-A" "unused"
2019-11-06T22:10:18.4052165Z ------------------------------------------
2019-11-06T22:10:18.4052190Z 
2019-11-06T22:10:18.4052360Z ------------------------------------------
2019-11-06T22:10:18.4052411Z stderr:
2019-11-06T22:10:18.4052411Z stderr:
2019-11-06T22:10:18.4052574Z ------------------------------------------
2019-11-06T22:10:18.4052613Z error[E0744]: match expression is not allowed in a const
2019-11-06T22:10:18.4052808Z   --> /checkout/src/test/ui/issues/issue-50577.rs:3:16
2019-11-06T22:10:18.4052845Z    |
2019-11-06T22:10:18.4052877Z LL |         Drop = assert_eq!(1, 1)
2019-11-06T22:10:18.4052998Z    |
2019-11-06T22:10:18.4053241Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4053289Z 
2019-11-06T22:10:18.4053330Z error[E0744]: if expression is not allowed in a const
2019-11-06T22:10:18.4053330Z error[E0744]: if expression is not allowed in a const
2019-11-06T22:10:18.4053519Z   --> /checkout/src/test/ui/issues/issue-50577.rs:3:16
2019-11-06T22:10:18.4053554Z    |
2019-11-06T22:10:18.4053602Z LL |         Drop = assert_eq!(1, 1)
2019-11-06T22:10:18.4053664Z    |
2019-11-06T22:10:18.4053917Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4053948Z 
2019-11-06T22:10:18.4053982Z error[E0744]: match expression is not allowed in a const
2019-11-06T22:10:18.4053982Z error[E0744]: match expression is not allowed in a const
2019-11-06T22:10:18.4054177Z   --> /checkout/src/test/ui/issues/issue-50577.rs:3:16
2019-11-06T22:10:18.4054212Z    |
2019-11-06T22:10:18.4054243Z LL |         Drop = assert_eq!(1, 1)
2019-11-06T22:10:18.4054321Z    |
2019-11-06T22:10:18.4055317Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-06T22:10:18.4055374Z 
2019-11-06T22:10:18.4055439Z error[E0317]: if may be missing an else clause
2019-11-06T22:10:18.4055439Z error[E0317]: if may be missing an else clause
2019-11-06T22:10:18.4055691Z   --> /checkout/src/test/ui/issues/issue-50577.rs:3:16
2019-11-06T22:10:18.4055739Z    |
2019-11-06T22:10:18.4055798Z LL |         Drop = assert_eq!(1, 1)
2019-11-06T22:10:18.4055889Z    |                |
2019-11-06T22:10:18.4055889Z    |                |
2019-11-06T22:10:18.4055935Z    |                expected (), found isize
2019-11-06T22:10:18.4055997Z    |                found here
2019-11-06T22:10:18.4056081Z    = note: expected type `()`
2019-11-06T22:10:18.4056142Z               found type `isize`
2019-11-06T22:10:18.4056142Z               found type `isize`
2019-11-06T22:10:18.4056190Z    = note: `if` expressions without `else` evaluate to `()`
2019-11-06T22:10:18.4056244Z    = help: consider adding an `else` block that evaluates to the expected type
2019-11-06T22:10:18.4056842Z 
2019-11-06T22:10:18.4056891Z error: aborting due to 4 previous errors
2019-11-06T22:10:18.4056922Z 
2019-11-06T22:10:18.4056986Z Some errors have detailed explanations: E0317, E0744.
---
2019-11-06T22:10:18.4060978Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-06T22:10:18.4061032Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-06T22:10:18.4061057Z 
2019-11-06T22:10:18.4061094Z 
2019-11-06T22:10:18.4062169Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-06T22:10:18.4062369Z 
2019-11-06T22:10:18.4062390Z 
2019-11-06T22:10:18.4062424Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-06T22:10:18.4062481Z Build completed unsuccessfully in 1:06:20
2019-11-06T22:10:18.4062481Z Build completed unsuccessfully in 1:06:20
2019-11-06T22:10:18.4090307Z == clock drift check ==
2019-11-06T22:10:18.4108836Z   local time: Wed Nov  6 22:10:18 UTC 2019
2019-11-06T22:10:18.6873594Z   network time: Wed, 06 Nov 2019 22:10:18 GMT
2019-11-06T22:10:18.6879485Z == end clock drift check ==
2019-11-06T22:10:19.8075727Z 
2019-11-06T22:10:19.8190440Z ##[error]Bash exited with code '1'.
2019-11-06T22:10:19.8218987Z ##[section]Starting: Checkout
2019-11-06T22:10:19.8220519Z ==============================================================================
2019-11-06T22:10:19.8220564Z Task         : Get sources
2019-11-06T22:10:19.8220616Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
