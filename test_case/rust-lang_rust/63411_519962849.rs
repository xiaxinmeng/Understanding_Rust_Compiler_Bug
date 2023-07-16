plain
2019-08-09T14:16:10.7956135Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T14:16:10.8192159Z ##[command]git config gc.auto 0
2019-08-09T14:16:10.8272921Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T14:16:10.8317845Z ##[command]git config --get-all http.proxy
2019-08-09T14:16:10.8460245Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63411/merge:refs/remotes/pull/63411/merge
---
2019-08-09T14:16:47.2789708Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T14:16:47.2789736Z 
2019-08-09T14:16:47.2789931Z   git checkout -b <new-branch-name>
2019-08-09T14:16:47.2789955Z 
2019-08-09T14:16:47.2789995Z HEAD is now at 104eafe9b Merge b81e887b24c6754df44485b82fefe92ad32b1a46 into 813a3a5d4b2be4e2101faa73a067da02a704a598
2019-08-09T14:16:47.3005290Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T14:16:47.3008623Z ==============================================================================
2019-08-09T14:16:47.3008673Z Task         : Bash
2019-08-09T14:16:47.3008710Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T15:19:12.8136478Z .................................................................................................... 1300/8853
2019-08-09T15:19:19.3844264Z .................................................................................................... 1400/8853
2019-08-09T15:19:25.8520221Z .................................................................................................... 1500/8853
2019-08-09T15:19:36.9006653Z ....................................................................................i............... 1600/8853
2019-08-09T15:19:44.6678705Z i................................................................................................... 1700/8853
2019-08-09T15:19:51.8688953Z ......................................................................iiiii......................... 1800/8853
2019-08-09T15:20:14.3067283Z .................................................................................................... 2000/8853
2019-08-09T15:20:16.8204582Z .................................................................................................... 2100/8853
2019-08-09T15:20:19.7875190Z .................................................................................................... 2200/8853
2019-08-09T15:20:27.9334815Z .................................................................................................... 2300/8853
---
2019-08-09T15:24:17.9091397Z .................................................................................................... 5200/8853
2019-08-09T15:24:28.9619005Z ..............................................................................................i..... 5300/8853
2019-08-09T15:24:37.1963315Z .................................................................................................... 5400/8853
2019-08-09T15:24:41.9643739Z .................................................................................................... 5500/8853
2019-08-09T15:24:53.5309507Z ........................................................................................ii...i..ii.. 5600/8853
2019-08-09T15:25:16.9773405Z .................................................................................................... 5800/8853
2019-08-09T15:25:22.2644764Z .................................................................................................... 5900/8853
2019-08-09T15:25:22.2644764Z .................................................................................................... 5900/8853
2019-08-09T15:25:26.9886924Z .........................................................................................i..ii...... 6000/8853
2019-08-09T15:25:57.8892218Z .................................................................................................... 6200/8853
2019-08-09T15:25:59.8947566Z ................................i................................................................... 6300/8853
2019-08-09T15:26:02.0422487Z .................................................................................................... 6400/8853
2019-08-09T15:26:04.5786504Z ....i............................................................................................... 6500/8853
2019-08-09T15:26:04.5786504Z ....i............................................................................................... 6500/8853
2019-08-09T15:26:09.3227877Z .................................................................................................... 6600/8853
2019-08-09T15:26:29.6769145Z .................................................................................................... 6700/8853
2019-08-09T15:26:48.4958124Z .................................................................................................... 6800/8853
2019-08-09T15:26:54.0683406Z .................................................................................................... 6900/8853
2019-08-09T15:26:59.4606612Z ...................................................................F.FFFF........................... 7000/8853
2019-08-09T15:27:11.0326808Z .................................................................................................... 7200/8853
2019-08-09T15:27:20.3408591Z .................................................................................................... 7300/8853
2019-08-09T15:27:29.5397577Z .................................................................................................... 7400/8853
2019-08-09T15:27:39.8051412Z ................................ii......i........................................................... 7500/8853
---
2019-08-09T15:30:08.2006909Z 1 error[E0308]: mismatched types
2019-08-09T15:30:08.2007411Z -   --> $DIR/associated-type-projection-from-supertrait.rs:33:23
2019-08-09T15:30:08.2008341Z +   --> $DIR/associated-type-projection-from-supertrait.rs:27:23
2019-08-09T15:30:08.2008683Z 3    |
2019-08-09T15:30:08.2008917Z 4 LL | fn b() { dent(ModelT, Blue); }
2019-08-09T15:30:08.2009172Z 5    |                       ^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:30:08.2009603Z 8               found type `Blue`
2019-08-09T15:30:08.2009840Z 9 
2019-08-09T15:30:08.2010081Z 10 error[E0308]: mismatched types
2019-08-09T15:30:08.2010567Z -   --> $DIR/associated-type-projection-from-supertrait.rs:34:23
2019-08-09T15:30:08.2010567Z -   --> $DIR/associated-type-projection-from-supertrait.rs:34:23
2019-08-09T15:30:08.2011112Z +   --> $DIR/associated-type-projection-from-supertrait.rs:28:23
2019-08-09T15:30:08.2011713Z 12    |
2019-08-09T15:30:08.2011901Z 13 LL | fn c() { dent(ModelU, Black); }
2019-08-09T15:30:08.2012129Z 14    |                       ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:30:08.2012488Z 17               found type `Black`
2019-08-09T15:30:08.2012686Z 18 
2019-08-09T15:30:08.2012866Z 19 error[E0308]: mismatched types
2019-08-09T15:30:08.2013260Z -   --> $DIR/associated-type-projection-from-supertrait.rs:40:28
2019-08-09T15:30:08.2013260Z -   --> $DIR/associated-type-projection-from-supertrait.rs:40:28
2019-08-09T15:30:08.2013715Z +   --> $DIR/associated-type-projection-from-supertrait.rs:32:28
2019-08-09T15:30:08.2013948Z 21    |
2019-08-09T15:30:08.2014134Z 22 LL | fn f() { ModelT.chip_paint(Blue); }
2019-08-09T15:30:08.2014347Z 23    |                            ^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:30:08.2014847Z 26               found type `Blue`
2019-08-09T15:30:08.2015041Z 27 
2019-08-09T15:30:08.2015400Z 28 error[E0308]: mismatched types
2019-08-09T15:30:08.2015810Z -   --> $DIR/associated-type-projection-from-supertrait.rs:41:28
2019-08-09T15:30:08.2015810Z -   --> $DIR/associated-type-projection-from-supertrait.rs:41:28
2019-08-09T15:30:08.2030983Z +   --> $DIR/associated-type-projection-from-supertrait.rs:33:28
2019-08-09T15:30:08.2037800Z 30    |
2019-08-09T15:30:08.2038701Z 31 LL | fn g() { ModelU.chip_paint(Black); }
2019-08-09T15:30:08.2039271Z 32    |                            ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:30:08.2039684Z 
2019-08-09T15:30:08.2039935Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2040660Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
2019-08-09T15:30:08.2040660Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
2019-08-09T15:30:08.2041244Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2042021Z To only update this specific test, also pass `--test-args associated-type/associated-type-projection-from-supertrait.rs`
2019-08-09T15:30:08.2042471Z error: 1 errors occurred comparing output.
2019-08-09T15:30:08.2042656Z status: exit code: 1
2019-08-09T15:30:08.2042656Z status: exit code: 1
2019-08-09T15:30:08.2043564Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2044257Z ------------------------------------------
2019-08-09T15:30:08.2044489Z 
2019-08-09T15:30:08.2045072Z ------------------------------------------
2019-08-09T15:30:08.2045342Z stderr:
2019-08-09T15:30:08.2045342Z stderr:
2019-08-09T15:30:08.2045724Z ------------------------------------------
2019-08-09T15:30:08.2045963Z error[E0308]: mismatched types
2019-08-09T15:30:08.2046567Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:27:23
2019-08-09T15:30:08.2046856Z    |
2019-08-09T15:30:08.2047076Z LL | fn b() { dent(ModelT, Blue); } //~ ERROR mismatched types
2019-08-09T15:30:08.2047279Z    |                       ^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:30:08.2048309Z    = note: expected type `Black`
2019-08-09T15:30:08.2048865Z               found type `Blue`
2019-08-09T15:30:08.2049798Z 
2019-08-09T15:30:08.2050791Z error[E0308]: mismatched types
2019-08-09T15:30:08.2050791Z error[E0308]: mismatched types
2019-08-09T15:30:08.2051639Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:28:23
2019-08-09T15:30:08.2052068Z    |
2019-08-09T15:30:08.2052300Z LL | fn c() { dent(ModelU, Black); } //~ ERROR mismatched types
2019-08-09T15:30:08.2052495Z    |                       ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:30:08.2052891Z    = note: expected type `Blue`
2019-08-09T15:30:08.2053077Z               found type `Black`
2019-08-09T15:30:08.2053242Z 
2019-08-09T15:30:08.2053442Z error[E0308]: mismatched types
2019-08-09T15:30:08.2053442Z error[E0308]: mismatched types
2019-08-09T15:30:08.2053870Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:32:28
2019-08-09T15:30:08.2054107Z    |
2019-08-09T15:30:08.2054319Z LL | fn f() { ModelT.chip_paint(Blue); } //~ ERROR mismatched types
2019-08-09T15:30:08.2054514Z    |                            ^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:30:08.2054896Z    = note: expected type `Black`
2019-08-09T15:30:08.2055079Z               found type `Blue`
2019-08-09T15:30:08.2055247Z 
2019-08-09T15:30:08.2055444Z error[E0308]: mismatched types
2019-08-09T15:30:08.2055444Z error[E0308]: mismatched types
2019-08-09T15:30:08.2056274Z   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:33:28
2019-08-09T15:30:08.2056521Z    |
2019-08-09T15:30:08.2056754Z LL | fn g() { ModelU.chip_paint(Black); } //~ ERROR mismatched types
2019-08-09T15:30:08.2056962Z    |                            ^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:30:08.2058250Z    = note: expected type `Blue`
2019-08-09T15:30:08.2058572Z               found type `Black`
2019-08-09T15:30:08.2058729Z 
2019-08-09T15:30:08.2058891Z error: aborting due to 4 previous errors
---
2019-08-09T15:30:08.2060493Z 
2019-08-09T15:30:08.2061089Z ---- [ui] ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs stdout ----
2019-08-09T15:30:08.2061463Z diff of stderr:
2019-08-09T15:30:08.2061579Z 
2019-08-09T15:30:08.2062239Z 1 error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
2019-08-09T15:30:08.2063656Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:31:10
2019-08-09T15:30:08.2063708Z 3    |
2019-08-09T15:30:08.2063708Z 3    |
2019-08-09T15:30:08.2063749Z 4 LL | fn b() { blue_car(ModelT); }
2019-08-09T15:30:08.2064021Z 5    |          ^^^^^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:30:08.2064110Z 7    = note: expected type `Black`
2019-08-09T15:30:08.2064150Z 8               found type `Blue`
2019-08-09T15:30:08.2064150Z 8               found type `Blue`
2019-08-09T15:30:08.2064188Z 9 note: required by `blue_car`
2019-08-09T15:30:08.2064704Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:27:1
2019-08-09T15:30:08.2064757Z 11    |
2019-08-09T15:30:08.2064757Z 11    |
2019-08-09T15:30:08.2064814Z 12 LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
2019-08-09T15:30:08.2064881Z 
2019-08-09T15:30:08.2065076Z 14 
2019-08-09T15:30:08.2065076Z 14 
2019-08-09T15:30:08.2065150Z 15 error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
2019-08-09T15:30:08.2065675Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:32:10
2019-08-09T15:30:08.2065740Z 17    |
2019-08-09T15:30:08.2065740Z 17    |
2019-08-09T15:30:08.2065776Z 18 LL | fn c() { black_car(ModelU); }
2019-08-09T15:30:08.2065819Z 19    |          ^^^^^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:30:08.2065905Z 21    = note: expected type `Blue`
2019-08-09T15:30:08.2065944Z 22               found type `Black`
2019-08-09T15:30:08.2065944Z 22               found type `Black`
2019-08-09T15:30:08.2065982Z 23 note: required by `black_car`
2019-08-09T15:30:08.2066481Z +   --> $DIR/associated-types-binding-to-type-defined-in-supertrait.rs:24:1
2019-08-09T15:30:08.2066522Z 25    |
2019-08-09T15:30:08.2066522Z 25    |
2019-08-09T15:30:08.2066768Z 26 LL | fn black_car<C:Car<Color=Black>>(c: C) {
2019-08-09T15:30:08.2066835Z 
2019-08-09T15:30:08.2066857Z 
2019-08-09T15:30:08.2066915Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2067263Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
2019-08-09T15:30:08.2067263Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
2019-08-09T15:30:08.2067944Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2068333Z To only update this specific test, also pass `--test-args associated-types/associated-types-binding-to-type-defined-in-supertrait.rs`
2019-08-09T15:30:08.2068567Z error: 1 errors occurred comparing output.
2019-08-09T15:30:08.2068638Z status: exit code: 1
2019-08-09T15:30:08.2068638Z status: exit code: 1
2019-08-09T15:30:08.2069577Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2069953Z ------------------------------------------
2019-08-09T15:30:08.2069997Z 
2019-08-09T15:30:08.2070274Z ------------------------------------------
2019-08-09T15:30:08.2070322Z stderr:
2019-08-09T15:30:08.2070322Z stderr:
2019-08-09T15:30:08.2070561Z ------------------------------------------
2019-08-09T15:30:08.2070646Z error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
2019-08-09T15:30:08.2071029Z    |
2019-08-09T15:30:08.2071029Z    |
2019-08-09T15:30:08.2071100Z LL | fn b() { blue_car(ModelT); } //~ ERROR type mismatch
2019-08-09T15:30:08.2071319Z    |          ^^^^^^^^ expected struct `Black`, found struct `Blue`
2019-08-09T15:30:08.2071572Z    = note: expected type `Black`
2019-08-09T15:30:08.2071607Z               found type `Blue`
2019-08-09T15:30:08.2071607Z               found type `Blue`
2019-08-09T15:30:08.2071642Z note: required by `blue_car`
2019-08-09T15:30:08.2072131Z    |
2019-08-09T15:30:08.2072131Z    |
2019-08-09T15:30:08.2072167Z LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
2019-08-09T15:30:08.2072247Z 
2019-08-09T15:30:08.2072247Z 
2019-08-09T15:30:08.2072362Z error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
2019-08-09T15:30:08.2072698Z    |
2019-08-09T15:30:08.2072698Z    |
2019-08-09T15:30:08.2072736Z LL | fn c() { black_car(ModelU); } //~ ERROR type mismatch
2019-08-09T15:30:08.2072778Z    |          ^^^^^^^^^ expected struct `Blue`, found struct `Black`
2019-08-09T15:30:08.2072866Z    = note: expected type `Blue`
2019-08-09T15:30:08.2072901Z               found type `Black`
2019-08-09T15:30:08.2072901Z               found type `Black`
2019-08-09T15:30:08.2072955Z note: required by `black_car`
2019-08-09T15:30:08.2073243Z    |
2019-08-09T15:30:08.2073243Z    |
2019-08-09T15:30:08.2073297Z LL | fn black_car<C:Car<Color=Black>>(c: C) {
2019-08-09T15:30:08.2073361Z 
2019-08-09T15:30:08.2073402Z error: aborting due to 2 previous errors
2019-08-09T15:30:08.2073443Z 
2019-08-09T15:30:08.2073656Z For more information about this error, try `rustc --explain E0271`.
2019-08-09T15:30:08.2073656Z For more information about this error, try `rustc --explain E0271`.
2019-08-09T15:30:08.2073685Z 
2019-08-09T15:30:08.2073870Z ------------------------------------------
2019-08-09T15:30:08.2073914Z 
2019-08-09T15:30:08.2073934Z 
2019-08-09T15:30:08.2074129Z ---- [ui] ui/hrtb/hrtb-conflate-regions.rs stdout ----
2019-08-09T15:30:08.2074169Z diff of stderr:
2019-08-09T15:30:08.2074191Z 
2019-08-09T15:30:08.2074446Z 1 error[E0277]: the trait bound `for<'a, 'b> SomeStruct: Foo<(&'a isize, &'b isize)>` is not satisfied
2019-08-09T15:30:08.2074638Z -   --> $DIR/hrtb-conflate-regions.rs:28:10
2019-08-09T15:30:08.2074825Z +   --> $DIR/hrtb-conflate-regions.rs:27:10
2019-08-09T15:30:08.2074956Z 3    |
2019-08-09T15:30:08.2074993Z 4 LL | fn b() { want_foo2::<SomeStruct>(); }
2019-08-09T15:30:08.2075268Z 5    |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a, 'b> Foo<(&'a isize, &'b isize)>` is not implemented for `SomeStruct`
2019-08-09T15:30:08.2075349Z 
2019-08-09T15:30:08.2075386Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2075647Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/hrtb-conflate-regions.stderr
2019-08-09T15:30:08.2075647Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/hrtb-conflate-regions.stderr
2019-08-09T15:30:08.2075878Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2076102Z To only update this specific test, also pass `--test-args hrtb/hrtb-conflate-regions.rs`
2019-08-09T15:30:08.2076185Z error: 1 errors occurred comparing output.
2019-08-09T15:30:08.2076222Z status: exit code: 1
2019-08-09T15:30:08.2076222Z status: exit code: 1
2019-08-09T15:30:08.2076834Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/hrtb-conflate-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-conflate-regions/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2077123Z ------------------------------------------
2019-08-09T15:30:08.2077168Z 
2019-08-09T15:30:08.2077352Z ------------------------------------------
2019-08-09T15:30:08.2077903Z stderr:
2019-08-09T15:30:08.2077903Z stderr:
2019-08-09T15:30:08.2078176Z ------------------------------------------
2019-08-09T15:30:08.2078492Z error[E0277]: the trait bound `for<'a, 'b> SomeStruct: Foo<(&'a isize, &'b isize)>` is not satisfied
2019-08-09T15:30:08.2078814Z    |
2019-08-09T15:30:08.2078814Z    |
2019-08-09T15:30:08.2078878Z LL | fn b() { want_foo2::<SomeStruct>(); } //~ ERROR
2019-08-09T15:30:08.2079289Z    |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a, 'b> Foo<(&'a isize, &'b isize)>` is not implemented for `SomeStruct`
2019-08-09T15:30:08.2079418Z    = help: the following implementations were found:
2019-08-09T15:30:08.2079418Z    = help: the following implementations were found:
2019-08-09T15:30:08.2079686Z              <SomeStruct as Foo<(&'a isize, &'a isize)>>
2019-08-09T15:30:08.2079735Z note: required by `want_foo2`
2019-08-09T15:30:08.2080046Z    |
2019-08-09T15:30:08.2080046Z    |
2019-08-09T15:30:08.2080087Z LL | / fn want_foo2<T>()
2019-08-09T15:30:08.2080341Z LL | |     where T : for<'a,'b> Foo<(&'a isize, &'b isize)>
2019-08-09T15:30:08.2080390Z LL | | {
2019-08-09T15:30:08.2080467Z    | |_^
2019-08-09T15:30:08.2080515Z 
2019-08-09T15:30:08.2080558Z error: aborting due to previous error
2019-08-09T15:30:08.2080597Z 
---
2019-08-09T15:30:08.2081576Z diff of stderr:
2019-08-09T15:30:08.2081599Z 
2019-08-09T15:30:08.2081636Z 7    = note: `#[warn(incomplete_features)]` on by default
2019-08-09T15:30:08.2081690Z 8 
2019-08-09T15:30:08.2081732Z 9 error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
2019-08-09T15:30:08.2082133Z +   --> $DIR/bound-normalization-fail.rs:28:32
2019-08-09T15:30:08.2082170Z 11    |
2019-08-09T15:30:08.2082170Z 11    |
2019-08-09T15:30:08.2082371Z 12 LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
2019-08-09T15:30:08.2082513Z 13    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-09T15:30:08.2082542Z 
2019-08-09T15:30:08.2082581Z 17    = note: the return type of a function must have a statically known size
2019-08-09T15:30:08.2082625Z 18 
2019-08-09T15:30:08.2082909Z 19 error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
2019-08-09T15:30:08.2083285Z +   --> $DIR/bound-normalization-fail.rs:44:41
2019-08-09T15:30:08.2083341Z 21    |
2019-08-09T15:30:08.2083341Z 21    |
2019-08-09T15:30:08.2083551Z 22 LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
2019-08-09T15:30:08.2083601Z 23    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-09T15:30:08.2083669Z 
2019-08-09T15:30:08.2083705Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2083977Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
2019-08-09T15:30:08.2083977Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
2019-08-09T15:30:08.2084257Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2084516Z To only update this specific test, also pass `--test-args impl-trait/bound-normalization-fail.rs`
2019-08-09T15:30:08.2084604Z error: 1 errors occurred comparing output.
2019-08-09T15:30:08.2084643Z status: exit code: 1
2019-08-09T15:30:08.2084643Z status: exit code: 1
2019-08-09T15:30:08.2085463Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bound-normalization-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2085744Z ------------------------------------------
2019-08-09T15:30:08.2085853Z 
2019-08-09T15:30:08.2086062Z ------------------------------------------
2019-08-09T15:30:08.2086098Z stderr:
---
2019-08-09T15:30:08.2086678Z    |            ^^^^^^^^^^^^^^^^^^^^^^
2019-08-09T15:30:08.2086728Z    |
2019-08-09T15:30:08.2086764Z    = note: `#[warn(incomplete_features)]` on by default
2019-08-09T15:30:08.2086796Z 
2019-08-09T15:30:08.2086837Z error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
2019-08-09T15:30:08.2087106Z    |
2019-08-09T15:30:08.2087106Z    |
2019-08-09T15:30:08.2087304Z LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
2019-08-09T15:30:08.2087824Z    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-09T15:30:08.2087926Z    = note: expected type `()`
2019-08-09T15:30:08.2087926Z    = note: expected type `()`
2019-08-09T15:30:08.2087993Z               found type `<T as impl_trait::Trait>::Assoc`
2019-08-09T15:30:08.2088045Z    = note: the return type of a function must have a statically known size
2019-08-09T15:30:08.2088077Z 
2019-08-09T15:30:08.2088419Z error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
2019-08-09T15:30:08.2088857Z    |
2019-08-09T15:30:08.2088857Z    |
2019-08-09T15:30:08.2089135Z LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
2019-08-09T15:30:08.2089228Z    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
2019-08-09T15:30:08.2089317Z    = note: expected type `()`
2019-08-09T15:30:08.2089317Z    = note: expected type `()`
2019-08-09T15:30:08.2089586Z               found type `<T as lifetimes::Trait<'static>>::Assoc`
2019-08-09T15:30:08.2089642Z    = note: the return type of a function must have a statically known size
2019-08-09T15:30:08.2089735Z error: aborting due to 2 previous errors
2019-08-09T15:30:08.2089765Z 
2019-08-09T15:30:08.2090008Z For more information about this error, try `rustc --explain E0271`.
2019-08-09T15:30:08.2090041Z 
2019-08-09T15:30:08.2090041Z 
2019-08-09T15:30:08.2090273Z ------------------------------------------
2019-08-09T15:30:08.2090305Z 
2019-08-09T15:30:08.2090331Z 
2019-08-09T15:30:08.2090566Z ---- [ui] ui/issues/issue-12028.rs stdout ----
2019-08-09T15:30:08.2090613Z diff of stderr:
2019-08-09T15:30:08.2090663Z 
2019-08-09T15:30:08.2090713Z 1 error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2019-08-09T15:30:08.2090943Z -   --> $DIR/issue-12028.rs:29:14
2019-08-09T15:30:08.2091352Z +   --> $DIR/issue-12028.rs:27:14
2019-08-09T15:30:08.2091426Z 4 LL |         self.input_stream(&mut stream);
2019-08-09T15:30:08.2091481Z 5    |              ^^^^^^^^^^^^
2019-08-09T15:30:08.2091506Z 
2019-08-09T15:30:08.2091526Z 
2019-08-09T15:30:08.2091526Z 
2019-08-09T15:30:08.2091563Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2092002Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/issue-12028.stderr
2019-08-09T15:30:08.2092208Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2092584Z To only update this specific test, also pass `--test-args issues/issue-12028.rs`
2019-08-09T15:30:08.2092849Z error: 1 errors occurred comparing output.
2019-08-09T15:30:08.2092884Z status: exit code: 1
2019-08-09T15:30:08.2092884Z status: exit code: 1
2019-08-09T15:30:08.2093543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12028.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2093851Z ------------------------------------------
2019-08-09T15:30:08.2093897Z 
2019-08-09T15:30:08.2094078Z ------------------------------------------
2019-08-09T15:30:08.2094114Z stderr:
2019-08-09T15:30:08.2094114Z stderr:
2019-08-09T15:30:08.2094319Z ------------------------------------------
2019-08-09T15:30:08.2094365Z error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2019-08-09T15:30:08.2094572Z   --> /checkout/src/test/ui/issues/issue-12028.rs:27:14
2019-08-09T15:30:08.2094632Z    |
2019-08-09T15:30:08.2094672Z LL |         self.input_stream(&mut stream); //~ ERROR type annotations required
2019-08-09T15:30:08.2094734Z 
2019-08-09T15:30:08.2094786Z error: aborting due to previous error
2019-08-09T15:30:08.2094809Z 
2019-08-09T15:30:08.2095014Z For more information about this error, try `rustc --explain E0284`.
2019-08-09T15:30:08.2095014Z For more information about this error, try `rustc --explain E0284`.
2019-08-09T15:30:08.2095041Z 
2019-08-09T15:30:08.2095237Z ------------------------------------------
2019-08-09T15:30:08.2095262Z 
2019-08-09T15:30:08.2095283Z 
2019-08-09T15:30:08.2095494Z ---- [ui] ui/regions/regions-outlives-projection-container-hrtb.rs#migrate stdout ----
2019-08-09T15:30:08.2095554Z diff of stderr:
2019-08-09T15:30:08.2095645Z 
2019-08-09T15:30:08.2096069Z 1 error[E0491]: in type `&'a WithHrAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2096493Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-09T15:30:08.2096531Z 3    |
2019-08-09T15:30:08.2096531Z 3    |
2019-08-09T15:30:08.2096717Z 4 LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2096802Z 
2019-08-09T15:30:08.2096832Z 6    |
2019-08-09T15:30:08.2096832Z 6    |
2019-08-09T15:30:08.2097059Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 32:15
2019-08-09T15:30:08.2097256Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:32:15
2019-08-09T15:30:08.2097466Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-09T15:30:08.2097678Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:27:15
2019-08-09T15:30:08.2097723Z 9    |
2019-08-09T15:30:08.2097891Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2097969Z 
2019-08-09T15:30:08.2097969Z 
2019-08-09T15:30:08.2098677Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 32:18
2019-08-09T15:30:08.2098939Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:32:18
2019-08-09T15:30:08.2099251Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-09T15:30:08.2099502Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:27:18
2019-08-09T15:30:08.2099549Z 14    |
2019-08-09T15:30:08.2099776Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2099854Z 
2019-08-09T15:30:08.2099891Z 17 
2019-08-09T15:30:08.2099891Z 17 
2019-08-09T15:30:08.2100204Z 18 error[E0491]: in type `&'a WithHrAssocSub<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2100708Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-09T15:30:08.2100777Z 20    |
2019-08-09T15:30:08.2100777Z 20    |
2019-08-09T15:30:08.2101112Z 21 LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-09T15:30:08.2101207Z 
2019-08-09T15:30:08.2101267Z 23    |
2019-08-09T15:30:08.2101267Z 23    |
2019-08-09T15:30:08.2101564Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 53:19
2019-08-09T15:30:08.2102433Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:53:19
2019-08-09T15:30:08.2102694Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 46:19
2019-08-09T15:30:08.2102895Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:46:19
2019-08-09T15:30:08.2102933Z 26    |
2019-08-09T15:30:08.2103128Z 27 LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:30:08.2103200Z 
2019-08-09T15:30:08.2103200Z 
2019-08-09T15:30:08.2103431Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 53:22
2019-08-09T15:30:08.2103850Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:53:22
2019-08-09T15:30:08.2104096Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 46:22
2019-08-09T15:30:08.2104303Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:46:22
2019-08-09T15:30:08.2104361Z 31    |
2019-08-09T15:30:08.2104540Z 32 LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:30:08.2104623Z 
2019-08-09T15:30:08.2104644Z 
2019-08-09T15:30:08.2104682Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2104682Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2104995Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate/regions-outlives-projection-container-hrtb.migrate.stderr
2019-08-09T15:30:08.2105368Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2105625Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-09T15:30:08.2105657Z 
2019-08-09T15:30:08.2105714Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-09T15:30:08.2105752Z status: exit code: 1
2019-08-09T15:30:08.2106478Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2106779Z ------------------------------------------
2019-08-09T15:30:08.2106815Z 
2019-08-09T15:30:08.2107001Z ------------------------------------------
2019-08-09T15:30:08.2107038Z stderr:
2019-08-09T15:30:08.2107038Z stderr:
2019-08-09T15:30:08.2107241Z ------------------------------------------
2019-08-09T15:30:08.2108003Z error[E0491]: in type `&'a WithHrAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2108402Z    |
2019-08-09T15:30:08.2108402Z    |
2019-08-09T15:30:08.2108639Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2108751Z    |
2019-08-09T15:30:08.2108751Z    |
2019-08-09T15:30:08.2109014Z note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-09T15:30:08.2109371Z    |
2019-08-09T15:30:08.2109371Z    |
2019-08-09T15:30:08.2109576Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2109730Z    |               ^^
2019-08-09T15:30:08.2110084Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-09T15:30:08.2110408Z    |
2019-08-09T15:30:08.2110408Z    |
2019-08-09T15:30:08.2110630Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2110705Z 
2019-08-09T15:30:08.2110705Z 
2019-08-09T15:30:08.2111307Z error[E0491]: in type `&'a WithHrAssocSub<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2111589Z    |
2019-08-09T15:30:08.2111589Z    |
2019-08-09T15:30:08.2111791Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-09T15:30:08.2111887Z    |
2019-08-09T15:30:08.2111887Z    |
2019-08-09T15:30:08.2112108Z note: the pointer is valid for the lifetime 'a as defined on the function body at 46:19
2019-08-09T15:30:08.2112388Z    |
2019-08-09T15:30:08.2112388Z    |
2019-08-09T15:30:08.2112560Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:30:08.2112616Z    |                   ^^
2019-08-09T15:30:08.2115751Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 46:22
2019-08-09T15:30:08.2116072Z    |
2019-08-09T15:30:08.2116072Z    |
2019-08-09T15:30:08.2116246Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:30:08.2116450Z 
2019-08-09T15:30:08.2116507Z error: aborting due to 2 previous errors
2019-08-09T15:30:08.2116531Z 
2019-08-09T15:30:08.2116766Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-09T15:30:08.2117366Z 1 error: lifetime may not live long enough
2019-08-09T15:30:08.2117560Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:35:12
2019-08-09T15:30:08.2117838Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-09T15:30:08.2118429Z 3    |
2019-08-09T15:30:08.2118702Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2118960Z 5    |               -- -- lifetime `'b` defined here
2019-08-09T15:30:08.2119283Z 10    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:30:08.2119334Z 11 
2019-08-09T15:30:08.2119399Z 12 error: lifetime may not live long enough
2019-08-09T15:30:08.2119650Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-09T15:30:08.2119650Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-09T15:30:08.2119897Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-09T15:30:08.2119959Z 14    |
2019-08-09T15:30:08.2120174Z 15 LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:30:08.2143503Z 16    |                   -- -- lifetime `'b` defined here
2019-08-09T15:30:08.2143593Z 
2019-08-09T15:30:08.2143638Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2143638Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2144333Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll/regions-outlives-projection-container-hrtb.nll.stderr
2019-08-09T15:30:08.2144573Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2145026Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-09T15:30:08.2145074Z 
2019-08-09T15:30:08.2145254Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-09T15:30:08.2145307Z status: exit code: 1
2019-08-09T15:30:08.2146333Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.nll/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2146663Z ------------------------------------------
2019-08-09T15:30:08.2146694Z 
2019-08-09T15:30:08.2146880Z ------------------------------------------
2019-08-09T15:30:08.2146939Z stderr:
2019-08-09T15:30:08.2146939Z stderr:
2019-08-09T15:30:08.2147127Z ------------------------------------------
2019-08-09T15:30:08.2147168Z error: lifetime may not live long enough
2019-08-09T15:30:08.2147397Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-09T15:30:08.2147824Z    |
2019-08-09T15:30:08.2148103Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2148350Z    |               -- -- lifetime `'b` defined here
2019-08-09T15:30:08.2148626Z    |               lifetime `'a` defined here
2019-08-09T15:30:08.2148672Z ...
2019-08-09T15:30:08.2148672Z ...
2019-08-09T15:30:08.2149184Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2149467Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:30:08.2149711Z error: lifetime may not live long enough
2019-08-09T15:30:08.2150026Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-09T15:30:08.2150078Z    |
2019-08-09T15:30:08.2150078Z    |
2019-08-09T15:30:08.2150303Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-09T15:30:08.2150542Z    |                   -- -- lifetime `'b` defined here
2019-08-09T15:30:08.2150816Z    |                   lifetime `'a` defined here
2019-08-09T15:30:08.2150873Z ...
2019-08-09T15:30:08.2150873Z ...
2019-08-09T15:30:08.2151270Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-09T15:30:08.2151512Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:30:08.2151593Z error: aborting due to 2 previous errors
2019-08-09T15:30:08.2151617Z 
2019-08-09T15:30:08.2151638Z 
2019-08-09T15:30:08.2151851Z ------------------------------------------
2019-08-09T15:30:08.2151851Z ------------------------------------------
2019-08-09T15:30:08.2151878Z 
2019-08-09T15:30:08.2151900Z 
2019-08-09T15:30:08.2152121Z ---- [ui] ui/regions/regions-outlives-projection-container-wc.rs#migrate stdout ----
2019-08-09T15:30:08.2152178Z diff of stderr:
2019-08-09T15:30:08.2152202Z 
2019-08-09T15:30:08.2152450Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2153064Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-09T15:30:08.2153103Z 3    |
2019-08-09T15:30:08.2153103Z 3    |
2019-08-09T15:30:08.2153294Z 4 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2153371Z 
2019-08-09T15:30:08.2153403Z 6    |
2019-08-09T15:30:08.2153403Z 6    |
2019-08-09T15:30:08.2153631Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 31:15
2019-08-09T15:30:08.2153840Z -   --> $DIR/regions-outlives-projection-container-wc.rs:31:15
2019-08-09T15:30:08.2154060Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-09T15:30:08.2154349Z +   --> $DIR/regions-outlives-projection-container-wc.rs:27:15
2019-08-09T15:30:08.2154394Z 9    |
2019-08-09T15:30:08.2154590Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2154667Z 
2019-08-09T15:30:08.2154667Z 
2019-08-09T15:30:08.2154900Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 31:18
2019-08-09T15:30:08.2155106Z -   --> $DIR/regions-outlives-projection-container-wc.rs:31:18
2019-08-09T15:30:08.2155351Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-09T15:30:08.2155552Z +   --> $DIR/regions-outlives-projection-container-wc.rs:27:18
2019-08-09T15:30:08.2155591Z 14    |
2019-08-09T15:30:08.2155765Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2155838Z 
2019-08-09T15:30:08.2155859Z 
2019-08-09T15:30:08.2155902Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2155902Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2156368Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate/regions-outlives-projection-container-wc.migrate.stderr
2019-08-09T15:30:08.2156751Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2156985Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-09T15:30:08.2157015Z 
2019-08-09T15:30:08.2157049Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-09T15:30:08.2157095Z status: exit code: 1
2019-08-09T15:30:08.2158246Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2158741Z ------------------------------------------
2019-08-09T15:30:08.2158776Z 
2019-08-09T15:30:08.2159016Z ------------------------------------------
2019-08-09T15:30:08.2159062Z stderr:
2019-08-09T15:30:08.2159062Z stderr:
2019-08-09T15:30:08.2159279Z ------------------------------------------
2019-08-09T15:30:08.2159579Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2159914Z    |
2019-08-09T15:30:08.2159914Z    |
2019-08-09T15:30:08.2160156Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2160258Z    |
2019-08-09T15:30:08.2160258Z    |
2019-08-09T15:30:08.2160522Z note: the pointer is valid for the lifetime 'a as defined on the function body at 27:15
2019-08-09T15:30:08.2160852Z    |
2019-08-09T15:30:08.2160852Z    |
2019-08-09T15:30:08.2161055Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2161292Z    |               ^^
2019-08-09T15:30:08.2161686Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 27:18
2019-08-09T15:30:08.2162312Z    |
2019-08-09T15:30:08.2162312Z    |
2019-08-09T15:30:08.2162473Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2162541Z 
2019-08-09T15:30:08.2162582Z error: aborting due to previous error
2019-08-09T15:30:08.2162605Z 
2019-08-09T15:30:08.2162799Z For more information about this error, try `rustc --explain E0491`.
---
2019-08-09T15:30:08.2163467Z 1 error: lifetime may not live long enough
2019-08-09T15:30:08.2163674Z -   --> $DIR/regions-outlives-projection-container-wc.rs:37:12
2019-08-09T15:30:08.2163866Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-09T15:30:08.2163903Z 3    |
2019-08-09T15:30:08.2164287Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2164488Z 5    |               -- -- lifetime `'b` defined here
2019-08-09T15:30:08.2164544Z 
2019-08-09T15:30:08.2164580Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2164580Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2164885Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll/regions-outlives-projection-container-wc.nll.stderr
2019-08-09T15:30:08.2165096Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2165340Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-09T15:30:08.2165370Z 
2019-08-09T15:30:08.2165408Z error in revision `nll`: 1 errors occurred comparing output.
2019-08-09T15:30:08.2165445Z status: exit code: 1
2019-08-09T15:30:08.2166131Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.nll/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2166497Z ------------------------------------------
2019-08-09T15:30:08.2166525Z 
2019-08-09T15:30:08.2166718Z ------------------------------------------
2019-08-09T15:30:08.2166755Z stderr:
2019-08-09T15:30:08.2166755Z stderr:
2019-08-09T15:30:08.2166935Z ------------------------------------------
2019-08-09T15:30:08.2166974Z error: lifetime may not live long enough
2019-08-09T15:30:08.2167208Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs:33:12
2019-08-09T15:30:08.2167250Z    |
2019-08-09T15:30:08.2168003Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2168261Z    |               -- -- lifetime `'b` defined here
2019-08-09T15:30:08.2168531Z    |               lifetime `'a` defined here
2019-08-09T15:30:08.2168601Z ...
2019-08-09T15:30:08.2168601Z ...
2019-08-09T15:30:08.2168835Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2169103Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-09T15:30:08.2169199Z error: aborting due to previous error
2019-08-09T15:30:08.2169228Z 
2019-08-09T15:30:08.2169254Z 
2019-08-09T15:30:08.2169470Z ------------------------------------------
2019-08-09T15:30:08.2169470Z ------------------------------------------
2019-08-09T15:30:08.2169512Z 
2019-08-09T15:30:08.2169538Z 
2019-08-09T15:30:08.2169784Z ---- [ui] ui/regions/regions-outlives-projection-container.rs stdout ----
2019-08-09T15:30:08.2169834Z diff of stderr:
2019-08-09T15:30:08.2169869Z 
2019-08-09T15:30:08.2170156Z 1 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2170671Z +   --> $DIR/regions-outlives-projection-container.rs:36:13
2019-08-09T15:30:08.2170818Z 3    |
2019-08-09T15:30:08.2170818Z 3    |
2019-08-09T15:30:08.2171091Z 4 LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2171191Z 
2019-08-09T15:30:08.2171229Z 6    |
2019-08-09T15:30:08.2171229Z 6    |
2019-08-09T15:30:08.2171972Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 32:15
2019-08-09T15:30:08.2172181Z -   --> $DIR/regions-outlives-projection-container.rs:32:15
2019-08-09T15:30:08.2172393Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 28:15
2019-08-09T15:30:08.2172584Z +   --> $DIR/regions-outlives-projection-container.rs:28:15
2019-08-09T15:30:08.2172638Z 9    |
2019-08-09T15:30:08.2172805Z 10 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2172873Z 
2019-08-09T15:30:08.2172873Z 
2019-08-09T15:30:08.2173111Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 32:18
2019-08-09T15:30:08.2173309Z -   --> $DIR/regions-outlives-projection-container.rs:32:18
2019-08-09T15:30:08.2173535Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 28:18
2019-08-09T15:30:08.2173738Z +   --> $DIR/regions-outlives-projection-container.rs:28:18
2019-08-09T15:30:08.2173774Z 14    |
2019-08-09T15:30:08.2173936Z 15 LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2174003Z 
2019-08-09T15:30:08.2174034Z 17 
2019-08-09T15:30:08.2174034Z 17 
2019-08-09T15:30:08.2174264Z 18 error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2174747Z +   --> $DIR/regions-outlives-projection-container.rs:54:13
2019-08-09T15:30:08.2174785Z 20    |
2019-08-09T15:30:08.2174785Z 20    |
2019-08-09T15:30:08.2174982Z 21 LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2175062Z 
2019-08-09T15:30:08.2175108Z 23    |
2019-08-09T15:30:08.2175108Z 23    |
2019-08-09T15:30:08.2175509Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 54:18
2019-08-09T15:30:08.2175708Z -   --> $DIR/regions-outlives-projection-container.rs:54:18
2019-08-09T15:30:08.2175940Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 50:18
2019-08-09T15:30:08.2176137Z +   --> $DIR/regions-outlives-projection-container.rs:50:18
2019-08-09T15:30:08.2176174Z 26    |
2019-08-09T15:30:08.2176344Z 27 LL | fn without_assoc<'a,'b>() {
2019-08-09T15:30:08.2176421Z 
2019-08-09T15:30:08.2176421Z 
2019-08-09T15:30:08.2176647Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 54:21
2019-08-09T15:30:08.2176862Z -   --> $DIR/regions-outlives-projection-container.rs:54:21
2019-08-09T15:30:08.2177099Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 50:21
2019-08-09T15:30:08.2177301Z +   --> $DIR/regions-outlives-projection-container.rs:50:21
2019-08-09T15:30:08.2177353Z 31    |
2019-08-09T15:30:08.2177941Z 32 LL | fn without_assoc<'a,'b>() {
2019-08-09T15:30:08.2178018Z 
2019-08-09T15:30:08.2178074Z 34 
2019-08-09T15:30:08.2178074Z 34 
2019-08-09T15:30:08.2178366Z 35 error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2178855Z +   --> $DIR/regions-outlives-projection-container.rs:63:12
2019-08-09T15:30:08.2178901Z 37    |
2019-08-09T15:30:08.2178901Z 37    |
2019-08-09T15:30:08.2179126Z 38 LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-09T15:30:08.2179227Z 
2019-08-09T15:30:08.2179265Z 40    |
2019-08-09T15:30:08.2179265Z 40    |
2019-08-09T15:30:08.2179531Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 62:20
2019-08-09T15:30:08.2179882Z -   --> $DIR/regions-outlives-projection-container.rs:62:20
2019-08-09T15:30:08.2180188Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 58:20
2019-08-09T15:30:08.2180431Z +   --> $DIR/regions-outlives-projection-container.rs:58:20
2019-08-09T15:30:08.2180487Z 43    |
2019-08-09T15:30:08.2180703Z 44 LL | fn call_with_assoc<'a,'b>() {
2019-08-09T15:30:08.2180779Z 
2019-08-09T15:30:08.2180779Z 
2019-08-09T15:30:08.2181387Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 62:23
2019-08-09T15:30:08.2182640Z -   --> $DIR/regions-outlives-projection-container.rs:62:23
2019-08-09T15:30:08.2182937Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 58:23
2019-08-09T15:30:08.2183358Z +   --> $DIR/regions-outlives-projection-container.rs:58:23
2019-08-09T15:30:08.2183396Z 48    |
2019-08-09T15:30:08.2183577Z 49 LL | fn call_with_assoc<'a,'b>() {
2019-08-09T15:30:08.2183658Z 
2019-08-09T15:30:08.2183689Z 51 
2019-08-09T15:30:08.2183689Z 51 
2019-08-09T15:30:08.2183932Z 52 error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2184344Z +   --> $DIR/regions-outlives-projection-container.rs:70:12
2019-08-09T15:30:08.2184381Z 54    |
2019-08-09T15:30:08.2184381Z 54    |
2019-08-09T15:30:08.2184579Z 55 LL |     call::<&'a WithoutAssoc<TheType<'b>>>();
2019-08-09T15:30:08.2184646Z 
2019-08-09T15:30:08.2184684Z 57    |
2019-08-09T15:30:08.2184684Z 57    |
2019-08-09T15:30:08.2185045Z - note: the pointer is valid for the lifetime 'a as defined on the function body at 71:23
2019-08-09T15:30:08.2185245Z -   --> $DIR/regions-outlives-projection-container.rs:71:23
2019-08-09T15:30:08.2185480Z + note: the pointer is valid for the lifetime 'a as defined on the function body at 67:23
2019-08-09T15:30:08.2185684Z +   --> $DIR/regions-outlives-projection-container.rs:67:23
2019-08-09T15:30:08.2185723Z 60    |
2019-08-09T15:30:08.2185899Z 61 LL | fn call_without_assoc<'a,'b>() {
2019-08-09T15:30:08.2185968Z 
2019-08-09T15:30:08.2185968Z 
2019-08-09T15:30:08.2186199Z - note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 71:26
2019-08-09T15:30:08.2186590Z -   --> $DIR/regions-outlives-projection-container.rs:71:26
2019-08-09T15:30:08.2186833Z + note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 67:26
2019-08-09T15:30:08.2187039Z +   --> $DIR/regions-outlives-projection-container.rs:67:26
2019-08-09T15:30:08.2187102Z 65    |
2019-08-09T15:30:08.2187287Z 66 LL | fn call_without_assoc<'a,'b>() {
2019-08-09T15:30:08.2187875Z 
2019-08-09T15:30:08.2187921Z 
2019-08-09T15:30:08.2187978Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2188378Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
2019-08-09T15:30:08.2188378Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
2019-08-09T15:30:08.2188648Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2188938Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container.rs`
2019-08-09T15:30:08.2189030Z error: 1 errors occurred comparing output.
2019-08-09T15:30:08.2189075Z status: exit code: 1
2019-08-09T15:30:08.2189075Z status: exit code: 1
2019-08-09T15:30:08.2189970Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2190361Z ------------------------------------------
2019-08-09T15:30:08.2190407Z 
2019-08-09T15:30:08.2190629Z ------------------------------------------
2019-08-09T15:30:08.2190675Z stderr:
2019-08-09T15:30:08.2190675Z stderr:
2019-08-09T15:30:08.2190890Z ------------------------------------------
2019-08-09T15:30:08.2191392Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2191882Z    |
2019-08-09T15:30:08.2191882Z    |
2019-08-09T15:30:08.2192272Z LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2192364Z    |
2019-08-09T15:30:08.2192364Z    |
2019-08-09T15:30:08.2192767Z note: the pointer is valid for the lifetime 'a as defined on the function body at 28:15
2019-08-09T15:30:08.2193384Z    |
2019-08-09T15:30:08.2193384Z    |
2019-08-09T15:30:08.2193570Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2193610Z    |               ^^
2019-08-09T15:30:08.2193853Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 28:18
2019-08-09T15:30:08.2194134Z    |
2019-08-09T15:30:08.2194134Z    |
2019-08-09T15:30:08.2194312Z LL | fn with_assoc<'a,'b>() {
2019-08-09T15:30:08.2194643Z 
2019-08-09T15:30:08.2194643Z 
2019-08-09T15:30:08.2194911Z error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2195205Z    |
2019-08-09T15:30:08.2195205Z    |
2019-08-09T15:30:08.2195404Z LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-09T15:30:08.2195495Z    |
2019-08-09T15:30:08.2195495Z    |
2019-08-09T15:30:08.2195715Z note: the pointer is valid for the lifetime 'a as defined on the function body at 50:18
2019-08-09T15:30:08.2195985Z    |
2019-08-09T15:30:08.2195985Z    |
2019-08-09T15:30:08.2196159Z LL | fn without_assoc<'a,'b>() {
2019-08-09T15:30:08.2196198Z    |                  ^^
2019-08-09T15:30:08.2196432Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 50:21
2019-08-09T15:30:08.2196877Z    |
2019-08-09T15:30:08.2196877Z    |
2019-08-09T15:30:08.2197053Z LL | fn without_assoc<'a,'b>() {
2019-08-09T15:30:08.2197166Z 
2019-08-09T15:30:08.2197166Z 
2019-08-09T15:30:08.2198225Z error[E0491]: in type `&'a WithAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2198609Z    |
2019-08-09T15:30:08.2198609Z    |
2019-08-09T15:30:08.2198830Z LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-09T15:30:08.2198936Z    |
2019-08-09T15:30:08.2198936Z    |
2019-08-09T15:30:08.2199197Z note: the pointer is valid for the lifetime 'a as defined on the function body at 58:20
2019-08-09T15:30:08.2199530Z    |
2019-08-09T15:30:08.2199530Z    |
2019-08-09T15:30:08.2199742Z LL | fn call_with_assoc<'a,'b>() {
2019-08-09T15:30:08.2199789Z    |                    ^^
2019-08-09T15:30:08.2200178Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 58:23
2019-08-09T15:30:08.2200532Z    |
2019-08-09T15:30:08.2200532Z    |
2019-08-09T15:30:08.2200762Z LL | fn call_with_assoc<'a,'b>() {
2019-08-09T15:30:08.2200839Z 
2019-08-09T15:30:08.2200839Z 
2019-08-09T15:30:08.2201136Z error[E0491]: in type `&'a WithoutAssoc<TheType<'b>>`, reference has a longer lifetime than the data it references
2019-08-09T15:30:08.2201607Z    |
2019-08-09T15:30:08.2201607Z    |
2019-08-09T15:30:08.2201830Z LL |     call::<&'a WithoutAssoc<TheType<'b>>>(); //~ ERROR reference has a longer lifetime
2019-08-09T15:30:08.2201929Z    |
2019-08-09T15:30:08.2201929Z    |
2019-08-09T15:30:08.2202153Z note: the pointer is valid for the lifetime 'a as defined on the function body at 67:23
2019-08-09T15:30:08.2202430Z    |
2019-08-09T15:30:08.2202430Z    |
2019-08-09T15:30:08.2202608Z LL | fn call_without_assoc<'a,'b>() {
2019-08-09T15:30:08.2202666Z    |                       ^^
2019-08-09T15:30:08.2202899Z note: but the referenced data is only valid for the lifetime 'b as defined on the function body at 67:26
2019-08-09T15:30:08.2203339Z    |
2019-08-09T15:30:08.2203339Z    |
2019-08-09T15:30:08.2203670Z LL | fn call_without_assoc<'a,'b>() {
2019-08-09T15:30:08.2203731Z 
2019-08-09T15:30:08.2203776Z error: aborting due to 4 previous errors
2019-08-09T15:30:08.2203882Z 
2019-08-09T15:30:08.2204097Z For more information about this error, try `rustc --explain E0491`.
2019-08-09T15:30:08.2204097Z For more information about this error, try `rustc --explain E0491`.
2019-08-09T15:30:08.2204124Z 
2019-08-09T15:30:08.2204308Z ------------------------------------------
2019-08-09T15:30:08.2204334Z 
2019-08-09T15:30:08.2204361Z 
2019-08-09T15:30:08.2204563Z ---- [ui] ui/specialization/defaultimpl/specialization-no-default.rs stdout ----
2019-08-09T15:30:08.2204610Z diff of stderr:
2019-08-09T15:30:08.2204632Z 
2019-08-09T15:30:08.2204671Z 1 error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2205045Z +   --> $DIR/specialization-no-default.rs:20:5
2019-08-09T15:30:08.2205081Z 3    |
2019-08-09T15:30:08.2205081Z 3    |
2019-08-09T15:30:08.2205115Z 4 LL | / impl<T> Foo for T {
2019-08-09T15:30:08.2205161Z 5 LL | |     fn foo(&self) {}
2019-08-09T15:30:08.2205183Z 
2019-08-09T15:30:08.2205220Z 13    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2205279Z 14 
2019-08-09T15:30:08.2205320Z 15 error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2205710Z +   --> $DIR/specialization-no-default.rs:23:5
2019-08-09T15:30:08.2205748Z 17    |
2019-08-09T15:30:08.2205748Z 17    |
2019-08-09T15:30:08.2205781Z 18 LL | / impl<T> Foo for T {
2019-08-09T15:30:08.2205815Z 19 LL | |     fn foo(&self) {}
2019-08-09T15:30:08.2205851Z 
2019-08-09T15:30:08.2205888Z 27    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2205923Z 28 
2019-08-09T15:30:08.2205969Z 29 error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2206334Z +   --> $DIR/specialization-no-default.rs:37:5
2019-08-09T15:30:08.2206383Z 31    |
2019-08-09T15:30:08.2206383Z 31    |
2019-08-09T15:30:08.2206425Z 32 LL | / impl<T> Bar for T {
2019-08-09T15:30:08.2206459Z 33 LL | |     type T = u8;
2019-08-09T15:30:08.2206480Z 
2019-08-09T15:30:08.2206530Z 40    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2206566Z 41 
2019-08-09T15:30:08.2206669Z 42 error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2207082Z +   --> $DIR/specialization-no-default.rs:55:5
2019-08-09T15:30:08.2207118Z 44    |
2019-08-09T15:30:08.2207118Z 44    |
2019-08-09T15:30:08.2207152Z 45 LL | / impl<T: Clone> Baz for T {
2019-08-09T15:30:08.2207194Z 46 LL | |     fn baz(&self) {}
2019-08-09T15:30:08.2207216Z 
2019-08-09T15:30:08.2207252Z 53    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2207299Z 54 
2019-08-09T15:30:08.2207339Z 55 error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2207729Z +   --> $DIR/specialization-no-default.rs:74:5
2019-08-09T15:30:08.2207765Z 57    |
2019-08-09T15:30:08.2207765Z 57    |
2019-08-09T15:30:08.2207799Z 58 LL | / impl<T: Clone> Redundant for T {
2019-08-09T15:30:08.2208261Z 59 LL | |     fn redundant(&self) {}
2019-08-09T15:30:08.2208365Z 
2019-08-09T15:30:08.2208411Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2208807Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/specialization-no-default.stderr
2019-08-09T15:30:08.2208807Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/specialization-no-default.stderr
2019-08-09T15:30:08.2209082Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2209372Z To only update this specific test, also pass `--test-args specialization/defaultimpl/specialization-no-default.rs`
2019-08-09T15:30:08.2209467Z error: 1 errors occurred comparing output.
2019-08-09T15:30:08.2209511Z status: exit code: 1
2019-08-09T15:30:08.2209511Z status: exit code: 1
2019-08-09T15:30:08.2210536Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/defaultimpl/specialization-no-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-no-default/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2210873Z ------------------------------------------
2019-08-09T15:30:08.2210907Z 
2019-08-09T15:30:08.2211128Z ------------------------------------------
2019-08-09T15:30:08.2211172Z stderr:
2019-08-09T15:30:08.2211172Z stderr:
2019-08-09T15:30:08.2211397Z ------------------------------------------
2019-08-09T15:30:08.2211461Z error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2211949Z    |
2019-08-09T15:30:08.2211949Z    |
2019-08-09T15:30:08.2211984Z LL | / impl<T> Foo for T {
2019-08-09T15:30:08.2212022Z LL | |     fn foo(&self) {}
2019-08-09T15:30:08.2212079Z LL | |     fn bar(&self) {}
2019-08-09T15:30:08.2212116Z LL | | }
2019-08-09T15:30:08.2212302Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2212353Z ...
2019-08-09T15:30:08.2212393Z LL |       fn foo(&self) {} //~ ERROR E0520
2019-08-09T15:30:08.2212436Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `foo`
2019-08-09T15:30:08.2212475Z    |
2019-08-09T15:30:08.2212528Z    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2212556Z 
2019-08-09T15:30:08.2212598Z error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2212901Z    |
2019-08-09T15:30:08.2212901Z    |
2019-08-09T15:30:08.2212937Z LL | / impl<T> Foo for T {
2019-08-09T15:30:08.2213066Z LL | |     fn foo(&self) {}
2019-08-09T15:30:08.2213111Z LL | |     fn bar(&self) {}
2019-08-09T15:30:08.2213147Z LL | | }
2019-08-09T15:30:08.2213515Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2213564Z ...
2019-08-09T15:30:08.2213600Z LL |       fn bar(&self) {} //~ ERROR E0520
2019-08-09T15:30:08.2213641Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `bar`
2019-08-09T15:30:08.2213688Z    |
2019-08-09T15:30:08.2213891Z    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2213917Z 
2019-08-09T15:30:08.2213956Z error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2214402Z    |
2019-08-09T15:30:08.2214402Z    |
2019-08-09T15:30:08.2214434Z LL | / impl<T> Bar for T {
2019-08-09T15:30:08.2214484Z LL | |     type T = u8;
2019-08-09T15:30:08.2214517Z LL | | }
2019-08-09T15:30:08.2214685Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2214744Z ...
2019-08-09T15:30:08.2214779Z LL |       type T = (); //~ ERROR E0520
2019-08-09T15:30:08.2214817Z    |       ^^^^^^^^^^^^ cannot specialize default item `T`
2019-08-09T15:30:08.2214851Z    |
2019-08-09T15:30:08.2214902Z    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2214926Z 
2019-08-09T15:30:08.2214965Z error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2215229Z    |
2019-08-09T15:30:08.2215229Z    |
2019-08-09T15:30:08.2215263Z LL | / impl<T: Clone> Baz for T {
2019-08-09T15:30:08.2215306Z LL | |     fn baz(&self) {}
2019-08-09T15:30:08.2215411Z LL | | }
2019-08-09T15:30:08.2215595Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2215631Z ...
2019-08-09T15:30:08.2215674Z LL |       fn baz(&self) {} //~ ERROR E0520
2019-08-09T15:30:08.2215720Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `baz`
2019-08-09T15:30:08.2215753Z    |
2019-08-09T15:30:08.2215800Z    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2215824Z 
2019-08-09T15:30:08.2215864Z error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2216137Z    |
2019-08-09T15:30:08.2216137Z    |
2019-08-09T15:30:08.2216170Z LL | / impl<T: Clone> Redundant for T {
2019-08-09T15:30:08.2216205Z LL | |     fn redundant(&self) {}
2019-08-09T15:30:08.2216253Z LL | | }
2019-08-09T15:30:08.2216420Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2216462Z ...
2019-08-09T15:30:08.2216512Z LL |       fn redundant(&self) {} //~ ERROR E0520
2019-08-09T15:30:08.2216552Z    |       ^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `redundant`
2019-08-09T15:30:08.2216587Z    |
2019-08-09T15:30:08.2216642Z    = note: to specialize, `redundant` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2216702Z error: aborting due to 5 previous errors
2019-08-09T15:30:08.2216724Z 
2019-08-09T15:30:08.2216941Z For more information about this error, try `rustc --explain E0520`.
2019-08-09T15:30:08.2216969Z 
2019-08-09T15:30:08.2216969Z 
2019-08-09T15:30:08.2217144Z ------------------------------------------
2019-08-09T15:30:08.2217169Z 
2019-08-09T15:30:08.2217190Z 
2019-08-09T15:30:08.2217393Z ---- [ui] ui/specialization/specialization-no-default.rs stdout ----
2019-08-09T15:30:08.2217432Z diff of stderr:
2019-08-09T15:30:08.2217454Z 
2019-08-09T15:30:08.2217498Z 1 error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2218471Z +   --> $DIR/specialization-no-default.rs:20:5
2019-08-09T15:30:08.2218521Z 3    |
2019-08-09T15:30:08.2218521Z 3    |
2019-08-09T15:30:08.2218575Z 4 LL | / impl<T> Foo for T {
2019-08-09T15:30:08.2218734Z 5 LL | |     fn foo(&self) {}
2019-08-09T15:30:08.2218771Z 
2019-08-09T15:30:08.2218828Z 13    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2218873Z 14 
2019-08-09T15:30:08.2218925Z 15 error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2219440Z +   --> $DIR/specialization-no-default.rs:23:5
2019-08-09T15:30:08.2219486Z 17    |
2019-08-09T15:30:08.2219486Z 17    |
2019-08-09T15:30:08.2219527Z 18 LL | / impl<T> Foo for T {
2019-08-09T15:30:08.2219587Z 19 LL | |     fn foo(&self) {}
2019-08-09T15:30:08.2219615Z 
2019-08-09T15:30:08.2219661Z 27    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2219727Z 28 
2019-08-09T15:30:08.2219776Z 29 error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2220259Z +   --> $DIR/specialization-no-default.rs:37:5
2019-08-09T15:30:08.2220306Z 31    |
2019-08-09T15:30:08.2220306Z 31    |
2019-08-09T15:30:08.2220346Z 32 LL | / impl<T> Bar for T {
2019-08-09T15:30:08.2220388Z 33 LL | |     type T = u8;
2019-08-09T15:30:08.2220424Z 
2019-08-09T15:30:08.2220470Z 40    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2220513Z 41 
2019-08-09T15:30:08.2220571Z 42 error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2221026Z +   --> $DIR/specialization-no-default.rs:55:5
2019-08-09T15:30:08.2221071Z 44    |
2019-08-09T15:30:08.2221071Z 44    |
2019-08-09T15:30:08.2221207Z 45 LL | / impl<T: Clone> Baz for T {
2019-08-09T15:30:08.2221251Z 46 LL | |     fn baz(&self) {}
2019-08-09T15:30:08.2221448Z 
2019-08-09T15:30:08.2221499Z 53    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2221535Z 54 
2019-08-09T15:30:08.2221582Z 55 error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2222334Z +   --> $DIR/specialization-no-default.rs:74:5
2019-08-09T15:30:08.2222373Z 57    |
2019-08-09T15:30:08.2222373Z 57    |
2019-08-09T15:30:08.2222409Z 58 LL | / impl<T: Clone> Redundant for T {
2019-08-09T15:30:08.2222463Z 59 LL | |     fn redundant(&self) {}
2019-08-09T15:30:08.2222509Z 
2019-08-09T15:30:08.2222546Z The actual stderr differed from the expected stderr.
2019-08-09T15:30:08.2222832Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/specialization-no-default.stderr
2019-08-09T15:30:08.2222832Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/specialization-no-default.stderr
2019-08-09T15:30:08.2223057Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T15:30:08.2223309Z To only update this specific test, also pass `--test-args specialization/specialization-no-default.rs`
2019-08-09T15:30:08.2223384Z error: 1 errors occurred comparing output.
2019-08-09T15:30:08.2223421Z status: exit code: 1
2019-08-09T15:30:08.2223421Z status: exit code: 1
2019-08-09T15:30:08.2224086Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/specialization-no-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-no-default/auxiliary" "-A" "unused"
2019-08-09T15:30:08.2224371Z ------------------------------------------
2019-08-09T15:30:08.2224399Z 
2019-08-09T15:30:08.2224579Z ------------------------------------------
2019-08-09T15:30:08.2224713Z stderr:
2019-08-09T15:30:08.2224713Z stderr:
2019-08-09T15:30:08.2224929Z ------------------------------------------
2019-08-09T15:30:08.2224977Z error[E0520]: `foo` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2225406Z    |
2019-08-09T15:30:08.2225406Z    |
2019-08-09T15:30:08.2225439Z LL | / impl<T> Foo for T {
2019-08-09T15:30:08.2225481Z LL | |     fn foo(&self) {}
2019-08-09T15:30:08.2225516Z LL | |     fn bar(&self) {}
2019-08-09T15:30:08.2225549Z LL | | }
2019-08-09T15:30:08.2225719Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2225762Z ...
2019-08-09T15:30:08.2225796Z LL |       fn foo(&self) {} //~ ERROR E0520
2019-08-09T15:30:08.2225844Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `foo`
2019-08-09T15:30:08.2225887Z    |
2019-08-09T15:30:08.2225924Z    = note: to specialize, `foo` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2225949Z 
2019-08-09T15:30:08.2226006Z error[E0520]: `bar` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2226254Z    |
2019-08-09T15:30:08.2226254Z    |
2019-08-09T15:30:08.2226286Z LL | / impl<T> Foo for T {
2019-08-09T15:30:08.2226335Z LL | |     fn foo(&self) {}
2019-08-09T15:30:08.2226369Z LL | |     fn bar(&self) {}
2019-08-09T15:30:08.2226402Z LL | | }
2019-08-09T15:30:08.2226584Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2226619Z ...
2019-08-09T15:30:08.2226654Z LL |       fn bar(&self) {} //~ ERROR E0520
2019-08-09T15:30:08.2226703Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `bar`
2019-08-09T15:30:08.2226738Z    |
2019-08-09T15:30:08.2226849Z    = note: to specialize, `bar` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2226875Z 
2019-08-09T15:30:08.2226924Z error[E0520]: `T` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2227204Z    |
2019-08-09T15:30:08.2227204Z    |
2019-08-09T15:30:08.2227246Z LL | / impl<T> Bar for T {
2019-08-09T15:30:08.2227280Z LL | |     type T = u8;
2019-08-09T15:30:08.2227311Z LL | | }
2019-08-09T15:30:08.2227490Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2227525Z ...
2019-08-09T15:30:08.2227559Z LL |       type T = (); //~ ERROR E0520
2019-08-09T15:30:08.2227597Z    |       ^^^^^^^^^^^^ cannot specialize default item `T`
2019-08-09T15:30:08.2227645Z    |
2019-08-09T15:30:08.2227681Z    = note: to specialize, `T` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2227706Z 
2019-08-09T15:30:08.2227760Z error[E0520]: `baz` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2228375Z    |
2019-08-09T15:30:08.2228375Z    |
2019-08-09T15:30:08.2228417Z LL | / impl<T: Clone> Baz for T {
2019-08-09T15:30:08.2228493Z LL | |     fn baz(&self) {}
2019-08-09T15:30:08.2228535Z LL | | }
2019-08-09T15:30:08.2228792Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2228850Z ...
2019-08-09T15:30:08.2228894Z LL |       fn baz(&self) {} //~ ERROR E0520
2019-08-09T15:30:08.2228943Z    |       ^^^^^^^^^^^^^^^^ cannot specialize default item `baz`
2019-08-09T15:30:08.2228998Z    |
2019-08-09T15:30:08.2229045Z    = note: to specialize, `baz` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2229078Z 
2019-08-09T15:30:08.2229127Z error[E0520]: `redundant` specializes an item from a parent `impl`, but that item is not marked `default`
2019-08-09T15:30:08.2229461Z    |
2019-08-09T15:30:08.2229461Z    |
2019-08-09T15:30:08.2229504Z LL | / impl<T: Clone> Redundant for T {
2019-08-09T15:30:08.2229562Z LL | |     fn redundant(&self) {}
2019-08-09T15:30:08.2229605Z LL | | }
2019-08-09T15:30:08.2229814Z    | |_- parent `impl` is here
2019-08-09T15:30:08.2229966Z ...
2019-08-09T15:30:08.2230021Z LL |       default fn redundant(&self) {} //~ ERROR E0520
2019-08-09T15:30:08.2230074Z    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `redundant`
2019-08-09T15:30:08.2230132Z    |
2019-08-09T15:30:08.2230181Z    = note: to specialize, `redundant` in the parent `impl` must be marked `default`
2019-08-09T15:30:08.2230256Z error: aborting due to 5 previous errors
2019-08-09T15:30:08.2230293Z 
2019-08-09T15:30:08.2230575Z For more information about this error, try `rustc --explain E0520`.
2019-08-09T15:30:08.2230610Z 
---
2019-08-09T15:30:08.2234316Z test result: FAILED. 8808 passed; 12 failed; 33 ignored; 0 measured; 0 filtered out
2019-08-09T15:30:08.2234348Z 
2019-08-09T15:30:08.2234369Z 
2019-08-09T15:30:08.2234390Z 
2019-08-09T15:30:08.2235651Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-09T15:30:08.2236179Z 
2019-08-09T15:30:08.2236202Z 
2019-08-09T15:30:08.2236244Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-09T15:30:08.2236282Z Build completed unsuccessfully in 1:06:46
2019-08-09T15:30:08.2236282Z Build completed unsuccessfully in 1:06:46
2019-08-09T15:30:08.2236509Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-09T15:30:08.2236572Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-09T15:30:08.9774400Z ##[error]Bash exited with code '1'.
2019-08-09T15:30:08.9819042Z ##[section]Starting: Checkout
2019-08-09T15:30:08.9820916Z ==============================================================================
2019-08-09T15:30:08.9820972Z Task         : Get sources
2019-08-09T15:30:08.9821019Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
