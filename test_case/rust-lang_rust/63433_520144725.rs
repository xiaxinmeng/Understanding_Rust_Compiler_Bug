plain
2019-08-10T11:10:29.9431752Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T11:10:29.9643337Z ##[command]git config gc.auto 0
2019-08-10T11:10:29.9723273Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T11:10:29.9780269Z ##[command]git config --get-all http.proxy
2019-08-10T11:10:29.9922284Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63433/merge:refs/remotes/pull/63433/merge
---
2019-08-10T11:11:04.0735337Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T11:11:04.0735374Z 
2019-08-10T11:11:04.0735778Z   git checkout -b <new-branch-name>
2019-08-10T11:11:04.0735801Z 
2019-08-10T11:11:04.0735860Z HEAD is now at 44df50651 Merge 03e95ae4127db1b0ae465e8b58383744b7184a70 into d19a359444295bab01de7ff44a9d72302e573bc9
2019-08-10T11:11:04.0898285Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T11:11:04.0900671Z ==============================================================================
2019-08-10T11:11:04.0900730Z Task         : Bash
2019-08-10T11:11:04.0900764Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T12:10:04.8188577Z .................................................................................................... 1300/8868
2019-08-10T12:10:11.3246700Z .................................................................................................... 1400/8868
2019-08-10T12:10:17.4691796Z .................................................................................................... 1500/8868
2019-08-10T12:10:27.9978821Z ....................................................F...............................i............... 1600/8868
2019-08-10T12:10:35.7013756Z i................................................................................................... 1700/8868
2019-08-10T12:10:42.3642714Z ...........................................................................iiiii.................... 1800/8868
2019-08-10T12:11:04.0671530Z .................................................................................................... 2000/8868
2019-08-10T12:11:06.4354232Z .................................................................................................... 2100/8868
2019-08-10T12:11:09.1051626Z .................................................................................................... 2200/8868
2019-08-10T12:11:16.7398158Z .................................................................................................... 2300/8868
---
2019-08-10T12:15:00.8526392Z .................................................................................................... 5200/8868
2019-08-10T12:15:11.4484587Z .................................................................................................... 5300/8868
2019-08-10T12:15:18.8727016Z ....i............................................................................................... 5400/8868
2019-08-10T12:15:24.2093415Z .................................................................................................... 5500/8868
2019-08-10T12:15:36.7177622Z ...................................................................................................i 5600/8868
2019-08-10T12:15:50.3727841Z i...i..ii...........i............................................................................... 5700/8868
2019-08-10T12:16:04.5434105Z .................................................................................................... 5900/8868
2019-08-10T12:16:09.2377467Z .................................................................................................... 6000/8868
2019-08-10T12:16:09.2377467Z .................................................................................................... 6000/8868
2019-08-10T12:16:23.3343243Z i..ii............................................................................................... 6100/8868
2019-08-10T12:16:41.6845539Z ...........................................i........................................................ 6300/8868
2019-08-10T12:16:43.7280845Z .................................................................................................... 6400/8868
2019-08-10T12:16:46.1399250Z ...............i.................................................................................... 6500/8868
2019-08-10T12:16:50.5223949Z .................................................................................................... 6600/8868
---
2019-08-10T12:20:51.3315285Z -    |     ^^^^^^^^^^^
2019-08-10T12:20:51.3315352Z + LL |     hint_unreachable()
2019-08-10T12:20:51.3315396Z +    |     ^^^^^^^^^^^^^^^^^^
2019-08-10T12:20:51.3315602Z 6    |     |
2019-08-10T12:20:51.3315860Z -    |     tried to call a function with return type T passing return place of type !
2019-08-10T12:20:51.3315913Z +    |     reached the configured maximum number of stack frames
2019-08-10T12:20:51.3316156Z 8    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-08-10T12:20:51.3316408Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3316638Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3316878Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3317128Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3317358Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3317590Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3317849Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3318084Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3318498Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3319075Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3319299Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3319549Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3319777Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3319999Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3320347Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3320613Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3320837Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3321080Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3321308Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3321722Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3322484Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3322767Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3323040Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3323344Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3323626Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3323899Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3324191Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3324463Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3324868Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3325163Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3325645Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3326039Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3326297Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3326529Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3326759Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3327008Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3327239Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3327478Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3327730Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3328112Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3328375Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3328631Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3328858Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3329103Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3329331Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3329553Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3329802Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3330031Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3330256Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3330586Z 9    |     inside call to `fake_type::<i32>` at $DIR/uninhabited-const-issue-61744.rs:12:36
2019-08-10T12:20:51.3330644Z 10 ...
2019-08-10T12:20:51.3330684Z 11 LL |     const CONSTANT: i32 = unsafe { fake_type() };
2019-08-10T12:20:51.3330751Z 
2019-08-10T12:20:51.3330790Z The actual stderr differed from the expected stderr.
2019-08-10T12:20:51.3331124Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/uninhabited-const-issue-61744.stderr
2019-08-10T12:20:51.3331124Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/uninhabited-const-issue-61744.stderr
2019-08-10T12:20:51.3331387Z To update references, rerun the tests and pass the `--bless` flag
2019-08-10T12:20:51.3332147Z To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`
2019-08-10T12:20:51.3332254Z error: 1 errors occurred comparing output.
2019-08-10T12:20:51.3332300Z status: exit code: 1
2019-08-10T12:20:51.3332300Z status: exit code: 1
2019-08-10T12:20:51.3333115Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary" "-A" "unused"
2019-08-10T12:20:51.3333599Z ------------------------------------------
2019-08-10T12:20:51.3333634Z 
2019-08-10T12:20:51.3333880Z ------------------------------------------
2019-08-10T12:20:51.3333926Z stderr:
2019-08-10T12:20:51.3333926Z stderr:
2019-08-10T12:20:51.3334147Z ------------------------------------------
2019-08-10T12:20:51.3334217Z error: any use of this value will cause an error
2019-08-10T12:20:51.3334483Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-08-10T12:20:51.3334535Z    |
2019-08-10T12:20:51.3334609Z LL |     hint_unreachable()
2019-08-10T12:20:51.3335145Z    |     ^^^^^^^^^^^^^^^^^^
2019-08-10T12:20:51.3335192Z    |     |
2019-08-10T12:20:51.3335259Z    |     reached the configured maximum number of stack frames
2019-08-10T12:20:51.3335736Z    |     inside call to `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-08-10T12:20:51.3335988Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3336266Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3336511Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3336754Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3337026Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3337270Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3337529Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3337778Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3338022Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3338290Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3338537Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3338779Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3339134Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3339409Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3339654Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3339917Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3340169Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3340413Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3340677Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3340927Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3341190Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3341613Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3345789Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3346341Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3346612Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3346875Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3347163Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3347425Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3347687Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3347972Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3348234Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3348505Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3348948Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3349357Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3349626Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3349878Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3350125Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3350385Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3350644Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3350890Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3351153Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3351672Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3352386Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3352723Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3353020Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3353337Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3353656Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3353955Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3354272Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-08-10T12:20:51.3354589Z    |     inside call to `fake_type::<i32>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:12:36
2019-08-10T12:20:51.3354643Z ...
2019-08-10T12:20:51.3354706Z LL |     const CONSTANT: i32 = unsafe { fake_type() };
2019-08-10T12:20:51.3355000Z    |
2019-08-10T12:20:51.3355062Z    = note: `#[deny(const_err)]` on by default
2019-08-10T12:20:51.3355096Z 
2019-08-10T12:20:51.3355244Z error[E0080]: erroneous constant used
2019-08-10T12:20:51.3355244Z error[E0080]: erroneous constant used
2019-08-10T12:20:51.3355689Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:18:10
2019-08-10T12:20:51.3355746Z    |
2019-08-10T12:20:51.3355943Z LL |     dbg!(i32::CONSTANT); //~ ERROR erroneous constant used
2019-08-10T12:20:51.3355987Z    |          ^^^^^^^^^^^^^ referenced constant has errors
2019-08-10T12:20:51.3356063Z error: aborting due to 2 previous errors
2019-08-10T12:20:51.3356087Z 
2019-08-10T12:20:51.3356304Z For more information about this error, try `rustc --explain E0080`.
2019-08-10T12:20:51.3356333Z 
---
2019-08-10T12:20:51.3362509Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-10T12:20:51.3362587Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-10T12:20:51.3362674Z 
2019-08-10T12:20:51.3362702Z 
2019-08-10T12:20:51.3364370Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-10T12:20:51.3364675Z 
2019-08-10T12:20:51.3364708Z 
2019-08-10T12:20:51.3372245Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-10T12:20:51.3372342Z Build completed unsuccessfully in 1:03:45
2019-08-10T12:20:51.3372342Z Build completed unsuccessfully in 1:03:45
2019-08-10T12:20:52.1224284Z ##[error]Bash exited with code '1'.
2019-08-10T12:20:52.1273086Z ##[section]Starting: Checkout
2019-08-10T12:20:52.1275299Z ==============================================================================
2019-08-10T12:20:52.1275371Z Task         : Get sources
2019-08-10T12:20:52.1275439Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
