plain
2019-08-07T08:47:21.5520462Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-07T08:47:21.5732787Z ##[command]git config gc.auto 0
2019-08-07T08:47:21.5827757Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-07T08:47:21.5899309Z ##[command]git config --get-all http.proxy
2019-08-07T08:47:21.6054410Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63347/merge:refs/remotes/pull/63347/merge
---
2019-08-07T08:47:56.1223920Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-07T08:47:56.1223966Z 
2019-08-07T08:47:56.1224151Z   git checkout -b <new-branch-name>
2019-08-07T08:47:56.1224177Z 
2019-08-07T08:47:56.1224223Z HEAD is now at 817ca27b2 Merge 6c7fdef8aa025309d4d17535fc75fd79a48ac84d into 5421d94960018235654c7fb39aa1c502a3564621
2019-08-07T08:47:56.1381532Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-07T08:47:56.1384375Z ==============================================================================
2019-08-07T08:47:56.1384427Z Task         : Bash
2019-08-07T08:47:56.1384466Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-07T09:49:05.2586697Z .................................................................................................... 1400/8842
2019-08-07T09:49:11.1884097Z .................................................................................................... 1500/8842
2019-08-07T09:49:21.6952283Z ...............................................................................i...............i.... 1600/8842
2019-08-07T09:49:29.7821872Z .................................................................................................... 1700/8842
2019-08-07T09:49:38.1425563Z .................................................................iiiii.............................. 1800/8842
2019-08-07T09:49:57.3032598Z .................................................................................................... 2000/8842
2019-08-07T09:49:59.7225774Z .................................................................................................... 2100/8842
2019-08-07T09:50:02.7021765Z .................................................................................................... 2200/8842
2019-08-07T09:50:10.3684036Z .................................................................................................... 2300/8842
---
2019-08-07T09:53:53.2631856Z .................................................................................................... 5200/8842
2019-08-07T09:54:03.2739928Z ....................................................................................i............... 5300/8842
2019-08-07T09:54:11.1592673Z .................................................................................................... 5400/8842
2019-08-07T09:54:16.6394689Z .................................................................................................... 5500/8842
2019-08-07T09:54:27.6344214Z ..............................................................................ii...i..ii...........i 5600/8842
2019-08-07T09:54:50.8661330Z .................................................................................................... 5800/8842
2019-08-07T09:54:56.0026480Z .................................................................................................... 5900/8842
2019-08-07T09:54:56.0026480Z .................................................................................................... 5900/8842
2019-08-07T09:55:00.7843637Z ...............................................................................i..ii................ 6000/8842
2019-08-07T09:55:31.0760635Z .................................................................................................... 6200/8842
2019-08-07T09:55:33.2503343Z ......................i............................................................................. 6300/8842
2019-08-07T09:55:35.4521732Z ..............................................................................................i..... 6400/8842
2019-08-07T09:55:38.0944814Z .................................................................................................... 6500/8842
2019-08-07T09:55:38.0944814Z .................................................................................................... 6500/8842
2019-08-07T09:55:43.0006850Z .................................................................................................... 6600/8842
2019-08-07T09:56:04.7139682Z .................................................................................................... 6700/8842
2019-08-07T09:56:21.0113179Z .................................................................................................... 6800/8842
2019-08-07T09:56:26.4953542Z .....................................F..F........................................................... 6900/8842
2019-08-07T09:56:32.0469031Z .........................................................F.FFFF..................................... 7000/8842
2019-08-07T09:56:43.7243492Z .................................................................................................... 7200/8842
2019-08-07T09:56:53.5965663Z .................................................................................................... 7300/8842
2019-08-07T09:57:03.2909019Z .................................................................................................... 7400/8842
2019-08-07T09:57:11.6011091Z ......................ii......i..................................................................... 7500/8842
---
2019-08-07T09:59:36.8109281Z 1 error[E0308]: mismatched types
2019-08-07T09:59:36.8109566Z -   --> $DIR/associated-type-projection-from-supertrait.rs:33:23
2019-08-07T09:59:36.8109814Z +   --> $DIR/associated-type-projection-from-supertrait.rs:27:23
2019-08-07T09:59:36.8109863Z 3    |
2019-08-07T09:59:36.8109908Z 4 LL | fn b() { dent(ModelT, Blue); }
2019-08-07T09:59:36.8109976Z 5    |                       ^^^^ expected struct `Black`, found struct `Blue`
2019-08-07T09:59:36.8110070Z 8               found type `Blue`
2019-08-07T09:59:36.8110127Z 9 
2019-08-07T09:59:36.8110170Z 10 error[E0308]: mismatched types
2019-08-07T09:59:36.8110424Z -   --> $DIR/associated-type-projection-from-supertrait.rs:34:23
2019-08-07T09:59:36.8110424Z -   --> $DIR/associated-type-projection-from-supertrait.rs:34:23
2019-08-07T09:59:36.8110666Z +   --> $DIR/associated-type-projection-from-supertrait.rs:28:23
2019-08-07T09:59:36.8110733Z 12    |
2019-08-07T09:59:36.8110777Z 13 LL | fn c() { dent(ModelU, Black); }
2019-08-07T09:59:36.8110829Z 14    |                       ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-07T09:59:36.8110922Z 17               found type `Black`
2019-08-07T09:59:36.8110964Z 18 
2019-08-07T09:59:36.8111006Z 19 error[E0308]: mismatched types
2019-08-07T09:59:36.8111263Z -   --> $DIR/associated-type-projection-from-supertrait.rs:40:28
2019-08-07T09:59:36.8111263Z -   --> $DIR/associated-type-projection-from-supertrait.rs:40:28
2019-08-07T09:59:36.8111501Z +   --> $DIR/associated-type-projection-from-supertrait.rs:32:28
2019-08-07T09:59:36.8111558Z 21    |
2019-08-07T09:59:36.8111619Z 22 LL | fn f() { ModelT.chip_paint(Blue); }
2019-08-07T09:59:36.8111672Z 23    |                            ^^^^ expected struct `Black`, found struct `Blue`
2019-08-07T09:59:36.8112746Z 26               found type `Blue`
2019-08-07T09:59:36.8112822Z 27 
2019-08-07T09:59:36.8112856Z 28 error[E0308]: mismatched types
2019-08-07T09:59:36.8113132Z -   --> $DIR/associated-type-projection-from-supertrait.rs:41:28
2019-08-07T09:59:36.8113132Z -   --> $DIR/associated-type-projection-from-supertrait.rs:41:28
2019-08-07T09:59:36.8113349Z +   --> $DIR/associated-type-projection-from-supertrait.rs:33:28
2019-08-07T09:59:36.8113674Z 30    |
2019-08-07T09:59:36.8113714Z 31 LL | fn g() { ModelU.chip_paint(Black); }
2019-08-07T09:59:36.8113774Z 32    |                            ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-07T09:59:36.8113822Z 
2019-08-07T09:59:36.8113857Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8114426Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
2019-08-07T09:59:36.8114426Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
2019-08-07T09:59:36.8114831Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8115137Z To only update this specific test, also pass `--test-args associated-type/associated-type-projection-from-supertrait.rs`
2019-08-07T09:59:36.8115210Z error: 1 errors occurred comparing output.
2019-08-07T09:59:36.8115248Z status: exit code: 1
2019-08-07T09:59:36.8115248Z status: exit code: 1
2019-08-07T09:59:36.8115938Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8116244Z ------------------------------------------
2019-08-07T09:59:36.8116273Z 
2019-08-07T09:59:36.8116463Z ------------------------------------------
2019-08-07T09:59:36.8116518Z stderr:
2019-08-07T09:59:36.8116518Z stderr:
2019-08-07T09:59:36.8116700Z ------------------------------------------
2019-08-07T09:59:36.8116739Z error[E0308]: mismatched types
2019-08-07T09:59:36.8116981Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:27:23
2019-08-07T09:59:36.8117025Z    |
2019-08-07T09:59:36.8117067Z LL | fn b() { dent(ModelT, Blue); } //~ ERROR mismatched types
2019-08-07T09:59:36.8117127Z    |                       ^^^^ expected struct `Black`, found struct `Blue`
2019-08-07T09:59:36.8117207Z    = note: expected type `Black`
2019-08-07T09:59:36.8117260Z               found type `Blue`
2019-08-07T09:59:36.8117285Z 
2019-08-07T09:59:36.8117320Z error[E0308]: mismatched types
2019-08-07T09:59:36.8117320Z error[E0308]: mismatched types
2019-08-07T09:59:36.8117853Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:28:23
2019-08-07T09:59:36.8118038Z    |
2019-08-07T09:59:36.8118078Z LL | fn c() { dent(ModelU, Black); } //~ ERROR mismatched types
2019-08-07T09:59:36.8118121Z    |                       ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-07T09:59:36.8118681Z    = note: expected type `Blue`
2019-08-07T09:59:36.8118733Z               found type `Black`
2019-08-07T09:59:36.8118763Z 
2019-08-07T09:59:36.8118821Z error[E0308]: mismatched types
2019-08-07T09:59:36.8118821Z error[E0308]: mismatched types
2019-08-07T09:59:36.8120869Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:32:28
2019-08-07T09:59:36.8120949Z    |
2019-08-07T09:59:36.8121021Z LL | fn f() { ModelT.chip_paint(Blue); } //~ ERROR mismatched types
2019-08-07T09:59:36.8121091Z    |                            ^^^^ expected struct `Black`, found struct `Blue`
2019-08-07T09:59:36.8121195Z    = note: expected type `Black`
2019-08-07T09:59:36.8121240Z               found type `Blue`
2019-08-07T09:59:36.8121408Z 
2019-08-07T09:59:36.8121460Z error[E0308]: mismatched types
2019-08-07T09:59:36.8121460Z error[E0308]: mismatched types
2019-08-07T09:59:36.8122157Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:33:28
2019-08-07T09:59:36.8122366Z    |
2019-08-07T09:59:36.8122406Z LL | fn g() { ModelU.chip_paint(Black); } //~ ERROR mismatched types
2019-08-07T09:59:36.8122466Z    |                            ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-07T09:59:36.8122537Z    = note: expected type `Blue`
2019-08-07T09:59:36.8122590Z               found type `Black`
2019-08-07T09:59:36.8122614Z 
2019-08-07T09:59:36.8122649Z error: aborting due to 4 previous errors
---
2019-08-07T09:59:36.8123401Z 
2019-08-07T09:59:36.8123635Z ---- [ui] ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs stdout ----
2019-08-07T09:59:36.8123676Z diff of stderr:
2019-08-07T09:59:36.8123699Z 
2019-08-07T09:59:36.8123752Z 1 error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
2019-08-07T09:59:36.8124156Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:31:10
2019-08-07T09:59:36.8124210Z 3    |
2019-08-07T09:59:36.8124210Z 3    |
2019-08-07T09:59:36.8124246Z 4 LL | fn b() { blue_car(ModelT); }
2019-08-07T09:59:36.8124286Z 5    |          ^^^^^^^^ expected struct `Black`, found struct `Blue`
2019-08-07T09:59:36.8124366Z 7    = note: expected type `Black`
2019-08-07T09:59:36.8124402Z 8               found type `Blue`
2019-08-07T09:59:36.8124402Z 8               found type `Blue`
2019-08-07T09:59:36.8124437Z 9 note: required by `blue_car`
2019-08-07T09:59:36.8124859Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:27:1
2019-08-07T09:59:36.8124897Z 11    |
2019-08-07T09:59:36.8124897Z 11    |
2019-08-07T09:59:36.8124933Z 12 LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
2019-08-07T09:59:36.8125016Z 
2019-08-07T09:59:36.8125046Z 14 
2019-08-07T09:59:36.8125046Z 14 
2019-08-07T09:59:36.8125102Z 15 error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
2019-08-07T09:59:36.8125505Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:32:10
2019-08-07T09:59:36.8125559Z 17    |
2019-08-07T09:59:36.8125559Z 17    |
2019-08-07T09:59:36.8125601Z 18 LL | fn c() { black_car(ModelU); }
2019-08-07T09:59:36.8125640Z 19    |          ^^^^^^^^^ expected struct `Blue`, found struct `Black`
2019-08-07T09:59:36.8125716Z 21    = note: expected type `Blue`
2019-08-07T09:59:36.8125757Z 22               found type `Black`
2019-08-07T09:59:36.8125757Z 22               found type `Black`
2019-08-07T09:59:36.8125792Z 23 note: required by `black_car`
2019-08-07T09:59:36.8126207Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:24:1
2019-08-07T09:59:36.8126244Z 25    |
2019-08-07T09:59:36.8126244Z 25    |
2019-08-07T09:59:36.8126294Z 26 LL | fn black_car<C:Car<Color=Black>>(c: C) {
2019-08-07T09:59:36.8126532Z 
2019-08-07T09:59:36.8126554Z 
2019-08-07T09:59:36.8126593Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8127117Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
2019-08-07T09:59:36.8127117Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
2019-08-07T09:59:36.8127346Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8127711Z To only update this specific test, also pass `--test-args associated-types/associated-types-binding-to-type-defined-in-supertrait.rs`
2019-08-07T09:59:36.8127795Z error: 1 errors occurred comparing output.
2019-08-07T09:59:36.8127850Z status: exit code: 1
2019-08-07T09:59:36.8127850Z status: exit code: 1
2019-08-07T09:59:36.8129113Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8129564Z ------------------------------------------
2019-08-07T09:59:36.8129599Z 
2019-08-07T09:59:36.8129834Z ------------------------------------------
2019-08-07T09:59:36.8129879Z stderr:
2019-08-07T09:59:36.8129879Z stderr:
2019-08-07T09:59:36.8130089Z ------------------------------------------
2019-08-07T09:59:36.8130158Z error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
2019-08-07T09:59:36.8130506Z    |
2019-08-07T09:59:36.8130506Z    |
2019-08-07T09:59:36.8130572Z LL | fn b() { blue_car(ModelT); } //~ ERROR type mismatch
2019-08-07T09:59:36.8130623Z    |          ^^^^^^^^ expected struct `Black`, found struct `Blue`
2019-08-07T09:59:36.8130735Z    = note: expected type `Black`
2019-08-07T09:59:36.8130780Z               found type `Blue`
2019-08-07T09:59:36.8130780Z               found type `Blue`
2019-08-07T09:59:36.8130824Z note: required by `blue_car`
2019-08-07T09:59:36.8131188Z    |
2019-08-07T09:59:36.8131188Z    |
2019-08-07T09:59:36.8131232Z LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
2019-08-07T09:59:36.8131327Z 
2019-08-07T09:59:36.8131327Z 
2019-08-07T09:59:36.8131375Z error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
2019-08-07T09:59:36.8131729Z    |
2019-08-07T09:59:36.8131729Z    |
2019-08-07T09:59:36.8131777Z LL | fn c() { black_car(ModelU); } //~ ERROR type mismatch
2019-08-07T09:59:36.8131991Z    |          ^^^^^^^^^ expected struct `Blue`, found struct `Black`
2019-08-07T09:59:36.8132250Z    = note: expected type `Blue`
2019-08-07T09:59:36.8132286Z               found type `Black`
2019-08-07T09:59:36.8132286Z               found type `Black`
2019-08-07T09:59:36.8132322Z note: required by `black_car`
2019-08-07T09:59:36.8132617Z    |
2019-08-07T09:59:36.8132617Z    |
2019-08-07T09:59:36.8132653Z LL | fn black_car<C:Car<Color=Black>>(c: C) {
2019-08-07T09:59:36.8132731Z 
2019-08-07T09:59:36.8132765Z error: aborting due to 2 previous errors
2019-08-07T09:59:36.8132789Z 
2019-08-07T09:59:36.8133158Z For more information about this error, try `rustc --explain E0271`.
2019-08-07T09:59:36.8133158Z For more information about this error, try `rustc --explain E0271`.
2019-08-07T09:59:36.8133185Z 
2019-08-07T09:59:36.8133351Z ------------------------------------------
2019-08-07T09:59:36.8133376Z 
2019-08-07T09:59:36.8133411Z 
2019-08-07T09:59:36.8133591Z ---- [ui] ui/hrtb/hrtb-conflate-regions.rs stdout ----
2019-08-07T09:59:36.8133637Z diff of stderr:
2019-08-07T09:59:36.8133660Z 
2019-08-07T09:59:36.8133897Z 1 error[E0277]: the trait bound `for<'a, 'b> SomeStruct: Foo<(&'a isize, &'b isize)>` is not satisfied
2019-08-07T09:59:36.8134078Z -   --> $DIR/hrtb-conflate-regions.rs:28:10
2019-08-07T09:59:36.8134318Z +   --> $DIR/hrtb-conflate-regions.rs:27:10
2019-08-07T09:59:36.8134379Z 3    |
2019-08-07T09:59:36.8134415Z 4 LL | fn b() { want_foo2::<SomeStruct>(); }
2019-08-07T09:59:36.8134850Z 5    |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a, 'b> Foo<(&'a isize, &'b isize)>` is not implemented for `SomeStruct`
2019-08-07T09:59:36.8134919Z 
2019-08-07T09:59:36.8134957Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8135208Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/hrtb-conflate-regions.stderr
2019-08-07T09:59:36.8135208Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/hrtb-conflate-regions.stderr
2019-08-07T09:59:36.8135425Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8135846Z To only update this specific test, also pass `--test-args hrtb/hrtb-conflate-regions.rs`
2019-08-07T09:59:36.8135916Z error: 1 errors occurred comparing output.
2019-08-07T09:59:36.8135969Z status: exit code: 1
2019-08-07T09:59:36.8135969Z status: exit code: 1
2019-08-07T09:59:36.8137155Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/hrtb-conflate-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8137570Z ------------------------------------------
2019-08-07T09:59:36.8137604Z 
2019-08-07T09:59:36.8138031Z ------------------------------------------
2019-08-07T09:59:36.8138077Z stderr:
2019-08-07T09:59:36.8138077Z stderr:
2019-08-07T09:59:36.8138911Z ------------------------------------------
2019-08-07T09:59:36.8139255Z error[E0277]: the trait bound `for<'a, 'b> SomeStruct: Foo<(&'a isize, &'b isize)>` is not satisfied
2019-08-07T09:59:36.8139618Z    |
2019-08-07T09:59:36.8139618Z    |
2019-08-07T09:59:36.8139689Z LL | fn b() { want_foo2::<SomeStruct>(); } //~ ERROR
2019-08-07T09:59:36.8140022Z    |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a, 'b> Foo<(&'a isize, &'b isize)>` is not implemented for `SomeStruct`
2019-08-07T09:59:36.8140146Z    = help: the following implementations were found:
2019-08-07T09:59:36.8140146Z    = help: the following implementations were found:
2019-08-07T09:59:36.8141041Z              <SomeStruct as Foo<(&'a isize, &'a isize)>>
2019-08-07T09:59:36.8141738Z note: required by `want_foo2`
2019-08-07T09:59:36.8149388Z    |
2019-08-07T09:59:36.8149388Z    |
2019-08-07T09:59:36.8149435Z LL | / fn want_foo2<T>()
2019-08-07T09:59:36.8152152Z LL | |     where T : for<'a,'b> Foo<(&'a isize, &'b isize)>
2019-08-07T09:59:36.8152247Z LL | | {
2019-08-07T09:59:36.8152314Z    | |_^
2019-08-07T09:59:36.8152354Z 
2019-08-07T09:59:36.8152407Z error: aborting due to previous error
2019-08-07T09:59:36.8152435Z 
---
2019-08-07T09:59:36.8153590Z diff of stderr:
2019-08-07T09:59:36.8153614Z 
2019-08-07T09:59:36.8153650Z 7    = note: `#[warn(incomplete_features)]` on by default
2019-08-07T09:59:36.8153702Z 8 
2019-08-07T09:59:36.8153743Z 9 error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
2019-08-07T09:59:36.8154154Z +   --> $DIR/bound-normalization-fail.rs:28:32
2019-08-07T09:59:36.8154214Z 11    |
2019-08-07T09:59:36.8154214Z 11    |
2019-08-07T09:59:36.8154573Z 12 LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
2019-08-07T09:59:36.8154645Z 13    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-07T09:59:36.8154694Z 
2019-08-07T09:59:36.8154734Z 17    = note: the return type of a function must have a statically known size
2019-08-07T09:59:36.8154772Z 18 
2019-08-07T09:59:36.8155090Z 19 error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
2019-08-07T09:59:36.8156470Z +   --> $DIR/bound-normalization-fail.rs:44:41
2019-08-07T09:59:36.8156540Z 21    |
2019-08-07T09:59:36.8156540Z 21    |
2019-08-07T09:59:36.8156937Z 22 LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
2019-08-07T09:59:36.8157966Z 23    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-07T09:59:36.8158049Z 
2019-08-07T09:59:36.8158101Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8162686Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
2019-08-07T09:59:36.8162686Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
2019-08-07T09:59:36.8163142Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8165540Z To only update this specific test, also pass `--test-args impl-trait/bound-normalization-fail.rs`
2019-08-07T09:59:36.8165664Z error: 1 errors occurred comparing output.
2019-08-07T09:59:36.8165702Z status: exit code: 1
2019-08-07T09:59:36.8165702Z status: exit code: 1
2019-08-07T09:59:36.8179682Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bound-normalization-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8180143Z ------------------------------------------
2019-08-07T09:59:36.8180207Z 
2019-08-07T09:59:36.8180467Z ------------------------------------------
2019-08-07T09:59:36.8180519Z stderr:
---
2019-08-07T09:59:36.8181326Z    |            ^^^^^^^^^^^^^^^^^^^^^^
2019-08-07T09:59:36.8181370Z    |
2019-08-07T09:59:36.8181447Z    = note: `#[warn(incomplete_features)]` on by default
2019-08-07T09:59:36.8181482Z 
2019-08-07T09:59:36.8181535Z error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
2019-08-07T09:59:36.8182207Z    |
2019-08-07T09:59:36.8182207Z    |
2019-08-07T09:59:36.8182419Z LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
2019-08-07T09:59:36.8182489Z    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-07T09:59:36.8182566Z    = note: expected type `()`
2019-08-07T09:59:36.8182566Z    = note: expected type `()`
2019-08-07T09:59:36.8182606Z               found type `<T as impl_trait::Trait>::Assoc`
2019-08-07T09:59:36.8182678Z    = note: the return type of a function must have a statically known size
2019-08-07T09:59:36.8182706Z 
2019-08-07T09:59:36.8182963Z error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
2019-08-07T09:59:36.8183425Z    |
2019-08-07T09:59:36.8183425Z    |
2019-08-07T09:59:36.8183683Z LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
2019-08-07T09:59:36.8183756Z    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-07T09:59:36.8183831Z    = note: expected type `()`
2019-08-07T09:59:36.8183831Z    = note: expected type `()`
2019-08-07T09:59:36.8184063Z               found type `<T as lifetimes::Trait<'static>>::Assoc`
2019-08-07T09:59:36.8184113Z    = note: the return type of a function must have a statically known size
2019-08-07T09:59:36.8184265Z error: aborting due to 2 previous errors
2019-08-07T09:59:36.8184307Z 
2019-08-07T09:59:36.8184551Z For more information about this error, try `rustc --explain E0271`.
2019-08-07T09:59:36.8184583Z 
2019-08-07T09:59:36.8184583Z 
2019-08-07T09:59:36.8184774Z ------------------------------------------
2019-08-07T09:59:36.8184823Z 
2019-08-07T09:59:36.8184855Z 
2019-08-07T09:59:36.8185056Z ---- [ui] ui/issues/issue-12028.rs stdout ----
2019-08-07T09:59:36.8185098Z diff of stderr:
2019-08-07T09:59:36.8185139Z 
2019-08-07T09:59:36.8185183Z 1 error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2019-08-07T09:59:36.8185386Z -   --> $DIR/issue-12028.rs:29:14
2019-08-07T09:59:36.8185577Z +   --> $DIR/issue-12028.rs:27:14
2019-08-07T09:59:36.8185673Z 4 LL |         self.input_stream(&mut stream);
2019-08-07T09:59:36.8185711Z 5    |              ^^^^^^^^^^^^
2019-08-07T09:59:36.8185752Z 
2019-08-07T09:59:36.8185774Z 
2019-08-07T09:59:36.8185774Z 
2019-08-07T09:59:36.8185822Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8186083Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/issue-12028.stderr
2019-08-07T09:59:36.8186322Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8186582Z To only update this specific test, also pass `--test-args issues/issue-12028.rs`
2019-08-07T09:59:36.8186675Z error: 1 errors occurred comparing output.
2019-08-07T09:59:36.8186715Z status: exit code: 1
2019-08-07T09:59:36.8186715Z status: exit code: 1
2019-08-07T09:59:36.8187327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12028.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8187657Z ------------------------------------------
2019-08-07T09:59:36.8187691Z 
2019-08-07T09:59:36.8187928Z ------------------------------------------
2019-08-07T09:59:36.8187979Z stderr:
2019-08-07T09:59:36.8187979Z stderr:
2019-08-07T09:59:36.8188186Z ------------------------------------------
2019-08-07T09:59:36.8188618Z error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2019-08-07T09:59:36.8188984Z   --> /checkout/src/test/ui/issues/issue-12028.rs:27:14
2019-08-07T09:59:36.8189046Z    |
2019-08-07T09:59:36.8189123Z LL |         self.input_stream(&mut stream); //~ ERROR type annotations required
2019-08-07T09:59:36.8189210Z 
2019-08-07T09:59:36.8189276Z error: aborting due to previous error
2019-08-07T09:59:36.8189310Z 
2019-08-07T09:59:36.8189602Z For more information about this error, try `rustc --explain E0284`.
2019-08-07T09:59:36.8189602Z For more information about this error, try `rustc --explain E0284`.
2019-08-07T09:59:36.8189656Z 
2019-08-07T09:59:36.8189915Z ------------------------------------------
2019-08-07T09:59:36.8189974Z 
2019-08-07T09:59:36.8190004Z 
2019-08-07T09:59:36.8190473Z ---- [ui] ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs#migrate stdout ----
2019-08-07T09:59:36.8190553Z diff of stderr:
2019-08-07T09:59:36.8190609Z 
2019-08-07T09:59:36.8190992Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8191623Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-07T09:59:36.8191678Z 3    |
2019-08-07T09:59:36.8191678Z 3    |
2019-08-07T09:59:36.8192114Z 4 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8192206Z 
2019-08-07T09:59:36.8192325Z 6    |
2019-08-07T09:59:36.8192325Z 6    |
2019-08-07T09:59:36.8192594Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 37:15
2019-08-07T09:59:36.8192843Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:37:15
2019-08-07T09:59:36.8193086Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 33:15
2019-08-07T09:59:36.8193310Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:33:15
2019-08-07T09:59:36.8193371Z 9    |
2019-08-07T09:59:36.8193563Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8193631Z 
2019-08-07T09:59:36.8193631Z 
2019-08-07T09:59:36.8194065Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 37:18
2019-08-07T09:59:36.8194285Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:37:18
2019-08-07T09:59:36.8194527Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 33:18
2019-08-07T09:59:36.8194971Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:33:18
2019-08-07T09:59:36.8195014Z 14    |
2019-08-07T09:59:36.8195198Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8195282Z 
2019-08-07T09:59:36.8195303Z 
2019-08-07T09:59:36.8195348Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8195348Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8195696Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate/regions-assoc-type-in-supertrait-outlives-container.migrate.stderr
2019-08-07T09:59:36.8195923Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8196205Z To only update this specific test, also pass `--test-args regions/regions-assoc-type-in-supertrait-outlives-container.rs`
2019-08-07T09:59:36.8196242Z 
2019-08-07T09:59:36.8196282Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-07T09:59:36.8196330Z status: exit code: 1
2019-08-07T09:59:36.8197062Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.migrate/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8197372Z ------------------------------------------
2019-08-07T09:59:36.8197403Z 
2019-08-07T09:59:36.8197754Z ------------------------------------------
2019-08-07T09:59:36.8197804Z stderr:
2019-08-07T09:59:36.8197804Z stderr:
2019-08-07T09:59:36.8198200Z ------------------------------------------
2019-08-07T09:59:36.8198948Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8199488Z    |
2019-08-07T09:59:36.8199488Z    |
2019-08-07T09:59:36.8199787Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8199918Z    |
2019-08-07T09:59:36.8199918Z    |
2019-08-07T09:59:36.8200206Z note: the pointer is valid for the lifetime 'a as defined on the function body at 33:15
2019-08-07T09:59:36.8200590Z    |
2019-08-07T09:59:36.8200590Z    |
2019-08-07T09:59:36.8200820Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8200892Z    |               ^^
2019-08-07T09:59:36.8201201Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 33:18
2019-08-07T09:59:36.8201704Z    |
2019-08-07T09:59:36.8201704Z    |
2019-08-07T09:59:36.8201936Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8202031Z 
2019-08-07T09:59:36.8202263Z error: aborting due to previous error
2019-08-07T09:59:36.8202292Z 
2019-08-07T09:59:36.8202686Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-07T09:59:36.8203662Z 1 error: lifetime may not live long enough
2019-08-07T09:59:36.8203879Z -   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:43:12
2019-08-07T09:59:36.8204104Z +   --> $DIR/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-07T09:59:36.8204142Z 3    |
2019-08-07T09:59:36.8204311Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8204529Z 5    |               -- -- lifetime `'b` defined here
2019-08-07T09:59:36.8204577Z 
2019-08-07T09:59:36.8204612Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8204612Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8204931Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll/regions-assoc-type-in-supertrait-outlives-container.nll.stderr
2019-08-07T09:59:36.8205141Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8205400Z To only update this specific test, also pass `--test-args regions/regions-assoc-type-in-supertrait-outlives-container.rs`
2019-08-07T09:59:36.8205433Z 
2019-08-07T09:59:36.8205469Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-07T09:59:36.8205512Z status: exit code: 1
2019-08-07T09:59:36.8206191Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.nll/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8206671Z ------------------------------------------
2019-08-07T09:59:36.8206703Z 
2019-08-07T09:59:36.8206919Z ------------------------------------------
2019-08-07T09:59:36.8206971Z stderr:
2019-08-07T09:59:36.8206971Z stderr:
2019-08-07T09:59:36.8207166Z ------------------------------------------
2019-08-07T09:59:36.8207209Z error: lifetime may not live long enough
2019-08-07T09:59:36.8207728Z   --> /checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs:39:12
2019-08-07T09:59:36.8207798Z    |
2019-08-07T09:59:36.8208022Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8209042Z    |               -- -- lifetime `'b` defined here
2019-08-07T09:59:36.8209342Z    |               lifetime `'a` defined here
2019-08-07T09:59:36.8209416Z ...
2019-08-07T09:59:36.8209416Z ...
2019-08-07T09:59:36.8209672Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8209970Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-07T09:59:36.8210083Z error: aborting due to previous error
2019-08-07T09:59:36.8210112Z 
2019-08-07T09:59:36.8210253Z 
2019-08-07T09:59:36.8210525Z ------------------------------------------
2019-08-07T09:59:36.8210525Z ------------------------------------------
2019-08-07T09:59:36.8210584Z 
2019-08-07T09:59:36.8210611Z 
2019-08-07T09:59:36.8210894Z ---- [ui] ui/regions/regions-outlives-projection-container-hrtb.rs#migrate stdout ----
2019-08-07T09:59:36.8210952Z diff of stderr:
2019-08-07T09:59:36.8211014Z 
2019-08-07T09:59:36.8211333Z 1 error[E0491]: in type `&'a WithHrAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8212058Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-07T09:59:36.8212102Z 3    |
2019-08-07T09:59:36.8212102Z 3    |
2019-08-07T09:59:36.8212308Z 4 LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8212400Z 
2019-08-07T09:59:36.8212431Z 6    |
2019-08-07T09:59:36.8212431Z 6    |
2019-08-07T09:59:36.8212661Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 32:15
2019-08-07T09:59:36.8212913Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:32:15
2019-08-07T09:59:36.8213309Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-07T09:59:36.8213527Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:27:15
2019-08-07T09:59:36.8213588Z 9    |
2019-08-07T09:59:36.8213777Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8213841Z 
2019-08-07T09:59:36.8213841Z 
2019-08-07T09:59:36.8214098Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 32:18
2019-08-07T09:59:36.8214311Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:32:18
2019-08-07T09:59:36.8214551Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-07T09:59:36.8214785Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:27:18
2019-08-07T09:59:36.8214824Z 14    |
2019-08-07T09:59:36.8215017Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8215291Z 
2019-08-07T09:59:36.8215321Z 17 
2019-08-07T09:59:36.8215321Z 17 
2019-08-07T09:59:36.8215579Z 18 error[E0491]: in type `&'a WithHrAssocSub<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8216052Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-07T09:59:36.8216093Z 20    |
2019-08-07T09:59:36.8216093Z 20    |
2019-08-07T09:59:36.8216323Z 21 LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-07T09:59:36.8216398Z 
2019-08-07T09:59:36.8216448Z 23    |
2019-08-07T09:59:36.8216448Z 23    |
2019-08-07T09:59:36.8216681Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 53:19
2019-08-07T09:59:36.8216894Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:53:19
2019-08-07T09:59:36.8217153Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 46:19
2019-08-07T09:59:36.8217373Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:46:19
2019-08-07T09:59:36.8217414Z 26    |
2019-08-07T09:59:36.8217601Z 27 LL | fn with_assoc_sub<'a,'b>() {
2019-08-07T09:59:36.8218105Z 
2019-08-07T09:59:36.8218105Z 
2019-08-07T09:59:36.8218499Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 53:22
2019-08-07T09:59:36.8218799Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:53:22
2019-08-07T09:59:36.8219114Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 46:22
2019-08-07T09:59:36.8219386Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:46:22
2019-08-07T09:59:36.8219458Z 31    |
2019-08-07T09:59:36.8219697Z 32 LL | fn with_assoc_sub<'a,'b>() {
2019-08-07T09:59:36.8219893Z 
2019-08-07T09:59:36.8219944Z 
2019-08-07T09:59:36.8219993Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8219993Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8220433Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate/regions-outlives-projection-container-hrtb.migrate.stderr
2019-08-07T09:59:36.8220744Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8221069Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-07T09:59:36.8221111Z 
2019-08-07T09:59:36.8221180Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-07T09:59:36.8221398Z status: exit code: 1
2019-08-07T09:59:36.8222589Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8222885Z ------------------------------------------
2019-08-07T09:59:36.8222932Z 
2019-08-07T09:59:36.8223113Z ------------------------------------------
2019-08-07T09:59:36.8223151Z stderr:
2019-08-07T09:59:36.8223151Z stderr:
2019-08-07T09:59:36.8223340Z ------------------------------------------
2019-08-07T09:59:36.8223578Z error[E0491]: in type `&'a WithHrAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8223871Z    |
2019-08-07T09:59:36.8223871Z    |
2019-08-07T09:59:36.8224063Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8224153Z    |
2019-08-07T09:59:36.8224153Z    |
2019-08-07T09:59:36.8224372Z note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-07T09:59:36.8224628Z    |
2019-08-07T09:59:36.8224628Z    |
2019-08-07T09:59:36.8224815Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8224853Z    |               ^^
2019-08-07T09:59:36.8225073Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-07T09:59:36.8225350Z    |
2019-08-07T09:59:36.8225350Z    |
2019-08-07T09:59:36.8225513Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8225601Z 
2019-08-07T09:59:36.8225601Z 
2019-08-07T09:59:36.8225833Z error[E0491]: in type `&'a WithHrAssocSub<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8226112Z    |
2019-08-07T09:59:36.8226112Z    |
2019-08-07T09:59:36.8226376Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-07T09:59:36.8226485Z    |
2019-08-07T09:59:36.8226485Z    |
2019-08-07T09:59:36.8226722Z note: the pointer is valid for the lifetime 'a as defined on the function body at 46:19
2019-08-07T09:59:36.8226993Z    |
2019-08-07T09:59:36.8226993Z    |
2019-08-07T09:59:36.8227164Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-07T09:59:36.8227202Z    |                   ^^
2019-08-07T09:59:36.8227438Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 46:22
2019-08-07T09:59:36.8227797Z    |
2019-08-07T09:59:36.8227797Z    |
2019-08-07T09:59:36.8227985Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-07T09:59:36.8228049Z 
2019-08-07T09:59:36.8228225Z error: aborting due to 2 previous errors
2019-08-07T09:59:36.8228468Z 
2019-08-07T09:59:36.8228790Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-07T09:59:36.8229587Z 1 error: lifetime may not live long enough
2019-08-07T09:59:36.8229853Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:35:12
2019-08-07T09:59:36.8230136Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-07T09:59:36.8230212Z 3    |
2019-08-07T09:59:36.8230443Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8230699Z 5    |               -- -- lifetime `'b` defined here
2019-08-07T09:59:36.8231068Z 10    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-07T09:59:36.8231126Z 11 
2019-08-07T09:59:36.8231191Z 12 error: lifetime may not live long enough
2019-08-07T09:59:36.8231467Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-07T09:59:36.8231467Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-07T09:59:36.8232055Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-07T09:59:36.8232095Z 14    |
2019-08-07T09:59:36.8232289Z 15 LL | fn with_assoc_sub<'a,'b>() {
2019-08-07T09:59:36.8232482Z 16    |                   -- -- lifetime `'b` defined here
2019-08-07T09:59:36.8232530Z 
2019-08-07T09:59:36.8232582Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8232582Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8232880Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll/regions-outlives-projection-container-hrtb.nll.stderr
2019-08-07T09:59:36.8233116Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8233376Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-07T09:59:36.8233408Z 
2019-08-07T09:59:36.8233445Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-07T09:59:36.8233499Z status: exit code: 1
2019-08-07T09:59:36.8234220Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8234567Z ------------------------------------------
2019-08-07T09:59:36.8234599Z 
2019-08-07T09:59:36.8234816Z ------------------------------------------
2019-08-07T09:59:36.8234855Z stderr:
2019-08-07T09:59:36.8234855Z stderr:
2019-08-07T09:59:36.8235041Z ------------------------------------------
2019-08-07T09:59:36.8235101Z error: lifetime may not live long enough
2019-08-07T09:59:36.8235329Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-07T09:59:36.8235373Z    |
2019-08-07T09:59:36.8235580Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8235778Z    |               -- -- lifetime `'b` defined here
2019-08-07T09:59:36.8236155Z    |               lifetime `'a` defined here
2019-08-07T09:59:36.8236196Z ...
2019-08-07T09:59:36.8236196Z ...
2019-08-07T09:59:36.8236398Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8236660Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-07T09:59:36.8236916Z error: lifetime may not live long enough
2019-08-07T09:59:36.8237152Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-07T09:59:36.8237213Z    |
2019-08-07T09:59:36.8237213Z    |
2019-08-07T09:59:36.8237780Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-07T09:59:36.8238032Z    |                   -- -- lifetime `'b` defined here
2019-08-07T09:59:36.8238878Z    |                   lifetime `'a` defined here
2019-08-07T09:59:36.8238931Z ...
2019-08-07T09:59:36.8238931Z ...
2019-08-07T09:59:36.8239209Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-07T09:59:36.8239539Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-07T09:59:36.8239631Z error: aborting due to 2 previous errors
2019-08-07T09:59:36.8239682Z 
2019-08-07T09:59:36.8239711Z 
2019-08-07T09:59:36.8239966Z ------------------------------------------
2019-08-07T09:59:36.8239966Z ------------------------------------------
2019-08-07T09:59:36.8240004Z 
2019-08-07T09:59:36.8240030Z 
2019-08-07T09:59:36.8240332Z ---- [ui] ui/regions/regions-outlives-projection-container-wc.rs#migrate stdout ----
2019-08-07T09:59:36.8240387Z diff of stderr:
2019-08-07T09:59:36.8240417Z 
2019-08-07T09:59:36.8240747Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8241309Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-07T09:59:36.8241382Z 3    |
2019-08-07T09:59:36.8241382Z 3    |
2019-08-07T09:59:36.8241638Z 4 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8241910Z 
2019-08-07T09:59:36.8241966Z 6    |
2019-08-07T09:59:36.8241966Z 6    |
2019-08-07T09:59:36.8242210Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 31:15
2019-08-07T09:59:36.8242439Z -   --> $DIR/regions-outlives-projection-container-wc.rs:31:15
2019-08-07T09:59:36.8242701Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-07T09:59:36.8242925Z +   --> $DIR/regions-outlives-projection-container-wc.rs:27:15
2019-08-07T09:59:36.8242968Z 9    |
2019-08-07T09:59:36.8243177Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8243247Z 
2019-08-07T09:59:36.8243247Z 
2019-08-07T09:59:36.8243501Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 31:18
2019-08-07T09:59:36.8243743Z -   --> $DIR/regions-outlives-projection-container-wc.rs:31:18
2019-08-07T09:59:36.8244011Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-07T09:59:36.8244233Z +   --> $DIR/regions-outlives-projection-container-wc.rs:27:18
2019-08-07T09:59:36.8244296Z 14    |
2019-08-07T09:59:36.8244491Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8244684Z 
2019-08-07T09:59:36.8244728Z 
2019-08-07T09:59:36.8244767Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8244767Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8245121Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate/regions-outlives-projection-container-wc.migrate.stderr
2019-08-07T09:59:36.8245376Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8245636Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-07T09:59:36.8245671Z 
2019-08-07T09:59:36.8245730Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-07T09:59:36.8245857Z status: exit code: 1
2019-08-07T09:59:36.8246800Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8247145Z ------------------------------------------
2019-08-07T09:59:36.8247201Z 
2019-08-07T09:59:36.8247425Z ------------------------------------------
2019-08-07T09:59:36.8247482Z stderr:
2019-08-07T09:59:36.8247482Z stderr:
2019-08-07T09:59:36.8247879Z ------------------------------------------
2019-08-07T09:59:36.8248160Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8248994Z    |
2019-08-07T09:59:36.8248994Z    |
2019-08-07T09:59:36.8249257Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8249380Z    |
2019-08-07T09:59:36.8249380Z    |
2019-08-07T09:59:36.8249666Z note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-07T09:59:36.8250032Z    |
2019-08-07T09:59:36.8250032Z    |
2019-08-07T09:59:36.8250264Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8250317Z    |               ^^
2019-08-07T09:59:36.8250620Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-07T09:59:36.8251010Z    |
2019-08-07T09:59:36.8251010Z    |
2019-08-07T09:59:36.8251234Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8251348Z 
2019-08-07T09:59:36.8251393Z error: aborting due to previous error
2019-08-07T09:59:36.8251424Z 
2019-08-07T09:59:36.8251714Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-07T09:59:36.8252581Z 1 error: lifetime may not live long enough
2019-08-07T09:59:36.8252806Z -   --> $DIR/regions-outlives-projection-container-wc.rs:37:12
2019-08-07T09:59:36.8253083Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-07T09:59:36.8253148Z 3    |
2019-08-07T09:59:36.8253518Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8253731Z 5    |               -- -- lifetime `'b` defined here
2019-08-07T09:59:36.8253909Z 
2019-08-07T09:59:36.8253950Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8253950Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8254475Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll/regions-outlives-projection-container-wc.nll.stderr
2019-08-07T09:59:36.8254919Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8255197Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-07T09:59:36.8255235Z 
2019-08-07T09:59:36.8255295Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-07T09:59:36.8255421Z status: exit code: 1
2019-08-07T09:59:36.8256714Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8257055Z ------------------------------------------
2019-08-07T09:59:36.8257110Z 
2019-08-07T09:59:36.8257490Z ------------------------------------------
2019-08-07T09:59:36.8257534Z stderr:
2019-08-07T09:59:36.8257534Z stderr:
2019-08-07T09:59:36.8257768Z ------------------------------------------
2019-08-07T09:59:36.8257981Z error: lifetime may not live long enough
2019-08-07T09:59:36.8258645Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs:33:12
2019-08-07T09:59:36.8258730Z    |
2019-08-07T09:59:36.8258985Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8259243Z    |               -- -- lifetime `'b` defined here
2019-08-07T09:59:36.8259562Z    |               lifetime `'a` defined here
2019-08-07T09:59:36.8259614Z ...
2019-08-07T09:59:36.8259614Z ...
2019-08-07T09:59:36.8259863Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8260188Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-07T09:59:36.8260277Z error: aborting due to previous error
2019-08-07T09:59:36.8260306Z 
2019-08-07T09:59:36.8260352Z 
2019-08-07T09:59:36.8260595Z ------------------------------------------
2019-08-07T09:59:36.8260595Z ------------------------------------------
2019-08-07T09:59:36.8260644Z 
2019-08-07T09:59:36.8260670Z 
2019-08-07T09:59:36.8260962Z ---- [ui] ui/regions/regions-outlives-projection-container.rs stdout ----
2019-08-07T09:59:36.8261021Z diff of stderr:
2019-08-07T09:59:36.8261051Z 
2019-08-07T09:59:36.8261378Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8262062Z +   --> $DIR/regions-outlives-projection-container.rs:36:13
2019-08-07T09:59:36.8262104Z 3    |
2019-08-07T09:59:36.8262104Z 3    |
2019-08-07T09:59:36.8262329Z 4 LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8262398Z 
2019-08-07T09:59:36.8262429Z 6    |
2019-08-07T09:59:36.8262429Z 6    |
2019-08-07T09:59:36.8262685Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 32:15
2019-08-07T09:59:36.8263066Z -   --> $DIR/regions-outlives-projection-container.rs:32:15
2019-08-07T09:59:36.8263303Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 28:15
2019-08-07T09:59:36.8263531Z +   --> $DIR/regions-outlives-projection-container.rs:28:15
2019-08-07T09:59:36.8263571Z 9    |
2019-08-07T09:59:36.8263750Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8263936Z 
2019-08-07T09:59:36.8263936Z 
2019-08-07T09:59:36.8264207Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 32:18
2019-08-07T09:59:36.8264436Z -   --> $DIR/regions-outlives-projection-container.rs:32:18
2019-08-07T09:59:36.8264854Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 28:18
2019-08-07T09:59:36.8265058Z +   --> $DIR/regions-outlives-projection-container.rs:28:18
2019-08-07T09:59:36.8265114Z 14    |
2019-08-07T09:59:36.8265288Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8265423Z 
2019-08-07T09:59:36.8265472Z 17 
2019-08-07T09:59:36.8265472Z 17 
2019-08-07T09:59:36.8265733Z 18 error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8266147Z +   --> $DIR/regions-outlives-projection-container.rs:54:13
2019-08-07T09:59:36.8266189Z 20    |
2019-08-07T09:59:36.8266189Z 20    |
2019-08-07T09:59:36.8266383Z 21 LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8266472Z 
2019-08-07T09:59:36.8266504Z 23    |
2019-08-07T09:59:36.8266504Z 23    |
2019-08-07T09:59:36.8266729Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 54:18
2019-08-07T09:59:36.8266957Z -   --> $DIR/regions-outlives-projection-container.rs:54:18
2019-08-07T09:59:36.8267190Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 50:18
2019-08-07T09:59:36.8267397Z +   --> $DIR/regions-outlives-projection-container.rs:50:18
2019-08-07T09:59:36.8267464Z 26    |
2019-08-07T09:59:36.8267717Z 27 LL | fn without_assoc<'a,'b>() {
2019-08-07T09:59:36.8267781Z 
2019-08-07T09:59:36.8267781Z 
2019-08-07T09:59:36.8268047Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 54:21
2019-08-07T09:59:36.8269141Z -   --> $DIR/regions-outlives-projection-container.rs:54:21
2019-08-07T09:59:36.8269465Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 50:21
2019-08-07T09:59:36.8269762Z +   --> $DIR/regions-outlives-projection-container.rs:50:21
2019-08-07T09:59:36.8269815Z 31    |
2019-08-07T09:59:36.8270050Z 32 LL | fn without_assoc<'a,'b>() {
2019-08-07T09:59:36.8270155Z 
2019-08-07T09:59:36.8270195Z 34 
2019-08-07T09:59:36.8270195Z 34 
2019-08-07T09:59:36.8270512Z 35 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8271087Z +   --> $DIR/regions-outlives-projection-container.rs:63:12
2019-08-07T09:59:36.8271138Z 37    |
2019-08-07T09:59:36.8271138Z 37    |
2019-08-07T09:59:36.8271406Z 38 LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-07T09:59:36.8271506Z 
2019-08-07T09:59:36.8271564Z 40    |
2019-08-07T09:59:36.8271564Z 40    |
2019-08-07T09:59:36.8272025Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 62:20
2019-08-07T09:59:36.8272418Z -   --> $DIR/regions-outlives-projection-container.rs:62:20
2019-08-07T09:59:36.8272653Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 58:20
2019-08-07T09:59:36.8272890Z +   --> $DIR/regions-outlives-projection-container.rs:58:20
2019-08-07T09:59:36.8272934Z 43    |
2019-08-07T09:59:36.8273124Z 44 LL | fn call_with_assoc<'a,'b>() {
2019-08-07T09:59:36.8273222Z 
2019-08-07T09:59:36.8273222Z 
2019-08-07T09:59:36.8273469Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 62:23
2019-08-07T09:59:36.8273699Z -   --> $DIR/regions-outlives-projection-container.rs:62:23
2019-08-07T09:59:36.8274035Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 58:23
2019-08-07T09:59:36.8274293Z +   --> $DIR/regions-outlives-projection-container.rs:58:23
2019-08-07T09:59:36.8274356Z 48    |
2019-08-07T09:59:36.8274548Z 49 LL | fn call_with_assoc<'a,'b>() {
2019-08-07T09:59:36.8274614Z 
2019-08-07T09:59:36.8274664Z 51 
2019-08-07T09:59:36.8274664Z 51 
2019-08-07T09:59:36.8274918Z 52 error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8275360Z +   --> $DIR/regions-outlives-projection-container.rs:70:12
2019-08-07T09:59:36.8275484Z 54    |
2019-08-07T09:59:36.8275484Z 54    |
2019-08-07T09:59:36.8275711Z 55 LL |     call::<&'a WithoutAssoc<TheType<'b>>>();
2019-08-07T09:59:36.8275805Z 
2019-08-07T09:59:36.8275838Z 57    |
2019-08-07T09:59:36.8275838Z 57    |
2019-08-07T09:59:36.8276260Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 71:23
2019-08-07T09:59:36.8276502Z -   --> $DIR/regions-outlives-projection-container.rs:71:23
2019-08-07T09:59:36.8276742Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 67:23
2019-08-07T09:59:36.8276958Z +   --> $DIR/regions-outlives-projection-container.rs:67:23
2019-08-07T09:59:36.8277019Z 60    |
2019-08-07T09:59:36.8277219Z 61 LL | fn call_without_assoc<'a,'b>() {
2019-08-07T09:59:36.8277290Z 
2019-08-07T09:59:36.8277290Z 
2019-08-07T09:59:36.8277926Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 71:26
2019-08-07T09:59:36.8278597Z -   --> $DIR/regions-outlives-projection-container.rs:71:26
2019-08-07T09:59:36.8278945Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 67:26
2019-08-07T09:59:36.8279241Z +   --> $DIR/regions-outlives-projection-container.rs:67:26
2019-08-07T09:59:36.8279307Z 65    |
2019-08-07T09:59:36.8279552Z 66 LL | fn call_without_assoc<'a,'b>() {
2019-08-07T09:59:36.8279658Z 
2019-08-07T09:59:36.8279684Z 
2019-08-07T09:59:36.8279730Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8280121Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
2019-08-07T09:59:36.8280121Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
2019-08-07T09:59:36.8280408Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8280725Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container.rs`
2019-08-07T09:59:36.8280847Z error: 1 errors occurred comparing output.
2019-08-07T09:59:36.8280894Z status: exit code: 1
2019-08-07T09:59:36.8280894Z status: exit code: 1
2019-08-07T09:59:36.8282062Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8282366Z ------------------------------------------
2019-08-07T09:59:36.8282396Z 
2019-08-07T09:59:36.8282590Z ------------------------------------------
2019-08-07T09:59:36.8282658Z stderr:
2019-08-07T09:59:36.8282658Z stderr:
2019-08-07T09:59:36.8283042Z ------------------------------------------
2019-08-07T09:59:36.8283303Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8283733Z    |
2019-08-07T09:59:36.8283733Z    |
2019-08-07T09:59:36.8283978Z LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8284085Z    |
2019-08-07T09:59:36.8284085Z    |
2019-08-07T09:59:36.8284318Z note: the pointer is valid for the lifetime 'a as defined on the function body at 28:15
2019-08-07T09:59:36.8284623Z    |
2019-08-07T09:59:36.8284623Z    |
2019-08-07T09:59:36.8284814Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8284857Z    |               ^^
2019-08-07T09:59:36.8285243Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 28:18
2019-08-07T09:59:36.8285528Z    |
2019-08-07T09:59:36.8285528Z    |
2019-08-07T09:59:36.8285734Z LL | fn with_assoc<'a,'b>() {
2019-08-07T09:59:36.8285814Z 
2019-08-07T09:59:36.8285814Z 
2019-08-07T09:59:36.8286088Z error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8286558Z    |
2019-08-07T09:59:36.8286558Z    |
2019-08-07T09:59:36.8286967Z LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-07T09:59:36.8287075Z    |
2019-08-07T09:59:36.8287075Z    |
2019-08-07T09:59:36.8287534Z note: the pointer is valid for the lifetime 'a as defined on the function body at 50:18
2019-08-07T09:59:36.8287858Z    |
2019-08-07T09:59:36.8287858Z    |
2019-08-07T09:59:36.8288051Z LL | fn without_assoc<'a,'b>() {
2019-08-07T09:59:36.8288114Z    |                  ^^
2019-08-07T09:59:36.8288522Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 50:21
2019-08-07T09:59:36.8289269Z    |
2019-08-07T09:59:36.8289269Z    |
2019-08-07T09:59:36.8289499Z LL | fn without_assoc<'a,'b>() {
2019-08-07T09:59:36.8289583Z 
2019-08-07T09:59:36.8289583Z 
2019-08-07T09:59:36.8289911Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8290257Z    |
2019-08-07T09:59:36.8290257Z    |
2019-08-07T09:59:36.8290518Z LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-07T09:59:36.8290631Z    |
2019-08-07T09:59:36.8290631Z    |
2019-08-07T09:59:36.8290937Z note: the pointer is valid for the lifetime 'a as defined on the function body at 58:20
2019-08-07T09:59:36.8291294Z    |
2019-08-07T09:59:36.8291294Z    |
2019-08-07T09:59:36.8291545Z LL | fn call_with_assoc<'a,'b>() {
2019-08-07T09:59:36.8291599Z    |                    ^^
2019-08-07T09:59:36.8291902Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 58:23
2019-08-07T09:59:36.8292607Z    |
2019-08-07T09:59:36.8292607Z    |
2019-08-07T09:59:36.8293033Z LL | fn call_with_assoc<'a,'b>() {
2019-08-07T09:59:36.8293116Z 
2019-08-07T09:59:36.8293116Z 
2019-08-07T09:59:36.8293369Z error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-07T09:59:36.8293677Z    |
2019-08-07T09:59:36.8293677Z    |
2019-08-07T09:59:36.8294090Z LL |     call::<&'a WithoutAssoc<TheType<'b>>>(); //~ ERROR reference has a longer lifetime
2019-08-07T09:59:36.8294297Z    |
2019-08-07T09:59:36.8294297Z    |
2019-08-07T09:59:36.8294747Z note: the pointer is valid for the lifetime 'a as defined on the function body at 67:23
2019-08-07T09:59:36.8295056Z    |
2019-08-07T09:59:36.8295056Z    |
2019-08-07T09:59:36.8295257Z LL | fn call_without_assoc<'a,'b>() {
2019-08-07T09:59:36.8295301Z    |                       ^^
2019-08-07T09:59:36.8295557Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 67:26
2019-08-07T09:59:36.8295957Z    |
2019-08-07T09:59:36.8295957Z    |
2019-08-07T09:59:36.8296177Z LL | fn call_without_assoc<'a,'b>() {
2019-08-07T09:59:36.8296270Z 
2019-08-07T09:59:36.8296307Z error: aborting due to 4 previous errors
2019-08-07T09:59:36.8296341Z 
2019-08-07T09:59:36.8296593Z For more information about this error, try `rustc --explain E0491`.
2019-08-07T09:59:36.8296593Z For more information about this error, try `rustc --explain E0491`.
2019-08-07T09:59:36.8296625Z 
2019-08-07T09:59:36.8296992Z ------------------------------------------
2019-08-07T09:59:36.8297022Z 
2019-08-07T09:59:36.8297064Z 
2019-08-07T09:59:36.8297291Z ---- [ui] ui/specialization/defaultimpl/specialization-no-default.rs stdout ----
2019-08-07T09:59:36.8297336Z diff of stderr:
2019-08-07T09:59:36.8297360Z 
2019-08-07T09:59:36.8297421Z 1 error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8297998Z +   --> $DIR/specialization-no-default.rs:20:5
2019-08-07T09:59:36.8298064Z 3    |
2019-08-07T09:59:36.8298064Z 3    |
2019-08-07T09:59:36.8298099Z 4 LL | / impl<T> Foo for T {
2019-08-07T09:59:36.8298133Z 5 LL | |     fn foo(&self) {}
2019-08-07T09:59:36.8298155Z 
2019-08-07T09:59:36.8298217Z 13    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8298254Z 14 
2019-08-07T09:59:36.8298294Z 15 error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8299361Z +   --> $DIR/specialization-no-default.rs:23:5
2019-08-07T09:59:36.8299414Z 17    |
2019-08-07T09:59:36.8299414Z 17    |
2019-08-07T09:59:36.8299481Z 18 LL | / impl<T> Foo for T {
2019-08-07T09:59:36.8299528Z 19 LL | |     fn foo(&self) {}
2019-08-07T09:59:36.8299558Z 
2019-08-07T09:59:36.8299607Z 27    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8299676Z 28 
2019-08-07T09:59:36.8299729Z 29 error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8300274Z +   --> $DIR/specialization-no-default.rs:37:5
2019-08-07T09:59:36.8300327Z 31    |
2019-08-07T09:59:36.8300327Z 31    |
2019-08-07T09:59:36.8300381Z 32 LL | / impl<T> Bar for T {
2019-08-07T09:59:36.8300446Z 33 LL | |     type T = u8;
2019-08-07T09:59:36.8300477Z 
2019-08-07T09:59:36.8300526Z 40    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8300571Z 41 
2019-08-07T09:59:36.8300642Z 42 error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8301156Z +   --> $DIR/specialization-no-default.rs:55:5
2019-08-07T09:59:36.8301228Z 44    |
2019-08-07T09:59:36.8301228Z 44    |
2019-08-07T09:59:36.8301276Z 45 LL | / impl<T: Clone> Baz for T {
2019-08-07T09:59:36.8301323Z 46 LL | |     fn baz(&self) {}
2019-08-07T09:59:36.8301365Z 
2019-08-07T09:59:36.8301433Z 53    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8301479Z 54 
2019-08-07T09:59:36.8301532Z 55 error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8302546Z +   --> $DIR/specialization-no-default.rs:74:5
2019-08-07T09:59:36.8302590Z 57    |
2019-08-07T09:59:36.8302590Z 57    |
2019-08-07T09:59:36.8302646Z 58 LL | / impl<T: Clone> Redundant for T {
2019-08-07T09:59:36.8302685Z 59 LL | |     fn redundant(&self) {}
2019-08-07T09:59:36.8302731Z 
2019-08-07T09:59:36.8302786Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8303252Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/specialization-no-default.stderr
2019-08-07T09:59:36.8303252Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/specialization-no-default.stderr
2019-08-07T09:59:36.8303477Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8304076Z To only update this specific test, also pass `--test-args specialization/defaultimpl/specialization-no-default.rs`
2019-08-07T09:59:36.8304151Z error: 1 errors occurred comparing output.
2019-08-07T09:59:36.8304206Z status: exit code: 1
2019-08-07T09:59:36.8304206Z status: exit code: 1
2019-08-07T09:59:36.8304902Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/defaultimpl/specialization-no-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8305219Z ------------------------------------------
2019-08-07T09:59:36.8305251Z 
2019-08-07T09:59:36.8305467Z ------------------------------------------
2019-08-07T09:59:36.8305511Z stderr:
2019-08-07T09:59:36.8305511Z stderr:
2019-08-07T09:59:36.8305709Z ------------------------------------------
2019-08-07T09:59:36.8305768Z error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8306083Z    |
2019-08-07T09:59:36.8306083Z    |
2019-08-07T09:59:36.8306119Z LL | / impl<T> Foo for T {
2019-08-07T09:59:36.8306174Z LL | |     fn foo(&self) {}
2019-08-07T09:59:36.8306212Z LL | |     fn bar(&self) {}
2019-08-07T09:59:36.8306247Z LL | | }
2019-08-07T09:59:36.8306457Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8306497Z ...
2019-08-07T09:59:36.8306535Z LL |       fn foo(&self) {} //~ ERROR E0520
2019-08-07T09:59:36.8306596Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `foo`
2019-08-07T09:59:36.8306644Z    |
2019-08-07T09:59:36.8306684Z    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8306711Z 
2019-08-07T09:59:36.8306939Z error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8307239Z    |
2019-08-07T09:59:36.8307239Z    |
2019-08-07T09:59:36.8307293Z LL | / impl<T> Foo for T {
2019-08-07T09:59:36.8307329Z LL | |     fn foo(&self) {}
2019-08-07T09:59:36.8307364Z LL | |     fn bar(&self) {}
2019-08-07T09:59:36.8307416Z LL | | }
2019-08-07T09:59:36.8307604Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8307642Z ...
2019-08-07T09:59:36.8307678Z LL |       fn bar(&self) {} //~ ERROR E0520
2019-08-07T09:59:36.8307738Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `bar`
2019-08-07T09:59:36.8307773Z    |
2019-08-07T09:59:36.8307811Z    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8308027Z 
2019-08-07T09:59:36.8308070Z error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8309028Z    |
2019-08-07T09:59:36.8309028Z    |
2019-08-07T09:59:36.8309074Z LL | / impl<T> Bar for T {
2019-08-07T09:59:36.8309119Z LL | |     type T = u8;
2019-08-07T09:59:36.8309163Z LL | | }
2019-08-07T09:59:36.8309447Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8309498Z ...
2019-08-07T09:59:36.8309543Z LL |       type T = (); //~ ERROR E0520
2019-08-07T09:59:36.8309611Z    |       ^^^^^^^^^^^^ cannot specialize default item `T`
2019-08-07T09:59:36.8309658Z    |
2019-08-07T09:59:36.8309707Z    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8309741Z 
2019-08-07T09:59:36.8309811Z error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8310279Z    |
2019-08-07T09:59:36.8310279Z    |
2019-08-07T09:59:36.8310345Z LL | / impl<T: Clone> Baz for T {
2019-08-07T09:59:36.8310391Z LL | |     fn baz(&self) {}
2019-08-07T09:59:36.8310445Z LL | | }
2019-08-07T09:59:36.8347050Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8347134Z ...
2019-08-07T09:59:36.8347175Z LL |       fn baz(&self) {} //~ ERROR E0520
2019-08-07T09:59:36.8347236Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `baz`
2019-08-07T09:59:36.8347276Z    |
2019-08-07T09:59:36.8347319Z    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8347347Z 
2019-08-07T09:59:36.8347413Z error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8347907Z    |
2019-08-07T09:59:36.8347907Z    |
2019-08-07T09:59:36.8347970Z LL | / impl<T: Clone> Redundant for T {
2019-08-07T09:59:36.8348011Z LL | |     fn redundant(&self) {}
2019-08-07T09:59:36.8348047Z LL | | }
2019-08-07T09:59:36.8348729Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8348792Z ...
2019-08-07T09:59:36.8348855Z LL |       fn redundant(&self) {} //~ ERROR E0520
2019-08-07T09:59:36.8348913Z    |       ^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `redundant`
2019-08-07T09:59:36.8348975Z    |
2019-08-07T09:59:36.8349026Z    = note: to specialize, `redundant` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8349116Z error: aborting due to 5 previous errors
2019-08-07T09:59:36.8349147Z 
2019-08-07T09:59:36.8349431Z For more information about this error, try `rustc --explain E0520`.
2019-08-07T09:59:36.8349470Z 
2019-08-07T09:59:36.8349470Z 
2019-08-07T09:59:36.8349726Z ------------------------------------------
2019-08-07T09:59:36.8349764Z 
2019-08-07T09:59:36.8349804Z 
2019-08-07T09:59:36.8350071Z ---- [ui] ui/specialization/specialization-no-default.rs stdout ----
2019-08-07T09:59:36.8350139Z diff of stderr:
2019-08-07T09:59:36.8350172Z 
2019-08-07T09:59:36.8350224Z 1 error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8350922Z +   --> $DIR/specialization-no-default.rs:20:5
2019-08-07T09:59:36.8350974Z 3    |
2019-08-07T09:59:36.8350974Z 3    |
2019-08-07T09:59:36.8351020Z 4 LL | / impl<T> Foo for T {
2019-08-07T09:59:36.8351076Z 5 LL | |     fn foo(&self) {}
2019-08-07T09:59:36.8351108Z 
2019-08-07T09:59:36.8351158Z 13    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8351204Z 14 
2019-08-07T09:59:36.8351266Z 15 error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8351944Z +   --> $DIR/specialization-no-default.rs:23:5
2019-08-07T09:59:36.8352178Z 17    |
2019-08-07T09:59:36.8352178Z 17    |
2019-08-07T09:59:36.8352217Z 18 LL | / impl<T> Foo for T {
2019-08-07T09:59:36.8352255Z 19 LL | |     fn foo(&self) {}
2019-08-07T09:59:36.8352281Z 
2019-08-07T09:59:36.8352480Z 27    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8352538Z 28 
2019-08-07T09:59:36.8352583Z 29 error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8353268Z +   --> $DIR/specialization-no-default.rs:37:5
2019-08-07T09:59:36.8353476Z 31    |
2019-08-07T09:59:36.8353476Z 31    |
2019-08-07T09:59:36.8353529Z 32 LL | / impl<T> Bar for T {
2019-08-07T09:59:36.8353566Z 33 LL | |     type T = u8;
2019-08-07T09:59:36.8353589Z 
2019-08-07T09:59:36.8353627Z 40    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8353683Z 41 
2019-08-07T09:59:36.8353724Z 42 error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8354457Z +   --> $DIR/specialization-no-default.rs:55:5
2019-08-07T09:59:36.8354500Z 44    |
2019-08-07T09:59:36.8354500Z 44    |
2019-08-07T09:59:36.8354545Z 45 LL | / impl<T: Clone> Baz for T {
2019-08-07T09:59:36.8354597Z 46 LL | |     fn baz(&self) {}
2019-08-07T09:59:36.8354622Z 
2019-08-07T09:59:36.8354661Z 53    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8354696Z 54 
2019-08-07T09:59:36.8354751Z 55 error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8355163Z +   --> $DIR/specialization-no-default.rs:74:5
2019-08-07T09:59:36.8355222Z 57    |
2019-08-07T09:59:36.8355222Z 57    |
2019-08-07T09:59:36.8355260Z 58 LL | / impl<T: Clone> Redundant for T {
2019-08-07T09:59:36.8355297Z 59 LL | |     fn redundant(&self) {}
2019-08-07T09:59:36.8355370Z 
2019-08-07T09:59:36.8355409Z The actual stderr differed from the expected stderr.
2019-08-07T09:59:36.8355701Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/specialization-no-default.stderr
2019-08-07T09:59:36.8355701Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/specialization-no-default.stderr
2019-08-07T09:59:36.8355955Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T09:59:36.8356381Z To only update this specific test, also pass `--test-args specialization/specialization-no-default.rs`
2019-08-07T09:59:36.8356468Z error: 1 errors occurred comparing output.
2019-08-07T09:59:36.8356504Z status: exit code: 1
2019-08-07T09:59:36.8356504Z status: exit code: 1
2019-08-07T09:59:36.8357134Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/specialization-no-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/auxiliary" "-A" "unused"
2019-08-07T09:59:36.8357979Z ------------------------------------------
2019-08-07T09:59:36.8358037Z 
2019-08-07T09:59:36.8358713Z ------------------------------------------
2019-08-07T09:59:36.8358773Z stderr:
2019-08-07T09:59:36.8358773Z stderr:
2019-08-07T09:59:36.8359014Z ------------------------------------------
2019-08-07T09:59:36.8359106Z error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8359459Z    |
2019-08-07T09:59:36.8359459Z    |
2019-08-07T09:59:36.8359524Z LL | / impl<T> Foo for T {
2019-08-07T09:59:36.8359570Z LL | |     fn foo(&self) {}
2019-08-07T09:59:36.8359630Z LL | |     fn bar(&self) {}
2019-08-07T09:59:36.8359689Z LL | | }
2019-08-07T09:59:36.8359930Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8359984Z ...
2019-08-07T09:59:36.8360050Z LL |       fn foo(&self) {} //~ ERROR E0520
2019-08-07T09:59:36.8360224Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `foo`
2019-08-07T09:59:36.8360286Z    |
2019-08-07T09:59:36.8360336Z    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8360384Z 
2019-08-07T09:59:36.8360436Z error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8360840Z    |
2019-08-07T09:59:36.8360840Z    |
2019-08-07T09:59:36.8360885Z LL | / impl<T> Foo for T {
2019-08-07T09:59:36.8360931Z LL | |     fn foo(&self) {}
2019-08-07T09:59:36.8360994Z LL | |     fn bar(&self) {}
2019-08-07T09:59:36.8361040Z LL | | }
2019-08-07T09:59:36.8361402Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8361454Z ...
2019-08-07T09:59:36.8361522Z LL |       fn bar(&self) {} //~ ERROR E0520
2019-08-07T09:59:36.8361745Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `bar`
2019-08-07T09:59:36.8361952Z    |
2019-08-07T09:59:36.8362030Z    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8362059Z 
2019-08-07T09:59:36.8362100Z error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8362406Z    |
2019-08-07T09:59:36.8362406Z    |
2019-08-07T09:59:36.8362443Z LL | / impl<T> Bar for T {
2019-08-07T09:59:36.8362480Z LL | |     type T = u8;
2019-08-07T09:59:36.8362533Z LL | | }
2019-08-07T09:59:36.8362727Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8362768Z ...
2019-08-07T09:59:36.8362819Z LL |       type T = (); //~ ERROR E0520
2019-08-07T09:59:36.8362872Z    |       ^^^^^^^^^^^^ cannot specialize default item `T`
2019-08-07T09:59:36.8362908Z    |
2019-08-07T09:59:36.8362947Z    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8362985Z 
2019-08-07T09:59:36.8363032Z error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8363337Z    |
2019-08-07T09:59:36.8363337Z    |
2019-08-07T09:59:36.8363374Z LL | / impl<T: Clone> Baz for T {
2019-08-07T09:59:36.8363411Z LL | |     fn baz(&self) {}
2019-08-07T09:59:36.8363466Z LL | | }
2019-08-07T09:59:36.8363657Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8363699Z ...
2019-08-07T09:59:36.8363736Z LL |       fn baz(&self) {} //~ ERROR E0520
2019-08-07T09:59:36.8363795Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `baz`
2019-08-07T09:59:36.8363832Z    |
2019-08-07T09:59:36.8363871Z    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8363927Z 
2019-08-07T09:59:36.8363970Z error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-07T09:59:36.8364278Z    |
2019-08-07T09:59:36.8364278Z    |
2019-08-07T09:59:36.8364318Z LL | / impl<T: Clone> Redundant for T {
2019-08-07T09:59:36.8364356Z LL | |     fn redundant(&self) {}
2019-08-07T09:59:36.8364392Z LL | | }
2019-08-07T09:59:36.8364602Z    | |_- parent `impl` is here
2019-08-07T09:59:36.8364644Z ...
2019-08-07T09:59:36.8364681Z LL |       default fn redundant(&self) {} //~ ERROR E0520
2019-08-07T09:59:36.8364745Z    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `redundant`
2019-08-07T09:59:36.8364784Z    |
2019-08-07T09:59:36.8364824Z    = note: to specialize, `redundant` in the parent `impl` must be marked `default`
2019-08-07T09:59:36.8364903Z error: aborting due to 5 previous errors
2019-08-07T09:59:36.8364938Z 
2019-08-07T09:59:36.8365166Z For more information about this error, try `rustc --explain E0520`.
2019-08-07T09:59:36.8365198Z 
---
2019-08-07T09:59:36.8370769Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-07T09:59:36.8370847Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-07T09:59:36.8375227Z 
2019-08-07T09:59:36.8375804Z 
2019-08-07T09:59:36.8377451Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-07T09:59:36.8379068Z 
2019-08-07T09:59:36.8379130Z 
2019-08-07T09:59:36.8379183Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-07T09:59:36.8379236Z Build completed unsuccessfully in 1:05:26
2019-08-07T09:59:36.8379236Z Build completed unsuccessfully in 1:05:26
2019-08-07T09:59:37.5843601Z ##[error]Bash exited with code '1'.
2019-08-07T09:59:37.5900703Z ##[section]Starting: Checkout
2019-08-07T09:59:37.5902814Z ==============================================================================
2019-08-07T09:59:37.5902881Z Task         : Get sources
2019-08-07T09:59:37.5902927Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
