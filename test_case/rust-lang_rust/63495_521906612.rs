plain
2019-08-16T04:51:37.4282881Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T04:51:37.4282960Z 
2019-08-16T04:51:37.4283203Z   git checkout -b <new-branch-name>
2019-08-16T04:51:37.4283251Z 
2019-08-16T04:51:37.4283564Z HEAD is now at d4df2e3da Auto merge of #63495 - eddyb:mir-constant-ty, r=oli-obk
2019-08-16T04:51:37.4445652Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T04:51:37.4450528Z ==============================================================================
2019-08-16T04:51:37.4450610Z Task         : Bash
2019-08-16T04:51:37.4450882Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T06:50:53.4438781Z 
2019-08-16T06:50:53.4439193Z ---- [ui (nll)] ui/async-await/issues/issue-63388-1.rs stdout ----
2019-08-16T06:50:53.4439458Z diff of stderr:
2019-08-16T06:50:53.4439490Z 
2019-08-16T06:50:53.4440242Z 4 LL |     ) -> &dyn Foo
2019-08-16T06:50:53.4440398Z 6    |
2019-08-16T06:50:53.4440398Z 6    |
2019-08-16T06:50:53.4440708Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#27r
2019-08-16T06:50:53.4441114Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#22r
2019-08-16T06:50:53.4441287Z 9 error: lifetime may not live long enough
2019-08-16T06:50:53.4441572Z 10   --> $DIR/issue-63388-1.rs:15:5
2019-08-16T06:50:53.4441642Z 
2019-08-16T06:50:53.4441680Z 
2019-08-16T06:50:53.4441680Z 
2019-08-16T06:50:53.4444828Z The actual stderr differed from the expected stderr.
2019-08-16T06:50:53.4445197Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/issue-63388-1.nll.stderr
2019-08-16T06:50:53.4445451Z To update references, rerun the tests and pass the `--bless` flag
2019-08-16T06:50:53.4445717Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-1.rs`
2019-08-16T06:50:53.4445831Z error: 1 errors occurred comparing output.
2019-08-16T06:50:53.4445897Z status: exit code: 1
2019-08-16T06:50:53.4445897Z status: exit code: 1
2019-08-16T06:50:53.4446604Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/auxiliary" "-A" "unused"
2019-08-16T06:50:53.4447028Z ------------------------------------------
2019-08-16T06:50:53.4447067Z 
2019-08-16T06:50:53.4447398Z ------------------------------------------
2019-08-16T06:50:53.4447490Z stderr:
2019-08-16T06:50:53.4447490Z stderr:
2019-08-16T06:50:53.4447712Z ------------------------------------------
2019-08-16T06:50:53.4447795Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4448060Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:14:10
2019-08-16T06:50:53.4448123Z    |
2019-08-16T06:50:53.4448476Z LL |     ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-16T06:50:53.4448599Z    |
2019-08-16T06:50:53.4448599Z    |
2019-08-16T06:50:53.4448825Z    = note: hidden type `impl std::future::Future` captures lifetime '_#22r
2019-08-16T06:50:53.4448928Z error: lifetime may not live long enough
2019-08-16T06:50:53.4449140Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:15:5
2019-08-16T06:50:53.4450009Z    |
2019-08-16T06:50:53.4450009Z    |
2019-08-16T06:50:53.4450296Z LL |       async fn do_sth<'a>(
2019-08-16T06:50:53.4450592Z    |                       -- lifetime `'a` defined here
2019-08-16T06:50:53.4450877Z LL |           &'a self, foo: &dyn Foo
2019-08-16T06:50:53.4451163Z    |                          - lifetime `'_` defined here
2019-08-16T06:50:53.4451457Z LL |       ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-16T06:50:53.4451765Z LL | |         foo
2019-08-16T06:50:53.4451830Z LL | |     }
2019-08-16T06:50:53.4451830Z LL | |     }
2019-08-16T06:50:53.4452219Z    | |_____^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4452352Z error: aborting due to 2 previous errors
2019-08-16T06:50:53.4452415Z 
2019-08-16T06:50:53.4452694Z For more information about this error, try `rustc --explain E0700`.
2019-08-16T06:50:53.4452751Z 
2019-08-16T06:50:53.4452751Z 
2019-08-16T06:50:53.4453173Z ------------------------------------------
2019-08-16T06:50:53.4453215Z 
2019-08-16T06:50:53.4453424Z 
2019-08-16T06:50:53.4453825Z ---- [ui (nll)] ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs stdout ----
2019-08-16T06:50:53.4453912Z diff of stderr:
2019-08-16T06:50:53.4453942Z 
2019-08-16T06:50:53.4454147Z 4 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-16T06:50:53.4454287Z 6    |
2019-08-16T06:50:53.4454287Z 6    |
2019-08-16T06:50:53.4454521Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-16T06:50:53.4454747Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4454866Z 9 error: lifetime may not live long enough
2019-08-16T06:50:53.4455099Z 10   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-16T06:50:53.4455141Z 
2019-08-16T06:50:53.4455141Z 
2019-08-16T06:50:53.4455365Z 30 LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-08-16T06:50:53.4455502Z 32    |
2019-08-16T06:50:53.4455502Z 32    |
2019-08-16T06:50:53.4455722Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-16T06:50:53.4455973Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4456103Z 35 error: lifetime may not live long enough
2019-08-16T06:50:53.4456324Z 36   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-16T06:50:53.4456380Z 
2019-08-16T06:50:53.4456406Z 
2019-08-16T06:50:53.4456406Z 
2019-08-16T06:50:53.4456471Z The actual stderr differed from the expected stderr.
2019-08-16T06:50:53.4456794Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/arbitrary_self_types_pin_lifetime_mismatch-async.nll.stderr
2019-08-16T06:50:53.4457056Z To update references, rerun the tests and pass the `--bless` flag
2019-08-16T06:50:53.4457316Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`
2019-08-16T06:50:53.4457509Z error: 1 errors occurred comparing output.
2019-08-16T06:50:53.4457589Z status: exit code: 1
2019-08-16T06:50:53.4457589Z status: exit code: 1
2019-08-16T06:50:53.4458367Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/auxiliary" "-A" "unused"
2019-08-16T06:50:53.4458824Z ------------------------------------------
2019-08-16T06:50:53.4458865Z 
2019-08-16T06:50:53.4459085Z ------------------------------------------
2019-08-16T06:50:53.4459149Z stderr:
2019-08-16T06:50:53.4459149Z stderr:
2019-08-16T06:50:53.4459539Z ------------------------------------------
2019-08-16T06:50:53.4459609Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4460908Z    |
2019-08-16T06:50:53.4460908Z    |
2019-08-16T06:50:53.4461195Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-16T06:50:53.4461367Z    |
2019-08-16T06:50:53.4461367Z    |
2019-08-16T06:50:53.4461657Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4461795Z error: lifetime may not live long enough
2019-08-16T06:50:53.4462124Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-16T06:50:53.4462209Z    |
2019-08-16T06:50:53.4462209Z    |
2019-08-16T06:50:53.4462503Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-16T06:50:53.4462912Z    |                          -                       ^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4463048Z    |                          |
2019-08-16T06:50:53.4463335Z    |                          lifetime `'_` defined here
2019-08-16T06:50:53.4463807Z    |                          lifetime `'_` defined here
2019-08-16T06:50:53.4463905Z error: lifetime may not live long enough
2019-08-16T06:50:53.4464132Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:13:73
2019-08-16T06:50:53.4464208Z    |
2019-08-16T06:50:53.4464208Z    |
2019-08-16T06:50:53.4464429Z LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-16T06:50:53.4464774Z    |                          -                                              ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4464860Z    |                          |
2019-08-16T06:50:53.4465079Z    |                          lifetime `'_` defined here
2019-08-16T06:50:53.4465282Z    |                          lifetime `'_` defined here
2019-08-16T06:50:53.4465521Z 
2019-08-16T06:50:53.4465580Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4465925Z    |
2019-08-16T06:50:53.4465925Z    |
2019-08-16T06:50:53.4466352Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-16T06:50:53.4466499Z    |
2019-08-16T06:50:53.4466499Z    |
2019-08-16T06:50:53.4466728Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4467012Z error: lifetime may not live long enough
2019-08-16T06:50:53.4467367Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-16T06:50:53.4467450Z    |
2019-08-16T06:50:53.4467450Z    |
2019-08-16T06:50:53.4467744Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-16T06:50:53.4468101Z    |                  --              -                           ^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-08-16T06:50:53.4468207Z    |                  |               |
2019-08-16T06:50:53.4468439Z    |                  |               lifetime `'_` defined here
2019-08-16T06:50:53.4468721Z 
2019-08-16T06:50:53.4468787Z error: aborting due to 5 previous errors
2019-08-16T06:50:53.4468987Z 
2019-08-16T06:50:53.4469226Z For more information about this error, try `rustc --explain E0700`.
2019-08-16T06:50:53.4469226Z For more information about this error, try `rustc --explain E0700`.
2019-08-16T06:50:53.4469273Z 
2019-08-16T06:50:53.4470118Z ------------------------------------------
2019-08-16T06:50:53.4470202Z 
2019-08-16T06:50:53.4470240Z 
2019-08-16T06:50:53.4470521Z ---- [ui (nll)] ui/self/elision/lt-ref-self-async.rs stdout ----
2019-08-16T06:50:53.4470618Z diff of stderr:
2019-08-16T06:50:53.4470759Z 
2019-08-16T06:50:53.4471080Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4471254Z 6    |
2019-08-16T06:50:53.4471254Z 6    |
2019-08-16T06:50:53.4471543Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-16T06:50:53.4471867Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4472028Z 9 error: lifetime may not live long enough
2019-08-16T06:50:53.4472285Z 10   --> $DIR/lt-ref-self-async.rs:15:47
2019-08-16T06:50:53.4472350Z 
2019-08-16T06:50:53.4472350Z 
2019-08-16T06:50:53.4472624Z 24 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4472811Z 26    |
2019-08-16T06:50:53.4472811Z 26    |
2019-08-16T06:50:53.4473433Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-16T06:50:53.4473672Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4473808Z 29 error: lifetime may not live long enough
2019-08-16T06:50:53.4474029Z 30   --> $DIR/lt-ref-self-async.rs:21:53
2019-08-16T06:50:53.4474068Z 
2019-08-16T06:50:53.4474068Z 
2019-08-16T06:50:53.4474301Z 44 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4474446Z 46    |
2019-08-16T06:50:53.4474446Z 46    |
2019-08-16T06:50:53.4474669Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-16T06:50:53.4475084Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4475205Z 49 error: lifetime may not live long enough
2019-08-16T06:50:53.4475403Z 50   --> $DIR/lt-ref-self-async.rs:25:62
2019-08-16T06:50:53.4475442Z 
2019-08-16T06:50:53.4475442Z 
2019-08-16T06:50:53.4475668Z 64 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4475814Z 66    |
2019-08-16T06:50:53.4475814Z 66    |
2019-08-16T06:50:53.4476043Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-16T06:50:53.4476269Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4476390Z 69 error: lifetime may not live long enough
2019-08-16T06:50:53.4476595Z 70   --> $DIR/lt-ref-self-async.rs:29:62
2019-08-16T06:50:53.4476630Z 
2019-08-16T06:50:53.4476630Z 
2019-08-16T06:50:53.4476849Z 84 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4476989Z 86    |
2019-08-16T06:50:53.4476989Z 86    |
2019-08-16T06:50:53.4477286Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-16T06:50:53.4477549Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4477694Z 89 error: lifetime may not live long enough
2019-08-16T06:50:53.4477897Z 90   --> $DIR/lt-ref-self-async.rs:33:71
2019-08-16T06:50:53.4477933Z 
2019-08-16T06:50:53.4477933Z 
2019-08-16T06:50:53.4478162Z 104 LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4478299Z 106    |
2019-08-16T06:50:53.4478299Z 106    |
2019-08-16T06:50:53.4478509Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-16T06:50:53.4478751Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4478871Z 109 error: lifetime may not live long enough
2019-08-16T06:50:53.4479063Z 110   --> $DIR/lt-ref-self-async.rs:37:67
2019-08-16T06:50:53.4479121Z 
2019-08-16T06:50:53.4479147Z 
2019-08-16T06:50:53.4479147Z 
2019-08-16T06:50:53.4479210Z The actual stderr differed from the expected stderr.
2019-08-16T06:50:53.4480110Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/lt-ref-self-async.nll.stderr
2019-08-16T06:50:53.4480583Z To update references, rerun the tests and pass the `--bless` flag
2019-08-16T06:50:53.4480910Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`
2019-08-16T06:50:53.4481058Z error: 1 errors occurred comparing output.
2019-08-16T06:50:53.4481127Z status: exit code: 1
2019-08-16T06:50:53.4481127Z status: exit code: 1
2019-08-16T06:50:53.4482103Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/auxiliary" "-A" "unused"
2019-08-16T06:50:53.4482653Z ------------------------------------------
2019-08-16T06:50:53.4482706Z 
2019-08-16T06:50:53.4482955Z ------------------------------------------
2019-08-16T06:50:53.4483045Z stderr:
2019-08-16T06:50:53.4483045Z stderr:
2019-08-16T06:50:53.4483454Z ------------------------------------------
2019-08-16T06:50:53.4483718Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4484190Z    |
2019-08-16T06:50:53.4484190Z    |
2019-08-16T06:50:53.4484396Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4484532Z    |
2019-08-16T06:50:53.4484532Z    |
2019-08-16T06:50:53.4484750Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4484868Z error: lifetime may not live long enough
2019-08-16T06:50:53.4485103Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:15:47
2019-08-16T06:50:53.4485179Z    |
2019-08-16T06:50:53.4485179Z    |
2019-08-16T06:50:53.4485389Z LL |       async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4485623Z    |  _______________________-_______________________^
2019-08-16T06:50:53.4485682Z    | |                       |
2019-08-16T06:50:53.4485910Z    | |                       lifetime `'_` defined here
2019-08-16T06:50:53.4486124Z    | |                       lifetime `'_` defined here
2019-08-16T06:50:53.4486201Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4486268Z LL | |     }
2019-08-16T06:50:53.4486599Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4486663Z 
2019-08-16T06:50:53.4486740Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4487093Z    |
2019-08-16T06:50:53.4487093Z    |
2019-08-16T06:50:53.4487310Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4487444Z    |
2019-08-16T06:50:53.4487444Z    |
2019-08-16T06:50:53.4487686Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4487794Z error: lifetime may not live long enough
2019-08-16T06:50:53.4488018Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:21:53
2019-08-16T06:50:53.4488095Z    |
2019-08-16T06:50:53.4488095Z    |
2019-08-16T06:50:53.4491091Z LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4491865Z    |  _____________________________-_______________________^
2019-08-16T06:50:53.4491973Z    | |                             |
2019-08-16T06:50:53.4492402Z    | |                             lifetime `'_` defined here
2019-08-16T06:50:53.4492697Z    | |                             lifetime `'_` defined here
2019-08-16T06:50:53.4492981Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4493051Z LL | |     }
2019-08-16T06:50:53.4493765Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4493999Z 
2019-08-16T06:50:53.4494074Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4494372Z    |
2019-08-16T06:50:53.4494372Z    |
2019-08-16T06:50:53.4494576Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4494711Z    |
2019-08-16T06:50:53.4494711Z    |
2019-08-16T06:50:53.4494948Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4495055Z error: lifetime may not live long enough
2019-08-16T06:50:53.4495278Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:25:62
2019-08-16T06:50:53.4495349Z    |
2019-08-16T06:50:53.4495349Z    |
2019-08-16T06:50:53.4495556Z LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4495789Z    |  _____________________________________-________________________^
2019-08-16T06:50:53.4495850Z    | |                                     |
2019-08-16T06:50:53.4496075Z    | |                                     lifetime `'_` defined here
2019-08-16T06:50:53.4496288Z    | |                                     lifetime `'_` defined here
2019-08-16T06:50:53.4496364Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4496412Z LL | |     }
2019-08-16T06:50:53.4496675Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4496727Z 
2019-08-16T06:50:53.4496799Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4497103Z    |
2019-08-16T06:50:53.4497103Z    |
2019-08-16T06:50:53.4497307Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4497439Z    |
2019-08-16T06:50:53.4497439Z    |
2019-08-16T06:50:53.4497662Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4497764Z error: lifetime may not live long enough
2019-08-16T06:50:53.4497974Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:29:62
2019-08-16T06:50:53.4498045Z    |
2019-08-16T06:50:53.4498045Z    |
2019-08-16T06:50:53.4498249Z LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4498561Z    |  _____________________________________-________________________^
2019-08-16T06:50:53.4498635Z    | |                                     |
2019-08-16T06:50:53.4498891Z    | |                                     lifetime `'_` defined here
2019-08-16T06:50:53.4499122Z    | |                                     lifetime `'_` defined here
2019-08-16T06:50:53.4499201Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4499604Z LL | |     }
2019-08-16T06:50:53.4500170Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4500239Z 
2019-08-16T06:50:53.4500336Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4500731Z    |
2019-08-16T06:50:53.4500731Z    |
2019-08-16T06:50:53.4501015Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4501216Z    |
2019-08-16T06:50:53.4501216Z    |
2019-08-16T06:50:53.4501525Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4501772Z error: lifetime may not live long enough
2019-08-16T06:50:53.4502092Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:33:71
2019-08-16T06:50:53.4502189Z    |
2019-08-16T06:50:53.4502189Z    |
2019-08-16T06:50:53.4502478Z LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4502800Z    |  _____________________________________________-_________________________^
2019-08-16T06:50:53.4502889Z    | |                                             |
2019-08-16T06:50:53.4503358Z    | |                                             lifetime `'_` defined here
2019-08-16T06:50:53.4503746Z    | |                                             lifetime `'_` defined here
2019-08-16T06:50:53.4503825Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4503882Z LL | |     }
2019-08-16T06:50:53.4504145Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4504196Z 
2019-08-16T06:50:53.4504276Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4504573Z    |
2019-08-16T06:50:53.4504573Z    |
2019-08-16T06:50:53.4504780Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4504916Z    |
2019-08-16T06:50:53.4504916Z    |
2019-08-16T06:50:53.4505139Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-08-16T06:50:53.4505240Z error: lifetime may not live long enough
2019-08-16T06:50:53.4505450Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:37:67
2019-08-16T06:50:53.4505521Z    |
2019-08-16T06:50:53.4505521Z    |
2019-08-16T06:50:53.4505738Z LL |       async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4505982Z    |  _________________________________________-_________________________^
2019-08-16T06:50:53.4506053Z    | |                                         |
2019-08-16T06:50:53.4506472Z    | |                                         lifetime `'_` defined here
2019-08-16T06:50:53.4506703Z    | |                                         lifetime `'_` defined here
2019-08-16T06:50:53.4506783Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4507007Z LL | |     }
2019-08-16T06:50:53.4507288Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4507408Z error: aborting due to 12 previous errors
2019-08-16T06:50:53.4507444Z 
2019-08-16T06:50:53.4509056Z For more information about this error, try `rustc --explain E0700`.
2019-08-16T06:50:53.4509141Z 
2019-08-16T06:50:53.4509141Z 
2019-08-16T06:50:53.4510387Z ------------------------------------------
2019-08-16T06:50:53.4510488Z 
2019-08-16T06:50:53.4510524Z 
2019-08-16T06:50:53.4510844Z ---- [ui (nll)] ui/self/elision/ref-mut-self-async.rs stdout ----
2019-08-16T06:50:53.4510957Z diff of stderr:
2019-08-16T06:50:53.4510998Z 
2019-08-16T06:50:53.4511295Z 4 LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4511472Z 6    |
2019-08-16T06:50:53.4511472Z 6    |
2019-08-16T06:50:53.4511762Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-16T06:50:53.4512085Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4512246Z 9 error: lifetime may not live long enough
2019-08-16T06:50:53.4512506Z 10   --> $DIR/ref-mut-self-async.rs:15:51
2019-08-16T06:50:53.4512570Z 
2019-08-16T06:50:53.4512570Z 
2019-08-16T06:50:53.4513014Z 24 LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4513516Z 26    |
2019-08-16T06:50:53.4513516Z 26    |
2019-08-16T06:50:53.4513759Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-16T06:50:53.4514107Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4514236Z 29 error: lifetime may not live long enough
2019-08-16T06:50:53.4514455Z 30   --> $DIR/ref-mut-self-async.rs:21:57
2019-08-16T06:50:53.4514495Z 
2019-08-16T06:50:53.4514495Z 
2019-08-16T06:50:53.4514716Z 44 LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4514863Z 46    |
2019-08-16T06:50:53.4514863Z 46    |
2019-08-16T06:50:53.4515101Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-16T06:50:53.4515354Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4515489Z 49 error: lifetime may not live long enough
2019-08-16T06:50:53.4515850Z 50   --> $DIR/ref-mut-self-async.rs:25:66
2019-08-16T06:50:53.4515887Z 
2019-08-16T06:50:53.4515887Z 
2019-08-16T06:50:53.4516111Z 64 LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4516260Z 66    |
2019-08-16T06:50:53.4516260Z 66    |
2019-08-16T06:50:53.4516479Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-16T06:50:53.4516722Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4516841Z 69 error: lifetime may not live long enough
2019-08-16T06:50:53.4517031Z 70   --> $DIR/ref-mut-self-async.rs:29:66
2019-08-16T06:50:53.4517083Z 
2019-08-16T06:50:53.4517083Z 
2019-08-16T06:50:53.4517299Z 84 LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4517453Z 86    |
2019-08-16T06:50:53.4517453Z 86    |
2019-08-16T06:50:53.4517688Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-16T06:50:53.4517912Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4518041Z 89 error: lifetime may not live long enough
2019-08-16T06:50:53.4518251Z 90   --> $DIR/ref-mut-self-async.rs:33:75
2019-08-16T06:50:53.4518287Z 
2019-08-16T06:50:53.4518287Z 
2019-08-16T06:50:53.4518568Z 104 LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4518713Z 106    |
2019-08-16T06:50:53.4518713Z 106    |
2019-08-16T06:50:53.4518925Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-16T06:50:53.4519164Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4520095Z 109 error: lifetime may not live long enough
2019-08-16T06:50:53.4520425Z 110   --> $DIR/ref-mut-self-async.rs:37:75
2019-08-16T06:50:53.4520494Z 
2019-08-16T06:50:53.4520530Z 
2019-08-16T06:50:53.4520530Z 
2019-08-16T06:50:53.4520597Z The actual stderr differed from the expected stderr.
2019-08-16T06:50:53.4521016Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/ref-mut-self-async.nll.stderr
2019-08-16T06:50:53.4521356Z To update references, rerun the tests and pass the `--bless` flag
2019-08-16T06:50:53.4521688Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`
2019-08-16T06:50:53.4521830Z error: 1 errors occurred comparing output.
2019-08-16T06:50:53.4521916Z status: exit code: 1
2019-08-16T06:50:53.4521916Z status: exit code: 1
2019-08-16T06:50:53.4522883Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/auxiliary" "-A" "unused"
2019-08-16T06:50:53.4523967Z ------------------------------------------
2019-08-16T06:50:53.4524007Z 
2019-08-16T06:50:53.4524193Z ------------------------------------------
2019-08-16T06:50:53.4524259Z stderr:
2019-08-16T06:50:53.4524259Z stderr:
2019-08-16T06:50:53.4524438Z ------------------------------------------
2019-08-16T06:50:53.4524518Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4524824Z    |
2019-08-16T06:50:53.4524824Z    |
2019-08-16T06:50:53.4525064Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-16T06:50:53.4525204Z    |
2019-08-16T06:50:53.4525204Z    |
2019-08-16T06:50:53.4525424Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4525527Z error: lifetime may not live long enough
2019-08-16T06:50:53.4525740Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:15:51
2019-08-16T06:50:53.4525812Z    |
2019-08-16T06:50:53.4525812Z    |
2019-08-16T06:50:53.4526028Z LL |       async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-16T06:50:53.4526257Z    |  _______________________-___________________________^
2019-08-16T06:50:53.4526328Z    | |                       |
2019-08-16T06:50:53.4526525Z    | |                       lifetime `'_` defined here
2019-08-16T06:50:53.4526742Z    | |                       lifetime `'_` defined here
2019-08-16T06:50:53.4526869Z LL | |     }
2019-08-16T06:50:53.4526869Z LL | |     }
2019-08-16T06:50:53.4527118Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4527178Z 
2019-08-16T06:50:53.4527250Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4527556Z    |
2019-08-16T06:50:53.4527556Z    |
2019-08-16T06:50:53.4527756Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4527904Z    |
2019-08-16T06:50:53.4527904Z    |
2019-08-16T06:50:53.4528114Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4528216Z error: lifetime may not live long enough
2019-08-16T06:50:53.4528654Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:21:57
2019-08-16T06:50:53.4528733Z    |
2019-08-16T06:50:53.4528733Z    |
2019-08-16T06:50:53.4529026Z LL |       async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-16T06:50:53.4529871Z    |  _____________________________-___________________________^
2019-08-16T06:50:53.4529974Z    | |                             |
2019-08-16T06:50:53.4530305Z    | |                             lifetime `'_` defined here
2019-08-16T06:50:53.4530589Z    | |                             lifetime `'_` defined here
2019-08-16T06:50:53.4530689Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4530757Z LL | |     }
2019-08-16T06:50:53.4531102Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4531172Z 
2019-08-16T06:50:53.4531267Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4531668Z    |
2019-08-16T06:50:53.4531668Z    |
2019-08-16T06:50:53.4531959Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4532143Z    |
2019-08-16T06:50:53.4532143Z    |
2019-08-16T06:50:53.4532592Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4532729Z error: lifetime may not live long enough
2019-08-16T06:50:53.4533017Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:25:66
2019-08-16T06:50:53.4533267Z    |
2019-08-16T06:50:53.4533267Z    |
2019-08-16T06:50:53.4533667Z LL |       async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4533906Z    |  _____________________________________-____________________________^
2019-08-16T06:50:53.4533967Z    | |                                     |
2019-08-16T06:50:53.4534192Z    | |                                     lifetime `'_` defined here
2019-08-16T06:50:53.4534405Z    | |                                     lifetime `'_` defined here
2019-08-16T06:50:53.4534489Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4534540Z LL | |     }
2019-08-16T06:50:53.4534802Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4534863Z 
2019-08-16T06:50:53.4534934Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4535233Z    |
2019-08-16T06:50:53.4535233Z    |
2019-08-16T06:50:53.4535439Z LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4535574Z    |
2019-08-16T06:50:53.4535574Z    |
2019-08-16T06:50:53.4535799Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-08-16T06:50:53.4535900Z error: lifetime may not live long enough
2019-08-16T06:50:53.4536119Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:29:66
2019-08-16T06:50:53.4536193Z    |
2019-08-16T06:50:53.4536193Z    |
2019-08-16T06:50:53.4536405Z LL |       async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4536645Z    |  _____________________________________-____________________________^
2019-08-16T06:50:53.4536716Z    | |                                     |
2019-08-16T06:50:53.4536947Z    | |                                     lifetime `'_` defined here
2019-08-16T06:50:53.4537163Z    | |                                     lifetime `'_` defined here
2019-08-16T06:50:53.4537238Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-16T06:50:53.4537288Z LL | |     }
2019-08-16T06:50:53.4537541Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-16T06:50:53.4537591Z 
2019-08-16T06:50:53.4537661Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-16T06:50:53.4538034Z    |
2019-08-16T06:50:53.4538034Z    |
2019-08-16T06:50:53.4538275Z LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-16T06:50:53.4538426Z    |
2019-08-16T06:50:53.4538426Z    |
2019-08-16T06:50:53.4538659Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
---
2019-08-16T06:50:54.1132603Z test result: FAILED. 8843 passed; 7 failed; 72 ignored; 0 measured; 0 filtered out
2019-08-16T06:50:54.1132698Z 
2019-08-16T06:50:54.1132733Z 
2019-08-16T06:50:54.1132767Z 
2019-08-16T06:50:54.1134966Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-16T06:50:54.1135571Z 
2019-08-16T06:50:54.1135602Z 
2019-08-16T06:50:54.1135673Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-16T06:50:54.1135737Z Build completed unsuccessfully in 1:54:11
2019-08-16T06:50:54.1135737Z Build completed unsuccessfully in 1:54:11
2019-08-16T06:50:54.1135810Z == clock drift check ==
2019-08-16T06:50:54.1135870Z   local time: Fri Aug 16 06:50:53 UTC 2019
2019-08-16T06:50:54.1135945Z   network time: Fri, 16 Aug 2019 06:50:53 GMT
2019-08-16T06:50:54.1136000Z == end clock drift check ==
2019-08-16T06:50:54.3789782Z ##[error]Bash exited with code '1'.
2019-08-16T06:50:54.3827798Z ##[section]Starting: Upload CPU usage statistics
2019-08-16T06:50:54.3835873Z ==============================================================================
2019-08-16T06:50:54.3835948Z Task         : Bash
2019-08-16T06:50:54.3836020Z Description  : Run a Bash script on macOS, Linux, or Windows
