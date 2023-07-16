plain
2019-08-14T18:45:18.6038607Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T18:45:18.6038908Z 
2019-08-14T18:45:18.6039429Z   git checkout -b <new-branch-name>
2019-08-14T18:45:18.6039709Z 
2019-08-14T18:45:18.6040275Z HEAD is now at 53b4f1d1d Auto merge of #63571 - Centril:rollup-v2rciiu, r=Centril
2019-08-14T18:45:18.6198774Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T18:45:18.6201797Z ==============================================================================
2019-08-14T18:45:18.6202337Z Task         : Bash
2019-08-14T18:45:18.6202413Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T20:46:52.5035507Z 
2019-08-14T20:46:52.5042036Z ---- [ui (nll)] ui/async-await/issues/issue-63388-1.rs stdout ----
2019-08-14T20:46:52.5042174Z diff of stderr:
2019-08-14T20:46:52.5042222Z 
2019-08-14T20:46:52.5042493Z 4 LL |     ) -> &dyn Foo
2019-08-14T20:46:52.5042640Z 6    |
2019-08-14T20:46:52.5042640Z 6    |
2019-08-14T20:46:52.5042920Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#27r
2019-08-14T20:46:52.5043686Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#22r
2019-08-14T20:46:52.5043866Z 9 error: lifetime may not live long enough
2019-08-14T20:46:52.5044123Z 10   --> $DIR/issue-63388-1.rs:15:5
2019-08-14T20:46:52.5044183Z 
2019-08-14T20:46:52.5044217Z 
2019-08-14T20:46:52.5044217Z 
2019-08-14T20:46:52.5044283Z The actual stderr differed from the expected stderr.
2019-08-14T20:46:52.5044665Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/issue-63388-1.nll.stderr
2019-08-14T20:46:52.5045137Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T20:46:52.5045525Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-1.rs`
2019-08-14T20:46:52.5045662Z error: 1 errors occurred comparing output.
2019-08-14T20:46:52.5045729Z status: exit code: 1
2019-08-14T20:46:52.5045729Z status: exit code: 1
2019-08-14T20:46:52.5046671Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/auxiliary" "-A" "unused"
2019-08-14T20:46:52.5047339Z ------------------------------------------
2019-08-14T20:46:52.5047405Z 
2019-08-14T20:46:52.5047642Z ------------------------------------------
2019-08-14T20:46:52.5047724Z stderr:
2019-08-14T20:46:52.5047724Z stderr:
2019-08-14T20:46:52.5047953Z ------------------------------------------
2019-08-14T20:46:52.5048052Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5048344Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:14:10
2019-08-14T20:46:52.5048446Z    |
2019-08-14T20:46:52.5048690Z LL |     ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-14T20:46:52.5048837Z    |
2019-08-14T20:46:52.5048837Z    |
2019-08-14T20:46:52.5049121Z    = note: hidden type `impl std::future::Future` captures lifetime '_#22r
2019-08-14T20:46:52.5049248Z error: lifetime may not live long enough
2019-08-14T20:46:52.5049519Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:15:5
2019-08-14T20:46:52.5051516Z    |
2019-08-14T20:46:52.5051516Z    |
2019-08-14T20:46:52.5051840Z LL |       async fn do_sth<'a>(
2019-08-14T20:46:52.5052531Z    |                       -- lifetime `'a` defined here
2019-08-14T20:46:52.5052777Z LL |           &'a self, foo: &dyn Foo
2019-08-14T20:46:52.5053048Z    |                          - lifetime `'_` defined here
2019-08-14T20:46:52.5053305Z LL |       ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-14T20:46:52.5053462Z LL | |         foo
2019-08-14T20:46:52.5053539Z LL | |     }
2019-08-14T20:46:52.5053539Z LL | |     }
2019-08-14T20:46:52.5053851Z    | |_____^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5053991Z error: aborting due to 2 previous errors
2019-08-14T20:46:52.5054033Z 
2019-08-14T20:46:52.5054310Z For more information about this error, try `rustc --explain E0700`.
2019-08-14T20:46:52.5054362Z 
2019-08-14T20:46:52.5054362Z 
2019-08-14T20:46:52.5054660Z ------------------------------------------
2019-08-14T20:46:52.5058358Z 
2019-08-14T20:46:52.5058419Z 
2019-08-14T20:46:52.5062484Z ---- [ui (nll)] ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs stdout ----
2019-08-14T20:46:52.5062591Z diff of stderr:
2019-08-14T20:46:52.5062653Z 
2019-08-14T20:46:52.5062944Z 4 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-14T20:46:52.5063110Z 6    |
2019-08-14T20:46:52.5063110Z 6    |
2019-08-14T20:46:52.5063425Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T20:46:52.5063714Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5063871Z 9 error: lifetime may not live long enough
2019-08-14T20:46:52.5064200Z 10   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-14T20:46:52.5064259Z 
2019-08-14T20:46:52.5064259Z 
2019-08-14T20:46:52.5064571Z 30 LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-08-14T20:46:52.5064914Z 32    |
2019-08-14T20:46:52.5064914Z 32    |
2019-08-14T20:46:52.5065250Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T20:46:52.5065576Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5065738Z 35 error: lifetime may not live long enough
2019-08-14T20:46:52.5066036Z 36   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-14T20:46:52.5066207Z 
2019-08-14T20:46:52.5066245Z 
2019-08-14T20:46:52.5066245Z 
2019-08-14T20:46:52.5066315Z The actual stderr differed from the expected stderr.
2019-08-14T20:46:52.5066792Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/arbitrary_self_types_pin_lifetime_mismatch-async.nll.stderr
2019-08-14T20:46:52.5067149Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T20:46:52.5067519Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`
2019-08-14T20:46:52.5067665Z error: 1 errors occurred comparing output.
2019-08-14T20:46:52.5067734Z status: exit code: 1
2019-08-14T20:46:52.5067734Z status: exit code: 1
2019-08-14T20:46:52.5068825Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/auxiliary" "-A" "unused"
2019-08-14T20:46:52.5069407Z ------------------------------------------
2019-08-14T20:46:52.5069457Z 
2019-08-14T20:46:52.5069713Z ------------------------------------------
2019-08-14T20:46:52.5069799Z stderr:
2019-08-14T20:46:52.5069799Z stderr:
2019-08-14T20:46:52.5070048Z ------------------------------------------
2019-08-14T20:46:52.5070152Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5070596Z    |
2019-08-14T20:46:52.5070596Z    |
2019-08-14T20:46:52.5070889Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-14T20:46:52.5071055Z    |
2019-08-14T20:46:52.5071055Z    |
2019-08-14T20:46:52.5071735Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5071877Z error: lifetime may not live long enough
2019-08-14T20:46:52.5072214Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-14T20:46:52.5072315Z    |
2019-08-14T20:46:52.5072315Z    |
2019-08-14T20:46:52.5072590Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-14T20:46:52.5073001Z    |                          -                       ^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5073123Z    |                          |
2019-08-14T20:46:52.5073407Z    |                          lifetime `'_` defined here
2019-08-14T20:46:52.5073713Z    |                          lifetime `'_` defined here
2019-08-14T20:46:52.5073838Z error: lifetime may not live long enough
2019-08-14T20:46:52.5074158Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:13:73
2019-08-14T20:46:52.5074256Z    |
2019-08-14T20:46:52.5074256Z    |
2019-08-14T20:46:52.5074560Z LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-14T20:46:52.5075123Z    |                          -                                              ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5075250Z    |                          |
2019-08-14T20:46:52.5075573Z    |                          lifetime `'_` defined here
2019-08-14T20:46:52.5075856Z    |                          lifetime `'_` defined here
2019-08-14T20:46:52.5075924Z 
2019-08-14T20:46:52.5076085Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5076546Z    |
2019-08-14T20:46:52.5076546Z    |
2019-08-14T20:46:52.5077192Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-14T20:46:52.5077380Z    |
2019-08-14T20:46:52.5077380Z    |
2019-08-14T20:46:52.5077689Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5077823Z error: lifetime may not live long enough
2019-08-14T20:46:52.5078156Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-14T20:46:52.5078240Z    |
2019-08-14T20:46:52.5078240Z    |
2019-08-14T20:46:52.5078564Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-14T20:46:52.5078980Z    |                  --              -                           ^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-08-14T20:46:52.5079112Z    |                  |               |
2019-08-14T20:46:52.5079402Z    |                  |               lifetime `'_` defined here
2019-08-14T20:46:52.5079745Z 
2019-08-14T20:46:52.5079820Z error: aborting due to 5 previous errors
2019-08-14T20:46:52.5079864Z 
2019-08-14T20:46:52.5084798Z For more information about this error, try `rustc --explain E0700`.
2019-08-14T20:46:52.5084798Z For more information about this error, try `rustc --explain E0700`.
2019-08-14T20:46:52.5084933Z 
2019-08-14T20:46:52.5085327Z ------------------------------------------
2019-08-14T20:46:52.5085378Z 
2019-08-14T20:46:52.5085427Z 
2019-08-14T20:46:52.5085695Z ---- [ui (nll)] ui/self/elision/lt-ref-self-async.rs stdout ----
2019-08-14T20:46:52.5085790Z diff of stderr:
2019-08-14T20:46:52.5085828Z 
2019-08-14T20:46:52.5086099Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5086274Z 6    |
2019-08-14T20:46:52.5086274Z 6    |
2019-08-14T20:46:52.5086550Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T20:46:52.5086853Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5087010Z 9 error: lifetime may not live long enough
2019-08-14T20:46:52.5087274Z 10   --> $DIR/lt-ref-self-async.rs:15:47
2019-08-14T20:46:52.5087321Z 
2019-08-14T20:46:52.5087321Z 
2019-08-14T20:46:52.5087622Z 24 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5087799Z 26    |
2019-08-14T20:46:52.5087799Z 26    |
2019-08-14T20:46:52.5088092Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T20:46:52.5088419Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5088586Z 29 error: lifetime may not live long enough
2019-08-14T20:46:52.5088867Z 30   --> $DIR/lt-ref-self-async.rs:21:53
2019-08-14T20:46:52.5088915Z 
2019-08-14T20:46:52.5088915Z 
2019-08-14T20:46:52.5089199Z 44 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5089380Z 46    |
2019-08-14T20:46:52.5089380Z 46    |
2019-08-14T20:46:52.5089857Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T20:46:52.5090339Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5090515Z 49 error: lifetime may not live long enough
2019-08-14T20:46:52.5090829Z 50   --> $DIR/lt-ref-self-async.rs:25:62
2019-08-14T20:46:52.5090878Z 
2019-08-14T20:46:52.5090878Z 
2019-08-14T20:46:52.5091475Z 64 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5091790Z 66    |
2019-08-14T20:46:52.5091790Z 66    |
2019-08-14T20:46:52.5092120Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T20:46:52.5092425Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5092574Z 69 error: lifetime may not live long enough
2019-08-14T20:46:52.5092818Z 70   --> $DIR/lt-ref-self-async.rs:29:62
2019-08-14T20:46:52.5092880Z 
2019-08-14T20:46:52.5092880Z 
2019-08-14T20:46:52.5096570Z 84 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5096796Z 86    |
2019-08-14T20:46:52.5096796Z 86    |
2019-08-14T20:46:52.5097106Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T20:46:52.5097395Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5097545Z 89 error: lifetime may not live long enough
2019-08-14T20:46:52.5097816Z 90   --> $DIR/lt-ref-self-async.rs:33:71
2019-08-14T20:46:52.5097863Z 
2019-08-14T20:46:52.5097863Z 
2019-08-14T20:46:52.5098152Z 104 LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5098328Z 106    |
2019-08-14T20:46:52.5098328Z 106    |
2019-08-14T20:46:52.5098598Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T20:46:52.5098898Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5099056Z 109 error: lifetime may not live long enough
2019-08-14T20:46:52.5099305Z 110   --> $DIR/lt-ref-self-async.rs:37:67
2019-08-14T20:46:52.5099366Z 
2019-08-14T20:46:52.5099400Z 
2019-08-14T20:46:52.5099400Z 
2019-08-14T20:46:52.5099462Z The actual stderr differed from the expected stderr.
2019-08-14T20:46:52.5099837Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/lt-ref-self-async.nll.stderr
2019-08-14T20:46:52.5100146Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T20:46:52.5100470Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`
2019-08-14T20:46:52.5100604Z error: 1 errors occurred comparing output.
2019-08-14T20:46:52.5100671Z status: exit code: 1
2019-08-14T20:46:52.5100671Z status: exit code: 1
2019-08-14T20:46:52.5103103Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/auxiliary" "-A" "unused"
2019-08-14T20:46:52.5103678Z ------------------------------------------
2019-08-14T20:46:52.5103747Z 
2019-08-14T20:46:52.5104006Z ------------------------------------------
2019-08-14T20:46:52.5104091Z stderr:
2019-08-14T20:46:52.5104091Z stderr:
2019-08-14T20:46:52.5104341Z ------------------------------------------
2019-08-14T20:46:52.5104445Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5104853Z    |
2019-08-14T20:46:52.5104853Z    |
2019-08-14T20:46:52.5105268Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5105450Z    |
2019-08-14T20:46:52.5105450Z    |
2019-08-14T20:46:52.5105799Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5105933Z error: lifetime may not live long enough
2019-08-14T20:46:52.5106226Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:15:47
2019-08-14T20:46:52.5106412Z    |
2019-08-14T20:46:52.5106412Z    |
2019-08-14T20:46:52.5106709Z LL |       async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5107002Z    |  _______________________-_______________________^
2019-08-14T20:46:52.5107078Z    | |                       |
2019-08-14T20:46:52.5107365Z    | |                       lifetime `'_` defined here
2019-08-14T20:46:52.5107643Z    | |                       lifetime `'_` defined here
2019-08-14T20:46:52.5107737Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5107815Z LL | |     }
2019-08-14T20:46:52.5108164Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5108232Z 
2019-08-14T20:46:52.5108325Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5108736Z    |
2019-08-14T20:46:52.5108736Z    |
2019-08-14T20:46:52.5109014Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5109182Z    |
2019-08-14T20:46:52.5109182Z    |
2019-08-14T20:46:52.5109486Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5109606Z error: lifetime may not live long enough
2019-08-14T20:46:52.5109910Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:21:53
2019-08-14T20:46:52.5109986Z    |
2019-08-14T20:46:52.5109986Z    |
2019-08-14T20:46:52.5110286Z LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5110574Z    |  _____________________________-_______________________^
2019-08-14T20:46:52.5110668Z    | |                             |
2019-08-14T20:46:52.5110959Z    | |                             lifetime `'_` defined here
2019-08-14T20:46:52.5111541Z    | |                             lifetime `'_` defined here
2019-08-14T20:46:52.5111647Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5111725Z LL | |     }
2019-08-14T20:46:52.5112065Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5112133Z 
2019-08-14T20:46:52.5112208Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5112584Z    |
2019-08-14T20:46:52.5112584Z    |
2019-08-14T20:46:52.5112868Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5113039Z    |
2019-08-14T20:46:52.5113039Z    |
2019-08-14T20:46:52.5113327Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5113441Z error: lifetime may not live long enough
2019-08-14T20:46:52.5113727Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:25:62
2019-08-14T20:46:52.5113809Z    |
2019-08-14T20:46:52.5113809Z    |
2019-08-14T20:46:52.5114087Z LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5114362Z    |  _____________________________________-________________________^
2019-08-14T20:46:52.5114457Z    | |                                     |
2019-08-14T20:46:52.5114726Z    | |                                     lifetime `'_` defined here
2019-08-14T20:46:52.5115017Z    | |                                     lifetime `'_` defined here
2019-08-14T20:46:52.5115110Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5115290Z LL | |     }
2019-08-14T20:46:52.5115654Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5115721Z 
2019-08-14T20:46:52.5115795Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5116249Z    |
2019-08-14T20:46:52.5116249Z    |
2019-08-14T20:46:52.5116552Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5116720Z    |
2019-08-14T20:46:52.5116720Z    |
2019-08-14T20:46:52.5116987Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5117115Z error: lifetime may not live long enough
2019-08-14T20:46:52.5117396Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:29:62
2019-08-14T20:46:52.5117467Z    |
2019-08-14T20:46:52.5117467Z    |
2019-08-14T20:46:52.5117753Z LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5118029Z    |  _____________________________________-________________________^
2019-08-14T20:46:52.5118124Z    | |                                     |
2019-08-14T20:46:52.5118389Z    | |                                     lifetime `'_` defined here
2019-08-14T20:46:52.5118676Z    | |                                     lifetime `'_` defined here
2019-08-14T20:46:52.5118767Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5118846Z LL | |     }
2019-08-14T20:46:52.5119158Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5119240Z 
2019-08-14T20:46:52.5119313Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5119682Z    |
2019-08-14T20:46:52.5119682Z    |
2019-08-14T20:46:52.5119974Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5120151Z    |
2019-08-14T20:46:52.5120151Z    |
2019-08-14T20:46:52.5120421Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5120548Z error: lifetime may not live long enough
2019-08-14T20:46:52.5120841Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:33:71
2019-08-14T20:46:52.5120914Z    |
2019-08-14T20:46:52.5120914Z    |
2019-08-14T20:46:52.5121560Z LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5121866Z    |  _____________________________________________-_________________________^
2019-08-14T20:46:52.5121970Z    | |                                             |
2019-08-14T20:46:52.5122250Z    | |                                             lifetime `'_` defined here
2019-08-14T20:46:52.5122565Z    | |                                             lifetime `'_` defined here
2019-08-14T20:46:52.5122647Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5122728Z LL | |     }
2019-08-14T20:46:52.5123036Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5123117Z 
2019-08-14T20:46:52.5123190Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5123571Z    |
2019-08-14T20:46:52.5123571Z    |
2019-08-14T20:46:52.5123852Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5124024Z    |
2019-08-14T20:46:52.5124024Z    |
2019-08-14T20:46:52.5124290Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T20:46:52.5124417Z error: lifetime may not live long enough
2019-08-14T20:46:52.5124808Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:37:67
2019-08-14T20:46:52.5124892Z    |
2019-08-14T20:46:52.5124892Z    |
2019-08-14T20:46:52.5125208Z LL |       async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5125492Z    |  _________________________________________-_________________________^
2019-08-14T20:46:52.5125591Z    | |                                         |
2019-08-14T20:46:52.5125961Z    | |                                         lifetime `'_` defined here
2019-08-14T20:46:52.5126262Z    | |                                         lifetime `'_` defined here
2019-08-14T20:46:52.5126343Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5126423Z LL | |     }
2019-08-14T20:46:52.5126733Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5126877Z error: aborting due to 12 previous errors
2019-08-14T20:46:52.5126933Z 
2019-08-14T20:46:52.5127202Z For more information about this error, try `rustc --explain E0700`.
2019-08-14T20:46:52.5127253Z 
2019-08-14T20:46:52.5127253Z 
2019-08-14T20:46:52.5127502Z ------------------------------------------
2019-08-14T20:46:52.5127547Z 
2019-08-14T20:46:52.5127580Z 
2019-08-14T20:46:52.5127850Z ---- [ui (nll)] ui/self/elision/ref-mut-self-async.rs stdout ----
2019-08-14T20:46:52.5127924Z diff of stderr:
2019-08-14T20:46:52.5127978Z 
2019-08-14T20:46:52.5128242Z 4 LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5128408Z 6    |
2019-08-14T20:46:52.5128408Z 6    |
2019-08-14T20:46:52.5128691Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T20:46:52.5128976Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5129128Z 9 error: lifetime may not live long enough
2019-08-14T20:46:52.5129388Z 10   --> $DIR/ref-mut-self-async.rs:15:51
2019-08-14T20:46:52.5129441Z 
2019-08-14T20:46:52.5129441Z 
2019-08-14T20:46:52.5129935Z 24 LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5130117Z 26    |
2019-08-14T20:46:52.5130117Z 26    |
2019-08-14T20:46:52.5130395Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T20:46:52.5130697Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5130862Z 29 error: lifetime may not live long enough
2019-08-14T20:46:52.5131441Z 30   --> $DIR/ref-mut-self-async.rs:21:57
2019-08-14T20:46:52.5131514Z 
2019-08-14T20:46:52.5131514Z 
2019-08-14T20:46:52.5131841Z 44 LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5132018Z 46    |
2019-08-14T20:46:52.5132018Z 46    |
2019-08-14T20:46:52.5132305Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T20:46:52.5132606Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5132759Z 49 error: lifetime may not live long enough
2019-08-14T20:46:52.5133021Z 50   --> $DIR/ref-mut-self-async.rs:25:66
2019-08-14T20:46:52.5133066Z 
2019-08-14T20:46:52.5133066Z 
2019-08-14T20:46:52.5133350Z 64 LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5133536Z 66    |
2019-08-14T20:46:52.5133536Z 66    |
2019-08-14T20:46:52.5133806Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T20:46:52.5134108Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5134259Z 69 error: lifetime may not live long enough
2019-08-14T20:46:52.5134501Z 70   --> $DIR/ref-mut-self-async.rs:29:66
2019-08-14T20:46:52.5134546Z 
2019-08-14T20:46:52.5134546Z 
2019-08-14T20:46:52.5134969Z 84 LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5135166Z 86    |
2019-08-14T20:46:52.5135166Z 86    |
2019-08-14T20:46:52.5135481Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T20:46:52.5135769Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5136003Z 89 error: lifetime may not live long enough
2019-08-14T20:46:52.5136284Z 90   --> $DIR/ref-mut-self-async.rs:33:75
2019-08-14T20:46:52.5136329Z 
2019-08-14T20:46:52.5136329Z 
2019-08-14T20:46:52.5136611Z 104 LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5136796Z 106    |
2019-08-14T20:46:52.5136796Z 106    |
2019-08-14T20:46:52.5137079Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T20:46:52.5137370Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5137537Z 109 error: lifetime may not live long enough
2019-08-14T20:46:52.5137782Z 110   --> $DIR/ref-mut-self-async.rs:37:75
2019-08-14T20:46:52.5137827Z 
2019-08-14T20:46:52.5137876Z 
2019-08-14T20:46:52.5137876Z 
2019-08-14T20:46:52.5137939Z The actual stderr differed from the expected stderr.
2019-08-14T20:46:52.5138312Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/ref-mut-self-async.nll.stderr
2019-08-14T20:46:52.5138624Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T20:46:52.5138948Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`
2019-08-14T20:46:52.5139083Z error: 1 errors occurred comparing output.
2019-08-14T20:46:52.5139149Z status: exit code: 1
2019-08-14T20:46:52.5139149Z status: exit code: 1
2019-08-14T20:46:52.5140093Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/auxiliary" "-A" "unused"
2019-08-14T20:46:52.5140602Z ------------------------------------------
2019-08-14T20:46:52.5140663Z 
2019-08-14T20:46:52.5140900Z ------------------------------------------
2019-08-14T20:46:52.5140981Z stderr:
2019-08-14T20:46:52.5140981Z stderr:
2019-08-14T20:46:52.5141501Z ------------------------------------------
2019-08-14T20:46:52.5141613Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5142016Z    |
2019-08-14T20:46:52.5142016Z    |
2019-08-14T20:46:52.5142300Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-14T20:46:52.5142471Z    |
2019-08-14T20:46:52.5142471Z    |
2019-08-14T20:46:52.5142753Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5142892Z error: lifetime may not live long enough
2019-08-14T20:46:52.5143164Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:15:51
2019-08-14T20:46:52.5143253Z    |
2019-08-14T20:46:52.5143253Z    |
2019-08-14T20:46:52.5143533Z LL |       async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-14T20:46:52.5143817Z    |  _______________________-___________________________^
2019-08-14T20:46:52.5143892Z    | |                       |
2019-08-14T20:46:52.5144268Z    | |                       lifetime `'_` defined here
2019-08-14T20:46:52.5144569Z    | |                       lifetime `'_` defined here
2019-08-14T20:46:52.5144718Z LL | |     }
2019-08-14T20:46:52.5144718Z LL | |     }
2019-08-14T20:46:52.5145042Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5145107Z 
2019-08-14T20:46:52.5145196Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5145666Z    |
2019-08-14T20:46:52.5145666Z    |
2019-08-14T20:46:52.5145924Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5146089Z    |
2019-08-14T20:46:52.5146089Z    |
2019-08-14T20:46:52.5149273Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5149422Z error: lifetime may not live long enough
2019-08-14T20:46:52.5149730Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:21:57
2019-08-14T20:46:52.5149818Z    |
2019-08-14T20:46:52.5149818Z    |
2019-08-14T20:46:52.5150078Z LL |       async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-14T20:46:52.5150361Z    |  _____________________________-___________________________^
2019-08-14T20:46:52.5150438Z    | |                             |
2019-08-14T20:46:52.5150716Z    | |                             lifetime `'_` defined here
2019-08-14T20:46:52.5150993Z    | |                             lifetime `'_` defined here
2019-08-14T20:46:52.5151087Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5151415Z LL | |     }
2019-08-14T20:46:52.5151807Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5151874Z 
2019-08-14T20:46:52.5151950Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5152356Z    |
2019-08-14T20:46:52.5152356Z    |
2019-08-14T20:46:52.5152626Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5152800Z    |
2019-08-14T20:46:52.5152800Z    |
2019-08-14T20:46:52.5153083Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5153206Z error: lifetime may not live long enough
2019-08-14T20:46:52.5153493Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:25:66
2019-08-14T20:46:52.5153566Z    |
2019-08-14T20:46:52.5153566Z    |
2019-08-14T20:46:52.5153848Z LL |       async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5154128Z    |  _____________________________________-____________________________^
2019-08-14T20:46:52.5154224Z    | |                                     |
2019-08-14T20:46:52.5154515Z    | |                                     lifetime `'_` defined here
2019-08-14T20:46:52.5154795Z    | |                                     lifetime `'_` defined here
2019-08-14T20:46:52.5154889Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5154954Z LL | |     }
2019-08-14T20:46:52.5155279Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5155344Z 
2019-08-14T20:46:52.5155417Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5155816Z    |
2019-08-14T20:46:52.5155816Z    |
2019-08-14T20:46:52.5156081Z LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5156252Z    |
2019-08-14T20:46:52.5156252Z    |
2019-08-14T20:46:52.5156539Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T20:46:52.5156796Z error: lifetime may not live long enough
2019-08-14T20:46:52.5157120Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:29:66
2019-08-14T20:46:52.5157193Z    |
2019-08-14T20:46:52.5157193Z    |
2019-08-14T20:46:52.5157478Z LL |       async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5157756Z    |  _____________________________________-____________________________^
2019-08-14T20:46:52.5157958Z    | |                                     |
2019-08-14T20:46:52.5158268Z    | |                                     lifetime `'_` defined here
2019-08-14T20:46:52.5158546Z    | |                                     lifetime `'_` defined here
2019-08-14T20:46:52.5158640Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T20:46:52.5158706Z LL | |     }
2019-08-14T20:46:52.5159032Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T20:46:52.5159098Z 
2019-08-14T20:46:52.5159182Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T20:46:52.5159555Z    |
2019-08-14T20:46:52.5159555Z    |
2019-08-14T20:46:52.5159848Z LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-14T20:46:52.5160039Z    |
2019-08-14T20:46:52.5160039Z    |
2019-08-14T20:46:52.5160322Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
---
2019-08-14T20:46:52.5320916Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-14T20:46:52.5321028Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-14T20:46:52.5321854Z 
2019-08-14T20:46:52.5321955Z 
2019-08-14T20:46:52.5324887Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-14T20:46:52.5325730Z 
2019-08-14T20:46:52.5325784Z 
2019-08-14T20:46:52.5325850Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-14T20:46:52.5325948Z Build completed unsuccessfully in 1:56:53
2019-08-14T20:46:52.5325948Z Build completed unsuccessfully in 1:56:53
2019-08-14T20:46:52.5326018Z == clock drift check ==
2019-08-14T20:46:52.5326097Z   local time: Wed Aug 14 20:46:52 UTC 2019
2019-08-14T20:46:52.7918272Z   network time: Wed, 14 Aug 2019 20:46:52 GMT
2019-08-14T20:46:52.7928634Z == end clock drift check ==
2019-08-14T20:46:53.7700715Z ##[error]Bash exited with code '1'.
2019-08-14T20:46:53.7752237Z ##[section]Starting: Upload CPU usage statistics
2019-08-14T20:46:53.7760378Z ==============================================================================
2019-08-14T20:46:53.7760476Z Task         : Bash
2019-08-14T20:46:53.7760562Z Description  : Run a Bash script on macOS, Linux, or Windows
