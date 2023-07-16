plain
2019-08-15T10:25:22.1280719Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T10:25:22.1280860Z 
2019-08-15T10:25:22.1281114Z   git checkout -b <new-branch-name>
2019-08-15T10:25:22.1281162Z 
2019-08-15T10:25:22.1281493Z HEAD is now at a3687de17 Auto merge of #63588 - Centril:rollup-9ad6qs2, r=Centril
2019-08-15T10:25:22.1446054Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-15T10:25:22.1449623Z ==============================================================================
2019-08-15T10:25:22.1449713Z Task         : Bash
2019-08-15T10:25:22.1449796Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-15T12:28:13.7959997Z 
2019-08-15T12:28:13.7961015Z ---- [ui (nll)] ui/async-await/issues/issue-63388-1.rs stdout ----
2019-08-15T12:28:13.7961115Z diff of stderr:
2019-08-15T12:28:13.7961157Z 
2019-08-15T12:28:13.7961417Z 4 LL |     ) -> &dyn Foo
2019-08-15T12:28:13.7961566Z 6    |
2019-08-15T12:28:13.7961566Z 6    |
2019-08-15T12:28:13.7962196Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#27r
2019-08-15T12:28:13.7962565Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#22r
2019-08-15T12:28:13.7962728Z 9 error: lifetime may not live long enough
2019-08-15T12:28:13.7962994Z 10   --> $DIR/issue-63388-1.rs:15:5
2019-08-15T12:28:13.7963058Z 
2019-08-15T12:28:13.7963094Z 
2019-08-15T12:28:13.7963094Z 
2019-08-15T12:28:13.7963162Z The actual stderr differed from the expected stderr.
2019-08-15T12:28:13.7963587Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/issue-63388-1.nll.stderr
2019-08-15T12:28:13.7965400Z To update references, rerun the tests and pass the `--bless` flag
2019-08-15T12:28:13.7967026Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-1.rs`
2019-08-15T12:28:13.7967185Z error: 1 errors occurred comparing output.
2019-08-15T12:28:13.7967272Z status: exit code: 1
2019-08-15T12:28:13.7967272Z status: exit code: 1
2019-08-15T12:28:13.7968816Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/auxiliary" "-A" "unused"
2019-08-15T12:28:13.7969460Z ------------------------------------------
2019-08-15T12:28:13.7969512Z 
2019-08-15T12:28:13.7969772Z ------------------------------------------
2019-08-15T12:28:13.7969861Z stderr:
2019-08-15T12:28:13.7969861Z stderr:
2019-08-15T12:28:13.7970109Z ------------------------------------------
2019-08-15T12:28:13.7970216Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.7970549Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:14:10
2019-08-15T12:28:13.7970632Z    |
2019-08-15T12:28:13.7970915Z LL |     ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-15T12:28:13.7971069Z    |
2019-08-15T12:28:13.7971069Z    |
2019-08-15T12:28:13.7971358Z    = note: hidden type `impl std::future::Future` captures lifetime '_#22r
2019-08-15T12:28:13.7971495Z error: lifetime may not live long enough
2019-08-15T12:28:13.7972258Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:15:5
2019-08-15T12:28:13.7972556Z    |
2019-08-15T12:28:13.7972556Z    |
2019-08-15T12:28:13.7972863Z LL |       async fn do_sth<'a>(
2019-08-15T12:28:13.7973145Z    |                       -- lifetime `'a` defined here
2019-08-15T12:28:13.7973380Z LL |           &'a self, foo: &dyn Foo
2019-08-15T12:28:13.7973655Z    |                          - lifetime `'_` defined here
2019-08-15T12:28:13.7973908Z LL |       ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-15T12:28:13.7974059Z LL | |         foo
2019-08-15T12:28:13.7974135Z LL | |     }
2019-08-15T12:28:13.7974135Z LL | |     }
2019-08-15T12:28:13.7974466Z    | |_____^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.7974592Z error: aborting due to 2 previous errors
2019-08-15T12:28:13.7974651Z 
2019-08-15T12:28:13.7974911Z For more information about this error, try `rustc --explain E0700`.
2019-08-15T12:28:13.7974962Z 
2019-08-15T12:28:13.7974962Z 
2019-08-15T12:28:13.7975317Z ------------------------------------------
2019-08-15T12:28:13.7975488Z 
2019-08-15T12:28:13.7975523Z 
2019-08-15T12:28:13.7975821Z ---- [ui (nll)] ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs stdout ----
2019-08-15T12:28:13.7975920Z diff of stderr:
2019-08-15T12:28:13.7975958Z 
2019-08-15T12:28:13.7976210Z 4 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-15T12:28:13.7976378Z 6    |
2019-08-15T12:28:13.7976378Z 6    |
2019-08-15T12:28:13.7976664Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-15T12:28:13.7976946Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.7977106Z 9 error: lifetime may not live long enough
2019-08-15T12:28:13.7977418Z 10   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-15T12:28:13.7977472Z 
2019-08-15T12:28:13.7977472Z 
2019-08-15T12:28:13.7977785Z 30 LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-08-15T12:28:13.7977977Z 32    |
2019-08-15T12:28:13.7977977Z 32    |
2019-08-15T12:28:13.7978270Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-15T12:28:13.7978597Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.7978857Z 35 error: lifetime may not live long enough
2019-08-15T12:28:13.7979125Z 36   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-15T12:28:13.7979193Z 
2019-08-15T12:28:13.7979227Z 
2019-08-15T12:28:13.7979227Z 
2019-08-15T12:28:13.7979291Z The actual stderr differed from the expected stderr.
2019-08-15T12:28:13.7979844Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/arbitrary_self_types_pin_lifetime_mismatch-async.nll.stderr
2019-08-15T12:28:13.7980278Z To update references, rerun the tests and pass the `--bless` flag
2019-08-15T12:28:13.7980662Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`
2019-08-15T12:28:13.7980817Z error: 1 errors occurred comparing output.
2019-08-15T12:28:13.7980901Z status: exit code: 1
2019-08-15T12:28:13.7980901Z status: exit code: 1
2019-08-15T12:28:13.7982168Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/auxiliary" "-A" "unused"
2019-08-15T12:28:13.7982944Z ------------------------------------------
2019-08-15T12:28:13.7982994Z 
2019-08-15T12:28:13.7983250Z ------------------------------------------
2019-08-15T12:28:13.7983318Z stderr:
2019-08-15T12:28:13.7983318Z stderr:
2019-08-15T12:28:13.7983564Z ------------------------------------------
2019-08-15T12:28:13.7983651Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.7984063Z    |
2019-08-15T12:28:13.7984063Z    |
2019-08-15T12:28:13.7984333Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-15T12:28:13.7984498Z    |
2019-08-15T12:28:13.7984498Z    |
2019-08-15T12:28:13.7984765Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.7984904Z error: lifetime may not live long enough
2019-08-15T12:28:13.7985319Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-15T12:28:13.7985405Z    |
2019-08-15T12:28:13.7985405Z    |
2019-08-15T12:28:13.7985666Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-15T12:28:13.7986026Z    |                          -                       ^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.7986144Z    |                          |
2019-08-15T12:28:13.7986393Z    |                          lifetime `'_` defined here
2019-08-15T12:28:13.7986658Z    |                          lifetime `'_` defined here
2019-08-15T12:28:13.7986781Z error: lifetime may not live long enough
2019-08-15T12:28:13.7987061Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:13:73
2019-08-15T12:28:13.7987153Z    |
2019-08-15T12:28:13.7987153Z    |
2019-08-15T12:28:13.7987431Z LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-15T12:28:13.7987851Z    |                          -                                              ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.7987962Z    |                          |
2019-08-15T12:28:13.7988226Z    |                          lifetime `'_` defined here
2019-08-15T12:28:13.7988592Z    |                          lifetime `'_` defined here
2019-08-15T12:28:13.7988657Z 
2019-08-15T12:28:13.7988731Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.7989126Z    |
2019-08-15T12:28:13.7989126Z    |
2019-08-15T12:28:13.7989423Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-15T12:28:13.7989698Z    |
2019-08-15T12:28:13.7989698Z    |
2019-08-15T12:28:13.7990016Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.7990159Z error: lifetime may not live long enough
2019-08-15T12:28:13.7990470Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-15T12:28:13.7990550Z    |
2019-08-15T12:28:13.7990550Z    |
2019-08-15T12:28:13.7990845Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-15T12:28:13.7991239Z    |                  --              -                           ^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-08-15T12:28:13.7991361Z    |                  |               |
2019-08-15T12:28:13.7991621Z    |                  |               lifetime `'_` defined here
2019-08-15T12:28:13.7992241Z 
2019-08-15T12:28:13.7992331Z error: aborting due to 5 previous errors
2019-08-15T12:28:13.7992373Z 
2019-08-15T12:28:13.7992822Z For more information about this error, try `rustc --explain E0700`.
2019-08-15T12:28:13.7992822Z For more information about this error, try `rustc --explain E0700`.
2019-08-15T12:28:13.7992878Z 
2019-08-15T12:28:13.7993108Z ------------------------------------------
2019-08-15T12:28:13.7993169Z 
2019-08-15T12:28:13.7993204Z 
2019-08-15T12:28:13.7993458Z ---- [ui (nll)] ui/self/elision/lt-ref-self-async.rs stdout ----
2019-08-15T12:28:13.7993549Z diff of stderr:
2019-08-15T12:28:13.7993587Z 
2019-08-15T12:28:13.7993849Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-15T12:28:13.7994012Z 6    |
2019-08-15T12:28:13.7994012Z 6    |
2019-08-15T12:28:13.7994281Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-15T12:28:13.7994581Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.7994740Z 9 error: lifetime may not live long enough
2019-08-15T12:28:13.7995011Z 10   --> $DIR/lt-ref-self-async.rs:15:47
2019-08-15T12:28:13.7995077Z 
2019-08-15T12:28:13.7995077Z 
2019-08-15T12:28:13.7995363Z 24 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-15T12:28:13.7995538Z 26    |
2019-08-15T12:28:13.7995538Z 26    |
2019-08-15T12:28:13.7995937Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-15T12:28:13.7996213Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.7996370Z 29 error: lifetime may not live long enough
2019-08-15T12:28:13.7996639Z 30   --> $DIR/lt-ref-self-async.rs:21:53
2019-08-15T12:28:13.7996684Z 
2019-08-15T12:28:13.7996684Z 
2019-08-15T12:28:13.7996956Z 44 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.7997152Z 46    |
2019-08-15T12:28:13.7997152Z 46    |
2019-08-15T12:28:13.7997437Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-15T12:28:13.7997754Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.7997916Z 49 error: lifetime may not live long enough
2019-08-15T12:28:13.7998168Z 50   --> $DIR/lt-ref-self-async.rs:25:62
2019-08-15T12:28:13.7998215Z 
2019-08-15T12:28:13.7998215Z 
2019-08-15T12:28:13.7998498Z 64 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.7998675Z 66    |
2019-08-15T12:28:13.7998675Z 66    |
2019-08-15T12:28:13.7999062Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-15T12:28:13.7999370Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.7999525Z 69 error: lifetime may not live long enough
2019-08-15T12:28:13.7999781Z 70   --> $DIR/lt-ref-self-async.rs:29:62
2019-08-15T12:28:13.7999826Z 
2019-08-15T12:28:13.7999826Z 
2019-08-15T12:28:13.8000194Z 84 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8000396Z 86    |
2019-08-15T12:28:13.8000396Z 86    |
2019-08-15T12:28:13.8000719Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-15T12:28:13.8001001Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.8001156Z 89 error: lifetime may not live long enough
2019-08-15T12:28:13.8001411Z 90   --> $DIR/lt-ref-self-async.rs:33:71
2019-08-15T12:28:13.8001455Z 
2019-08-15T12:28:13.8001455Z 
2019-08-15T12:28:13.8002003Z 104 LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8002194Z 106    |
2019-08-15T12:28:13.8002194Z 106    |
2019-08-15T12:28:13.8002816Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-15T12:28:13.8003191Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.8003480Z 109 error: lifetime may not live long enough
2019-08-15T12:28:13.8003768Z 110   --> $DIR/lt-ref-self-async.rs:37:67
2019-08-15T12:28:13.8003831Z 
2019-08-15T12:28:13.8003865Z 
2019-08-15T12:28:13.8003865Z 
2019-08-15T12:28:13.8003928Z The actual stderr differed from the expected stderr.
2019-08-15T12:28:13.8004309Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/lt-ref-self-async.nll.stderr
2019-08-15T12:28:13.8004619Z To update references, rerun the tests and pass the `--bless` flag
2019-08-15T12:28:13.8004920Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`
2019-08-15T12:28:13.8005056Z error: 1 errors occurred comparing output.
2019-08-15T12:28:13.8005121Z status: exit code: 1
2019-08-15T12:28:13.8005121Z status: exit code: 1
2019-08-15T12:28:13.8006152Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/auxiliary" "-A" "unused"
2019-08-15T12:28:13.8006667Z ------------------------------------------
2019-08-15T12:28:13.8006731Z 
2019-08-15T12:28:13.8007098Z ------------------------------------------
2019-08-15T12:28:13.8007228Z stderr:
2019-08-15T12:28:13.8007228Z stderr:
2019-08-15T12:28:13.8007460Z ------------------------------------------
2019-08-15T12:28:13.8007562Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8008163Z    |
2019-08-15T12:28:13.8008163Z    |
2019-08-15T12:28:13.8008423Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-15T12:28:13.8008600Z    |
2019-08-15T12:28:13.8008600Z    |
2019-08-15T12:28:13.8008867Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.8008997Z error: lifetime may not live long enough
2019-08-15T12:28:13.8009266Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:15:47
2019-08-15T12:28:13.8009357Z    |
2019-08-15T12:28:13.8009357Z    |
2019-08-15T12:28:13.8009607Z LL |       async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-15T12:28:13.8009879Z    |  _______________________-_______________________^
2019-08-15T12:28:13.8009951Z    | |                       |
2019-08-15T12:28:13.8010217Z    | |                       lifetime `'_` defined here
2019-08-15T12:28:13.8010469Z    | |                       lifetime `'_` defined here
2019-08-15T12:28:13.8010673Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8010759Z LL | |     }
2019-08-15T12:28:13.8011128Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8011193Z 
2019-08-15T12:28:13.8011286Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8011915Z    |
2019-08-15T12:28:13.8011915Z    |
2019-08-15T12:28:13.8012238Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-15T12:28:13.8012405Z    |
2019-08-15T12:28:13.8012405Z    |
2019-08-15T12:28:13.8012689Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.8012818Z error: lifetime may not live long enough
2019-08-15T12:28:13.8013097Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:21:53
2019-08-15T12:28:13.8013186Z    |
2019-08-15T12:28:13.8013186Z    |
2019-08-15T12:28:13.8013619Z LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-15T12:28:13.8013906Z    |  _____________________________-_______________________^
2019-08-15T12:28:13.8013979Z    | |                             |
2019-08-15T12:28:13.8014254Z    | |                             lifetime `'_` defined here
2019-08-15T12:28:13.8014516Z    | |                             lifetime `'_` defined here
2019-08-15T12:28:13.8014609Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8014675Z LL | |     }
2019-08-15T12:28:13.8015002Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8015067Z 
2019-08-15T12:28:13.8015157Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8015527Z    |
2019-08-15T12:28:13.8015527Z    |
2019-08-15T12:28:13.8015796Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8015974Z    |
2019-08-15T12:28:13.8015974Z    |
2019-08-15T12:28:13.8016256Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.8016384Z error: lifetime may not live long enough
2019-08-15T12:28:13.8016650Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:25:62
2019-08-15T12:28:13.8016737Z    |
2019-08-15T12:28:13.8016737Z    |
2019-08-15T12:28:13.8017000Z LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8017659Z    |  _____________________________________-________________________^
2019-08-15T12:28:13.8017752Z    | |                                     |
2019-08-15T12:28:13.8018043Z    | |                                     lifetime `'_` defined here
2019-08-15T12:28:13.8018481Z    | |                                     lifetime `'_` defined here
2019-08-15T12:28:13.8018605Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8018678Z LL | |     }
2019-08-15T12:28:13.8019055Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8019121Z 
2019-08-15T12:28:13.8019213Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8019583Z    |
2019-08-15T12:28:13.8019583Z    |
2019-08-15T12:28:13.8019841Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8020009Z    |
2019-08-15T12:28:13.8020009Z    |
2019-08-15T12:28:13.8022224Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.8022392Z error: lifetime may not live long enough
2019-08-15T12:28:13.8022977Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:29:62
2019-08-15T12:28:13.8023100Z    |
2019-08-15T12:28:13.8023100Z    |
2019-08-15T12:28:13.8023417Z LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8023974Z    |  _____________________________________-________________________^
2019-08-15T12:28:13.8024065Z    | |                                     |
2019-08-15T12:28:13.8024355Z    | |                                     lifetime `'_` defined here
2019-08-15T12:28:13.8024629Z    | |                                     lifetime `'_` defined here
2019-08-15T12:28:13.8024724Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8024789Z LL | |     }
2019-08-15T12:28:13.8025116Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8025287Z 
2019-08-15T12:28:13.8025377Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8025874Z    |
2019-08-15T12:28:13.8025874Z    |
2019-08-15T12:28:13.8029028Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8029240Z    |
2019-08-15T12:28:13.8029240Z    |
2019-08-15T12:28:13.8029547Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.8029680Z error: lifetime may not live long enough
2019-08-15T12:28:13.8029948Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:33:71
2019-08-15T12:28:13.8030037Z    |
2019-08-15T12:28:13.8030037Z    |
2019-08-15T12:28:13.8030310Z LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8030612Z    |  _____________________________________________-_________________________^
2019-08-15T12:28:13.8030697Z    | |                                             |
2019-08-15T12:28:13.8031006Z    | |                                             lifetime `'_` defined here
2019-08-15T12:28:13.8031304Z    | |                                             lifetime `'_` defined here
2019-08-15T12:28:13.8031404Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8031470Z LL | |     }
2019-08-15T12:28:13.8035397Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8035479Z 
2019-08-15T12:28:13.8035578Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8035983Z    |
2019-08-15T12:28:13.8035983Z    |
2019-08-15T12:28:13.8036247Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8040615Z    |
2019-08-15T12:28:13.8040615Z    |
2019-08-15T12:28:13.8042249Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-15T12:28:13.8042420Z error: lifetime may not live long enough
2019-08-15T12:28:13.8042716Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:37:67
2019-08-15T12:28:13.8042806Z    |
2019-08-15T12:28:13.8042806Z    |
2019-08-15T12:28:13.8043074Z LL |       async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8043368Z    |  _________________________________________-_________________________^
2019-08-15T12:28:13.8043449Z    | |                                         |
2019-08-15T12:28:13.8043737Z    | |                                         lifetime `'_` defined here
2019-08-15T12:28:13.8044014Z    | |                                         lifetime `'_` defined here
2019-08-15T12:28:13.8044111Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8044176Z LL | |     }
2019-08-15T12:28:13.8044501Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8046349Z error: aborting due to 12 previous errors
2019-08-15T12:28:13.8046478Z 
2019-08-15T12:28:13.8047172Z For more information about this error, try `rustc --explain E0700`.
2019-08-15T12:28:13.8047271Z 
2019-08-15T12:28:13.8047271Z 
2019-08-15T12:28:13.8047561Z ------------------------------------------
2019-08-15T12:28:13.8047603Z 
2019-08-15T12:28:13.8047659Z 
2019-08-15T12:28:13.8052497Z ---- [ui (nll)] ui/self/elision/ref-mut-self-async.rs stdout ----
2019-08-15T12:28:13.8052616Z diff of stderr:
2019-08-15T12:28:13.8052657Z 
2019-08-15T12:28:13.8052948Z 4 LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-08-15T12:28:13.8053116Z 6    |
2019-08-15T12:28:13.8053116Z 6    |
2019-08-15T12:28:13.8059201Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-15T12:28:13.8059655Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8059860Z 9 error: lifetime may not live long enough
2019-08-15T12:28:13.8060156Z 10   --> $DIR/ref-mut-self-async.rs:15:51
2019-08-15T12:28:13.8060417Z 
2019-08-15T12:28:13.8060417Z 
2019-08-15T12:28:13.8060801Z 24 LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-15T12:28:13.8060998Z 26    |
2019-08-15T12:28:13.8060998Z 26    |
2019-08-15T12:28:13.8061323Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-15T12:28:13.8061642Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8062243Z 29 error: lifetime may not live long enough
2019-08-15T12:28:13.8062795Z 30   --> $DIR/ref-mut-self-async.rs:21:57
2019-08-15T12:28:13.8062872Z 
2019-08-15T12:28:13.8062872Z 
2019-08-15T12:28:13.8063220Z 44 LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8063425Z 46    |
2019-08-15T12:28:13.8063425Z 46    |
2019-08-15T12:28:13.8063763Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-15T12:28:13.8064150Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8064328Z 49 error: lifetime may not live long enough
2019-08-15T12:28:13.8064622Z 50   --> $DIR/ref-mut-self-async.rs:25:66
2019-08-15T12:28:13.8064672Z 
2019-08-15T12:28:13.8064672Z 
2019-08-15T12:28:13.8064988Z 64 LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8065184Z 66    |
2019-08-15T12:28:13.8065184Z 66    |
2019-08-15T12:28:13.8065485Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-15T12:28:13.8065819Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8065990Z 69 error: lifetime may not live long enough
2019-08-15T12:28:13.8066269Z 70   --> $DIR/ref-mut-self-async.rs:29:66
2019-08-15T12:28:13.8066338Z 
2019-08-15T12:28:13.8066338Z 
2019-08-15T12:28:13.8066665Z 84 LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8066870Z 86    |
2019-08-15T12:28:13.8066870Z 86    |
2019-08-15T12:28:13.8067187Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-15T12:28:13.8067503Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8067672Z 89 error: lifetime may not live long enough
2019-08-15T12:28:13.8067959Z 90   --> $DIR/ref-mut-self-async.rs:33:75
2019-08-15T12:28:13.8068008Z 
2019-08-15T12:28:13.8068008Z 
2019-08-15T12:28:13.8068342Z 104 LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8068612Z 106    |
2019-08-15T12:28:13.8068612Z 106    |
2019-08-15T12:28:13.8069064Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-15T12:28:13.8069557Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8069730Z 109 error: lifetime may not live long enough
2019-08-15T12:28:13.8070007Z 110   --> $DIR/ref-mut-self-async.rs:37:75
2019-08-15T12:28:13.8070077Z 
2019-08-15T12:28:13.8070116Z 
2019-08-15T12:28:13.8070116Z 
2019-08-15T12:28:13.8070187Z The actual stderr differed from the expected stderr.
2019-08-15T12:28:13.8070610Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/ref-mut-self-async.nll.stderr
2019-08-15T12:28:13.8070967Z To update references, rerun the tests and pass the `--bless` flag
2019-08-15T12:28:13.8071317Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`
2019-08-15T12:28:13.8071468Z error: 1 errors occurred comparing output.
2019-08-15T12:28:13.8071552Z status: exit code: 1
2019-08-15T12:28:13.8071552Z status: exit code: 1
2019-08-15T12:28:13.8073140Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/auxiliary" "-A" "unused"
2019-08-15T12:28:13.8073964Z ------------------------------------------
2019-08-15T12:28:13.8074020Z 
2019-08-15T12:28:13.8074285Z ------------------------------------------
2019-08-15T12:28:13.8074382Z stderr:
2019-08-15T12:28:13.8074382Z stderr:
2019-08-15T12:28:13.8074638Z ------------------------------------------
2019-08-15T12:28:13.8074765Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8075207Z    |
2019-08-15T12:28:13.8075207Z    |
2019-08-15T12:28:13.8075539Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-15T12:28:13.8075730Z    |
2019-08-15T12:28:13.8075730Z    |
2019-08-15T12:28:13.8076030Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8076174Z error: lifetime may not live long enough
2019-08-15T12:28:13.8076474Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:15:51
2019-08-15T12:28:13.8076574Z    |
2019-08-15T12:28:13.8076574Z    |
2019-08-15T12:28:13.8076884Z LL |       async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-15T12:28:13.8077204Z    |  _______________________-___________________________^
2019-08-15T12:28:13.8077294Z    | |                       |
2019-08-15T12:28:13.8077598Z    | |                       lifetime `'_` defined here
2019-08-15T12:28:13.8077895Z    | |                       lifetime `'_` defined here
2019-08-15T12:28:13.8078077Z LL | |     }
2019-08-15T12:28:13.8078077Z LL | |     }
2019-08-15T12:28:13.8078426Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8078498Z 
2019-08-15T12:28:13.8109027Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8109684Z    |
2019-08-15T12:28:13.8109684Z    |
2019-08-15T12:28:13.8110009Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-15T12:28:13.8110234Z    |
2019-08-15T12:28:13.8110234Z    |
2019-08-15T12:28:13.8110712Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8110898Z error: lifetime may not live long enough
2019-08-15T12:28:13.8111277Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:21:57
2019-08-15T12:28:13.8111361Z    |
2019-08-15T12:28:13.8111361Z    |
2019-08-15T12:28:13.8112058Z LL |       async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-15T12:28:13.8112442Z    |  _____________________________-___________________________^
2019-08-15T12:28:13.8112550Z    | |                             |
2019-08-15T12:28:13.8113088Z    | |                             lifetime `'_` defined here
2019-08-15T12:28:13.8113422Z    | |                             lifetime `'_` defined here
2019-08-15T12:28:13.8113508Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8113596Z LL | |     }
2019-08-15T12:28:13.8113944Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8114026Z 
2019-08-15T12:28:13.8114126Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8114736Z    |
2019-08-15T12:28:13.8114736Z    |
2019-08-15T12:28:13.8115053Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8115245Z    |
2019-08-15T12:28:13.8115245Z    |
2019-08-15T12:28:13.8115543Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8115685Z error: lifetime may not live long enough
2019-08-15T12:28:13.8116461Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:25:66
2019-08-15T12:28:13.8116557Z    |
2019-08-15T12:28:13.8116557Z    |
2019-08-15T12:28:13.8116881Z LL |       async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8117193Z    |  _____________________________________-____________________________^
2019-08-15T12:28:13.8117314Z    | |                                     |
2019-08-15T12:28:13.8117621Z    | |                                     lifetime `'_` defined here
2019-08-15T12:28:13.8117958Z    | |                                     lifetime `'_` defined here
2019-08-15T12:28:13.8118046Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8118128Z LL | |     }
2019-08-15T12:28:13.8118474Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8118557Z 
2019-08-15T12:28:13.8118642Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8119053Z    |
2019-08-15T12:28:13.8119053Z    |
2019-08-15T12:28:13.8119367Z LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8119557Z    |
2019-08-15T12:28:13.8119557Z    |
2019-08-15T12:28:13.8119866Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-15T12:28:13.8120016Z error: lifetime may not live long enough
2019-08-15T12:28:13.8120324Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:29:66
2019-08-15T12:28:13.8120418Z    |
2019-08-15T12:28:13.8120418Z    |
2019-08-15T12:28:13.8120715Z LL |       async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8121039Z    |  _____________________________________-____________________________^
2019-08-15T12:28:13.8121127Z    | |                                     |
2019-08-15T12:28:13.8121436Z    | |                                     lifetime `'_` defined here
2019-08-15T12:28:13.8122263Z    | |                                     lifetime `'_` defined here
2019-08-15T12:28:13.8122380Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-15T12:28:13.8122454Z LL | |     }
2019-08-15T12:28:13.8123214Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-15T12:28:13.8123305Z 
2019-08-15T12:28:13.8123415Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-15T12:28:13.8123883Z    |
2019-08-15T12:28:13.8123883Z    |
2019-08-15T12:28:13.8124192Z LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-15T12:28:13.8124392Z    |
2019-08-15T12:28:13.8124392Z    |
2019-08-15T12:28:13.8124700Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
---
2019-08-15T12:28:13.8260123Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-15T12:28:13.8260247Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-15T12:28:13.8260306Z 
2019-08-15T12:28:13.8260343Z 
2019-08-15T12:28:13.8262962Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-15T12:28:13.8263825Z 
2019-08-15T12:28:13.8263868Z 
2019-08-15T12:28:13.8264044Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-15T12:28:13.8264138Z Build completed unsuccessfully in 1:58:17
2019-08-15T12:28:13.8264138Z Build completed unsuccessfully in 1:58:17
2019-08-15T12:28:13.8264231Z == clock drift check ==
2019-08-15T12:28:13.8264303Z   local time: Thu Aug 15 12:28:13 UTC 2019
2019-08-15T12:28:13.8809211Z   network time: Thu, 15 Aug 2019 12:28:13 GMT
2019-08-15T12:28:13.8809357Z == end clock drift check ==
2019-08-15T12:28:14.7318987Z ##[error]Bash exited with code '1'.
2019-08-15T12:28:14.7360168Z ##[section]Starting: Upload CPU usage statistics
2019-08-15T12:28:14.7369355Z ==============================================================================
2019-08-15T12:28:14.7369478Z Task         : Bash
2019-08-15T12:28:14.7369555Z Description  : Run a Bash script on macOS, Linux, or Windows
