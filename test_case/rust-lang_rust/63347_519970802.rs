plain
2019-08-09T14:35:27.8604426Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T14:35:27.8823544Z ##[command]git config gc.auto 0
2019-08-09T14:35:27.8900542Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T14:35:27.8957030Z ##[command]git config --get-all http.proxy
2019-08-09T14:35:27.9108124Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63347/merge:refs/remotes/pull/63347/merge
---
2019-08-09T14:36:01.7453642Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T14:36:01.7453676Z 
2019-08-09T14:36:01.7453888Z   git checkout -b <new-branch-name>
2019-08-09T14:36:01.7453918Z 
2019-08-09T14:36:01.7453989Z HEAD is now at b1901774b Merge 2dfb04d6b59eaed23b98c14f05299ab4d0061a0a into 813a3a5d4b2be4e2101faa73a067da02a704a598
2019-08-09T14:36:01.7604110Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T14:36:01.7607507Z ==============================================================================
2019-08-09T14:36:01.7607571Z Task         : Bash
2019-08-09T14:36:01.7607639Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T15:41:11.9669493Z .................................................................................................... 1300/8853
2019-08-09T15:41:19.0290892Z .................................................................................................... 1400/8853
2019-08-09T15:41:25.7717878Z .................................................................................................... 1500/8853
2019-08-09T15:41:37.2818447Z ....................................................................................i............... 1600/8853
2019-08-09T15:41:45.4261303Z i................................................................................................... 1700/8853
2019-08-09T15:41:52.8805173Z ......................................................................iiiii......................... 1800/8853
2019-08-09T15:42:16.3404507Z .................................................................................................... 2000/8853
2019-08-09T15:42:18.9957481Z .................................................................................................... 2100/8853
2019-08-09T15:42:22.0278249Z .................................................................................................... 2200/8853
2019-08-09T15:42:30.4429327Z .................................................................................................... 2300/8853
---
2019-08-09T15:46:33.2656916Z .................................................................................................... 5200/8853
2019-08-09T15:46:45.2676904Z ..............................................................................................i..... 5300/8853
2019-08-09T15:46:54.0483240Z .................................................................................................... 5400/8853
2019-08-09T15:46:59.2866892Z .................................................................................................... 5500/8853
2019-08-09T15:47:11.7429461Z ........................................................................................ii...i..ii.. 5600/8853
2019-08-09T15:47:38.8694592Z .................................................................................................... 5800/8853
2019-08-09T15:47:44.7336590Z .................................................................................................... 5900/8853
2019-08-09T15:47:44.7336590Z .................................................................................................... 5900/8853
2019-08-09T15:47:49.6751475Z .........................................................................................i..ii...... 6000/8853
2019-08-09T15:48:22.5039174Z .................................................................................................... 6200/8853
2019-08-09T15:48:24.7167729Z ................................i................................................................... 6300/8853
2019-08-09T15:48:27.0760583Z .................................................................................................... 6400/8853
2019-08-09T15:48:29.8607162Z ....i............................................................................................... 6500/8853
2019-08-09T15:48:29.8607162Z ....i............................................................................................... 6500/8853
2019-08-09T15:48:34.9985924Z .................................................................................................... 6600/8853
2019-08-09T15:48:56.5522929Z .................................................................................................... 6700/8853
2019-08-09T15:49:16.7815653Z .................................................................................................... 6800/8853
2019-08-09T15:49:22.7961021Z ................................................FF.................................................. 6900/8853
2019-08-09T15:49:29.3339938Z ...................................................................F.FFFF........................... 7000/8853
2019-08-09T15:49:41.4431849Z .................................................................................................... 7200/8853
2019-08-09T15:49:51.4127135Z .................................................................................................... 7300/8853
2019-08-09T15:50:01.3219206Z .................................................................................................... 7400/8853
2019-08-09T15:50:12.3346108Z ................................ii......i........................................................... 7500/8853
---
2019-08-09T15:52:51.4281216Z 1 error[E0308]: mismatched types
2019-08-09T15:52:51.4281510Z -   --> $DIR/associated-type-projection-from-supertrait.rs:33:23
2019-08-09T15:52:51.4281762Z +   --> $DIR/associated-type-projection-from-supertrait.rs:27:23
2019-08-09T15:52:51.4281835Z 3    |
2019-08-09T15:52:51.4281881Z 4 LL | fn b() { dent(ModelT, Blue); }
2019-08-09T15:52:51.4281952Z 5    |                       ^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:52:51.4282032Z 8               found type `Blue`
2019-08-09T15:52:51.4282093Z 9 
2019-08-09T15:52:51.4282138Z 10 error[E0308]: mismatched types
2019-08-09T15:52:51.4283128Z -   --> $DIR/associated-type-projection-from-supertrait.rs:34:23
2019-08-09T15:52:51.4283128Z -   --> $DIR/associated-type-projection-from-supertrait.rs:34:23
2019-08-09T15:52:51.4283793Z +   --> $DIR/associated-type-projection-from-supertrait.rs:28:23
2019-08-09T15:52:51.4283883Z 12    |
2019-08-09T15:52:51.4283928Z 13 LL | fn c() { dent(ModelU, Black); }
2019-08-09T15:52:51.4283980Z 14    |                       ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:52:51.4284079Z 17               found type `Black`
2019-08-09T15:52:51.4284123Z 18 
2019-08-09T15:52:51.4284166Z 19 error[E0308]: mismatched types
2019-08-09T15:52:51.4284474Z -   --> $DIR/associated-type-projection-from-supertrait.rs:40:28
2019-08-09T15:52:51.4284474Z -   --> $DIR/associated-type-projection-from-supertrait.rs:40:28
2019-08-09T15:52:51.4284715Z +   --> $DIR/associated-type-projection-from-supertrait.rs:32:28
2019-08-09T15:52:51.4284762Z 21    |
2019-08-09T15:52:51.4284824Z 22 LL | fn f() { ModelT.chip_paint(Blue); }
2019-08-09T15:52:51.4284879Z 23    |                            ^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:52:51.4284973Z 26               found type `Blue`
2019-08-09T15:52:51.4285017Z 27 
2019-08-09T15:52:51.4285070Z 28 error[E0308]: mismatched types
2019-08-09T15:52:51.4285315Z -   --> $DIR/associated-type-projection-from-supertrait.rs:41:28
2019-08-09T15:52:51.4285315Z -   --> $DIR/associated-type-projection-from-supertrait.rs:41:28
2019-08-09T15:52:51.4285576Z +   --> $DIR/associated-type-projection-from-supertrait.rs:33:28
2019-08-09T15:52:51.4285623Z 30    |
2019-08-09T15:52:51.4285668Z 31 LL | fn g() { ModelU.chip_paint(Black); }
2019-08-09T15:52:51.4285739Z 32    |                            ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:52:51.4285912Z 
2019-08-09T15:52:51.4285959Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4286356Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
2019-08-09T15:52:51.4286356Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
2019-08-09T15:52:51.4286615Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4287003Z To only update this specific test, also pass `--test-args associated-type/associated-type-projection-from-supertrait.rs`
2019-08-09T15:52:51.4287088Z error: 1 errors occurred comparing output.
2019-08-09T15:52:51.4287134Z status: exit code: 1
2019-08-09T15:52:51.4287134Z status: exit code: 1
2019-08-09T15:52:51.4295406Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4295969Z ------------------------------------------
2019-08-09T15:52:51.4296010Z 
2019-08-09T15:52:51.4296244Z ------------------------------------------
2019-08-09T15:52:51.4296312Z stderr:
2019-08-09T15:52:51.4296312Z stderr:
2019-08-09T15:52:51.4296528Z ------------------------------------------
2019-08-09T15:52:51.4296578Z error[E0308]: mismatched types
2019-08-09T15:52:51.4296869Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:27:23
2019-08-09T15:52:51.4296924Z    |
2019-08-09T15:52:51.4296976Z LL | fn b() { dent(ModelT, Blue); } //~ ERROR mismatched types
2019-08-09T15:52:51.4297058Z    |                       ^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:52:51.4297152Z    = note: expected type `Black`
2019-08-09T15:52:51.4297217Z               found type `Blue`
2019-08-09T15:52:51.4297248Z 
2019-08-09T15:52:51.4297290Z error[E0308]: mismatched types
2019-08-09T15:52:51.4297290Z error[E0308]: mismatched types
2019-08-09T15:52:51.4297566Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:28:23
2019-08-09T15:52:51.4297633Z    |
2019-08-09T15:52:51.4297823Z LL | fn c() { dent(ModelU, Black); } //~ ERROR mismatched types
2019-08-09T15:52:51.4297887Z    |                       ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:52:51.4297996Z    = note: expected type `Blue`
2019-08-09T15:52:51.4298042Z               found type `Black`
2019-08-09T15:52:51.4298072Z 
2019-08-09T15:52:51.4298200Z error[E0308]: mismatched types
2019-08-09T15:52:51.4298200Z error[E0308]: mismatched types
2019-08-09T15:52:51.4298502Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:32:28
2019-08-09T15:52:51.4298562Z    |
2019-08-09T15:52:51.4298629Z LL | fn f() { ModelT.chip_paint(Blue); } //~ ERROR mismatched types
2019-08-09T15:52:51.4298685Z    |                            ^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:52:51.4298794Z    = note: expected type `Black`
2019-08-09T15:52:51.4298840Z               found type `Blue`
2019-08-09T15:52:51.4298870Z 
2019-08-09T15:52:51.4298920Z error[E0308]: mismatched types
2019-08-09T15:52:51.4298920Z error[E0308]: mismatched types
2019-08-09T15:52:51.4299210Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:33:28
2019-08-09T15:52:51.4306356Z    |
2019-08-09T15:52:51.4306520Z LL | fn g() { ModelU.chip_paint(Black); } //~ ERROR mismatched types
2019-08-09T15:52:51.4306610Z    |                            ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:52:51.4306704Z    = note: expected type `Blue`
2019-08-09T15:52:51.4306933Z               found type `Black`
2019-08-09T15:52:51.4306968Z 
2019-08-09T15:52:51.4307014Z error: aborting due to 4 previous errors
---
2019-08-09T15:52:51.4307914Z 
2019-08-09T15:52:51.4308213Z ---- [ui] ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs stdout ----
2019-08-09T15:52:51.4308267Z diff of stderr:
2019-08-09T15:52:51.4308298Z 
2019-08-09T15:52:51.4308365Z 1 error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
2019-08-09T15:52:51.4308882Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:31:10
2019-08-09T15:52:51.4308947Z 3    |
2019-08-09T15:52:51.4308947Z 3    |
2019-08-09T15:52:51.4309004Z 4 LL | fn b() { blue_car(ModelT); }
2019-08-09T15:52:51.4309056Z 5    |          ^^^^^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:52:51.4309150Z 7    = note: expected type `Black`
2019-08-09T15:52:51.4309199Z 8               found type `Blue`
2019-08-09T15:52:51.4309199Z 8               found type `Blue`
2019-08-09T15:52:51.4309244Z 9 note: required by `blue_car`
2019-08-09T15:52:51.4309784Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:27:1
2019-08-09T15:52:51.4309833Z 11    |
2019-08-09T15:52:51.4309833Z 11    |
2019-08-09T15:52:51.4309879Z 12 LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
2019-08-09T15:52:51.4309983Z 
2019-08-09T15:52:51.4310024Z 14 
2019-08-09T15:52:51.4310024Z 14 
2019-08-09T15:52:51.4310091Z 15 error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
2019-08-09T15:52:51.4310614Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:32:10
2019-08-09T15:52:51.4310681Z 17    |
2019-08-09T15:52:51.4310681Z 17    |
2019-08-09T15:52:51.4310726Z 18 LL | fn c() { black_car(ModelU); }
2019-08-09T15:52:51.4310777Z 19    |          ^^^^^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:52:51.4310871Z 21    = note: expected type `Blue`
2019-08-09T15:52:51.4310917Z 22               found type `Black`
2019-08-09T15:52:51.4310917Z 22               found type `Black`
2019-08-09T15:52:51.4310964Z 23 note: required by `black_car`
2019-08-09T15:52:51.4311624Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:24:1
2019-08-09T15:52:51.4311673Z 25    |
2019-08-09T15:52:51.4311673Z 25    |
2019-08-09T15:52:51.4311737Z 26 LL | fn black_car<C:Car<Color=Black>>(c: C) {
2019-08-09T15:52:51.4311821Z 
2019-08-09T15:52:51.4311847Z 
2019-08-09T15:52:51.4311902Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4312312Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
2019-08-09T15:52:51.4312312Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
2019-08-09T15:52:51.4312574Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4312923Z To only update this specific test, also pass `--test-args associated-types/associated-types-binding-to-type-defined-in-supertrait.rs`
2019-08-09T15:52:51.4313009Z error: 1 errors occurred comparing output.
2019-08-09T15:52:51.4337475Z status: exit code: 1
2019-08-09T15:52:51.4337475Z status: exit code: 1
2019-08-09T15:52:51.4338871Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4340734Z ------------------------------------------
2019-08-09T15:52:51.4340792Z 
2019-08-09T15:52:51.4341038Z ------------------------------------------
2019-08-09T15:52:51.4341086Z stderr:
2019-08-09T15:52:51.4341086Z stderr:
2019-08-09T15:52:51.4341299Z ------------------------------------------
2019-08-09T15:52:51.4341362Z error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
2019-08-09T15:52:51.4341723Z    |
2019-08-09T15:52:51.4341723Z    |
2019-08-09T15:52:51.4341782Z LL | fn b() { blue_car(ModelT); } //~ ERROR type mismatch
2019-08-09T15:52:51.4341836Z    |          ^^^^^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:52:51.4341936Z    = note: expected type `Black`
2019-08-09T15:52:51.4341983Z               found type `Blue`
2019-08-09T15:52:51.4341983Z               found type `Blue`
2019-08-09T15:52:51.4342029Z note: required by `blue_car`
2019-08-09T15:52:51.4342385Z    |
2019-08-09T15:52:51.4342385Z    |
2019-08-09T15:52:51.4342432Z LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
2019-08-09T15:52:51.4342527Z 
2019-08-09T15:52:51.4342527Z 
2019-08-09T15:52:51.4342577Z error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
2019-08-09T15:52:51.4342937Z    |
2019-08-09T15:52:51.4342937Z    |
2019-08-09T15:52:51.4342994Z LL | fn c() { black_car(ModelU); } //~ ERROR type mismatch
2019-08-09T15:52:51.4343047Z    |          ^^^^^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:52:51.4343813Z    = note: expected type `Blue`
2019-08-09T15:52:51.4343869Z               found type `Black`
2019-08-09T15:52:51.4343869Z               found type `Black`
2019-08-09T15:52:51.4343914Z note: required by `black_car`
2019-08-09T15:52:51.4344472Z    |
2019-08-09T15:52:51.4344472Z    |
2019-08-09T15:52:51.4344528Z LL | fn black_car<C:Car<Color=Black>>(c: C) {
2019-08-09T15:52:51.4344620Z 
2019-08-09T15:52:51.4344665Z error: aborting due to 2 previous errors
2019-08-09T15:52:51.4344695Z 
2019-08-09T15:52:51.4344972Z For more information about this error, try `rustc --explain E0271`.
2019-08-09T15:52:51.4344972Z For more information about this error, try `rustc --explain E0271`.
2019-08-09T15:52:51.4345008Z 
2019-08-09T15:52:51.4345220Z ------------------------------------------
2019-08-09T15:52:51.4345262Z 
2019-08-09T15:52:51.4345300Z 
2019-08-09T15:52:51.4345532Z ---- [ui] ui/hrtb/hrtb-conflate-regions.rs stdout ----
2019-08-09T15:52:51.4345582Z diff of stderr:
2019-08-09T15:52:51.4345612Z 
2019-08-09T15:52:51.4345898Z 1 error[E0277]: the trait bound `for<'a, 'b> SomeStruct: Foo<(&'a isize, &'b isize)>` is not satisfied
2019-08-09T15:52:51.4346125Z -   --> $DIR/hrtb-conflate-regions.rs:28:10
2019-08-09T15:52:51.4346345Z +   --> $DIR/hrtb-conflate-regions.rs:27:10
2019-08-09T15:52:51.4346417Z 3    |
2019-08-09T15:52:51.4346464Z 4 LL | fn b() { want_foo2::<SomeStruct>(); }
2019-08-09T15:52:51.4346769Z 5    |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a, 'b> Foo<(&'a isize, &'b isize)>` is not implemented for `SomeStruct`
2019-08-09T15:52:51.4346849Z 
2019-08-09T15:52:51.4346896Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4347204Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/hrtb-conflate-regions.stderr
2019-08-09T15:52:51.4347204Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/hrtb-conflate-regions.stderr
2019-08-09T15:52:51.4347575Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4348304Z To only update this specific test, also pass `--test-args hrtb/hrtb-conflate-regions.rs`
2019-08-09T15:52:51.4348378Z error: 1 errors occurred comparing output.
2019-08-09T15:52:51.4348428Z status: exit code: 1
2019-08-09T15:52:51.4348428Z status: exit code: 1
2019-08-09T15:52:51.4349108Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/hrtb-conflate-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4349414Z ------------------------------------------
2019-08-09T15:52:51.4349446Z 
2019-08-09T15:52:51.4349802Z ------------------------------------------
2019-08-09T15:52:51.4349858Z stderr:
2019-08-09T15:52:51.4349858Z stderr:
2019-08-09T15:52:51.4350216Z ------------------------------------------
2019-08-09T15:52:51.4350641Z error[E0277]: the trait bound `for<'a, 'b> SomeStruct: Foo<(&'a isize, &'b isize)>` is not satisfied
2019-08-09T15:52:51.4350944Z    |
2019-08-09T15:52:51.4350944Z    |
2019-08-09T15:52:51.4350987Z LL | fn b() { want_foo2::<SomeStruct>(); } //~ ERROR
2019-08-09T15:52:51.4351467Z    |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a, 'b> Foo<(&'a isize, &'b isize)>` is not implemented for `SomeStruct`
2019-08-09T15:52:51.4351562Z    = help: the following implementations were found:
2019-08-09T15:52:51.4351562Z    = help: the following implementations were found:
2019-08-09T15:52:51.4351794Z              <SomeStruct as Foo<(&'a isize, &'a isize)>>
2019-08-09T15:52:51.4351852Z note: required by `want_foo2`
2019-08-09T15:52:51.4352127Z    |
2019-08-09T15:52:51.4352127Z    |
2019-08-09T15:52:51.4352180Z LL | / fn want_foo2<T>()
2019-08-09T15:52:51.4352580Z LL | |     where T : for<'a,'b> Foo<(&'a isize, &'b isize)>
2019-08-09T15:52:51.4352631Z LL | | {
2019-08-09T15:52:51.4352726Z    | |_^
2019-08-09T15:52:51.4352753Z 
2019-08-09T15:52:51.4352797Z error: aborting due to previous error
2019-08-09T15:52:51.4352918Z 
---
2019-08-09T15:52:51.4354566Z diff of stderr:
2019-08-09T15:52:51.4354609Z 
2019-08-09T15:52:51.4354667Z 7    = note: `#[warn(incomplete_features)]` on by default
2019-08-09T15:52:51.4354711Z 8 
2019-08-09T15:52:51.4354765Z 9 error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
2019-08-09T15:52:51.4355234Z +   --> $DIR/bound-normalization-fail.rs:28:32
2019-08-09T15:52:51.4355280Z 11    |
2019-08-09T15:52:51.4355280Z 11    |
2019-08-09T15:52:51.4355872Z 12 LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
2019-08-09T15:52:51.4355949Z 13    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-09T15:52:51.4355985Z 
2019-08-09T15:52:51.4356044Z 17    = note: the return type of a function must have a statically known size
2019-08-09T15:52:51.4356091Z 18 
2019-08-09T15:52:51.4356421Z 19 error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
2019-08-09T15:52:51.4357037Z +   --> $DIR/bound-normalization-fail.rs:44:41
2019-08-09T15:52:51.4357403Z 21    |
2019-08-09T15:52:51.4357403Z 21    |
2019-08-09T15:52:51.4359984Z 22 LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
2019-08-09T15:52:51.4360094Z 23    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-09T15:52:51.4360160Z 
2019-08-09T15:52:51.4360222Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4360661Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
2019-08-09T15:52:51.4360661Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
2019-08-09T15:52:51.4360986Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4361286Z To only update this specific test, also pass `--test-args impl-trait/bound-normalization-fail.rs`
2019-08-09T15:52:51.4361370Z error: 1 errors occurred comparing output.
2019-08-09T15:52:51.4361439Z status: exit code: 1
2019-08-09T15:52:51.4361439Z status: exit code: 1
2019-08-09T15:52:51.4362305Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bound-normalization-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4362644Z ------------------------------------------
2019-08-09T15:52:51.4362680Z 
2019-08-09T15:52:51.4362909Z ------------------------------------------
2019-08-09T15:52:51.4362955Z stderr:
---
2019-08-09T15:52:51.4364108Z    |            ^^^^^^^^^^^^^^^^^^^^^^
2019-08-09T15:52:51.4364382Z    |
2019-08-09T15:52:51.4364448Z    = note: `#[warn(incomplete_features)]` on by default
2019-08-09T15:52:51.4364502Z 
2019-08-09T15:52:51.4365493Z error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
2019-08-09T15:52:51.4366010Z    |
2019-08-09T15:52:51.4366010Z    |
2019-08-09T15:52:51.4366250Z LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
2019-08-09T15:52:51.4366310Z    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-09T15:52:51.4366432Z    = note: expected type `()`
2019-08-09T15:52:51.4366432Z    = note: expected type `()`
2019-08-09T15:52:51.4366482Z               found type `<T as impl_trait::Trait>::Assoc`
2019-08-09T15:52:51.4366535Z    = note: the return type of a function must have a statically known size
2019-08-09T15:52:51.4366583Z 
2019-08-09T15:52:51.4366869Z error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
2019-08-09T15:52:51.4367197Z    |
2019-08-09T15:52:51.4367197Z    |
2019-08-09T15:52:51.4367445Z LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
2019-08-09T15:52:51.4367507Z    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-09T15:52:51.4367614Z    = note: expected type `()`
2019-08-09T15:52:51.4367614Z    = note: expected type `()`
2019-08-09T15:52:51.4367851Z               found type `<T as lifetimes::Trait<'static>>::Assoc`
2019-08-09T15:52:51.4368027Z    = note: the return type of a function must have a statically known size
2019-08-09T15:52:51.4368106Z error: aborting due to 2 previous errors
2019-08-09T15:52:51.4368136Z 
2019-08-09T15:52:51.4368415Z For more information about this error, try `rustc --explain E0271`.
2019-08-09T15:52:51.4368452Z 
2019-08-09T15:52:51.4368452Z 
2019-08-09T15:52:51.4368668Z ------------------------------------------
2019-08-09T15:52:51.4368701Z 
2019-08-09T15:52:51.4368728Z 
2019-08-09T15:52:51.4368970Z ---- [ui] ui/issues/issue-12028.rs stdout ----
2019-08-09T15:52:51.4369019Z diff of stderr:
2019-08-09T15:52:51.4369049Z 
2019-08-09T15:52:51.4369101Z 1 error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2019-08-09T15:52:51.4369339Z -   --> $DIR/issue-12028.rs:29:14
2019-08-09T15:52:51.4369550Z +   --> $DIR/issue-12028.rs:27:14
2019-08-09T15:52:51.4369659Z 4 LL |         self.input_stream(&mut stream);
2019-08-09T15:52:51.4369715Z 5    |              ^^^^^^^^^^^^
2019-08-09T15:52:51.4369746Z 
2019-08-09T15:52:51.4369772Z 
2019-08-09T15:52:51.4369772Z 
2019-08-09T15:52:51.4369832Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4370127Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/issue-12028.stderr
2019-08-09T15:52:51.4370458Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4375093Z To only update this specific test, also pass `--test-args issues/issue-12028.rs`
2019-08-09T15:52:51.4375203Z error: 1 errors occurred comparing output.
2019-08-09T15:52:51.4375248Z status: exit code: 1
2019-08-09T15:52:51.4375248Z status: exit code: 1
2019-08-09T15:52:51.4376318Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12028.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4376689Z ------------------------------------------
2019-08-09T15:52:51.4376726Z 
2019-08-09T15:52:51.4376946Z ------------------------------------------
2019-08-09T15:52:51.4377071Z stderr:
2019-08-09T15:52:51.4377071Z stderr:
2019-08-09T15:52:51.4377427Z ------------------------------------------
2019-08-09T15:52:51.4377499Z error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2019-08-09T15:52:51.4377787Z   --> /checkout/src/test/ui/issues/issue-12028.rs:27:14
2019-08-09T15:52:51.4377841Z    |
2019-08-09T15:52:51.4377891Z LL |         self.input_stream(&mut stream); //~ ERROR type annotations required
2019-08-09T15:52:51.4378000Z 
2019-08-09T15:52:51.4378044Z error: aborting due to previous error
2019-08-09T15:52:51.4378074Z 
2019-08-09T15:52:51.4378330Z For more information about this error, try `rustc --explain E0284`.
2019-08-09T15:52:51.4378330Z For more information about this error, try `rustc --explain E0284`.
2019-08-09T15:52:51.4378366Z 
2019-08-09T15:52:51.4378578Z ------------------------------------------
2019-08-09T15:52:51.4378610Z 
2019-08-09T15:52:51.4378653Z 
2019-08-09T15:52:51.4380465Z ---- [ui] ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs#migrate stdout ----
2019-08-09T15:52:51.4380532Z diff of stderr:
2019-08-09T15:52:51.4380573Z 
2019-08-09T15:52:51.4380881Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4381707Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-09T15:52:51.4381769Z 3    |
2019-08-09T15:52:51.4381769Z 3    |
2019-08-09T15:52:51.4381997Z 4 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4382220Z 
2019-08-09T15:52:51.4382280Z 6    |
2019-08-09T15:52:51.4382280Z 6    |
2019-08-09T15:52:51.4382849Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 37:15
2019-08-09T15:52:51.4383434Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:37:15
2019-08-09T15:52:51.4383867Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 33:15
2019-08-09T15:52:51.4384131Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:33:15
2019-08-09T15:52:51.4384180Z 9    |
2019-08-09T15:52:51.4384408Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4384488Z 
2019-08-09T15:52:51.4384488Z 
2019-08-09T15:52:51.4384763Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 37:18
2019-08-09T15:52:51.4385041Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:37:18
2019-08-09T15:52:51.4385332Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 33:18
2019-08-09T15:52:51.4385583Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:33:18
2019-08-09T15:52:51.4385650Z 14    |
2019-08-09T15:52:51.4385854Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4385948Z 
2019-08-09T15:52:51.4385975Z 
2019-08-09T15:52:51.4386022Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4386022Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4386415Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate/regions-assoc-type-in-supertrait-outlives-container.migrate.stderr
2019-08-09T15:52:51.4386699Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4387000Z To only update this specific test, also pass `--test-args regions/regions-assoc-type-in-supertrait-outlives-container.rs`
2019-08-09T15:52:51.4387063Z 
2019-08-09T15:52:51.4387112Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-09T15:52:51.4387160Z status: exit code: 1
2019-08-09T15:52:51.4388136Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4388505Z ------------------------------------------
2019-08-09T15:52:51.4388551Z 
2019-08-09T15:52:51.4388769Z ------------------------------------------
2019-08-09T15:52:51.4388817Z stderr:
2019-08-09T15:52:51.4388817Z stderr:
2019-08-09T15:52:51.4389042Z ------------------------------------------
2019-08-09T15:52:51.4390646Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4391030Z    |
2019-08-09T15:52:51.4391030Z    |
2019-08-09T15:52:51.4391271Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4391384Z    |
2019-08-09T15:52:51.4391384Z    |
2019-08-09T15:52:51.4391645Z note: the pointer is valid for the lifetime 'a as defined on the function body at 33:15
2019-08-09T15:52:51.4392035Z    |
2019-08-09T15:52:51.4392035Z    |
2019-08-09T15:52:51.4392234Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4392420Z    |               ^^
2019-08-09T15:52:51.4392734Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 33:18
2019-08-09T15:52:51.4393058Z    |
2019-08-09T15:52:51.4393058Z    |
2019-08-09T15:52:51.4394018Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4394114Z 
2019-08-09T15:52:51.4394170Z error: aborting due to previous error
2019-08-09T15:52:51.4394218Z 
2019-08-09T15:52:51.4394472Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-09T15:52:51.4395206Z 1 error: lifetime may not live long enough
2019-08-09T15:52:51.4395454Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:43:12
2019-08-09T15:52:51.4395701Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-09T15:52:51.4395769Z 3    |
2019-08-09T15:52:51.4395974Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4396202Z 5    |               -- -- lifetime `'b` defined here
2019-08-09T15:52:51.4396289Z 
2019-08-09T15:52:51.4396336Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4396336Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4396713Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll/regions-assoc-type-in-supertrait-outlives-container.nll.stderr
2019-08-09T15:52:51.4396993Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4397593Z To only update this specific test, also pass `--test-args regions/regions-assoc-type-in-supertrait-outlives-container.rs`
2019-08-09T15:52:51.4397669Z 
2019-08-09T15:52:51.4397718Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-09T15:52:51.4397764Z status: exit code: 1
2019-08-09T15:52:51.4398779Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4399387Z ------------------------------------------
2019-08-09T15:52:51.4399423Z 
2019-08-09T15:52:51.4399634Z ------------------------------------------
2019-08-09T15:52:51.4399678Z stderr:
2019-08-09T15:52:51.4399678Z stderr:
2019-08-09T15:52:51.4399900Z ------------------------------------------
2019-08-09T15:52:51.4399949Z error: lifetime may not live long enough
2019-08-09T15:52:51.4400367Z   --> /checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-09T15:52:51.4400433Z    |
2019-08-09T15:52:51.4400633Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4400993Z    |               -- -- lifetime `'b` defined here
2019-08-09T15:52:51.4401249Z    |               lifetime `'a` defined here
2019-08-09T15:52:51.4401292Z ...
2019-08-09T15:52:51.4401292Z ...
2019-08-09T15:52:51.4401511Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4401907Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:52:51.4402238Z error: aborting due to previous error
2019-08-09T15:52:51.4402283Z 
2019-08-09T15:52:51.4402306Z 
2019-08-09T15:52:51.4402516Z ------------------------------------------
2019-08-09T15:52:51.4402516Z ------------------------------------------
2019-08-09T15:52:51.4402545Z 
2019-08-09T15:52:51.4402569Z 
2019-08-09T15:52:51.4403820Z ---- [ui] ui/regions/regions-outlives-projection-container-hrtb.rs#migrate stdout ----
2019-08-09T15:52:51.4403883Z diff of stderr:
2019-08-09T15:52:51.4403914Z 
2019-08-09T15:52:51.4404270Z 1 error[E0491]: in type `&'a WithHrAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4404760Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-09T15:52:51.4404826Z 3    |
2019-08-09T15:52:51.4404826Z 3    |
2019-08-09T15:52:51.4405056Z 4 LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4405149Z 
2019-08-09T15:52:51.4405206Z 6    |
2019-08-09T15:52:51.4405206Z 6    |
2019-08-09T15:52:51.4405469Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 32:15
2019-08-09T15:52:51.4405711Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:32:15
2019-08-09T15:52:51.4405989Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-09T15:52:51.4406234Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:27:15
2019-08-09T15:52:51.4406282Z 9    |
2019-08-09T15:52:51.4406509Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4406588Z 
2019-08-09T15:52:51.4406588Z 
2019-08-09T15:52:51.4406864Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 32:18
2019-08-09T15:52:51.4407123Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:32:18
2019-08-09T15:52:51.4407400Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-09T15:52:51.4407810Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:27:18
2019-08-09T15:52:51.4407874Z 14    |
2019-08-09T15:52:51.4408223Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4408313Z 
2019-08-09T15:52:51.4408351Z 17 
2019-08-09T15:52:51.4408351Z 17 
2019-08-09T15:52:51.4408624Z 18 error[E0491]: in type `&'a WithHrAssocSub<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4409775Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-09T15:52:51.4409994Z 20    |
2019-08-09T15:52:51.4409994Z 20    |
2019-08-09T15:52:51.4410226Z 21 LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-09T15:52:51.4410329Z 
2019-08-09T15:52:51.4410369Z 23    |
2019-08-09T15:52:51.4410369Z 23    |
2019-08-09T15:52:51.4410627Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 53:19
2019-08-09T15:52:51.4410898Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:53:19
2019-08-09T15:52:51.4411163Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 46:19
2019-08-09T15:52:51.4411422Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:46:19
2019-08-09T15:52:51.4411470Z 26    |
2019-08-09T15:52:51.4411676Z 27 LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:52:51.4411771Z 
2019-08-09T15:52:51.4411771Z 
2019-08-09T15:52:51.4412054Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 53:22
2019-08-09T15:52:51.4412298Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:53:22
2019-08-09T15:52:51.4412740Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 46:22
2019-08-09T15:52:51.4412975Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:46:22
2019-08-09T15:52:51.4413785Z 31    |
2019-08-09T15:52:51.4414079Z 32 LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:52:51.4414161Z 
2019-08-09T15:52:51.4414188Z 
2019-08-09T15:52:51.4414249Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4414249Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4414617Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate/regions-outlives-projection-container-hrtb.migrate.stderr
2019-08-09T15:52:51.4414884Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4415191Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-09T15:52:51.4415230Z 
2019-08-09T15:52:51.4415276Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-09T15:52:51.4415340Z status: exit code: 1
2019-08-09T15:52:51.4416178Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4416523Z ------------------------------------------
2019-08-09T15:52:51.4416558Z 
2019-08-09T15:52:51.4416787Z ------------------------------------------
2019-08-09T15:52:51.4416835Z stderr:
2019-08-09T15:52:51.4416835Z stderr:
2019-08-09T15:52:51.4417188Z ------------------------------------------
2019-08-09T15:52:51.4417444Z error[E0491]: in type `&'a WithHrAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4417763Z    |
2019-08-09T15:52:51.4417763Z    |
2019-08-09T15:52:51.4418295Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4418377Z    |
2019-08-09T15:52:51.4418377Z    |
2019-08-09T15:52:51.4418771Z note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-09T15:52:51.4419146Z    |
2019-08-09T15:52:51.4419146Z    |
2019-08-09T15:52:51.4419511Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4419571Z    |               ^^
2019-08-09T15:52:51.4419998Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-09T15:52:51.4420522Z    |
2019-08-09T15:52:51.4420522Z    |
2019-08-09T15:52:51.4420878Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4420951Z 
2019-08-09T15:52:51.4420951Z 
2019-08-09T15:52:51.4422023Z error[E0491]: in type `&'a WithHrAssocSub<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4422356Z    |
2019-08-09T15:52:51.4422356Z    |
2019-08-09T15:52:51.4422615Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-09T15:52:51.4422714Z    |
2019-08-09T15:52:51.4422714Z    |
2019-08-09T15:52:51.4422989Z note: the pointer is valid for the lifetime 'a as defined on the function body at 46:19
2019-08-09T15:52:51.4423721Z    |
2019-08-09T15:52:51.4423721Z    |
2019-08-09T15:52:51.4423951Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:52:51.4424118Z    |                   ^^
2019-08-09T15:52:51.4424415Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 46:22
2019-08-09T15:52:51.4424746Z    |
2019-08-09T15:52:51.4424746Z    |
2019-08-09T15:52:51.4424949Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:52:51.4425044Z 
2019-08-09T15:52:51.4425088Z error: aborting due to 2 previous errors
2019-08-09T15:52:51.4425127Z 
2019-08-09T15:52:51.4426324Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-09T15:52:51.4427214Z 1 error: lifetime may not live long enough
2019-08-09T15:52:51.4427476Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:35:12
2019-08-09T15:52:51.4427717Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-09T15:52:51.4427765Z 3    |
2019-08-09T15:52:51.4427987Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4428216Z 5    |               -- -- lifetime `'b` defined here
2019-08-09T15:52:51.4428542Z 10    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:52:51.4428595Z 11 
2019-08-09T15:52:51.4428642Z 12 error: lifetime may not live long enough
2019-08-09T15:52:51.4428884Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-09T15:52:51.4428884Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-09T15:52:51.4429144Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-09T15:52:51.4429192Z 14    |
2019-08-09T15:52:51.4429398Z 15 LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:52:51.4429647Z 16    |                   -- -- lifetime `'b` defined here
2019-08-09T15:52:51.4429717Z 
2019-08-09T15:52:51.4429763Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4429763Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4430138Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll/regions-outlives-projection-container-hrtb.nll.stderr
2019-08-09T15:52:51.4430395Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4430830Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-09T15:52:51.4430880Z 
2019-08-09T15:52:51.4430927Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-09T15:52:51.4430973Z status: exit code: 1
2019-08-09T15:52:51.4431849Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4432209Z ------------------------------------------
2019-08-09T15:52:51.4432244Z 
2019-08-09T15:52:51.4432460Z ------------------------------------------
2019-08-09T15:52:51.4432525Z stderr:
2019-08-09T15:52:51.4432525Z stderr:
2019-08-09T15:52:51.4432739Z ------------------------------------------
2019-08-09T15:52:51.4432789Z error: lifetime may not live long enough
2019-08-09T15:52:51.4433063Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-09T15:52:51.4433560Z    |
2019-08-09T15:52:51.4433840Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4434086Z    |               -- -- lifetime `'b` defined here
2019-08-09T15:52:51.4434350Z    |               lifetime `'a` defined here
2019-08-09T15:52:51.4434411Z ...
2019-08-09T15:52:51.4434411Z ...
2019-08-09T15:52:51.4434637Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4434905Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:52:51.4435016Z error: lifetime may not live long enough
2019-08-09T15:52:51.4435280Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-09T15:52:51.4435330Z    |
2019-08-09T15:52:51.4435330Z    |
2019-08-09T15:52:51.4435551Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:52:51.4435777Z    |                   -- -- lifetime `'b` defined here
2019-08-09T15:52:51.4436061Z    |                   lifetime `'a` defined here
2019-08-09T15:52:51.4436117Z ...
2019-08-09T15:52:51.4436117Z ...
2019-08-09T15:52:51.4436348Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-09T15:52:51.4436638Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:52:51.4436721Z error: aborting due to 2 previous errors
2019-08-09T15:52:51.4436750Z 
2019-08-09T15:52:51.4436988Z 
2019-08-09T15:52:51.4437838Z ------------------------------------------
2019-08-09T15:52:51.4437838Z ------------------------------------------
2019-08-09T15:52:51.4437887Z 
2019-08-09T15:52:51.4437913Z 
2019-08-09T15:52:51.4438171Z ---- [ui] ui/regions/regions-outlives-projection-container-wc.rs#migrate stdout ----
2019-08-09T15:52:51.4438239Z diff of stderr:
2019-08-09T15:52:51.4438268Z 
2019-08-09T15:52:51.4438553Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4439432Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-09T15:52:51.4439482Z 3    |
2019-08-09T15:52:51.4439482Z 3    |
2019-08-09T15:52:51.4439724Z 4 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4439808Z 
2019-08-09T15:52:51.4439848Z 6    |
2019-08-09T15:52:51.4439848Z 6    |
2019-08-09T15:52:51.4440125Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 31:15
2019-08-09T15:52:51.4440367Z -   --> $DIR/regions-outlives-projection-container-wc.rs:31:15
2019-08-09T15:52:51.4440749Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-09T15:52:51.4441094Z +   --> $DIR/regions-outlives-projection-container-wc.rs:27:15
2019-08-09T15:52:51.4441142Z 9    |
2019-08-09T15:52:51.4441345Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4441441Z 
2019-08-09T15:52:51.4441441Z 
2019-08-09T15:52:51.4442497Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 31:18
2019-08-09T15:52:51.4442773Z -   --> $DIR/regions-outlives-projection-container-wc.rs:31:18
2019-08-09T15:52:51.4443076Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-09T15:52:51.4443619Z +   --> $DIR/regions-outlives-projection-container-wc.rs:27:18
2019-08-09T15:52:51.4443677Z 14    |
2019-08-09T15:52:51.4443907Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4444276Z 
2019-08-09T15:52:51.4444330Z 
2019-08-09T15:52:51.4444397Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4444397Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4444826Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate/regions-outlives-projection-container-wc.migrate.stderr
2019-08-09T15:52:51.4445085Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4445395Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-09T15:52:51.4445576Z 
2019-08-09T15:52:51.4445623Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-09T15:52:51.4445689Z status: exit code: 1
2019-08-09T15:52:51.4446569Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4446913Z ------------------------------------------
2019-08-09T15:52:51.4446948Z 
2019-08-09T15:52:51.4447399Z ------------------------------------------
2019-08-09T15:52:51.4447444Z stderr:
2019-08-09T15:52:51.4447444Z stderr:
2019-08-09T15:52:51.4447645Z ------------------------------------------
2019-08-09T15:52:51.4448109Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4448442Z    |
2019-08-09T15:52:51.4448442Z    |
2019-08-09T15:52:51.4448680Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4448776Z    |
2019-08-09T15:52:51.4448776Z    |
2019-08-09T15:52:51.4449050Z note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-09T15:52:51.4449365Z    |
2019-08-09T15:52:51.4449365Z    |
2019-08-09T15:52:51.4449590Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4449639Z    |               ^^
2019-08-09T15:52:51.4449912Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-09T15:52:51.4450241Z    |
2019-08-09T15:52:51.4450241Z    |
2019-08-09T15:52:51.4450440Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4450535Z 
2019-08-09T15:52:51.4450655Z error: aborting due to previous error
2019-08-09T15:52:51.4450691Z 
2019-08-09T15:52:51.4450948Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-09T15:52:51.4451666Z 1 error: lifetime may not live long enough
2019-08-09T15:52:51.4452459Z -   --> $DIR/regions-outlives-projection-container-wc.rs:37:12
2019-08-09T15:52:51.4452717Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-09T15:52:51.4452765Z 3    |
2019-08-09T15:52:51.4452983Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4453587Z 5    |               -- -- lifetime `'b` defined here
2019-08-09T15:52:51.4453660Z 
2019-08-09T15:52:51.4453741Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4453741Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4454114Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll/regions-outlives-projection-container-wc.nll.stderr
2019-08-09T15:52:51.4454370Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4454680Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-09T15:52:51.4454849Z 
2019-08-09T15:52:51.4454897Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-09T15:52:51.4454962Z status: exit code: 1
2019-08-09T15:52:51.4455821Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4456155Z ------------------------------------------
2019-08-09T15:52:51.4456200Z 
2019-08-09T15:52:51.4456432Z ------------------------------------------
2019-08-09T15:52:51.4456479Z stderr:
2019-08-09T15:52:51.4456479Z stderr:
2019-08-09T15:52:51.4456690Z ------------------------------------------
2019-08-09T15:52:51.4456740Z error: lifetime may not live long enough
2019-08-09T15:52:51.4457015Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs:33:12
2019-08-09T15:52:51.4457067Z    |
2019-08-09T15:52:51.4457270Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4457523Z    |               -- -- lifetime `'b` defined here
2019-08-09T15:52:51.4457793Z    |               lifetime `'a` defined here
2019-08-09T15:52:51.4457857Z ...
2019-08-09T15:52:51.4457857Z ...
2019-08-09T15:52:51.4458081Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4458347Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:52:51.4458447Z error: aborting due to previous error
2019-08-09T15:52:51.4458485Z 
2019-08-09T15:52:51.4458512Z 
2019-08-09T15:52:51.4458723Z ------------------------------------------
2019-08-09T15:52:51.4458723Z ------------------------------------------
2019-08-09T15:52:51.4458770Z 
2019-08-09T15:52:51.4458797Z 
2019-08-09T15:52:51.4459037Z ---- [ui] ui/regions/regions-outlives-projection-container.rs stdout ----
2019-08-09T15:52:51.4459088Z diff of stderr:
2019-08-09T15:52:51.4459133Z 
2019-08-09T15:52:51.4459947Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4460728Z +   --> $DIR/regions-outlives-projection-container.rs:36:13
2019-08-09T15:52:51.4460775Z 3    |
2019-08-09T15:52:51.4460775Z 3    |
2019-08-09T15:52:51.4461152Z 4 LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4461428Z 
2019-08-09T15:52:51.4461466Z 6    |
2019-08-09T15:52:51.4461466Z 6    |
2019-08-09T15:52:51.4461916Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 32:15
2019-08-09T15:52:51.4462174Z -   --> $DIR/regions-outlives-projection-container.rs:32:15
2019-08-09T15:52:51.4462418Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 28:15
2019-08-09T15:52:51.4462636Z +   --> $DIR/regions-outlives-projection-container.rs:28:15
2019-08-09T15:52:51.4462695Z 9    |
2019-08-09T15:52:51.4462889Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4462986Z 
2019-08-09T15:52:51.4462986Z 
2019-08-09T15:52:51.4463876Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 32:18
2019-08-09T15:52:51.4464125Z -   --> $DIR/regions-outlives-projection-container.rs:32:18
2019-08-09T15:52:51.4464423Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 28:18
2019-08-09T15:52:51.4464664Z +   --> $DIR/regions-outlives-projection-container.rs:28:18
2019-08-09T15:52:51.4464822Z 14    |
2019-08-09T15:52:51.4465044Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4465143Z 
2019-08-09T15:52:51.4465182Z 17 
2019-08-09T15:52:51.4465182Z 17 
2019-08-09T15:52:51.4465487Z 18 error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4465960Z +   --> $DIR/regions-outlives-projection-container.rs:54:13
2019-08-09T15:52:51.4466033Z 20    |
2019-08-09T15:52:51.4466033Z 20    |
2019-08-09T15:52:51.4466269Z 21 LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4466354Z 
2019-08-09T15:52:51.4466412Z 23    |
2019-08-09T15:52:51.4466412Z 23    |
2019-08-09T15:52:51.4466671Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 54:18
2019-08-09T15:52:51.4466908Z -   --> $DIR/regions-outlives-projection-container.rs:54:18
2019-08-09T15:52:51.4467198Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 50:18
2019-08-09T15:52:51.4467436Z +   --> $DIR/regions-outlives-projection-container.rs:50:18
2019-08-09T15:52:51.4467483Z 26    |
2019-08-09T15:52:51.4467768Z 27 LL | fn without_assoc<'a,'b>() {
2019-08-09T15:52:51.4467848Z 
2019-08-09T15:52:51.4467848Z 
2019-08-09T15:52:51.4468269Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 54:21
2019-08-09T15:52:51.4468523Z -   --> $DIR/regions-outlives-projection-container.rs:54:21
2019-08-09T15:52:51.4468795Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 50:21
2019-08-09T15:52:51.4469028Z +   --> $DIR/regions-outlives-projection-container.rs:50:21
2019-08-09T15:52:51.4469090Z 31    |
2019-08-09T15:52:51.4469288Z 32 LL | fn without_assoc<'a,'b>() {
2019-08-09T15:52:51.4469519Z 
2019-08-09T15:52:51.4469756Z 34 
2019-08-09T15:52:51.4469756Z 34 
2019-08-09T15:52:51.4470032Z 35 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4470686Z +   --> $DIR/regions-outlives-projection-container.rs:63:12
2019-08-09T15:52:51.4470734Z 37    |
2019-08-09T15:52:51.4470734Z 37    |
2019-08-09T15:52:51.4470952Z 38 LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-09T15:52:51.4471128Z 
2019-08-09T15:52:51.4471175Z 40    |
2019-08-09T15:52:51.4471175Z 40    |
2019-08-09T15:52:51.4471454Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 62:20
2019-08-09T15:52:51.4471710Z -   --> $DIR/regions-outlives-projection-container.rs:62:20
2019-08-09T15:52:51.4471972Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 58:20
2019-08-09T15:52:51.4472208Z +   --> $DIR/regions-outlives-projection-container.rs:58:20
2019-08-09T15:52:51.4472283Z 43    |
2019-08-09T15:52:51.4472492Z 44 LL | fn call_with_assoc<'a,'b>() {
2019-08-09T15:52:51.4472588Z 
2019-08-09T15:52:51.4472588Z 
2019-08-09T15:52:51.4472863Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 62:23
2019-08-09T15:52:51.4473100Z -   --> $DIR/regions-outlives-projection-container.rs:62:23
2019-08-09T15:52:51.4473765Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 58:23
2019-08-09T15:52:51.4474019Z +   --> $DIR/regions-outlives-projection-container.rs:58:23
2019-08-09T15:52:51.4474068Z 48    |
2019-08-09T15:52:51.4474274Z 49 LL | fn call_with_assoc<'a,'b>() {
2019-08-09T15:52:51.4474371Z 
2019-08-09T15:52:51.4474411Z 51 
2019-08-09T15:52:51.4474411Z 51 
2019-08-09T15:52:51.4474716Z 52 error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4475322Z +   --> $DIR/regions-outlives-projection-container.rs:70:12
2019-08-09T15:52:51.4475389Z 54    |
2019-08-09T15:52:51.4475389Z 54    |
2019-08-09T15:52:51.4475613Z 55 LL |     call::<&'a WithoutAssoc<TheType<'b>>>();
2019-08-09T15:52:51.4475697Z 
2019-08-09T15:52:51.4475754Z 57    |
2019-08-09T15:52:51.4475754Z 57    |
2019-08-09T15:52:51.4476023Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 71:23
2019-08-09T15:52:51.4476262Z -   --> $DIR/regions-outlives-projection-container.rs:71:23
2019-08-09T15:52:51.4476541Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 67:23
2019-08-09T15:52:51.4476779Z +   --> $DIR/regions-outlives-projection-container.rs:67:23
2019-08-09T15:52:51.4477040Z 60    |
2019-08-09T15:52:51.4477658Z 61 LL | fn call_without_assoc<'a,'b>() {
2019-08-09T15:52:51.4477749Z 
2019-08-09T15:52:51.4477749Z 
2019-08-09T15:52:51.4478197Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 71:26
2019-08-09T15:52:51.4478453Z -   --> $DIR/regions-outlives-projection-container.rs:71:26
2019-08-09T15:52:51.4478733Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 67:26
2019-08-09T15:52:51.4478970Z +   --> $DIR/regions-outlives-projection-container.rs:67:26
2019-08-09T15:52:51.4479036Z 65    |
2019-08-09T15:52:51.4479258Z 66 LL | fn call_without_assoc<'a,'b>() {
2019-08-09T15:52:51.4479359Z 
2019-08-09T15:52:51.4479387Z 
2019-08-09T15:52:51.4479431Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4480720Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
2019-08-09T15:52:51.4480720Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
2019-08-09T15:52:51.4481026Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4481828Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container.rs`
2019-08-09T15:52:51.4481944Z error: 1 errors occurred comparing output.
2019-08-09T15:52:51.4481989Z status: exit code: 1
2019-08-09T15:52:51.4481989Z status: exit code: 1
2019-08-09T15:52:51.4482934Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4483625Z ------------------------------------------
2019-08-09T15:52:51.4483694Z 
2019-08-09T15:52:51.4483923Z ------------------------------------------
2019-08-09T15:52:51.4483969Z stderr:
2019-08-09T15:52:51.4483969Z stderr:
2019-08-09T15:52:51.4484194Z ------------------------------------------
2019-08-09T15:52:51.4484483Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4484830Z    |
2019-08-09T15:52:51.4484830Z    |
2019-08-09T15:52:51.4485060Z LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4485170Z    |
2019-08-09T15:52:51.4485170Z    |
2019-08-09T15:52:51.4485428Z note: the pointer is valid for the lifetime 'a as defined on the function body at 28:15
2019-08-09T15:52:51.4485877Z    |
2019-08-09T15:52:51.4485877Z    |
2019-08-09T15:52:51.4486100Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4486148Z    |               ^^
2019-08-09T15:52:51.4486423Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 28:18
2019-08-09T15:52:51.4486904Z    |
2019-08-09T15:52:51.4486904Z    |
2019-08-09T15:52:51.4487242Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:52:51.4487338Z 
2019-08-09T15:52:51.4487338Z 
2019-08-09T15:52:51.4487903Z error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4488185Z    |
2019-08-09T15:52:51.4488185Z    |
2019-08-09T15:52:51.4488375Z LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-09T15:52:51.4488477Z    |
2019-08-09T15:52:51.4488477Z    |
2019-08-09T15:52:51.4488697Z note: the pointer is valid for the lifetime 'a as defined on the function body at 50:18
2019-08-09T15:52:51.4489145Z    |
2019-08-09T15:52:51.4489145Z    |
2019-08-09T15:52:51.4489505Z LL | fn without_assoc<'a,'b>() {
2019-08-09T15:52:51.4489549Z    |                  ^^
2019-08-09T15:52:51.4489982Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 50:21
2019-08-09T15:52:51.4490281Z    |
2019-08-09T15:52:51.4490281Z    |
2019-08-09T15:52:51.4490712Z LL | fn without_assoc<'a,'b>() {
2019-08-09T15:52:51.4490788Z 
2019-08-09T15:52:51.4490788Z 
2019-08-09T15:52:51.4491233Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4491568Z    |
2019-08-09T15:52:51.4491568Z    |
2019-08-09T15:52:51.4491786Z LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-09T15:52:51.4491896Z    |
2019-08-09T15:52:51.4491896Z    |
2019-08-09T15:52:51.4492153Z note: the pointer is valid for the lifetime 'a as defined on the function body at 58:20
2019-08-09T15:52:51.4492478Z    |
2019-08-09T15:52:51.4492478Z    |
2019-08-09T15:52:51.4492755Z LL | fn call_with_assoc<'a,'b>() {
2019-08-09T15:52:51.4492831Z    |                    ^^
2019-08-09T15:52:51.4493418Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 58:23
2019-08-09T15:52:51.4493815Z    |
2019-08-09T15:52:51.4493815Z    |
2019-08-09T15:52:51.4494023Z LL | fn call_with_assoc<'a,'b>() {
2019-08-09T15:52:51.4494115Z 
2019-08-09T15:52:51.4494115Z 
2019-08-09T15:52:51.4494415Z error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:52:51.4494726Z    |
2019-08-09T15:52:51.4494726Z    |
2019-08-09T15:52:51.4494999Z LL |     call::<&'a WithoutAssoc<TheType<'b>>>(); //~ ERROR reference has a longer lifetime
2019-08-09T15:52:51.4495108Z    |
2019-08-09T15:52:51.4495108Z    |
2019-08-09T15:52:51.4495381Z note: the pointer is valid for the lifetime 'a as defined on the function body at 67:23
2019-08-09T15:52:51.4495692Z    |
2019-08-09T15:52:51.4495692Z    |
2019-08-09T15:52:51.4495898Z LL | fn call_without_assoc<'a,'b>() {
2019-08-09T15:52:51.4495962Z    |                       ^^
2019-08-09T15:52:51.4496236Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 67:26
2019-08-09T15:52:51.4496696Z    |
2019-08-09T15:52:51.4496696Z    |
2019-08-09T15:52:51.4496904Z LL | fn call_without_assoc<'a,'b>() {
2019-08-09T15:52:51.4497001Z 
2019-08-09T15:52:51.4497044Z error: aborting due to 4 previous errors
2019-08-09T15:52:51.4497074Z 
2019-08-09T15:52:51.4497321Z For more information about this error, try `rustc --explain E0491`.
2019-08-09T15:52:51.4497321Z For more information about this error, try `rustc --explain E0491`.
2019-08-09T15:52:51.4497374Z 
2019-08-09T15:52:51.4497590Z ------------------------------------------
2019-08-09T15:52:51.4497623Z 
2019-08-09T15:52:51.4497649Z 
2019-08-09T15:52:51.4497895Z ---- [ui] ui/specialization/defaultimpl/specialization-no-default.rs stdout ----
2019-08-09T15:52:51.4497965Z diff of stderr:
2019-08-09T15:52:51.4497994Z 
2019-08-09T15:52:51.4498045Z 1 error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4498845Z +   --> $DIR/specialization-no-default.rs:20:5
2019-08-09T15:52:51.4498907Z 3    |
2019-08-09T15:52:51.4498907Z 3    |
2019-08-09T15:52:51.4498970Z 4 LL | / impl<T> Foo for T {
2019-08-09T15:52:51.4499016Z 5 LL | |     fn foo(&self) {}
2019-08-09T15:52:51.4499046Z 
2019-08-09T15:52:51.4499096Z 13    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4499161Z 14 
2019-08-09T15:52:51.4499226Z 15 error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4499733Z +   --> $DIR/specialization-no-default.rs:23:5
2019-08-09T15:52:51.4499780Z 17    |
2019-08-09T15:52:51.4499780Z 17    |
2019-08-09T15:52:51.4499823Z 18 LL | / impl<T> Foo for T {
2019-08-09T15:52:51.4499885Z 19 LL | |     fn foo(&self) {}
2019-08-09T15:52:51.4499915Z 
2019-08-09T15:52:51.4499964Z 27    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4500020Z 28 
2019-08-09T15:52:51.4500088Z 29 error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4500541Z +   --> $DIR/specialization-no-default.rs:37:5
2019-08-09T15:52:51.4500604Z 31    |
2019-08-09T15:52:51.4500604Z 31    |
2019-08-09T15:52:51.4500647Z 32 LL | / impl<T> Bar for T {
2019-08-09T15:52:51.4500692Z 33 LL | |     type T = u8;
2019-08-09T15:52:51.4500840Z 
2019-08-09T15:52:51.4500914Z 40    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4500960Z 41 
2019-08-09T15:52:51.4501011Z 42 error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4501502Z +   --> $DIR/specialization-no-default.rs:55:5
2019-08-09T15:52:51.4501559Z 44    |
2019-08-09T15:52:51.4501559Z 44    |
2019-08-09T15:52:51.4501619Z 45 LL | / impl<T: Clone> Baz for T {
2019-08-09T15:52:51.4501666Z 46 LL | |     fn baz(&self) {}
2019-08-09T15:52:51.4501695Z 
2019-08-09T15:52:51.4501744Z 53    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4501805Z 54 
2019-08-09T15:52:51.4501857Z 55 error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4502482Z +   --> $DIR/specialization-no-default.rs:74:5
2019-08-09T15:52:51.4502527Z 57    |
2019-08-09T15:52:51.4502527Z 57    |
2019-08-09T15:52:51.4502571Z 58 LL | / impl<T: Clone> Redundant for T {
2019-08-09T15:52:51.4502788Z 59 LL | |     fn redundant(&self) {}
2019-08-09T15:52:51.4502843Z 
2019-08-09T15:52:51.4502886Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4503783Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/specialization-no-default.stderr
2019-08-09T15:52:51.4503783Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/specialization-no-default.stderr
2019-08-09T15:52:51.4504214Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4504507Z To only update this specific test, also pass `--test-args specialization/defaultimpl/specialization-no-default.rs`
2019-08-09T15:52:51.4504610Z error: 1 errors occurred comparing output.
2019-08-09T15:52:51.4504654Z status: exit code: 1
2019-08-09T15:52:51.4504654Z status: exit code: 1
2019-08-09T15:52:51.4505751Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/defaultimpl/specialization-no-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4506184Z ------------------------------------------
2019-08-09T15:52:51.4506221Z 
2019-08-09T15:52:51.4506437Z ------------------------------------------
2019-08-09T15:52:51.4506482Z stderr:
2019-08-09T15:52:51.4506482Z stderr:
2019-08-09T15:52:51.4506709Z ------------------------------------------
2019-08-09T15:52:51.4506775Z error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4507120Z    |
2019-08-09T15:52:51.4507120Z    |
2019-08-09T15:52:51.4507164Z LL | / impl<T> Foo for T {
2019-08-09T15:52:51.4507209Z LL | |     fn foo(&self) {}
2019-08-09T15:52:51.4507315Z LL | |     fn bar(&self) {}
2019-08-09T15:52:51.4507359Z LL | | }
2019-08-09T15:52:51.4507563Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4507634Z ...
2019-08-09T15:52:51.4507680Z LL |       fn foo(&self) {} //~ ERROR E0520
2019-08-09T15:52:51.4507730Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `foo`
2019-08-09T15:52:51.4507793Z    |
2019-08-09T15:52:51.4507842Z    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4507875Z 
2019-08-09T15:52:51.4507925Z error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4508380Z    |
2019-08-09T15:52:51.4508380Z    |
2019-08-09T15:52:51.4508422Z LL | / impl<T> Foo for T {
2019-08-09T15:52:51.4508483Z LL | |     fn foo(&self) {}
2019-08-09T15:52:51.4508528Z LL | |     fn bar(&self) {}
2019-08-09T15:52:51.4508570Z LL | | }
2019-08-09T15:52:51.4508792Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4508854Z ...
2019-08-09T15:52:51.4508899Z LL |       fn bar(&self) {} //~ ERROR E0520
2019-08-09T15:52:51.4508958Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `bar`
2019-08-09T15:52:51.4509022Z    |
2019-08-09T15:52:51.4509070Z    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4509104Z 
2019-08-09T15:52:51.4509173Z error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4509492Z    |
2019-08-09T15:52:51.4509492Z    |
2019-08-09T15:52:51.4509542Z LL | / impl<T> Bar for T {
2019-08-09T15:52:51.4509604Z LL | |     type T = u8;
2019-08-09T15:52:51.4509647Z LL | | }
2019-08-09T15:52:51.4509852Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4509913Z ...
2019-08-09T15:52:51.4509959Z LL |       type T = (); //~ ERROR E0520
2019-08-09T15:52:51.4510012Z    |       ^^^^^^^^^^^^ cannot specialize default item `T`
2019-08-09T15:52:51.4510073Z    |
2019-08-09T15:52:51.4510122Z    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4510233Z 
2019-08-09T15:52:51.4510285Z error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4510782Z    |
2019-08-09T15:52:51.4510782Z    |
2019-08-09T15:52:51.4510824Z LL | / impl<T: Clone> Baz for T {
2019-08-09T15:52:51.4510884Z LL | |     fn baz(&self) {}
2019-08-09T15:52:51.4510925Z LL | | }
2019-08-09T15:52:51.4511300Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4511363Z ...
2019-08-09T15:52:51.4511409Z LL |       fn baz(&self) {} //~ ERROR E0520
2019-08-09T15:52:51.4511458Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `baz`
2019-08-09T15:52:51.4511503Z    |
2019-08-09T15:52:51.4511725Z    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4511758Z 
2019-08-09T15:52:51.4511808Z error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4512544Z    |
2019-08-09T15:52:51.4512544Z    |
2019-08-09T15:52:51.4512587Z LL | / impl<T: Clone> Redundant for T {
2019-08-09T15:52:51.4512805Z LL | |     fn redundant(&self) {}
2019-08-09T15:52:51.4512847Z LL | | }
2019-08-09T15:52:51.4513516Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4513571Z ...
2019-08-09T15:52:51.4513637Z LL |       fn redundant(&self) {} //~ ERROR E0520
2019-08-09T15:52:51.4513700Z    |       ^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `redundant`
2019-08-09T15:52:51.4513746Z    |
2019-08-09T15:52:51.4513812Z    = note: to specialize, `redundant` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4513891Z error: aborting due to 5 previous errors
2019-08-09T15:52:51.4513920Z 
2019-08-09T15:52:51.4514189Z For more information about this error, try `rustc --explain E0520`.
2019-08-09T15:52:51.4514233Z 
2019-08-09T15:52:51.4514233Z 
2019-08-09T15:52:51.4514445Z ------------------------------------------
2019-08-09T15:52:51.4514478Z 
2019-08-09T15:52:51.4514519Z 
2019-08-09T15:52:51.4514759Z ---- [ui] ui/specialization/specialization-no-default.rs stdout ----
2019-08-09T15:52:51.4514809Z diff of stderr:
2019-08-09T15:52:51.4514838Z 
2019-08-09T15:52:51.4514906Z 1 error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4515450Z +   --> $DIR/specialization-no-default.rs:20:5
2019-08-09T15:52:51.4515523Z 3    |
2019-08-09T15:52:51.4515523Z 3    |
2019-08-09T15:52:51.4515568Z 4 LL | / impl<T> Foo for T {
2019-08-09T15:52:51.4515613Z 5 LL | |     fn foo(&self) {}
2019-08-09T15:52:51.4515644Z 
2019-08-09T15:52:51.4515708Z 13    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4515755Z 14 
2019-08-09T15:52:51.4515807Z 15 error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4516305Z +   --> $DIR/specialization-no-default.rs:23:5
2019-08-09T15:52:51.4516352Z 17    |
2019-08-09T15:52:51.4516352Z 17    |
2019-08-09T15:52:51.4516411Z 18 LL | / impl<T> Foo for T {
2019-08-09T15:52:51.4516457Z 19 LL | |     fn foo(&self) {}
2019-08-09T15:52:51.4516486Z 
2019-08-09T15:52:51.4516534Z 27    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4516596Z 28 
2019-08-09T15:52:51.4516655Z 29 error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4517380Z +   --> $DIR/specialization-no-default.rs:37:5
2019-08-09T15:52:51.4517434Z 31    |
2019-08-09T15:52:51.4517434Z 31    |
2019-08-09T15:52:51.4517476Z 32 LL | / impl<T> Bar for T {
2019-08-09T15:52:51.4517521Z 33 LL | |     type T = u8;
2019-08-09T15:52:51.4517569Z 
2019-08-09T15:52:51.4517715Z 40    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4517760Z 41 
2019-08-09T15:52:51.4517831Z 42 error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4518324Z +   --> $DIR/specialization-no-default.rs:55:5
2019-08-09T15:52:51.4518390Z 44    |
2019-08-09T15:52:51.4518390Z 44    |
2019-08-09T15:52:51.4518434Z 45 LL | / impl<T: Clone> Baz for T {
2019-08-09T15:52:51.4518489Z 46 LL | |     fn baz(&self) {}
2019-08-09T15:52:51.4518520Z 
2019-08-09T15:52:51.4518585Z 53    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4518631Z 54 
2019-08-09T15:52:51.4518683Z 55 error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4519152Z +   --> $DIR/specialization-no-default.rs:74:5
2019-08-09T15:52:51.4519207Z 57    |
2019-08-09T15:52:51.4519207Z 57    |
2019-08-09T15:52:51.4519269Z 58 LL | / impl<T: Clone> Redundant for T {
2019-08-09T15:52:51.4519317Z 59 LL | |     fn redundant(&self) {}
2019-08-09T15:52:51.4519372Z 
2019-08-09T15:52:51.4519572Z The actual stderr differed from the expected stderr.
2019-08-09T15:52:51.4519910Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/specialization-no-default.stderr
2019-08-09T15:52:51.4519910Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/specialization-no-default.stderr
2019-08-09T15:52:51.4520163Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:52:51.4520450Z To only update this specific test, also pass `--test-args specialization/specialization-no-default.rs`
2019-08-09T15:52:51.4520529Z error: 1 errors occurred comparing output.
2019-08-09T15:52:51.4520573Z status: exit code: 1
2019-08-09T15:52:51.4520573Z status: exit code: 1
2019-08-09T15:52:51.4521523Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/specialization-no-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/auxiliary" "-A" "unused"
2019-08-09T15:52:51.4521941Z ------------------------------------------
2019-08-09T15:52:51.4521983Z 
2019-08-09T15:52:51.4522218Z ------------------------------------------
2019-08-09T15:52:51.4522283Z stderr:
2019-08-09T15:52:51.4522283Z stderr:
2019-08-09T15:52:51.4522496Z ------------------------------------------
2019-08-09T15:52:51.4522552Z error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4522889Z    |
2019-08-09T15:52:51.4522889Z    |
2019-08-09T15:52:51.4522932Z LL | / impl<T> Foo for T {
2019-08-09T15:52:51.4522994Z LL | |     fn foo(&self) {}
2019-08-09T15:52:51.4523040Z LL | |     fn bar(&self) {}
2019-08-09T15:52:51.4523082Z LL | | }
2019-08-09T15:52:51.4523829Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4523886Z ...
2019-08-09T15:52:51.4523930Z LL |       fn foo(&self) {} //~ ERROR E0520
2019-08-09T15:52:51.4523979Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `foo`
2019-08-09T15:52:51.4524053Z    |
2019-08-09T15:52:51.4524101Z    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4524136Z 
2019-08-09T15:52:51.4524203Z error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4524526Z    |
2019-08-09T15:52:51.4524526Z    |
2019-08-09T15:52:51.4524586Z LL | / impl<T> Foo for T {
2019-08-09T15:52:51.4524739Z LL | |     fn foo(&self) {}
2019-08-09T15:52:51.4524784Z LL | |     fn bar(&self) {}
2019-08-09T15:52:51.4524828Z LL | | }
2019-08-09T15:52:51.4525072Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4525118Z ...
2019-08-09T15:52:51.4525163Z LL |       fn bar(&self) {} //~ ERROR E0520
2019-08-09T15:52:51.4525231Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `bar`
2019-08-09T15:52:51.4525276Z    |
2019-08-09T15:52:51.4525334Z    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4525368Z 
2019-08-09T15:52:51.4525436Z error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4525743Z    |
2019-08-09T15:52:51.4525743Z    |
2019-08-09T15:52:51.4525804Z LL | / impl<T> Bar for T {
2019-08-09T15:52:51.4525848Z LL | |     type T = u8;
2019-08-09T15:52:51.4525890Z LL | | }
2019-08-09T15:52:51.4526124Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4526169Z ...
2019-08-09T15:52:51.4526213Z LL |       type T = (); //~ ERROR E0520
2019-08-09T15:52:51.4526264Z    |       ^^^^^^^^^^^^ cannot specialize default item `T`
2019-08-09T15:52:51.4526326Z    |
2019-08-09T15:52:51.4526375Z    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4526408Z 
2019-08-09T15:52:51.4526475Z error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4526945Z    |
2019-08-09T15:52:51.4526945Z    |
2019-08-09T15:52:51.4527003Z LL | / impl<T: Clone> Baz for T {
2019-08-09T15:52:51.4527221Z LL | |     fn baz(&self) {}
2019-08-09T15:52:51.4527262Z LL | | }
2019-08-09T15:52:51.4527515Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4527572Z ...
2019-08-09T15:52:51.4527615Z LL |       fn baz(&self) {} //~ ERROR E0520
2019-08-09T15:52:51.4527663Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `baz`
2019-08-09T15:52:51.4527733Z    |
2019-08-09T15:52:51.4527779Z    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4527810Z 
2019-08-09T15:52:51.4527859Z error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:52:51.4528171Z    |
2019-08-09T15:52:51.4528171Z    |
2019-08-09T15:52:51.4528281Z LL | / impl<T: Clone> Redundant for T {
2019-08-09T15:52:51.4528353Z LL | |     fn redundant(&self) {}
2019-08-09T15:52:51.4528395Z LL | | }
2019-08-09T15:52:51.4528780Z    | |_- parent `impl` is here
2019-08-09T15:52:51.4528841Z ...
2019-08-09T15:52:51.4528887Z LL |       default fn redundant(&self) {} //~ ERROR E0520
2019-08-09T15:52:51.4528940Z    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `redundant`
2019-08-09T15:52:51.4529006Z    |
2019-08-09T15:52:51.4529065Z    = note: to specialize, `redundant` in the parent `impl` must be marked `default`
2019-08-09T15:52:51.4529141Z error: aborting due to 5 previous errors
2019-08-09T15:52:51.4529188Z 
2019-08-09T15:52:51.4529433Z For more information about this error, try `rustc --explain E0520`.
2019-08-09T15:52:51.4529468Z 
---
2019-08-09T15:52:51.4535387Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-09T15:52:51.4535449Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-09T15:52:51.4535483Z 
2019-08-09T15:52:51.4535526Z 
2019-08-09T15:52:51.4537124Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-09T15:52:51.4537731Z 
2019-08-09T15:52:51.4537761Z 
2019-08-09T15:52:51.4537808Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-09T15:52:51.4537862Z Build completed unsuccessfully in 1:10:31
2019-08-09T15:52:51.4537862Z Build completed unsuccessfully in 1:10:31
2019-08-09T15:52:52.2025099Z ##[error]Bash exited with code '1'.
2019-08-09T15:52:52.2066370Z ##[section]Starting: Checkout
2019-08-09T15:52:52.2068069Z ==============================================================================
2019-08-09T15:52:52.2068126Z Task         : Get sources
2019-08-09T15:52:52.2068194Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
