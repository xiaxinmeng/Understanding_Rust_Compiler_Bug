plain
2019-08-13T10:02:15.8517506Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T10:02:15.8517759Z 
2019-08-13T10:02:15.8518267Z   git checkout -b <new-branch-name>
2019-08-13T10:02:15.8518311Z 
2019-08-13T10:02:15.8518947Z HEAD is now at 917eeccb6 Auto merge of #63515 - Centril:rollup-r0z0yi8, r=Centril
2019-08-13T10:02:15.8673659Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T10:02:15.8676584Z ==============================================================================
2019-08-13T10:02:15.8676687Z Task         : Bash
2019-08-13T10:02:15.8676757Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T11:58:48.9365499Z 
2019-08-13T11:58:48.9366311Z ---- [ui (nll)] ui/async-await/issues/issue-63388-1.rs stdout ----
2019-08-13T11:58:48.9366409Z diff of stderr:
2019-08-13T11:58:48.9366451Z 
2019-08-13T11:58:48.9367004Z - error[E0623]: lifetime mismatch
2019-08-13T11:58:48.9367104Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9367400Z 2   --> $DIR/issue-63388-1.rs:14:10
2019-08-13T11:58:48.9367601Z 3    |
2019-08-13T11:58:48.9367922Z - LL |         &'a self, foo: &dyn Foo
2019-08-13T11:58:48.9368238Z -    |         -------- this parameter and the return type are declared with different lifetimes...
2019-08-13T11:58:48.9368696Z 6 LL |     ) -> &dyn Foo
2019-08-13T11:58:48.9369173Z -    |          |
2019-08-13T11:58:48.9369173Z -    |          |
2019-08-13T11:58:48.9369500Z -    |          ...but data from `foo` is returned here
2019-08-13T11:58:48.9369593Z +    |
2019-08-13T11:58:48.9370043Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#27r
2019-08-13T11:58:48.9370424Z - error: aborting due to previous error
2019-08-13T11:58:48.9370424Z - error: aborting due to previous error
2019-08-13T11:58:48.9370519Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9370967Z +   --> $DIR/issue-63388-1.rs:15:5
2019-08-13T11:58:48.9371046Z +    |
2019-08-13T11:58:48.9371277Z + LL |       async fn do_sth<'a>(
2019-08-13T11:58:48.9371737Z +    |                       -- lifetime `'a` defined here
2019-08-13T11:58:48.9372014Z + LL |           &'a self, foo: &dyn Foo
2019-08-13T11:58:48.9372440Z +    |                          - lifetime `'_` defined here
2019-08-13T11:58:48.9372723Z + LL |       ) -> &dyn Foo
2019-08-13T11:58:48.9372792Z + LL | /     {
2019-08-13T11:58:48.9373237Z + LL | |         foo
2019-08-13T11:58:48.9373347Z + LL | |     }
2019-08-13T11:58:48.9374269Z +    | |_____^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9374463Z + error: aborting due to 2 previous errors
2019-08-13T11:58:48.9374530Z + 
2019-08-13T11:58:48.9375035Z + For more information about this error, try `rustc --explain E0700`.
2019-08-13T11:58:48.9375121Z 13 
2019-08-13T11:58:48.9375121Z 13 
2019-08-13T11:58:48.9375159Z 
2019-08-13T11:58:48.9375210Z 
2019-08-13T11:58:48.9375277Z The actual stderr differed from the expected stderr.
2019-08-13T11:58:48.9375873Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/issue-63388-1.nll.stderr
2019-08-13T11:58:48.9376185Z To update references, rerun the tests and pass the `--bless` flag
2019-08-13T11:58:48.9376518Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-1.rs`
2019-08-13T11:58:48.9376837Z error: 1 errors occurred comparing output.
2019-08-13T11:58:48.9376905Z status: exit code: 1
2019-08-13T11:58:48.9376905Z status: exit code: 1
2019-08-13T11:58:48.9377917Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/auxiliary" "-A" "unused"
2019-08-13T11:58:48.9404602Z ------------------------------------------
2019-08-13T11:58:48.9404703Z 
2019-08-13T11:58:48.9405113Z ------------------------------------------
2019-08-13T11:58:48.9405403Z stderr:
2019-08-13T11:58:48.9405403Z stderr:
2019-08-13T11:58:48.9405701Z ------------------------------------------
2019-08-13T11:58:48.9405808Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9407174Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:14:10
2019-08-13T11:58:48.9407296Z    |
2019-08-13T11:58:48.9408265Z LL |     ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-13T11:58:48.9408627Z    |
2019-08-13T11:58:48.9408627Z    |
2019-08-13T11:58:48.9409086Z    = note: hidden type `impl std::future::Future` captures lifetime '_#27r
2019-08-13T11:58:48.9409381Z error: lifetime may not live long enough
2019-08-13T11:58:48.9409710Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:15:5
2019-08-13T11:58:48.9409804Z    |
2019-08-13T11:58:48.9409804Z    |
2019-08-13T11:58:48.9410205Z LL |       async fn do_sth<'a>(
2019-08-13T11:58:48.9410501Z    |                       -- lifetime `'a` defined here
2019-08-13T11:58:48.9410920Z LL |           &'a self, foo: &dyn Foo
2019-08-13T11:58:48.9411400Z    |                          - lifetime `'_` defined here
2019-08-13T11:58:48.9419201Z LL |       ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-08-13T11:58:48.9419383Z LL | |         foo
2019-08-13T11:58:48.9419465Z LL | |     }
2019-08-13T11:58:48.9419465Z LL | |     }
2019-08-13T11:58:48.9419904Z    | |_____^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9420052Z error: aborting due to 2 previous errors
2019-08-13T11:58:48.9420094Z 
2019-08-13T11:58:48.9420383Z For more information about this error, try `rustc --explain E0700`.
2019-08-13T11:58:48.9420436Z 
2019-08-13T11:58:48.9420436Z 
2019-08-13T11:58:48.9420681Z ------------------------------------------
2019-08-13T11:58:48.9420727Z 
2019-08-13T11:58:48.9420761Z 
2019-08-13T11:58:48.9421041Z ---- [ui (nll)] ui/async-await/issues/issue-63388-2.rs stdout ----
2019-08-13T11:58:48.9421117Z diff of stderr:
2019-08-13T11:58:48.9421348Z 
2019-08-13T11:58:48.9421419Z 6    |
2019-08-13T11:58:48.9421827Z 7    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `foo` or `bar`
2019-08-13T11:58:48.9422194Z - error: cannot infer an appropriate lifetime
2019-08-13T11:58:48.9422434Z -   --> $DIR/issue-63388-2.rs:13:9
2019-08-13T11:58:48.9422652Z -    |
2019-08-13T11:58:48.9422652Z -    |
2019-08-13T11:58:48.9422891Z - LL |         foo: &dyn Foo, bar: &'a dyn Foo
2019-08-13T11:58:48.9423144Z -    |         ^^^ ...but this borrow...
2019-08-13T11:58:48.9423364Z - LL |     ) -> &dyn Foo
2019-08-13T11:58:48.9423887Z -    |          -------- this return type evaluates to the `'static` lifetime...
2019-08-13T11:58:48.9424133Z -    |
2019-08-13T11:58:48.9424423Z - note: ...can't outlive the lifetime '_ as defined on the method body at 13:14
2019-08-13T11:58:48.9424671Z -   --> $DIR/issue-63388-2.rs:13:14
2019-08-13T11:58:48.9424890Z -    |
2019-08-13T11:58:48.9425138Z - LL |         foo: &dyn Foo, bar: &'a dyn Foo
2019-08-13T11:58:48.9425526Z -    |              ^
2019-08-13T11:58:48.9425897Z - help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime '_ as defined on the method body at 13:14
2019-08-13T11:58:48.9426148Z -    |
2019-08-13T11:58:48.9426364Z - LL |     ) -> &dyn Foo + '_
2019-08-13T11:58:48.9426801Z - 
2019-08-13T11:58:48.9427048Z - error: aborting due to 2 previous errors
2019-08-13T11:58:48.9427120Z + error: aborting due to previous error
2019-08-13T11:58:48.9427203Z 28 
2019-08-13T11:58:48.9427203Z 28 
2019-08-13T11:58:48.9427473Z 29 For more information about this error, try `rustc --explain E0106`.
2019-08-13T11:58:48.9427566Z 30 
2019-08-13T11:58:48.9427602Z 
2019-08-13T11:58:48.9427636Z 
2019-08-13T11:58:48.9427719Z The actual stderr differed from the expected stderr.
2019-08-13T11:58:48.9428246Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2.nll/issue-63388-2.nll.stderr
2019-08-13T11:58:48.9428557Z To update references, rerun the tests and pass the `--bless` flag
2019-08-13T11:58:48.9428871Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-2.rs`
2019-08-13T11:58:48.9428988Z error: 1 errors occurred comparing output.
2019-08-13T11:58:48.9429071Z status: exit code: 1
2019-08-13T11:58:48.9429071Z status: exit code: 1
2019-08-13T11:58:48.9430006Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2.nll/auxiliary" "-A" "unused"
2019-08-13T11:58:48.9430504Z ------------------------------------------
2019-08-13T11:58:48.9430550Z 
2019-08-13T11:58:48.9430793Z ------------------------------------------
2019-08-13T11:58:48.9430860Z stderr:
2019-08-13T11:58:48.9430860Z stderr:
2019-08-13T11:58:48.9431094Z ------------------------------------------
2019-08-13T11:58:48.9431163Z error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9431447Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-2.rs:14:10
2019-08-13T11:58:48.9431521Z    |
2019-08-13T11:58:48.9431785Z LL |     ) -> &dyn Foo //~ ERROR missing lifetime specifier
2019-08-13T11:58:48.9432091Z    |          ^ help: consider using the named lifetime: `&'a`
2019-08-13T11:58:48.9432179Z    |
2019-08-13T11:58:48.9432507Z    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `foo` or `bar`
2019-08-13T11:58:48.9432750Z error: aborting due to previous error
2019-08-13T11:58:48.9432817Z 
2019-08-13T11:58:48.9433103Z For more information about this error, try `rustc --explain E0106`.
2019-08-13T11:58:48.9433170Z 
2019-08-13T11:58:48.9433170Z 
2019-08-13T11:58:48.9433567Z ------------------------------------------
2019-08-13T11:58:48.9433613Z 
2019-08-13T11:58:48.9433663Z 
2019-08-13T11:58:48.9434149Z ---- [ui (nll)] ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs stdout ----
2019-08-13T11:58:48.9434252Z diff of stderr:
2019-08-13T11:58:48.9434293Z 
2019-08-13T11:58:48.9434532Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9434635Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9434922Z 2   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:45
2019-08-13T11:58:48.9435014Z 3    |
2019-08-13T11:58:48.9435269Z 4 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-13T11:58:48.9435591Z -    |                                             ^
2019-08-13T11:58:48.9435793Z +    |                                             ^^^^
2019-08-13T11:58:48.9435864Z 6    |
2019-08-13T11:58:48.9435864Z 6    |
2019-08-13T11:58:48.9436227Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9436531Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-13T11:58:48.9436856Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9437307Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:60
2019-08-13T11:58:48.9437307Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:60
2019-08-13T11:58:48.9437383Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9437688Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-13T11:58:48.9437762Z 11    |
2019-08-13T11:58:48.9438053Z - LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T11:58:48.9438561Z -    |
2019-08-13T11:58:48.9438561Z -    |
2019-08-13T11:58:48.9438876Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9439149Z + LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-13T11:58:48.9439526Z +    |                          -                       ^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9439626Z +    |                          |
2019-08-13T11:58:48.9439888Z +    |                          lifetime `'_` defined here
2019-08-13T11:58:48.9440138Z +    |                          lifetime `'_` defined here
2019-08-13T11:58:48.9440445Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9443146Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:67
2019-08-13T11:58:48.9443146Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:67
2019-08-13T11:58:48.9443250Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9444365Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:13:73
2019-08-13T11:58:48.9444457Z 19    |
2019-08-13T11:58:48.9444775Z 20 LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T11:58:48.9445062Z -    |                                                                   ^
2019-08-13T11:58:48.9445498Z +    |                          -                                              ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9445629Z +    |                          |
2019-08-13T11:58:48.9445891Z +    |                          lifetime `'_` defined here
2019-08-13T11:58:48.9446179Z +    |                          lifetime `'_` defined here
2019-08-13T11:58:48.9446254Z + 
2019-08-13T11:58:48.9446346Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9446777Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:58
2019-08-13T11:58:48.9446901Z 22    |
2019-08-13T11:58:48.9447412Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9447720Z + LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-08-13T11:58:48.9447891Z +    |
2019-08-13T11:58:48.9447891Z +    |
2019-08-13T11:58:48.9448168Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-13T11:58:48.9448482Z - error: aborting due to 3 previous errors
2019-08-13T11:58:48.9448482Z - error: aborting due to 3 previous errors
2019-08-13T11:58:48.9448553Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9448831Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-13T11:58:48.9448905Z +    |
2019-08-13T11:58:48.9449176Z + LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-08-13T11:58:48.9449569Z +    |                  --              -                           ^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-08-13T11:58:48.9449785Z +    |                  |               |
2019-08-13T11:58:48.9450073Z +    |                  |               lifetime `'_` defined here
2019-08-13T11:58:48.9450409Z 26 
2019-08-13T11:58:48.9450684Z - For more information about this error, try `rustc --explain E0106`.
2019-08-13T11:58:48.9450759Z + error: aborting due to 5 previous errors
2019-08-13T11:58:48.9450842Z + 
2019-08-13T11:58:48.9450842Z + 
2019-08-13T11:58:48.9451097Z + For more information about this error, try `rustc --explain E0700`.
2019-08-13T11:58:48.9451191Z 28 
2019-08-13T11:58:48.9451227Z 
2019-08-13T11:58:48.9451276Z 
2019-08-13T11:58:48.9451341Z The actual stderr differed from the expected stderr.
2019-08-13T11:58:48.9451775Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/arbitrary_self_types_pin_lifetime_mismatch-async.nll.stderr
2019-08-13T11:58:48.9452084Z To update references, rerun the tests and pass the `--bless` flag
2019-08-13T11:58:48.9452415Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`
2019-08-13T11:58:48.9452555Z error: 1 errors occurred comparing output.
2019-08-13T11:58:48.9452621Z status: exit code: 1
2019-08-13T11:58:48.9452621Z status: exit code: 1
2019-08-13T11:58:48.9453983Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/auxiliary" "-A" "unused"
2019-08-13T11:58:48.9454628Z ------------------------------------------
2019-08-13T11:58:48.9454677Z 
2019-08-13T11:58:48.9454928Z ------------------------------------------
2019-08-13T11:58:48.9455013Z stderr:
2019-08-13T11:58:48.9455013Z stderr:
2019-08-13T11:58:48.9455240Z ------------------------------------------
2019-08-13T11:58:48.9455343Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9455655Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:45
2019-08-13T11:58:48.9455754Z    |
2019-08-13T11:58:48.9456005Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-13T11:58:48.9456174Z    |
2019-08-13T11:58:48.9456174Z    |
2019-08-13T11:58:48.9456561Z    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-13T11:58:48.9456712Z error: lifetime may not live long enough
2019-08-13T11:58:48.9457043Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:50
2019-08-13T11:58:48.9457139Z    |
2019-08-13T11:58:48.9457139Z    |
2019-08-13T11:58:48.9457392Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-13T11:58:48.9457880Z    |                          -                       ^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9457984Z    |                          |
2019-08-13T11:58:48.9458265Z    |                          lifetime `'_` defined here
2019-08-13T11:58:48.9458541Z    |                          lifetime `'_` defined here
2019-08-13T11:58:48.9458650Z error: lifetime may not live long enough
2019-08-13T11:58:48.9458972Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:13:73
2019-08-13T11:58:48.9459144Z    |
2019-08-13T11:58:48.9459144Z    |
2019-08-13T11:58:48.9459474Z LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T11:58:48.9459891Z    |                          -                                              ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9460018Z    |                          |
2019-08-13T11:58:48.9460291Z    |                          lifetime `'_` defined here
2019-08-13T11:58:48.9460551Z    |                          lifetime `'_` defined here
2019-08-13T11:58:48.9460600Z 
2019-08-13T11:58:48.9460693Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9461150Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:58
2019-08-13T11:58:48.9461245Z    |
2019-08-13T11:58:48.9461544Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-13T11:58:48.9461724Z    |
2019-08-13T11:58:48.9461724Z    |
2019-08-13T11:58:48.9461988Z    = note: hidden type `impl std::future::Future` captures lifetime '_#18r
2019-08-13T11:58:48.9462116Z error: lifetime may not live long enough
2019-08-13T11:58:48.9470103Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:19:62
2019-08-13T11:58:48.9470255Z    |
2019-08-13T11:58:48.9470255Z    |
2019-08-13T11:58:48.9470659Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-13T11:58:48.9471690Z    |                  --              -                           ^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-08-13T11:58:48.9471827Z    |                  |               |
2019-08-13T11:58:48.9472140Z    |                  |               lifetime `'_` defined here
2019-08-13T11:58:48.9472499Z 
2019-08-13T11:58:48.9472561Z error: aborting due to 5 previous errors
2019-08-13T11:58:48.9472621Z 
2019-08-13T11:58:48.9472892Z For more information about this error, try `rustc --explain E0700`.
2019-08-13T11:58:48.9472892Z For more information about this error, try `rustc --explain E0700`.
2019-08-13T11:58:48.9472966Z 
2019-08-13T11:58:48.9473197Z ------------------------------------------
2019-08-13T11:58:48.9473243Z 
2019-08-13T11:58:48.9473295Z 
2019-08-13T11:58:48.9473553Z ---- [ui (nll)] ui/self/elision/lt-ref-self-async.rs stdout ----
2019-08-13T11:58:48.9473829Z diff of stderr:
2019-08-13T11:58:48.9473873Z 
2019-08-13T11:58:48.9474209Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9474318Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9474815Z 2   --> $DIR/lt-ref-self-async.rs:15:42
2019-08-13T11:58:48.9474915Z 3    |
2019-08-13T11:58:48.9475177Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-13T11:58:48.9476988Z -    |                                          ^
2019-08-13T11:58:48.9477105Z +    |                                          ^^^^
2019-08-13T11:58:48.9477174Z 6    |
2019-08-13T11:58:48.9477174Z 6    |
2019-08-13T11:58:48.9477581Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9477889Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9478219Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9478476Z -   --> $DIR/lt-ref-self-async.rs:23:48
2019-08-13T11:58:48.9478476Z -   --> $DIR/lt-ref-self-async.rs:23:48
2019-08-13T11:58:48.9478548Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9478800Z +   --> $DIR/lt-ref-self-async.rs:15:47
2019-08-13T11:58:48.9478869Z 11    |
2019-08-13T11:58:48.9479139Z + LL |       async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-13T11:58:48.9479395Z +    |  _______________________-_______________________^
2019-08-13T11:58:48.9479499Z +    | |                       |
2019-08-13T11:58:48.9479915Z +    | |                       lifetime `'_` defined here
2019-08-13T11:58:48.9480198Z +    | |                       lifetime `'_` defined here
2019-08-13T11:58:48.9480271Z + LL | |         f
2019-08-13T11:58:48.9480353Z + LL | |     }
2019-08-13T11:58:48.9480671Z +    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9480776Z + 
2019-08-13T11:58:48.9480869Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9481121Z +   --> $DIR/lt-ref-self-async.rs:21:48
2019-08-13T11:58:48.9481294Z +    |
2019-08-13T11:58:48.9481553Z 12 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-13T11:58:48.9481915Z +    |                                                ^^^^
2019-08-13T11:58:48.9482005Z 14    |
2019-08-13T11:58:48.9482005Z 14    |
2019-08-13T11:58:48.9482326Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9482653Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9482986Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9483225Z -   --> $DIR/lt-ref-self-async.rs:29:57
2019-08-13T11:58:48.9483225Z -   --> $DIR/lt-ref-self-async.rs:29:57
2019-08-13T11:58:48.9483546Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9484151Z +   --> $DIR/lt-ref-self-async.rs:21:53
2019-08-13T11:58:48.9484246Z 19    |
2019-08-13T11:58:48.9484510Z + LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-13T11:58:48.9484794Z +    |  _____________________________-_______________________^
2019-08-13T11:58:48.9484868Z +    | |                             |
2019-08-13T11:58:48.9485150Z +    | |                             lifetime `'_` defined here
2019-08-13T11:58:48.9485416Z +    | |                             lifetime `'_` defined here
2019-08-13T11:58:48.9485532Z + LL | |         f
2019-08-13T11:58:48.9485596Z + LL | |     }
2019-08-13T11:58:48.9485934Z +    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9486022Z + 
2019-08-13T11:58:48.9486117Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9486371Z +   --> $DIR/lt-ref-self-async.rs:25:57
2019-08-13T11:58:48.9486458Z +    |
2019-08-13T11:58:48.9486725Z 20 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9487120Z +    |                                                         ^^^^
2019-08-13T11:58:48.9487194Z 22    |
2019-08-13T11:58:48.9487194Z 22    |
2019-08-13T11:58:48.9487521Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9487939Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9488325Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9488579Z -   --> $DIR/lt-ref-self-async.rs:35:57
2019-08-13T11:58:48.9488579Z -   --> $DIR/lt-ref-self-async.rs:35:57
2019-08-13T11:58:48.9488652Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9488911Z +   --> $DIR/lt-ref-self-async.rs:25:62
2019-08-13T11:58:48.9488980Z 27    |
2019-08-13T11:58:48.9489266Z + LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9489548Z +    |  _____________________________________-________________________^
2019-08-13T11:58:48.9489647Z +    | |                                     |
2019-08-13T11:58:48.9489921Z +    | |                                     lifetime `'_` defined here
2019-08-13T11:58:48.9490217Z +    | |                                     lifetime `'_` defined here
2019-08-13T11:58:48.9490293Z + LL | |         f
2019-08-13T11:58:48.9490374Z + LL | |     }
2019-08-13T11:58:48.9490701Z +    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9490900Z + 
2019-08-13T11:58:48.9490976Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9491274Z +   --> $DIR/lt-ref-self-async.rs:29:57
2019-08-13T11:58:48.9491348Z +    |
2019-08-13T11:58:48.9491630Z 28 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9492004Z +    |                                                         ^^^^
2019-08-13T11:58:48.9492094Z 30    |
2019-08-13T11:58:48.9492094Z 30    |
2019-08-13T11:58:48.9492407Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9492722Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9493050Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9493309Z -   --> $DIR/lt-ref-self-async.rs:41:66
2019-08-13T11:58:48.9493309Z -   --> $DIR/lt-ref-self-async.rs:41:66
2019-08-13T11:58:48.9493400Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9493817Z +   --> $DIR/lt-ref-self-async.rs:29:62
2019-08-13T11:58:48.9493910Z 35    |
2019-08-13T11:58:48.9494206Z + LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9494504Z +    |  _____________________________________-________________________^
2019-08-13T11:58:48.9494585Z +    | |                                     |
2019-08-13T11:58:48.9494873Z +    | |                                     lifetime `'_` defined here
2019-08-13T11:58:48.9495149Z +    | |                                     lifetime `'_` defined here
2019-08-13T11:58:48.9495243Z + LL | |         f
2019-08-13T11:58:48.9495307Z + LL | |     }
2019-08-13T11:58:48.9495642Z +    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9495729Z + 
2019-08-13T11:58:48.9495830Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9496096Z +   --> $DIR/lt-ref-self-async.rs:33:66
2019-08-13T11:58:48.9496183Z +    |
2019-08-13T11:58:48.9496457Z 36 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9497398Z +    |                                                                  ^^^^
2019-08-13T11:58:48.9497500Z 38    |
2019-08-13T11:58:48.9497500Z 38    |
2019-08-13T11:58:48.9497833Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9498155Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9498484Z - error[E0106]: missing lifetime specifier
2019-08-13T11:58:48.9498741Z -   --> $DIR/lt-ref-self-async.rs:47:62
2019-08-13T11:58:48.9498741Z -   --> $DIR/lt-ref-self-async.rs:47:62
2019-08-13T11:58:48.9498924Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9499247Z +   --> $DIR/lt-ref-self-async.rs:33:71
2019-08-13T11:58:48.9499316Z 43    |
2019-08-13T11:58:48.9499612Z + LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9499904Z +    |  _____________________________________________-_________________________^
2019-08-13T11:58:48.9500008Z +    | |                                             |
2019-08-13T11:58:48.9500293Z +    | |                                             lifetime `'_` defined here
2019-08-13T11:58:48.9500600Z +    | |                                             lifetime `'_` defined here
2019-08-13T11:58:48.9500678Z + LL | |         f
2019-08-13T11:58:48.9500758Z + LL | |     }
2019-08-13T11:58:48.9501076Z +    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9501181Z + 
2019-08-13T11:58:48.9501264Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9501648Z +   --> $DIR/lt-ref-self-async.rs:37:62
2019-08-13T11:58:48.9501718Z +    |
2019-08-13T11:58:48.9502171Z 44 LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9502543Z +    |                                                              ^^^^
2019-08-13T11:58:48.9502613Z 46    |
2019-08-13T11:58:48.9502613Z 46    |
2019-08-13T11:58:48.9502937Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T11:58:48.9503242Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9503936Z - error: aborting due to 6 previous errors
2019-08-13T11:58:48.9503936Z - error: aborting due to 6 previous errors
2019-08-13T11:58:48.9504020Z + error: lifetime may not live long enough
2019-08-13T11:58:48.9504415Z +   --> $DIR/lt-ref-self-async.rs:37:67
2019-08-13T11:58:48.9504483Z +    |
2019-08-13T11:58:48.9504785Z + LL |       async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9505085Z +    |  _________________________________________-_________________________^
2019-08-13T11:58:48.9505187Z +    | |                                         |
2019-08-13T11:58:48.9505463Z +    | |                                         lifetime `'_` defined here
2019-08-13T11:58:48.9505763Z +    | |                                         lifetime `'_` defined here
2019-08-13T11:58:48.9505840Z + LL | |         f
2019-08-13T11:58:48.9505919Z + LL | |     }
2019-08-13T11:58:48.9506236Z +    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9506604Z - For more information about this error, try `rustc --explain E0106`.
2019-08-13T11:58:48.9506701Z + error: aborting due to 12 previous errors
2019-08-13T11:58:48.9506767Z + 
2019-08-13T11:58:48.9507055Z + For more information about this error, try `rustc --explain E0700`.
2019-08-13T11:58:48.9507055Z + For more information about this error, try `rustc --explain E0700`.
2019-08-13T11:58:48.9507137Z 52 
2019-08-13T11:58:48.9507192Z 
2019-08-13T11:58:48.9507226Z 
2019-08-13T11:58:48.9507455Z The actual stderr differed from the expected stderr.
2019-08-13T11:58:48.9507823Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/lt-ref-self-async.nll.stderr
2019-08-13T11:58:48.9508180Z To update references, rerun the tests and pass the `--bless` flag
2019-08-13T11:58:48.9508472Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`
2019-08-13T11:58:48.9508606Z error: 1 errors occurred comparing output.
2019-08-13T11:58:48.9508671Z status: exit code: 1
2019-08-13T11:58:48.9508671Z status: exit code: 1
2019-08-13T11:58:48.9510321Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/auxiliary" "-A" "unused"
2019-08-13T11:58:48.9510957Z ------------------------------------------
2019-08-13T11:58:48.9511028Z 
2019-08-13T11:58:48.9511291Z ------------------------------------------
2019-08-13T11:58:48.9511381Z stderr:
2019-08-13T11:58:48.9511381Z stderr:
2019-08-13T11:58:48.9511626Z ------------------------------------------
2019-08-13T11:58:48.9511737Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9512051Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:15:42
2019-08-13T11:58:48.9512151Z    |
2019-08-13T11:58:48.9512420Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-13T11:58:48.9512697Z    |
2019-08-13T11:58:48.9512697Z    |
2019-08-13T11:58:48.9513036Z    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9513208Z error: lifetime may not live long enough
2019-08-13T11:58:48.9513503Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:15:47
2019-08-13T11:58:48.9513600Z    |
2019-08-13T11:58:48.9513600Z    |
2019-08-13T11:58:48.9514065Z LL |       async fn ref_self(&self, f: &u32) -> &u32 {
2019-08-13T11:58:48.9514475Z    |  _______________________-_______________________^
2019-08-13T11:58:48.9514548Z    | |                       |
2019-08-13T11:58:48.9514819Z    | |                       lifetime `'_` defined here
2019-08-13T11:58:48.9515075Z    | |                       lifetime `'_` defined here
2019-08-13T11:58:48.9515171Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-13T11:58:48.9515240Z LL | |     }
2019-08-13T11:58:48.9515581Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9515656Z 
2019-08-13T11:58:48.9515749Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9516038Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:21:48
2019-08-13T11:58:48.9516131Z    |
2019-08-13T11:58:48.9516384Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-13T11:58:48.9516552Z    |
2019-08-13T11:58:48.9516552Z    |
2019-08-13T11:58:48.9516840Z    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9516973Z error: lifetime may not live long enough
2019-08-13T11:58:48.9517242Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:21:53
2019-08-13T11:58:48.9517453Z    |
2019-08-13T11:58:48.9517453Z    |
2019-08-13T11:58:48.9517970Z LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-13T11:58:48.9518248Z    |  _____________________________-_______________________^
2019-08-13T11:58:48.9518328Z    | |                             |
2019-08-13T11:58:48.9519189Z    | |                             lifetime `'_` defined here
2019-08-13T11:58:48.9519465Z    | |                             lifetime `'_` defined here
2019-08-13T11:58:48.9519561Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-13T11:58:48.9519628Z LL | |     }
2019-08-13T11:58:48.9519960Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9520025Z 
2019-08-13T11:58:48.9520118Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-08-13T11:58:48.9520405Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:25:57
2019-08-13T11:58:48.9520497Z    |
2019-08-13T11:58:48.9520760Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9521062Z    |
2019-08-13T11:58:48.9521062Z    |
2019-08-13T11:58:48.9521385Z    = note: hidden type `impl std::future::Future` captures lifetime '_#28r
2019-08-13T11:58:48.9521522Z error: lifetime may not live long enough
2019-08-13T11:58:48.9521794Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:25:62
2019-08-13T11:58:48.9521884Z    |
2019-08-13T11:58:48.9521884Z    |
2019-08-13T11:58:48.9522148Z LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-13T11:58:48.9522439Z    |  _____________________________________-________________________^
2019-08-13T11:58:48.9522518Z    | |                                     |
2019-08-13T11:58:48.9522807Z    | |                                     lifetime `'_` defined here
2019-08-13T11:58:48.9523082Z    | |                                     lifetime `'_` defined here
2019-08-13T11:58:48.9523194Z LL | |         f //~^ ERROR lifetime mismatch
2019-08-13T11:58:48.9523261Z LL | |     }
2019-08-13T11:58:48.9524026Z    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-08-13T11:58:48.9524326Z 
---
2019-08-13T11:58:48.9809241Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-13T11:58:48.9809355Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-13T11:58:48.9809407Z 
2019-08-13T11:58:48.9809440Z 
2019-08-13T11:58:48.9811329Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-13T11:58:48.9812157Z 
2019-08-13T11:58:48.9812196Z 
2019-08-13T11:58:48.9812282Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-13T11:58:48.9812375Z Build completed unsuccessfully in 1:51:44
2019-08-13T11:58:48.9812375Z Build completed unsuccessfully in 1:51:44
2019-08-13T11:58:49.7007028Z ##[error]Bash exited with code '1'.
2019-08-13T11:58:49.7077341Z ##[section]Starting: Upload CPU usage statistics
2019-08-13T11:58:49.7084064Z ==============================================================================
2019-08-13T11:58:49.7084158Z Task         : Bash
2019-08-13T11:58:49.7084247Z Description  : Run a Bash script on macOS, Linux, or Windows
