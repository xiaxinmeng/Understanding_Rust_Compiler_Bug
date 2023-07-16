plain
2020-03-02T18:58:10.4467647Z ========================== Starting Command Output ===========================
2020-03-02T18:58:10.4470249Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/32bc9c8d-67af-4e7f-b7e9-ff83674fc4a1.sh
2020-03-02T18:58:10.4470553Z 
2020-03-02T18:58:10.4473744Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-02T18:58:10.4493707Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69642/merge to s
2020-03-02T18:58:10.4497410Z Task         : Get sources
2020-03-02T18:58:10.4497752Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T18:58:10.4498065Z Version      : 1.0.0
2020-03-02T18:58:10.4498276Z Author       : Microsoft
---
2020-03-02T18:58:11.4409923Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-02T18:58:11.4415513Z ##[command]git config gc.auto 0
2020-03-02T18:58:11.4419256Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-02T18:58:11.4422700Z ##[command]git config --get-all http.proxy
2020-03-02T18:58:11.4428792Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69642/merge:refs/remotes/pull/69642/merge
---
2020-03-02T19:57:41.2209540Z .................................................................................................... 1700/9742
2020-03-02T19:57:45.4669009Z .................................................................................................... 1800/9742
2020-03-02T19:57:56.5184890Z ......................................................................i............................. 1900/9742
2020-03-02T19:58:02.5734409Z .................................................................................................... 2000/9742
2020-03-02T19:58:16.9913641Z ............................................................iiiii................................... 2100/9742
2020-03-02T19:58:26.8835462Z .................................................................................................... 2300/9742
2020-03-02T19:58:29.0091902Z ..........................F......................................................................... 2400/9742
2020-03-02T19:58:31.8398494Z ...........................................................................F........................ 2500/9742
2020-03-02T19:58:51.0927819Z .................................................................................................... 2600/9742
---
2020-03-02T20:01:30.1378517Z .....................i...............i.............................................................. 5000/9742
2020-03-02T20:01:39.5263653Z .................................................................................................... 5100/9742
2020-03-02T20:01:44.8020293Z ................................................................i................................... 5200/9742
2020-03-02T20:01:50.9219066Z .................................................................................................... 5300/9742
2020-03-02T20:01:59.4508039Z ...........................................ii.ii........i...i....................................... 5400/9742
2020-03-02T20:02:07.1824172Z .................................................................................................... 5600/9742
2020-03-02T20:02:16.2609819Z .................................................................................................... 5700/9742
2020-03-02T20:02:22.8571046Z ..................................i................................................................. 5800/9742
2020-03-02T20:02:28.4678641Z .................................................................................................... 5900/9742
2020-03-02T20:02:28.4678641Z .................................................................................................... 5900/9742
2020-03-02T20:02:38.6557038Z .................................................................................................... 6000/9742
2020-03-02T20:02:48.3649190Z ..........................ii...i..ii...........i.................................................... 6100/9742
2020-03-02T20:03:03.5425017Z .................................................................................................... 6300/9742
2020-03-02T20:03:06.9178456Z .................................................................................................... 6400/9742
2020-03-02T20:03:06.9178456Z .................................................................................................... 6400/9742
2020-03-02T20:03:13.0959643Z .........................................................i..ii...................................... 6500/9742
2020-03-02T20:03:38.7713704Z .................................................................................................... 6700/9742
2020-03-02T20:03:40.9944931Z .................................................i.................................................. 6800/9742
2020-03-02T20:03:42.8948389Z .................................................................................................... 6900/9742
2020-03-02T20:03:44.9120812Z ...............................................................................i.................... 7000/9742
---
2020-03-02T20:05:18.6791608Z .................................................................................................... 7700/9742
2020-03-02T20:05:22.8735018Z .................................................................................................... 7800/9742
2020-03-02T20:05:27.9772585Z .................................................................................................... 7900/9742
2020-03-02T20:05:35.2350975Z .........................i...........................................F.............................. 8000/9742
2020-03-02T20:05:43.3543911Z ...........................................................................iiiiiii.i................ 8100/9742
2020-03-02T20:05:58.3229769Z ................i......i............................................................................ 8300/9742
2020-03-02T20:06:03.3133961Z ...F................................................................................................ 8400/9742
2020-03-02T20:06:15.4719245Z .................................................................................................... 8500/9742
2020-03-02T20:06:24.7264379Z .................................................................................................... 8600/9742
---
2020-03-02T20:08:14.9028906Z 64 error[E0221]: ambiguous associated type `Color` in bounds of `X`
2020-03-02T20:08:14.9030046Z -   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:35:20
2020-03-02T20:08:14.9031189Z +   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:38:20
2020-03-02T20:08:14.9031904Z 66    |
2020-03-02T20:08:14.9032213Z 67 LL |     type Color;
2020-03-02T20:08:14.9033057Z 68    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9033947Z 70 LL |     type Color;
2020-03-02T20:08:14.9034797Z 71    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9035410Z 72 ...
2020-03-02T20:08:14.9035410Z 72 ...
2020-03-02T20:08:14.9036115Z - LL |     fn e(&self, _: X::Color) where X : Box;
2020-03-02T20:08:14.9036757Z + LL |     fn f(&self, _: X::Color) where X : Box { }
2020-03-02T20:08:14.9037886Z 75    |
2020-03-02T20:08:14.9038273Z 76 help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9038814Z 
2020-03-02T20:08:14.9039078Z 77    |
2020-03-02T20:08:14.9039078Z 77    |
2020-03-02T20:08:14.9039840Z - LL |     fn e(&self, _: <X as Box>::Color) where X : Box;
2020-03-02T20:08:14.9040527Z + LL |     fn f(&self, _: <X as Box>::Color) where X : Box { }
2020-03-02T20:08:14.9044402Z 80 help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9045009Z 81    |
2020-03-02T20:08:14.9045262Z 
2020-03-02T20:08:14.9045262Z 
2020-03-02T20:08:14.9046143Z - LL |     fn e(&self, _: <X as Vehicle>::Color) where X : Box;
2020-03-02T20:08:14.9078925Z + LL |     fn f(&self, _: <X as Vehicle>::Color) where X : Box { }
2020-03-02T20:08:14.9080355Z 84 
2020-03-02T20:08:14.9080800Z 85 error[E0221]: ambiguous associated type `Color` in bounds of `X`
2020-03-02T20:08:14.9081376Z 
2020-03-02T20:08:14.9082454Z -   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:38:20
2020-03-02T20:08:14.9082454Z -   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:38:20
2020-03-02T20:08:14.9083793Z +   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:30:20
2020-03-02T20:08:14.9084453Z 87    |
2020-03-02T20:08:14.9084782Z 88 LL |     type Color;
2020-03-02T20:08:14.9086060Z 89    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9096315Z 91 LL |     type Color;
2020-03-02T20:08:14.9097039Z 92    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9097279Z 93 ...
2020-03-02T20:08:14.9097279Z 93 ...
2020-03-02T20:08:14.9097765Z - LL |     fn f(&self, _: X::Color) where X : Box { }
2020-03-02T20:08:14.9098144Z + LL |     fn d(&self, _: X::Color) where X : Box { }
2020-03-02T20:08:14.9098811Z 96    |
2020-03-02T20:08:14.9099263Z 97 help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9099477Z 
2020-03-02T20:08:14.9099627Z 98    |
2020-03-02T20:08:14.9099627Z 98    |
2020-03-02T20:08:14.9100140Z - LL |     fn f(&self, _: <X as Box>::Color) where X : Box { }
2020-03-02T20:08:14.9100555Z + LL |     fn d(&self, _: <X as Box>::Color) where X : Box { }
2020-03-02T20:08:14.9101341Z 101 help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9101579Z 102    |
2020-03-02T20:08:14.9101699Z 
2020-03-02T20:08:14.9101699Z 
2020-03-02T20:08:14.9102212Z - LL |     fn f(&self, _: <X as Vehicle>::Color) where X : Box { }
2020-03-02T20:08:14.9102647Z + LL |     fn d(&self, _: <X as Vehicle>::Color) where X : Box { }
2020-03-02T20:08:14.9103247Z 105 
2020-03-02T20:08:14.9103517Z 106 error[E0221]: ambiguous associated type `Color` in bounds of `X`
2020-03-02T20:08:14.9103754Z 
2020-03-02T20:08:14.9104302Z -   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:30:20
2020-03-02T20:08:14.9104302Z -   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:30:20
2020-03-02T20:08:14.9105043Z +   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:35:20
2020-03-02T20:08:14.9105372Z 108    |
2020-03-02T20:08:14.9105556Z 109 LL |     type Color;
2020-03-02T20:08:14.9106054Z 110    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9106437Z 112 LL |     type Color;
2020-03-02T20:08:14.9106921Z 113    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9107158Z 114 ...
2020-03-02T20:08:14.9107158Z 114 ...
2020-03-02T20:08:14.9107610Z - LL |     fn d(&self, _: X::Color) where X : Box { }
2020-03-02T20:08:14.9107992Z + LL |     fn e(&self, _: X::Color) where X : Box;
2020-03-02T20:08:14.9108630Z 117    |
2020-03-02T20:08:14.9108872Z 118 help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9109090Z 
2020-03-02T20:08:14.9109227Z 119    |
2020-03-02T20:08:14.9109227Z 119    |
2020-03-02T20:08:14.9109718Z - LL |     fn d(&self, _: <X as Box>::Color) where X : Box { }
2020-03-02T20:08:14.9110148Z + LL |     fn e(&self, _: <X as Box>::Color) where X : Box;
2020-03-02T20:08:14.9110804Z 122 help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9111058Z 123    |
2020-03-02T20:08:14.9111177Z 
2020-03-02T20:08:14.9111177Z 
2020-03-02T20:08:14.9112427Z - LL |     fn d(&self, _: <X as Vehicle>::Color) where X : Box { }
2020-03-02T20:08:14.9112880Z + LL |     fn e(&self, _: <X as Vehicle>::Color) where X : Box;
2020-03-02T20:08:14.9113718Z 126 
2020-03-02T20:08:14.9114096Z 127 error: aborting due to 6 previous errors
2020-03-02T20:08:14.9114308Z 
2020-03-02T20:08:14.9114412Z 
2020-03-02T20:08:14.9114412Z 
2020-03-02T20:08:14.9114755Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9115951Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause/associated-type-projection-ambig-between-bound-and-where-clause.stderr
2020-03-02T20:08:14.9117321Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9118310Z To only update this specific test, also pass `--test-args associated-type/associated-type-projection-ambig-between-bound-and-where-clause.rs`
2020-03-02T20:08:14.9118912Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9119163Z status: exit code: 1
2020-03-02T20:08:14.9119163Z status: exit code: 1
2020-03-02T20:08:14.9121786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause/auxiliary"
2020-03-02T20:08:14.9123852Z ------------------------------------------
2020-03-02T20:08:14.9124042Z 
2020-03-02T20:08:14.9124424Z ------------------------------------------
2020-03-02T20:08:14.9124653Z stderr:
2020-03-02T20:08:14.9124653Z stderr:
2020-03-02T20:08:14.9125045Z ------------------------------------------
2020-03-02T20:08:14.9125372Z error[E0221]: ambiguous associated type `Color` in bounds of `C`
2020-03-02T20:08:14.9126142Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause.rs:16:24
2020-03-02T20:08:14.9126523Z    |
2020-03-02T20:08:14.9126693Z LL |     type Color;
2020-03-02T20:08:14.9127163Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9127559Z LL |     type Color;
2020-03-02T20:08:14.9128013Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9128235Z ...
2020-03-02T20:08:14.9128235Z ...
2020-03-02T20:08:14.9128457Z LL | fn a<C:Vehicle+Box>(_: C::Color) {
2020-03-02T20:08:14.9129080Z    |
2020-03-02T20:08:14.9129303Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9129537Z    |
2020-03-02T20:08:14.9129537Z    |
2020-03-02T20:08:14.9129782Z LL | fn a<C:Vehicle+Box>(_: <C as Box>::Color) {
2020-03-02T20:08:14.9130405Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9130623Z    |
2020-03-02T20:08:14.9130623Z    |
2020-03-02T20:08:14.9130874Z LL | fn a<C:Vehicle+Box>(_: <C as Vehicle>::Color) {
2020-03-02T20:08:14.9131415Z 
2020-03-02T20:08:14.9131669Z error[E0221]: ambiguous associated type `Color` in bounds of `C`
2020-03-02T20:08:14.9132425Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause.rs:20:12
2020-03-02T20:08:14.9132819Z    |
2020-03-02T20:08:14.9132819Z    |
2020-03-02T20:08:14.9132994Z LL |     type Color;
2020-03-02T20:08:14.9133444Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9133853Z LL |     type Color;
2020-03-02T20:08:14.9134290Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9134522Z ...
2020-03-02T20:08:14.9134522Z ...
2020-03-02T20:08:14.9134766Z LL | fn b<C>(_: C::Color) where C : Vehicle+Box {
2020-03-02T20:08:14.9135485Z    |
2020-03-02T20:08:14.9135723Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9135941Z    |
2020-03-02T20:08:14.9135941Z    |
2020-03-02T20:08:14.9136203Z LL | fn b<C>(_: <C as Box>::Color) where C : Vehicle+Box {
2020-03-02T20:08:14.9136812Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9137030Z    |
2020-03-02T20:08:14.9137030Z    |
2020-03-02T20:08:14.9137314Z LL | fn b<C>(_: <C as Vehicle>::Color) where C : Vehicle+Box {
2020-03-02T20:08:14.9137828Z 
2020-03-02T20:08:14.9138080Z error[E0221]: ambiguous associated type `Color` in bounds of `C`
2020-03-02T20:08:14.9138860Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause.rs:24:12
2020-03-02T20:08:14.9139237Z    |
2020-03-02T20:08:14.9139237Z    |
2020-03-02T20:08:14.9139403Z LL |     type Color;
2020-03-02T20:08:14.9139868Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9140256Z LL |     type Color;
2020-03-02T20:08:14.9140707Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9140925Z ...
2020-03-02T20:08:14.9140925Z ...
2020-03-02T20:08:14.9141182Z LL | fn c<C>(_: C::Color) where C : Vehicle, C : Box {
2020-03-02T20:08:14.9141856Z    |
2020-03-02T20:08:14.9142078Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9142313Z    |
2020-03-02T20:08:14.9142313Z    |
2020-03-02T20:08:14.9142597Z LL | fn c<C>(_: <C as Box>::Color) where C : Vehicle, C : Box {
2020-03-02T20:08:14.9143273Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9143491Z    |
2020-03-02T20:08:14.9143491Z    |
2020-03-02T20:08:14.9143779Z LL | fn c<C>(_: <C as Vehicle>::Color) where C : Vehicle, C : Box {
2020-03-02T20:08:14.9144311Z 
2020-03-02T20:08:14.9144557Z error[E0221]: ambiguous associated type `Color` in bounds of `X`
2020-03-02T20:08:14.9145318Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause.rs:38:20
2020-03-02T20:08:14.9145708Z    |
2020-03-02T20:08:14.9145708Z    |
2020-03-02T20:08:14.9145875Z LL |     type Color;
2020-03-02T20:08:14.9146329Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9146738Z LL |     type Color;
2020-03-02T20:08:14.9147175Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9147409Z ...
2020-03-02T20:08:14.9147409Z ...
2020-03-02T20:08:14.9147664Z LL |     fn f(&self, _: X::Color) where X : Box { }
2020-03-02T20:08:14.9148294Z    |
2020-03-02T20:08:14.9148517Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9148740Z    |
2020-03-02T20:08:14.9148740Z    |
2020-03-02T20:08:14.9149013Z LL |     fn f(&self, _: <X as Box>::Color) where X : Box { }
2020-03-02T20:08:14.9149654Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9149872Z    |
2020-03-02T20:08:14.9149872Z    |
2020-03-02T20:08:14.9150168Z LL |     fn f(&self, _: <X as Vehicle>::Color) where X : Box { }
2020-03-02T20:08:14.9150712Z 
2020-03-02T20:08:14.9150977Z error[E0221]: ambiguous associated type `Color` in bounds of `X`
2020-03-02T20:08:14.9151729Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause.rs:30:20
2020-03-02T20:08:14.9152115Z    |
2020-03-02T20:08:14.9152115Z    |
2020-03-02T20:08:14.9152296Z LL |     type Color;
2020-03-02T20:08:14.9160774Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9161215Z LL |     type Color;
2020-03-02T20:08:14.9161661Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9161883Z ...
2020-03-02T20:08:14.9161883Z ...
2020-03-02T20:08:14.9162133Z LL |     fn d(&self, _: X::Color) where X : Box { }
2020-03-02T20:08:14.9162767Z    |
2020-03-02T20:08:14.9162993Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9163228Z    |
2020-03-02T20:08:14.9163228Z    |
2020-03-02T20:08:14.9163518Z LL |     fn d(&self, _: <X as Box>::Color) where X : Box { }
2020-03-02T20:08:14.9164168Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9164387Z    |
2020-03-02T20:08:14.9164387Z    |
2020-03-02T20:08:14.9164668Z LL |     fn d(&self, _: <X as Vehicle>::Color) where X : Box { }
2020-03-02T20:08:14.9165231Z 
2020-03-02T20:08:14.9165481Z error[E0221]: ambiguous associated type `Color` in bounds of `X`
2020-03-02T20:08:14.9166241Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-ambig-between-bound-and-where-clause.rs:35:20
2020-03-02T20:08:14.9166630Z    |
2020-03-02T20:08:14.9166630Z    |
2020-03-02T20:08:14.9166799Z LL |     type Color;
2020-03-02T20:08:14.9167255Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9167664Z LL |     type Color;
2020-03-02T20:08:14.9168108Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9168342Z ...
2020-03-02T20:08:14.9168342Z ...
2020-03-02T20:08:14.9168740Z LL |     fn e(&self, _: X::Color) where X : Box;
2020-03-02T20:08:14.9169370Z    |
2020-03-02T20:08:14.9169595Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9169884Z    |
2020-03-02T20:08:14.9169884Z    |
2020-03-02T20:08:14.9170164Z LL |     fn e(&self, _: <X as Box>::Color) where X : Box;
2020-03-02T20:08:14.9170791Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9171022Z    |
2020-03-02T20:08:14.9171022Z    |
2020-03-02T20:08:14.9171298Z LL |     fn e(&self, _: <X as Vehicle>::Color) where X : Box;
2020-03-02T20:08:14.9171833Z 
2020-03-02T20:08:14.9172046Z error: aborting due to 6 previous errors
2020-03-02T20:08:14.9172230Z 
2020-03-02T20:08:14.9172700Z For more information about this error, try `rustc --explain E0221`.
---
2020-03-02T20:08:14.9174774Z 94    |
2020-03-02T20:08:14.9175684Z 95    = help: consider introducing a new type parameter, adding `where` constraints using the fully-qualified path to the associated types
2020-03-02T20:08:14.9176078Z 96 
2020-03-02T20:08:14.9176487Z - error: aborting due to 6 previous errors
2020-03-02T20:08:14.9177159Z + error[E0277]: the size for values of type `(dyn BoxCar + 'static)` cannot be known at compilation time
2020-03-02T20:08:14.9177868Z +   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:23:23
2020-03-02T20:08:14.9178170Z +    |
2020-03-02T20:08:14.9178450Z + LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
2020-03-02T20:08:14.9179048Z +    |                       ^ doesn't have a size known at compile-time
2020-03-02T20:08:14.9179316Z +    |
2020-03-02T20:08:14.9179867Z +    = help: the trait `std::marker::Sized` is not implemented for `(dyn BoxCar + 'static)`
2020-03-02T20:08:14.9181292Z +    = note: all local variables must have a statically known size
2020-03-02T20:08:14.9181886Z +    = help: unsized locals are gated as an unstable feature
2020-03-02T20:08:14.9182132Z 98 
2020-03-02T20:08:14.9182612Z - Some errors have detailed explanations: E0191, E0221, E0222.
2020-03-02T20:08:14.9182612Z - Some errors have detailed explanations: E0191, E0221, E0222.
2020-03-02T20:08:14.9183784Z + error[E0277]: the size for values of type `(dyn BoxCar + 'static)` cannot be known at compilation time
2020-03-02T20:08:14.9184837Z +   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:32:25
2020-03-02T20:08:14.9185142Z +    |
2020-03-02T20:08:14.9185522Z + LL | fn dent_object_2<COLOR>(c: dyn BoxCar) where <dyn BoxCar as Vehicle>::Color = COLOR {
2020-03-02T20:08:14.9186205Z +    |                         ^ doesn't have a size known at compile-time
2020-03-02T20:08:14.9186477Z +    |
2020-03-02T20:08:14.9187038Z +    = help: the trait `std::marker::Sized` is not implemented for `(dyn BoxCar + 'static)`
2020-03-02T20:08:14.9188448Z +    = note: all local variables must have a statically known size
2020-03-02T20:08:14.9188834Z +    = help: unsized locals are gated as an unstable feature
2020-03-02T20:08:14.9189077Z + 
2020-03-02T20:08:14.9189287Z + error: aborting due to 8 previous errors
---
2020-03-02T20:08:14.9190932Z 
2020-03-02T20:08:14.9191035Z 
2020-03-02T20:08:14.9191254Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9192215Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-multiple-supertraits/associated-type-projection-from-multiple-supertraits.stderr
2020-03-02T20:08:14.9193125Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9193874Z To only update this specific test, also pass `--test-args associated-type/associated-type-projection-from-multiple-supertraits.rs`
2020-03-02T20:08:14.9194441Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9194694Z status: exit code: 1
2020-03-02T20:08:14.9194694Z status: exit code: 1
2020-03-02T20:08:14.9197110Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-multiple-supertraits" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-multiple-supertraits/auxiliary"
2020-03-02T20:08:14.9199054Z ------------------------------------------
2020-03-02T20:08:14.9199239Z 
2020-03-02T20:08:14.9199621Z ------------------------------------------
2020-03-02T20:08:14.9199849Z stderr:
2020-03-02T20:08:14.9199849Z stderr:
2020-03-02T20:08:14.9200240Z ------------------------------------------
2020-03-02T20:08:14.9200570Z error: equality constraints are not yet supported in `where` clauses
2020-03-02T20:08:14.9201308Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs:32:46
2020-03-02T20:08:14.9201664Z    |
2020-03-02T20:08:14.9202021Z LL | fn dent_object_2<COLOR>(c: dyn BoxCar) where <dyn BoxCar as Vehicle>::Color = COLOR {
2020-03-02T20:08:14.9202858Z    |
2020-03-02T20:08:14.9202858Z    |
2020-03-02T20:08:14.9203554Z    = note: see issue #20041 <***/issues/20041> for more information
2020-03-02T20:08:14.9204041Z error[E0221]: ambiguous associated type `Color` in bounds of `C`
2020-03-02T20:08:14.9204772Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs:19:32
2020-03-02T20:08:14.9205122Z    |
2020-03-02T20:08:14.9205289Z LL |     type Color;
2020-03-02T20:08:14.9205289Z LL |     type Color;
2020-03-02T20:08:14.9205753Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9206151Z LL |     type Color;
2020-03-02T20:08:14.9206594Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9206830Z ...
2020-03-02T20:08:14.9206830Z ...
2020-03-02T20:08:14.9207071Z LL | fn dent<C:BoxCar>(c: C, color: C::Color) {
2020-03-02T20:08:14.9207733Z    |
2020-03-02T20:08:14.9207955Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9208174Z    |
2020-03-02T20:08:14.9208174Z    |
2020-03-02T20:08:14.9208454Z LL | fn dent<C:BoxCar>(c: C, color: <C as Box>::Color) {
2020-03-02T20:08:14.9209385Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9209621Z    |
2020-03-02T20:08:14.9209621Z    |
2020-03-02T20:08:14.9209892Z LL | fn dent<C:BoxCar>(c: C, color: <C as Vehicle>::Color) {
2020-03-02T20:08:14.9210480Z 
2020-03-02T20:08:14.9210480Z 
2020-03-02T20:08:14.9210737Z error[E0222]: ambiguous associated type `Color` in bounds of `BoxCar`
2020-03-02T20:08:14.9211945Z    |
2020-03-02T20:08:14.9212113Z LL |     type Color;
2020-03-02T20:08:14.9212113Z LL |     type Color;
2020-03-02T20:08:14.9212575Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9213043Z LL |     type Color;
2020-03-02T20:08:14.9213490Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9213710Z ...
2020-03-02T20:08:14.9213710Z ...
2020-03-02T20:08:14.9213977Z LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
2020-03-02T20:08:14.9214670Z    |
2020-03-02T20:08:14.9214984Z    = help: consider introducing a new type parameter `T` and adding `where` constraints:
2020-03-02T20:08:14.9216882Z                where
2020-03-02T20:08:14.9216882Z                where
2020-03-02T20:08:14.9217114Z                    T: BoxCar,
2020-03-02T20:08:14.9217423Z                    T: Box::Color = COLOR,
2020-03-02T20:08:14.9217738Z                    T: Vehicle::Color = COLOR
2020-03-02T20:08:14.9217942Z 
2020-03-02T20:08:14.9218306Z error[E0191]: the value of the associated types `Color` (from trait `Box`), `Color` (from trait `Vehicle`) must be specified
2020-03-02T20:08:14.9219613Z    |
2020-03-02T20:08:14.9219784Z LL |     type Color;
2020-03-02T20:08:14.9219784Z LL |     type Color;
2020-03-02T20:08:14.9220262Z    |     ----------- `Vehicle::Color` defined here
2020-03-02T20:08:14.9220668Z LL |     type Color;
2020-03-02T20:08:14.9221125Z    |     ----------- `Box::Color` defined here
2020-03-02T20:08:14.9221350Z ...
2020-03-02T20:08:14.9221350Z ...
2020-03-02T20:08:14.9221602Z LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
2020-03-02T20:08:14.9222136Z    |                              ^^^^^^^^^^^^^^^^^^^ associated types `Color` (from trait `Vehicle`), `Color` (from trait `Box`) must be specified
2020-03-02T20:08:14.9225955Z    = help: consider introducing a new type parameter, adding `where` constraints using the fully-qualified path to the associated types
2020-03-02T20:08:14.9226350Z 
2020-03-02T20:08:14.9226603Z error[E0221]: ambiguous associated type `Color` in bounds of `C`
2020-03-02T20:08:14.9227338Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs:28:29
2020-03-02T20:08:14.9227338Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs:28:29
2020-03-02T20:08:14.9227705Z    |
2020-03-02T20:08:14.9227875Z LL |     type Color;
2020-03-02T20:08:14.9228331Z    |     ----------- ambiguous `Color` from `Vehicle`
2020-03-02T20:08:14.9228740Z LL |     type Color;
2020-03-02T20:08:14.9229180Z    |     ----------- ambiguous `Color` from `Box`
2020-03-02T20:08:14.9229696Z ...
2020-03-02T20:08:14.9229696Z ...
2020-03-02T20:08:14.9229934Z LL | fn paint<C:BoxCar>(c: C, d: C::Color) {
2020-03-02T20:08:14.9230579Z    |
2020-03-02T20:08:14.9230810Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9231035Z    |
2020-03-02T20:08:14.9231035Z    |
2020-03-02T20:08:14.9231294Z LL | fn paint<C:BoxCar>(c: C, d: <C as Box>::Color) {
2020-03-02T20:08:14.9231954Z help: use fully qualified syntax to disambiguate
2020-03-02T20:08:14.9232174Z    |
2020-03-02T20:08:14.9232174Z    |
2020-03-02T20:08:14.9232452Z LL | fn paint<C:BoxCar>(c: C, d: <C as Vehicle>::Color) {
2020-03-02T20:08:14.9233007Z 
2020-03-02T20:08:14.9233007Z 
2020-03-02T20:08:14.9233387Z error[E0191]: the value of the associated types `Color` (from trait `Box`), `Color` (from trait `Vehicle`) must be specified
2020-03-02T20:08:14.9234728Z    |
2020-03-02T20:08:14.9234911Z LL |     type Color;
2020-03-02T20:08:14.9234911Z LL |     type Color;
2020-03-02T20:08:14.9235532Z    |     ----------- `Vehicle::Color` defined here
2020-03-02T20:08:14.9235954Z LL |     type Color;
2020-03-02T20:08:14.9236409Z    |     ----------- `Box::Color` defined here
2020-03-02T20:08:14.9236635Z ...
2020-03-02T20:08:14.9236635Z ...
2020-03-02T20:08:14.9236977Z LL | fn dent_object_2<COLOR>(c: dyn BoxCar) where <dyn BoxCar as Vehicle>::Color = COLOR {
2020-03-02T20:08:14.9237648Z    |                                ^^^^^^ associated types `Color` (from trait `Vehicle`), `Color` (from trait `Box`) must be specified
2020-03-02T20:08:14.9238711Z    = help: consider introducing a new type parameter, adding `where` constraints using the fully-qualified path to the associated types
2020-03-02T20:08:14.9239069Z 
2020-03-02T20:08:14.9239069Z 
2020-03-02T20:08:14.9239631Z error[E0277]: the size for values of type `(dyn BoxCar + 'static)` cannot be known at compilation time
2020-03-02T20:08:14.9240791Z    |
2020-03-02T20:08:14.9240791Z    |
2020-03-02T20:08:14.9241049Z LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
2020-03-02T20:08:14.9241629Z    |                       ^ doesn't have a size known at compile-time
2020-03-02T20:08:14.9241906Z    |
2020-03-02T20:08:14.9242450Z    = help: the trait `std::marker::Sized` is not implemented for `(dyn BoxCar + 'static)`
2020-03-02T20:08:14.9243852Z    = note: all local variables must have a statically known size
2020-03-02T20:08:14.9250630Z    = help: unsized locals are gated as an unstable feature
2020-03-02T20:08:14.9250863Z 
2020-03-02T20:08:14.9250863Z 
2020-03-02T20:08:14.9252312Z error[E0277]: the size for values of type `(dyn BoxCar + 'static)` cannot be known at compilation time
2020-03-02T20:08:14.9253474Z    |
2020-03-02T20:08:14.9253474Z    |
2020-03-02T20:08:14.9255324Z LL | fn dent_object_2<COLOR>(c: dyn BoxCar) where <dyn BoxCar as Vehicle>::Color = COLOR {
2020-03-02T20:08:14.9256102Z    |                         ^ doesn't have a size known at compile-time
2020-03-02T20:08:14.9256369Z    |
2020-03-02T20:08:14.9256939Z    = help: the trait `std::marker::Sized` is not implemented for `(dyn BoxCar + 'static)`
2020-03-02T20:08:14.9258320Z    = note: all local variables must have a statically known size
2020-03-02T20:08:14.9258697Z    = help: unsized locals are gated as an unstable feature
2020-03-02T20:08:14.9258918Z 
2020-03-02T20:08:14.9259119Z error: aborting due to 8 previous errors
---
2020-03-02T20:08:14.9262218Z 
2020-03-02T20:08:14.9262773Z 1 error[E0212]: cannot extract an associated type from a higher-ranked trait bound in this context
2020-03-02T20:08:14.9263463Z -   --> $DIR/associated-types-project-from-hrtb-in-trait-method.rs:13:32
2020-03-02T20:08:14.9263922Z -    |
2020-03-02T20:08:14.9264346Z - LL |     fn some_method(&self, arg: I::A);
2020-03-02T20:08:14.9265241Z -    |                                ^^^^ help: use a fully qualified path with inferred lifetimes: `<I as Foo<&isize>>::A`
2020-03-02T20:08:14.9266620Z - error[E0212]: cannot extract an associated type from a higher-ranked trait bound in this context
2020-03-02T20:08:14.9267330Z 8   --> $DIR/associated-types-project-from-hrtb-in-trait-method.rs:32:24
2020-03-02T20:08:14.9267644Z 9    |
2020-03-02T20:08:14.9267644Z 9    |
2020-03-02T20:08:14.9268057Z 10 LL |     fn mango(&self) -> X::Assoc {
2020-03-02T20:08:14.9268260Z 
2020-03-02T20:08:14.9269035Z 11    |                        ^^^^^^^^ help: use a fully qualified path with inferred lifetimes: `<X as Banana<'_>>::Assoc`
2020-03-02T20:08:14.9270000Z + error[E0212]: cannot extract an associated type from a higher-ranked trait bound in this context
2020-03-02T20:08:14.9270700Z +   --> $DIR/associated-types-project-from-hrtb-in-trait-method.rs:13:32
2020-03-02T20:08:14.9270993Z +    |
2020-03-02T20:08:14.9270993Z +    |
2020-03-02T20:08:14.9271231Z + LL |     fn some_method(&self, arg: I::A);
2020-03-02T20:08:14.9271766Z +    |                                ^^^^ help: use a fully qualified path with inferred lifetimes: `<I as Foo<&isize>>::A`
2020-03-02T20:08:14.9272397Z 13 error: aborting due to 2 previous errors
2020-03-02T20:08:14.9272625Z 14 
2020-03-02T20:08:14.9272735Z 
2020-03-02T20:08:14.9272839Z 
2020-03-02T20:08:14.9272839Z 
2020-03-02T20:08:14.9273057Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9273998Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-trait-method/associated-types-project-from-hrtb-in-trait-method.stderr
2020-03-02T20:08:14.9274827Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9275579Z To only update this specific test, also pass `--test-args associated-types/associated-types-project-from-hrtb-in-trait-method.rs`
2020-03-02T20:08:14.9276155Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9276406Z status: exit code: 1
2020-03-02T20:08:14.9276406Z status: exit code: 1
2020-03-02T20:08:14.9278978Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-trait-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-trait-method" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-trait-method/auxiliary"
2020-03-02T20:08:14.9280931Z ------------------------------------------
2020-03-02T20:08:14.9281118Z 
2020-03-02T20:08:14.9281514Z ------------------------------------------
2020-03-02T20:08:14.9281729Z stderr:
2020-03-02T20:08:14.9281729Z stderr:
2020-03-02T20:08:14.9282248Z ------------------------------------------
2020-03-02T20:08:14.9282874Z error[E0212]: cannot extract an associated type from a higher-ranked trait bound in this context
2020-03-02T20:08:14.9283655Z   --> /checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-trait-method.rs:32:24
2020-03-02T20:08:14.9284006Z    |
2020-03-02T20:08:14.9284413Z LL |     fn mango(&self) -> X::Assoc {
2020-03-02T20:08:14.9285168Z    |                        ^^^^^^^^ help: use a fully qualified path with inferred lifetimes: `<X as Banana<'_>>::Assoc`
2020-03-02T20:08:14.9286530Z error[E0212]: cannot extract an associated type from a higher-ranked trait bound in this context
2020-03-02T20:08:14.9287300Z   --> /checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-trait-method.rs:13:32
2020-03-02T20:08:14.9287655Z    |
2020-03-02T20:08:14.9287655Z    |
2020-03-02T20:08:14.9287898Z LL |     fn some_method(&self, arg: I::A);
2020-03-02T20:08:14.9288438Z    |                                ^^^^ help: use a fully qualified path with inferred lifetimes: `<I as Foo<&isize>>::A`
2020-03-02T20:08:14.9289124Z error: aborting due to 2 previous errors
2020-03-02T20:08:14.9289455Z 
2020-03-02T20:08:14.9289559Z 
2020-03-02T20:08:14.9289965Z ------------------------------------------
---
2020-03-02T20:08:14.9291262Z 
2020-03-02T20:08:14.9291901Z 1 error[E0582]: binding for associated type `Item` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9292791Z +   --> $DIR/bound-lifetime-in-binding-only.rs:27:31
2020-03-02T20:08:14.9293048Z +    |
2020-03-02T20:08:14.9293509Z + LL | fn angle3(_: &dyn for<'a> Foo<Item=&'a i32>) {
2020-03-02T20:08:14.9294054Z + 
2020-03-02T20:08:14.9294689Z + error[E0582]: binding for associated type `Item` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9295976Z 2   --> $DIR/bound-lifetime-in-binding-only.rs:12:25
2020-03-02T20:08:14.9296231Z 3    |
2020-03-02T20:08:14.9296231Z 3    |
2020-03-02T20:08:14.9296679Z 4 LL | fn angle<T: for<'a> Foo<Item=&'a i32>>() {
2020-03-02T20:08:14.9297025Z 15    |
2020-03-02T20:08:14.9297025Z 15    |
2020-03-02T20:08:14.9297503Z 16 LL | fn angle2<T>() where for<'a> T: Foo<Item=&'a i32> {
2020-03-02T20:08:14.9298247Z - 
2020-03-02T20:08:14.9298870Z - error[E0582]: binding for associated type `Item` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9299559Z -   --> $DIR/bound-lifetime-in-binding-only.rs:27:31
2020-03-02T20:08:14.9299954Z -    |
2020-03-02T20:08:14.9299954Z -    |
2020-03-02T20:08:14.9300521Z - LL | fn angle3(_: &dyn for<'a> Foo<Item=&'a i32>) {
2020-03-02T20:08:14.9301270Z 24 
2020-03-02T20:08:14.9301485Z 25 error: aborting due to 4 previous errors
2020-03-02T20:08:14.9301710Z 26 
2020-03-02T20:08:14.9301825Z 
2020-03-02T20:08:14.9301825Z 
2020-03-02T20:08:14.9301930Z 
2020-03-02T20:08:14.9302150Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9303006Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.angle/bound-lifetime-in-binding-only.angle.stderr
2020-03-02T20:08:14.9303899Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9304616Z To only update this specific test, also pass `--test-args associated-types/bound-lifetime-in-binding-only.rs`
2020-03-02T20:08:14.9304925Z 
2020-03-02T20:08:14.9305177Z error in revision `angle`: 1 errors occurred comparing output.
2020-03-02T20:08:14.9305486Z status: exit code: 1
2020-03-02T20:08:14.9307802Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "angle" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.angle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.angle/auxiliary"
2020-03-02T20:08:14.9309812Z ------------------------------------------
2020-03-02T20:08:14.9310000Z 
2020-03-02T20:08:14.9310396Z ------------------------------------------
2020-03-02T20:08:14.9310611Z stderr:
2020-03-02T20:08:14.9310611Z stderr:
2020-03-02T20:08:14.9311134Z ------------------------------------------
2020-03-02T20:08:14.9311838Z error[E0582]: binding for associated type `Item` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9312684Z   --> /checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs:27:31
2020-03-02T20:08:14.9313000Z    |
2020-03-02T20:08:14.9313585Z LL | fn angle3(_: &dyn for<'a> Foo<Item=&'a i32>) {
2020-03-02T20:08:14.9314099Z 
2020-03-02T20:08:14.9314804Z error[E0582]: binding for associated type `Item` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9315554Z   --> /checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs:12:25
2020-03-02T20:08:14.9315861Z    |
2020-03-02T20:08:14.9315861Z    |
2020-03-02T20:08:14.9316288Z LL | fn angle<T: for<'a> Foo<Item=&'a i32>>() {
2020-03-02T20:08:14.9316765Z 
2020-03-02T20:08:14.9317373Z error[E0582]: binding for associated type `Item` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9318133Z   --> /checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs:17:37
2020-03-02T20:08:14.9318443Z    |
2020-03-02T20:08:14.9318443Z    |
2020-03-02T20:08:14.9319005Z LL | fn angle1<T>() where T: for<'a> Foo<Item=&'a i32> {
2020-03-02T20:08:14.9319571Z 
2020-03-02T20:08:14.9320614Z error[E0582]: binding for associated type `Item` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9321388Z   --> /checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs:22:37
2020-03-02T20:08:14.9321694Z    |
2020-03-02T20:08:14.9321694Z    |
2020-03-02T20:08:14.9322145Z LL | fn angle2<T>() where for<'a> T: Foo<Item=&'a i32> {
2020-03-02T20:08:14.9322710Z 
2020-03-02T20:08:14.9323006Z error: aborting due to 4 previous errors
2020-03-02T20:08:14.9323185Z 
2020-03-02T20:08:14.9323663Z For more information about this error, try `rustc --explain E0582`.
2020-03-02T20:08:14.9323663Z For more information about this error, try `rustc --explain E0582`.
2020-03-02T20:08:14.9323894Z 
2020-03-02T20:08:14.9324271Z ------------------------------------------
2020-03-02T20:08:14.9324460Z 
2020-03-02T20:08:14.9324564Z 
2020-03-02T20:08:14.9325069Z ---- [ui] ui/associated-types/bound-lifetime-in-binding-only.rs#paren stdout ----
2020-03-02T20:08:14.9325369Z diff of stderr:
2020-03-02T20:08:14.9325499Z 
2020-03-02T20:08:14.9326137Z 1 error[E0582]: binding for associated type `Output` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9326820Z +   --> $DIR/bound-lifetime-in-binding-only.rs:47:35
2020-03-02T20:08:14.9327516Z + LL | fn paren3(_: &dyn for<'a> Fn() -> &'a i32) {
2020-03-02T20:08:14.9327840Z +    |                                   ^^^^^^^
2020-03-02T20:08:14.9328052Z + 
2020-03-02T20:08:14.9328052Z + 
2020-03-02T20:08:14.9328692Z + error[E0582]: binding for associated type `Output` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9329607Z 3    |
2020-03-02T20:08:14.9329607Z 3    |
2020-03-02T20:08:14.9330045Z 4 LL | fn paren<T: for<'a> Fn() -> &'a i32>() {
2020-03-02T20:08:14.9330386Z 15    |
2020-03-02T20:08:14.9330386Z 15    |
2020-03-02T20:08:14.9330850Z 16 LL | fn paren2<T>() where for<'a> T: Fn() -> &'a i32 {
2020-03-02T20:08:14.9331593Z - 
2020-03-02T20:08:14.9331593Z - 
2020-03-02T20:08:14.9332220Z - error[E0582]: binding for associated type `Output` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9333304Z -    |
2020-03-02T20:08:14.9333736Z - LL | fn paren3(_: &dyn for<'a> Fn() -> &'a i32) {
2020-03-02T20:08:14.9334252Z -    |                                   ^^^^^^^
2020-03-02T20:08:14.9334468Z 24 
2020-03-02T20:08:14.9334468Z 24 
2020-03-02T20:08:14.9334682Z 25 error: aborting due to 4 previous errors
2020-03-02T20:08:14.9334907Z 26 
2020-03-02T20:08:14.9335017Z 
2020-03-02T20:08:14.9335119Z 
2020-03-02T20:08:14.9335511Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9336466Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.paren/bound-lifetime-in-binding-only.paren.stderr
2020-03-02T20:08:14.9337252Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9338016Z To only update this specific test, also pass `--test-args associated-types/bound-lifetime-in-binding-only.rs`
2020-03-02T20:08:14.9338338Z 
2020-03-02T20:08:14.9338588Z error in revision `paren`: 1 errors occurred comparing output.
2020-03-02T20:08:14.9338886Z status: exit code: 1
2020-03-02T20:08:14.9341697Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "paren" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.paren" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.paren/auxiliary"
2020-03-02T20:08:14.9343567Z ------------------------------------------
2020-03-02T20:08:14.9343753Z 
2020-03-02T20:08:14.9344135Z ------------------------------------------
2020-03-02T20:08:14.9344362Z stderr:
2020-03-02T20:08:14.9344362Z stderr:
2020-03-02T20:08:14.9344752Z ------------------------------------------
2020-03-02T20:08:14.9345444Z error[E0582]: binding for associated type `Output` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9346522Z    |
2020-03-02T20:08:14.9346943Z LL | fn paren3(_: &dyn for<'a> Fn() -> &'a i32) {
2020-03-02T20:08:14.9347275Z    |                                   ^^^^^^^
2020-03-02T20:08:14.9347466Z 
2020-03-02T20:08:14.9347466Z 
2020-03-02T20:08:14.9348082Z error[E0582]: binding for associated type `Output` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9349161Z    |
2020-03-02T20:08:14.9349161Z    |
2020-03-02T20:08:14.9349570Z LL | fn paren<T: for<'a> Fn() -> &'a i32>() {
2020-03-02T20:08:14.9350055Z 
2020-03-02T20:08:14.9350055Z 
2020-03-02T20:08:14.9350671Z error[E0582]: binding for associated type `Output` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9351743Z    |
2020-03-02T20:08:14.9351743Z    |
2020-03-02T20:08:14.9352185Z LL | fn paren1<T>() where T: for<'a> Fn() -> &'a i32 {
2020-03-02T20:08:14.9352748Z 
2020-03-02T20:08:14.9352748Z 
2020-03-02T20:08:14.9353367Z error[E0582]: binding for associated type `Output` references lifetime `'a`, which does not appear in the trait input types
2020-03-02T20:08:14.9354447Z    |
2020-03-02T20:08:14.9354447Z    |
2020-03-02T20:08:14.9354890Z LL | fn paren2<T>() where for<'a> T: Fn() -> &'a i32 {
2020-03-02T20:08:14.9355826Z 
2020-03-02T20:08:14.9356035Z error: aborting due to 4 previous errors
2020-03-02T20:08:14.9356216Z 
2020-03-02T20:08:14.9356707Z For more information about this error, try `rustc --explain E0582`.
2020-03-02T20:08:14.9356707Z For more information about this error, try `rustc --explain E0582`.
2020-03-02T20:08:14.9356952Z 
2020-03-02T20:08:14.9357328Z ------------------------------------------
2020-03-02T20:08:14.9357513Z 
2020-03-02T20:08:14.9357616Z 
2020-03-02T20:08:14.9358021Z ---- [ui] ui/error-codes/E0229.rs stdout ----
2020-03-02T20:08:14.9358261Z diff of stderr:
2020-03-02T20:08:14.9358486Z 
2020-03-02T20:08:14.9358727Z 4 LL | fn baz<I>(x: &<I as Foo<A=Bar>>::A) {}
2020-03-02T20:08:14.9359098Z 5    |                         ^^^^^ associated type not allowed here
2020-03-02T20:08:14.9359740Z - error: aborting due to previous error
2020-03-02T20:08:14.9360148Z + error[E0277]: the trait bound `I: Foo` is not satisfied
2020-03-02T20:08:14.9360631Z +   --> $DIR/E0229.rs:13:1
2020-03-02T20:08:14.9360826Z +    |
2020-03-02T20:08:14.9360826Z +    |
2020-03-02T20:08:14.9361083Z + LL | fn baz<I>(x: &<I as Foo<A=Bar>>::A) {}
2020-03-02T20:08:14.9361472Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `I`
2020-03-02T20:08:14.9361765Z +    |
2020-03-02T20:08:14.9362061Z + help: consider restricting this type parameter with `I: Foo`
2020-03-02T20:08:14.9362548Z +   --> $DIR/E0229.rs:13:8
2020-03-02T20:08:14.9362740Z +    |
2020-03-02T20:08:14.9362981Z + LL | fn baz<I>(x: &<I as Foo<A=Bar>>::A) {}
2020-03-02T20:08:14.9363408Z 8 
2020-03-02T20:08:14.9363872Z - For more information about this error, try `rustc --explain E0229`.
2020-03-02T20:08:14.9364225Z + error: aborting due to 2 previous errors
2020-03-02T20:08:14.9364432Z + 
2020-03-02T20:08:14.9364432Z + 
2020-03-02T20:08:14.9364686Z + Some errors have detailed explanations: E0229, E0277.
2020-03-02T20:08:14.9365276Z + For more information about an error, try `rustc --explain E0229`.
2020-03-02T20:08:14.9365532Z 10 
2020-03-02T20:08:14.9365644Z 
2020-03-02T20:08:14.9365747Z 
2020-03-02T20:08:14.9365979Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9366641Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0229/E0229.stderr
2020-03-02T20:08:14.9367282Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9367901Z To only update this specific test, also pass `--test-args error-codes/E0229.rs`
2020-03-02T20:08:14.9368360Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9368626Z status: exit code: 1
2020-03-02T20:08:14.9368626Z status: exit code: 1
2020-03-02T20:08:14.9370630Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0229.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0229" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0229/auxiliary"
2020-03-02T20:08:14.9372290Z ------------------------------------------
2020-03-02T20:08:14.9372475Z 
2020-03-02T20:08:14.9372869Z ------------------------------------------
2020-03-02T20:08:14.9373085Z stderr:
2020-03-02T20:08:14.9373085Z stderr:
2020-03-02T20:08:14.9373474Z ------------------------------------------
2020-03-02T20:08:14.9373813Z error[E0229]: associated type bindings are not allowed here
2020-03-02T20:08:14.9374362Z   --> /checkout/src/test/ui/error-codes/E0229.rs:13:25
2020-03-02T20:08:14.9374603Z    |
2020-03-02T20:08:14.9374850Z LL | fn baz<I>(x: &<I as Foo<A=Bar>>::A) {}
2020-03-02T20:08:14.9375946Z    |                         ^^^^^ associated type not allowed here
2020-03-02T20:08:14.9376422Z error[E0277]: the trait bound `I: Foo` is not satisfied
2020-03-02T20:08:14.9376993Z   --> /checkout/src/test/ui/error-codes/E0229.rs:13:1
2020-03-02T20:08:14.9377228Z    |
2020-03-02T20:08:14.9377228Z    |
2020-03-02T20:08:14.9377459Z LL | fn baz<I>(x: &<I as Foo<A=Bar>>::A) {}
2020-03-02T20:08:14.9377854Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `I`
2020-03-02T20:08:14.9378415Z help: consider restricting this type parameter with `I: Foo`
2020-03-02T20:08:14.9379000Z   --> /checkout/src/test/ui/error-codes/E0229.rs:13:8
2020-03-02T20:08:14.9379242Z    |
2020-03-02T20:08:14.9379242Z    |
2020-03-02T20:08:14.9379543Z LL | fn baz<I>(x: &<I as Foo<A=Bar>>::A) {}
2020-03-02T20:08:14.9379939Z 
2020-03-02T20:08:14.9380138Z error: aborting due to 2 previous errors
2020-03-02T20:08:14.9380320Z 
2020-03-02T20:08:14.9380572Z Some errors have detailed explanations: E0229, E0277.
---
2020-03-02T20:08:14.9382111Z 
2020-03-02T20:08:14.9382504Z ---- [ui] ui/error-codes/E0719.rs stdout ----
2020-03-02T20:08:14.9382745Z diff of stderr:
2020-03-02T20:08:14.9382889Z 
2020-03-02T20:08:14.9383261Z 1 error[E0719]: the value of the associated type `Item` (from trait `std::iter::Iterator`) is already specified
2020-03-02T20:08:14.9383851Z -   --> $DIR/E0719.rs:1:33
2020-03-02T20:08:14.9384203Z -    |
2020-03-02T20:08:14.9384650Z - LL | trait Foo: Iterator<Item = i32, Item = i32> {}
2020-03-02T20:08:14.9385199Z -    |                     ----------  ^^^^^^^^^^ re-bound here
2020-03-02T20:08:14.9386122Z -    |                     `Item` bound here first
2020-03-02T20:08:14.9386487Z - 
2020-03-02T20:08:14.9386487Z - 
2020-03-02T20:08:14.9387168Z - error[E0719]: the value of the associated type `Item` (from trait `std::iter::Iterator`) is already specified
2020-03-02T20:08:14.9387764Z 10   --> $DIR/E0719.rs:6:42
2020-03-02T20:08:14.9387963Z 11    |
2020-03-02T20:08:14.9388433Z 12 LL | fn test() -> Box<dyn Iterator<Item = (), Item = Unit>> {
2020-03-02T20:08:14.9388664Z 
2020-03-02T20:08:14.9389132Z 13    |                               ---------  ^^^^^^^^^^^ re-bound here
2020-03-02T20:08:14.9389787Z 15    |                               `Item` bound here first
2020-03-02T20:08:14.9390021Z + 
2020-03-02T20:08:14.9390021Z + 
2020-03-02T20:08:14.9390820Z + error[E0719]: the value of the associated type `Item` (from trait `std::iter::Iterator`) is already specified
2020-03-02T20:08:14.9391487Z +   --> $DIR/E0719.rs:1:33
2020-03-02T20:08:14.9391679Z +    |
2020-03-02T20:08:14.9391922Z + LL | trait Foo: Iterator<Item = i32, Item = i32> {}
2020-03-02T20:08:14.9392483Z +    |                     ----------  ^^^^^^^^^^ re-bound here
2020-03-02T20:08:14.9393056Z +    |                     `Item` bound here first
2020-03-02T20:08:14.9393288Z 16 
2020-03-02T20:08:14.9393501Z 17 error: aborting due to 2 previous errors
2020-03-02T20:08:14.9393711Z 18 
2020-03-02T20:08:14.9393711Z 18 
2020-03-02T20:08:14.9393821Z 
2020-03-02T20:08:14.9393941Z 
2020-03-02T20:08:14.9394160Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9394826Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719/E0719.stderr
2020-03-02T20:08:14.9395485Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9396100Z To only update this specific test, also pass `--test-args error-codes/E0719.rs`
2020-03-02T20:08:14.9396574Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9396829Z status: exit code: 1
2020-03-02T20:08:14.9396829Z status: exit code: 1
2020-03-02T20:08:14.9398988Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0719.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719/auxiliary"
2020-03-02T20:08:14.9400660Z ------------------------------------------
2020-03-02T20:08:14.9400858Z 
2020-03-02T20:08:14.9401316Z ------------------------------------------
2020-03-02T20:08:14.9401536Z stderr:
2020-03-02T20:08:14.9401536Z stderr:
2020-03-02T20:08:14.9401934Z ------------------------------------------
2020-03-02T20:08:14.9402394Z error[E0719]: the value of the associated type `Item` (from trait `std::iter::Iterator`) is already specified
2020-03-02T20:08:14.9403406Z    |
2020-03-02T20:08:14.9403406Z    |
2020-03-02T20:08:14.9403850Z LL | fn test() -> Box<dyn Iterator<Item = (), Item = Unit>> {
2020-03-02T20:08:14.9404427Z    |                               ---------  ^^^^^^^^^^^ re-bound here
2020-03-02T20:08:14.9405065Z    |                               `Item` bound here first
2020-03-02T20:08:14.9405278Z 
2020-03-02T20:08:14.9405278Z 
2020-03-02T20:08:14.9405645Z error[E0719]: the value of the associated type `Item` (from trait `std::iter::Iterator`) is already specified
2020-03-02T20:08:14.9406563Z    |
2020-03-02T20:08:14.9406563Z    |
2020-03-02T20:08:14.9406802Z LL | trait Foo: Iterator<Item = i32, Item = i32> {}
2020-03-02T20:08:14.9407350Z    |                     ----------  ^^^^^^^^^^ re-bound here
2020-03-02T20:08:14.9407899Z    |                     `Item` bound here first
2020-03-02T20:08:14.9408109Z 
2020-03-02T20:08:14.9408311Z error: aborting due to 2 previous errors
2020-03-02T20:08:14.9408493Z 
2020-03-02T20:08:14.9408493Z 
2020-03-02T20:08:14.9408595Z 
2020-03-02T20:08:14.9408984Z ------------------------------------------
2020-03-02T20:08:14.9409170Z 
2020-03-02T20:08:14.9409276Z 
2020-03-02T20:08:14.9409708Z ---- [ui] ui/impl-trait/impl_trait_projections.rs stdout ----
2020-03-02T20:08:14.9409988Z diff of stderr:
2020-03-02T20:08:14.9410121Z 
2020-03-02T20:08:14.9410646Z 28 LL | fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {
2020-03-02T20:08:14.9411609Z 29    |                                                  ^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<impl std::iter::Iterator as Trait>::Item`
2020-03-02T20:08:14.9412509Z - error: aborting due to 5 previous errors
2020-03-02T20:08:14.9412915Z + error[E0277]: the trait bound `impl std::fmt::Debug: std::iter::Step` is not satisfied
2020-03-02T20:08:14.9413535Z +   --> $DIR/impl_trait_projections.rs:25:1
2020-03-02T20:08:14.9413768Z +    |
2020-03-02T20:08:14.9413768Z +    |
2020-03-02T20:08:14.9414028Z + LL | / fn projection_with_named_trait_inside_path_is_disallowed()
2020-03-02T20:08:14.9414642Z + LL | |     -> <::std::ops::Range<impl Debug> as Iterator>::Item
2020-03-02T20:08:14.9415072Z + LL | |
2020-03-02T20:08:14.9418092Z + LL | | {
2020-03-02T20:08:14.9418347Z + LL | |     (1i32..100).next().unwrap()
2020-03-02T20:08:14.9418567Z + LL | | }
2020-03-02T20:08:14.9418908Z +    | |_^ the trait `std::iter::Step` is not implemented for `impl std::fmt::Debug`
2020-03-02T20:08:14.9419929Z +    = note: required because of the requirements on the impl of `std::iter::Iterator` for `std::ops::Range<impl std::fmt::Debug>`
2020-03-02T20:08:14.9420361Z 32 
2020-03-02T20:08:14.9420994Z - For more information about this error, try `rustc --explain E0223`.
2020-03-02T20:08:14.9421331Z + error: aborting due to 6 previous errors
---
2020-03-02T20:08:14.9422757Z 
2020-03-02T20:08:14.9422877Z 
2020-03-02T20:08:14.9423094Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9423839Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl_trait_projections/impl_trait_projections.stderr
2020-03-02T20:08:14.9424719Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9425998Z To only update this specific test, also pass `--test-args impl-trait/impl_trait_projections.rs`
2020-03-02T20:08:14.9426661Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9426913Z status: exit code: 1
2020-03-02T20:08:14.9426913Z status: exit code: 1
2020-03-02T20:08:14.9429051Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl_trait_projections.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl_trait_projections" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl_trait_projections/auxiliary"
2020-03-02T20:08:14.9431063Z ------------------------------------------
2020-03-02T20:08:14.9431266Z 
2020-03-02T20:08:14.9431654Z ------------------------------------------
2020-03-02T20:08:14.9431876Z stderr:
2020-03-02T20:08:14.9431876Z stderr:
2020-03-02T20:08:14.9432282Z ------------------------------------------
2020-03-02T20:08:14.9432604Z error[E0667]: `impl Trait` is not allowed in path parameters
2020-03-02T20:08:14.9433210Z   --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:12:51
2020-03-02T20:08:14.9433509Z    |
2020-03-02T20:08:14.9434034Z LL | fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {
2020-03-02T20:08:14.9434689Z 
2020-03-02T20:08:14.9434948Z error[E0667]: `impl Trait` is not allowed in path parameters
2020-03-02T20:08:14.9435542Z   --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:19:9
2020-03-02T20:08:14.9435818Z    |
---
2020-03-02T20:08:14.9438829Z 
2020-03-02T20:08:14.9439068Z error[E0667]: `impl Trait` is not allowed in path parameters
2020-03-02T20:08:14.9439683Z   --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:33:29
2020-03-02T20:08:14.9439957Z    |
2020-03-02T20:08:14.9440434Z LL |     -> <dyn Iterator<Item = impl Debug> as Iterator>::Item
2020-03-02T20:08:14.9440987Z 
2020-03-02T20:08:14.9441188Z error[E0223]: ambiguous associated type
2020-03-02T20:08:14.9441747Z   --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:12:50
2020-03-02T20:08:14.9442039Z    |
2020-03-02T20:08:14.9442039Z    |
2020-03-02T20:08:14.9442562Z LL | fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {
2020-03-02T20:08:14.9443503Z    |                                                  ^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<impl std::iter::Iterator as Trait>::Item`
2020-03-02T20:08:14.9444303Z error[E0277]: the trait bound `impl std::fmt::Debug: std::iter::Step` is not satisfied
2020-03-02T20:08:14.9444976Z   --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:25:1
2020-03-02T20:08:14.9445265Z    |
2020-03-02T20:08:14.9445516Z LL | / fn projection_with_named_trait_inside_path_is_disallowed()
2020-03-02T20:08:14.9445516Z LL | / fn projection_with_named_trait_inside_path_is_disallowed()
2020-03-02T20:08:14.9446105Z LL | |     -> <::std::ops::Range<impl Debug> as Iterator>::Item
2020-03-02T20:08:14.9446511Z LL | | //~^ ERROR `impl Trait` is not allowed in path parameters
2020-03-02T20:08:14.9446769Z LL | | {
2020-03-02T20:08:14.9446978Z LL | |     (1i32..100).next().unwrap()
2020-03-02T20:08:14.9447596Z    | |_^ the trait `std::iter::Step` is not implemented for `impl std::fmt::Debug`
2020-03-02T20:08:14.9447919Z    |
2020-03-02T20:08:14.9448366Z    = note: required because of the requirements on the impl of `std::iter::Iterator` for `std::ops::Range<impl std::fmt::Debug>`
2020-03-02T20:08:14.9448763Z 
---
2020-03-02T20:08:14.9453241Z - error: aborting due to 3 previous errors
2020-03-02T20:08:14.9453517Z + error[E0308]: mismatched types
2020-03-02T20:08:14.9453968Z +   --> $DIR/issue-26638.rs:1:69
2020-03-02T20:08:14.9454174Z +    |
2020-03-02T20:08:14.9454717Z + LL | fn parse_type(iter: Box<dyn Iterator<Item=&str>+'static>) -> &str { iter.next() }
2020-03-02T20:08:14.9455469Z +    |                                                                     ^^^^^^^^^^^ expected `&str`, found enum `std::option::Option`
2020-03-02T20:08:14.9456333Z +    = note: expected reference `&'static str`
2020-03-02T20:08:14.9456698Z +                    found enum `std::option::Option<&str>`
2020-03-02T20:08:14.9456960Z 30 
2020-03-02T20:08:14.9457424Z - For more information about this error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9457424Z - For more information about this error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9457838Z + error[E0061]: this function takes 1 argument but 0 arguments were supplied
2020-03-02T20:08:14.9458360Z +   --> $DIR/issue-26638.rs:4:47
2020-03-02T20:08:14.9458569Z +    |
2020-03-02T20:08:14.9459051Z + LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
2020-03-02T20:08:14.9459667Z +    |                                               ^^^^-- supplied 0 arguments
2020-03-02T20:08:14.9460420Z +    |                                               expected 1 argument
2020-03-02T20:08:14.9460675Z + 
2020-03-02T20:08:14.9460869Z + error[E0308]: mismatched types
2020-03-02T20:08:14.9461316Z +   --> $DIR/issue-26638.rs:4:47
2020-03-02T20:08:14.9461316Z +   --> $DIR/issue-26638.rs:4:47
2020-03-02T20:08:14.9461521Z +    |
2020-03-02T20:08:14.9461984Z + LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
2020-03-02T20:08:14.9462706Z +    |
2020-03-02T20:08:14.9463114Z +    = note: expected reference `&'static str`
2020-03-02T20:08:14.9463404Z +               found reference `&u8`
2020-03-02T20:08:14.9463612Z + 
---
2020-03-02T20:08:14.9465278Z 
2020-03-02T20:08:14.9465380Z 
2020-03-02T20:08:14.9465599Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9466284Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26638/issue-26638.stderr
2020-03-02T20:08:14.9466950Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9467559Z To only update this specific test, also pass `--test-args issues/issue-26638.rs`
2020-03-02T20:08:14.9468031Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9468280Z status: exit code: 1
2020-03-02T20:08:14.9468280Z status: exit code: 1
2020-03-02T20:08:14.9470350Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26638.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26638" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26638/auxiliary"
2020-03-02T20:08:14.9472080Z ------------------------------------------
2020-03-02T20:08:14.9472280Z 
2020-03-02T20:08:14.9472661Z ------------------------------------------
2020-03-02T20:08:14.9472875Z stderr:
2020-03-02T20:08:14.9472875Z stderr:
2020-03-02T20:08:14.9473277Z ------------------------------------------
2020-03-02T20:08:14.9473562Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9474077Z   --> /checkout/src/test/ui/issues/issue-26638.rs:1:62
2020-03-02T20:08:14.9474327Z    |
2020-03-02T20:08:14.9474860Z LL | fn parse_type(iter: Box<dyn Iterator<Item=&str>+'static>) -> &str { iter.next() }
2020-03-02T20:08:14.9475900Z    |
2020-03-02T20:08:14.9476596Z    = help: this function's return type contains a borrowed value, but the signature does not say which one of `iter`'s 2 lifetimes it is borrowed from
2020-03-02T20:08:14.9477107Z help: consider introducing a named lifetime parameter
2020-03-02T20:08:14.9477356Z    |
2020-03-02T20:08:14.9477356Z    |
2020-03-02T20:08:14.9477899Z LL | fn parse_type<'a>(iter: Box<dyn Iterator<Item=&str>+'static>) -> &'a str { iter.next() }
2020-03-02T20:08:14.9478596Z 
2020-03-02T20:08:14.9478802Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9479319Z   --> /checkout/src/test/ui/issues/issue-26638.rs:4:40
2020-03-02T20:08:14.9479579Z    |
2020-03-02T20:08:14.9479579Z    |
2020-03-02T20:08:14.9480032Z LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
2020-03-02T20:08:14.9481237Z    |
2020-03-02T20:08:14.9481910Z    = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
2020-03-02T20:08:14.9482286Z 
2020-03-02T20:08:14.9482489Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9482489Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9483014Z   --> /checkout/src/test/ui/issues/issue-26638.rs:7:22
2020-03-02T20:08:14.9483256Z    |
2020-03-02T20:08:14.9483671Z LL | fn parse_type_3() -> &str { unimplemented!() }
2020-03-02T20:08:14.9484314Z    |                      ^ help: consider giving it a 'static lifetime: `&'static`
2020-03-02T20:08:14.9485269Z    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
2020-03-02T20:08:14.9485608Z 
2020-03-02T20:08:14.9485793Z error[E0308]: mismatched types
2020-03-02T20:08:14.9486288Z   --> /checkout/src/test/ui/issues/issue-26638.rs:1:69
2020-03-02T20:08:14.9486288Z   --> /checkout/src/test/ui/issues/issue-26638.rs:1:69
2020-03-02T20:08:14.9486535Z    |
2020-03-02T20:08:14.9487073Z LL | fn parse_type(iter: Box<dyn Iterator<Item=&str>+'static>) -> &str { iter.next() }
2020-03-02T20:08:14.9487675Z    |                                                                     ^^^^^^^^^^^ expected `&str`, found enum `std::option::Option`
2020-03-02T20:08:14.9488529Z    = note: expected reference `&'static str`
2020-03-02T20:08:14.9488877Z                    found enum `std::option::Option<&str>`
2020-03-02T20:08:14.9489110Z 
2020-03-02T20:08:14.9489395Z error[E0061]: this function takes 1 argument but 0 arguments were supplied
2020-03-02T20:08:14.9489395Z error[E0061]: this function takes 1 argument but 0 arguments were supplied
2020-03-02T20:08:14.9489967Z   --> /checkout/src/test/ui/issues/issue-26638.rs:4:47
2020-03-02T20:08:14.9490265Z    |
2020-03-02T20:08:14.9490744Z LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
2020-03-02T20:08:14.9491350Z    |                                               ^^^^-- supplied 0 arguments
2020-03-02T20:08:14.9492126Z    |                                               expected 1 argument
2020-03-02T20:08:14.9492358Z 
2020-03-02T20:08:14.9492540Z error[E0308]: mismatched types
2020-03-02T20:08:14.9493044Z   --> /checkout/src/test/ui/issues/issue-26638.rs:4:47
2020-03-02T20:08:14.9493044Z   --> /checkout/src/test/ui/issues/issue-26638.rs:4:47
2020-03-02T20:08:14.9493302Z    |
2020-03-02T20:08:14.9493754Z LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
2020-03-02T20:08:14.9494453Z    |
2020-03-02T20:08:14.9494852Z    = note: expected reference `&'static str`
2020-03-02T20:08:14.9495287Z               found reference `&u8`
2020-03-02T20:08:14.9495485Z 
---
2020-03-02T20:08:14.9504040Z 
2020-03-02T20:08:14.9504143Z 
2020-03-02T20:08:14.9504363Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9505040Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3214/issue-3214.stderr
2020-03-02T20:08:14.9505702Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9506307Z To only update this specific test, also pass `--test-args issues/issue-3214.rs`
2020-03-02T20:08:14.9506774Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9507023Z status: exit code: 1
2020-03-02T20:08:14.9507023Z status: exit code: 1
2020-03-02T20:08:14.9509016Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3214.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3214" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3214/auxiliary"
2020-03-02T20:08:14.9510690Z ------------------------------------------
2020-03-02T20:08:14.9510876Z 
2020-03-02T20:08:14.9511342Z ------------------------------------------
2020-03-02T20:08:14.9511563Z stderr:
---
2020-03-02T20:08:14.9513982Z    |    --- - type parameter from outer function
2020-03-02T20:08:14.9514229Z    |    |
2020-03-02T20:08:14.9514497Z    |    try adding a local generic parameter in this method instead
2020-03-02T20:08:14.9514784Z LL |     struct Foo {
2020-03-02T20:08:14.9515328Z LL |         x: T, //~ ERROR can't use generic parameters from outer function
2020-03-02T20:08:14.9515721Z    |            ^ use of generic parameter from outer function
2020-03-02T20:08:14.9516206Z error[E0107]: wrong number of type arguments: expected 0, found 1
2020-03-02T20:08:14.9516795Z   --> /checkout/src/test/ui/issues/issue-3214.rs:6:26
2020-03-02T20:08:14.9517035Z    |
2020-03-02T20:08:14.9517231Z LL |     impl<T> Drop for Foo<T> {
---
2020-03-02T20:08:14.9521725Z 
2020-03-02T20:08:14.9522215Z ---- [ui] ui/lifetimes/lifetime-errors/ex1b-return-no-names-if-else.rs stdout ----
2020-03-02T20:08:14.9522527Z diff of stderr:
2020-03-02T20:08:14.9522676Z 
2020-03-02T20:08:14.9523122Z 10 LL | fn foo<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
2020-03-02T20:08:14.9523695Z 12 
2020-03-02T20:08:14.9524076Z - error: aborting due to previous error
2020-03-02T20:08:14.9524400Z + error[E0621]: explicit lifetime required in the type of `x`
2020-03-02T20:08:14.9524955Z +   --> $DIR/ex1b-return-no-names-if-else.rs:2:16
2020-03-02T20:08:14.9524955Z +   --> $DIR/ex1b-return-no-names-if-else.rs:2:16
2020-03-02T20:08:14.9525202Z +    |
2020-03-02T20:08:14.9525608Z + LL | fn foo(x: &i32, y: &i32) -> &i32 {
2020-03-02T20:08:14.9526279Z +    |           ---- help: add explicit lifetime `'static` to the type of `x`: `&'static i32`
2020-03-02T20:08:14.9526691Z + LL |     if x > y { x } else { y }
2020-03-02T20:08:14.9527164Z +    |                ^ lifetime `'static` required
2020-03-02T20:08:14.9527866Z - For more information about this error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9528245Z + error[E0621]: explicit lifetime required in the type of `y`
2020-03-02T20:08:14.9528784Z +   --> $DIR/ex1b-return-no-names-if-else.rs:2:27
2020-03-02T20:08:14.9529042Z +    |
2020-03-02T20:08:14.9529042Z +    |
2020-03-02T20:08:14.9529447Z + LL | fn foo(x: &i32, y: &i32) -> &i32 {
2020-03-02T20:08:14.9530137Z +    |                    ---- help: add explicit lifetime `'static` to the type of `y`: `&'static i32`
2020-03-02T20:08:14.9530577Z + LL |     if x > y { x } else { y }
2020-03-02T20:08:14.9531080Z +    |                           ^ lifetime `'static` required
2020-03-02T20:08:14.9531544Z + error: aborting due to 3 previous errors
2020-03-02T20:08:14.9531754Z + 
2020-03-02T20:08:14.9532058Z + Some errors have detailed explanations: E0106, E0621.
2020-03-02T20:08:14.9532662Z + For more information about an error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9532662Z + For more information about an error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9532917Z 16 
2020-03-02T20:08:14.9533029Z 
2020-03-02T20:08:14.9533133Z 
2020-03-02T20:08:14.9533367Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9534271Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1b-return-no-names-if-else/ex1b-return-no-names-if-else.stderr
2020-03-02T20:08:14.9535021Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9535879Z To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex1b-return-no-names-if-else.rs`
2020-03-02T20:08:14.9536400Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9536668Z status: exit code: 1
2020-03-02T20:08:14.9536668Z status: exit code: 1
2020-03-02T20:08:14.9538965Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex1b-return-no-names-if-else.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1b-return-no-names-if-else" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1b-return-no-names-if-else/auxiliary"
2020-03-02T20:08:14.9540835Z ------------------------------------------
2020-03-02T20:08:14.9541033Z 
2020-03-02T20:08:14.9541417Z ------------------------------------------
2020-03-02T20:08:14.9541631Z stderr:
2020-03-02T20:08:14.9541631Z stderr:
2020-03-02T20:08:14.9542020Z ------------------------------------------
2020-03-02T20:08:14.9542319Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9542942Z   --> /checkout/src/test/ui/lifetimes/lifetime-errors/ex1b-return-no-names-if-else.rs:1:29
2020-03-02T20:08:14.9543261Z    |
2020-03-02T20:08:14.9543754Z LL | fn foo(x: &i32, y: &i32) -> &i32 { //~ ERROR missing lifetime
2020-03-02T20:08:14.9544606Z    |
2020-03-02T20:08:14.9544606Z    |
2020-03-02T20:08:14.9545277Z    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
2020-03-02T20:08:14.9546004Z    |
2020-03-02T20:08:14.9546004Z    |
2020-03-02T20:08:14.9546519Z LL | fn foo<'a>(x: &'a i32, y: &'a i32) -> &'a i32 { //~ ERROR missing lifetime
2020-03-02T20:08:14.9547085Z 
2020-03-02T20:08:14.9547340Z error[E0621]: explicit lifetime required in the type of `x`
2020-03-02T20:08:14.9547997Z   --> /checkout/src/test/ui/lifetimes/lifetime-errors/ex1b-return-no-names-if-else.rs:2:16
2020-03-02T20:08:14.9548313Z    |
2020-03-02T20:08:14.9548313Z    |
2020-03-02T20:08:14.9548802Z LL | fn foo(x: &i32, y: &i32) -> &i32 { //~ ERROR missing lifetime
2020-03-02T20:08:14.9549499Z    |           ---- help: add explicit lifetime `'static` to the type of `x`: `&'static i32`
2020-03-02T20:08:14.9549902Z LL |     if x > y { x } else { y }
2020-03-02T20:08:14.9550381Z    |                ^ lifetime `'static` required
2020-03-02T20:08:14.9550817Z error[E0621]: explicit lifetime required in the type of `y`
2020-03-02T20:08:14.9551482Z   --> /checkout/src/test/ui/lifetimes/lifetime-errors/ex1b-return-no-names-if-else.rs:2:27
2020-03-02T20:08:14.9551797Z    |
2020-03-02T20:08:14.9551797Z    |
2020-03-02T20:08:14.9552271Z LL | fn foo(x: &i32, y: &i32) -> &i32 { //~ ERROR missing lifetime
2020-03-02T20:08:14.9553009Z    |                    ---- help: add explicit lifetime `'static` to the type of `y`: `&'static i32`
2020-03-02T20:08:14.9553431Z LL |     if x > y { x } else { y }
2020-03-02T20:08:14.9553993Z    |                           ^ lifetime `'static` required
2020-03-02T20:08:14.9554434Z error: aborting due to 3 previous errors
2020-03-02T20:08:14.9554616Z 
2020-03-02T20:08:14.9554856Z Some errors have detailed explanations: E0106, E0621.
2020-03-02T20:08:14.9555496Z For more information about an error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9555496Z For more information about an error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9555725Z 
2020-03-02T20:08:14.9556105Z ------------------------------------------
2020-03-02T20:08:14.9556288Z 
2020-03-02T20:08:14.9556391Z 
2020-03-02T20:08:14.9556921Z ---- [ui] ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3.rs stdout ----
2020-03-02T20:08:14.9557240Z diff of stderr:
2020-03-02T20:08:14.9557370Z 
2020-03-02T20:08:14.9557822Z 4 LL | fn bar(x: &str) -> &dyn Foo<Item = dyn Bar> { &() }
2020-03-02T20:08:14.9558367Z 6 
2020-03-02T20:08:14.9558760Z - error: aborting due to previous error
2020-03-02T20:08:14.9558760Z - error: aborting due to previous error
2020-03-02T20:08:14.9559306Z + error[E0277]: the trait bound `(): Foo<'_>` is not satisfied
2020-03-02T20:08:14.9559927Z +   --> $DIR/object-lifetime-default-dyn-binding-nonstatic3.rs:16:47
2020-03-02T20:08:14.9560228Z +    |
2020-03-02T20:08:14.9560674Z + LL | fn bar(x: &str) -> &dyn Foo<Item = dyn Bar> { &() }
2020-03-02T20:08:14.9561352Z +    |                                               ^^^ the trait `Foo<'_>` is not implemented for `()`
2020-03-02T20:08:14.9561692Z +    |
2020-03-02T20:08:14.9562204Z +    = note: required for the cast to the object type `dyn Foo<'_, Item = dyn Bar>`
2020-03-02T20:08:14.9562722Z + error: aborting due to 2 previous errors
2020-03-02T20:08:14.9562932Z + 
2020-03-02T20:08:14.9563393Z + For more information about this error, try `rustc --explain E0277`.
2020-03-02T20:08:14.9563648Z 9 
2020-03-02T20:08:14.9563648Z 9 
2020-03-02T20:08:14.9563769Z 
2020-03-02T20:08:14.9563874Z 
2020-03-02T20:08:14.9564093Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9564993Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3/object-lifetime-default-dyn-binding-nonstatic3.stderr
2020-03-02T20:08:14.9565804Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9566543Z To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-dyn-binding-nonstatic3.rs`
2020-03-02T20:08:14.9567095Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9567350Z status: exit code: 1
2020-03-02T20:08:14.9567350Z status: exit code: 1
2020-03-02T20:08:14.9569701Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3/auxiliary"
2020-03-02T20:08:14.9571604Z ------------------------------------------
2020-03-02T20:08:14.9571790Z 
2020-03-02T20:08:14.9572171Z ------------------------------------------
2020-03-02T20:08:14.9572403Z stderr:
2020-03-02T20:08:14.9572403Z stderr:
2020-03-02T20:08:14.9572791Z ------------------------------------------
2020-03-02T20:08:14.9573218Z error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
2020-03-02T20:08:14.9574021Z   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3.rs:16:36
2020-03-02T20:08:14.9574359Z    |
2020-03-02T20:08:14.9574795Z LL | fn bar(x: &str) -> &dyn Foo<Item = dyn Bar> { &() }
2020-03-02T20:08:14.9575695Z 
2020-03-02T20:08:14.9575695Z 
2020-03-02T20:08:14.9576177Z error[E0277]: the trait bound `(): Foo<'_>` is not satisfied
2020-03-02T20:08:14.9577299Z    |
2020-03-02T20:08:14.9577299Z    |
2020-03-02T20:08:14.9577745Z LL | fn bar(x: &str) -> &dyn Foo<Item = dyn Bar> { &() }
2020-03-02T20:08:14.9578424Z    |                                               ^^^ the trait `Foo<'_>` is not implemented for `()`
2020-03-02T20:08:14.9578748Z    |
2020-03-02T20:08:14.9579251Z    = note: required for the cast to the object type `dyn Foo<'_, Item = dyn Bar>`
2020-03-02T20:08:14.9579730Z error: aborting due to 2 previous errors
2020-03-02T20:08:14.9579911Z 
2020-03-02T20:08:14.9580362Z For more information about this error, try `rustc --explain E0277`.
2020-03-02T20:08:14.9580606Z 
2020-03-02T20:08:14.9580606Z 
2020-03-02T20:08:14.9580990Z ------------------------------------------
2020-03-02T20:08:14.9581173Z 
2020-03-02T20:08:14.9581277Z 
2020-03-02T20:08:14.9581769Z ---- [ui] ui/object-safety/object-safety-supertrait-mentions-Self.rs stdout ----
2020-03-02T20:08:14.9582068Z diff of stderr:
2020-03-02T20:08:14.9582199Z 
2020-03-02T20:08:14.9582615Z 9 LL | fn make_baz<T:Baz>(t: &T) -> &dyn Baz {
2020-03-02T20:08:14.9583030Z 10    |                               ^^^^^^^ the trait `Baz` cannot be made into an object
2020-03-02T20:08:14.9583750Z - error: aborting due to previous error
2020-03-02T20:08:14.9584138Z + error[E0277]: the size for values of type `Self` cannot be known at compilation time
2020-03-02T20:08:14.9584767Z +   --> $DIR/object-safety-supertrait-mentions-Self.rs:8:13
2020-03-02T20:08:14.9585036Z +    |
2020-03-02T20:08:14.9585036Z +    |
2020-03-02T20:08:14.9585225Z + LL | trait Bar<T> {
2020-03-02T20:08:14.9585630Z +    | ------------ required by `Bar`
2020-03-02T20:08:14.9585835Z + ...
2020-03-02T20:08:14.9586047Z + LL | trait Baz : Bar<Self> {
2020-03-02T20:08:14.9586844Z +    |
2020-03-02T20:08:14.9587150Z +    = help: the trait `std::marker::Sized` is not implemented for `Self`
2020-03-02T20:08:14.9587983Z +    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-03-02T20:08:14.9588396Z 13 
---
2020-03-02T20:08:14.9590617Z 
2020-03-02T20:08:14.9590720Z 
2020-03-02T20:08:14.9590953Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9591800Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-safety/object-safety-supertrait-mentions-Self/object-safety-supertrait-mentions-Self.stderr
2020-03-02T20:08:14.9592577Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9593288Z To only update this specific test, also pass `--test-args object-safety/object-safety-supertrait-mentions-Self.rs`
2020-03-02T20:08:14.9593819Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9594072Z status: exit code: 1
2020-03-02T20:08:14.9594072Z status: exit code: 1
2020-03-02T20:08:14.9596409Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-safety/object-safety-supertrait-mentions-Self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-safety/object-safety-supertrait-mentions-Self" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-safety/object-safety-supertrait-mentions-Self/auxiliary"
2020-03-02T20:08:14.9598321Z ------------------------------------------
2020-03-02T20:08:14.9598521Z 
2020-03-02T20:08:14.9598901Z ------------------------------------------
2020-03-02T20:08:14.9599116Z stderr:
2020-03-02T20:08:14.9599116Z stderr:
2020-03-02T20:08:14.9599518Z ------------------------------------------
2020-03-02T20:08:14.9599839Z error[E0038]: the trait `Baz` cannot be made into an object
2020-03-02T20:08:14.9600497Z   --> /checkout/src/test/ui/object-safety/object-safety-supertrait-mentions-Self.rs:15:31
2020-03-02T20:08:14.9600835Z    |
2020-03-02T20:08:14.9601031Z LL | trait Baz : Bar<Self> {
2020-03-02T20:08:14.9601575Z    |       ---   --------- ...because it uses `Self` as a type parameter in this
2020-03-02T20:08:14.9602143Z    |       this trait cannot be made into an object...
2020-03-02T20:08:14.9602371Z ...
2020-03-02T20:08:14.9602371Z ...
2020-03-02T20:08:14.9602794Z LL | fn make_baz<T:Baz>(t: &T) -> &dyn Baz {
2020-03-02T20:08:14.9603180Z    |                               ^^^^^^^ the trait `Baz` cannot be made into an object
2020-03-02T20:08:14.9604069Z error[E0277]: the size for values of type `Self` cannot be known at compilation time
2020-03-02T20:08:14.9604795Z   --> /checkout/src/test/ui/object-safety/object-safety-supertrait-mentions-Self.rs:8:13
2020-03-02T20:08:14.9605110Z    |
2020-03-02T20:08:14.9605278Z LL | trait Bar<T> {
2020-03-02T20:08:14.9605278Z LL | trait Bar<T> {
2020-03-02T20:08:14.9605693Z    | ------------ required by `Bar`
2020-03-02T20:08:14.9605890Z ...
2020-03-02T20:08:14.9606080Z LL | trait Baz : Bar<Self> {
2020-03-02T20:08:14.9606871Z    |
2020-03-02T20:08:14.9607166Z    = help: the trait `std::marker::Sized` is not implemented for `Self`
2020-03-02T20:08:14.9608002Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-03-02T20:08:14.9608382Z 
---
2020-03-02T20:08:14.9610503Z 
2020-03-02T20:08:14.9610873Z ---- [ui] ui/rfc1623-2.rs stdout ----
2020-03-02T20:08:14.9611111Z diff of stderr:
2020-03-02T20:08:14.9611244Z 
2020-03-02T20:08:14.9611697Z 24 LL |     &(non_elidable as for<'a> fn(&'a u8, &'a u8) -> &'a u8);
2020-03-02T20:08:14.9612334Z 26 
2020-03-02T20:08:14.9612728Z - error: aborting due to 2 previous errors
2020-03-02T20:08:14.9612728Z - error: aborting due to 2 previous errors
2020-03-02T20:08:14.9613482Z + error[E0605]: non-primitive cast: `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 {non_elidable}` as `for<'r, 's> fn(&'r u8, &'s u8) -> &u8`
2020-03-02T20:08:14.9614142Z +   --> $DIR/rfc1623-2.rs:10:6
2020-03-02T20:08:14.9614343Z +    |
2020-03-02T20:08:14.9614761Z + LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
2020-03-02T20:08:14.9615440Z +    |
2020-03-02T20:08:14.9615796Z +    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2020-03-02T20:08:14.9616156Z 28 
2020-03-02T20:08:14.9616630Z - For more information about this error, try `rustc --explain E0106`.
---
2020-03-02T20:08:14.9618354Z 30 
2020-03-02T20:08:14.9618464Z 
2020-03-02T20:08:14.9618568Z 
2020-03-02T20:08:14.9618788Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9619523Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623-2/rfc1623-2.stderr
2020-03-02T20:08:14.9620156Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9620739Z To only update this specific test, also pass `--test-args rfc1623-2.rs`
2020-03-02T20:08:14.9621195Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9621442Z status: exit code: 1
2020-03-02T20:08:14.9621442Z status: exit code: 1
2020-03-02T20:08:14.9623397Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1623-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623-2/auxiliary"
2020-03-02T20:08:14.9625011Z ------------------------------------------
2020-03-02T20:08:14.9625196Z 
2020-03-02T20:08:14.9625577Z ------------------------------------------
2020-03-02T20:08:14.9625791Z stderr:
2020-03-02T20:08:14.9625791Z stderr:
2020-03-02T20:08:14.9626197Z ------------------------------------------
2020-03-02T20:08:14.9626483Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9626967Z   --> /checkout/src/test/ui/rfc1623-2.rs:8:42
2020-03-02T20:08:14.9627205Z    |
2020-03-02T20:08:14.9627638Z LL | static NON_ELIDABLE_FN: &fn(&u8, &u8) -> &u8 =
2020-03-02T20:08:14.9628540Z    |
2020-03-02T20:08:14.9629237Z    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or argument 2
2020-03-02T20:08:14.9630153Z    = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
2020-03-02T20:08:14.9630876Z help: consider making the type lifetime-generic with a new `'a` lifetime
2020-03-02T20:08:14.9630876Z help: consider making the type lifetime-generic with a new `'a` lifetime
2020-03-02T20:08:14.9631145Z    |
2020-03-02T20:08:14.9631634Z LL | static NON_ELIDABLE_FN: &for<'a> fn(&'a u8, &'a u8) -> &'a u8 =
2020-03-02T20:08:14.9632240Z 
2020-03-02T20:08:14.9632444Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9632950Z   --> /checkout/src/test/ui/rfc1623-2.rs:10:39
2020-03-02T20:08:14.9633177Z    |
2020-03-02T20:08:14.9633177Z    |
2020-03-02T20:08:14.9633586Z LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
2020-03-02T20:08:14.9634453Z    |
2020-03-02T20:08:14.9635145Z    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or argument 2
2020-03-02T20:08:14.9636062Z    = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
2020-03-02T20:08:14.9636769Z help: consider making the type lifetime-generic with a new `'a` lifetime
2020-03-02T20:08:14.9636769Z help: consider making the type lifetime-generic with a new `'a` lifetime
2020-03-02T20:08:14.9637035Z    |
2020-03-02T20:08:14.9637506Z LL |     &(non_elidable as for<'a> fn(&'a u8, &'a u8) -> &'a u8);
2020-03-02T20:08:14.9638082Z 
2020-03-02T20:08:14.9638082Z 
2020-03-02T20:08:14.9638756Z error[E0605]: non-primitive cast: `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 {non_elidable}` as `for<'r, 's> fn(&'r u8, &'s u8) -> &u8`
2020-03-02T20:08:14.9639657Z    |
2020-03-02T20:08:14.9639657Z    |
2020-03-02T20:08:14.9640129Z LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
2020-03-02T20:08:14.9640655Z    |
2020-03-02T20:08:14.9641020Z    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2020-03-02T20:08:14.9641384Z 
2020-03-02T20:08:14.9641585Z error: aborting due to 3 previous errors
---
2020-03-02T20:08:14.9645159Z - error: aborting due to previous error
2020-03-02T20:08:14.9645447Z + error[E0223]: ambiguous associated type
2020-03-02T20:08:14.9645896Z +   --> $DIR/issue-35987.rs:9:32
2020-03-02T20:08:14.9646125Z +    |
2020-03-02T20:08:14.9646574Z + LL |     fn add(self, rhs: Self) -> Self::Output {
2020-03-02T20:08:14.9648272Z +    |                                ^^^^^^^^^^^^ help: use fully-qualified syntax: `<Foo<T> as Trait>::Output`
2020-03-02T20:08:14.9649196Z - For more information about this error, try `rustc --explain E0404`.
2020-03-02T20:08:14.9649536Z + error: aborting due to 2 previous errors
2020-03-02T20:08:14.9649758Z + 
2020-03-02T20:08:14.9650007Z + Some errors have detailed explanations: E0223, E0404.
2020-03-02T20:08:14.9650007Z + Some errors have detailed explanations: E0223, E0404.
2020-03-02T20:08:14.9650582Z + For more information about an error, try `rustc --explain E0223`.
2020-03-02T20:08:14.9650849Z 15 
2020-03-02T20:08:14.9650961Z 
2020-03-02T20:08:14.9651064Z 
2020-03-02T20:08:14.9651292Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9652221Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-35987/issue-35987.stderr
2020-03-02T20:08:14.9652897Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9653828Z To only update this specific test, also pass `--test-args span/issue-35987.rs`
2020-03-02T20:08:14.9654304Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9654554Z status: exit code: 1
2020-03-02T20:08:14.9654554Z status: exit code: 1
2020-03-02T20:08:14.9656789Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-35987.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-35987" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-35987/auxiliary"
2020-03-02T20:08:14.9658462Z ------------------------------------------
2020-03-02T20:08:14.9658652Z 
2020-03-02T20:08:14.9659037Z ------------------------------------------
2020-03-02T20:08:14.9659266Z stderr:
2020-03-02T20:08:14.9659266Z stderr:
2020-03-02T20:08:14.9659656Z ------------------------------------------
2020-03-02T20:08:14.9659972Z error[E0404]: expected trait, found type parameter `Add`
2020-03-02T20:08:14.9660522Z   --> /checkout/src/test/ui/span/issue-35987.rs:5:21
2020-03-02T20:08:14.9660765Z    |
2020-03-02T20:08:14.9660983Z LL | impl<T: Clone, Add> Add for Foo<T> {
2020-03-02T20:08:14.9661483Z    |
2020-03-02T20:08:14.9661781Z help: possible better candidate is found in another module, you can import it into scope
2020-03-02T20:08:14.9662090Z    |
2020-03-02T20:08:14.9662397Z LL | use std::ops::Add;
2020-03-02T20:08:14.9662397Z LL | use std::ops::Add;
2020-03-02T20:08:14.9662586Z    |
2020-03-02T20:08:14.9662696Z 
2020-03-02T20:08:14.9662913Z error[E0223]: ambiguous associated type
2020-03-02T20:08:14.9663431Z   --> /checkout/src/test/ui/span/issue-35987.rs:9:32
2020-03-02T20:08:14.9663736Z    |
2020-03-02T20:08:14.9664192Z LL |     fn add(self, rhs: Self) -> Self::Output {
2020-03-02T20:08:14.9664942Z    |                                ^^^^^^^^^^^^ help: use fully-qualified syntax: `<Foo<T> as Trait>::Output`
2020-03-02T20:08:14.9665498Z error: aborting due to 2 previous errors
2020-03-02T20:08:14.9665695Z 
2020-03-02T20:08:14.9665937Z Some errors have detailed explanations: E0223, E0404.
2020-03-02T20:08:14.9666503Z For more information about an error, try `rustc --explain E0223`.
---
2020-03-02T20:08:14.9669971Z - error: aborting due to 3 previous errors
2020-03-02T20:08:14.9670466Z + error[E0392]: parameter `'a` is never used
2020-03-02T20:08:14.9670962Z +   --> $DIR/return-without-lifetime.rs:2:12
2020-03-02T20:08:14.9671195Z +    |
2020-03-02T20:08:14.9671566Z + LL | struct Foo<'a>(&usize);
2020-03-02T20:08:14.9671818Z +    |            ^^ unused parameter
2020-03-02T20:08:14.9672690Z +    = help: consider removing `'a`, referring to it in a field, or using a marker such as `std::marker::PhantomData`
2020-03-02T20:08:14.9673088Z 24 
2020-03-02T20:08:14.9673556Z - For more information about this error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9673905Z + error: aborting due to 4 previous errors
2020-03-02T20:08:14.9673905Z + error: aborting due to 4 previous errors
2020-03-02T20:08:14.9674113Z + 
2020-03-02T20:08:14.9674364Z + Some errors have detailed explanations: E0106, E0392.
2020-03-02T20:08:14.9674957Z + For more information about an error, try `rustc --explain E0106`.
2020-03-02T20:08:14.9675210Z 26 
2020-03-02T20:08:14.9675320Z 
2020-03-02T20:08:14.9675421Z 
2020-03-02T20:08:14.9675641Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9676413Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/return-without-lifetime/return-without-lifetime.stderr
2020-03-02T20:08:14.9677120Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9677786Z To only update this specific test, also pass `--test-args suggestions/return-without-lifetime.rs`
2020-03-02T20:08:14.9678272Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9678525Z status: exit code: 1
2020-03-02T20:08:14.9678525Z status: exit code: 1
2020-03-02T20:08:14.9680678Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/return-without-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/return-without-lifetime" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/return-without-lifetime/auxiliary"
2020-03-02T20:08:14.9682431Z ------------------------------------------
2020-03-02T20:08:14.9682618Z 
2020-03-02T20:08:14.9682998Z ------------------------------------------
2020-03-02T20:08:14.9683262Z stderr:
---
2020-03-02T20:08:14.9686136Z 
2020-03-02T20:08:14.9686341Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9686916Z   --> /checkout/src/test/ui/suggestions/return-without-lifetime.rs:5:34
2020-03-02T20:08:14.9687193Z    |
2020-03-02T20:08:14.9687655Z LL | fn func1<'a>(_arg: &'a Thing) -> &() { unimplemented!() }
2020-03-02T20:08:14.9688343Z    |                    ---------     ^ help: consider using the named lifetime: `&'a`
2020-03-02T20:08:14.9689373Z    = help: this function's return type contains a borrowed value, but the signature does not say which one of `_arg`'s 2 lifetimes it is borrowed from
2020-03-02T20:08:14.9689781Z 
2020-03-02T20:08:14.9689990Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:14.9690550Z   --> /checkout/src/test/ui/suggestions/return-without-lifetime.rs:7:35
2020-03-02T20:08:14.9690550Z   --> /checkout/src/test/ui/suggestions/return-without-lifetime.rs:7:35
2020-03-02T20:08:14.9690841Z    |
2020-03-02T20:08:14.9691311Z LL | fn func2<'a>(_arg: &Thing<'a>) -> &() { unimplemented!() }
2020-03-02T20:08:14.9691987Z    |                    ----------     ^ help: consider using the named lifetime: `&'a`
2020-03-02T20:08:14.9693023Z    = help: this function's return type contains a borrowed value, but the signature does not say which one of `_arg`'s 2 lifetimes it is borrowed from
2020-03-02T20:08:14.9693411Z 
2020-03-02T20:08:14.9693798Z error[E0392]: parameter `'a` is never used
2020-03-02T20:08:14.9694377Z   --> /checkout/src/test/ui/suggestions/return-without-lifetime.rs:2:12
---
2020-03-02T20:08:14.9698722Z 
2020-03-02T20:08:14.9699098Z ---- [ui] ui/tag-type-args.rs stdout ----
2020-03-02T20:08:14.9699331Z diff of stderr:
2020-03-02T20:08:14.9699462Z 
2020-03-02T20:08:14.9699691Z 4 LL | fn foo(c: Quux) { assert!((false)); }
2020-03-02T20:08:14.9699992Z 5    |           ^^^^ expected 1 type argument
2020-03-02T20:08:14.9700600Z - error: aborting due to previous error
2020-03-02T20:08:14.9700892Z + error[E0392]: parameter `T` is never used
2020-03-02T20:08:14.9701355Z +   --> $DIR/tag-type-args.rs:1:11
2020-03-02T20:08:14.9701580Z +    |
2020-03-02T20:08:14.9701580Z +    |
2020-03-02T20:08:14.9701765Z + LL | enum Quux<T> { Bar }
2020-03-02T20:08:14.9702009Z +    |           ^ unused parameter
2020-03-02T20:08:14.9702624Z +    = help: consider removing `T`, referring to it in a field, or using a marker such as `std::marker::PhantomData`
2020-03-02T20:08:14.9703017Z 8 
2020-03-02T20:08:14.9703479Z - For more information about this error, try `rustc --explain E0107`.
2020-03-02T20:08:14.9703827Z + error: aborting due to 2 previous errors
2020-03-02T20:08:14.9703827Z + error: aborting due to 2 previous errors
2020-03-02T20:08:14.9704031Z + 
2020-03-02T20:08:14.9704282Z + Some errors have detailed explanations: E0107, E0392.
2020-03-02T20:08:14.9704867Z + For more information about an error, try `rustc --explain E0107`.
2020-03-02T20:08:14.9705120Z 10 
2020-03-02T20:08:14.9705228Z 
2020-03-02T20:08:14.9705332Z 
2020-03-02T20:08:14.9705636Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9706318Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-type-args/tag-type-args.stderr
2020-03-02T20:08:14.9706962Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9707639Z To only update this specific test, also pass `--test-args tag-type-args.rs`
2020-03-02T20:08:14.9708091Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9708357Z status: exit code: 1
2020-03-02T20:08:14.9708357Z status: exit code: 1
2020-03-02T20:08:14.9710316Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tag-type-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-type-args" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-type-args/auxiliary"
2020-03-02T20:08:14.9711948Z ------------------------------------------
2020-03-02T20:08:14.9712137Z 
2020-03-02T20:08:14.9712530Z ------------------------------------------
2020-03-02T20:08:14.9712745Z stderr:
2020-03-02T20:08:14.9712745Z stderr:
2020-03-02T20:08:14.9713132Z ------------------------------------------
2020-03-02T20:08:14.9713497Z error[E0107]: wrong number of type arguments: expected 1, found 0
2020-03-02T20:08:14.9714053Z   --> /checkout/src/test/ui/tag-type-args.rs:3:11
2020-03-02T20:08:14.9714292Z    |
2020-03-02T20:08:14.9714603Z LL | fn foo(c: Quux) { assert!((false)); } //~ ERROR wrong number of type arguments
2020-03-02T20:08:14.9714972Z    |           ^^^^ expected 1 type argument
2020-03-02T20:08:14.9715364Z error[E0392]: parameter `T` is never used
2020-03-02T20:08:14.9715879Z   --> /checkout/src/test/ui/tag-type-args.rs:1:11
2020-03-02T20:08:14.9716116Z    |
2020-03-02T20:08:14.9716116Z    |
2020-03-02T20:08:14.9716294Z LL | enum Quux<T> { Bar }
2020-03-02T20:08:14.9716540Z    |           ^ unused parameter
2020-03-02T20:08:14.9717123Z    = help: consider removing `T`, referring to it in a field, or using a marker such as `std::marker::PhantomData`
2020-03-02T20:08:14.9717507Z 
2020-03-02T20:08:14.9717709Z error: aborting due to 2 previous errors
2020-03-02T20:08:14.9717892Z 
---
2020-03-02T20:08:14.9719617Z 
2020-03-02T20:08:14.9720056Z ---- [ui] ui/typeck/typeck_type_placeholder_item.rs stdout ----
2020-03-02T20:08:14.9720326Z diff of stderr:
2020-03-02T20:08:14.9720457Z 
2020-03-02T20:08:14.9720779Z 64    |               help: replace with the correct return type: `(i32, i32)`
2020-03-02T20:08:14.9721387Z 66 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9722000Z -   --> $DIR/typeck_type_placeholder_item.rs:12:15
2020-03-02T20:08:14.9722389Z -    |
2020-03-02T20:08:14.9722389Z -    |
2020-03-02T20:08:14.9722768Z - LL | static TEST3: _ = "test";
2020-03-02T20:08:14.9723541Z -    |               |
2020-03-02T20:08:14.9723985Z -    |               not allowed in type signatures
2020-03-02T20:08:14.9724604Z -    |               help: replace `_` with the correct type: `&'static str`
2020-03-02T20:08:14.9725043Z - 
2020-03-02T20:08:14.9725043Z - 
2020-03-02T20:08:14.9725565Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9726168Z -   --> $DIR/typeck_type_placeholder_item.rs:15:15
2020-03-02T20:08:14.9726558Z -    |
2020-03-02T20:08:14.9726980Z - LL | static TEST4: _ = 145;
2020-03-02T20:08:14.9727763Z -    |               |
2020-03-02T20:08:14.9728207Z -    |               not allowed in type signatures
2020-03-02T20:08:14.9728788Z -    |               help: replace `_` with the correct type: `i32`
2020-03-02T20:08:14.9729292Z - 
---
2020-03-02T20:08:14.9732092Z - 
2020-03-02T20:08:14.9732613Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9733208Z 91   --> $DIR/typeck_type_placeholder_item.rs:21:13
2020-03-02T20:08:14.9733470Z 92    |
2020-03-02T20:08:14.9733673Z 93 LL | fn test6(_: _) { }
2020-03-02T20:08:14.9733840Z 
2020-03-02T20:08:14.9734215Z 167    |                                      help: replace with the correct return type: `*const *const usize`
2020-03-02T20:08:14.9734911Z 169 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9735614Z -   --> $DIR/typeck_type_placeholder_item.rs:66:8
2020-03-02T20:08:14.9736018Z -    |
2020-03-02T20:08:14.9736345Z - LL |     a: _,
---
2020-03-02T20:08:14.9738777Z -    |         not allowed in type signatures
2020-03-02T20:08:14.9739138Z -    |
2020-03-02T20:08:14.9739527Z - help: use type parameters instead
2020-03-02T20:08:14.9739875Z -    |
2020-03-02T20:08:14.9740225Z - LL | struct Test10<T> {
2020-03-02T20:08:14.9740604Z - LL |     a: T,
2020-03-02T20:08:14.9740920Z - LL |
2020-03-02T20:08:14.9741258Z - LL |     b: (T, T),
2020-03-02T20:08:14.9741871Z - 
2020-03-02T20:08:14.9743683Z - error: missing type for `static` item
2020-03-02T20:08:14.9744197Z -   --> $DIR/typeck_type_placeholder_item.rs:72:12
2020-03-02T20:08:14.9744602Z -    |
2020-03-02T20:08:14.9744602Z -    |
2020-03-02T20:08:14.9744949Z - LL |     static A = 42;
2020-03-02T20:08:14.9745472Z -    |            ^ help: provide a type for the item: `A: i32`
2020-03-02T20:08:14.9745901Z - 
2020-03-02T20:08:14.9746429Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9747020Z -   --> $DIR/typeck_type_placeholder_item.rs:74:15
2020-03-02T20:08:14.9747419Z -    |
2020-03-02T20:08:14.9747790Z - LL |     static B: _ = 42;
2020-03-02T20:08:14.9748540Z -    |               |
2020-03-02T20:08:14.9749007Z -    |               not allowed in type signatures
2020-03-02T20:08:14.9749590Z -    |               help: replace `_` with the correct type: `i32`
2020-03-02T20:08:14.9750010Z - 
2020-03-02T20:08:14.9750010Z - 
2020-03-02T20:08:14.9750552Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9751154Z -   --> $DIR/typeck_type_placeholder_item.rs:76:15
2020-03-02T20:08:14.9751540Z -    |
2020-03-02T20:08:14.9751961Z - LL |     static C: Option<_> = Some(42);
2020-03-02T20:08:14.9752860Z - 
2020-03-02T20:08:14.9753391Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9753994Z 210   --> $DIR/typeck_type_placeholder_item.rs:79:21
2020-03-02T20:08:14.9754248Z 211    |
2020-03-02T20:08:14.9754248Z 211    |
2020-03-02T20:08:14.9754642Z 212 LL |     fn fn_test() -> _ { 5 }
2020-03-02T20:08:14.9754816Z 
2020-03-02T20:08:14.9755256Z 226    |                      help: replace with the correct return type: `(i32, i32)`
2020-03-02T20:08:14.9755908Z 228 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9756512Z -   --> $DIR/typeck_type_placeholder_item.rs:85:22
2020-03-02T20:08:14.9756982Z -    |
2020-03-02T20:08:14.9756982Z -    |
2020-03-02T20:08:14.9757399Z - LL |     static FN_TEST3: _ = "test";
2020-03-02T20:08:14.9758228Z -    |                      |
2020-03-02T20:08:14.9758721Z -    |                      not allowed in type signatures
2020-03-02T20:08:14.9759356Z -    |                      help: replace `_` with the correct type: `&'static str`
2020-03-02T20:08:14.9759810Z - 
2020-03-02T20:08:14.9759810Z - 
2020-03-02T20:08:14.9760347Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9760936Z -   --> $DIR/typeck_type_placeholder_item.rs:88:22
2020-03-02T20:08:14.9761324Z -    |
2020-03-02T20:08:14.9761734Z - LL |     static FN_TEST4: _ = 145;
2020-03-02T20:08:14.9762561Z -    |                      |
2020-03-02T20:08:14.9763050Z -    |                      not allowed in type signatures
2020-03-02T20:08:14.9763663Z -    |                      help: replace `_` with the correct type: `i32`
2020-03-02T20:08:14.9764108Z - 
2020-03-02T20:08:14.9764108Z - 
2020-03-02T20:08:14.9764641Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9765232Z -   --> $DIR/typeck_type_placeholder_item.rs:91:22
2020-03-02T20:08:14.9765620Z -    |
2020-03-02T20:08:14.9766051Z - LL |     static FN_TEST5: (_, _) = (1, 2);
2020-03-02T20:08:14.9766979Z - 
2020-03-02T20:08:14.9767512Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9768111Z 253   --> $DIR/typeck_type_placeholder_item.rs:94:20
2020-03-02T20:08:14.9768365Z 254    |
2020-03-02T20:08:14.9768365Z 254    |
2020-03-02T20:08:14.9768601Z 255 LL |     fn fn_test6(_: _) { }
2020-03-02T20:08:14.9768786Z 
2020-03-02T20:08:14.9769198Z 288 LL |     fn fn_test8<T>(_f: fn() -> T) { }
2020-03-02T20:08:14.9769723Z 290 
2020-03-02T20:08:14.9770252Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9770848Z -   --> $DIR/typeck_type_placeholder_item.rs:123:12
2020-03-02T20:08:14.9771251Z -    |
---
2020-03-02T20:08:14.9774112Z -    |             not allowed in type signatures
2020-03-02T20:08:14.9774478Z -    |
2020-03-02T20:08:14.9774852Z - help: use type parameters instead
2020-03-02T20:08:14.9775340Z -    |
2020-03-02T20:08:14.9775707Z - LL |     struct FnTest10<T> {
2020-03-02T20:08:14.9776090Z - LL |         a: T,
2020-03-02T20:08:14.9776426Z - LL |
2020-03-02T20:08:14.9776774Z - LL |         b: (T, T),
2020-03-02T20:08:14.9777401Z - 
2020-03-02T20:08:14.9777619Z 310 error[E0282]: type annotations needed
2020-03-02T20:08:14.9778135Z 311   --> $DIR/typeck_type_placeholder_item.rs:128:18
2020-03-02T20:08:14.9778390Z 312    |
2020-03-02T20:08:14.9778390Z 312    |
2020-03-02T20:08:14.9778526Z 
2020-03-02T20:08:14.9778858Z 341    |                           help: replace with the correct return type: `(i32, i32)`
2020-03-02T20:08:14.9779514Z 343 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9780117Z -   --> $DIR/typeck_type_placeholder_item.rs:154:21
2020-03-02T20:08:14.9780646Z +   --> $DIR/typeck_type_placeholder_item.rs:40:24
2020-03-02T20:08:14.9780908Z 345    |
2020-03-02T20:08:14.9780908Z 345    |
2020-03-02T20:08:14.9781337Z - LL | struct BadStruct<_>(_);
2020-03-02T20:08:14.9782275Z -    |
2020-03-02T20:08:14.9782652Z - help: use type parameters instead
2020-03-02T20:08:14.9782999Z -    |
2020-03-02T20:08:14.9782999Z -    |
2020-03-02T20:08:14.9783430Z - LL | struct BadStruct<T>(T);
2020-03-02T20:08:14.9783842Z -    |                  ^  ^
2020-03-02T20:08:14.9784265Z + LL |     fn test9(&self) -> _ { () }
2020-03-02T20:08:14.9784773Z +    |                        |
2020-03-02T20:08:14.9785063Z +    |                        not allowed in type signatures
2020-03-02T20:08:14.9785480Z +    |                        help: replace with the correct return type: `()`
2020-03-02T20:08:14.9785805Z 353 
2020-03-02T20:08:14.9785805Z 353 
2020-03-02T20:08:14.9786110Z 354 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9786715Z -   --> $DIR/typeck_type_placeholder_item.rs:159:32
2020-03-02T20:08:14.9787123Z -    |
2020-03-02T20:08:14.9787514Z - LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:14.9788456Z - 
2020-03-02T20:08:14.9788975Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9789580Z -   --> $DIR/typeck_type_placeholder_item.rs:159:15
2020-03-02T20:08:14.9789982Z -    |
2020-03-02T20:08:14.9789982Z -    |
2020-03-02T20:08:14.9790377Z - LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:14.9791242Z - 
2020-03-02T20:08:14.9791766Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9792357Z -   --> $DIR/typeck_type_placeholder_item.rs:159:15
2020-03-02T20:08:14.9792760Z -    |
2020-03-02T20:08:14.9792760Z -    |
2020-03-02T20:08:14.9793153Z - LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:14.9794161Z -    |               |
2020-03-02T20:08:14.9794609Z -    |               not allowed in type signatures
2020-03-02T20:08:14.9794981Z -    |
2020-03-02T20:08:14.9795355Z - help: use type parameters instead
2020-03-02T20:08:14.9795355Z - help: use type parameters instead
2020-03-02T20:08:14.9795720Z -    |
2020-03-02T20:08:14.9796122Z - LL | impl<T> BadTrait<T> for BadStruct<T> {}
2020-03-02T20:08:14.9796947Z - 
2020-03-02T20:08:14.9797469Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9798061Z -   --> $DIR/typeck_type_placeholder_item.rs:164:34
2020-03-02T20:08:14.9798464Z -    |
2020-03-02T20:08:14.9798464Z -    |
2020-03-02T20:08:14.9798857Z - LL | fn impl_trait() -> impl BadTrait<_> {
2020-03-02T20:08:14.9799816Z - 
2020-03-02T20:08:14.9800340Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9800935Z -   --> $DIR/typeck_type_placeholder_item.rs:170:25
2020-03-02T20:08:14.9801337Z -    |
2020-03-02T20:08:14.9801337Z -    |
2020-03-02T20:08:14.9801708Z - LL | struct BadStruct1<_, _>(_);
2020-03-02T20:08:14.9802629Z -    |
2020-03-02T20:08:14.9803005Z - help: use type parameters instead
2020-03-02T20:08:14.9803352Z -    |
2020-03-02T20:08:14.9803352Z -    |
2020-03-02T20:08:14.9803719Z - LL | struct BadStruct1<T, _>(T);
2020-03-02T20:08:14.9804470Z - 
2020-03-02T20:08:14.9804988Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9805592Z -   --> $DIR/typeck_type_placeholder_item.rs:175:25
2020-03-02T20:08:14.9805980Z -    |
2020-03-02T20:08:14.9805980Z -    |
2020-03-02T20:08:14.9806355Z - LL | struct BadStruct2<_, T>(_, T);
2020-03-02T20:08:14.9807268Z -    |
2020-03-02T20:08:14.9807695Z - help: use type parameters instead
2020-03-02T20:08:14.9808071Z -    |
2020-03-02T20:08:14.9808071Z -    |
2020-03-02T20:08:14.9809133Z - LL | struct BadStruct2<K, T>(K, T);
2020-03-02T20:08:14.9809950Z - 
2020-03-02T20:08:14.9810569Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9811166Z -   --> $DIR/typeck_type_placeholder_item.rs:179:14
2020-03-02T20:08:14.9811572Z -    |
2020-03-02T20:08:14.9811572Z -    |
2020-03-02T20:08:14.9811913Z - LL | type X = Box<_>;
2020-03-02T20:08:14.9812724Z - 
2020-03-02T20:08:14.9813489Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9814082Z -   --> $DIR/typeck_type_placeholder_item.rs:179:14
2020-03-02T20:08:14.9814474Z -    |
2020-03-02T20:08:14.9814474Z -    |
2020-03-02T20:08:14.9814826Z - LL | type X = Box<_>;
2020-03-02T20:08:14.9815793Z - 
2020-03-02T20:08:14.9816331Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9816927Z 420   --> $DIR/typeck_type_placeholder_item.rs:43:27
2020-03-02T20:08:14.9817179Z 421    |
2020-03-02T20:08:14.9817179Z 421    |
2020-03-02T20:08:14.9817440Z 422 LL |     fn test10(&self, _x : _) { }
2020-03-02T20:08:14.9817838Z 498    |                      ^^^      ^
2020-03-02T20:08:14.9818040Z 499 
2020-03-02T20:08:14.9818362Z 500 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9818959Z +   --> $DIR/typeck_type_placeholder_item.rs:58:24
2020-03-02T20:08:14.9818959Z +   --> $DIR/typeck_type_placeholder_item.rs:58:24
2020-03-02T20:08:14.9819203Z +    |
2020-03-02T20:08:14.9819611Z + LL |     fn clone(&self) -> _ { Test9 }
2020-03-02T20:08:14.9820113Z +    |                        |
2020-03-02T20:08:14.9820416Z +    |                        not allowed in type signatures
2020-03-02T20:08:14.9820846Z +    |                        help: replace with the correct return type: `Test9`
2020-03-02T20:08:14.9821159Z + 
2020-03-02T20:08:14.9821159Z + 
2020-03-02T20:08:14.9821469Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9822068Z 501   --> $DIR/typeck_type_placeholder_item.rs:61:37
2020-03-02T20:08:14.9822331Z 502    |
2020-03-02T20:08:14.9822636Z 503 LL |     fn clone_from(&mut self, other: _) { *self = Test9; }
2020-03-02T20:08:14.9823100Z 509    |                  ^^^                   ^
2020-03-02T20:08:14.9823332Z 510 
2020-03-02T20:08:14.9823640Z 511 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9824242Z +   --> $DIR/typeck_type_placeholder_item.rs:107:31
2020-03-02T20:08:14.9824242Z +   --> $DIR/typeck_type_placeholder_item.rs:107:31
2020-03-02T20:08:14.9824489Z +    |
2020-03-02T20:08:14.9824902Z + LL |         fn fn_test9(&self) -> _ { () }
2020-03-02T20:08:14.9825450Z +    |                               |
2020-03-02T20:08:14.9825782Z +    |                               not allowed in type signatures
2020-03-02T20:08:14.9826226Z +    |                               help: replace with the correct return type: `()`
2020-03-02T20:08:14.9826542Z + 
2020-03-02T20:08:14.9826542Z + 
2020-03-02T20:08:14.9826859Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9827471Z 512   --> $DIR/typeck_type_placeholder_item.rs:110:34
2020-03-02T20:08:14.9827725Z 513    |
2020-03-02T20:08:14.9827993Z 514 LL |         fn fn_test10(&self, _x : _) { }
2020-03-02T20:08:14.9828420Z 520    |                     ^^^             ^
2020-03-02T20:08:14.9828646Z 521 
2020-03-02T20:08:14.9828953Z 522 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9829554Z +   --> $DIR/typeck_type_placeholder_item.rs:115:28
2020-03-02T20:08:14.9829554Z +   --> $DIR/typeck_type_placeholder_item.rs:115:28
2020-03-02T20:08:14.9829816Z +    |
2020-03-02T20:08:14.9830225Z + LL |         fn clone(&self) -> _ { FnTest9 }
2020-03-02T20:08:14.9830837Z +    |                            |
2020-03-02T20:08:14.9831162Z +    |                            not allowed in type signatures
2020-03-02T20:08:14.9831625Z +    |                            help: replace with the correct return type: `main::FnTest9`
2020-03-02T20:08:14.9832015Z + 
2020-03-02T20:08:14.9832015Z + 
2020-03-02T20:08:14.9832328Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9832940Z 523   --> $DIR/typeck_type_placeholder_item.rs:118:41
2020-03-02T20:08:14.9833193Z 524    |
2020-03-02T20:08:14.9833509Z 525 LL |         fn clone_from(&mut self, other: _) { *self = FnTest9; }
2020-03-02T20:08:14.9833989Z 531    |                      ^^^                   ^
2020-03-02T20:08:14.9834226Z 532 
2020-03-02T20:08:14.9834532Z 533 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9835314Z +   --> $DIR/typeck_type_placeholder_item.rs:12:15
2020-03-02T20:08:14.9835314Z +   --> $DIR/typeck_type_placeholder_item.rs:12:15
2020-03-02T20:08:14.9835571Z +    |
2020-03-02T20:08:14.9835778Z + LL | static TEST3: _ = "test";
2020-03-02T20:08:14.9836231Z +    |               |
2020-03-02T20:08:14.9836496Z +    |               not allowed in type signatures
2020-03-02T20:08:14.9837102Z +    |               help: replace `_` with the correct type: `&'static str`
2020-03-02T20:08:14.9837405Z + 
2020-03-02T20:08:14.9837405Z + 
2020-03-02T20:08:14.9837714Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9838305Z +   --> $DIR/typeck_type_placeholder_item.rs:15:15
2020-03-02T20:08:14.9838552Z +    |
2020-03-02T20:08:14.9838765Z + LL | static TEST4: _ = 145;
2020-03-02T20:08:14.9839195Z +    |               |
2020-03-02T20:08:14.9839470Z +    |               not allowed in type signatures
2020-03-02T20:08:14.9839850Z +    |               help: replace `_` with the correct type: `i32`
2020-03-02T20:08:14.9840126Z + 
---
2020-03-02T20:08:14.9844786Z +    |         not allowed in type signatures
2020-03-02T20:08:14.9845018Z +    |
2020-03-02T20:08:14.9845224Z + help: use type parameters instead
2020-03-02T20:08:14.9845426Z +    |
2020-03-02T20:08:14.9845620Z + LL | struct Test10<T> {
2020-03-02T20:08:14.9845831Z + LL |     a: T,
2020-03-02T20:08:14.9846002Z + LL |
2020-03-02T20:08:14.9846194Z + LL |     b: (T, T),
2020-03-02T20:08:14.9846508Z + 
2020-03-02T20:08:14.9846713Z + error: missing type for `static` item
2020-03-02T20:08:14.9847232Z +   --> $DIR/typeck_type_placeholder_item.rs:72:12
2020-03-02T20:08:14.9847476Z +    |
2020-03-02T20:08:14.9847476Z +    |
2020-03-02T20:08:14.9847657Z + LL |     static A = 42;
2020-03-02T20:08:14.9847991Z +    |            ^ help: provide a type for the item: `A: i32`
2020-03-02T20:08:14.9848264Z + 
2020-03-02T20:08:14.9848561Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9849221Z +   --> $DIR/typeck_type_placeholder_item.rs:74:15
2020-03-02T20:08:14.9849469Z +    |
2020-03-02T20:08:14.9849666Z + LL |     static B: _ = 42;
2020-03-02T20:08:14.9850112Z +    |               |
2020-03-02T20:08:14.9850373Z +    |               not allowed in type signatures
2020-03-02T20:08:14.9850822Z +    |               help: replace `_` with the correct type: `i32`
2020-03-02T20:08:14.9851101Z + 
2020-03-02T20:08:14.9851101Z + 
2020-03-02T20:08:14.9851399Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9852020Z +   --> $DIR/typeck_type_placeholder_item.rs:76:15
2020-03-02T20:08:14.9852264Z +    |
2020-03-02T20:08:14.9852490Z + LL |     static C: Option<_> = Some(42);
2020-03-02T20:08:14.9853074Z + 
2020-03-02T20:08:14.9853370Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9853968Z +   --> $DIR/typeck_type_placeholder_item.rs:85:22
2020-03-02T20:08:14.9854233Z +    |
2020-03-02T20:08:14.9854233Z +    |
2020-03-02T20:08:14.9854455Z + LL |     static FN_TEST3: _ = "test";
2020-03-02T20:08:14.9854961Z +    |                      |
2020-03-02T20:08:14.9855354Z +    |                      not allowed in type signatures
2020-03-02T20:08:14.9856006Z +    |                      help: replace `_` with the correct type: `&'static str`
2020-03-02T20:08:14.9856334Z + 
2020-03-02T20:08:14.9856334Z + 
2020-03-02T20:08:14.9856631Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9857219Z +   --> $DIR/typeck_type_placeholder_item.rs:88:22
2020-03-02T20:08:14.9858154Z +    |
2020-03-02T20:08:14.9858378Z + LL |     static FN_TEST4: _ = 145;
2020-03-02T20:08:14.9858880Z +    |                      |
2020-03-02T20:08:14.9859166Z +    |                      not allowed in type signatures
2020-03-02T20:08:14.9859571Z +    |                      help: replace `_` with the correct type: `i32`
2020-03-02T20:08:14.9859880Z + 
2020-03-02T20:08:14.9859880Z + 
2020-03-02T20:08:14.9860177Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9860784Z +   --> $DIR/typeck_type_placeholder_item.rs:91:22
2020-03-02T20:08:14.9861049Z +    |
2020-03-02T20:08:14.9861280Z + LL |     static FN_TEST5: (_, _) = (1, 2);
2020-03-02T20:08:14.9861884Z + 
2020-03-02T20:08:14.9862178Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9862773Z +   --> $DIR/typeck_type_placeholder_item.rs:123:12
2020-03-02T20:08:14.9863032Z +    |
---
2020-03-02T20:08:14.9864714Z +    |             not allowed in type signatures
2020-03-02T20:08:14.9864937Z +    |
2020-03-02T20:08:14.9865138Z + help: use type parameters instead
2020-03-02T20:08:14.9865357Z +    |
2020-03-02T20:08:14.9865551Z + LL |     struct FnTest10<T> {
2020-03-02T20:08:14.9865781Z + LL |         a: T,
2020-03-02T20:08:14.9865974Z + LL |
2020-03-02T20:08:14.9866162Z + LL |         b: (T, T),
2020-03-02T20:08:14.9866483Z + 
2020-03-02T20:08:14.9866796Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9867403Z +   --> $DIR/typeck_type_placeholder_item.rs:154:21
2020-03-02T20:08:14.9867647Z +    |
2020-03-02T20:08:14.9867647Z +    |
2020-03-02T20:08:14.9867854Z + LL | struct BadStruct<_>(_);
2020-03-02T20:08:14.9868381Z +    |
2020-03-02T20:08:14.9868671Z + help: use type parameters instead
2020-03-02T20:08:14.9868876Z +    |
2020-03-02T20:08:14.9868876Z +    |
2020-03-02T20:08:14.9869064Z + LL | struct BadStruct<T>(T);
2020-03-02T20:08:14.9869488Z + 
2020-03-02T20:08:14.9869784Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9870459Z +   --> $DIR/typeck_type_placeholder_item.rs:159:32
2020-03-02T20:08:14.9870709Z +    |
2020-03-02T20:08:14.9870709Z +    |
2020-03-02T20:08:14.9870924Z + LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:14.9871524Z + 
2020-03-02T20:08:14.9871818Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9872413Z +   --> $DIR/typeck_type_placeholder_item.rs:159:15
2020-03-02T20:08:14.9872674Z +    |
2020-03-02T20:08:14.9872674Z +    |
2020-03-02T20:08:14.9872890Z + LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:14.9873436Z + 
2020-03-02T20:08:14.9873729Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9874322Z +   --> $DIR/typeck_type_placeholder_item.rs:159:15
2020-03-02T20:08:14.9874586Z +    |
2020-03-02T20:08:14.9874586Z +    |
2020-03-02T20:08:14.9874803Z + LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:14.9875442Z +    |               |
2020-03-02T20:08:14.9875700Z +    |               not allowed in type signatures
2020-03-02T20:08:14.9875925Z +    |
2020-03-02T20:08:14.9876141Z + help: use type parameters instead
2020-03-02T20:08:14.9876141Z + help: use type parameters instead
2020-03-02T20:08:14.9876345Z +    |
2020-03-02T20:08:14.9876566Z + LL | impl<T> BadTrait<T> for BadStruct<T> {}
2020-03-02T20:08:14.9877071Z + 
2020-03-02T20:08:14.9877372Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9877991Z 534   --> $DIR/typeck_type_placeholder_item.rs:164:34
2020-03-02T20:08:14.9878246Z 535    |
2020-03-02T20:08:14.9878246Z 535    |
2020-03-02T20:08:14.9878654Z 536 LL | fn impl_trait() -> impl BadTrait<_> {
2020-03-02T20:08:14.9879120Z 537    |                                  ^ not allowed in type signatures
2020-03-02T20:08:14.9879390Z 538 
2020-03-02T20:08:14.9879697Z 539 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9880317Z +   --> $DIR/typeck_type_placeholder_item.rs:170:25
2020-03-02T20:08:14.9880317Z +   --> $DIR/typeck_type_placeholder_item.rs:170:25
2020-03-02T20:08:14.9880567Z +    |
2020-03-02T20:08:14.9880795Z + LL | struct BadStruct1<_, _>(_);
2020-03-02T20:08:14.9881361Z +    |
2020-03-02T20:08:14.9881562Z + help: use type parameters instead
2020-03-02T20:08:14.9881778Z +    |
2020-03-02T20:08:14.9881778Z +    |
2020-03-02T20:08:14.9881975Z + LL | struct BadStruct1<T, _>(T);
2020-03-02T20:08:14.9882409Z + 
2020-03-02T20:08:14.9882720Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9883319Z +   --> $DIR/typeck_type_placeholder_item.rs:175:25
2020-03-02T20:08:14.9883564Z +    |
2020-03-02T20:08:14.9883564Z +    |
2020-03-02T20:08:14.9883787Z + LL | struct BadStruct2<_, T>(_, T);
2020-03-02T20:08:14.9884349Z +    |
2020-03-02T20:08:14.9884565Z + help: use type parameters instead
2020-03-02T20:08:14.9884766Z +    |
2020-03-02T20:08:14.9884766Z +    |
2020-03-02T20:08:14.9884968Z + LL | struct BadStruct2<K, T>(K, T);
2020-03-02T20:08:14.9885415Z + 
2020-03-02T20:08:14.9885713Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9886323Z +   --> $DIR/typeck_type_placeholder_item.rs:179:14
2020-03-02T20:08:14.9886570Z +    |
2020-03-02T20:08:14.9886570Z +    |
2020-03-02T20:08:14.9886749Z + LL | type X = Box<_>;
2020-03-02T20:08:14.9887306Z + 
2020-03-02T20:08:14.9887600Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9888218Z +   --> $DIR/typeck_type_placeholder_item.rs:179:14
2020-03-02T20:08:14.9888526Z +    |
2020-03-02T20:08:14.9888526Z +    |
2020-03-02T20:08:14.9888701Z + LL | type X = Box<_>;
2020-03-02T20:08:14.9889201Z + 
2020-03-02T20:08:14.9889496Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9890097Z +   --> $DIR/typeck_type_placeholder_item.rs:164:34
2020-03-02T20:08:14.9890358Z +    |
2020-03-02T20:08:14.9890358Z +    |
2020-03-02T20:08:14.9890751Z + LL | fn impl_trait() -> impl BadTrait<_> {
2020-03-02T20:08:14.9891366Z + 
2020-03-02T20:08:14.9891663Z + error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9892267Z 540   --> $DIR/typeck_type_placeholder_item.rs:186:21
2020-03-02T20:08:14.9892534Z 541    |
2020-03-02T20:08:14.9892534Z 541    |
2020-03-02T20:08:14.9892734Z 542 LL | type Y = impl Trait<_>;
2020-03-02T20:08:14.9893069Z 568    |              |
2020-03-02T20:08:14.9893344Z 569    |              not allowed in type signatures
2020-03-02T20:08:14.9893723Z 570    |              help: replace `_` with the correct type: `i32`
2020-03-02T20:08:14.9894148Z - 
2020-03-02T20:08:14.9894148Z - 
2020-03-02T20:08:14.9894684Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9895380Z -   --> $DIR/typeck_type_placeholder_item.rs:40:24
2020-03-02T20:08:14.9896421Z -    |
2020-03-02T20:08:14.9896852Z - LL |     fn test9(&self) -> _ { () }
2020-03-02T20:08:14.9897681Z -    |                        |
2020-03-02T20:08:14.9898193Z -    |                        not allowed in type signatures
2020-03-02T20:08:14.9898825Z -    |                        help: replace with the correct return type: `()`
2020-03-02T20:08:14.9899269Z - 
2020-03-02T20:08:14.9899269Z - 
2020-03-02T20:08:14.9899807Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9900406Z -   --> $DIR/typeck_type_placeholder_item.rs:58:24
2020-03-02T20:08:14.9900793Z -    |
2020-03-02T20:08:14.9901202Z - LL |     fn clone(&self) -> _ { Test9 }
2020-03-02T20:08:14.9902034Z -    |                        |
2020-03-02T20:08:14.9902540Z -    |                        not allowed in type signatures
2020-03-02T20:08:14.9903179Z -    |                        help: replace with the correct return type: `Test9`
2020-03-02T20:08:14.9903627Z - 
2020-03-02T20:08:14.9903627Z - 
2020-03-02T20:08:14.9904161Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9904760Z -   --> $DIR/typeck_type_placeholder_item.rs:107:31
2020-03-02T20:08:14.9905154Z -    |
2020-03-02T20:08:14.9905575Z - LL |         fn fn_test9(&self) -> _ { () }
2020-03-02T20:08:14.9906469Z -    |                               |
2020-03-02T20:08:14.9906988Z -    |                               not allowed in type signatures
2020-03-02T20:08:14.9907675Z -    |                               help: replace with the correct return type: `()`
2020-03-02T20:08:14.9908139Z - 
2020-03-02T20:08:14.9908139Z - 
2020-03-02T20:08:14.9908659Z - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9909273Z -   --> $DIR/typeck_type_placeholder_item.rs:115:28
2020-03-02T20:08:14.9909662Z -    |
2020-03-02T20:08:14.9910071Z - LL |         fn clone(&self) -> _ { FnTest9 }
2020-03-02T20:08:14.9910962Z -    |                            |
2020-03-02T20:08:14.9911469Z -    |                            not allowed in type signatures
2020-03-02T20:08:14.9912260Z -    |                            help: replace with the correct return type: `main::FnTest9`
2020-03-02T20:08:14.9912610Z 607 
2020-03-02T20:08:14.9912610Z 607 
2020-03-02T20:08:14.9912918Z 608 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9913610Z 609   --> $DIR/typeck_type_placeholder_item.rs:204:14
2020-03-02T20:08:14.9913834Z 
2020-03-02T20:08:14.9913938Z 
2020-03-02T20:08:14.9914157Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:14.9914940Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/typeck_type_placeholder_item.stderr
2020-03-02T20:08:14.9915663Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:14.9916342Z To only update this specific test, also pass `--test-args typeck/typeck_type_placeholder_item.rs`
2020-03-02T20:08:14.9916828Z error: 1 errors occurred comparing output.
2020-03-02T20:08:14.9917082Z status: exit code: 1
2020-03-02T20:08:14.9917082Z status: exit code: 1
2020-03-02T20:08:14.9919238Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/auxiliary"
2020-03-02T20:08:14.9921002Z ------------------------------------------
2020-03-02T20:08:14.9921187Z 
2020-03-02T20:08:14.9921580Z ------------------------------------------
2020-03-02T20:08:14.9921795Z stderr:
2020-03-02T20:08:14.9921795Z stderr:
2020-03-02T20:08:14.9922181Z ------------------------------------------
2020-03-02T20:08:14.9922499Z error: expected identifier, found reserved identifier `_`
2020-03-02T20:08:14.9923113Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:154:18
2020-03-02T20:08:14.9923402Z    |
2020-03-02T20:08:14.9923585Z LL | struct BadStruct<_>(_);
2020-03-02T20:08:14.9924146Z 
2020-03-02T20:08:14.9924378Z error: expected identifier, found reserved identifier `_`
2020-03-02T20:08:14.9924986Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:157:16
2020-03-02T20:08:14.9925268Z    |
2020-03-02T20:08:14.9925268Z    |
2020-03-02T20:08:14.9925445Z LL | trait BadTrait<_> {}
2020-03-02T20:08:14.9925981Z 
2020-03-02T20:08:14.9926209Z error: expected identifier, found reserved identifier `_`
2020-03-02T20:08:14.9926800Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:170:19
2020-03-02T20:08:14.9927102Z    |
2020-03-02T20:08:14.9927102Z    |
2020-03-02T20:08:14.9927295Z LL | struct BadStruct1<_, _>(_);
2020-03-02T20:08:14.9927858Z 
2020-03-02T20:08:14.9928089Z error: expected identifier, found reserved identifier `_`
2020-03-02T20:08:14.9928687Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:170:22
2020-03-02T20:08:14.9928983Z    |
2020-03-02T20:08:14.9928983Z    |
2020-03-02T20:08:14.9929177Z LL | struct BadStruct1<_, _>(_);
2020-03-02T20:08:14.9929738Z 
2020-03-02T20:08:14.9929982Z error: expected identifier, found reserved identifier `_`
2020-03-02T20:08:14.9930574Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:175:19
2020-03-02T20:08:14.9930858Z    |
2020-03-02T20:08:14.9930858Z    |
2020-03-02T20:08:14.9931071Z LL | struct BadStruct2<_, T>(_, T);
2020-03-02T20:08:14.9931679Z 
2020-03-02T20:08:14.9931919Z error: associated constant in `impl` without body
2020-03-02T20:08:14.9932506Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:208:5
2020-03-02T20:08:14.9932791Z    |
2020-03-02T20:08:14.9932791Z    |
2020-03-02T20:08:14.9932970Z LL |     const C: _;
2020-03-02T20:08:14.9933418Z    |     ^^^^^^^^^^-
2020-03-02T20:08:14.9933619Z    |               |
2020-03-02T20:08:14.9933952Z    |               help: provide a definition for the constant: `= <expr>;`
2020-03-02T20:08:14.9934247Z 
2020-03-02T20:08:14.9934801Z error[E0403]: the name `_` is already used for a generic parameter in this item's generic parameters
2020-03-02T20:08:14.9935906Z    |
2020-03-02T20:08:14.9935906Z    |
2020-03-02T20:08:14.9936096Z LL | struct BadStruct1<_, _>(_);
2020-03-02T20:08:14.9936530Z    |                   -  ^ already used
2020-03-02T20:08:14.9937033Z    |                   first use of `_`
2020-03-02T20:08:14.9937209Z 
2020-03-02T20:08:14.9937512Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9938156Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:6:14
---
2020-03-02T20:08:14.9940093Z 
2020-03-02T20:08:14.9940382Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9941036Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:9:16
2020-03-02T20:08:14.9941311Z    |
2020-03-02T20:08:14.9941682Z LL | fn test2() -> (_, _) { (5, 5) }
2020-03-02T20:08:14.9942095Z    |               -^--^-
2020-03-02T20:08:14.9942320Z    |               ||  |
2020-03-02T20:08:14.9942587Z    |               ||  not allowed in type signatures
2020-03-02T20:08:14.9942915Z    |               |not allowed in type signatures
2020-03-02T20:08:14.9943305Z    |               help: replace with the correct return type: `(i32, i32)`
2020-03-02T20:08:14.9943864Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9944529Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:21:13
2020-03-02T20:08:14.9944810Z    |
2020-03-02T20:08:14.9944810Z    |
2020-03-02T20:08:14.9944993Z LL | fn test6(_: _) { }
2020-03-02T20:08:14.9945492Z    |
2020-03-02T20:08:14.9945685Z help: use type parameters instead
2020-03-02T20:08:14.9945890Z    |
2020-03-02T20:08:14.9946080Z LL | fn test6<T>(_: T) { }
2020-03-02T20:08:14.9946080Z LL | fn test6<T>(_: T) { }
2020-03-02T20:08:14.9946301Z    |         ^^^    ^
2020-03-02T20:08:14.9946446Z 
2020-03-02T20:08:14.9946754Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9947401Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:24:18
2020-03-02T20:08:14.9947682Z    |
2020-03-02T20:08:14.9947909Z LL | fn test6_b<T>(_: _, _: T) { }
2020-03-02T20:08:14.9948450Z    |
2020-03-02T20:08:14.9948658Z help: use type parameters instead
2020-03-02T20:08:14.9948851Z    |
2020-03-02T20:08:14.9948851Z    |
2020-03-02T20:08:14.9949066Z LL | fn test6_b<T, K>(_: K, _: T) { }
2020-03-02T20:08:14.9949487Z 
2020-03-02T20:08:14.9949773Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9950417Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:27:30
2020-03-02T20:08:14.9950709Z    |
2020-03-02T20:08:14.9950709Z    |
2020-03-02T20:08:14.9950973Z LL | fn test6_c<T, K, L, A, B>(_: _, _: (T, K, L, A, B)) { }
2020-03-02T20:08:14.9951680Z    |
2020-03-02T20:08:14.9951874Z help: use type parameters instead
2020-03-02T20:08:14.9952067Z    |
2020-03-02T20:08:14.9952067Z    |
2020-03-02T20:08:14.9952351Z LL | fn test6_c<T, K, L, A, B, C>(_: C, _: (T, K, L, A, B)) { }
2020-03-02T20:08:14.9952906Z 
2020-03-02T20:08:14.9953195Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9953872Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:30:13
2020-03-02T20:08:14.9954151Z    |
2020-03-02T20:08:14.9954151Z    |
2020-03-02T20:08:14.9954378Z LL | fn test7(x: _) { let _x: usize = x; }
2020-03-02T20:08:14.9954923Z    |
2020-03-02T20:08:14.9955116Z help: use type parameters instead
2020-03-02T20:08:14.9955320Z    |
2020-03-02T20:08:14.9955320Z    |
2020-03-02T20:08:14.9955552Z LL | fn test7<T>(x: T) { let _x: usize = x; }
2020-03-02T20:08:14.9955964Z 
2020-03-02T20:08:14.9956266Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9956918Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:33:22
2020-03-02T20:08:14.9957204Z    |
2020-03-02T20:08:14.9957204Z    |
2020-03-02T20:08:14.9957586Z LL | fn test8(_f: fn() -> _) { }
2020-03-02T20:08:14.9958104Z 
2020-03-02T20:08:14.9958407Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9959055Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:33:22
2020-03-02T20:08:14.9959333Z    |
2020-03-02T20:08:14.9959333Z    |
2020-03-02T20:08:14.9959715Z LL | fn test8(_f: fn() -> _) { }
2020-03-02T20:08:14.9960253Z    |
2020-03-02T20:08:14.9960459Z help: use type parameters instead
2020-03-02T20:08:14.9960653Z    |
2020-03-02T20:08:14.9960653Z    |
2020-03-02T20:08:14.9961035Z LL | fn test8<T>(_f: fn() -> T) { }
2020-03-02T20:08:14.9961469Z 
2020-03-02T20:08:14.9976757Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9978370Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:47:26
2020-03-02T20:08:14.9978865Z    |
2020-03-02T20:08:14.9978865Z    |
2020-03-02T20:08:14.9979556Z LL | fn test11(x: &usize) -> &_ {
2020-03-02T20:08:14.9980318Z    |                         -^
2020-03-02T20:08:14.9981226Z    |                         |not allowed in type signatures
2020-03-02T20:08:14.9981226Z    |                         |not allowed in type signatures
2020-03-02T20:08:14.9982255Z    |                         help: replace with the correct return type: `&&usize`
2020-03-02T20:08:14.9983261Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9984434Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:52:52
2020-03-02T20:08:14.9984926Z    |
2020-03-02T20:08:14.9984926Z    |
2020-03-02T20:08:14.9985705Z LL | unsafe fn test12(x: *const usize) -> *const *const _ {
2020-03-02T20:08:14.9987229Z    |                                      |             |
2020-03-02T20:08:14.9987900Z    |                                      |             not allowed in type signatures
2020-03-02T20:08:14.9987900Z    |                                      |             not allowed in type signatures
2020-03-02T20:08:14.9988806Z    |                                      help: replace with the correct return type: `*const *const usize`
2020-03-02T20:08:14.9989903Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9991040Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:79:21
2020-03-02T20:08:14.9991521Z    |
2020-03-02T20:08:14.9991521Z    |
2020-03-02T20:08:14.9992142Z LL |     fn fn_test() -> _ { 5 }
2020-03-02T20:08:14.9992941Z    |                     |
2020-03-02T20:08:14.9993677Z    |                     not allowed in type signatures
2020-03-02T20:08:14.9994369Z    |                     help: replace with the correct return type: `i32`
2020-03-02T20:08:14.9994859Z 
2020-03-02T20:08:14.9994859Z 
2020-03-02T20:08:14.9995349Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:14.9996598Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:82:23
2020-03-02T20:08:14.9997096Z    |
2020-03-02T20:08:14.9997780Z LL |     fn fn_test2() -> (_, _) { (5, 5) }
2020-03-02T20:08:14.9998525Z    |                      -^--^-
2020-03-02T20:08:14.9998959Z    |                      ||  |
2020-03-02T20:08:14.9999457Z    |                      ||  not allowed in type signatures
2020-03-02T20:08:15.0000040Z    |                      |not allowed in type signatures
2020-03-02T20:08:15.0000770Z    |                      help: replace with the correct return type: `(i32, i32)`
2020-03-02T20:08:15.0001768Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0002882Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:94:20
2020-03-02T20:08:15.0003383Z    |
2020-03-02T20:08:15.0003383Z    |
2020-03-02T20:08:15.0003727Z LL |     fn fn_test6(_: _) { }
2020-03-02T20:08:15.0004664Z    |
2020-03-02T20:08:15.0004994Z help: use type parameters instead
2020-03-02T20:08:15.0005330Z    |
2020-03-02T20:08:15.0005330Z    |
2020-03-02T20:08:15.0005700Z LL |     fn fn_test6<T>(_: T) { }
2020-03-02T20:08:15.0006397Z 
2020-03-02T20:08:15.0006888Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0007809Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:97:20
2020-03-02T20:08:15.0008090Z    |
2020-03-02T20:08:15.0008090Z    |
2020-03-02T20:08:15.0008334Z LL |     fn fn_test7(x: _) { let _x: usize = x; }
2020-03-02T20:08:15.0008923Z    |
2020-03-02T20:08:15.0009116Z help: use type parameters instead
2020-03-02T20:08:15.0009322Z    |
2020-03-02T20:08:15.0009322Z    |
2020-03-02T20:08:15.0009570Z LL |     fn fn_test7<T>(x: T) { let _x: usize = x; }
2020-03-02T20:08:15.0010023Z 
2020-03-02T20:08:15.0010329Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0010982Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:100:29
2020-03-02T20:08:15.0011263Z    |
2020-03-02T20:08:15.0011263Z    |
2020-03-02T20:08:15.0011672Z LL |     fn fn_test8(_f: fn() -> _) { }
2020-03-02T20:08:15.0012229Z 
2020-03-02T20:08:15.0012529Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0013175Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:100:29
2020-03-02T20:08:15.0013465Z    |
2020-03-02T20:08:15.0013465Z    |
2020-03-02T20:08:15.0013872Z LL |     fn fn_test8(_f: fn() -> _) { }
2020-03-02T20:08:15.0014452Z    |
2020-03-02T20:08:15.0014661Z help: use type parameters instead
2020-03-02T20:08:15.0014856Z    |
2020-03-02T20:08:15.0014856Z    |
2020-03-02T20:08:15.0015375Z LL |     fn fn_test8<T>(_f: fn() -> T) { }
2020-03-02T20:08:15.0015858Z 
2020-03-02T20:08:15.0016057Z error[E0282]: type annotations needed
2020-03-02T20:08:15.0016623Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:128:18
2020-03-02T20:08:15.0016920Z    |
2020-03-02T20:08:15.0016920Z    |
2020-03-02T20:08:15.0017347Z LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
2020-03-02T20:08:15.0017841Z 
2020-03-02T20:08:15.0018141Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0018858Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:128:28
2020-03-02T20:08:15.0019147Z    |
2020-03-02T20:08:15.0019147Z    |
2020-03-02T20:08:15.0019598Z LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
2020-03-02T20:08:15.0020322Z    |                            |
2020-03-02T20:08:15.0020638Z    |                            not allowed in type signatures
2020-03-02T20:08:15.0020856Z 
2020-03-02T20:08:15.0021144Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0021144Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0021816Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:132:30
2020-03-02T20:08:15.0022104Z    |
2020-03-02T20:08:15.0022530Z LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }
2020-03-02T20:08:15.0023024Z    |                             -^--^-
2020-03-02T20:08:15.0023291Z    |                             ||  |
2020-03-02T20:08:15.0023605Z    |                             ||  not allowed in type signatures
2020-03-02T20:08:15.0023988Z    |                             |not allowed in type signatures
2020-03-02T20:08:15.0024433Z    |                             help: replace with the correct return type: `(i32, i32)`
2020-03-02T20:08:15.0025022Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0025692Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:135:33
2020-03-02T20:08:15.0025972Z    |
2020-03-02T20:08:15.0025972Z    |
2020-03-02T20:08:15.0026398Z LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
2020-03-02T20:08:15.0027163Z    |                           |     |
2020-03-02T20:08:15.0027481Z    |                           |     not allowed in type signatures
2020-03-02T20:08:15.0027481Z    |                           |     not allowed in type signatures
2020-03-02T20:08:15.0027940Z    |                           help: replace with the correct return type: `(i32, i32)`
2020-03-02T20:08:15.0028532Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0029187Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:40:24
2020-03-02T20:08:15.0029467Z    |
2020-03-02T20:08:15.0029467Z    |
2020-03-02T20:08:15.0029885Z LL |     fn test9(&self) -> _ { () }
2020-03-02T20:08:15.0030385Z    |                        |
2020-03-02T20:08:15.0030670Z    |                        not allowed in type signatures
2020-03-02T20:08:15.0031088Z    |                        help: replace with the correct return type: `()`
2020-03-02T20:08:15.0031366Z 
2020-03-02T20:08:15.0031366Z 
2020-03-02T20:08:15.0031653Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0032297Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:43:27
2020-03-02T20:08:15.0032589Z    |
2020-03-02T20:08:15.0032808Z LL |     fn test10(&self, _x : _) { }
2020-03-02T20:08:15.0033395Z    |
2020-03-02T20:08:15.0033592Z help: use type parameters instead
2020-03-02T20:08:15.0033784Z    |
2020-03-02T20:08:15.0033784Z    |
2020-03-02T20:08:15.0034021Z LL |     fn test10<T>(&self, _x : T) { }
2020-03-02T20:08:15.0034467Z 
2020-03-02T20:08:15.0034758Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0035421Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:140:31
2020-03-02T20:08:15.0035702Z    |
2020-03-02T20:08:15.0035702Z    |
2020-03-02T20:08:15.0035924Z LL |     fn method_test1(&self, x: _);
2020-03-02T20:08:15.0036524Z    |
2020-03-02T20:08:15.0036717Z help: use type parameters instead
2020-03-02T20:08:15.0036923Z    |
2020-03-02T20:08:15.0036923Z    |
2020-03-02T20:08:15.0037151Z LL |     fn method_test1<T>(&self, x: T);
2020-03-02T20:08:15.0037617Z 
2020-03-02T20:08:15.0037974Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0038637Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:142:31
2020-03-02T20:08:15.0038920Z    |
2020-03-02T20:08:15.0038920Z    |
2020-03-02T20:08:15.0039345Z LL |     fn method_test2(&self, x: _) -> _;
2020-03-02T20:08:15.0040083Z    |                               |
2020-03-02T20:08:15.0040408Z    |                               not allowed in type signatures
2020-03-02T20:08:15.0040660Z    |
2020-03-02T20:08:15.0040853Z help: use type parameters instead
2020-03-02T20:08:15.0040853Z help: use type parameters instead
2020-03-02T20:08:15.0041057Z    |
2020-03-02T20:08:15.0041485Z LL |     fn method_test2<T>(&self, x: T) -> T;
2020-03-02T20:08:15.0041998Z 
2020-03-02T20:08:15.0042285Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0042936Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:144:31
2020-03-02T20:08:15.0043219Z    |
---
2020-03-02T20:08:15.0045426Z 
2020-03-02T20:08:15.0045716Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0046375Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:146:26
2020-03-02T20:08:15.0046657Z    |
2020-03-02T20:08:15.0046864Z LL |     fn assoc_fn_test1(x: _);
2020-03-02T20:08:15.0047441Z    |
2020-03-02T20:08:15.0047636Z help: use type parameters instead
2020-03-02T20:08:15.0047828Z    |
2020-03-02T20:08:15.0047828Z    |
2020-03-02T20:08:15.0048056Z LL |     fn assoc_fn_test1<T>(x: T);
2020-03-02T20:08:15.0048490Z 
2020-03-02T20:08:15.0048793Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0049451Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:148:26
2020-03-02T20:08:15.0049734Z    |
2020-03-02T20:08:15.0049734Z    |
2020-03-02T20:08:15.0050137Z LL |     fn assoc_fn_test2(x: _) -> _;
2020-03-02T20:08:15.0050782Z    |                          |
2020-03-02T20:08:15.0051076Z    |                          not allowed in type signatures
2020-03-02T20:08:15.0051332Z    |
2020-03-02T20:08:15.0051524Z help: use type parameters instead
2020-03-02T20:08:15.0051524Z help: use type parameters instead
2020-03-02T20:08:15.0051715Z    |
2020-03-02T20:08:15.0052132Z LL |     fn assoc_fn_test2<T>(x: T) -> T;
2020-03-02T20:08:15.0052610Z 
2020-03-02T20:08:15.0052915Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0053561Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:150:28
2020-03-02T20:08:15.0053850Z    |
---
2020-03-02T20:08:15.0056115Z 
2020-03-02T20:08:15.0056402Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0057046Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:58:24
2020-03-02T20:08:15.0057342Z    |
2020-03-02T20:08:15.0057786Z LL |     fn clone(&self) -> _ { Test9 }
2020-03-02T20:08:15.0058291Z    |                        |
2020-03-02T20:08:15.0058580Z    |                        not allowed in type signatures
2020-03-02T20:08:15.0059046Z    |                        help: replace with the correct return type: `Test9`
2020-03-02T20:08:15.0059329Z 
2020-03-02T20:08:15.0059329Z 
2020-03-02T20:08:15.0059630Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0060287Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:61:37
2020-03-02T20:08:15.0060568Z    |
2020-03-02T20:08:15.0060851Z LL |     fn clone_from(&mut self, other: _) { *self = Test9; }
2020-03-02T20:08:15.0061503Z    |
2020-03-02T20:08:15.0061711Z help: use type parameters instead
2020-03-02T20:08:15.0061901Z    |
2020-03-02T20:08:15.0061901Z    |
2020-03-02T20:08:15.0062177Z LL |     fn clone_from<T>(&mut self, other: T) { *self = Test9; }
2020-03-02T20:08:15.0062730Z 
2020-03-02T20:08:15.0063020Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0063673Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:107:31
2020-03-02T20:08:15.0063971Z    |
2020-03-02T20:08:15.0063971Z    |
2020-03-02T20:08:15.0064363Z LL |         fn fn_test9(&self) -> _ { () }
2020-03-02T20:08:15.0064906Z    |                               |
2020-03-02T20:08:15.0065214Z    |                               not allowed in type signatures
2020-03-02T20:08:15.0065645Z    |                               help: replace with the correct return type: `()`
2020-03-02T20:08:15.0065958Z 
2020-03-02T20:08:15.0065958Z 
2020-03-02T20:08:15.0066244Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0066897Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:110:34
2020-03-02T20:08:15.0067194Z    |
2020-03-02T20:08:15.0067428Z LL |         fn fn_test10(&self, _x : _) { }
2020-03-02T20:08:15.0068054Z    |
2020-03-02T20:08:15.0068247Z help: use type parameters instead
2020-03-02T20:08:15.0068437Z    |
2020-03-02T20:08:15.0068437Z    |
2020-03-02T20:08:15.0068679Z LL |         fn fn_test10<T>(&self, _x : T) { }
2020-03-02T20:08:15.0069185Z 
2020-03-02T20:08:15.0069471Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0070135Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:115:28
2020-03-02T20:08:15.0070419Z    |
2020-03-02T20:08:15.0070419Z    |
2020-03-02T20:08:15.0070813Z LL |         fn clone(&self) -> _ { FnTest9 }
2020-03-02T20:08:15.0071343Z    |                            |
2020-03-02T20:08:15.0071645Z    |                            not allowed in type signatures
2020-03-02T20:08:15.0072106Z    |                            help: replace with the correct return type: `main::FnTest9`
2020-03-02T20:08:15.0072415Z 
2020-03-02T20:08:15.0072415Z 
2020-03-02T20:08:15.0072707Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0073360Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:118:41
2020-03-02T20:08:15.0073658Z    |
2020-03-02T20:08:15.0073935Z LL |         fn clone_from(&mut self, other: _) { *self = FnTest9; }
2020-03-02T20:08:15.0074631Z    |
2020-03-02T20:08:15.0074824Z help: use type parameters instead
2020-03-02T20:08:15.0075015Z    |
2020-03-02T20:08:15.0075015Z    |
2020-03-02T20:08:15.0075315Z LL |         fn clone_from<T>(&mut self, other: T) { *self = FnTest9; }
2020-03-02T20:08:15.0075932Z 
2020-03-02T20:08:15.0076238Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0076895Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:12:15
2020-03-02T20:08:15.0077232Z    |
2020-03-02T20:08:15.0077232Z    |
2020-03-02T20:08:15.0077441Z LL | static TEST3: _ = "test";
2020-03-02T20:08:15.0077870Z    |               |
2020-03-02T20:08:15.0078124Z    |               not allowed in type signatures
2020-03-02T20:08:15.0078737Z    |               help: replace `_` with the correct type: `&'static str`
2020-03-02T20:08:15.0079010Z 
---
2020-03-02T20:08:15.0084647Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:66:8
2020-03-02T20:08:15.0084939Z    |
2020-03-02T20:08:15.0085105Z LL |     a: _,
2020-03-02T20:08:15.0085345Z    |        ^ not allowed in type signatures
2020-03-02T20:08:15.0085723Z LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0086068Z LL |     b: (_, _),
2020-03-02T20:08:15.0086565Z    |         |
2020-03-02T20:08:15.0086810Z    |         not allowed in type signatures
2020-03-02T20:08:15.0087019Z    |
2020-03-02T20:08:15.0087212Z help: use type parameters instead
2020-03-02T20:08:15.0087212Z help: use type parameters instead
2020-03-02T20:08:15.0087417Z    |
2020-03-02T20:08:15.0087590Z LL | struct Test10<T> {
2020-03-02T20:08:15.0087785Z LL |     a: T,
2020-03-02T20:08:15.0088129Z LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0088458Z LL |     b: (T, T),
2020-03-02T20:08:15.0088740Z 
2020-03-02T20:08:15.0088949Z error: missing type for `static` item
2020-03-02T20:08:15.0089506Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:72:12
2020-03-02T20:08:15.0089791Z    |
2020-03-02T20:08:15.0089791Z    |
2020-03-02T20:08:15.0089980Z LL |     static A = 42;
2020-03-02T20:08:15.0090292Z    |            ^ help: provide a type for the item: `A: i32`
2020-03-02T20:08:15.0090538Z 
2020-03-02T20:08:15.0090840Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0091492Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:74:15
2020-03-02T20:08:15.0091770Z    |
2020-03-02T20:08:15.0091960Z LL |     static B: _ = 42;
2020-03-02T20:08:15.0092393Z    |               |
2020-03-02T20:08:15.0092644Z    |               not allowed in type signatures
2020-03-02T20:08:15.0093024Z    |               help: replace `_` with the correct type: `i32`
2020-03-02T20:08:15.0093277Z 
---
2020-03-02T20:08:15.0095466Z 
2020-03-02T20:08:15.0095756Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0096482Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:85:22
2020-03-02T20:08:15.0096777Z    |
2020-03-02T20:08:15.0096988Z LL |     static FN_TEST3: _ = "test";
2020-03-02T20:08:15.0097482Z    |                      |
2020-03-02T20:08:15.0097755Z    |                      not allowed in type signatures
2020-03-02T20:08:15.0098381Z    |                      help: replace `_` with the correct type: `&'static str`
2020-03-02T20:08:15.0098682Z 
2020-03-02T20:08:15.0098682Z 
2020-03-02T20:08:15.0098967Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0099614Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:88:22
2020-03-02T20:08:15.0099909Z    |
2020-03-02T20:08:15.0100119Z LL |     static FN_TEST4: _ = 145;
2020-03-02T20:08:15.0100593Z    |                      |
2020-03-02T20:08:15.0100889Z    |                      not allowed in type signatures
2020-03-02T20:08:15.0101279Z    |                      help: replace `_` with the correct type: `i32`
2020-03-02T20:08:15.0101545Z 
2020-03-02T20:08:15.0101545Z 
2020-03-02T20:08:15.0101845Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0102490Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:91:22
2020-03-02T20:08:15.0102770Z    |
2020-03-02T20:08:15.0103008Z LL |     static FN_TEST5: (_, _) = (1, 2);
2020-03-02T20:08:15.0103558Z 
2020-03-02T20:08:15.0103849Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0104511Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:123:12
2020-03-02T20:08:15.0104793Z    |
2020-03-02T20:08:15.0104793Z    |
2020-03-02T20:08:15.0104963Z LL |         a: _,
2020-03-02T20:08:15.0105235Z    |            ^ not allowed in type signatures
2020-03-02T20:08:15.0105634Z LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0105986Z LL |         b: (_, _),
2020-03-02T20:08:15.0106527Z    |             |
2020-03-02T20:08:15.0106771Z    |             not allowed in type signatures
2020-03-02T20:08:15.0106999Z    |
2020-03-02T20:08:15.0107194Z help: use type parameters instead
2020-03-02T20:08:15.0107194Z help: use type parameters instead
2020-03-02T20:08:15.0107386Z    |
2020-03-02T20:08:15.0107584Z LL |     struct FnTest10<T> {
2020-03-02T20:08:15.0107801Z LL |         a: T,
2020-03-02T20:08:15.0108146Z LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0108508Z LL |         b: (T, T),
2020-03-02T20:08:15.0108797Z 
2020-03-02T20:08:15.0109085Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0109757Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:154:21
2020-03-02T20:08:15.0110040Z    |
2020-03-02T20:08:15.0110040Z    |
2020-03-02T20:08:15.0110222Z LL | struct BadStruct<_>(_);
2020-03-02T20:08:15.0110753Z    |
2020-03-02T20:08:15.0110948Z help: use type parameters instead
2020-03-02T20:08:15.0111153Z    |
2020-03-02T20:08:15.0111153Z    |
2020-03-02T20:08:15.0111335Z LL | struct BadStruct<T>(T);
2020-03-02T20:08:15.0111707Z 
2020-03-02T20:08:15.0112009Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0112731Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:159:32
2020-03-02T20:08:15.0113018Z    |
2020-03-02T20:08:15.0113018Z    |
2020-03-02T20:08:15.0113238Z LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:15.0113786Z 
2020-03-02T20:08:15.0114134Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0114798Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:159:15
2020-03-02T20:08:15.0115081Z    |
2020-03-02T20:08:15.0115081Z    |
2020-03-02T20:08:15.0115301Z LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:15.0115795Z 
2020-03-02T20:08:15.0116083Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0116745Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:159:15
2020-03-02T20:08:15.0117025Z    |
2020-03-02T20:08:15.0117025Z    |
2020-03-02T20:08:15.0117231Z LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-02T20:08:15.0117864Z    |               |
2020-03-02T20:08:15.0118116Z    |               not allowed in type signatures
2020-03-02T20:08:15.0118349Z    |
2020-03-02T20:08:15.0118541Z help: use type parameters instead
2020-03-02T20:08:15.0118541Z help: use type parameters instead
2020-03-02T20:08:15.0118739Z    |
2020-03-02T20:08:15.0118964Z LL | impl<T> BadTrait<T> for BadStruct<T> {}
2020-03-02T20:08:15.0119425Z 
2020-03-02T20:08:15.0119713Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0120376Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:164:34
2020-03-02T20:08:15.0120954Z    |
2020-03-02T20:08:15.0120954Z    |
2020-03-02T20:08:15.0121371Z LL | fn impl_trait() -> impl BadTrait<_> {
2020-03-02T20:08:15.0121955Z 
2020-03-02T20:08:15.0122246Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0122907Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:170:25
2020-03-02T20:08:15.0123191Z    |
2020-03-02T20:08:15.0123191Z    |
2020-03-02T20:08:15.0123382Z LL | struct BadStruct1<_, _>(_);
2020-03-02T20:08:15.0123941Z    |
2020-03-02T20:08:15.0124134Z help: use type parameters instead
2020-03-02T20:08:15.0124329Z    |
2020-03-02T20:08:15.0124329Z    |
2020-03-02T20:08:15.0124532Z LL | struct BadStruct1<T, _>(T);
2020-03-02T20:08:15.0124929Z 
2020-03-02T20:08:15.0125232Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0125882Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:175:25
2020-03-02T20:08:15.0126163Z    |
2020-03-02T20:08:15.0126163Z    |
2020-03-02T20:08:15.0126375Z LL | struct BadStruct2<_, T>(_, T);
2020-03-02T20:08:15.0126928Z    |
2020-03-02T20:08:15.0127121Z help: use type parameters instead
2020-03-02T20:08:15.0127325Z    |
2020-03-02T20:08:15.0127325Z    |
2020-03-02T20:08:15.0127520Z LL | struct BadStruct2<K, T>(K, T);
2020-03-02T20:08:15.0127944Z 
2020-03-02T20:08:15.0128234Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0128920Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:179:14
2020-03-02T20:08:15.0129217Z    |
2020-03-02T20:08:15.0129217Z    |
2020-03-02T20:08:15.0129389Z LL | type X = Box<_>;
2020-03-02T20:08:15.0129843Z 
2020-03-02T20:08:15.0130143Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0130793Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:179:14
2020-03-02T20:08:15.0131074Z    |
2020-03-02T20:08:15.0131074Z    |
2020-03-02T20:08:15.0131259Z LL | type X = Box<_>;
2020-03-02T20:08:15.0131788Z 
2020-03-02T20:08:15.0132074Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0132746Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:164:34
2020-03-02T20:08:15.0133087Z    |
2020-03-02T20:08:15.0133087Z    |
2020-03-02T20:08:15.0133485Z LL | fn impl_trait() -> impl BadTrait<_> {
2020-03-02T20:08:15.0134064Z 
2020-03-02T20:08:15.0134352Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0135010Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:186:21
2020-03-02T20:08:15.0135401Z    |
2020-03-02T20:08:15.0135401Z    |
2020-03-02T20:08:15.0135588Z LL | type Y = impl Trait<_>;
2020-03-02T20:08:15.0136095Z 
2020-03-02T20:08:15.0136387Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0137050Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:186:21
2020-03-02T20:08:15.0137346Z    |
2020-03-02T20:08:15.0137346Z    |
2020-03-02T20:08:15.0137532Z LL | type Y = impl Trait<_>;
2020-03-02T20:08:15.0138040Z 
2020-03-02T20:08:15.0138327Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0138975Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:195:14
2020-03-02T20:08:15.0139270Z    |
---
2020-03-02T20:08:15.0141764Z 
2020-03-02T20:08:15.0142068Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0142729Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:199:14
2020-03-02T20:08:15.0143009Z    |
2020-03-02T20:08:15.0143210Z LL |     const D: _ = 42;
2020-03-02T20:08:15.0143621Z    |              |
2020-03-02T20:08:15.0143870Z    |              not allowed in type signatures
2020-03-02T20:08:15.0144245Z    |              help: replace `_` with the correct type: `i32`
2020-03-02T20:08:15.0144495Z 
---
2020-03-02T20:08:15.0150080Z 
2020-03-02T20:08:15.0150441Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-02T20:08:15.0151102Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:211:14
2020-03-02T20:08:15.0151381Z    |
2020-03-02T20:08:15.0151582Z LL |     const D: _ = 42;
2020-03-02T20:08:15.0152047Z    |              |
2020-03-02T20:08:15.0152311Z    |              not allowed in type signatures
2020-03-02T20:08:15.0152670Z    |              help: replace `_` with the correct type: `i32`
2020-03-02T20:08:15.0152923Z 
---
2020-03-02T20:08:15.0155065Z 
2020-03-02T20:08:15.0155534Z ---- [ui] ui/unboxed-closures/unboxed-closure-sugar-region.rs stdout ----
2020-03-02T20:08:15.0155836Z diff of stderr:
2020-03-02T20:08:15.0155967Z 
2020-03-02T20:08:15.0156256Z 4 LL | fn test2(x: &dyn Foo<(isize,),Output=()>, y: &dyn Foo(isize)) {
2020-03-02T20:08:15.0156724Z 5    |                                                   ^^^ expected 1 lifetime argument
2020-03-02T20:08:15.0157392Z - error: aborting due to previous error
2020-03-02T20:08:15.0157768Z + error[E0107]: wrong number of lifetime arguments: expected 1, found 0
2020-03-02T20:08:15.0158346Z +   --> $DIR/unboxed-closure-sugar-region.rs:23:58
2020-03-02T20:08:15.0158591Z +    |
2020-03-02T20:08:15.0158591Z +    |
2020-03-02T20:08:15.0158940Z + LL |     eq::< dyn Foo<(isize,),Output=()>,               dyn Foo(isize)                      >();
2020-03-02T20:08:15.0159439Z +    |                                                          ^^^ expected 1 lifetime argument
2020-03-02T20:08:15.0160218Z - For more information about this error, try `rustc --explain E0107`.
2020-03-02T20:08:15.0160635Z + error[E0107]: wrong number of lifetime arguments: expected 1, found 0
2020-03-02T20:08:15.0161211Z +   --> $DIR/unboxed-closure-sugar-region.rs:27:58
2020-03-02T20:08:15.0161474Z +    |
2020-03-02T20:08:15.0161474Z +    |
2020-03-02T20:08:15.0162041Z + LL |     eq::< dyn Foo<'static, (isize,),Output=()>,      dyn Foo(isize)                      >();
2020-03-02T20:08:15.0162537Z +    |                                                          ^^^ expected 1 lifetime argument
2020-03-02T20:08:15.0163099Z + error[E0621]: explicit lifetime required in the type of `x`
2020-03-02T20:08:15.0163639Z +   --> $DIR/unboxed-closure-sugar-region.rs:33:5
2020-03-02T20:08:15.0163901Z +    |
2020-03-02T20:08:15.0163901Z +    |
2020-03-02T20:08:15.0164200Z + LL | fn test2(x: &dyn Foo<(isize,),Output=()>, y: &dyn Foo(isize)) {
2020-03-02T20:08:15.0165093Z +    |             ---------------------------- help: add explicit lifetime `'static` to the type of `x`: `&dyn Foo<'static, (isize,), Output = ()>`
2020-03-02T20:08:15.0165570Z + ...
2020-03-02T20:08:15.0165759Z + LL |     same_type(x, y)
2020-03-02T20:08:15.0166203Z +    |     ^^^^^^^^^ lifetime `'static` required
2020-03-02T20:08:15.0166649Z + error: aborting due to 4 previous errors
2020-03-02T20:08:15.0166855Z + 
2020-03-02T20:08:15.0167118Z + Some errors have detailed explanations: E0107, E0621.
2020-03-02T20:08:15.0167693Z + For more information about an error, try `rustc --explain E0107`.
2020-03-02T20:08:15.0167693Z + For more information about an error, try `rustc --explain E0107`.
2020-03-02T20:08:15.0167946Z 10 
2020-03-02T20:08:15.0168057Z 
2020-03-02T20:08:15.0168175Z 
2020-03-02T20:08:15.0168395Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:15.0169189Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-region/unboxed-closure-sugar-region.stderr
2020-03-02T20:08:15.0169935Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:15.0170675Z To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closure-sugar-region.rs`
2020-03-02T20:08:15.0171190Z error: 1 errors occurred comparing output.
2020-03-02T20:08:15.0171454Z status: exit code: 1
2020-03-02T20:08:15.0171454Z status: exit code: 1
2020-03-02T20:08:15.0173753Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-region" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-region/auxiliary"
2020-03-02T20:08:15.0175671Z ------------------------------------------
2020-03-02T20:08:15.0175873Z 
2020-03-02T20:08:15.0176256Z ------------------------------------------
2020-03-02T20:08:15.0176470Z stderr:
2020-03-02T20:08:15.0176470Z stderr:
2020-03-02T20:08:15.0176859Z ------------------------------------------
2020-03-02T20:08:15.0177238Z error[E0107]: wrong number of lifetime arguments: expected 1, found 0
2020-03-02T20:08:15.0177901Z   --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-region.rs:30:51
2020-03-02T20:08:15.0178225Z    |
2020-03-02T20:08:15.0178511Z LL | fn test2(x: &dyn Foo<(isize,),Output=()>, y: &dyn Foo(isize)) {
2020-03-02T20:08:15.0178942Z    |                                                   ^^^ expected 1 lifetime argument
2020-03-02T20:08:15.0179494Z error[E0107]: wrong number of lifetime arguments: expected 1, found 0
2020-03-02T20:08:15.0180153Z   --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-region.rs:23:58
2020-03-02T20:08:15.0180452Z    |
2020-03-02T20:08:15.0180452Z    |
2020-03-02T20:08:15.0180796Z LL |     eq::< dyn Foo<(isize,),Output=()>,               dyn Foo(isize)                      >();
2020-03-02T20:08:15.0181284Z    |                                                          ^^^ expected 1 lifetime argument
2020-03-02T20:08:15.0181850Z error[E0107]: wrong number of lifetime arguments: expected 1, found 0
2020-03-02T20:08:15.0182513Z   --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-region.rs:27:58
2020-03-02T20:08:15.0182818Z    |
2020-03-02T20:08:15.0182818Z    |
2020-03-02T20:08:15.0183382Z LL |     eq::< dyn Foo<'static, (isize,),Output=()>,      dyn Foo(isize)                      >();
2020-03-02T20:08:15.0183869Z    |                                                          ^^^ expected 1 lifetime argument
2020-03-02T20:08:15.0184397Z error[E0621]: explicit lifetime required in the type of `x`
2020-03-02T20:08:15.0185018Z   --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-region.rs:33:5
2020-03-02T20:08:15.0185314Z    |
2020-03-02T20:08:15.0185314Z    |
2020-03-02T20:08:15.0185619Z LL | fn test2(x: &dyn Foo<(isize,),Output=()>, y: &dyn Foo(isize)) {
2020-03-02T20:08:15.0186495Z    |             ---------------------------- help: add explicit lifetime `'static` to the type of `x`: `&dyn Foo<'static, (isize,), Output = ()>`
2020-03-02T20:08:15.0186952Z ...
2020-03-02T20:08:15.0187142Z LL |     same_type(x, y)
2020-03-02T20:08:15.0187577Z    |     ^^^^^^^^^ lifetime `'static` required
2020-03-02T20:08:15.0187980Z error: aborting due to 4 previous errors
2020-03-02T20:08:15.0188161Z 
2020-03-02T20:08:15.0188403Z Some errors have detailed explanations: E0107, E0621.
2020-03-02T20:08:15.0188969Z For more information about an error, try `rustc --explain E0107`.
2020-03-02T20:08:15.0188969Z For more information about an error, try `rustc --explain E0107`.
2020-03-02T20:08:15.0189209Z 
2020-03-02T20:08:15.0189589Z ------------------------------------------
2020-03-02T20:08:15.0189772Z 
2020-03-02T20:08:15.0189874Z 
2020-03-02T20:08:15.0190496Z ---- [ui] ui/unboxed-closures/unboxed-closure-sugar-wrong-number-number-type-parameters.rs stdout ----
2020-03-02T20:08:15.0190840Z diff of stderr:
2020-03-02T20:08:15.0190973Z 
2020-03-02T20:08:15.0191160Z 10 LL | fn foo(_: dyn Zero())
2020-03-02T20:08:15.0191479Z 11    |               ^^^^^^ associated type `Output` not found
2020-03-02T20:08:15.0192178Z - error: aborting due to 2 previous errors
2020-03-02T20:08:15.0192178Z - error: aborting due to 2 previous errors
2020-03-02T20:08:15.0192836Z + error[E0277]: the size for values of type `(dyn Zero + 'static)` cannot be known at compilation time
2020-03-02T20:08:15.0193545Z +   --> $DIR/unboxed-closure-sugar-wrong-number-number-type-parameters.rs:5:8
2020-03-02T20:08:15.0193850Z +    |
2020-03-02T20:08:15.0194060Z + LL | fn foo(_: dyn Zero())
2020-03-02T20:08:15.0194541Z +    |        ^ doesn't have a size known at compile-time
2020-03-02T20:08:15.0194783Z +    |
2020-03-02T20:08:15.0195338Z +    = help: the trait `std::marker::Sized` is not implemented for `(dyn Zero + 'static)`
2020-03-02T20:08:15.0196746Z +    = note: all function arguments must have a statically known size
2020-03-02T20:08:15.0197137Z +    = help: unsized locals are gated as an unstable feature
2020-03-02T20:08:15.0197389Z 14 
2020-03-02T20:08:15.0197833Z - Some errors have detailed explanations: E0107, E0220.
---
2020-03-02T20:08:15.0199605Z 
2020-03-02T20:08:15.0199703Z 
2020-03-02T20:08:15.0199925Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:15.0200877Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-wrong-number-number-type-parameters/unboxed-closure-sugar-wrong-number-number-type-parameters.stderr
2020-03-02T20:08:15.0201712Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:15.0202492Z To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closure-sugar-wrong-number-number-type-parameters.rs`
2020-03-02T20:08:15.0203060Z error: 1 errors occurred comparing output.
2020-03-02T20:08:15.0203328Z status: exit code: 1
2020-03-02T20:08:15.0203328Z status: exit code: 1
2020-03-02T20:08:15.0205802Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-wrong-number-number-type-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-wrong-number-number-type-parameters" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-wrong-number-number-type-parameters/auxiliary"
2020-03-02T20:08:15.0207739Z ------------------------------------------
2020-03-02T20:08:15.0207937Z 
2020-03-02T20:08:15.0208316Z ------------------------------------------
2020-03-02T20:08:15.0208529Z stderr:
2020-03-02T20:08:15.0208529Z stderr:
2020-03-02T20:08:15.0208930Z ------------------------------------------
2020-03-02T20:08:15.0209280Z error[E0107]: wrong number of type arguments: expected 0, found 1
2020-03-02T20:08:15.0210026Z   --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-wrong-number-number-type-parameters.rs:5:15
2020-03-02T20:08:15.0210406Z    |
2020-03-02T20:08:15.0210595Z LL | fn foo(_: dyn Zero())
2020-03-02T20:08:15.0211076Z 
2020-03-02T20:08:15.0211076Z 
2020-03-02T20:08:15.0211374Z error[E0220]: associated type `Output` not found for `Zero`
2020-03-02T20:08:15.0212488Z    |
2020-03-02T20:08:15.0212488Z    |
2020-03-02T20:08:15.0212738Z LL | fn foo(_: dyn Zero())
2020-03-02T20:08:15.0213031Z    |               ^^^^^^ associated type `Output` not found
2020-03-02T20:08:15.0213246Z 
2020-03-02T20:08:15.0213824Z error[E0277]: the size for values of type `(dyn Zero + 'static)` cannot be known at compilation time
2020-03-02T20:08:15.0214979Z    |
2020-03-02T20:08:15.0214979Z    |
2020-03-02T20:08:15.0215272Z LL | fn foo(_: dyn Zero())
2020-03-02T20:08:15.0215757Z    |        ^ doesn't have a size known at compile-time
2020-03-02T20:08:15.0215990Z    |
2020-03-02T20:08:15.0216538Z    = help: the trait `std::marker::Sized` is not implemented for `(dyn Zero + 'static)`
2020-03-02T20:08:15.0217922Z    = note: all function arguments must have a statically known size
2020-03-02T20:08:15.0218308Z    = help: unsized locals are gated as an unstable feature
2020-03-02T20:08:15.0218526Z 
2020-03-02T20:08:15.0218727Z error: aborting due to 3 previous errors
---
2020-03-02T20:08:15.0220650Z 
2020-03-02T20:08:15.0221120Z ---- [ui] ui/underscore-lifetime/underscore-lifetime-binders.rs stdout ----
2020-03-02T20:08:15.0221425Z diff of stderr:
2020-03-02T20:08:15.0221557Z 
2020-03-02T20:08:15.0222008Z 36 LL | fn foo2<'a>(_: &'a u8, y: &'a u8) -> &'a u8 { y }
2020-03-02T20:08:15.0222580Z 38 
2020-03-02T20:08:15.0222970Z - error: aborting due to 5 previous errors
2020-03-02T20:08:15.0222970Z - error: aborting due to 5 previous errors
2020-03-02T20:08:15.0223616Z + error[E0491]: in type `&'static &'a u8`, reference has a longer lifetime than the data it references
2020-03-02T20:08:15.0224431Z +   --> $DIR/underscore-lifetime-binders.rs:2:16
2020-03-02T20:08:15.0224909Z +    |
2020-03-02T20:08:15.0225448Z + LL | struct Baz<'a>(&'_ &'a u8);
2020-03-02T20:08:15.0225919Z +    |
2020-03-02T20:08:15.0226168Z +    = note: the pointer is valid for the static lifetime
2020-03-02T20:08:15.0226900Z + note: but the referenced data is only valid for the lifetime `'a` as defined on the struct at 2:12
2020-03-02T20:08:15.0227584Z +   --> $DIR/underscore-lifetime-binders.rs:2:12
2020-03-02T20:08:15.0227584Z +   --> $DIR/underscore-lifetime-binders.rs:2:12
2020-03-02T20:08:15.0227825Z +    |
2020-03-02T20:08:15.0228218Z + LL | struct Baz<'a>(&'_ &'a u8);
2020-03-02T20:08:15.0228613Z 40 
2020-03-02T20:08:15.0229069Z - Some errors have detailed explanations: E0106, E0637.
2020-03-02T20:08:15.0229398Z + error: aborting due to 6 previous errors
2020-03-02T20:08:15.0229609Z + 
2020-03-02T20:08:15.0229609Z + 
2020-03-02T20:08:15.0229872Z + Some errors have detailed explanations: E0106, E0491, E0637.
2020-03-02T20:08:15.0230480Z 42 For more information about an error, try `rustc --explain E0106`.
2020-03-02T20:08:15.0230733Z 43 
2020-03-02T20:08:15.0230842Z 
2020-03-02T20:08:15.0230944Z 
2020-03-02T20:08:15.0231176Z The actual stderr differed from the expected stderr.
2020-03-02T20:08:15.0231973Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders/underscore-lifetime-binders.stderr
2020-03-02T20:08:15.0232726Z To update references, rerun the tests and pass the `--bless` flag
2020-03-02T20:08:15.0233498Z To only update this specific test, also pass `--test-args underscore-lifetime/underscore-lifetime-binders.rs`
2020-03-02T20:08:15.0234026Z error: 1 errors occurred comparing output.
2020-03-02T20:08:15.0234275Z status: exit code: 1
2020-03-02T20:08:15.0234275Z status: exit code: 1
2020-03-02T20:08:15.0236504Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders/auxiliary"
2020-03-02T20:08:15.0238413Z ------------------------------------------
2020-03-02T20:08:15.0238618Z 
2020-03-02T20:08:15.0238998Z ------------------------------------------
2020-03-02T20:08:15.0239212Z stderr:
---
2020-03-02T20:08:15.0242069Z 
2020-03-02T20:08:15.0242459Z error[E0637]: `'_` cannot be used here
2020-03-02T20:08:15.0243056Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:10:25
2020-03-02T20:08:15.0243357Z    |
2020-03-02T20:08:15.0243846Z LL | fn meh() -> Box<dyn for<'_> Meh<'_>> //~ ERROR cannot be used here
2020-03-02T20:08:15.0244433Z    |                         ^^ `'_` is a reserved lifetime name
2020-03-02T20:08:15.0244875Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:15.0245469Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:2:17
2020-03-02T20:08:15.0245771Z    |
2020-03-02T20:08:15.0245771Z    |
2020-03-02T20:08:15.0246257Z LL | struct Baz<'a>(&'_ &'a u8); //~ ERROR missing lifetime specifier
2020-03-02T20:08:15.0246873Z    |                 ^^ help: consider using the named lifetime: `'a`
2020-03-02T20:08:15.0247342Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:15.0247958Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:10:33
2020-03-02T20:08:15.0248261Z    |
2020-03-02T20:08:15.0248261Z    |
2020-03-02T20:08:15.0248729Z LL | fn meh() -> Box<dyn for<'_> Meh<'_>> //~ ERROR cannot be used here
2020-03-02T20:08:15.0249442Z    |                                 ^^ help: consider giving it a 'static lifetime: `'static`
2020-03-02T20:08:15.0250384Z    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
2020-03-02T20:08:15.0250722Z 
2020-03-02T20:08:15.0250928Z error[E0106]: missing lifetime specifier
2020-03-02T20:08:15.0251526Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:16:35
2020-03-02T20:08:15.0251526Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:16:35
2020-03-02T20:08:15.0251846Z    |
2020-03-02T20:08:15.0252380Z LL | fn foo2(_: &'_ u8, y: &'_ u8) -> &'_ u8 { y } //~ ERROR missing lifetime specifier
2020-03-02T20:08:15.0253313Z    |
2020-03-02T20:08:15.0253983Z    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or `y`
2020-03-02T20:08:15.0254479Z help: consider introducing a named lifetime parameter
2020-03-02T20:08:15.0254723Z    |
2020-03-02T20:08:15.0254723Z    |
2020-03-02T20:08:15.0255461Z LL | fn foo2<'a>(_: &'a u8, y: &'a u8) -> &'a u8 { y } //~ ERROR missing lifetime specifier
2020-03-02T20:08:15.0256175Z 
2020-03-02T20:08:15.0256175Z 
2020-03-02T20:08:15.0256774Z error[E0491]: in type `&'static &'a u8`, reference has a longer lifetime than the data it references
2020-03-02T20:08:15.0257866Z    |
2020-03-02T20:08:15.0257866Z    |
2020-03-02T20:08:15.0258341Z LL | struct Baz<'a>(&'_ &'a u8); //~ ERROR missing lifetime specifier
2020-03-02T20:08:15.0258835Z    |
2020-03-02T20:08:15.0259091Z    = note: the pointer is valid for the static lifetime
2020-03-02T20:08:15.0259784Z note: but the referenced data is only valid for the lifetime `'a` as defined on the struct at 2:12
2020-03-02T20:08:15.0260530Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:2:12
2020-03-02T20:08:15.0260530Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:2:12
2020-03-02T20:08:15.0260843Z    |
2020-03-02T20:08:15.0261305Z LL | struct Baz<'a>(&'_ &'a u8); //~ ERROR missing lifetime specifier
2020-03-02T20:08:15.0261755Z 
2020-03-02T20:08:15.0261954Z error: aborting due to 6 previous errors
2020-03-02T20:08:15.0262136Z 
2020-03-02T20:08:15.0262390Z Some errors have detailed explanations: E0106, E0491, E0637.
---
2020-03-02T20:08:15.0276137Z 
2020-03-02T20:08:15.0276238Z 
2020-03-02T20:08:15.0276755Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-02T20:08:15.0278103Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-02T20:08:15.0282272Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-02T20:08:15.0285174Z 
2020-03-02T20:08:15.0285281Z 
2020-03-02T20:08:15.0285531Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-02T20:08:15.0285898Z Build completed unsuccessfully in 1:03:48
2020-03-02T20:08:15.0285898Z Build completed unsuccessfully in 1:03:48
2020-03-02T20:08:15.0286164Z == clock drift check ==
2020-03-02T20:08:15.0286427Z   local time: Mon Mar  2 20:08:14 UTC 2020
2020-03-02T20:08:15.5846324Z   network time: Mon, 02 Mar 2020 20:08:15 GMT
2020-03-02T20:08:15.5851497Z == end clock drift check ==
2020-03-02T20:08:15.9842542Z 
2020-03-02T20:08:15.9926760Z ##[error]Bash exited with code '1'.
2020-03-02T20:08:15.9940136Z ##[section]Finishing: Run build
2020-03-02T20:08:15.9996946Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69642/merge to s
2020-03-02T20:08:16.0001895Z Task         : Get sources
2020-03-02T20:08:16.0002259Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T20:08:16.0002613Z Version      : 1.0.0
2020-03-02T20:08:16.0002850Z Author       : Microsoft
2020-03-02T20:08:16.0002850Z Author       : Microsoft
2020-03-02T20:08:16.0003230Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-02T20:08:16.0003683Z ==============================================================================
2020-03-02T20:08:16.3261452Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-02T20:08:16.3304650Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69642/merge to s
2020-03-02T20:08:16.3393955Z Cleaning up task key
2020-03-02T20:08:16.3395264Z Start cleaning up orphan processes.
2020-03-02T20:08:16.3556580Z Terminate orphan process: pid (3893) (python)
2020-03-02T20:08:16.3796085Z ##[section]Finishing: Finalize Job
