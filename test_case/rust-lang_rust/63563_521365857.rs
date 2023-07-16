plain
2019-08-14T16:33:26.5667157Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T16:33:26.5668859Z 
2019-08-14T16:33:26.5670285Z   git checkout -b <new-branch-name>
2019-08-14T16:33:26.5672542Z 
2019-08-14T16:33:26.5674620Z HEAD is now at 6edcad108 Auto merge of #63563 - Centril:rollup-j9nld0c, r=Centril
2019-08-14T16:33:26.5877950Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T16:33:26.5880826Z ==============================================================================
2019-08-14T16:33:26.5880924Z Task         : Bash
2019-08-14T16:33:26.5880992Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T18:33:25.9562060Z 
2019-08-14T18:33:25.9616711Z ---- [ui (nll)] ui/async-await/issues/issue-63388-1.rs stdout ----
2019-08-14T18:33:25.9617159Z diff of stderr:
2019-08-14T18:33:25.9617339Z 
2019-08-14T18:33:25.9617717Z 4 LL |     ) -> &dyn Foo
2019-08-14T18:33:25.9618068Z 6    |
2019-08-14T18:33:25.9618068Z 6    |
2019-08-14T18:33:25.9618454Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#27r
2019-08-14T18:33:25.9619180Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#22r
2019-08-14T18:33:25.9619654Z 9 error: lifetime may not live long enough
2019-08-14T18:33:25.9619997Z 10   --> $DIR/issue-63388-1.rs:15:5
2019-08-14T18:33:25.9620151Z 
2019-08-14T18:33:25.9620297Z 
2019-08-14T18:33:25.9620297Z 
2019-08-14T18:33:25.9620450Z The actual stderr differed from the expected stderr.
2019-08-14T18:33:25.9621352Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/issue-63388-1.nll.stderr
2019-08-14T18:33:25.9621908Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:33:25.9622393Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-1.rs`
2019-08-14T18:33:25.9622759Z error: 1 errors occurred comparing output.
2019-08-14T18:33:25.9622944Z status: exit code: 1
2019-08-14T18:33:25.9622944Z status: exit code: 1
2019-08-14T18:33:25.9624233Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/auxiliary" "-A" "unused"
2019-08-14T18:33:25.9625405Z ------------------------------------------
2019-08-14T18:33:25.9625572Z 
2019-08-14T18:33:25.9626117Z ------------------------------------------
2019-08-14T18:33:25.9626302Z stderr:
2019-08-14T18:33:25.9626302Z stderr:
2019-08-14T18:33:25.9626644Z ------------------------------------------
2019-08-14T18:33:25.9626859Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9627252Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:14:10
2019-08-14T18:33:25.9627463Z    |
2019-08-14T18:33:25.9627823Z LL |     ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-14T18:33:25.9628172Z    |
2019-08-14T18:33:25.9628172Z    |
2019-08-14T18:33:25.9628551Z    = note: hidden type `impl std::future::Future` captures lifetime '_#22r
2019-08-14T18:33:25.9628877Z error: lifetime may not live long enough
2019-08-14T18:33:25.9629233Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:15:5
2019-08-14T18:33:25.9629607Z    |
2019-08-14T18:33:25.9629607Z    |
2019-08-14T18:33:25.9629975Z LL |       async fn do_sth<'a>(
2019-08-14T18:33:25.9630352Z    |                       -- lifetime `'a` defined here
2019-08-14T18:33:25.9630755Z LL |           &'a self, foo: &dyn Foo
2019-08-14T18:33:25.9631567Z    |                          - lifetime `'_` defined here
2019-08-14T18:33:25.9632083Z LL |       ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-14T18:33:25.9632484Z LL | |         foo
2019-08-14T18:33:25.9632647Z LL | |     }
2019-08-14T18:33:25.9632647Z LL | |     }
2019-08-14T18:33:25.9633111Z    | |_____^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9633481Z error: aborting due to 2 previous errors
2019-08-14T18:33:25.9633618Z 
2019-08-14T18:33:25.9634026Z For more information about this error, try `rustc --explain E0700`.
2019-08-14T18:33:25.9634201Z 
2019-08-14T18:33:25.9634201Z 
2019-08-14T18:33:25.9634572Z ------------------------------------------
2019-08-14T18:33:25.9634912Z 
2019-08-14T18:33:25.9635237Z 
2019-08-14T18:33:25.9635626Z ---- [ui (nll)] ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs stdout ----
2019-08-14T18:33:25.9635832Z diff of stderr:
2019-08-14T18:33:25.9635953Z 
2019-08-14T18:33:25.9636476Z 4 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-14T18:33:25.9636871Z 6    |
2019-08-14T18:33:25.9636871Z 6    |
2019-08-14T18:33:25.9637252Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T18:33:25.9645823Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9645987Z 9 error: lifetime may not live long enough
2019-08-14T18:33:25.9646252Z 10   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-14T18:33:25.9646320Z 
2019-08-14T18:33:25.9646320Z 
2019-08-14T18:33:25.9646568Z 30 LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-08-14T18:33:25.9646791Z 32    |
2019-08-14T18:33:25.9646791Z 32    |
2019-08-14T18:33:25.9647068Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T18:33:25.9647335Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9647482Z 35 error: lifetime may not live long enough
2019-08-14T18:33:25.9648072Z 36   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-14T18:33:25.9648135Z 
2019-08-14T18:33:25.9648166Z 
2019-08-14T18:33:25.9648166Z 
2019-08-14T18:33:25.9648245Z The actual stderr differed from the expected stderr.
2019-08-14T18:33:25.9648664Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/arbitrary_self_types_pin_lifetime_mismatch-async.nll.stderr
2019-08-14T18:33:25.9648963Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:33:25.9649290Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`
2019-08-14T18:33:25.9649406Z error: 1 errors occurred comparing output.
2019-08-14T18:33:25.9649486Z status: exit code: 1
2019-08-14T18:33:25.9649486Z status: exit code: 1
2019-08-14T18:33:25.9650423Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/auxiliary" "-A" "unused"
2019-08-14T18:33:25.9651377Z ------------------------------------------
2019-08-14T18:33:25.9651432Z 
2019-08-14T18:33:25.9651708Z ------------------------------------------
2019-08-14T18:33:25.9651778Z stderr:
2019-08-14T18:33:25.9651778Z stderr:
2019-08-14T18:33:25.9652021Z ------------------------------------------
2019-08-14T18:33:25.9652108Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9652551Z    |
2019-08-14T18:33:25.9652551Z    |
2019-08-14T18:33:25.9652805Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-14T18:33:25.9652973Z    |
2019-08-14T18:33:25.9652973Z    |
2019-08-14T18:33:25.9653257Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9653374Z error: lifetime may not live long enough
2019-08-14T18:33:25.9653694Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-14T18:33:25.9653775Z    |
2019-08-14T18:33:25.9653775Z    |
2019-08-14T18:33:25.9654040Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-14T18:33:25.9654416Z    |                          -                       ^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9654697Z    |                          |
2019-08-14T18:33:25.9654954Z    |                          lifetime `'_` defined here
2019-08-14T18:33:25.9655188Z    |                          lifetime `'_` defined here
2019-08-14T18:33:25.9655302Z error: lifetime may not live long enough
2019-08-14T18:33:25.9655578Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:13:73
2019-08-14T18:33:25.9655652Z    |
2019-08-14T18:33:25.9655652Z    |
2019-08-14T18:33:25.9655919Z LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-14T18:33:25.9656474Z    |                          -                                              ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9657761Z    |                          |
2019-08-14T18:33:25.9658099Z    |                          lifetime `'_` defined here
2019-08-14T18:33:25.9658349Z    |                          lifetime `'_` defined here
2019-08-14T18:33:25.9658904Z 
2019-08-14T18:33:25.9660105Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9660596Z    |
2019-08-14T18:33:25.9660596Z    |
2019-08-14T18:33:25.9660853Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-14T18:33:25.9661385Z    |
2019-08-14T18:33:25.9661385Z    |
2019-08-14T18:33:25.9661722Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9661842Z error: lifetime may not live long enough
2019-08-14T18:33:25.9662150Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-14T18:33:25.9662248Z    |
2019-08-14T18:33:25.9662248Z    |
2019-08-14T18:33:25.9662528Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-14T18:33:25.9662958Z    |                  --              -                           ^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-08-14T18:33:25.9663068Z    |                  |               |
2019-08-14T18:33:25.9663346Z    |                  |               lifetime `'_` defined here
2019-08-14T18:33:25.9663663Z 
2019-08-14T18:33:25.9663726Z error: aborting due to 5 previous errors
2019-08-14T18:33:25.9663886Z 
2019-08-14T18:33:25.9664194Z For more information about this error, try `rustc --explain E0700`.
2019-08-14T18:33:25.9664194Z For more information about this error, try `rustc --explain E0700`.
2019-08-14T18:33:25.9664248Z 
2019-08-14T18:33:25.9664649Z ------------------------------------------
2019-08-14T18:33:25.9664691Z 
2019-08-14T18:33:25.9664722Z 
2019-08-14T18:33:25.9664969Z ---- [ui (nll)] ui/self/elision/lt-ref-self-async.rs stdout ----
2019-08-14T18:33:25.9665039Z diff of stderr:
2019-08-14T18:33:25.9665092Z 
2019-08-14T18:33:25.9665324Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9665477Z 6    |
2019-08-14T18:33:25.9665477Z 6    |
2019-08-14T18:33:25.9665736Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T18:33:25.9665993Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9666133Z 9 error: lifetime may not live long enough
2019-08-14T18:33:25.9666364Z 10   --> $DIR/lt-ref-self-async.rs:15:47
2019-08-14T18:33:25.9666412Z 
2019-08-14T18:33:25.9666412Z 
2019-08-14T18:33:25.9666643Z 24 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9666799Z 26    |
2019-08-14T18:33:25.9666799Z 26    |
2019-08-14T18:33:25.9667056Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T18:33:25.9667311Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9667462Z 29 error: lifetime may not live long enough
2019-08-14T18:33:25.9667697Z 30   --> $DIR/lt-ref-self-async.rs:21:53
2019-08-14T18:33:25.9667737Z 
2019-08-14T18:33:25.9667737Z 
2019-08-14T18:33:25.9667988Z 44 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9668152Z 46    |
2019-08-14T18:33:25.9668152Z 46    |
2019-08-14T18:33:25.9668392Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T18:33:25.9668676Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9668818Z 49 error: lifetime may not live long enough
2019-08-14T18:33:25.9669035Z 50   --> $DIR/lt-ref-self-async.rs:25:62
2019-08-14T18:33:25.9669092Z 
2019-08-14T18:33:25.9669092Z 
2019-08-14T18:33:25.9669326Z 64 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9669569Z 66    |
2019-08-14T18:33:25.9669569Z 66    |
2019-08-14T18:33:25.9669862Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T18:33:25.9670116Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9670258Z 69 error: lifetime may not live long enough
2019-08-14T18:33:25.9670492Z 70   --> $DIR/lt-ref-self-async.rs:29:62
2019-08-14T18:33:25.9670532Z 
2019-08-14T18:33:25.9670532Z 
2019-08-14T18:33:25.9670793Z 84 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9671312Z 86    |
2019-08-14T18:33:25.9671312Z 86    |
2019-08-14T18:33:25.9671617Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T18:33:25.9672818Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9672980Z 89 error: lifetime may not live long enough
2019-08-14T18:33:25.9673233Z 90   --> $DIR/lt-ref-self-async.rs:33:71
2019-08-14T18:33:25.9673297Z 
2019-08-14T18:33:25.9673297Z 
2019-08-14T18:33:25.9673566Z 104 LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9673749Z 106    |
2019-08-14T18:33:25.9673749Z 106    |
2019-08-14T18:33:25.9674034Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-14T18:33:25.9674318Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9674747Z 109 error: lifetime may not live long enough
2019-08-14T18:33:25.9675017Z 110   --> $DIR/lt-ref-self-async.rs:37:67
2019-08-14T18:33:25.9675059Z 
2019-08-14T18:33:25.9675090Z 
2019-08-14T18:33:25.9675090Z 
2019-08-14T18:33:25.9675167Z The actual stderr differed from the expected stderr.
2019-08-14T18:33:25.9675498Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/lt-ref-self-async.nll.stderr
2019-08-14T18:33:25.9675781Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:33:25.9676051Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`
2019-08-14T18:33:25.9676177Z error: 1 errors occurred comparing output.
2019-08-14T18:33:25.9676257Z status: exit code: 1
2019-08-14T18:33:25.9676257Z status: exit code: 1
2019-08-14T18:33:25.9677099Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/auxiliary" "-A" "unused"
2019-08-14T18:33:25.9677578Z ------------------------------------------
2019-08-14T18:33:25.9677622Z 
2019-08-14T18:33:25.9677850Z ------------------------------------------
2019-08-14T18:33:25.9677911Z stderr:
2019-08-14T18:33:25.9677911Z stderr:
2019-08-14T18:33:25.9678130Z ------------------------------------------
2019-08-14T18:33:25.9678210Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9678565Z    |
2019-08-14T18:33:25.9678565Z    |
2019-08-14T18:33:25.9678806Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9678955Z    |
2019-08-14T18:33:25.9678955Z    |
2019-08-14T18:33:25.9679193Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9679313Z error: lifetime may not live long enough
2019-08-14T18:33:25.9681633Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:15:47
2019-08-14T18:33:25.9681975Z    |
2019-08-14T18:33:25.9681975Z    |
2019-08-14T18:33:25.9682395Z LL |       async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9682654Z    |  _______________________-_______________________^
2019-08-14T18:33:25.9682745Z    | |                       |
2019-08-14T18:33:25.9682995Z    | |                       lifetime `'_` defined here
2019-08-14T18:33:25.9683263Z    | |                       lifetime `'_` defined here
2019-08-14T18:33:25.9683358Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9683443Z LL | |     }
2019-08-14T18:33:25.9683756Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9683841Z 
2019-08-14T18:33:25.9684655Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9685470Z    |
2019-08-14T18:33:25.9685470Z    |
2019-08-14T18:33:25.9685963Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9686131Z    |
2019-08-14T18:33:25.9686131Z    |
2019-08-14T18:33:25.9687132Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9687263Z error: lifetime may not live long enough
2019-08-14T18:33:25.9687708Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:21:53
2019-08-14T18:33:25.9688044Z    |
2019-08-14T18:33:25.9688044Z    |
2019-08-14T18:33:25.9688365Z LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9688753Z    |  _____________________________-_______________________^
2019-08-14T18:33:25.9688840Z    | |                             |
2019-08-14T18:33:25.9689073Z    | |                             lifetime `'_` defined here
2019-08-14T18:33:25.9689325Z    | |                             lifetime `'_` defined here
2019-08-14T18:33:25.9689406Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9689484Z LL | |     }
2019-08-14T18:33:25.9689763Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9689840Z 
2019-08-14T18:33:25.9689910Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9690249Z    |
2019-08-14T18:33:25.9690249Z    |
2019-08-14T18:33:25.9690507Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9690667Z    |
2019-08-14T18:33:25.9690667Z    |
2019-08-14T18:33:25.9691329Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9691472Z error: lifetime may not live long enough
2019-08-14T18:33:25.9691744Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:25:62
2019-08-14T18:33:25.9693506Z    |
2019-08-14T18:33:25.9693506Z    |
2019-08-14T18:33:25.9693909Z LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9694216Z    |  _____________________________________-________________________^
2019-08-14T18:33:25.9694316Z    | |                                     |
2019-08-14T18:33:25.9694715Z    | |                                     lifetime `'_` defined here
2019-08-14T18:33:25.9694977Z    | |                                     lifetime `'_` defined here
2019-08-14T18:33:25.9695060Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9695140Z LL | |     }
2019-08-14T18:33:25.9695419Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9695481Z 
2019-08-14T18:33:25.9695566Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9696170Z    |
2019-08-14T18:33:25.9696170Z    |
2019-08-14T18:33:25.9696499Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9696661Z    |
2019-08-14T18:33:25.9696661Z    |
2019-08-14T18:33:25.9696903Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9697030Z error: lifetime may not live long enough
2019-08-14T18:33:25.9697275Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:29:62
2019-08-14T18:33:25.9697371Z    |
2019-08-14T18:33:25.9697371Z    |
2019-08-14T18:33:25.9697782Z LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9698056Z    |  _____________________________________-________________________^
2019-08-14T18:33:25.9698341Z    | |                                     |
2019-08-14T18:33:25.9698599Z    | |                                     lifetime `'_` defined here
2019-08-14T18:33:25.9699352Z    | |                                     lifetime `'_` defined here
2019-08-14T18:33:25.9699442Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9699523Z LL | |     }
2019-08-14T18:33:25.9699851Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9699913Z 
2019-08-14T18:33:25.9700001Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9700470Z    |
2019-08-14T18:33:25.9700470Z    |
2019-08-14T18:33:25.9700773Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9701147Z    |
2019-08-14T18:33:25.9701147Z    |
2019-08-14T18:33:25.9701627Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9701766Z error: lifetime may not live long enough
2019-08-14T18:33:25.9702044Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:33:71
2019-08-14T18:33:25.9702136Z    |
2019-08-14T18:33:25.9702136Z    |
2019-08-14T18:33:25.9702409Z LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9702711Z    |  _____________________________________________-_________________________^
2019-08-14T18:33:25.9702799Z    | |                                             |
2019-08-14T18:33:25.9703092Z    | |                                             lifetime `'_` defined here
2019-08-14T18:33:25.9703401Z    | |                                             lifetime `'_` defined here
2019-08-14T18:33:25.9703484Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9703570Z LL | |     }
2019-08-14T18:33:25.9705206Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9705288Z 
2019-08-14T18:33:25.9705398Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9707635Z    |
2019-08-14T18:33:25.9707635Z    |
2019-08-14T18:33:25.9708526Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9708716Z    |
2019-08-14T18:33:25.9708716Z    |
2019-08-14T18:33:25.9708999Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-14T18:33:25.9709256Z error: lifetime may not live long enough
2019-08-14T18:33:25.9709511Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:37:67
2019-08-14T18:33:25.9709599Z    |
2019-08-14T18:33:25.9709599Z    |
2019-08-14T18:33:25.9709847Z LL |       async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9710127Z    |  _________________________________________-_________________________^
2019-08-14T18:33:25.9710205Z    | |                                         |
2019-08-14T18:33:25.9718850Z    | |                                         lifetime `'_` defined here
2019-08-14T18:33:25.9719546Z    | |                                         lifetime `'_` defined here
2019-08-14T18:33:25.9719637Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9719725Z LL | |     }
2019-08-14T18:33:25.9720219Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9721113Z error: aborting due to 12 previous errors
2019-08-14T18:33:25.9721160Z 
2019-08-14T18:33:25.9721629Z For more information about this error, try `rustc --explain E0700`.
2019-08-14T18:33:25.9721689Z 
2019-08-14T18:33:25.9721689Z 
2019-08-14T18:33:25.9721950Z ------------------------------------------
2019-08-14T18:33:25.9721996Z 
2019-08-14T18:33:25.9722030Z 
2019-08-14T18:33:25.9722300Z ---- [ui (nll)] ui/self/elision/ref-mut-self-async.rs stdout ----
2019-08-14T18:33:25.9722377Z diff of stderr:
2019-08-14T18:33:25.9722417Z 
2019-08-14T18:33:25.9722694Z 4 LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9722869Z 6    |
2019-08-14T18:33:25.9722869Z 6    |
2019-08-14T18:33:25.9723142Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T18:33:25.9723446Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9723604Z 9 error: lifetime may not live long enough
2019-08-14T18:33:25.9724159Z 10   --> $DIR/ref-mut-self-async.rs:15:51
2019-08-14T18:33:25.9724222Z 
2019-08-14T18:33:25.9724222Z 
2019-08-14T18:33:25.9724586Z 24 LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9724768Z 26    |
2019-08-14T18:33:25.9724768Z 26    |
2019-08-14T18:33:25.9725060Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T18:33:25.9725343Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9725497Z 29 error: lifetime may not live long enough
2019-08-14T18:33:25.9726103Z 30   --> $DIR/ref-mut-self-async.rs:21:57
2019-08-14T18:33:25.9726153Z 
2019-08-14T18:33:25.9726153Z 
2019-08-14T18:33:25.9726432Z 44 LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9726611Z 46    |
2019-08-14T18:33:25.9726611Z 46    |
2019-08-14T18:33:25.9726882Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T18:33:25.9727176Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9727330Z 49 error: lifetime may not live long enough
2019-08-14T18:33:25.9727565Z 50   --> $DIR/ref-mut-self-async.rs:25:66
2019-08-14T18:33:25.9727626Z 
2019-08-14T18:33:25.9727626Z 
2019-08-14T18:33:25.9727883Z 64 LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9728068Z 66    |
2019-08-14T18:33:25.9728068Z 66    |
2019-08-14T18:33:25.9728350Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T18:33:25.9728626Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9728775Z 69 error: lifetime may not live long enough
2019-08-14T18:33:25.9729023Z 70   --> $DIR/ref-mut-self-async.rs:29:66
2019-08-14T18:33:25.9729076Z 
2019-08-14T18:33:25.9729076Z 
2019-08-14T18:33:25.9729363Z 84 LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9729551Z 86    |
2019-08-14T18:33:25.9729551Z 86    |
2019-08-14T18:33:25.9729810Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T18:33:25.9730099Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9730360Z 89 error: lifetime may not live long enough
2019-08-14T18:33:25.9731155Z 90   --> $DIR/ref-mut-self-async.rs:33:75
2019-08-14T18:33:25.9731206Z 
2019-08-14T18:33:25.9731206Z 
2019-08-14T18:33:25.9731534Z 104 LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9731930Z 106    |
2019-08-14T18:33:25.9731930Z 106    |
2019-08-14T18:33:25.9732256Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-14T18:33:25.9732556Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9732714Z 109 error: lifetime may not live long enough
2019-08-14T18:33:25.9732973Z 110   --> $DIR/ref-mut-self-async.rs:37:75
2019-08-14T18:33:25.9733020Z 
2019-08-14T18:33:25.9733055Z 
2019-08-14T18:33:25.9733055Z 
2019-08-14T18:33:25.9733175Z The actual stderr differed from the expected stderr.
2019-08-14T18:33:25.9733545Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/ref-mut-self-async.nll.stderr
2019-08-14T18:33:25.9733865Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:33:25.9734379Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`
2019-08-14T18:33:25.9734513Z error: 1 errors occurred comparing output.
2019-08-14T18:33:25.9734597Z status: exit code: 1
2019-08-14T18:33:25.9734597Z status: exit code: 1
2019-08-14T18:33:25.9735821Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/auxiliary" "-A" "unused"
2019-08-14T18:33:25.9736535Z ------------------------------------------
2019-08-14T18:33:25.9736583Z 
2019-08-14T18:33:25.9736827Z ------------------------------------------
2019-08-14T18:33:25.9736894Z stderr:
2019-08-14T18:33:25.9736894Z stderr:
2019-08-14T18:33:25.9737129Z ------------------------------------------
2019-08-14T18:33:25.9737216Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9737601Z    |
2019-08-14T18:33:25.9737601Z    |
2019-08-14T18:33:25.9737889Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-14T18:33:25.9738061Z    |
2019-08-14T18:33:25.9738061Z    |
2019-08-14T18:33:25.9738327Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9738457Z error: lifetime may not live long enough
2019-08-14T18:33:25.9738820Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:15:51
2019-08-14T18:33:25.9738894Z    |
2019-08-14T18:33:25.9738894Z    |
2019-08-14T18:33:25.9739289Z LL |       async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-14T18:33:25.9739731Z    |  _______________________-___________________________^
2019-08-14T18:33:25.9739825Z    | |                       |
2019-08-14T18:33:25.9740081Z    | |                       lifetime `'_` defined here
2019-08-14T18:33:25.9740346Z    | |                       lifetime `'_` defined here
2019-08-14T18:33:25.9740494Z LL | |     }
2019-08-14T18:33:25.9740494Z LL | |     }
2019-08-14T18:33:25.9740833Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9741684Z 
2019-08-14T18:33:25.9741890Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9742351Z    |
2019-08-14T18:33:25.9742351Z    |
2019-08-14T18:33:25.9742624Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9742799Z    |
2019-08-14T18:33:25.9742799Z    |
2019-08-14T18:33:25.9743067Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9743211Z error: lifetime may not live long enough
2019-08-14T18:33:25.9743706Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:21:57
2019-08-14T18:33:25.9743788Z    |
2019-08-14T18:33:25.9743788Z    |
2019-08-14T18:33:25.9744091Z LL |       async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-14T18:33:25.9744361Z    |  _____________________________-___________________________^
2019-08-14T18:33:25.9744456Z    | |                             |
2019-08-14T18:33:25.9744876Z    | |                             lifetime `'_` defined here
2019-08-14T18:33:25.9745147Z    | |                             lifetime `'_` defined here
2019-08-14T18:33:25.9745222Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9745308Z LL | |     }
2019-08-14T18:33:25.9745602Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9745683Z 
2019-08-14T18:33:25.9745757Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9746303Z    |
2019-08-14T18:33:25.9746303Z    |
2019-08-14T18:33:25.9746576Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9746757Z    |
2019-08-14T18:33:25.9746757Z    |
2019-08-14T18:33:25.9747195Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9747339Z error: lifetime may not live long enough
2019-08-14T18:33:25.9747618Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:25:66
2019-08-14T18:33:25.9747691Z    |
2019-08-14T18:33:25.9747691Z    |
2019-08-14T18:33:25.9747961Z LL |       async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9748229Z    |  _____________________________________-____________________________^
2019-08-14T18:33:25.9748323Z    | |                                     |
2019-08-14T18:33:25.9748590Z    | |                                     lifetime `'_` defined here
2019-08-14T18:33:25.9748867Z    | |                                     lifetime `'_` defined here
2019-08-14T18:33:25.9748947Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9749028Z LL | |     }
2019-08-14T18:33:25.9749484Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9749561Z 
2019-08-14T18:33:25.9749639Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9750115Z    |
2019-08-14T18:33:25.9750115Z    |
2019-08-14T18:33:25.9750411Z LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9750578Z    |
2019-08-14T18:33:25.9750578Z    |
2019-08-14T18:33:25.9750827Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-14T18:33:25.9751645Z error: lifetime may not live long enough
2019-08-14T18:33:25.9751967Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:29:66
2019-08-14T18:33:25.9752042Z    |
2019-08-14T18:33:25.9752042Z    |
2019-08-14T18:33:25.9752321Z LL |       async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9752600Z    |  _____________________________________-____________________________^
2019-08-14T18:33:25.9752803Z    | |                                     |
2019-08-14T18:33:25.9753297Z    | |                                     lifetime `'_` defined here
2019-08-14T18:33:25.9753611Z    | |                                     lifetime `'_` defined here
2019-08-14T18:33:25.9753692Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-14T18:33:25.9753781Z LL | |     }
2019-08-14T18:33:25.9754091Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-14T18:33:25.9754185Z 
2019-08-14T18:33:25.9754261Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-14T18:33:25.9754845Z    |
2019-08-14T18:33:25.9754845Z    |
2019-08-14T18:33:25.9755117Z LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-14T18:33:25.9755302Z    |
2019-08-14T18:33:25.9755302Z    |
2019-08-14T18:33:25.9755551Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
---
2019-08-14T18:33:25.9907070Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-14T18:33:25.9907165Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-14T18:33:25.9907273Z 
2019-08-14T18:33:25.9907314Z 
2019-08-14T18:33:25.9909059Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-14T18:33:25.9909720Z 
2019-08-14T18:33:25.9909763Z 
2019-08-14T18:33:25.9909826Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-14T18:33:25.9909914Z Build completed unsuccessfully in 1:55:14
2019-08-14T18:33:25.9909914Z Build completed unsuccessfully in 1:55:14
2019-08-14T18:33:25.9909979Z == clock drift check ==
2019-08-14T18:33:25.9910052Z   local time: Wed Aug 14 18:33:25 UTC 2019
2019-08-14T18:33:26.1394411Z   network time: Wed, 14 Aug 2019 18:33:26 GMT
2019-08-14T18:33:26.1394698Z == end clock drift check ==
2019-08-14T18:33:26.8909450Z ##[error]Bash exited with code '1'.
2019-08-14T18:33:26.8949613Z ##[section]Starting: Upload CPU usage statistics
2019-08-14T18:33:26.8957787Z ==============================================================================
2019-08-14T18:33:26.8957872Z Task         : Bash
2019-08-14T18:33:26.8957957Z Description  : Run a Bash script on macOS, Linux, or Windows
