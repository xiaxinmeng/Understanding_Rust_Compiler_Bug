plain
2019-08-28T21:20:24.8660424Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-28T21:20:24.8844292Z ##[command]git config gc.auto 0
2019-08-28T21:20:24.8922437Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-28T21:20:24.8981805Z ##[command]git config --get-all http.proxy
2019-08-28T21:20:24.9117316Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63982/merge:refs/remotes/pull/63982/merge
---
2019-08-28T22:23:03.8780777Z .................................................................................................... 1500/8972
2019-08-28T22:23:09.4648469Z .................................................................................................... 1600/8972
2019-08-28T22:23:22.0830563Z ................................................i...............i................................... 1700/8972
2019-08-28T22:23:30.2566042Z .................................................................................................... 1800/8972
2019-08-28T22:23:44.5384747Z .......................................iiiii........................................................ 1900/8972
2019-08-28T22:23:55.2460855Z .................................................................................................... 2100/8972
2019-08-28T22:23:57.8709630Z .................................................................................................... 2200/8972
2019-08-28T22:24:01.9048053Z .................................F.............................F....................F............... 2300/8972
2019-08-28T22:24:09.3780358Z .................................................................................................... 2400/8972
---
2019-08-28T22:27:06.2529285Z ..........................i...............i......................................................... 4700/8972
2019-08-28T22:27:17.9607367Z ...................................................................F................................ 4800/8972
2019-08-28T22:27:24.0819581Z .................................................................................................... 4900/8972
2019-08-28T22:27:34.6760893Z .................................................................................................... 5000/8972
2019-08-28T22:27:40.2393576Z .......ii.ii........................................................................................ 5100/8972
2019-08-28T22:27:53.3014981Z .................................................................................................... 5300/8972
2019-08-28T22:28:01.5563545Z ......................................................................i............................. 5400/8972
2019-08-28T22:28:08.7509932Z .................................................................................................... 5500/8972
2019-08-28T22:28:15.5450096Z .................................................................................................... 5600/8972
2019-08-28T22:28:15.5450096Z .................................................................................................... 5600/8972
2019-08-28T22:28:25.9441437Z ................................................................ii...i..ii...........i.............. 5700/8972
2019-08-28T22:28:51.1976661Z .................................................................................................... 5900/8972
2019-08-28T22:29:00.5851669Z .................................................................................................... 6000/8972
2019-08-28T22:29:00.5851669Z .................................................................................................... 6000/8972
2019-08-28T22:29:08.5529387Z .................................................................i..ii.............................. 6100/8972
2019-08-28T22:29:37.2916209Z ...............................................................................F.................... 6300/8972
2019-08-28T22:29:39.3979603Z ....................i............................................................................... 6400/8972
2019-08-28T22:29:41.5151097Z ............................................................................................i....... 6500/8972
2019-08-28T22:29:44.2462239Z .................................................................................................... 6600/8972
---
2019-08-28T22:33:41.3967003Z 
2019-08-28T22:33:41.3967707Z ---- [ui] ui/error-codes/E0616.rs stdout ----
2019-08-28T22:33:41.3967933Z diff of stderr:
2019-08-28T22:33:41.3968101Z 
2019-08-28T22:33:41.3968556Z - error[E0616]: field `x` of struct `a::Foo` is private
2019-08-28T22:33:41.3968767Z + error[E0616]: field `x` of `struct` `a::Foo` is private
2019-08-28T22:33:41.3969483Z 2   --> $DIR/E0616.rs:13:5
2019-08-28T22:33:41.3969896Z 4 LL |     f.x;
2019-08-28T22:33:41.3970017Z 
2019-08-28T22:33:41.3970155Z 
2019-08-28T22:33:41.3970302Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.3970302Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.3970780Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0616/E0616.stderr
2019-08-28T22:33:41.3975244Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.3975553Z To only update this specific test, also pass `--test-args error-codes/E0616.rs`
2019-08-28T22:33:41.3975637Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.3975682Z status: exit code: 1
2019-08-28T22:33:41.3975682Z status: exit code: 1
2019-08-28T22:33:41.3976474Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0616.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0616" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0616/auxiliary" "-A" "unused"
2019-08-28T22:33:41.3976813Z ------------------------------------------
2019-08-28T22:33:41.3976848Z 
2019-08-28T22:33:41.3977065Z ------------------------------------------
2019-08-28T22:33:41.3977126Z stderr:
2019-08-28T22:33:41.3977126Z stderr:
2019-08-28T22:33:41.3977340Z ------------------------------------------
2019-08-28T22:33:41.3977392Z error[E0616]: field `x` of `struct` `a::Foo` is private
2019-08-28T22:33:41.3977850Z    |
2019-08-28T22:33:41.3978056Z LL |     f.x; //~ ERROR E0616
2019-08-28T22:33:41.3978116Z    |     ^^^
2019-08-28T22:33:41.3978161Z 
---
2019-08-28T22:33:41.3978882Z 
2019-08-28T22:33:41.3979131Z ---- [ui] ui/error-codes/ex-E0611.rs stdout ----
2019-08-28T22:33:41.3979182Z diff of stderr:
2019-08-28T22:33:41.3979209Z 
2019-08-28T22:33:41.3979443Z - error[E0616]: field `0` of struct `a::Foo` is private
2019-08-28T22:33:41.3979509Z + error[E0616]: field `0` of `struct` `a::Foo` is private
2019-08-28T22:33:41.3979721Z 2   --> $DIR/ex-E0611.rs:11:4
2019-08-28T22:33:41.3979821Z 4 LL |    y.0;
2019-08-28T22:33:41.3979850Z 
2019-08-28T22:33:41.3979876Z 
2019-08-28T22:33:41.3979929Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.3979929Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.3980250Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/ex-E0611/ex-E0611.stderr
2019-08-28T22:33:41.3980507Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.3982279Z To only update this specific test, also pass `--test-args error-codes/ex-E0611.rs`
2019-08-28T22:33:41.3982403Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.3982463Z status: exit code: 1
2019-08-28T22:33:41.3982463Z status: exit code: 1
2019-08-28T22:33:41.3983259Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/ex-E0611.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/ex-E0611" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/ex-E0611/auxiliary" "-A" "unused"
2019-08-28T22:33:41.3983593Z ------------------------------------------
2019-08-28T22:33:41.3983629Z 
2019-08-28T22:33:41.3983852Z ------------------------------------------
2019-08-28T22:33:41.3983897Z stderr:
2019-08-28T22:33:41.3983897Z stderr:
2019-08-28T22:33:41.3984124Z ------------------------------------------
2019-08-28T22:33:41.3984175Z error[E0616]: field `0` of `struct` `a::Foo` is private
2019-08-28T22:33:41.3984712Z    |
2019-08-28T22:33:41.3984712Z    |
2019-08-28T22:33:41.3984761Z LL |    y.0; //~ ERROR field `0` of struct `a::Foo` is private
2019-08-28T22:33:41.3984836Z 
2019-08-28T22:33:41.3984896Z error: aborting due to previous error
2019-08-28T22:33:41.3984926Z 
2019-08-28T22:33:41.3985175Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.3985175Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.3985208Z 
2019-08-28T22:33:41.3985448Z ------------------------------------------
2019-08-28T22:33:41.3985480Z 
2019-08-28T22:33:41.3985506Z 
2019-08-28T22:33:41.3985730Z ---- [ui] ui/explore-issue-38412.rs stdout ----
2019-08-28T22:33:41.3985792Z diff of stderr:
2019-08-28T22:33:41.3985819Z 
2019-08-28T22:33:41.3986209Z 16    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.3986287Z 17    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.3986346Z 18 
2019-08-28T22:33:41.3986623Z - error[E0616]: field `b_crate` of struct `pub_and_stability::Record` is private
2019-08-28T22:33:41.3986695Z + error[E0616]: field `b_crate` of `struct` `pub_and_stability::Record` is private
2019-08-28T22:33:41.3986922Z 20   --> $DIR/explore-issue-38412.rs:31:5
2019-08-28T22:33:41.3986967Z 21    |
2019-08-28T22:33:41.3987022Z 22 LL |     r.b_crate;
2019-08-28T22:33:41.3987092Z 23    |     ^^^^^^^^^
2019-08-28T22:33:41.3987133Z 24 
2019-08-28T22:33:41.3987133Z 24 
2019-08-28T22:33:41.3987499Z - error[E0616]: field `c_mod` of struct `pub_and_stability::Record` is private
2019-08-28T22:33:41.3987568Z + error[E0616]: field `c_mod` of `struct` `pub_and_stability::Record` is private
2019-08-28T22:33:41.3987830Z 26   --> $DIR/explore-issue-38412.rs:32:5
2019-08-28T22:33:41.3987933Z 28 LL |     r.c_mod;
2019-08-28T22:33:41.3987962Z 
2019-08-28T22:33:41.3988003Z 29    |     ^^^^^^^
2019-08-28T22:33:41.3988068Z 30 
2019-08-28T22:33:41.3988068Z 30 
2019-08-28T22:33:41.3988328Z - error[E0616]: field `d_priv` of struct `pub_and_stability::Record` is private
2019-08-28T22:33:41.3988385Z + error[E0616]: field `d_priv` of `struct` `pub_and_stability::Record` is private
2019-08-28T22:33:41.3988622Z 32   --> $DIR/explore-issue-38412.rs:33:5
2019-08-28T22:33:41.3988708Z 34 LL |     r.d_priv;
2019-08-28T22:33:41.3988736Z 
2019-08-28T22:33:41.3988736Z 
2019-08-28T22:33:41.3989330Z 43    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.3989411Z 44    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.3989458Z 45 
2019-08-28T22:33:41.3989794Z - error[E0616]: field `3` of struct `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.3989852Z + error[E0616]: field `3` of `struct` `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.3990076Z 47   --> $DIR/explore-issue-38412.rs:38:5
2019-08-28T22:33:41.3990178Z 49 LL |     t.3;
2019-08-28T22:33:41.3990216Z 
2019-08-28T22:33:41.3990256Z 50    |     ^^^
2019-08-28T22:33:41.3990312Z 51 
2019-08-28T22:33:41.3990312Z 51 
2019-08-28T22:33:41.3990562Z - error[E0616]: field `4` of struct `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.3990616Z + error[E0616]: field `4` of `struct` `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.3990852Z 53   --> $DIR/explore-issue-38412.rs:39:5
2019-08-28T22:33:41.3990938Z 55 LL |     t.4;
2019-08-28T22:33:41.3990979Z 
2019-08-28T22:33:41.3991020Z 56    |     ^^^
2019-08-28T22:33:41.3991060Z 57 
2019-08-28T22:33:41.3991060Z 57 
2019-08-28T22:33:41.3991315Z - error[E0616]: field `5` of struct `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.3991386Z + error[E0616]: field `5` of `struct` `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.3991613Z 59   --> $DIR/explore-issue-38412.rs:40:5
2019-08-28T22:33:41.3991713Z 61 LL |     t.5;
2019-08-28T22:33:41.3991741Z 
2019-08-28T22:33:41.3991766Z 
2019-08-28T22:33:41.3991810Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.3991810Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.3992304Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explore-issue-38412/explore-issue-38412.stderr
2019-08-28T22:33:41.3992562Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.3992826Z To only update this specific test, also pass `--test-args explore-issue-38412.rs`
2019-08-28T22:33:41.3992921Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.3992965Z status: exit code: 1
2019-08-28T22:33:41.3992965Z status: exit code: 1
2019-08-28T22:33:41.3993734Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/explore-issue-38412.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explore-issue-38412" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explore-issue-38412/auxiliary" "-A" "unused"
2019-08-28T22:33:41.3994069Z ------------------------------------------
2019-08-28T22:33:41.3994103Z 
2019-08-28T22:33:41.3995473Z ------------------------------------------
2019-08-28T22:33:41.3995541Z stderr:
2019-08-28T22:33:41.3995541Z stderr:
2019-08-28T22:33:41.3996210Z ------------------------------------------
2019-08-28T22:33:41.3996473Z error[E0658]: use of unstable library feature 'unstable_undeclared'
2019-08-28T22:33:41.3996860Z   --> /checkout/src/test/ui/explore-issue-38412.rs:21:63
2019-08-28T22:33:41.3996948Z    |
2019-08-28T22:33:41.3997000Z LL |     let Record { a_stable_pub: _, a_unstable_declared_pub: _, a_unstable_undeclared_pub: _, .. } =
2019-08-28T22:33:41.3997126Z    |
2019-08-28T22:33:41.3997126Z    |
2019-08-28T22:33:41.3997494Z    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.3997581Z    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.3997878Z error[E0658]: use of unstable library feature 'unstable_undeclared'
2019-08-28T22:33:41.3998116Z   --> /checkout/src/test/ui/explore-issue-38412.rs:30:5
2019-08-28T22:33:41.3998178Z    |
2019-08-28T22:33:41.3998178Z    |
2019-08-28T22:33:41.3998227Z LL |     r.a_unstable_undeclared_pub; //~ ERROR use of unstable library feature
2019-08-28T22:33:41.3998343Z    |
2019-08-28T22:33:41.3998343Z    |
2019-08-28T22:33:41.3998632Z    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.3998696Z    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.3998745Z 
2019-08-28T22:33:41.3998796Z error[E0616]: field `b_crate` of `struct` `pub_and_stability::Record` is private
2019-08-28T22:33:41.4006273Z   --> /checkout/src/test/ui/explore-issue-38412.rs:31:5
2019-08-28T22:33:41.4006431Z    |
2019-08-28T22:33:41.4006482Z LL |     r.b_crate;                   //~ ERROR is private
2019-08-28T22:33:41.4006559Z 
2019-08-28T22:33:41.4006559Z 
2019-08-28T22:33:41.4006622Z error[E0616]: field `c_mod` of `struct` `pub_and_stability::Record` is private
2019-08-28T22:33:41.4007025Z   --> /checkout/src/test/ui/explore-issue-38412.rs:32:5
2019-08-28T22:33:41.4007074Z    |
2019-08-28T22:33:41.4007135Z LL |     r.c_mod;                     //~ ERROR is private
2019-08-28T22:33:41.4007210Z 
2019-08-28T22:33:41.4007210Z 
2019-08-28T22:33:41.4007267Z error[E0616]: field `d_priv` of `struct` `pub_and_stability::Record` is private
2019-08-28T22:33:41.4007537Z   --> /checkout/src/test/ui/explore-issue-38412.rs:33:5
2019-08-28T22:33:41.4007584Z    |
2019-08-28T22:33:41.4007630Z LL |     r.d_priv;                    //~ ERROR is private
2019-08-28T22:33:41.4007717Z 
2019-08-28T22:33:41.4007964Z error[E0658]: use of unstable library feature 'unstable_undeclared'
2019-08-28T22:33:41.4008392Z   --> /checkout/src/test/ui/explore-issue-38412.rs:37:5
2019-08-28T22:33:41.4008456Z    |
2019-08-28T22:33:41.4008456Z    |
2019-08-28T22:33:41.4008505Z LL |     t.2;                         //~ ERROR use of unstable library feature
2019-08-28T22:33:41.4008612Z    |
2019-08-28T22:33:41.4008612Z    |
2019-08-28T22:33:41.4008935Z    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.4008994Z    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.4009045Z 
2019-08-28T22:33:41.4009447Z error[E0616]: field `3` of `struct` `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.4009775Z   --> /checkout/src/test/ui/explore-issue-38412.rs:38:5
2019-08-28T22:33:41.4009887Z LL |     t.3;                         //~ ERROR is private
2019-08-28T22:33:41.4009932Z    |     ^^^
2019-08-28T22:33:41.4009995Z 
2019-08-28T22:33:41.4009995Z 
2019-08-28T22:33:41.4010057Z error[E0616]: field `4` of `struct` `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.4010321Z   --> /checkout/src/test/ui/explore-issue-38412.rs:39:5
2019-08-28T22:33:41.4010426Z LL |     t.4;                         //~ ERROR is private
2019-08-28T22:33:41.4010472Z    |     ^^^
2019-08-28T22:33:41.4010499Z 
2019-08-28T22:33:41.4010499Z 
2019-08-28T22:33:41.4010545Z error[E0616]: field `5` of `struct` `pub_and_stability::Tuple` is private
2019-08-28T22:33:41.4010798Z   --> /checkout/src/test/ui/explore-issue-38412.rs:40:5
2019-08-28T22:33:41.4011017Z LL |     t.5;                         //~ ERROR is private
2019-08-28T22:33:41.4011092Z    |     ^^^
2019-08-28T22:33:41.4011120Z 
2019-08-28T22:33:41.4011582Z error[E0658]: use of unstable library feature 'unstable_undeclared'
2019-08-28T22:33:41.4011582Z error[E0658]: use of unstable library feature 'unstable_undeclared'
2019-08-28T22:33:41.4011874Z   --> /checkout/src/test/ui/explore-issue-38412.rs:44:7
2019-08-28T22:33:41.4011937Z    |
2019-08-28T22:33:41.4011987Z LL |     r.unstable_undeclared_trait_method(); //~ ERROR use of unstable library feature
2019-08-28T22:33:41.4012106Z    |
2019-08-28T22:33:41.4012106Z    |
2019-08-28T22:33:41.4012417Z    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.4012482Z    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.4012825Z error[E0658]: use of unstable library feature 'unstable_undeclared'
2019-08-28T22:33:41.4013086Z   --> /checkout/src/test/ui/explore-issue-38412.rs:48:7
2019-08-28T22:33:41.4013150Z    |
2019-08-28T22:33:41.4013150Z    |
2019-08-28T22:33:41.4013214Z LL |     r.unstable_undeclared();              //~ ERROR use of unstable library feature
2019-08-28T22:33:41.4013327Z    |
2019-08-28T22:33:41.4013327Z    |
2019-08-28T22:33:41.4013641Z    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.4013704Z    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.4013800Z error[E0624]: method `pub_crate` is private
2019-08-28T22:33:41.4014081Z   --> /checkout/src/test/ui/explore-issue-38412.rs:50:7
2019-08-28T22:33:41.4014132Z    |
2019-08-28T22:33:41.4014132Z    |
2019-08-28T22:33:41.4014199Z LL |     r.pub_crate();                        //~ ERROR `pub_crate` is private
2019-08-28T22:33:41.4014281Z 
2019-08-28T22:33:41.4014281Z 
2019-08-28T22:33:41.4014342Z error[E0624]: method `pub_mod` is private
2019-08-28T22:33:41.4014602Z   --> /checkout/src/test/ui/explore-issue-38412.rs:51:7
2019-08-28T22:33:41.4014651Z    |
2019-08-28T22:33:41.4014711Z LL |     r.pub_mod();                          //~ ERROR `pub_mod` is private
2019-08-28T22:33:41.4014807Z 
2019-08-28T22:33:41.4014853Z error[E0624]: method `private` is private
2019-08-28T22:33:41.4015124Z   --> /checkout/src/test/ui/explore-issue-38412.rs:52:7
2019-08-28T22:33:41.4015174Z    |
2019-08-28T22:33:41.4015174Z    |
2019-08-28T22:33:41.4015227Z LL |     r.private();                          //~ ERROR `private` is private
2019-08-28T22:33:41.4015443Z 
2019-08-28T22:33:41.4015748Z error[E0658]: use of unstable library feature 'unstable_undeclared'
2019-08-28T22:33:41.4016004Z   --> /checkout/src/test/ui/explore-issue-38412.rs:57:7
2019-08-28T22:33:41.4016067Z    |
2019-08-28T22:33:41.4016067Z    |
2019-08-28T22:33:41.4016120Z LL |     t.unstable_undeclared_trait_method(); //~ ERROR use of unstable library feature
2019-08-28T22:33:41.4016235Z    |
2019-08-28T22:33:41.4016235Z    |
2019-08-28T22:33:41.4016561Z    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.4016622Z    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.4016948Z error[E0658]: use of unstable library feature 'unstable_undeclared'
2019-08-28T22:33:41.4017518Z   --> /checkout/src/test/ui/explore-issue-38412.rs:61:7
2019-08-28T22:33:41.4017580Z    |
2019-08-28T22:33:41.4017580Z    |
2019-08-28T22:33:41.4017653Z LL |     t.unstable_undeclared();              //~ ERROR use of unstable library feature
2019-08-28T22:33:41.4018005Z    |
2019-08-28T22:33:41.4018005Z    |
2019-08-28T22:33:41.4018407Z    = note: for more information, see ***/issues/38412
2019-08-28T22:33:41.4018469Z    = help: add `#![feature(unstable_undeclared)]` to the crate attributes to enable
2019-08-28T22:33:41.4018556Z error[E0624]: method `pub_crate` is private
2019-08-28T22:33:41.4018850Z   --> /checkout/src/test/ui/explore-issue-38412.rs:63:7
2019-08-28T22:33:41.4018901Z    |
2019-08-28T22:33:41.4018901Z    |
2019-08-28T22:33:41.4019305Z LL |     t.pub_crate();                        //~ ERROR `pub_crate` is private
2019-08-28T22:33:41.4019544Z 
2019-08-28T22:33:41.4019544Z 
2019-08-28T22:33:41.4019587Z error[E0624]: method `pub_mod` is private
2019-08-28T22:33:41.4019928Z   --> /checkout/src/test/ui/explore-issue-38412.rs:64:7
2019-08-28T22:33:41.4019992Z    |
2019-08-28T22:33:41.4020042Z LL |     t.pub_mod();                          //~ ERROR `pub_mod` is private
2019-08-28T22:33:41.4020145Z 
2019-08-28T22:33:41.4020188Z error[E0624]: method `private` is private
2019-08-28T22:33:41.4020429Z   --> /checkout/src/test/ui/explore-issue-38412.rs:65:7
2019-08-28T22:33:41.4020474Z    |
2019-08-28T22:33:41.4020474Z    |
2019-08-28T22:33:41.4020539Z LL |     t.private();                          //~ ERROR `private` is private
2019-08-28T22:33:41.4020613Z 
2019-08-28T22:33:41.4020670Z error: aborting due to 19 previous errors
2019-08-28T22:33:41.4020700Z 
2019-08-28T22:33:41.4020752Z Some errors have detailed explanations: E0616, E0624, E0658.
---
2019-08-28T22:33:41.4021326Z 
2019-08-28T22:33:41.4021572Z ---- [ui] ui/hygiene/nested_macro_privacy.rs stdout ----
2019-08-28T22:33:41.4021629Z diff of stderr:
2019-08-28T22:33:41.4021657Z 
2019-08-28T22:33:41.4021889Z - error[E0616]: field `i` of struct `foo::S` is private
2019-08-28T22:33:41.4021956Z + error[E0616]: field `i` of `struct` `foo::S` is private
2019-08-28T22:33:41.4022223Z 3    |
2019-08-28T22:33:41.4022223Z 3    |
2019-08-28T22:33:41.4022282Z 4 LL |     S::default().i;
2019-08-28T22:33:41.4022336Z 
2019-08-28T22:33:41.4022382Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4022382Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4022723Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/nested_macro_privacy/nested_macro_privacy.stderr
2019-08-28T22:33:41.4022980Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4023253Z To only update this specific test, also pass `--test-args hygiene/nested_macro_privacy.rs`
2019-08-28T22:33:41.4023348Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4023522Z status: exit code: 1
2019-08-28T22:33:41.4023522Z status: exit code: 1
2019-08-28T22:33:41.4024354Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/nested_macro_privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/nested_macro_privacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/nested_macro_privacy/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4024689Z ------------------------------------------
2019-08-28T22:33:41.4024726Z 
2019-08-28T22:33:41.4024944Z ------------------------------------------
2019-08-28T22:33:41.4024989Z stderr:
2019-08-28T22:33:41.4024989Z stderr:
2019-08-28T22:33:41.4025218Z ------------------------------------------
2019-08-28T22:33:41.4025268Z error[E0616]: field `i` of `struct` `foo::S` is private
2019-08-28T22:33:41.4025589Z    |
2019-08-28T22:33:41.4025589Z    |
2019-08-28T22:33:41.4025638Z LL |     S::default().i; //~ ERROR field `i` of struct `foo::S` is private
2019-08-28T22:33:41.4025731Z 
2019-08-28T22:33:41.4025774Z error: aborting due to previous error
2019-08-28T22:33:41.4025803Z 
2019-08-28T22:33:41.4026047Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4026047Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4026097Z 
2019-08-28T22:33:41.4026389Z ------------------------------------------
2019-08-28T22:33:41.4026430Z 
2019-08-28T22:33:41.4026456Z 
2019-08-28T22:33:41.4026708Z ---- [ui] ui/issues/issue-25386.rs stdout ----
2019-08-28T22:33:41.4026775Z diff of stderr:
2019-08-28T22:33:41.4026803Z 
2019-08-28T22:33:41.4027045Z - error[E0616]: field `c_object` of struct `stuff::Item` is private
2019-08-28T22:33:41.4027117Z + error[E0616]: field `c_object` of `struct` `stuff::Item` is private
2019-08-28T22:33:41.4027344Z 2   --> $DIR/issue-25386.rs:19:11
2019-08-28T22:33:41.4027389Z 3    |
2019-08-28T22:33:41.4027433Z 4 LL |         (*$var.c_object).$member.is_some()
2019-08-28T22:33:41.4027480Z 
2019-08-28T22:33:41.4027525Z 7 LL |     println!("{}", check_ptr_exist!(item, name));
2019-08-28T22:33:41.4027844Z 9 
2019-08-28T22:33:41.4027844Z 9 
2019-08-28T22:33:41.4028083Z - error[E0616]: field `name` of struct `stuff::CObj` is private
2019-08-28T22:33:41.4028144Z + error[E0616]: field `name` of `struct` `stuff::CObj` is private
2019-08-28T22:33:41.4028376Z 11   --> $DIR/issue-25386.rs:19:9
2019-08-28T22:33:41.4028421Z 12    |
2019-08-28T22:33:41.4028466Z 13 LL |         (*$var.c_object).$member.is_some()
2019-08-28T22:33:41.4028536Z 
2019-08-28T22:33:41.4028581Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4028878Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25386/issue-25386.stderr
2019-08-28T22:33:41.4028878Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25386/issue-25386.stderr
2019-08-28T22:33:41.4029464Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4029752Z To only update this specific test, also pass `--test-args issues/issue-25386.rs`
2019-08-28T22:33:41.4029832Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4029894Z status: exit code: 1
2019-08-28T22:33:41.4029894Z status: exit code: 1
2019-08-28T22:33:41.4030645Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-25386.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25386" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25386/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4031134Z ------------------------------------------
2019-08-28T22:33:41.4031169Z 
2019-08-28T22:33:41.4031408Z ------------------------------------------
2019-08-28T22:33:41.4031453Z stderr:
2019-08-28T22:33:41.4031453Z stderr:
2019-08-28T22:33:41.4031662Z ------------------------------------------
2019-08-28T22:33:41.4031730Z error[E0616]: field `c_object` of `struct` `stuff::Item` is private
2019-08-28T22:33:41.4031969Z   --> /checkout/src/test/ui/issues/issue-25386.rs:19:11
2019-08-28T22:33:41.4032019Z    |
2019-08-28T22:33:41.4032088Z LL |         (*$var.c_object).$member.is_some()
2019-08-28T22:33:41.4032177Z ...
2019-08-28T22:33:41.4032177Z ...
2019-08-28T22:33:41.4032222Z LL |     println!("{}", check_ptr_exist!(item, name));
2019-08-28T22:33:41.4032530Z 
2019-08-28T22:33:41.4032530Z 
2019-08-28T22:33:41.4032577Z error[E0616]: field `name` of `struct` `stuff::CObj` is private
2019-08-28T22:33:41.4032838Z   --> /checkout/src/test/ui/issues/issue-25386.rs:19:9
2019-08-28T22:33:41.4032886Z    |
2019-08-28T22:33:41.4032930Z LL |         (*$var.c_object).$member.is_some()
2019-08-28T22:33:41.4033034Z ...
2019-08-28T22:33:41.4033034Z ...
2019-08-28T22:33:41.4033078Z LL |     println!("{}", check_ptr_exist!(item, name));
2019-08-28T22:33:41.4033380Z 
2019-08-28T22:33:41.4033504Z error: aborting due to 2 previous errors
2019-08-28T22:33:41.4033542Z 
2019-08-28T22:33:41.4033819Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4033819Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4033870Z 
2019-08-28T22:33:41.4034082Z ------------------------------------------
2019-08-28T22:33:41.4034113Z 
2019-08-28T22:33:41.4034139Z 
2019-08-28T22:33:41.4034373Z ---- [ui] ui/issues/issue-26472.rs stdout ----
2019-08-28T22:33:41.4034419Z diff of stderr:
2019-08-28T22:33:41.4034447Z 
2019-08-28T22:33:41.4034688Z - error[E0616]: field `len` of struct `sub::S` is private
2019-08-28T22:33:41.4034757Z + error[E0616]: field `len` of `struct` `sub::S` is private
2019-08-28T22:33:41.4034970Z 2   --> $DIR/issue-26472.rs:11:13
2019-08-28T22:33:41.4035015Z 3    |
2019-08-28T22:33:41.4035072Z 4 LL |     let v = s.len;
2019-08-28T22:33:41.4035142Z 6    |               |
2019-08-28T22:33:41.4035142Z 6    |               |
2019-08-28T22:33:41.4035193Z 7    |               help: a method `len` also exists, call it with parentheses: `len()`
2019-08-28T22:33:41.4035256Z 8 
2019-08-28T22:33:41.4035500Z - error[E0616]: field `len` of struct `sub::S` is private
2019-08-28T22:33:41.4035553Z + error[E0616]: field `len` of `struct` `sub::S` is private
2019-08-28T22:33:41.4035785Z 10   --> $DIR/issue-26472.rs:12:5
2019-08-28T22:33:41.4035830Z 11    |
2019-08-28T22:33:41.4035870Z 12 LL |     s.len = v;
2019-08-28T22:33:41.4035944Z 
2019-08-28T22:33:41.4035989Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4036295Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26472/issue-26472.stderr
2019-08-28T22:33:41.4036295Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26472/issue-26472.stderr
2019-08-28T22:33:41.4036564Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4036826Z To only update this specific test, also pass `--test-args issues/issue-26472.rs`
2019-08-28T22:33:41.4036920Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4036964Z status: exit code: 1
2019-08-28T22:33:41.4036964Z status: exit code: 1
2019-08-28T22:33:41.4037706Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26472.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26472" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26472/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4038159Z ------------------------------------------
2019-08-28T22:33:41.4038194Z 
2019-08-28T22:33:41.4038429Z ------------------------------------------
2019-08-28T22:33:41.4038474Z stderr:
2019-08-28T22:33:41.4038474Z stderr:
2019-08-28T22:33:41.4038685Z ------------------------------------------
2019-08-28T22:33:41.4038752Z error[E0616]: field `len` of `struct` `sub::S` is private
2019-08-28T22:33:41.4038991Z   --> /checkout/src/test/ui/issues/issue-26472.rs:11:13
2019-08-28T22:33:41.4039048Z    |
2019-08-28T22:33:41.4039403Z LL |     let v = s.len; //~ ERROR field `len` of struct `sub::S` is private
2019-08-28T22:33:41.4039673Z    |             ^^---
2019-08-28T22:33:41.4039722Z    |               |
2019-08-28T22:33:41.4039791Z    |               help: a method `len` also exists, call it with parentheses: `len()`
2019-08-28T22:33:41.4039825Z 
2019-08-28T22:33:41.4039870Z error[E0616]: field `len` of `struct` `sub::S` is private
2019-08-28T22:33:41.4040143Z   --> /checkout/src/test/ui/issues/issue-26472.rs:12:5
2019-08-28T22:33:41.4040191Z    |
2019-08-28T22:33:41.4040239Z LL |     s.len = v; //~ ERROR field `len` of struct `sub::S` is private
2019-08-28T22:33:41.4040327Z 
2019-08-28T22:33:41.4040371Z error: aborting due to 2 previous errors
2019-08-28T22:33:41.4040400Z 
2019-08-28T22:33:41.4040643Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4040643Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4040692Z 
2019-08-28T22:33:41.4041010Z ------------------------------------------
2019-08-28T22:33:41.4041052Z 
2019-08-28T22:33:41.4041077Z 
2019-08-28T22:33:41.4041351Z ---- [ui] ui/issues/issue-3763.rs stdout ----
2019-08-28T22:33:41.4041398Z diff of stderr:
2019-08-28T22:33:41.4041425Z 
2019-08-28T22:33:41.4041672Z - error[E0616]: field `priv_field` of struct `my_mod::MyStruct` is private
2019-08-28T22:33:41.4041746Z + error[E0616]: field `priv_field` of `struct` `my_mod::MyStruct` is private
2019-08-28T22:33:41.4041972Z 2   --> $DIR/issue-3763.rs:15:19
2019-08-28T22:33:41.4042016Z 3    |
2019-08-28T22:33:41.4042078Z 4 LL |     let _woohoo = (&my_struct).priv_field;
2019-08-28T22:33:41.4042152Z 5    |                   ^^^^^^^^^^^^^^^^^^^^^^^
2019-08-28T22:33:41.4042194Z 6 
2019-08-28T22:33:41.4042194Z 6 
2019-08-28T22:33:41.4042459Z - error[E0616]: field `priv_field` of struct `my_mod::MyStruct` is private
2019-08-28T22:33:41.4042513Z + error[E0616]: field `priv_field` of `struct` `my_mod::MyStruct` is private
2019-08-28T22:33:41.4042734Z 8   --> $DIR/issue-3763.rs:18:19
2019-08-28T22:33:41.4042795Z 9    |
2019-08-28T22:33:41.4042841Z 10 LL |     let _woohoo = (Box::new(my_struct)).priv_field;
2019-08-28T22:33:41.4042872Z 
2019-08-28T22:33:41.4042916Z 22 LL |     (Box::new(my_struct)).happyfun();
2019-08-28T22:33:41.4043021Z 24 
2019-08-28T22:33:41.4043021Z 24 
2019-08-28T22:33:41.4043273Z - error[E0616]: field `priv_field` of struct `my_mod::MyStruct` is private
2019-08-28T22:33:41.4043352Z + error[E0616]: field `priv_field` of `struct` `my_mod::MyStruct` is private
2019-08-28T22:33:41.4043567Z 26   --> $DIR/issue-3763.rs:24:16
2019-08-28T22:33:41.4043613Z 27    |
2019-08-28T22:33:41.4043676Z 28 LL |     let nope = my_struct.priv_field;
2019-08-28T22:33:41.4043730Z 
2019-08-28T22:33:41.4043774Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4044084Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3763/issue-3763.stderr
2019-08-28T22:33:41.4044084Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3763/issue-3763.stderr
2019-08-28T22:33:41.4044344Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4044609Z To only update this specific test, also pass `--test-args issues/issue-3763.rs`
2019-08-28T22:33:41.4044704Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4044747Z status: exit code: 1
2019-08-28T22:33:41.4044747Z status: exit code: 1
2019-08-28T22:33:41.4045512Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3763.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3763" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3763/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4045973Z ------------------------------------------
2019-08-28T22:33:41.4046008Z 
2019-08-28T22:33:41.4046229Z ------------------------------------------
2019-08-28T22:33:41.4046275Z stderr:
2019-08-28T22:33:41.4046275Z stderr:
2019-08-28T22:33:41.4046506Z ------------------------------------------
2019-08-28T22:33:41.4046559Z error[E0616]: field `priv_field` of `struct` `my_mod::MyStruct` is private
2019-08-28T22:33:41.4046798Z   --> /checkout/src/test/ui/issues/issue-3763.rs:15:19
2019-08-28T22:33:41.4046866Z    |
2019-08-28T22:33:41.4046922Z LL |     let _woohoo = (&my_struct).priv_field;
2019-08-28T22:33:41.4047005Z 
2019-08-28T22:33:41.4047005Z 
2019-08-28T22:33:41.4047070Z error[E0616]: field `priv_field` of `struct` `my_mod::MyStruct` is private
2019-08-28T22:33:41.4047337Z   --> /checkout/src/test/ui/issues/issue-3763.rs:18:19
2019-08-28T22:33:41.4047386Z    |
2019-08-28T22:33:41.4047451Z LL |     let _woohoo = (Box::new(my_struct)).priv_field;
2019-08-28T22:33:41.4047616Z 
2019-08-28T22:33:41.4047616Z 
2019-08-28T22:33:41.4047689Z error[E0624]: method `happyfun` is private
2019-08-28T22:33:41.4048407Z   --> /checkout/src/test/ui/issues/issue-3763.rs:21:18
2019-08-28T22:33:41.4048464Z    |
2019-08-28T22:33:41.4048512Z LL |     (&my_struct).happyfun();               //~ ERROR method `happyfun` is private
2019-08-28T22:33:41.4048611Z 
2019-08-28T22:33:41.4048611Z 
2019-08-28T22:33:41.4048654Z error[E0624]: method `happyfun` is private
2019-08-28T22:33:41.4048932Z   --> /checkout/src/test/ui/issues/issue-3763.rs:23:27
2019-08-28T22:33:41.4048981Z    |
2019-08-28T22:33:41.4049035Z LL |     (Box::new(my_struct)).happyfun();          //~ ERROR method `happyfun` is private
2019-08-28T22:33:41.4049135Z 
2019-08-28T22:33:41.4049135Z 
2019-08-28T22:33:41.4049184Z error[E0616]: field `priv_field` of `struct` `my_mod::MyStruct` is private
2019-08-28T22:33:41.4049701Z   --> /checkout/src/test/ui/issues/issue-3763.rs:24:16
2019-08-28T22:33:41.4049786Z    |
2019-08-28T22:33:41.4049835Z LL |     let nope = my_struct.priv_field;
2019-08-28T22:33:41.4049918Z 
2019-08-28T22:33:41.4049980Z error: aborting due to 5 previous errors
2019-08-28T22:33:41.4050011Z 
2019-08-28T22:33:41.4050058Z Some errors have detailed explanations: E0616, E0624.
---
2019-08-28T22:33:41.4050690Z 
2019-08-28T22:33:41.4050949Z ---- [ui] ui/issues/issue-54062.rs stdout ----
2019-08-28T22:33:41.4051000Z diff of stderr:
2019-08-28T22:33:41.4051029Z 
2019-08-28T22:33:41.4051289Z - error[E0616]: field `inner` of struct `std::sync::Mutex` is private
2019-08-28T22:33:41.4051362Z + error[E0616]: field `inner` of `struct` `std::sync::Mutex` is private
2019-08-28T22:33:41.4051605Z 2   --> $DIR/issue-54062.rs:10:13
2019-08-28T22:33:41.4051653Z 3    |
2019-08-28T22:33:41.4051717Z 4 LL |     let _ = test.comps.inner.lock().unwrap();
2019-08-28T22:33:41.4051777Z 
2019-08-28T22:33:41.4051825Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4052154Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54062/issue-54062.stderr
2019-08-28T22:33:41.4052154Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54062/issue-54062.stderr
2019-08-28T22:33:41.4052596Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4052886Z To only update this specific test, also pass `--test-args issues/issue-54062.rs`
2019-08-28T22:33:41.4052991Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4053038Z status: exit code: 1
2019-08-28T22:33:41.4053038Z status: exit code: 1
2019-08-28T22:33:41.4053826Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54062.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54062" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54062/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4054173Z ------------------------------------------
2019-08-28T22:33:41.4054216Z 
2019-08-28T22:33:41.4054452Z ------------------------------------------
2019-08-28T22:33:41.4054516Z stderr:
2019-08-28T22:33:41.4054516Z stderr:
2019-08-28T22:33:41.4054746Z ------------------------------------------
2019-08-28T22:33:41.4054800Z error[E0616]: field `inner` of `struct` `std::sync::Mutex` is private
2019-08-28T22:33:41.4055057Z   --> /checkout/src/test/ui/issues/issue-54062.rs:10:13
2019-08-28T22:33:41.4055125Z    |
2019-08-28T22:33:41.4055173Z LL |     let _ = test.comps.inner.lock().unwrap();
2019-08-28T22:33:41.4055369Z 
2019-08-28T22:33:41.4055369Z 
2019-08-28T22:33:41.4055725Z error[E0599]: no method named `unwrap` found for type `std::sys_common::mutex::MutexGuard<'_>` in the current scope
2019-08-28T22:33:41.4055993Z   --> /checkout/src/test/ui/issues/issue-54062.rs:10:37
2019-08-28T22:33:41.4056064Z    |
2019-08-28T22:33:41.4056111Z LL |     let _ = test.comps.inner.lock().unwrap();
2019-08-28T22:33:41.4056202Z 
2019-08-28T22:33:41.4056263Z error: aborting due to 2 previous errors
2019-08-28T22:33:41.4056293Z 
2019-08-28T22:33:41.4056339Z Some errors have detailed explanations: E0599, E0616.
---
2019-08-28T22:33:41.4056945Z 
2019-08-28T22:33:41.4057189Z ---- [ui] ui/paren-span.rs stdout ----
2019-08-28T22:33:41.4057246Z diff of stderr:
2019-08-28T22:33:41.4057277Z 
2019-08-28T22:33:41.4057518Z - error[E0616]: field `x` of struct `m::S` is private
2019-08-28T22:33:41.4057588Z + error[E0616]: field `x` of `struct` `m::S` is private
2019-08-28T22:33:41.4058059Z 3    |
2019-08-28T22:33:41.4058059Z 3    |
2019-08-28T22:33:41.4058123Z 4 LL |     paren!(s.x);
2019-08-28T22:33:41.4058181Z 
2019-08-28T22:33:41.4058228Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4058565Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/paren-span/paren-span.stderr
2019-08-28T22:33:41.4058565Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/paren-span/paren-span.stderr
2019-08-28T22:33:41.4058830Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4059401Z To only update this specific test, also pass `--test-args paren-span.rs`
2019-08-28T22:33:41.4059510Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4059556Z status: exit code: 1
2019-08-28T22:33:41.4059556Z status: exit code: 1
2019-08-28T22:33:41.4060632Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/paren-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/paren-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/paren-span/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4061196Z ------------------------------------------
2019-08-28T22:33:41.4061232Z 
2019-08-28T22:33:41.4061455Z ------------------------------------------
2019-08-28T22:33:41.4061500Z stderr:
2019-08-28T22:33:41.4061500Z stderr:
2019-08-28T22:33:41.4061730Z ------------------------------------------
2019-08-28T22:33:41.4061781Z error[E0616]: field `x` of `struct` `m::S` is private
2019-08-28T22:33:41.4062082Z    |
2019-08-28T22:33:41.4062082Z    |
2019-08-28T22:33:41.4062131Z LL |     paren!(s.x); //~ ERROR field `x` of struct `m::S` is private
2019-08-28T22:33:41.4062210Z 
2019-08-28T22:33:41.4062268Z error: aborting due to previous error
2019-08-28T22:33:41.4062297Z 
2019-08-28T22:33:41.4062546Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4062546Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4062579Z 
2019-08-28T22:33:41.4062808Z ------------------------------------------
2019-08-28T22:33:41.4062847Z 
2019-08-28T22:33:41.4062872Z 
2019-08-28T22:33:41.4063111Z ---- [ui] ui/privacy/private-struct-field-cross-crate.rs stdout ----
2019-08-28T22:33:41.4063175Z diff of stderr:
2019-08-28T22:33:41.4063204Z 
2019-08-28T22:33:41.4063452Z - error[E0616]: field `meows` of struct `cci_class::kitties::cat` is private
2019-08-28T22:33:41.4063524Z + error[E0616]: field `meows` of `struct` `cci_class::kitties::cat` is private
2019-08-28T22:33:41.4063892Z 3    |
2019-08-28T22:33:41.4063892Z 3    |
2019-08-28T22:33:41.4063947Z 4 LL |   assert_eq!(nyan.meows, 52);
2019-08-28T22:33:41.4064019Z 
2019-08-28T22:33:41.4064064Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4064439Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field-cross-crate/private-struct-field-cross-crate.stderr
2019-08-28T22:33:41.4064439Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field-cross-crate/private-struct-field-cross-crate.stderr
2019-08-28T22:33:41.4064711Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4065003Z To only update this specific test, also pass `--test-args privacy/private-struct-field-cross-crate.rs`
2019-08-28T22:33:41.4065101Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4065145Z status: exit code: 1
2019-08-28T22:33:41.4065145Z status: exit code: 1
2019-08-28T22:33:41.4066332Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-struct-field-cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field-cross-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field-cross-crate/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4066689Z ------------------------------------------
2019-08-28T22:33:41.4066724Z 
2019-08-28T22:33:41.4066941Z ------------------------------------------
2019-08-28T22:33:41.4066986Z stderr:
2019-08-28T22:33:41.4066986Z stderr:
2019-08-28T22:33:41.4067217Z ------------------------------------------
2019-08-28T22:33:41.4067270Z error[E0616]: field `meows` of `struct` `cci_class::kitties::cat` is private
2019-08-28T22:33:41.4067595Z    |
2019-08-28T22:33:41.4067595Z    |
2019-08-28T22:33:41.4067647Z LL |   assert_eq!(nyan.meows, 52);
2019-08-28T22:33:41.4067721Z 
2019-08-28T22:33:41.4067779Z error: aborting due to previous error
2019-08-28T22:33:41.4067808Z 
2019-08-28T22:33:41.4068057Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4068057Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4068090Z 
2019-08-28T22:33:41.4068319Z ------------------------------------------
2019-08-28T22:33:41.4068350Z 
2019-08-28T22:33:41.4068504Z 
2019-08-28T22:33:41.4068772Z ---- [ui] ui/privacy/private-struct-field.rs stdout ----
2019-08-28T22:33:41.4068840Z diff of stderr:
2019-08-28T22:33:41.4068868Z 
2019-08-28T22:33:41.4069520Z - error[E0616]: field `meows` of struct `cat::Cat` is private
2019-08-28T22:33:41.4069602Z + error[E0616]: field `meows` of `struct` `cat::Cat` is private
2019-08-28T22:33:41.4069922Z 3    |
2019-08-28T22:33:41.4069922Z 3    |
2019-08-28T22:33:41.4069966Z 4 LL |     assert_eq!(nyan.meows, 52);
2019-08-28T22:33:41.4070049Z 
2019-08-28T22:33:41.4070094Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4070410Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field/private-struct-field.stderr
2019-08-28T22:33:41.4070410Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field/private-struct-field.stderr
2019-08-28T22:33:41.4070682Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4071120Z To only update this specific test, also pass `--test-args privacy/private-struct-field.rs`
2019-08-28T22:33:41.4071235Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4071278Z status: exit code: 1
2019-08-28T22:33:41.4071278Z status: exit code: 1
2019-08-28T22:33:41.4072178Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-struct-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4075394Z ------------------------------------------
2019-08-28T22:33:41.4075472Z 
2019-08-28T22:33:41.4075712Z ------------------------------------------
2019-08-28T22:33:41.4075779Z stderr:
2019-08-28T22:33:41.4075779Z stderr:
2019-08-28T22:33:41.4076014Z ------------------------------------------
2019-08-28T22:33:41.4076066Z error[E0616]: field `meows` of `struct` `cat::Cat` is private
2019-08-28T22:33:41.4076389Z    |
2019-08-28T22:33:41.4076389Z    |
2019-08-28T22:33:41.4076441Z LL |     assert_eq!(nyan.meows, 52);    //~ ERROR field `meows` of struct `cat::Cat` is private
2019-08-28T22:33:41.4076525Z 
2019-08-28T22:33:41.4076591Z error: aborting due to previous error
2019-08-28T22:33:41.4076620Z 
2019-08-28T22:33:41.4076867Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4076867Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4076902Z 
2019-08-28T22:33:41.4077132Z ------------------------------------------
2019-08-28T22:33:41.4077162Z 
2019-08-28T22:33:41.4077188Z 
2019-08-28T22:33:41.4077417Z ---- [ui] ui/privacy/union-field-privacy-2.rs stdout ----
2019-08-28T22:33:41.4077481Z diff of stderr:
2019-08-28T22:33:41.4077518Z 
2019-08-28T22:33:41.4077747Z - error[E0616]: field `c` of struct `m::U` is private
2019-08-28T22:33:41.4077815Z + error[E0616]: field `c` of `union` `m::U` is private
2019-08-28T22:33:41.4078039Z 2   --> $DIR/union-field-privacy-2.rs:14:13
2019-08-28T22:33:41.4078085Z 3    |
2019-08-28T22:33:41.4078127Z 4 LL |     let c = u.c;
2019-08-28T22:33:41.4078198Z 
2019-08-28T22:33:41.4078243Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4078243Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4078568Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/union-field-privacy-2/union-field-privacy-2.stderr
2019-08-28T22:33:41.4078845Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4079402Z To only update this specific test, also pass `--test-args privacy/union-field-privacy-2.rs`
2019-08-28T22:33:41.4079514Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4079558Z status: exit code: 1
2019-08-28T22:33:41.4079558Z status: exit code: 1
2019-08-28T22:33:41.4080835Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/union-field-privacy-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/union-field-privacy-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/union-field-privacy-2/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4081224Z ------------------------------------------
2019-08-28T22:33:41.4081276Z 
2019-08-28T22:33:41.4081499Z ------------------------------------------
2019-08-28T22:33:41.4081545Z stderr:
2019-08-28T22:33:41.4081545Z stderr:
2019-08-28T22:33:41.4081779Z ------------------------------------------
2019-08-28T22:33:41.4081828Z error[E0616]: field `c` of `union` `m::U` is private
2019-08-28T22:33:41.4082087Z   --> /checkout/src/test/ui/privacy/union-field-privacy-2.rs:14:13
2019-08-28T22:33:41.4082153Z    |
2019-08-28T22:33:41.4082205Z LL |     let c = u.c; //~ ERROR field `c` of struct `m::U` is private
2019-08-28T22:33:41.4082279Z 
2019-08-28T22:33:41.4082338Z error: aborting due to previous error
2019-08-28T22:33:41.4082368Z 
2019-08-28T22:33:41.4082612Z For more information about this error, try `rustc --explain E0616`.
---
2019-08-28T22:33:41.4083496Z 
2019-08-28T22:33:41.4083538Z 34 LL |     use foo::bar::f;
2019-08-28T22:33:41.4083582Z 35    |                   ^
2019-08-28T22:33:41.4083640Z 36 
2019-08-28T22:33:41.4083885Z - error[E0616]: field `x` of struct `foo::bar::S` is private
2019-08-28T22:33:41.4083948Z + error[E0616]: field `x` of `struct` `foo::bar::S` is private
2019-08-28T22:33:41.4084215Z 39    |
2019-08-28T22:33:41.4084215Z 39    |
2019-08-28T22:33:41.4084257Z 40 LL |     S::default().x;
2019-08-28T22:33:41.4084286Z 
2019-08-28T22:33:41.4084342Z 52 LL |     S::g();
2019-08-28T22:33:41.4084426Z 54 
2019-08-28T22:33:41.4084426Z 54 
2019-08-28T22:33:41.4084694Z - error[E0616]: field `y` of struct `pub_restricted::Universe` is private
2019-08-28T22:33:41.4084757Z + error[E0616]: field `y` of `struct` `pub_restricted::Universe` is private
2019-08-28T22:33:41.4085029Z 57    |
2019-08-28T22:33:41.4085071Z 58 LL |     let _ = u.y;
2019-08-28T22:33:41.4085099Z 
2019-08-28T22:33:41.4085140Z 59    |             ^^^
2019-08-28T22:33:41.4085140Z 59    |             ^^^
2019-08-28T22:33:41.4085199Z 60 
2019-08-28T22:33:41.4085449Z - error[E0616]: field `z` of struct `pub_restricted::Universe` is private
2019-08-28T22:33:41.4085512Z + error[E0616]: field `z` of `struct` `pub_restricted::Universe` is private
2019-08-28T22:33:41.4085778Z 63    |
2019-08-28T22:33:41.4085819Z 64 LL |     let _ = u.z;
2019-08-28T22:33:41.4085846Z 
2019-08-28T22:33:41.4085872Z 
2019-08-28T22:33:41.4085872Z 
2019-08-28T22:33:41.4085931Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4086546Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/restricted/test/test.stderr
2019-08-28T22:33:41.4086821Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4087120Z To only update this specific test, also pass `--test-args privacy/restricted/test.rs`
2019-08-28T22:33:41.4087201Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4087265Z status: exit code: 1
2019-08-28T22:33:41.4087265Z status: exit code: 1
2019-08-28T22:33:41.4088015Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/restricted/test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/restricted/test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/restricted/test/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4088492Z ------------------------------------------
2019-08-28T22:33:41.4088536Z 
2019-08-28T22:33:41.4088777Z ------------------------------------------
2019-08-28T22:33:41.4088822Z stderr:
2019-08-28T22:33:41.4088822Z stderr:
2019-08-28T22:33:41.4089426Z ------------------------------------------
2019-08-28T22:33:41.4090094Z error[E0433]: failed to resolve: maybe a missing crate `bad`?
2019-08-28T22:33:41.4090503Z    |
2019-08-28T22:33:41.4090503Z    |
2019-08-28T22:33:41.4090570Z LL |     pub(in bad::path) mod m1 {} //~ ERROR failed to resolve: maybe a missing crate `bad`?
2019-08-28T22:33:41.4090639Z    |            ^^^ maybe a missing crate `bad`?
2019-08-28T22:33:41.4090717Z error: visibilities can only be restricted to ancestor modules
2019-08-28T22:33:41.4090982Z   --> /checkout/src/test/ui/privacy/restricted/test.rs:51:12
2019-08-28T22:33:41.4091032Z    |
2019-08-28T22:33:41.4091032Z    |
2019-08-28T22:33:41.4091082Z LL |     pub(in foo) mod m2 {} //~ ERROR visibilities can only be restricted to ancestor modules
2019-08-28T22:33:41.4091308Z 
2019-08-28T22:33:41.4091308Z 
2019-08-28T22:33:41.4091581Z error[E0364]: `f` is private, and cannot be re-exported
2019-08-28T22:33:41.4091888Z    |
2019-08-28T22:33:41.4091888Z    |
2019-08-28T22:33:41.4092136Z LL |         pub(super) use foo::bar::f as g; //~ ERROR cannot be re-exported
2019-08-28T22:33:41.4092310Z    |
2019-08-28T22:33:41.4092310Z    |
2019-08-28T22:33:41.4092357Z note: consider marking `f` as `pub` in the imported module
2019-08-28T22:33:41.4092670Z    |
2019-08-28T22:33:41.4092670Z    |
2019-08-28T22:33:41.4092920Z LL |         pub(super) use foo::bar::f as g; //~ ERROR cannot be re-exported
2019-08-28T22:33:41.4093021Z 
2019-08-28T22:33:41.4093065Z error[E0603]: struct `Crate` is private
2019-08-28T22:33:41.4093309Z   --> /checkout/src/test/ui/privacy/restricted/test.rs:38:25
2019-08-28T22:33:41.4093355Z    |
2019-08-28T22:33:41.4093355Z    |
2019-08-28T22:33:41.4093417Z LL |     use pub_restricted::Crate; //~ ERROR private
2019-08-28T22:33:41.4093464Z    |                         ^^^^^
2019-08-28T22:33:41.4093493Z 
2019-08-28T22:33:41.4093552Z error[E0603]: function `f` is private
2019-08-28T22:33:41.4093794Z   --> /checkout/src/test/ui/privacy/restricted/test.rs:30:19
2019-08-28T22:33:41.4093840Z    |
2019-08-28T22:33:41.4093883Z LL |     use foo::bar::f; //~ ERROR private
2019-08-28T22:33:41.4093982Z 
2019-08-28T22:33:41.4093982Z 
2019-08-28T22:33:41.4094026Z error[E0616]: field `x` of `struct` `foo::bar::S` is private
2019-08-28T22:33:41.4094330Z    |
2019-08-28T22:33:41.4094330Z    |
2019-08-28T22:33:41.4094373Z LL |     S::default().x; //~ ERROR private
2019-08-28T22:33:41.4094461Z 
2019-08-28T22:33:41.4094502Z error[E0624]: method `f` is private
2019-08-28T22:33:41.4094750Z   --> /checkout/src/test/ui/privacy/restricted/test.rs:32:18
2019-08-28T22:33:41.4094812Z    |
2019-08-28T22:33:41.4094812Z    |
2019-08-28T22:33:41.4094857Z LL |     S::default().f(); //~ ERROR private
2019-08-28T22:33:41.4094928Z 
2019-08-28T22:33:41.4094928Z 
2019-08-28T22:33:41.4094986Z error[E0624]: method `g` is private
2019-08-28T22:33:41.4095271Z    |
2019-08-28T22:33:41.4095271Z    |
2019-08-28T22:33:41.4095424Z LL |     S::g(); //~ ERROR private
2019-08-28T22:33:41.4095496Z 
2019-08-28T22:33:41.4095496Z 
2019-08-28T22:33:41.4095542Z error[E0616]: field `y` of `struct` `pub_restricted::Universe` is private
2019-08-28T22:33:41.4095879Z    |
2019-08-28T22:33:41.4095879Z    |
2019-08-28T22:33:41.4095923Z LL |     let _ = u.y; //~ ERROR private
2019-08-28T22:33:41.4096010Z 
2019-08-28T22:33:41.4096010Z 
2019-08-28T22:33:41.4096065Z error[E0616]: field `z` of `struct` `pub_restricted::Universe` is private
2019-08-28T22:33:41.4096373Z    |
2019-08-28T22:33:41.4096373Z    |
2019-08-28T22:33:41.4096416Z LL |     let _ = u.z; //~ ERROR private
2019-08-28T22:33:41.4096487Z 
2019-08-28T22:33:41.4096487Z 
2019-08-28T22:33:41.4096544Z error[E0624]: method `g` is private
2019-08-28T22:33:41.4096836Z    |
2019-08-28T22:33:41.4096894Z LL |     u.g(); //~ ERROR private
2019-08-28T22:33:41.4096938Z    |       ^
2019-08-28T22:33:41.4096965Z 
2019-08-28T22:33:41.4096965Z 
2019-08-28T22:33:41.4097006Z error[E0624]: method `h` is private
2019-08-28T22:33:41.4097259Z   --> /checkout/src/test/ui/privacy/restricted/test.rs:46:7
2019-08-28T22:33:41.4097305Z    |
2019-08-28T22:33:41.4097347Z LL |     u.h(); //~ ERROR private
2019-08-28T22:33:41.4097433Z 
2019-08-28T22:33:41.4097555Z error: aborting due to 12 previous errors
2019-08-28T22:33:41.4097593Z 
2019-08-28T22:33:41.4097658Z Some errors have detailed explanations: E0364, E0433, E0603, E0616, E0624.
---
2019-08-28T22:33:41.4099767Z 
2019-08-28T22:33:41.4099995Z ---- [ui] ui/proc-macro/issue-50493.rs stdout ----
2019-08-28T22:33:41.4100057Z diff of stderr:
2019-08-28T22:33:41.4100100Z 
2019-08-28T22:33:41.4100143Z 4 LL |     pub(in restricted) field: usize,
2019-08-28T22:33:41.4100231Z 6 
2019-08-28T22:33:41.4100231Z 6 
2019-08-28T22:33:41.4100489Z - error[E0616]: field `field` of struct `Restricted` is private
2019-08-28T22:33:41.4100543Z + error[E0616]: field `field` of `struct` `Restricted` is private
2019-08-28T22:33:41.4100754Z 8   --> $DIR/issue-50493.rs:6:10
2019-08-28T22:33:41.4100866Z 10 LL | #[derive(Derive)]
2019-08-28T22:33:41.4100895Z 
2019-08-28T22:33:41.4100920Z 
2019-08-28T22:33:41.4100979Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4100979Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4101283Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-50493/issue-50493.stderr
2019-08-28T22:33:41.4101529Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4102007Z To only update this specific test, also pass `--test-args proc-macro/issue-50493.rs`
2019-08-28T22:33:41.4102106Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4102150Z status: exit code: 1
2019-08-28T22:33:41.4102150Z status: exit code: 1
2019-08-28T22:33:41.4102971Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-50493.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-50493" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-50493/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4103298Z ------------------------------------------
2019-08-28T22:33:41.4103332Z 
2019-08-28T22:33:41.4103553Z ------------------------------------------
2019-08-28T22:33:41.4103756Z stderr:
2019-08-28T22:33:41.4103756Z stderr:
2019-08-28T22:33:41.4104011Z ------------------------------------------
2019-08-28T22:33:41.4104064Z error: visibilities can only be restricted to ancestor modules
2019-08-28T22:33:41.4104327Z   --> /checkout/src/test/ui/proc-macro/issue-50493.rs:8:12
2019-08-28T22:33:41.4104379Z    |
2019-08-28T22:33:41.4104430Z LL |     pub(in restricted) field: usize, //~ visibilities can only be restricted to ancestor modules
2019-08-28T22:33:41.4104527Z 
2019-08-28T22:33:41.4104527Z 
2019-08-28T22:33:41.4104580Z error[E0616]: field `field` of `struct` `Restricted` is private
2019-08-28T22:33:41.4104845Z   --> /checkout/src/test/ui/proc-macro/issue-50493.rs:6:10
2019-08-28T22:33:41.4104892Z    |
2019-08-28T22:33:41.4104941Z LL | #[derive(Derive)] //~ ERROR field `field` of struct `Restricted` is private
2019-08-28T22:33:41.4105031Z 
2019-08-28T22:33:41.4105073Z error: aborting due to 2 previous errors
2019-08-28T22:33:41.4105110Z 
2019-08-28T22:33:41.4105353Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4105353Z For more information about this error, try `rustc --explain E0616`.
2019-08-28T22:33:41.4105402Z 
2019-08-28T22:33:41.4105614Z ------------------------------------------
2019-08-28T22:33:41.4105645Z 
2019-08-28T22:33:41.4105670Z 
2019-08-28T22:33:41.4105913Z ---- [ui] ui/structs/struct-field-privacy.rs stdout ----
2019-08-28T22:33:41.4105960Z diff of stderr:
2019-08-28T22:33:41.4105990Z 
2019-08-28T22:33:41.4106221Z - error[E0616]: field `a` of struct `inner::A` is private
2019-08-28T22:33:41.4106379Z + error[E0616]: field `a` of `struct` `inner::A` is private
2019-08-28T22:33:41.4106688Z 3    |
2019-08-28T22:33:41.4106745Z 4 LL |     b.a;
2019-08-28T22:33:41.4106773Z 
2019-08-28T22:33:41.4106812Z 5    |     ^^^
2019-08-28T22:33:41.4106812Z 5    |     ^^^
2019-08-28T22:33:41.4106852Z 6 
2019-08-28T22:33:41.4107102Z - error[E0616]: field `b` of struct `inner::B` is private
2019-08-28T22:33:41.4107154Z + error[E0616]: field `b` of `struct` `inner::B` is private
2019-08-28T22:33:41.4107442Z 9    |
2019-08-28T22:33:41.4107484Z 10 LL |     c.b;
2019-08-28T22:33:41.4107511Z 
2019-08-28T22:33:41.4107551Z 11    |     ^^^
2019-08-28T22:33:41.4107551Z 11    |     ^^^
2019-08-28T22:33:41.4107607Z 12 
2019-08-28T22:33:41.4107837Z - error[E0616]: field `a` of struct `xc::A` is private
2019-08-28T22:33:41.4107888Z + error[E0616]: field `a` of `struct` `xc::A` is private
2019-08-28T22:33:41.4108174Z 15    |
2019-08-28T22:33:41.4108216Z 16 LL |     d.a;
2019-08-28T22:33:41.4108243Z 
2019-08-28T22:33:41.4108297Z 17    |     ^^^
2019-08-28T22:33:41.4108297Z 17    |     ^^^
2019-08-28T22:33:41.4108337Z 18 
2019-08-28T22:33:41.4108570Z - error[E0616]: field `b` of struct `xc::B` is private
2019-08-28T22:33:41.4108636Z + error[E0616]: field `b` of `struct` `xc::B` is private
2019-08-28T22:33:41.4108901Z 21    |
2019-08-28T22:33:41.4108948Z 22 LL |     e.b;
2019-08-28T22:33:41.4108991Z 
2019-08-28T22:33:41.4109030Z 23    |     ^^^
2019-08-28T22:33:41.4109030Z 23    |     ^^^
2019-08-28T22:33:41.4109070Z 24 
2019-08-28T22:33:41.4109599Z - error[E0616]: field `1` of struct `inner::Z` is private
2019-08-28T22:33:41.4109679Z + error[E0616]: field `1` of `struct` `inner::Z` is private
2019-08-28T22:33:41.4109953Z 27    |
2019-08-28T22:33:41.4110010Z 28 LL |     z.1;
2019-08-28T22:33:41.4110038Z 
2019-08-28T22:33:41.4110064Z 
2019-08-28T22:33:41.4110064Z 
2019-08-28T22:33:41.4110119Z The actual stderr differed from the expected stderr.
2019-08-28T22:33:41.4110449Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-field-privacy/struct-field-privacy.stderr
2019-08-28T22:33:41.4110703Z To update references, rerun the tests and pass the `--bless` flag
2019-08-28T22:33:41.4110990Z To only update this specific test, also pass `--test-args structs/struct-field-privacy.rs`
2019-08-28T22:33:41.4111192Z error: 1 errors occurred comparing output.
2019-08-28T22:33:41.4111238Z status: exit code: 1
2019-08-28T22:33:41.4111238Z status: exit code: 1
2019-08-28T22:33:41.4112061Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs/struct-field-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-field-privacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-field-privacy/auxiliary" "-A" "unused"
2019-08-28T22:33:41.4112392Z ------------------------------------------
2019-08-28T22:33:41.4112425Z 
2019-08-28T22:33:41.4112641Z ------------------------------------------
2019-08-28T22:33:41.4112702Z stderr:
2019-08-28T22:33:41.4112702Z stderr:
2019-08-28T22:33:41.4112914Z ------------------------------------------
2019-08-28T22:33:41.4112974Z error[E0616]: field `a` of `struct` `inner::A` is private
2019-08-28T22:33:41.4113284Z    |
2019-08-28T22:33:41.4113284Z    |
2019-08-28T22:33:41.4113331Z LL |     b.a; //~ ERROR: field `a` of struct `inner::A` is private
2019-08-28T22:33:41.4113419Z 
2019-08-28T22:33:41.4113419Z 
2019-08-28T22:33:41.4113463Z error[E0616]: field `b` of `struct` `inner::B` is private
2019-08-28T22:33:41.4113875Z    |
2019-08-28T22:33:41.4113875Z    |
2019-08-28T22:33:41.4113921Z LL |     c.b; //~ ERROR: field `b` of struct `inner::B` is private
2019-08-28T22:33:41.4113994Z 
2019-08-28T22:33:41.4113994Z 
2019-08-28T22:33:41.4114053Z error[E0616]: field `a` of `struct` `xc::A` is private
2019-08-28T22:33:41.4114382Z    |
2019-08-28T22:33:41.4114382Z    |
2019-08-28T22:33:41.4114452Z LL |     d.a; //~ ERROR: field `a` of struct `xc::A` is private
2019-08-28T22:33:41.4114525Z 
2019-08-28T22:33:41.4114525Z 
2019-08-28T22:33:41.4114569Z error[E0616]: field `b` of `struct` `xc::B` is private
2019-08-28T22:33:41.4114876Z    |
2019-08-28T22:33:41.4114876Z    |
2019-08-28T22:33:41.4114923Z LL |     e.b; //~ ERROR: field `b` of struct `xc::B` is private
2019-08-28T22:33:41.4115010Z 
2019-08-28T22:33:41.4115010Z 
2019-08-28T22:33:41.4115063Z error[E0616]: field `1` of `struct` `inner::Z` is private
2019-08-28T22:33:41.4115372Z    |
2019-08-28T22:33:41.4115372Z    |
2019-08-28T22:33:41.4115417Z LL |     z.1; //~ ERROR: field `1` of struct `inner::Z` is private
2019-08-28T22:33:41.4115554Z 
2019-08-28T22:33:41.4115596Z error: aborting due to 5 previous errors
2019-08-28T22:33:41.4115626Z 
2019-08-28T22:33:41.4115881Z For more information about this error, try `rustc --explain E0616`.
---
2019-08-28T22:33:41.4121813Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-28T22:33:41.4121897Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-28T22:33:41.4121931Z 
2019-08-28T22:33:41.4121957Z 
2019-08-28T22:33:41.4123633Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-28T22:33:41.4123908Z 
2019-08-28T22:33:41.4123938Z 
2019-08-28T22:33:41.4124002Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-28T22:33:41.4124055Z Build completed unsuccessfully in 1:06:05
2019-08-28T22:33:41.4124055Z Build completed unsuccessfully in 1:06:05
2019-08-28T22:33:41.4124105Z == clock drift check ==
2019-08-28T22:33:41.4124167Z   local time: Wed Aug 28 22:33:41 UTC 2019
2019-08-28T22:33:41.5593425Z   network time: Wed, 28 Aug 2019 22:33:41 GMT
2019-08-28T22:33:41.5597403Z == end clock drift check ==
2019-08-28T22:33:42.2718756Z ##[error]Bash exited with code '1'.
2019-08-28T22:33:42.2766647Z ##[section]Starting: Checkout
2019-08-28T22:33:42.2768611Z ==============================================================================
2019-08-28T22:33:42.2768693Z Task         : Get sources
2019-08-28T22:33:42.2768741Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
