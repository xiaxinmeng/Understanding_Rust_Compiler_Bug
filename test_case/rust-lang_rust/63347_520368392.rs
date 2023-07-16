plain
2019-08-12T09:00:38.4834305Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T09:00:38.4994784Z ##[command]git config gc.auto 0
2019-08-12T09:00:38.5075505Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T09:00:38.5146100Z ##[command]git config --get-all http.proxy
2019-08-12T09:00:38.5294150Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63347/merge:refs/remotes/pull/63347/merge
---
2019-08-12T09:01:20.4441538Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T09:01:20.4443038Z 
2019-08-12T09:01:20.4444794Z   git checkout -b <new-branch-name>
2019-08-12T09:01:20.4446365Z 
2019-08-12T09:01:20.4447205Z HEAD is now at 90115fcb8 Merge 02776dd1b5d9d34c69cd335f21e03e20bd3c5850 into 72f8043d44a8925e469daf5c10e2630c80c2a7d4
2019-08-12T09:01:20.4605215Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T09:01:20.4608388Z ==============================================================================
2019-08-12T09:01:20.4608478Z Task         : Bash
2019-08-12T09:01:20.4608529Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-12T10:02:02.1540344Z .................................................................................................... 1300/8872
2019-08-12T10:02:08.6979682Z .................................................................................................... 1400/8872
2019-08-12T10:02:14.8561888Z .................................................................................................... 1500/8872
2019-08-12T10:02:25.4944331Z ....................................................................................i............... 1600/8872
2019-08-12T10:02:33.2059022Z i................................................................................................... 1700/8872
2019-08-12T10:02:39.9874189Z ...........................................................................iiiii.................... 1800/8872
2019-08-12T10:03:02.3608212Z .................................................................................................... 2000/8872
2019-08-12T10:03:04.8533665Z .................................................................................................... 2100/8872
2019-08-12T10:03:07.6372436Z .................................................................................................... 2200/8872
2019-08-12T10:03:15.4410507Z .................................................................................................... 2300/8872
---
2019-08-12T10:07:12.3548810Z .................................................................................................... 5300/8872
2019-08-12T10:07:19.7261051Z .......i............................................................................................ 5400/8872
2019-08-12T10:07:25.1147481Z .................................................................................................... 5500/8872
2019-08-12T10:07:37.5606233Z .................................................................................................... 5600/8872
2019-08-12T10:07:51.7800120Z ..ii...i..ii...........i............................................................................ 5700/8872
2019-08-12T10:08:07.5993510Z .................................................................................................... 5900/8872
2019-08-12T10:08:12.3565881Z .................................................................................................... 6000/8872
2019-08-12T10:08:12.3565881Z .................................................................................................... 6000/8872
2019-08-12T10:08:26.6335409Z ...i..ii............................................................................................ 6100/8872
2019-08-12T10:08:45.4232937Z ..............................................i..................................................... 6300/8872
2019-08-12T10:08:47.6029426Z .................................................................................................... 6400/8872
2019-08-12T10:08:50.1291046Z ..................i................................................................................. 6500/8872
2019-08-12T10:08:54.6815684Z .................................................................................................... 6600/8872
2019-08-12T10:08:54.6815684Z .................................................................................................... 6600/8872
2019-08-12T10:09:11.9071103Z .................................................................................................... 6700/8872
2019-08-12T10:09:34.2675596Z .................................................................................................... 6800/8872
2019-08-12T10:09:39.5366612Z ..............................................................FF.................................... 6900/8872
2019-08-12T10:09:44.8494866Z .................................................................................F.FFFF............. 7000/8872
2019-08-12T10:09:55.4446133Z .................................................................................................... 7200/8872
2019-08-12T10:10:05.5326689Z .................................................................................................... 7300/8872
2019-08-12T10:10:15.2616822Z .................................................................................................... 7400/8872
2019-08-12T10:10:26.1540615Z ...............................................ii......i............................................ 7500/8872
---
2019-08-12T10:12:55.1325649Z 1 error[E0308]: mismatched types
2019-08-12T10:12:55.1325959Z -   --> $DIR/associated-type-projection-from-supertrait.rs:33:23
2019-08-12T10:12:55.1326222Z +   --> $DIR/associated-type-projection-from-supertrait.rs:27:23
2019-08-12T10:12:55.1326287Z 3    |
2019-08-12T10:12:55.1326356Z 4 LL | fn b() { dent(ModelT, Blue); }
2019-08-12T10:12:55.1326406Z 5    |                       ^^^^ expected struct `Black`, found struct `Blue`
2019-08-12T10:12:55.1326496Z 8               found type `Blue`
2019-08-12T10:12:55.1326538Z 9 
2019-08-12T10:12:55.1326579Z 10 error[E0308]: mismatched types
2019-08-12T10:12:55.1326846Z -   --> $DIR/associated-type-projection-from-supertrait.rs:34:23
2019-08-12T10:12:55.1326846Z -   --> $DIR/associated-type-projection-from-supertrait.rs:34:23
2019-08-12T10:12:55.1327122Z +   --> $DIR/associated-type-projection-from-supertrait.rs:28:23
2019-08-12T10:12:55.1327170Z 12    |
2019-08-12T10:12:55.1327211Z 13 LL | fn c() { dent(ModelU, Black); }
2019-08-12T10:12:55.1327274Z 14    |                       ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-12T10:12:55.1327346Z 17               found type `Black`
2019-08-12T10:12:55.1327387Z 18 
2019-08-12T10:12:55.1327445Z 19 error[E0308]: mismatched types
2019-08-12T10:12:55.1327761Z -   --> $DIR/associated-type-projection-from-supertrait.rs:40:28
2019-08-12T10:12:55.1327761Z -   --> $DIR/associated-type-projection-from-supertrait.rs:40:28
2019-08-12T10:12:55.1328042Z +   --> $DIR/associated-type-projection-from-supertrait.rs:32:28
2019-08-12T10:12:55.1328105Z 21    |
2019-08-12T10:12:55.1328150Z 22 LL | fn f() { ModelT.chip_paint(Blue); }
2019-08-12T10:12:55.1328200Z 23    |                            ^^^^ expected struct `Black`, found struct `Blue`
2019-08-12T10:12:55.1328395Z 26               found type `Blue`
2019-08-12T10:12:55.1328438Z 27 
2019-08-12T10:12:55.1328477Z 28 error[E0308]: mismatched types
2019-08-12T10:12:55.1328957Z -   --> $DIR/associated-type-projection-from-supertrait.rs:41:28
2019-08-12T10:12:55.1328957Z -   --> $DIR/associated-type-projection-from-supertrait.rs:41:28
2019-08-12T10:12:55.1329182Z +   --> $DIR/associated-type-projection-from-supertrait.rs:33:28
2019-08-12T10:12:55.1329222Z 30    |
2019-08-12T10:12:55.1329273Z 31 LL | fn g() { ModelU.chip_paint(Black); }
2019-08-12T10:12:55.1329316Z 32    |                            ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-12T10:12:55.1329373Z 
2019-08-12T10:12:55.1329580Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1329958Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
2019-08-12T10:12:55.1329958Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
2019-08-12T10:12:55.1330196Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1330496Z To only update this specific test, also pass `--test-args associated-type/associated-type-projection-from-supertrait.rs`
2019-08-12T10:12:55.1330569Z error: 1 errors occurred comparing output.
2019-08-12T10:12:55.1330621Z status: exit code: 1
2019-08-12T10:12:55.1330621Z status: exit code: 1
2019-08-12T10:12:55.1331584Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1332068Z ------------------------------------------
2019-08-12T10:12:55.1332100Z 
2019-08-12T10:12:55.1332323Z ------------------------------------------
2019-08-12T10:12:55.1332362Z stderr:
2019-08-12T10:12:55.1332362Z stderr:
2019-08-12T10:12:55.1332558Z ------------------------------------------
2019-08-12T10:12:55.1332614Z error[E0308]: mismatched types
2019-08-12T10:12:55.1332866Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:27:23
2019-08-12T10:12:55.1332912Z    |
2019-08-12T10:12:55.1332969Z LL | fn b() { dent(ModelT, Blue); } //~ ERROR mismatched types
2019-08-12T10:12:55.1333029Z    |                       ^^^^ expected struct `Black`, found struct `Blue`
2019-08-12T10:12:55.1333118Z    = note: expected type `Black`
2019-08-12T10:12:55.1333156Z               found type `Blue`
2019-08-12T10:12:55.1333180Z 
2019-08-12T10:12:55.1333216Z error[E0308]: mismatched types
2019-08-12T10:12:55.1333216Z error[E0308]: mismatched types
2019-08-12T10:12:55.1333483Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:28:23
2019-08-12T10:12:55.1333528Z    |
2019-08-12T10:12:55.1333567Z LL | fn c() { dent(ModelU, Black); } //~ ERROR mismatched types
2019-08-12T10:12:55.1333626Z    |                       ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-12T10:12:55.1333698Z    = note: expected type `Blue`
2019-08-12T10:12:55.1333735Z               found type `Black`
2019-08-12T10:12:55.1333776Z 
2019-08-12T10:12:55.1333810Z error[E0308]: mismatched types
2019-08-12T10:12:55.1333810Z error[E0308]: mismatched types
2019-08-12T10:12:55.1334058Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:32:28
2019-08-12T10:12:55.1334125Z    |
2019-08-12T10:12:55.1334173Z LL | fn f() { ModelT.chip_paint(Blue); } //~ ERROR mismatched types
2019-08-12T10:12:55.1334220Z    |                            ^^^^ expected struct `Black`, found struct `Blue`
2019-08-12T10:12:55.1334308Z    = note: expected type `Black`
2019-08-12T10:12:55.1334345Z               found type `Blue`
2019-08-12T10:12:55.1334369Z 
2019-08-12T10:12:55.1334419Z error[E0308]: mismatched types
2019-08-12T10:12:55.1334419Z error[E0308]: mismatched types
2019-08-12T10:12:55.1335131Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:33:28
2019-08-12T10:12:55.1335188Z    |
2019-08-12T10:12:55.1335252Z LL | fn g() { ModelU.chip_paint(Black); } //~ ERROR mismatched types
2019-08-12T10:12:55.1335306Z    |                            ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-12T10:12:55.1335414Z    = note: expected type `Blue`
2019-08-12T10:12:55.1335472Z               found type `Black`
2019-08-12T10:12:55.1335502Z 
2019-08-12T10:12:55.1335666Z error: aborting due to 4 previous errors
---
2019-08-12T10:12:55.1336451Z 
2019-08-12T10:12:55.1336752Z ---- [ui] ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs stdout ----
2019-08-12T10:12:55.1336806Z diff of stderr:
2019-08-12T10:12:55.1336835Z 
2019-08-12T10:12:55.1336897Z 1 error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
2019-08-12T10:12:55.1337470Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:31:10
2019-08-12T10:12:55.1337538Z 3    |
2019-08-12T10:12:55.1337538Z 3    |
2019-08-12T10:12:55.1337667Z 4 LL | fn b() { blue_car(ModelT); }
2019-08-12T10:12:55.1337725Z 5    |          ^^^^^^^^ expected struct `Black`, found struct `Blue`
2019-08-12T10:12:55.1337817Z 7    = note: expected type `Black`
2019-08-12T10:12:55.1337863Z 8               found type `Blue`
2019-08-12T10:12:55.1337863Z 8               found type `Blue`
2019-08-12T10:12:55.1337906Z 9 note: required by `blue_car`
2019-08-12T10:12:55.1338784Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:27:1
2019-08-12T10:12:55.1338826Z 11    |
2019-08-12T10:12:55.1338826Z 11    |
2019-08-12T10:12:55.1338880Z 12 LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
2019-08-12T10:12:55.1338946Z 
2019-08-12T10:12:55.1338992Z 14 
2019-08-12T10:12:55.1338992Z 14 
2019-08-12T10:12:55.1339034Z 15 error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
2019-08-12T10:12:55.1339522Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:32:10
2019-08-12T10:12:55.1339582Z 17    |
2019-08-12T10:12:55.1339582Z 17    |
2019-08-12T10:12:55.1339619Z 18 LL | fn c() { black_car(ModelU); }
2019-08-12T10:12:55.1339660Z 19    |          ^^^^^^^^^ expected struct `Blue`, found struct `Black`
2019-08-12T10:12:55.1339738Z 21    = note: expected type `Blue`
2019-08-12T10:12:55.1339776Z 22               found type `Black`
2019-08-12T10:12:55.1339776Z 22               found type `Black`
2019-08-12T10:12:55.1339828Z 23 note: required by `black_car`
2019-08-12T10:12:55.1340296Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:24:1
2019-08-12T10:12:55.1340337Z 25    |
2019-08-12T10:12:55.1340337Z 25    |
2019-08-12T10:12:55.1340389Z 26 LL | fn black_car<C:Car<Color=Black>>(c: C) {
2019-08-12T10:12:55.1340455Z 
2019-08-12T10:12:55.1340476Z 
2019-08-12T10:12:55.1340537Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1340897Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
2019-08-12T10:12:55.1340897Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
2019-08-12T10:12:55.1341154Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1341444Z To only update this specific test, also pass `--test-args associated-types/associated-types-binding-to-type-defined-in-supertrait.rs`
2019-08-12T10:12:55.1341515Z error: 1 errors occurred comparing output.
2019-08-12T10:12:55.1341567Z status: exit code: 1
2019-08-12T10:12:55.1341567Z status: exit code: 1
2019-08-12T10:12:55.1342402Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1342756Z ------------------------------------------
2019-08-12T10:12:55.1342802Z 
2019-08-12T10:12:55.1343005Z ------------------------------------------
2019-08-12T10:12:55.1343045Z stderr:
2019-08-12T10:12:55.1343045Z stderr:
2019-08-12T10:12:55.1343242Z ------------------------------------------
2019-08-12T10:12:55.1343307Z error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
2019-08-12T10:12:55.1343731Z    |
2019-08-12T10:12:55.1343731Z    |
2019-08-12T10:12:55.1343775Z LL | fn b() { blue_car(ModelT); } //~ ERROR type mismatch
2019-08-12T10:12:55.1343827Z    |          ^^^^^^^^ expected struct `Black`, found struct `Blue`
2019-08-12T10:12:55.1343918Z    = note: expected type `Black`
2019-08-12T10:12:55.1343957Z               found type `Blue`
2019-08-12T10:12:55.1343957Z               found type `Blue`
2019-08-12T10:12:55.1343996Z note: required by `blue_car`
2019-08-12T10:12:55.1344362Z    |
2019-08-12T10:12:55.1344362Z    |
2019-08-12T10:12:55.1344401Z LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
2019-08-12T10:12:55.1344484Z 
2019-08-12T10:12:55.1344484Z 
2019-08-12T10:12:55.1344526Z error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
2019-08-12T10:12:55.1345328Z    |
2019-08-12T10:12:55.1345328Z    |
2019-08-12T10:12:55.1345374Z LL | fn c() { black_car(ModelU); } //~ ERROR type mismatch
2019-08-12T10:12:55.1345430Z    |          ^^^^^^^^^ expected struct `Blue`, found struct `Black`
2019-08-12T10:12:55.1345530Z    = note: expected type `Blue`
2019-08-12T10:12:55.1345573Z               found type `Black`
2019-08-12T10:12:55.1345573Z               found type `Black`
2019-08-12T10:12:55.1345633Z note: required by `black_car`
2019-08-12T10:12:55.1345987Z    |
2019-08-12T10:12:55.1345987Z    |
2019-08-12T10:12:55.1346045Z LL | fn black_car<C:Car<Color=Black>>(c: C) {
2019-08-12T10:12:55.1346123Z 
2019-08-12T10:12:55.1346165Z error: aborting due to 2 previous errors
2019-08-12T10:12:55.1346210Z 
2019-08-12T10:12:55.1346467Z For more information about this error, try `rustc --explain E0271`.
2019-08-12T10:12:55.1346467Z For more information about this error, try `rustc --explain E0271`.
2019-08-12T10:12:55.1346503Z 
2019-08-12T10:12:55.1346729Z ------------------------------------------
2019-08-12T10:12:55.1346787Z 
2019-08-12T10:12:55.1346812Z 
2019-08-12T10:12:55.1347061Z ---- [ui] ui/hrtb/hrtb-conflate-regions.rs stdout ----
2019-08-12T10:12:55.1347111Z diff of stderr:
2019-08-12T10:12:55.1347152Z 
2019-08-12T10:12:55.1347438Z 1 error[E0277]: the trait bound `for<'a, 'b> SomeStruct: Foo<(&'a isize, &'b isize)>` is not satisfied
2019-08-12T10:12:55.1347679Z -   --> $DIR/hrtb-conflate-regions.rs:28:10
2019-08-12T10:12:55.1347929Z +   --> $DIR/hrtb-conflate-regions.rs:27:10
2019-08-12T10:12:55.1347974Z 3    |
2019-08-12T10:12:55.1348018Z 4 LL | fn b() { want_foo2::<SomeStruct>(); }
2019-08-12T10:12:55.1348425Z 5    |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a, 'b> Foo<(&'a isize, &'b isize)>` is not implemented for `SomeStruct`
2019-08-12T10:12:55.1348501Z 
2019-08-12T10:12:55.1348544Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1348851Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/hrtb-conflate-regions.stderr
2019-08-12T10:12:55.1348851Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/hrtb-conflate-regions.stderr
2019-08-12T10:12:55.1349234Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1349540Z To only update this specific test, also pass `--test-args hrtb/hrtb-conflate-regions.rs`
2019-08-12T10:12:55.1349633Z error: 1 errors occurred comparing output.
2019-08-12T10:12:55.1349674Z status: exit code: 1
2019-08-12T10:12:55.1349674Z status: exit code: 1
2019-08-12T10:12:55.1350384Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/hrtb-conflate-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1350933Z ------------------------------------------
2019-08-12T10:12:55.1350983Z 
2019-08-12T10:12:55.1351202Z ------------------------------------------
2019-08-12T10:12:55.1351245Z stderr:
2019-08-12T10:12:55.1351245Z stderr:
2019-08-12T10:12:55.1351472Z ------------------------------------------
2019-08-12T10:12:55.1351743Z error[E0277]: the trait bound `for<'a, 'b> SomeStruct: Foo<(&'a isize, &'b isize)>` is not satisfied
2019-08-12T10:12:55.1352049Z    |
2019-08-12T10:12:55.1352049Z    |
2019-08-12T10:12:55.1352091Z LL | fn b() { want_foo2::<SomeStruct>(); } //~ ERROR
2019-08-12T10:12:55.1352383Z    |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a, 'b> Foo<(&'a isize, &'b isize)>` is not implemented for `SomeStruct`
2019-08-12T10:12:55.1352487Z    = help: the following implementations were found:
2019-08-12T10:12:55.1352487Z    = help: the following implementations were found:
2019-08-12T10:12:55.1352809Z              <SomeStruct as Foo<(&'a isize, &'a isize)>>
2019-08-12T10:12:55.1352859Z note: required by `want_foo2`
2019-08-12T10:12:55.1353344Z    |
2019-08-12T10:12:55.1353344Z    |
2019-08-12T10:12:55.1353385Z LL | / fn want_foo2<T>()
2019-08-12T10:12:55.1353679Z LL | |     where T : for<'a,'b> Foo<(&'a isize, &'b isize)>
2019-08-12T10:12:55.1353722Z LL | | {
2019-08-12T10:12:55.1354071Z    | |_^
2019-08-12T10:12:55.1354115Z 
2019-08-12T10:12:55.1354155Z error: aborting due to previous error
2019-08-12T10:12:55.1354179Z 
---
2019-08-12T10:12:55.1355630Z diff of stderr:
2019-08-12T10:12:55.1355847Z 
2019-08-12T10:12:55.1355907Z 7    = note: `#[warn(incomplete_features)]` on by default
2019-08-12T10:12:55.1355969Z 8 
2019-08-12T10:12:55.1356028Z 9 error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
2019-08-12T10:12:55.1356978Z +   --> $DIR/bound-normalization-fail.rs:28:32
2019-08-12T10:12:55.1357026Z 11    |
2019-08-12T10:12:55.1357026Z 11    |
2019-08-12T10:12:55.1357281Z 12 LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
2019-08-12T10:12:55.1357355Z 13    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-12T10:12:55.1357662Z 
2019-08-12T10:12:55.1357714Z 17    = note: the return type of a function must have a statically known size
2019-08-12T10:12:55.1357776Z 18 
2019-08-12T10:12:55.1358549Z 19 error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
2019-08-12T10:12:55.1359014Z +   --> $DIR/bound-normalization-fail.rs:44:41
2019-08-12T10:12:55.1359319Z 21    |
2019-08-12T10:12:55.1359319Z 21    |
2019-08-12T10:12:55.1359778Z 22 LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
2019-08-12T10:12:55.1359838Z 23    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-12T10:12:55.1360059Z 
2019-08-12T10:12:55.1360098Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1360450Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
2019-08-12T10:12:55.1360450Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
2019-08-12T10:12:55.1361034Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1361437Z To only update this specific test, also pass `--test-args impl-trait/bound-normalization-fail.rs`
2019-08-12T10:12:55.1361673Z error: 1 errors occurred comparing output.
2019-08-12T10:12:55.1361825Z status: exit code: 1
2019-08-12T10:12:55.1361825Z status: exit code: 1
2019-08-12T10:12:55.1362786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bound-normalization-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1363418Z ------------------------------------------
2019-08-12T10:12:55.1363459Z 
2019-08-12T10:12:55.1363698Z ------------------------------------------
2019-08-12T10:12:55.1363738Z stderr:
---
2019-08-12T10:12:55.1365489Z    |            ^^^^^^^^^^^^^^^^^^^^^^
2019-08-12T10:12:55.1365545Z    |
2019-08-12T10:12:55.1365590Z    = note: `#[warn(incomplete_features)]` on by default
2019-08-12T10:12:55.1365619Z 
2019-08-12T10:12:55.1365669Z error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
2019-08-12T10:12:55.1366108Z    |
2019-08-12T10:12:55.1366108Z    |
2019-08-12T10:12:55.1366585Z LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
2019-08-12T10:12:55.1366673Z    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-12T10:12:55.1366896Z    = note: expected type `()`
2019-08-12T10:12:55.1366896Z    = note: expected type `()`
2019-08-12T10:12:55.1367095Z               found type `<T as impl_trait::Trait>::Assoc`
2019-08-12T10:12:55.1367176Z    = note: the return type of a function must have a statically known size
2019-08-12T10:12:55.1367208Z 
2019-08-12T10:12:55.1367602Z error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
2019-08-12T10:12:55.1368558Z    |
2019-08-12T10:12:55.1368558Z    |
2019-08-12T10:12:55.1369022Z LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
2019-08-12T10:12:55.1369085Z    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-12T10:12:55.1369159Z    = note: expected type `()`
2019-08-12T10:12:55.1369159Z    = note: expected type `()`
2019-08-12T10:12:55.1369404Z               found type `<T as lifetimes::Trait<'static>>::Assoc`
2019-08-12T10:12:55.1369700Z    = note: the return type of a function must have a statically known size
2019-08-12T10:12:55.1369820Z error: aborting due to 2 previous errors
2019-08-12T10:12:55.1369844Z 
2019-08-12T10:12:55.1370261Z For more information about this error, try `rustc --explain E0271`.
2019-08-12T10:12:55.1370427Z 
2019-08-12T10:12:55.1370427Z 
2019-08-12T10:12:55.1370712Z ------------------------------------------
2019-08-12T10:12:55.1370741Z 
2019-08-12T10:12:55.1370763Z 
2019-08-12T10:12:55.1370969Z ---- [ui] ui/issues/issue-12028.rs stdout ----
2019-08-12T10:12:55.1371026Z diff of stderr:
2019-08-12T10:12:55.1371050Z 
2019-08-12T10:12:55.1371094Z 1 error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2019-08-12T10:12:55.1372804Z -   --> $DIR/issue-12028.rs:29:14
2019-08-12T10:12:55.1373123Z +   --> $DIR/issue-12028.rs:27:14
2019-08-12T10:12:55.1373208Z 4 LL |         self.input_stream(&mut stream);
2019-08-12T10:12:55.1373396Z 5    |              ^^^^^^^^^^^^
2019-08-12T10:12:55.1373423Z 
2019-08-12T10:12:55.1373454Z 
2019-08-12T10:12:55.1373454Z 
2019-08-12T10:12:55.1373494Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1374055Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/issue-12028.stderr
2019-08-12T10:12:55.1374319Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1375338Z To only update this specific test, also pass `--test-args issues/issue-12028.rs`
2019-08-12T10:12:55.1375458Z error: 1 errors occurred comparing output.
2019-08-12T10:12:55.1375502Z status: exit code: 1
2019-08-12T10:12:55.1375502Z status: exit code: 1
2019-08-12T10:12:55.1376274Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12028.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1376611Z ------------------------------------------
2019-08-12T10:12:55.1376663Z 
2019-08-12T10:12:55.1376897Z ------------------------------------------
2019-08-12T10:12:55.1376943Z stderr:
2019-08-12T10:12:55.1376943Z stderr:
2019-08-12T10:12:55.1377184Z ------------------------------------------
2019-08-12T10:12:55.1377241Z error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2019-08-12T10:12:55.1377518Z   --> /checkout/src/test/ui/issues/issue-12028.rs:27:14
2019-08-12T10:12:55.1377586Z    |
2019-08-12T10:12:55.1377635Z LL |         self.input_stream(&mut stream); //~ ERROR type annotations required
2019-08-12T10:12:55.1377722Z 
2019-08-12T10:12:55.1377785Z error: aborting due to previous error
2019-08-12T10:12:55.1377822Z 
2019-08-12T10:12:55.1378228Z For more information about this error, try `rustc --explain E0284`.
2019-08-12T10:12:55.1378228Z For more information about this error, try `rustc --explain E0284`.
2019-08-12T10:12:55.1378261Z 
2019-08-12T10:12:55.1378581Z ------------------------------------------
2019-08-12T10:12:55.1378609Z 
2019-08-12T10:12:55.1378630Z 
2019-08-12T10:12:55.1378871Z ---- [ui] ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs#migrate stdout ----
2019-08-12T10:12:55.1378932Z diff of stderr:
2019-08-12T10:12:55.1378955Z 
2019-08-12T10:12:55.1379212Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1379692Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-12T10:12:55.1379734Z 3    |
2019-08-12T10:12:55.1379734Z 3    |
2019-08-12T10:12:55.1379958Z 4 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1380154Z 
2019-08-12T10:12:55.1380196Z 6    |
2019-08-12T10:12:55.1380196Z 6    |
2019-08-12T10:12:55.1380492Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 37:15
2019-08-12T10:12:55.1380851Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:37:15
2019-08-12T10:12:55.1381111Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 33:15
2019-08-12T10:12:55.1381375Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:33:15
2019-08-12T10:12:55.1381419Z 9    |
2019-08-12T10:12:55.1381743Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1381831Z 
2019-08-12T10:12:55.1381831Z 
2019-08-12T10:12:55.1382108Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 37:18
2019-08-12T10:12:55.1382367Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:37:18
2019-08-12T10:12:55.1382787Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 33:18
2019-08-12T10:12:55.1383049Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:33:18
2019-08-12T10:12:55.1383095Z 14    |
2019-08-12T10:12:55.1383321Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1383395Z 
2019-08-12T10:12:55.1383419Z 
2019-08-12T10:12:55.1383475Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1383475Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1383858Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate/regions-assoc-type-in-supertrait-outlives-container.migrate.stderr
2019-08-12T10:12:55.1384122Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1384870Z To only update this specific test, also pass `--test-args regions/regions-assoc-type-in-supertrait-outlives-container.rs`
2019-08-12T10:12:55.1384927Z 
2019-08-12T10:12:55.1384982Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-12T10:12:55.1385046Z status: exit code: 1
2019-08-12T10:12:55.1385950Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1386307Z ------------------------------------------
2019-08-12T10:12:55.1386350Z 
2019-08-12T10:12:55.1386600Z ------------------------------------------
2019-08-12T10:12:55.1386654Z stderr:
2019-08-12T10:12:55.1386654Z stderr:
2019-08-12T10:12:55.1386881Z ------------------------------------------
2019-08-12T10:12:55.1387198Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1387543Z    |
2019-08-12T10:12:55.1387543Z    |
2019-08-12T10:12:55.1387919Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1388107Z    |
2019-08-12T10:12:55.1388107Z    |
2019-08-12T10:12:55.1388362Z note: the pointer is valid for the lifetime 'a as defined on the function body at 33:15
2019-08-12T10:12:55.1388658Z    |
2019-08-12T10:12:55.1388658Z    |
2019-08-12T10:12:55.1388861Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1388911Z    |               ^^
2019-08-12T10:12:55.1389262Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 33:18
2019-08-12T10:12:55.1389613Z    |
2019-08-12T10:12:55.1389613Z    |
2019-08-12T10:12:55.1389803Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1389884Z 
2019-08-12T10:12:55.1389921Z error: aborting due to previous error
2019-08-12T10:12:55.1389945Z 
2019-08-12T10:12:55.1390170Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-12T10:12:55.1390895Z 1 error: lifetime may not live long enough
2019-08-12T10:12:55.1397913Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:43:12
2019-08-12T10:12:55.1398336Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-12T10:12:55.1398385Z 3    |
2019-08-12T10:12:55.1398630Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1398991Z 5    |               -- -- lifetime `'b` defined here
2019-08-12T10:12:55.1399050Z 
2019-08-12T10:12:55.1399107Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1399107Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1399473Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll/regions-assoc-type-in-supertrait-outlives-container.nll.stderr
2019-08-12T10:12:55.1399730Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1400162Z To only update this specific test, also pass `--test-args regions/regions-assoc-type-in-supertrait-outlives-container.rs`
2019-08-12T10:12:55.1400216Z 
2019-08-12T10:12:55.1400260Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-12T10:12:55.1400320Z status: exit code: 1
2019-08-12T10:12:55.1401156Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1401510Z ------------------------------------------
2019-08-12T10:12:55.1401551Z 
2019-08-12T10:12:55.1401794Z ------------------------------------------
2019-08-12T10:12:55.1401838Z stderr:
2019-08-12T10:12:55.1401838Z stderr:
2019-08-12T10:12:55.1402059Z ------------------------------------------
2019-08-12T10:12:55.1402122Z error: lifetime may not live long enough
2019-08-12T10:12:55.1402395Z   --> /checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-12T10:12:55.1402446Z    |
2019-08-12T10:12:55.1402674Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1402906Z    |               -- -- lifetime `'b` defined here
2019-08-12T10:12:55.1403306Z    |               lifetime `'a` defined here
2019-08-12T10:12:55.1403349Z ...
2019-08-12T10:12:55.1403349Z ...
2019-08-12T10:12:55.1403574Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1403853Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-12T10:12:55.1404059Z error: aborting due to previous error
2019-08-12T10:12:55.1404095Z 
2019-08-12T10:12:55.1404117Z 
2019-08-12T10:12:55.1404386Z ------------------------------------------
2019-08-12T10:12:55.1404386Z ------------------------------------------
2019-08-12T10:12:55.1404417Z 
2019-08-12T10:12:55.1404440Z 
2019-08-12T10:12:55.1404694Z ---- [ui] ui/regions/regions-outlives-projection-container-hrtb.rs#migrate stdout ----
2019-08-12T10:12:55.1405152Z diff of stderr:
2019-08-12T10:12:55.1405182Z 
2019-08-12T10:12:55.1405545Z 1 error[E0491]: in type `&'a WithHrAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1406087Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-12T10:12:55.1406134Z 3    |
2019-08-12T10:12:55.1406134Z 3    |
2019-08-12T10:12:55.1406379Z 4 LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1406597Z 
2019-08-12T10:12:55.1406635Z 6    |
2019-08-12T10:12:55.1406635Z 6    |
2019-08-12T10:12:55.1406966Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 32:15
2019-08-12T10:12:55.1407232Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:32:15
2019-08-12T10:12:55.1407512Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-12T10:12:55.1407784Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:27:15
2019-08-12T10:12:55.1407831Z 9    |
2019-08-12T10:12:55.1408051Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1408248Z 
2019-08-12T10:12:55.1408248Z 
2019-08-12T10:12:55.1408529Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 32:18
2019-08-12T10:12:55.1408779Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:32:18
2019-08-12T10:12:55.1409080Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-12T10:12:55.1409462Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:27:18
2019-08-12T10:12:55.1409506Z 14    |
2019-08-12T10:12:55.1409727Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1409798Z 
2019-08-12T10:12:55.1409833Z 17 
2019-08-12T10:12:55.1409833Z 17 
2019-08-12T10:12:55.1410135Z 18 error[E0491]: in type `&'a WithHrAssocSub<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1410616Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-12T10:12:55.1410674Z 20    |
2019-08-12T10:12:55.1410674Z 20    |
2019-08-12T10:12:55.1410903Z 21 LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-12T10:12:55.1410993Z 
2019-08-12T10:12:55.1411028Z 23    |
2019-08-12T10:12:55.1411028Z 23    |
2019-08-12T10:12:55.1411567Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 53:19
2019-08-12T10:12:55.1411847Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:53:19
2019-08-12T10:12:55.1412141Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 46:19
2019-08-12T10:12:55.1412392Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:46:19
2019-08-12T10:12:55.1412436Z 26    |
2019-08-12T10:12:55.1412666Z 27 LL | fn with_assoc_sub<'a,'b>() {
2019-08-12T10:12:55.1412741Z 
2019-08-12T10:12:55.1412741Z 
2019-08-12T10:12:55.1413033Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 53:22
2019-08-12T10:12:55.1413285Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:53:22
2019-08-12T10:12:55.1413689Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 46:22
2019-08-12T10:12:55.1413962Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:46:22
2019-08-12T10:12:55.1414017Z 31    |
2019-08-12T10:12:55.1414243Z 32 LL | fn with_assoc_sub<'a,'b>() {
2019-08-12T10:12:55.1414445Z 
2019-08-12T10:12:55.1414469Z 
2019-08-12T10:12:55.1414515Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1414515Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1415258Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate/regions-outlives-projection-container-hrtb.migrate.stderr
2019-08-12T10:12:55.1415565Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1415871Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-12T10:12:55.1415929Z 
2019-08-12T10:12:55.1415973Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-12T10:12:55.1416017Z status: exit code: 1
2019-08-12T10:12:55.1416877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1417382Z ------------------------------------------
2019-08-12T10:12:55.1417417Z 
2019-08-12T10:12:55.1417649Z ------------------------------------------
2019-08-12T10:12:55.1417694Z stderr:
2019-08-12T10:12:55.1417694Z stderr:
2019-08-12T10:12:55.1417945Z ------------------------------------------
2019-08-12T10:12:55.1418248Z error[E0491]: in type `&'a WithHrAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1418625Z    |
2019-08-12T10:12:55.1418625Z    |
2019-08-12T10:12:55.1418872Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1419030Z    |
2019-08-12T10:12:55.1419030Z    |
2019-08-12T10:12:55.1419304Z note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-12T10:12:55.1419650Z    |
2019-08-12T10:12:55.1419650Z    |
2019-08-12T10:12:55.1419911Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1419960Z    |               ^^
2019-08-12T10:12:55.1420270Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-12T10:12:55.1420612Z    |
2019-08-12T10:12:55.1420612Z    |
2019-08-12T10:12:55.1420843Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1420926Z 
2019-08-12T10:12:55.1420926Z 
2019-08-12T10:12:55.1421227Z error[E0491]: in type `&'a WithHrAssocSub<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1421570Z    |
2019-08-12T10:12:55.1421570Z    |
2019-08-12T10:12:55.1421811Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-12T10:12:55.1421920Z    |
2019-08-12T10:12:55.1421920Z    |
2019-08-12T10:12:55.1422190Z note: the pointer is valid for the lifetime 'a as defined on the function body at 46:19
2019-08-12T10:12:55.1422530Z    |
2019-08-12T10:12:55.1422530Z    |
2019-08-12T10:12:55.1422746Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-12T10:12:55.1422818Z    |                   ^^
2019-08-12T10:12:55.1423194Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 46:22
2019-08-12T10:12:55.1423574Z    |
2019-08-12T10:12:55.1423574Z    |
2019-08-12T10:12:55.1423900Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-12T10:12:55.1423973Z 
2019-08-12T10:12:55.1424027Z error: aborting due to 2 previous errors
2019-08-12T10:12:55.1424055Z 
2019-08-12T10:12:55.1424302Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-12T10:12:55.1425632Z 1 error: lifetime may not live long enough
2019-08-12T10:12:55.1425932Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:35:12
2019-08-12T10:12:55.1426212Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-12T10:12:55.1426260Z 3    |
2019-08-12T10:12:55.1426479Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1426737Z 5    |               -- -- lifetime `'b` defined here
2019-08-12T10:12:55.1427054Z 10    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-12T10:12:55.1427104Z 11 
2019-08-12T10:12:55.1427163Z 12 error: lifetime may not live long enough
2019-08-12T10:12:55.1427416Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-12T10:12:55.1427416Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-12T10:12:55.1427674Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-12T10:12:55.1427736Z 14    |
2019-08-12T10:12:55.1427959Z 15 LL | fn with_assoc_sub<'a,'b>() {
2019-08-12T10:12:55.1428208Z 16    |                   -- -- lifetime `'b` defined here
2019-08-12T10:12:55.1428292Z 
2019-08-12T10:12:55.1428343Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1428343Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1428711Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll/regions-outlives-projection-container-hrtb.nll.stderr
2019-08-12T10:12:55.1428996Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1429302Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-12T10:12:55.1429340Z 
2019-08-12T10:12:55.1429401Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-12T10:12:55.1429445Z status: exit code: 1
2019-08-12T10:12:55.1430287Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1430752Z ------------------------------------------
2019-08-12T10:12:55.1430784Z 
2019-08-12T10:12:55.1431008Z ------------------------------------------
2019-08-12T10:12:55.1431052Z stderr:
2019-08-12T10:12:55.1431052Z stderr:
2019-08-12T10:12:55.1431509Z ------------------------------------------
2019-08-12T10:12:55.1431551Z error: lifetime may not live long enough
2019-08-12T10:12:55.1431789Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-12T10:12:55.1431854Z    |
2019-08-12T10:12:55.1432048Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1432332Z    |               -- -- lifetime `'b` defined here
2019-08-12T10:12:55.1432627Z    |               lifetime `'a` defined here
2019-08-12T10:12:55.1432667Z ...
2019-08-12T10:12:55.1432667Z ...
2019-08-12T10:12:55.1432878Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1433145Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-12T10:12:55.1433213Z error: lifetime may not live long enough
2019-08-12T10:12:55.1433469Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-12T10:12:55.1433511Z    |
2019-08-12T10:12:55.1433511Z    |
2019-08-12T10:12:55.1433700Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-12T10:12:55.1433928Z    |                   -- -- lifetime `'b` defined here
2019-08-12T10:12:55.1434172Z    |                   lifetime `'a` defined here
2019-08-12T10:12:55.1434283Z ...
2019-08-12T10:12:55.1434283Z ...
2019-08-12T10:12:55.1434544Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-12T10:12:55.1435164Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-12T10:12:55.1435272Z error: aborting due to 2 previous errors
2019-08-12T10:12:55.1435300Z 
2019-08-12T10:12:55.1435325Z 
2019-08-12T10:12:55.1435595Z ------------------------------------------
2019-08-12T10:12:55.1435595Z ------------------------------------------
2019-08-12T10:12:55.1435628Z 
2019-08-12T10:12:55.1435668Z 
2019-08-12T10:12:55.1435937Z ---- [ui] ui/regions/regions-outlives-projection-container-wc.rs#migrate stdout ----
2019-08-12T10:12:55.1435987Z diff of stderr:
2019-08-12T10:12:55.1436014Z 
2019-08-12T10:12:55.1436326Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1436882Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-12T10:12:55.1436937Z 3    |
2019-08-12T10:12:55.1436937Z 3    |
2019-08-12T10:12:55.1437182Z 4 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1437278Z 
2019-08-12T10:12:55.1437316Z 6    |
2019-08-12T10:12:55.1437316Z 6    |
2019-08-12T10:12:55.1437590Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 31:15
2019-08-12T10:12:55.1437861Z -   --> $DIR/regions-outlives-projection-container-wc.rs:31:15
2019-08-12T10:12:55.1438140Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-12T10:12:55.1438396Z +   --> $DIR/regions-outlives-projection-container-wc.rs:27:15
2019-08-12T10:12:55.1438458Z 9    |
2019-08-12T10:12:55.1438678Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1438753Z 
2019-08-12T10:12:55.1438753Z 
2019-08-12T10:12:55.1439054Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 31:18
2019-08-12T10:12:55.1439333Z -   --> $DIR/regions-outlives-projection-container-wc.rs:31:18
2019-08-12T10:12:55.1439630Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-12T10:12:55.1439905Z +   --> $DIR/regions-outlives-projection-container-wc.rs:27:18
2019-08-12T10:12:55.1439951Z 14    |
2019-08-12T10:12:55.1440168Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1440258Z 
2019-08-12T10:12:55.1440283Z 
2019-08-12T10:12:55.1440326Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1440326Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1440710Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate/regions-outlives-projection-container-wc.migrate.stderr
2019-08-12T10:12:55.1440981Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1441280Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-12T10:12:55.1441446Z 
2019-08-12T10:12:55.1441500Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-12T10:12:55.1441652Z status: exit code: 1
2019-08-12T10:12:55.1442495Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1443060Z ------------------------------------------
2019-08-12T10:12:55.1443194Z 
2019-08-12T10:12:55.1443403Z ------------------------------------------
2019-08-12T10:12:55.1443458Z stderr:
2019-08-12T10:12:55.1443458Z stderr:
2019-08-12T10:12:55.1443658Z ------------------------------------------
2019-08-12T10:12:55.1443921Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1444228Z    |
2019-08-12T10:12:55.1444228Z    |
2019-08-12T10:12:55.1444438Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1444534Z    |
2019-08-12T10:12:55.1444534Z    |
2019-08-12T10:12:55.1445245Z note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-12T10:12:55.1445625Z    |
2019-08-12T10:12:55.1445625Z    |
2019-08-12T10:12:55.1445858Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1445906Z    |               ^^
2019-08-12T10:12:55.1446220Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-12T10:12:55.1446551Z    |
2019-08-12T10:12:55.1446551Z    |
2019-08-12T10:12:55.1446783Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1446857Z 
2019-08-12T10:12:55.1446898Z error: aborting due to previous error
2019-08-12T10:12:55.1446943Z 
2019-08-12T10:12:55.1447202Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-12T10:12:55.1447955Z 1 error: lifetime may not live long enough
2019-08-12T10:12:55.1448432Z -   --> $DIR/regions-outlives-projection-container-wc.rs:37:12
2019-08-12T10:12:55.1448672Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-12T10:12:55.1448713Z 3    |
2019-08-12T10:12:55.1448904Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1449115Z 5    |               -- -- lifetime `'b` defined here
2019-08-12T10:12:55.1449181Z 
2019-08-12T10:12:55.1449220Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1449220Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1449663Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll/regions-outlives-projection-container-wc.nll.stderr
2019-08-12T10:12:55.1449929Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1450322Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-12T10:12:55.1450385Z 
2019-08-12T10:12:55.1450525Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-12T10:12:55.1450575Z status: exit code: 1
2019-08-12T10:12:55.1451828Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1452190Z ------------------------------------------
2019-08-12T10:12:55.1452340Z 
2019-08-12T10:12:55.1452616Z ------------------------------------------
2019-08-12T10:12:55.1452662Z stderr:
2019-08-12T10:12:55.1452662Z stderr:
2019-08-12T10:12:55.1452912Z ------------------------------------------
2019-08-12T10:12:55.1452961Z error: lifetime may not live long enough
2019-08-12T10:12:55.1453235Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs:33:12
2019-08-12T10:12:55.1453300Z    |
2019-08-12T10:12:55.1453522Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1453761Z    |               -- -- lifetime `'b` defined here
2019-08-12T10:12:55.1454161Z    |               lifetime `'a` defined here
2019-08-12T10:12:55.1454204Z ...
2019-08-12T10:12:55.1454204Z ...
2019-08-12T10:12:55.1455020Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1455392Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-12T10:12:55.1455471Z error: aborting due to previous error
2019-08-12T10:12:55.1455528Z 
2019-08-12T10:12:55.1455552Z 
2019-08-12T10:12:55.1455794Z ------------------------------------------
2019-08-12T10:12:55.1455794Z ------------------------------------------
2019-08-12T10:12:55.1455826Z 
2019-08-12T10:12:55.1455851Z 
2019-08-12T10:12:55.1456124Z ---- [ui] ui/regions/regions-outlives-projection-container.rs stdout ----
2019-08-12T10:12:55.1456173Z diff of stderr:
2019-08-12T10:12:55.1456200Z 
2019-08-12T10:12:55.1456513Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1457035Z +   --> $DIR/regions-outlives-projection-container.rs:36:13
2019-08-12T10:12:55.1457096Z 3    |
2019-08-12T10:12:55.1457096Z 3    |
2019-08-12T10:12:55.1457337Z 4 LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1457415Z 
2019-08-12T10:12:55.1457467Z 6    |
2019-08-12T10:12:55.1457467Z 6    |
2019-08-12T10:12:55.1457741Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 32:15
2019-08-12T10:12:55.1458015Z -   --> $DIR/regions-outlives-projection-container.rs:32:15
2019-08-12T10:12:55.1458409Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 28:15
2019-08-12T10:12:55.1458653Z +   --> $DIR/regions-outlives-projection-container.rs:28:15
2019-08-12T10:12:55.1458697Z 9    |
2019-08-12T10:12:55.1461507Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1461597Z 
2019-08-12T10:12:55.1461597Z 
2019-08-12T10:12:55.1461880Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 32:18
2019-08-12T10:12:55.1462147Z -   --> $DIR/regions-outlives-projection-container.rs:32:18
2019-08-12T10:12:55.1462421Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 28:18
2019-08-12T10:12:55.1462759Z +   --> $DIR/regions-outlives-projection-container.rs:28:18
2019-08-12T10:12:55.1462833Z 14    |
2019-08-12T10:12:55.1463029Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1463244Z 
2019-08-12T10:12:55.1463279Z 17 
2019-08-12T10:12:55.1463279Z 17 
2019-08-12T10:12:55.1463582Z 18 error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1464050Z +   --> $DIR/regions-outlives-projection-container.rs:54:13
2019-08-12T10:12:55.1464091Z 20    |
2019-08-12T10:12:55.1464091Z 20    |
2019-08-12T10:12:55.1464304Z 21 LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1464391Z 
2019-08-12T10:12:55.1464424Z 23    |
2019-08-12T10:12:55.1464424Z 23    |
2019-08-12T10:12:55.1465292Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 54:18
2019-08-12T10:12:55.1465596Z -   --> $DIR/regions-outlives-projection-container.rs:54:18
2019-08-12T10:12:55.1466024Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 50:18
2019-08-12T10:12:55.1466310Z +   --> $DIR/regions-outlives-projection-container.rs:50:18
2019-08-12T10:12:55.1466358Z 26    |
2019-08-12T10:12:55.1466644Z 27 LL | fn without_assoc<'a,'b>() {
2019-08-12T10:12:55.1466737Z 
2019-08-12T10:12:55.1466737Z 
2019-08-12T10:12:55.1467029Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 54:21
2019-08-12T10:12:55.1467284Z -   --> $DIR/regions-outlives-projection-container.rs:54:21
2019-08-12T10:12:55.1467594Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 50:21
2019-08-12T10:12:55.1467851Z +   --> $DIR/regions-outlives-projection-container.rs:50:21
2019-08-12T10:12:55.1467898Z 31    |
2019-08-12T10:12:55.1468136Z 32 LL | fn without_assoc<'a,'b>() {
2019-08-12T10:12:55.1468221Z 
2019-08-12T10:12:55.1468259Z 34 
2019-08-12T10:12:55.1468259Z 34 
2019-08-12T10:12:55.1468585Z 35 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1469101Z +   --> $DIR/regions-outlives-projection-container.rs:63:12
2019-08-12T10:12:55.1469164Z 37    |
2019-08-12T10:12:55.1469164Z 37    |
2019-08-12T10:12:55.1469398Z 38 LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-12T10:12:55.1469478Z 
2019-08-12T10:12:55.1469534Z 40    |
2019-08-12T10:12:55.1469534Z 40    |
2019-08-12T10:12:55.1469808Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 62:20
2019-08-12T10:12:55.1470062Z -   --> $DIR/regions-outlives-projection-container.rs:62:20
2019-08-12T10:12:55.1470358Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 58:20
2019-08-12T10:12:55.1471587Z +   --> $DIR/regions-outlives-projection-container.rs:58:20
2019-08-12T10:12:55.1471657Z 43    |
2019-08-12T10:12:55.1471899Z 44 LL | fn call_with_assoc<'a,'b>() {
2019-08-12T10:12:55.1471980Z 
2019-08-12T10:12:55.1471980Z 
2019-08-12T10:12:55.1472253Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 62:23
2019-08-12T10:12:55.1472510Z -   --> $DIR/regions-outlives-projection-container.rs:62:23
2019-08-12T10:12:55.1472784Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 58:23
2019-08-12T10:12:55.1473022Z +   --> $DIR/regions-outlives-projection-container.rs:58:23
2019-08-12T10:12:55.1473083Z 48    |
2019-08-12T10:12:55.1473291Z 49 LL | fn call_with_assoc<'a,'b>() {
2019-08-12T10:12:55.1473380Z 
2019-08-12T10:12:55.1473415Z 51 
2019-08-12T10:12:55.1473415Z 51 
2019-08-12T10:12:55.1473696Z 52 error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1474461Z +   --> $DIR/regions-outlives-projection-container.rs:70:12
2019-08-12T10:12:55.1474517Z 54    |
2019-08-12T10:12:55.1474517Z 54    |
2019-08-12T10:12:55.1475319Z 55 LL |     call::<&'a WithoutAssoc<TheType<'b>>>();
2019-08-12T10:12:55.1475434Z 
2019-08-12T10:12:55.1475472Z 57    |
2019-08-12T10:12:55.1475472Z 57    |
2019-08-12T10:12:55.1475785Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 71:23
2019-08-12T10:12:55.1476042Z -   --> $DIR/regions-outlives-projection-container.rs:71:23
2019-08-12T10:12:55.1476321Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 67:23
2019-08-12T10:12:55.1476592Z +   --> $DIR/regions-outlives-projection-container.rs:67:23
2019-08-12T10:12:55.1476638Z 60    |
2019-08-12T10:12:55.1476867Z 61 LL | fn call_without_assoc<'a,'b>() {
2019-08-12T10:12:55.1477075Z 
2019-08-12T10:12:55.1477075Z 
2019-08-12T10:12:55.1477404Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 71:26
2019-08-12T10:12:55.1477664Z -   --> $DIR/regions-outlives-projection-container.rs:71:26
2019-08-12T10:12:55.1477979Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 67:26
2019-08-12T10:12:55.1478343Z +   --> $DIR/regions-outlives-projection-container.rs:67:26
2019-08-12T10:12:55.1478387Z 65    |
2019-08-12T10:12:55.1478611Z 66 LL | fn call_without_assoc<'a,'b>() {
2019-08-12T10:12:55.1478681Z 
2019-08-12T10:12:55.1478705Z 
2019-08-12T10:12:55.1478760Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1479091Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
2019-08-12T10:12:55.1479091Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
2019-08-12T10:12:55.1479355Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1479754Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container.rs`
2019-08-12T10:12:55.1479824Z error: 1 errors occurred comparing output.
2019-08-12T10:12:55.1479877Z status: exit code: 1
2019-08-12T10:12:55.1479877Z status: exit code: 1
2019-08-12T10:12:55.1480568Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1480878Z ------------------------------------------
2019-08-12T10:12:55.1480916Z 
2019-08-12T10:12:55.1481133Z ------------------------------------------
2019-08-12T10:12:55.1481173Z stderr:
2019-08-12T10:12:55.1481173Z stderr:
2019-08-12T10:12:55.1481372Z ------------------------------------------
2019-08-12T10:12:55.1481631Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1481940Z    |
2019-08-12T10:12:55.1481940Z    |
2019-08-12T10:12:55.1482164Z LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1482244Z    |
2019-08-12T10:12:55.1482244Z    |
2019-08-12T10:12:55.1482480Z note: the pointer is valid for the lifetime 'a as defined on the function body at 28:15
2019-08-12T10:12:55.1482787Z    |
2019-08-12T10:12:55.1482787Z    |
2019-08-12T10:12:55.1482979Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1483113Z    |               ^^
2019-08-12T10:12:55.1483397Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 28:18
2019-08-12T10:12:55.1483855Z    |
2019-08-12T10:12:55.1483855Z    |
2019-08-12T10:12:55.1484084Z LL | fn with_assoc<'a,'b>() {
2019-08-12T10:12:55.1484148Z 
2019-08-12T10:12:55.1484148Z 
2019-08-12T10:12:55.1484430Z error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1485199Z    |
2019-08-12T10:12:55.1485199Z    |
2019-08-12T10:12:55.1485514Z LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-12T10:12:55.1485743Z    |
2019-08-12T10:12:55.1485743Z    |
2019-08-12T10:12:55.1486105Z note: the pointer is valid for the lifetime 'a as defined on the function body at 50:18
2019-08-12T10:12:55.1486462Z    |
2019-08-12T10:12:55.1486462Z    |
2019-08-12T10:12:55.1486716Z LL | fn without_assoc<'a,'b>() {
2019-08-12T10:12:55.1486767Z    |                  ^^
2019-08-12T10:12:55.1487075Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 50:21
2019-08-12T10:12:55.1487441Z    |
2019-08-12T10:12:55.1487441Z    |
2019-08-12T10:12:55.1487677Z LL | fn without_assoc<'a,'b>() {
2019-08-12T10:12:55.1487774Z 
2019-08-12T10:12:55.1487774Z 
2019-08-12T10:12:55.1488207Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1488633Z    |
2019-08-12T10:12:55.1488633Z    |
2019-08-12T10:12:55.1488845Z LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-12T10:12:55.1488940Z    |
2019-08-12T10:12:55.1488940Z    |
2019-08-12T10:12:55.1489180Z note: the pointer is valid for the lifetime 'a as defined on the function body at 58:20
2019-08-12T10:12:55.1489480Z    |
2019-08-12T10:12:55.1489480Z    |
2019-08-12T10:12:55.1489672Z LL | fn call_with_assoc<'a,'b>() {
2019-08-12T10:12:55.1489714Z    |                    ^^
2019-08-12T10:12:55.1489965Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 58:23
2019-08-12T10:12:55.1490265Z    |
2019-08-12T10:12:55.1490265Z    |
2019-08-12T10:12:55.1490455Z LL | fn call_with_assoc<'a,'b>() {
2019-08-12T10:12:55.1490545Z 
2019-08-12T10:12:55.1490545Z 
2019-08-12T10:12:55.1490812Z error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-12T10:12:55.1491113Z    |
2019-08-12T10:12:55.1491113Z    |
2019-08-12T10:12:55.1491523Z LL |     call::<&'a WithoutAssoc<TheType<'b>>>(); //~ ERROR reference has a longer lifetime
2019-08-12T10:12:55.1491627Z    |
2019-08-12T10:12:55.1491627Z    |
2019-08-12T10:12:55.1491867Z note: the pointer is valid for the lifetime 'a as defined on the function body at 67:23
2019-08-12T10:12:55.1492170Z    |
2019-08-12T10:12:55.1492170Z    |
2019-08-12T10:12:55.1492363Z LL | fn call_without_assoc<'a,'b>() {
2019-08-12T10:12:55.1492405Z    |                       ^^
2019-08-12T10:12:55.1492673Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 67:26
2019-08-12T10:12:55.1493067Z    |
2019-08-12T10:12:55.1493067Z    |
2019-08-12T10:12:55.1493308Z LL | fn call_without_assoc<'a,'b>() {
2019-08-12T10:12:55.1493377Z 
2019-08-12T10:12:55.1493413Z error: aborting due to 4 previous errors
2019-08-12T10:12:55.1493453Z 
2019-08-12T10:12:55.1493680Z For more information about this error, try `rustc --explain E0491`.
2019-08-12T10:12:55.1493680Z For more information about this error, try `rustc --explain E0491`.
2019-08-12T10:12:55.1493710Z 
2019-08-12T10:12:55.1493910Z ------------------------------------------
2019-08-12T10:12:55.1493956Z 
2019-08-12T10:12:55.1493978Z 
2019-08-12T10:12:55.1494207Z ---- [ui] ui/specialization/defaultimpl/specialization-no-default.rs stdout ----
2019-08-12T10:12:55.1494251Z diff of stderr:
2019-08-12T10:12:55.1494290Z 
2019-08-12T10:12:55.1494333Z 1 error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1495511Z +   --> $DIR/specialization-no-default.rs:20:5
2019-08-12T10:12:55.1495561Z 3    |
2019-08-12T10:12:55.1495561Z 3    |
2019-08-12T10:12:55.1495602Z 4 LL | / impl<T> Foo for T {
2019-08-12T10:12:55.1495644Z 5 LL | |     fn foo(&self) {}
2019-08-12T10:12:55.1495690Z 
2019-08-12T10:12:55.1495736Z 13    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1495780Z 14 
2019-08-12T10:12:55.1495846Z 15 error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1496332Z +   --> $DIR/specialization-no-default.rs:23:5
2019-08-12T10:12:55.1496394Z 17    |
2019-08-12T10:12:55.1496394Z 17    |
2019-08-12T10:12:55.1496435Z 18 LL | / impl<T> Foo for T {
2019-08-12T10:12:55.1496477Z 19 LL | |     fn foo(&self) {}
2019-08-12T10:12:55.1496504Z 
2019-08-12T10:12:55.1496578Z 27    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1496628Z 28 
2019-08-12T10:12:55.1496678Z 29 error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1497183Z +   --> $DIR/specialization-no-default.rs:37:5
2019-08-12T10:12:55.1497228Z 31    |
2019-08-12T10:12:55.1497228Z 31    |
2019-08-12T10:12:55.1497269Z 32 LL | / impl<T> Bar for T {
2019-08-12T10:12:55.1497331Z 33 LL | |     type T = u8;
2019-08-12T10:12:55.1497357Z 
2019-08-12T10:12:55.1497403Z 40    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1497463Z 41 
2019-08-12T10:12:55.1497512Z 42 error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1498010Z +   --> $DIR/specialization-no-default.rs:55:5
2019-08-12T10:12:55.1498065Z 44    |
2019-08-12T10:12:55.1498065Z 44    |
2019-08-12T10:12:55.1498106Z 45 LL | / impl<T: Clone> Baz for T {
2019-08-12T10:12:55.1498277Z 46 LL | |     fn baz(&self) {}
2019-08-12T10:12:55.1498318Z 
2019-08-12T10:12:55.1498358Z 53    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1498395Z 54 
2019-08-12T10:12:55.1498454Z 55 error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1498881Z +   --> $DIR/specialization-no-default.rs:74:5
2019-08-12T10:12:55.1498936Z 57    |
2019-08-12T10:12:55.1498936Z 57    |
2019-08-12T10:12:55.1498973Z 58 LL | / impl<T: Clone> Redundant for T {
2019-08-12T10:12:55.1499010Z 59 LL | |     fn redundant(&self) {}
2019-08-12T10:12:55.1499056Z 
2019-08-12T10:12:55.1499110Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1499417Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/specialization-no-default.stderr
2019-08-12T10:12:55.1499417Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/specialization-no-default.stderr
2019-08-12T10:12:55.1499787Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1500101Z To only update this specific test, also pass `--test-args specialization/defaultimpl/specialization-no-default.rs`
2019-08-12T10:12:55.1500173Z error: 1 errors occurred comparing output.
2019-08-12T10:12:55.1500227Z status: exit code: 1
2019-08-12T10:12:55.1500227Z status: exit code: 1
2019-08-12T10:12:55.1500939Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/defaultimpl/specialization-no-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1501351Z ------------------------------------------
2019-08-12T10:12:55.1501382Z 
2019-08-12T10:12:55.1501604Z ------------------------------------------
2019-08-12T10:12:55.1501643Z stderr:
2019-08-12T10:12:55.1501643Z stderr:
2019-08-12T10:12:55.1501841Z ------------------------------------------
2019-08-12T10:12:55.1501906Z error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1502198Z    |
2019-08-12T10:12:55.1502198Z    |
2019-08-12T10:12:55.1502249Z LL | / impl<T> Foo for T {
2019-08-12T10:12:55.1502286Z LL | |     fn foo(&self) {}
2019-08-12T10:12:55.1502323Z LL | |     fn bar(&self) {}
2019-08-12T10:12:55.1502373Z LL | | }
2019-08-12T10:12:55.1502566Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1502612Z ...
2019-08-12T10:12:55.1502649Z LL |       fn foo(&self) {} //~ ERROR E0520
2019-08-12T10:12:55.1502713Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `foo`
2019-08-12T10:12:55.1502752Z    |
2019-08-12T10:12:55.1502792Z    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1502836Z 
2019-08-12T10:12:55.1502878Z error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1503187Z    |
2019-08-12T10:12:55.1503187Z    |
2019-08-12T10:12:55.1503223Z LL | / impl<T> Foo for T {
2019-08-12T10:12:55.1503259Z LL | |     fn foo(&self) {}
2019-08-12T10:12:55.1503295Z LL | |     fn bar(&self) {}
2019-08-12T10:12:55.1503346Z LL | | }
2019-08-12T10:12:55.1503537Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1503575Z ...
2019-08-12T10:12:55.1503629Z LL |       fn bar(&self) {} //~ ERROR E0520
2019-08-12T10:12:55.1503679Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `bar`
2019-08-12T10:12:55.1503722Z    |
2019-08-12T10:12:55.1503779Z    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1503806Z 
2019-08-12T10:12:55.1503848Z error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1504154Z    |
2019-08-12T10:12:55.1504154Z    |
2019-08-12T10:12:55.1504188Z LL | / impl<T> Bar for T {
2019-08-12T10:12:55.1504224Z LL | |     type T = u8;
2019-08-12T10:12:55.1504274Z LL | | }
2019-08-12T10:12:55.1504465Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1504503Z ...
2019-08-12T10:12:55.1504555Z LL |       type T = (); //~ ERROR E0520
2019-08-12T10:12:55.1504596Z    |       ^^^^^^^^^^^^ cannot specialize default item `T`
2019-08-12T10:12:55.1504632Z    |
2019-08-12T10:12:55.1505066Z    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1505137Z 
2019-08-12T10:12:55.1505294Z error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1505723Z    |
2019-08-12T10:12:55.1505723Z    |
2019-08-12T10:12:55.1505765Z LL | / impl<T: Clone> Baz for T {
2019-08-12T10:12:55.1505807Z LL | |     fn baz(&self) {}
2019-08-12T10:12:55.1505864Z LL | | }
2019-08-12T10:12:55.1506084Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1506127Z ...
2019-08-12T10:12:55.1506170Z LL |       fn baz(&self) {} //~ ERROR E0520
2019-08-12T10:12:55.1506234Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `baz`
2019-08-12T10:12:55.1506275Z    |
2019-08-12T10:12:55.1506322Z    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1506370Z 
2019-08-12T10:12:55.1506419Z error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1506891Z    |
2019-08-12T10:12:55.1506891Z    |
2019-08-12T10:12:55.1506933Z LL | / impl<T: Clone> Redundant for T {
2019-08-12T10:12:55.1506977Z LL | |     fn redundant(&self) {}
2019-08-12T10:12:55.1507017Z LL | | }
2019-08-12T10:12:55.1507257Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1507301Z ...
2019-08-12T10:12:55.1507345Z LL |       fn redundant(&self) {} //~ ERROR E0520
2019-08-12T10:12:55.1507411Z    |       ^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `redundant`
2019-08-12T10:12:55.1507453Z    |
2019-08-12T10:12:55.1507500Z    = note: to specialize, `redundant` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1507594Z error: aborting due to 5 previous errors
2019-08-12T10:12:55.1507621Z 
2019-08-12T10:12:55.1507881Z For more information about this error, try `rustc --explain E0520`.
2019-08-12T10:12:55.1507924Z 
2019-08-12T10:12:55.1507924Z 
2019-08-12T10:12:55.1508275Z ------------------------------------------
2019-08-12T10:12:55.1508314Z 
2019-08-12T10:12:55.1508460Z 
2019-08-12T10:12:55.1508683Z ---- [ui] ui/specialization/specialization-no-default.rs stdout ----
2019-08-12T10:12:55.1508742Z diff of stderr:
2019-08-12T10:12:55.1508765Z 
2019-08-12T10:12:55.1508807Z 1 error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1509250Z +   --> $DIR/specialization-no-default.rs:20:5
2019-08-12T10:12:55.1509290Z 3    |
2019-08-12T10:12:55.1509290Z 3    |
2019-08-12T10:12:55.1509325Z 4 LL | / impl<T> Foo for T {
2019-08-12T10:12:55.1509379Z 5 LL | |     fn foo(&self) {}
2019-08-12T10:12:55.1509403Z 
2019-08-12T10:12:55.1509443Z 13    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1509497Z 14 
2019-08-12T10:12:55.1509540Z 15 error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1509994Z +   --> $DIR/specialization-no-default.rs:23:5
2019-08-12T10:12:55.1510034Z 17    |
2019-08-12T10:12:55.1510034Z 17    |
2019-08-12T10:12:55.1510070Z 18 LL | / impl<T> Foo for T {
2019-08-12T10:12:55.1510106Z 19 LL | |     fn foo(&self) {}
2019-08-12T10:12:55.1510145Z 
2019-08-12T10:12:55.1510184Z 27    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1510221Z 28 
2019-08-12T10:12:55.1510279Z 29 error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1510699Z +   --> $DIR/specialization-no-default.rs:37:5
2019-08-12T10:12:55.1510739Z 31    |
2019-08-12T10:12:55.1510739Z 31    |
2019-08-12T10:12:55.1510789Z 32 LL | / impl<T> Bar for T {
2019-08-12T10:12:55.1510826Z 33 LL | |     type T = u8;
2019-08-12T10:12:55.1510849Z 
2019-08-12T10:12:55.1510914Z 40    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1511022Z 41 
2019-08-12T10:12:55.1511072Z 42 error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1511536Z +   --> $DIR/specialization-no-default.rs:55:5
2019-08-12T10:12:55.1511576Z 44    |
2019-08-12T10:12:55.1511576Z 44    |
2019-08-12T10:12:55.1511612Z 45 LL | / impl<T: Clone> Baz for T {
2019-08-12T10:12:55.1511665Z 46 LL | |     fn baz(&self) {}
2019-08-12T10:12:55.1511689Z 
2019-08-12T10:12:55.1511730Z 53    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1511782Z 54 
2019-08-12T10:12:55.1511825Z 55 error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1512313Z +   --> $DIR/specialization-no-default.rs:74:5
2019-08-12T10:12:55.1512452Z 57    |
2019-08-12T10:12:55.1512452Z 57    |
2019-08-12T10:12:55.1512495Z 58 LL | / impl<T: Clone> Redundant for T {
2019-08-12T10:12:55.1512533Z 59 LL | |     fn redundant(&self) {}
2019-08-12T10:12:55.1512596Z 
2019-08-12T10:12:55.1512633Z The actual stderr differed from the expected stderr.
2019-08-12T10:12:55.1512956Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/specialization-no-default.stderr
2019-08-12T10:12:55.1512956Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/specialization-no-default.stderr
2019-08-12T10:12:55.1513217Z To update references, rerun the tests and pass the `--bless` flag
2019-08-12T10:12:55.1513478Z To only update this specific test, also pass `--test-args specialization/specialization-no-default.rs`
2019-08-12T10:12:55.1513562Z error: 1 errors occurred comparing output.
2019-08-12T10:12:55.1513600Z status: exit code: 1
2019-08-12T10:12:55.1513600Z status: exit code: 1
2019-08-12T10:12:55.1514438Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/specialization-no-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/auxiliary" "-A" "unused"
2019-08-12T10:12:55.1515177Z ------------------------------------------
2019-08-12T10:12:55.1515247Z 
2019-08-12T10:12:55.1515535Z ------------------------------------------
2019-08-12T10:12:55.1515581Z stderr:
2019-08-12T10:12:55.1515581Z stderr:
2019-08-12T10:12:55.1515826Z ------------------------------------------
2019-08-12T10:12:55.1515881Z error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1516233Z    |
2019-08-12T10:12:55.1516233Z    |
2019-08-12T10:12:55.1516281Z LL | / impl<T> Foo for T {
2019-08-12T10:12:55.1516324Z LL | |     fn foo(&self) {}
2019-08-12T10:12:55.1516382Z LL | |     fn bar(&self) {}
2019-08-12T10:12:55.1516422Z LL | | }
2019-08-12T10:12:55.1516643Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1516686Z ...
2019-08-12T10:12:55.1516745Z LL |       fn foo(&self) {} //~ ERROR E0520
2019-08-12T10:12:55.1516793Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `foo`
2019-08-12T10:12:55.1516836Z    |
2019-08-12T10:12:55.1516900Z    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1516931Z 
2019-08-12T10:12:55.1516978Z error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1517312Z    |
2019-08-12T10:12:55.1517312Z    |
2019-08-12T10:12:55.1517352Z LL | / impl<T> Foo for T {
2019-08-12T10:12:55.1517401Z LL | |     fn foo(&self) {}
2019-08-12T10:12:55.1517570Z LL | |     fn bar(&self) {}
2019-08-12T10:12:55.1517619Z LL | | }
2019-08-12T10:12:55.1517869Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1517930Z ...
2019-08-12T10:12:55.1517974Z LL |       fn bar(&self) {} //~ ERROR E0520
2019-08-12T10:12:55.1518021Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `bar`
2019-08-12T10:12:55.1518081Z    |
2019-08-12T10:12:55.1518127Z    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1518157Z 
2019-08-12T10:12:55.1518205Z error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1518543Z    |
2019-08-12T10:12:55.1518543Z    |
2019-08-12T10:12:55.1518583Z LL | / impl<T> Bar for T {
2019-08-12T10:12:55.1518641Z LL | |     type T = u8;
2019-08-12T10:12:55.1518680Z LL | | }
2019-08-12T10:12:55.1519005Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1519050Z ...
2019-08-12T10:12:55.1519122Z LL |       type T = (); //~ ERROR E0520
2019-08-12T10:12:55.1519170Z    |       ^^^^^^^^^^^^ cannot specialize default item `T`
2019-08-12T10:12:55.1519211Z    |
2019-08-12T10:12:55.1519272Z    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1519303Z 
2019-08-12T10:12:55.1519350Z error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1519700Z    |
2019-08-12T10:12:55.1519700Z    |
2019-08-12T10:12:55.1519741Z LL | / impl<T: Clone> Baz for T {
2019-08-12T10:12:55.1519784Z LL | |     fn baz(&self) {}
2019-08-12T10:12:55.1519840Z LL | | }
2019-08-12T10:12:55.1520058Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1520101Z ...
2019-08-12T10:12:55.1520211Z LL |       fn baz(&self) {} //~ ERROR E0520
2019-08-12T10:12:55.1520267Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `baz`
2019-08-12T10:12:55.1520309Z    |
2019-08-12T10:12:55.1520378Z    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1520408Z 
2019-08-12T10:12:55.1520458Z error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-12T10:12:55.1520795Z    |
2019-08-12T10:12:55.1520795Z    |
2019-08-12T10:12:55.1520837Z LL | / impl<T: Clone> Redundant for T {
2019-08-12T10:12:55.1520880Z LL | |     fn redundant(&self) {}
2019-08-12T10:12:55.1520988Z LL | | }
2019-08-12T10:12:55.1521205Z    | |_- parent `impl` is here
2019-08-12T10:12:55.1521248Z ...
2019-08-12T10:12:55.1521308Z LL |       default fn redundant(&self) {} //~ ERROR E0520
2019-08-12T10:12:55.1521360Z    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `redundant`
2019-08-12T10:12:55.1521403Z    |
2019-08-12T10:12:55.1521473Z    = note: to specialize, `redundant` in the parent `impl` must be marked `default`
2019-08-12T10:12:55.1521553Z error: aborting due to 5 previous errors
2019-08-12T10:12:55.1521581Z 
2019-08-12T10:12:55.1521840Z For more information about this error, try `rustc --explain E0520`.
2019-08-12T10:12:55.1521891Z 
---
2019-08-12T10:12:55.1532284Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-12T10:12:55.1532374Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-12T10:12:55.1532406Z 
2019-08-12T10:12:55.1532430Z 
2019-08-12T10:12:55.1556999Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-12T10:12:55.1557310Z 
2019-08-12T10:12:55.1557355Z 
2019-08-12T10:12:55.1557403Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-12T10:12:55.1557455Z Build completed unsuccessfully in 1:05:34
2019-08-12T10:12:55.1557455Z Build completed unsuccessfully in 1:05:34
2019-08-12T10:12:55.9327406Z ##[error]Bash exited with code '1'.
2019-08-12T10:12:55.9381613Z ##[section]Starting: Checkout
2019-08-12T10:12:55.9383652Z ==============================================================================
2019-08-12T10:12:55.9383734Z Task         : Get sources
2019-08-12T10:12:55.9383789Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
