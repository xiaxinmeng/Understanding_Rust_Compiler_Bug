plain
2019-09-24T04:40:39.4154809Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T04:40:39.4357075Z ##[command]git config gc.auto 0
2019-09-24T04:40:39.4515377Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T04:40:39.4577977Z ##[command]git config --get-all http.proxy
2019-09-24T04:40:39.4736111Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-24T05:46:10.8419409Z .................................................................................................... 1500/9040
2019-09-24T05:46:17.0592876Z .................................................................................................... 1600/9040
2019-09-24T05:46:29.7751143Z ...........................................................................i...............i........ 1700/9040
2019-09-24T05:46:36.8339090Z .................................................................................................... 1800/9040
2019-09-24T05:46:45.0842052Z ..................................................................iiiii............................. 1900/9040
2019-09-24T05:47:06.1235297Z .................................................................................................... 2100/9040
2019-09-24T05:47:08.7251462Z .................................................................................................... 2200/9040
2019-09-24T05:47:11.8849153Z .................................................................................................... 2300/9040
2019-09-24T05:47:20.6380841Z .................................................................................................... 2400/9040
---
2019-09-24T05:50:26.3907119Z .......................................................i...............i............................ 4700/9040
2019-09-24T05:50:35.9638657Z .................................................................................................... 4800/9040
2019-09-24T05:50:44.7816169Z .................................................................................................... 4900/9040
2019-09-24T05:50:52.5644309Z .................................................................................................... 5000/9040
2019-09-24T05:51:02.6640849Z ..........................................ii.ii..................................................... 5100/9040
2019-09-24T05:51:12.8942090Z .................................................................................................... 5300/9040
2019-09-24T05:51:23.5757948Z .................................................................................................... 5400/9040
2019-09-24T05:51:31.1087330Z .......i............................................................................................ 5500/9040
2019-09-24T05:51:36.7039021Z .................................................................................................... 5600/9040
2019-09-24T05:51:36.7039021Z .................................................................................................... 5600/9040
2019-09-24T05:51:48.9068899Z .................................................................................................... 5700/9040
2019-09-24T05:52:02.3881664Z ..ii...i..ii...........i............................................................................ 5800/9040
2019-09-24T05:52:24.3398139Z .................................................................................................... 6000/9040
2019-09-24T05:52:33.8996552Z .................................................................................................... 6100/9040
2019-09-24T05:52:33.8996552Z .................................................................................................... 6100/9040
2019-09-24T05:52:50.6366758Z ....i..ii........................................................................................... 6200/9040
2019-09-24T05:53:10.5418634Z ................................................................i................................... 6400/9040
2019-09-24T05:53:12.7515320Z .................................................................................................... 6500/9040
2019-09-24T05:53:15.3638800Z ....................................i............................................................... 6600/9040
2019-09-24T05:53:19.5491096Z .................................................................................................... 6700/9040
---
2019-09-24T05:57:32.8107667Z 1 warning: skipping const checks
2019-09-24T05:57:32.8108572Z -   --> $DIR/const_fn_ptr.rs:25:5
2019-09-24T05:57:32.8109015Z +   --> $DIR/const_fn_ptr.rs:12:5
2019-09-24T05:57:32.8109211Z 3    |
2019-09-24T05:57:32.8109593Z - LL |     assert_eq!(Y, 4);
2019-09-24T05:57:32.8110338Z -    |
2019-09-24T05:57:32.8110860Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8111071Z + LL |     X(x)
2019-09-24T05:57:32.8111246Z +    |     ^^^^
2019-09-24T05:57:32.8111246Z +    |     ^^^^
2019-09-24T05:57:32.8111393Z 8 
2019-09-24T05:57:32.8111543Z 9 warning: skipping const checks
2019-09-24T05:57:32.8111919Z -   --> $DIR/const_fn_ptr.rs:25:5
2019-09-24T05:57:32.8112329Z +   --> $DIR/const_fn_ptr.rs:16:5
2019-09-24T05:57:32.8112537Z 11    |
2019-09-24T05:57:32.8112913Z - LL |     assert_eq!(Y, 4);
2019-09-24T05:57:32.8113656Z -    |
2019-09-24T05:57:32.8114739Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8114739Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8115018Z + LL |     X_const(x)
2019-09-24T05:57:32.8115353Z 16 
2019-09-24T05:57:32.8115505Z 17 warning: skipping const checks
2019-09-24T05:57:32.8115908Z -   --> $DIR/const_fn_ptr.rs:25:5
2019-09-24T05:57:32.8116320Z +   --> $DIR/const_fn_ptr.rs:20:5
2019-09-24T05:57:32.8116320Z +   --> $DIR/const_fn_ptr.rs:20:5
2019-09-24T05:57:32.8116516Z 19    |
2019-09-24T05:57:32.8116899Z - LL |     assert_eq!(Y, 4);
2019-09-24T05:57:32.8117830Z -    |
2019-09-24T05:57:32.8118359Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8119059Z - 
2019-09-24T05:57:32.8119457Z - warning: skipping const checks
2019-09-24T05:57:32.8119457Z - warning: skipping const checks
2019-09-24T05:57:32.8119872Z -   --> $DIR/const_fn_ptr.rs:27:5
2019-09-24T05:57:32.8120234Z -    |
2019-09-24T05:57:32.8120617Z - LL |     assert_eq!(y, 4);
2019-09-24T05:57:32.8121384Z -    |
2019-09-24T05:57:32.8121876Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8122267Z - 
2019-09-24T05:57:32.8122650Z - warning: skipping const checks
2019-09-24T05:57:32.8122650Z - warning: skipping const checks
2019-09-24T05:57:32.8123059Z -   --> $DIR/const_fn_ptr.rs:27:5
2019-09-24T05:57:32.8123426Z -    |
2019-09-24T05:57:32.8123806Z - LL |     assert_eq!(y, 4);
2019-09-24T05:57:32.8124573Z -    |
2019-09-24T05:57:32.8125692Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8126204Z - 
2019-09-24T05:57:32.8126618Z - warning: skipping const checks
2019-09-24T05:57:32.8126618Z - warning: skipping const checks
2019-09-24T05:57:32.8127020Z -   --> $DIR/const_fn_ptr.rs:27:5
2019-09-24T05:57:32.8127414Z -    |
2019-09-24T05:57:32.8127817Z - LL |     assert_eq!(y, 4);
2019-09-24T05:57:32.8128750Z -    |
2019-09-24T05:57:32.8129239Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8129621Z - 
2019-09-24T05:57:32.8130005Z - warning: skipping const checks
2019-09-24T05:57:32.8130005Z - warning: skipping const checks
2019-09-24T05:57:32.8130394Z -   --> $DIR/const_fn_ptr.rs:29:5
2019-09-24T05:57:32.8130772Z -    |
2019-09-24T05:57:32.8131152Z - LL |     assert_eq!(y, 4);
2019-09-24T05:57:32.8131954Z -    |
2019-09-24T05:57:32.8132503Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8132930Z - 
2019-09-24T05:57:32.8133321Z - warning: skipping const checks
2019-09-24T05:57:32.8133321Z - warning: skipping const checks
2019-09-24T05:57:32.8133873Z -   --> $DIR/const_fn_ptr.rs:29:5
2019-09-24T05:57:32.8134246Z -    |
2019-09-24T05:57:32.8135022Z - LL |     assert_eq!(y, 4);
2019-09-24T05:57:32.8135911Z -    |
2019-09-24T05:57:32.8136433Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8136832Z - 
2019-09-24T05:57:32.8137251Z - warning: skipping const checks
2019-09-24T05:57:32.8137251Z - warning: skipping const checks
2019-09-24T05:57:32.8137670Z -   --> $DIR/const_fn_ptr.rs:29:5
2019-09-24T05:57:32.8138925Z -    |
2019-09-24T05:57:32.8140851Z - LL |     assert_eq!(y, 4);
2019-09-24T05:57:32.8143544Z -    |
2019-09-24T05:57:32.8144215Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8145278Z - 
2019-09-24T05:57:32.8145859Z - warning: skipping const checks
2019-09-24T05:57:32.8145859Z - warning: skipping const checks
2019-09-24T05:57:32.8146394Z -   --> $DIR/const_fn_ptr.rs:32:5
2019-09-24T05:57:32.8146884Z -    |
2019-09-24T05:57:32.8147417Z - LL |     assert_eq!(Z, 4);
2019-09-24T05:57:32.8148637Z -    |
2019-09-24T05:57:32.8149274Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8149761Z - 
2019-09-24T05:57:32.8150274Z - warning: skipping const checks
2019-09-24T05:57:32.8150274Z - warning: skipping const checks
2019-09-24T05:57:32.8150833Z -   --> $DIR/const_fn_ptr.rs:32:5
2019-09-24T05:57:32.8151336Z -    |
2019-09-24T05:57:32.8151855Z - LL |     assert_eq!(Z, 4);
2019-09-24T05:57:32.8153452Z -    |
2019-09-24T05:57:32.8156546Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8158865Z - 
2019-09-24T05:57:32.8160692Z - warning: skipping const checks
2019-09-24T05:57:32.8160692Z - warning: skipping const checks
2019-09-24T05:57:32.8162462Z -   --> $DIR/const_fn_ptr.rs:32:5
2019-09-24T05:57:32.8164406Z -    |
2019-09-24T05:57:32.8166336Z - LL |     assert_eq!(Z, 4);
2019-09-24T05:57:32.8169250Z -    |
2019-09-24T05:57:32.8169994Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8170513Z - 
2019-09-24T05:57:32.8171025Z - warning: skipping const checks
2019-09-24T05:57:32.8171025Z - warning: skipping const checks
2019-09-24T05:57:32.8171572Z -   --> $DIR/const_fn_ptr.rs:34:5
2019-09-24T05:57:32.8172069Z -    |
2019-09-24T05:57:32.8172579Z - LL |     assert_eq!(z, 4);
2019-09-24T05:57:32.8173950Z -    |
2019-09-24T05:57:32.8175090Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8175884Z - 
2019-09-24T05:57:32.8176424Z - warning: skipping const checks
2019-09-24T05:57:32.8176424Z - warning: skipping const checks
2019-09-24T05:57:32.8177077Z -   --> $DIR/const_fn_ptr.rs:34:5
2019-09-24T05:57:32.8177613Z -    |
2019-09-24T05:57:32.8178127Z - LL |     assert_eq!(z, 4);
2019-09-24T05:57:32.8180079Z -    |
2019-09-24T05:57:32.8180634Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8181038Z - 
2019-09-24T05:57:32.8181430Z - warning: skipping const checks
2019-09-24T05:57:32.8181430Z - warning: skipping const checks
2019-09-24T05:57:32.8181819Z -   --> $DIR/const_fn_ptr.rs:34:5
2019-09-24T05:57:32.8182206Z -    |
2019-09-24T05:57:32.8182589Z - LL |     assert_eq!(z, 4);
2019-09-24T05:57:32.8183370Z -    |
2019-09-24T05:57:32.8183882Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8184681Z - 
2019-09-24T05:57:32.8185165Z - warning: skipping const checks
2019-09-24T05:57:32.8185165Z - warning: skipping const checks
2019-09-24T05:57:32.8185581Z -   --> $DIR/const_fn_ptr.rs:36:5
2019-09-24T05:57:32.8193523Z -    |
2019-09-24T05:57:32.8194139Z - LL |     assert_eq!(z, 4);
2019-09-24T05:57:32.8195443Z -    |
2019-09-24T05:57:32.8195976Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8196364Z - 
2019-09-24T05:57:32.8196787Z - warning: skipping const checks
2019-09-24T05:57:32.8196787Z - warning: skipping const checks
2019-09-24T05:57:32.8197227Z -   --> $DIR/const_fn_ptr.rs:36:5
2019-09-24T05:57:32.8197969Z -    |
2019-09-24T05:57:32.8198904Z - LL |     assert_eq!(z, 4);
2019-09-24T05:57:32.8200088Z -    |
2019-09-24T05:57:32.8200619Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8200999Z - 
2019-09-24T05:57:32.8201410Z - warning: skipping const checks
2019-09-24T05:57:32.8201410Z - warning: skipping const checks
2019-09-24T05:57:32.8201808Z -   --> $DIR/const_fn_ptr.rs:36:5
2019-09-24T05:57:32.8202168Z -    |
2019-09-24T05:57:32.8202572Z - LL |     assert_eq!(z, 4);
2019-09-24T05:57:32.8203316Z -    |
2019-09-24T05:57:32.8203824Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8204028Z + LL |     x(y)
2019-09-24T05:57:32.8204714Z +    |     ^^^^
2019-09-24T05:57:32.8204714Z +    |     ^^^^
2019-09-24T05:57:32.8204923Z 144 
2019-09-24T05:57:32.8205080Z 145 warning: constant `X_const` should have an upper case name
2019-09-24T05:57:32.8205963Z 
2019-09-24T05:57:32.8206101Z 
2019-09-24T05:57:32.8206260Z The actual stderr differed from the expected stderr.
2019-09-24T05:57:32.8206777Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr/const_fn_ptr.stderr
2019-09-24T05:57:32.8206777Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr/const_fn_ptr.stderr
2019-09-24T05:57:32.8207238Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T05:57:32.8207762Z To only update this specific test, also pass `--test-args consts/const-eval/const_fn_ptr.rs`
2019-09-24T05:57:32.8208270Z error: 1 errors occurred comparing output.
2019-09-24T05:57:32.8208417Z status: exit code: 0
2019-09-24T05:57:32.8208417Z status: exit code: 0
2019-09-24T05:57:32.8209466Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr/auxiliary"
2019-09-24T05:57:32.8210176Z ------------------------------------------
2019-09-24T05:57:32.8210355Z 
2019-09-24T05:57:32.8210754Z ------------------------------------------
2019-09-24T05:57:32.8210995Z stderr:
---
2019-09-24T05:57:32.8212674Z 
2019-09-24T05:57:32.8212821Z warning: skipping const checks
2019-09-24T05:57:32.8213233Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:16:5
2019-09-24T05:57:32.8213492Z    |
2019-09-24T05:57:32.8213646Z LL |     X_const(x)
2019-09-24T05:57:32.8213916Z 
2019-09-24T05:57:32.8214488Z warning: skipping const checks
2019-09-24T05:57:32.8215013Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:20:5
2019-09-24T05:57:32.8215228Z    |
2019-09-24T05:57:32.8215228Z    |
2019-09-24T05:57:32.8215384Z LL |     x(y)
2019-09-24T05:57:32.8215532Z    |     ^^^^
2019-09-24T05:57:32.8215662Z 
2019-09-24T05:57:32.8215859Z warning: constant `X_const` should have an upper case name
2019-09-24T05:57:32.8216478Z    |
2019-09-24T05:57:32.8216478Z    |
2019-09-24T05:57:32.8216883Z LL | const X_const: fn(usize) -> usize = double_const;
2019-09-24T05:57:32.8217116Z    |       ^^^^^^^ help: convert the identifier to upper case: `X_CONST`
2019-09-24T05:57:32.8217462Z    = note: `#[warn(non_upper_case_globals)]` on by default
2019-09-24T05:57:32.8217611Z 
2019-09-24T05:57:32.8217899Z 
2019-09-24T05:57:32.8218270Z ------------------------------------------
2019-09-24T05:57:32.8218270Z ------------------------------------------
2019-09-24T05:57:32.8218465Z 
2019-09-24T05:57:32.8218597Z 
2019-09-24T05:57:32.8219007Z ---- [ui] ui/consts/const-eval/const_fn_ptr_fail.rs stdout ----
2019-09-24T05:57:32.8219208Z normalized stderr:
2019-09-24T05:57:32.8219358Z warning: skipping const checks
2019-09-24T05:57:32.8219875Z   --> $DIR/const_fn_ptr_fail.rs:10:5
2019-09-24T05:57:32.8220120Z    |
2019-09-24T05:57:32.8220275Z LL |     X(x) // FIXME: this should error someday
2019-09-24T05:57:32.8220568Z 
2019-09-24T05:57:32.8220694Z 
2019-09-24T05:57:32.8220821Z 
2019-09-24T05:57:32.8220947Z 
2019-09-24T05:57:32.8220947Z 
2019-09-24T05:57:32.8221097Z The actual stderr differed from the expected stderr.
2019-09-24T05:57:32.8221584Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail/const_fn_ptr_fail.stderr
2019-09-24T05:57:32.8222055Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T05:57:32.8222848Z To only update this specific test, also pass `--test-args consts/const-eval/const_fn_ptr_fail.rs`
2019-09-24T05:57:32.8223709Z error: 1 errors occurred comparing output.
2019-09-24T05:57:32.8223999Z status: exit code: 0
2019-09-24T05:57:32.8223999Z status: exit code: 0
2019-09-24T05:57:32.8225413Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail/auxiliary"
2019-09-24T05:57:32.8227806Z ------------------------------------------
2019-09-24T05:57:32.8227878Z 
2019-09-24T05:57:32.8228327Z ------------------------------------------
2019-09-24T05:57:32.8228373Z stderr:
2019-09-24T05:57:32.8228373Z stderr:
2019-09-24T05:57:32.8228594Z ------------------------------------------
2019-09-24T05:57:32.8228641Z warning: skipping const checks
2019-09-24T05:57:32.8228881Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail.rs:10:5
2019-09-24T05:57:32.8228944Z    |
2019-09-24T05:57:32.8228989Z LL |     X(x) // FIXME: this should error someday
2019-09-24T05:57:32.8229061Z 
2019-09-24T05:57:32.8229086Z 
2019-09-24T05:57:32.8229310Z ------------------------------------------
2019-09-24T05:57:32.8229341Z 
---
2019-09-24T05:57:32.8229737Z 1 warning: skipping const checks
2019-09-24T05:57:32.8229973Z -   --> $DIR/const_fn_ptr_fail2.rs:16:5
2019-09-24T05:57:32.8230180Z +   --> $DIR/const_fn_ptr_fail2.rs:9:5
2019-09-24T05:57:32.8230224Z 3    |
2019-09-24T05:57:32.8230417Z - LL |     assert_eq!(Y, 4);
2019-09-24T05:57:32.8230802Z -    |
2019-09-24T05:57:32.8231107Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8231306Z - 
2019-09-24T05:57:32.8231502Z - warning: skipping const checks
2019-09-24T05:57:32.8231502Z - warning: skipping const checks
2019-09-24T05:57:32.8231708Z -   --> $DIR/const_fn_ptr_fail2.rs:16:5
2019-09-24T05:57:32.8231900Z -    |
2019-09-24T05:57:32.8232151Z - LL |     assert_eq!(Y, 4);
2019-09-24T05:57:32.8232534Z -    |
2019-09-24T05:57:32.8232846Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8233038Z - 
2019-09-24T05:57:32.8233232Z - warning: skipping const checks
2019-09-24T05:57:32.8233232Z - warning: skipping const checks
2019-09-24T05:57:32.8233457Z -   --> $DIR/const_fn_ptr_fail2.rs:16:5
2019-09-24T05:57:32.8233631Z -    |
2019-09-24T05:57:32.8233822Z - LL |     assert_eq!(Y, 4);
2019-09-24T05:57:32.8234644Z -    |
2019-09-24T05:57:32.8234974Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8235181Z - 
2019-09-24T05:57:32.8235386Z - warning: skipping const checks
2019-09-24T05:57:32.8235386Z - warning: skipping const checks
2019-09-24T05:57:32.8235599Z -   --> $DIR/const_fn_ptr_fail2.rs:21:5
2019-09-24T05:57:32.8235795Z -    |
2019-09-24T05:57:32.8235996Z - LL |     assert_eq!(Z, 4);
2019-09-24T05:57:32.8236372Z -    |
2019-09-24T05:57:32.8236714Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8237106Z - 
2019-09-24T05:57:32.8237310Z - warning: skipping const checks
2019-09-24T05:57:32.8237310Z - warning: skipping const checks
2019-09-24T05:57:32.8237544Z -   --> $DIR/const_fn_ptr_fail2.rs:21:5
2019-09-24T05:57:32.8237724Z -    |
2019-09-24T05:57:32.8238075Z - LL |     assert_eq!(Z, 4);
2019-09-24T05:57:32.8238459Z -    |
2019-09-24T05:57:32.8238916Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8239104Z - 
2019-09-24T05:57:32.8239293Z - warning: skipping const checks
2019-09-24T05:57:32.8239293Z - warning: skipping const checks
2019-09-24T05:57:32.8239489Z -   --> $DIR/const_fn_ptr_fail2.rs:21:5
2019-09-24T05:57:32.8239656Z -    |
2019-09-24T05:57:32.8239861Z - LL |     assert_eq!(Z, 4);
2019-09-24T05:57:32.8240213Z -    |
2019-09-24T05:57:32.8242690Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8243051Z + LL |     x(y)
2019-09-24T05:57:32.8243246Z +    |     ^^^^
2019-09-24T05:57:32.8243246Z +    |     ^^^^
2019-09-24T05:57:32.8243393Z 48 
2019-09-24T05:57:32.8243537Z 49 error[E0080]: evaluation of constant expression failed
2019-09-24T05:57:32.8243981Z 50   --> $DIR/const_fn_ptr_fail2.rs:16:5
2019-09-24T05:57:32.8244795Z 
2019-09-24T05:57:32.8244975Z 
2019-09-24T05:57:32.8245163Z The actual stderr differed from the expected stderr.
2019-09-24T05:57:32.8245702Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2/const_fn_ptr_fail2.stderr
2019-09-24T05:57:32.8246161Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T05:57:32.8246720Z To only update this specific test, also pass `--test-args consts/const-eval/const_fn_ptr_fail2.rs`
2019-09-24T05:57:32.8247083Z error: 1 errors occurred comparing output.
2019-09-24T05:57:32.8247282Z status: exit code: 1
2019-09-24T05:57:32.8247282Z status: exit code: 1
2019-09-24T05:57:32.8248559Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2/auxiliary" "-A" "unused"
2019-09-24T05:57:32.8249772Z ------------------------------------------
2019-09-24T05:57:32.8249960Z 
2019-09-24T05:57:32.8250319Z ------------------------------------------
2019-09-24T05:57:32.8250501Z stderr:
---
2019-09-24T05:57:32.8254889Z 
2019-09-24T05:57:32.8255100Z error[E0080]: evaluation of constant expression failed
2019-09-24T05:57:32.8255745Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:21:5
2019-09-24T05:57:32.8255958Z    |
2019-09-24T05:57:32.8256134Z LL |     assert_eq!(Z, 4);
2019-09-24T05:57:32.8256495Z    |     ^^^^^^^^^^^-^^^^^
2019-09-24T05:57:32.8256870Z    |                referenced constant has errors
2019-09-24T05:57:32.8257017Z    |
2019-09-24T05:57:32.8257504Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-24T05:57:32.8258137Z 
---
2019-09-24T05:57:32.8263338Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-24T05:57:32.8263565Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-24T05:57:32.8263725Z 
2019-09-24T05:57:32.8263850Z 
2019-09-24T05:57:32.8266121Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-24T05:57:32.8266752Z 
2019-09-24T05:57:32.8266890Z 
2019-09-24T05:57:32.8267072Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-24T05:57:32.8267238Z Build completed unsuccessfully in 1:09:36
2019-09-24T05:57:32.8267238Z Build completed unsuccessfully in 1:09:36
2019-09-24T05:57:32.8275690Z == clock drift check ==
2019-09-24T05:57:32.8292717Z   local time: Tue Sep 24 05:57:32 UTC 2019
2019-09-24T05:57:32.9792529Z   network time: Tue, 24 Sep 2019 05:57:32 GMT
2019-09-24T05:57:32.9792641Z == end clock drift check ==
2019-09-24T05:57:33.7162513Z ##[error]Bash exited with code '1'.
2019-09-24T05:57:33.7201597Z ##[section]Starting: Checkout
2019-09-24T05:57:33.7204016Z ==============================================================================
2019-09-24T05:57:33.7204072Z Task         : Get sources
2019-09-24T05:57:33.7204120Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
