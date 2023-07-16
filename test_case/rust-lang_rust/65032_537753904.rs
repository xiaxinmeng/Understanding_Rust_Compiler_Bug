plain
2019-10-03T01:59:12.3876427Z 
2019-10-03T01:59:12.3880583Z ---- [ui (nll)] ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs stdout ----
2019-10-03T01:59:12.3881330Z diff of stderr:
2019-10-03T01:59:12.3881601Z 
2019-10-03T01:59:12.3882267Z 7    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.3882982Z 9 error: lifetime may not live long enough
2019-10-03T01:59:12.3883621Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:50
2019-10-03T01:59:12.3884290Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:52
2019-10-03T01:59:12.3885050Z 11    |
2019-10-03T01:59:12.3885050Z 11    |
2019-10-03T01:59:12.3897869Z 12 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-10-03T01:59:12.3898830Z -    |                          -                       ^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3899968Z +    |                          -                         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3900738Z 15    |                          lifetime `'_` defined here
2019-10-03T01:59:12.3901238Z 16    |                          lifetime `'_` defined here
2019-10-03T01:59:12.3901438Z 
2019-10-03T01:59:12.3901626Z 17 
2019-10-03T01:59:12.3901626Z 17 
2019-10-03T01:59:12.3901806Z 18 error: lifetime may not live long enough
2019-10-03T01:59:12.3902277Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:73
2019-10-03T01:59:12.3902787Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:75
2019-10-03T01:59:12.3903012Z 20    |
2019-10-03T01:59:12.3903483Z 21 LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-10-03T01:59:12.3904133Z -    |                          -                                              ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3904814Z +    |                          -                                                ^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3906214Z 24    |                          lifetime `'_` defined here
2019-10-03T01:59:12.3907038Z 25    |                          lifetime `'_` defined here
2019-10-03T01:59:12.3907510Z 
2019-10-03T01:59:12.3907510Z 
2019-10-03T01:59:12.3908235Z 33    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.3909264Z 35 error: lifetime may not live long enough
2019-10-03T01:59:12.3910136Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:62
2019-10-03T01:59:12.3911008Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:64
2019-10-03T01:59:12.3911540Z 37    |
2019-10-03T01:59:12.3911540Z 37    |
2019-10-03T01:59:12.3912062Z 38 LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-10-03T01:59:12.3913030Z -    |                  --              -                           ^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-10-03T01:59:12.3914471Z -    |                  |               lifetime `'_` defined here
2019-10-03T01:59:12.3914471Z -    |                  |               lifetime `'_` defined here
2019-10-03T01:59:12.3915772Z +    |                  --              - lifetime `'_` defined here  ^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-10-03T01:59:12.3916644Z 42    |                  lifetime `'a` defined here
2019-10-03T01:59:12.3916880Z 43 
2019-10-03T01:59:12.3917087Z 44 error: aborting due to 5 previous errors
2019-10-03T01:59:12.3917269Z 
2019-10-03T01:59:12.3917269Z 
2019-10-03T01:59:12.3917440Z 
2019-10-03T01:59:12.3917631Z The actual stderr differed from the expected stderr.
2019-10-03T01:59:12.3918291Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/arbitrary_self_types_pin_lifetime_mismatch-async.nll.stderr
2019-10-03T01:59:12.3918909Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T01:59:12.3919520Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`
2019-10-03T01:59:12.3919938Z error: 1 errors occurred comparing output.
2019-10-03T01:59:12.3920144Z status: exit code: 1
2019-10-03T01:59:12.3920144Z status: exit code: 1
2019-10-03T01:59:12.3921399Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/auxiliary" "-A" "unused"
2019-10-03T01:59:12.3922600Z ------------------------------------------
2019-10-03T01:59:12.3922816Z 
2019-10-03T01:59:12.3923256Z ------------------------------------------
2019-10-03T01:59:12.3923494Z stderr:
2019-10-03T01:59:12.3923494Z stderr:
2019-10-03T01:59:12.3923927Z ------------------------------------------
2019-10-03T01:59:12.3924312Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.3925559Z    |
2019-10-03T01:59:12.3925559Z    |
2019-10-03T01:59:12.3926120Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-10-03T01:59:12.3926584Z    |
2019-10-03T01:59:12.3926584Z    |
2019-10-03T01:59:12.3927036Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.3927460Z error: lifetime may not live long enough
2019-10-03T01:59:12.3927956Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:52
2019-10-03T01:59:12.3928215Z    |
2019-10-03T01:59:12.3928215Z    |
2019-10-03T01:59:12.3928644Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-10-03T01:59:12.3929259Z    |                          -                         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3934178Z    |                          lifetime `'_` defined here
2019-10-03T01:59:12.3934794Z    |                          lifetime `'_` defined here
2019-10-03T01:59:12.3935518Z 
2019-10-03T01:59:12.3935754Z error: lifetime may not live long enough
2019-10-03T01:59:12.3935754Z error: lifetime may not live long enough
2019-10-03T01:59:12.3936324Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:75
2019-10-03T01:59:12.3936588Z    |
2019-10-03T01:59:12.3937094Z LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-10-03T01:59:12.3937943Z    |                          -                                                ^^^^^^^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3938801Z    |                          lifetime `'_` defined here
2019-10-03T01:59:12.3939298Z    |                          lifetime `'_` defined here
2019-10-03T01:59:12.3939503Z 
2019-10-03T01:59:12.3939503Z 
2019-10-03T01:59:12.3939695Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.3948317Z    |
2019-10-03T01:59:12.3948317Z    |
2019-10-03T01:59:12.3948922Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-10-03T01:59:12.3949148Z    |
2019-10-03T01:59:12.3949148Z    |
2019-10-03T01:59:12.3949500Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.3949661Z error: lifetime may not live long enough
2019-10-03T01:59:12.3949976Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:64
2019-10-03T01:59:12.3950080Z    |
2019-10-03T01:59:12.3950080Z    |
2019-10-03T01:59:12.3950390Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-10-03T01:59:12.3950810Z    |                  --              - lifetime `'_` defined here  ^^^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'a`
2019-10-03T01:59:12.3954679Z    |                  lifetime `'a` defined here
2019-10-03T01:59:12.3954752Z 
2019-10-03T01:59:12.3954819Z error: aborting due to 5 previous errors
2019-10-03T01:59:12.3954880Z 
---
2019-10-03T01:59:12.3956263Z 
2019-10-03T01:59:12.3956539Z ---- [ui (nll)] ui/self/elision/lt-ref-self-async.rs stdout ----
2019-10-03T01:59:12.3956637Z diff of stderr:
2019-10-03T01:59:12.3956679Z 
2019-10-03T01:59:12.3956977Z 7    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.3957155Z 9 error: lifetime may not live long enough
2019-10-03T01:59:12.3957436Z -   --> $DIR/lt-ref-self-async.rs:13:47
2019-10-03T01:59:12.3957736Z +   --> $DIR/lt-ref-self-async.rs:14:9
2019-10-03T01:59:12.3957813Z 11    |
2019-10-03T01:59:12.3957813Z 11    |
2019-10-03T01:59:12.3958112Z - LL |       async fn ref_self(&self, f: &u32) -> &u32 {
2019-10-03T01:59:12.3958409Z -    |  _______________________-_______________________^
2019-10-03T01:59:12.3958994Z -    | |                       lifetime `'_` defined here
2019-10-03T01:59:12.3959293Z -    | |                       lifetime `'_` defined here
2019-10-03T01:59:12.3973151Z - LL | |         f
2019-10-03T01:59:12.3973843Z - LL | |     }
2019-10-03T01:59:12.3973843Z - LL | |     }
2019-10-03T01:59:12.3974288Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3974593Z + LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-10-03T01:59:12.3975245Z +    |                       |
2019-10-03T01:59:12.3975606Z +    |                       lifetime `'_` defined here
2019-10-03T01:59:12.3975911Z +    |                       lifetime `'_` defined here
2019-10-03T01:59:12.3975990Z + LL |         f
2019-10-03T01:59:12.3975990Z + LL |         f
2019-10-03T01:59:12.3977468Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3977592Z 20 
2019-10-03T01:59:12.3977700Z 21 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.3978009Z 22   --> $DIR/lt-ref-self-async.rs:19:48
2019-10-03T01:59:12.3978275Z 
2019-10-03T01:59:12.3978633Z 27    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.3978821Z 29 error: lifetime may not live long enough
2019-10-03T01:59:12.3979077Z -   --> $DIR/lt-ref-self-async.rs:19:53
2019-10-03T01:59:12.3979338Z +   --> $DIR/lt-ref-self-async.rs:20:9
2019-10-03T01:59:12.3979411Z 31    |
2019-10-03T01:59:12.3979411Z 31    |
2019-10-03T01:59:12.3979696Z - LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-10-03T01:59:12.3979989Z -    |  _____________________________-_______________________^
2019-10-03T01:59:12.3980531Z -    | |                             lifetime `'_` defined here
2019-10-03T01:59:12.3980827Z -    | |                             lifetime `'_` defined here
2019-10-03T01:59:12.3981071Z - LL | |         f
2019-10-03T01:59:12.3981288Z - LL | |     }
2019-10-03T01:59:12.3981288Z - LL | |     }
2019-10-03T01:59:12.3981639Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3982755Z + LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-10-03T01:59:12.3983151Z +    |                             |
2019-10-03T01:59:12.3983439Z +    |                             lifetime `'_` defined here
2019-10-03T01:59:12.3983753Z +    |                             lifetime `'_` defined here
2019-10-03T01:59:12.3983832Z + LL |         f
2019-10-03T01:59:12.3983832Z + LL |         f
2019-10-03T01:59:12.3984409Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3984508Z 40 
2019-10-03T01:59:12.3984607Z 41 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.3984886Z 42   --> $DIR/lt-ref-self-async.rs:23:57
2019-10-03T01:59:12.3985301Z 
2019-10-03T01:59:12.3985660Z 47    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.3985841Z 49 error: lifetime may not live long enough
2019-10-03T01:59:12.3986146Z -   --> $DIR/lt-ref-self-async.rs:23:62
2019-10-03T01:59:12.3986419Z +   --> $DIR/lt-ref-self-async.rs:24:9
2019-10-03T01:59:12.3986513Z 51    |
2019-10-03T01:59:12.3986513Z 51    |
2019-10-03T01:59:12.3986826Z - LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.3987143Z -    |  _____________________________________-________________________^
2019-10-03T01:59:12.3987770Z -    | |                                     lifetime `'_` defined here
2019-10-03T01:59:12.3988100Z -    | |                                     lifetime `'_` defined here
2019-10-03T01:59:12.3988356Z - LL | |         f
2019-10-03T01:59:12.3988610Z - LL | |     }
2019-10-03T01:59:12.3988610Z - LL | |     }
2019-10-03T01:59:12.3989300Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3989664Z + LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.3990052Z +    |                                     |
2019-10-03T01:59:12.3990374Z +    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.3990683Z +    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.3990786Z + LL |         f
2019-10-03T01:59:12.3990786Z + LL |         f
2019-10-03T01:59:12.3991151Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3991260Z 60 
2019-10-03T01:59:12.3991362Z 61 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.3991656Z 62   --> $DIR/lt-ref-self-async.rs:27:57
2019-10-03T01:59:12.3991724Z 
2019-10-03T01:59:12.3992034Z 67    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.3992371Z 69 error: lifetime may not live long enough
2019-10-03T01:59:12.3992714Z -   --> $DIR/lt-ref-self-async.rs:27:62
2019-10-03T01:59:12.3993245Z +   --> $DIR/lt-ref-self-async.rs:28:9
2019-10-03T01:59:12.3993354Z 71    |
2019-10-03T01:59:12.3993354Z 71    |
2019-10-03T01:59:12.3993751Z - LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.3994895Z -    |  _____________________________________-________________________^
2019-10-03T01:59:12.3995878Z -    | |                                     lifetime `'_` defined here
2019-10-03T01:59:12.3996190Z -    | |                                     lifetime `'_` defined here
2019-10-03T01:59:12.3996424Z - LL | |         f
2019-10-03T01:59:12.3996656Z - LL | |     }
2019-10-03T01:59:12.3996656Z - LL | |     }
2019-10-03T01:59:12.3996984Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3997321Z + LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.3997687Z +    |                                     |
2019-10-03T01:59:12.3997984Z +    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.3998273Z +    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.3998370Z + LL |         f
2019-10-03T01:59:12.3998370Z + LL |         f
2019-10-03T01:59:12.3998712Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.3999228Z 80 
2019-10-03T01:59:12.3999336Z 81 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.3999693Z 82   --> $DIR/lt-ref-self-async.rs:31:66
2019-10-03T01:59:12.3999764Z 
2019-10-03T01:59:12.4000051Z 87    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.4000218Z 89 error: lifetime may not live long enough
2019-10-03T01:59:12.4000495Z -   --> $DIR/lt-ref-self-async.rs:31:71
2019-10-03T01:59:12.4000745Z +   --> $DIR/lt-ref-self-async.rs:32:9
2019-10-03T01:59:12.4000835Z 91    |
2019-10-03T01:59:12.4000835Z 91    |
2019-10-03T01:59:12.4001126Z - LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4001445Z -    |  _____________________________________________-_________________________^
2019-10-03T01:59:12.4002033Z -    | |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4002362Z -    | |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4002597Z - LL | |         f
2019-10-03T01:59:12.4002831Z - LL | |     }
2019-10-03T01:59:12.4002831Z - LL | |     }
2019-10-03T01:59:12.4003156Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4003499Z + LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4003876Z +    |                                             |
2019-10-03T01:59:12.4004182Z +    |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4004479Z +    |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4004581Z + LL |         f
2019-10-03T01:59:12.4004581Z + LL |         f
2019-10-03T01:59:12.4005806Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4005983Z 100 
2019-10-03T01:59:12.4006082Z 101 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4006389Z 102   --> $DIR/lt-ref-self-async.rs:35:62
2019-10-03T01:59:12.4006457Z 
2019-10-03T01:59:12.4006744Z 107    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.4007087Z 109 error: lifetime may not live long enough
2019-10-03T01:59:12.4007414Z -   --> $DIR/lt-ref-self-async.rs:35:67
2019-10-03T01:59:12.4007664Z +   --> $DIR/lt-ref-self-async.rs:36:9
2019-10-03T01:59:12.4007752Z 111    |
2019-10-03T01:59:12.4007752Z 111    |
2019-10-03T01:59:12.4008035Z - LL |       async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4008349Z -    |  _________________________________________-_________________________^
2019-10-03T01:59:12.4008935Z -    | |                                         lifetime `'_` defined here
2019-10-03T01:59:12.4009486Z -    | |                                         lifetime `'_` defined here
2019-10-03T01:59:12.4009737Z - LL | |         f
2019-10-03T01:59:12.4009970Z - LL | |     }
2019-10-03T01:59:12.4009970Z - LL | |     }
2019-10-03T01:59:12.4010294Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4010633Z + LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4011004Z +    |                                         |
2019-10-03T01:59:12.4011306Z +    |                                         lifetime `'_` defined here
2019-10-03T01:59:12.4011597Z +    |                                         lifetime `'_` defined here
2019-10-03T01:59:12.4011695Z + LL |         f
2019-10-03T01:59:12.4011695Z + LL |         f
2019-10-03T01:59:12.4012022Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4012347Z 121 error: aborting due to 12 previous errors
2019-10-03T01:59:12.4012418Z 122 
2019-10-03T01:59:12.4012456Z 
2019-10-03T01:59:12.4012507Z 
2019-10-03T01:59:12.4012507Z 
2019-10-03T01:59:12.4012576Z The actual stderr differed from the expected stderr.
2019-10-03T01:59:12.4013016Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/lt-ref-self-async.nll.stderr
2019-10-03T01:59:12.4013557Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T01:59:12.4013904Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`
2019-10-03T01:59:12.4014053Z error: 1 errors occurred comparing output.
2019-10-03T01:59:12.4014142Z status: exit code: 1
2019-10-03T01:59:12.4014142Z status: exit code: 1
2019-10-03T01:59:12.4015505Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/auxiliary" "-A" "unused"
2019-10-03T01:59:12.4016185Z ------------------------------------------
2019-10-03T01:59:12.4016242Z 
2019-10-03T01:59:12.4016508Z ------------------------------------------
2019-10-03T01:59:12.4016584Z stderr:
2019-10-03T01:59:12.4016584Z stderr:
2019-10-03T01:59:12.4016841Z ------------------------------------------
2019-10-03T01:59:12.4016936Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4017363Z    |
2019-10-03T01:59:12.4017363Z    |
2019-10-03T01:59:12.4017884Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4018058Z    |
2019-10-03T01:59:12.4018058Z    |
2019-10-03T01:59:12.4018355Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.4018497Z error: lifetime may not live long enough
2019-10-03T01:59:12.4018913Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:14:9
2019-10-03T01:59:12.4019025Z    |
2019-10-03T01:59:12.4019025Z    |
2019-10-03T01:59:12.4019314Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4019648Z    |                       |
2019-10-03T01:59:12.4019926Z    |                       lifetime `'_` defined here
2019-10-03T01:59:12.4020207Z    |                       lifetime `'_` defined here
2019-10-03T01:59:12.4020287Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4020287Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4020652Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4020727Z 
2019-10-03T01:59:12.4020824Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4021217Z    |
2019-10-03T01:59:12.4021217Z    |
2019-10-03T01:59:12.4021715Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4021911Z    |
2019-10-03T01:59:12.4021911Z    |
2019-10-03T01:59:12.4022214Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.4022357Z error: lifetime may not live long enough
2019-10-03T01:59:12.4022640Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:20:9
2019-10-03T01:59:12.4022734Z    |
2019-10-03T01:59:12.4022734Z    |
2019-10-03T01:59:12.4022994Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4023509Z    |                             |
2019-10-03T01:59:12.4023783Z    |                             lifetime `'_` defined here
2019-10-03T01:59:12.4024074Z    |                             lifetime `'_` defined here
2019-10-03T01:59:12.4024157Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4024157Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4024516Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4024589Z 
2019-10-03T01:59:12.4024686Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4025629Z    |
2019-10-03T01:59:12.4025629Z    |
2019-10-03T01:59:12.4025985Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4026203Z    |
2019-10-03T01:59:12.4026203Z    |
2019-10-03T01:59:12.4026489Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.4026627Z error: lifetime may not live long enough
2019-10-03T01:59:12.4026923Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:24:9
2019-10-03T01:59:12.4027002Z    |
2019-10-03T01:59:12.4027002Z    |
2019-10-03T01:59:12.4027289Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4027647Z    |                                     |
2019-10-03T01:59:12.4027931Z    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4028231Z    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4028331Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4028331Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4029314Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4029451Z 
2019-10-03T01:59:12.4029534Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4030005Z    |
2019-10-03T01:59:12.4030005Z    |
2019-10-03T01:59:12.4030291Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4030683Z    |
2019-10-03T01:59:12.4030683Z    |
2019-10-03T01:59:12.4031160Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.4031320Z error: lifetime may not live long enough
2019-10-03T01:59:12.4031655Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:28:9
2019-10-03T01:59:12.4031734Z    |
2019-10-03T01:59:12.4031734Z    |
2019-10-03T01:59:12.4032018Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4032378Z    |                                     |
2019-10-03T01:59:12.4032675Z    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4032959Z    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4033058Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4033058Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4033389Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4033477Z 
2019-10-03T01:59:12.4033566Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4033975Z    |
2019-10-03T01:59:12.4033975Z    |
2019-10-03T01:59:12.4034258Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4034454Z    |
2019-10-03T01:59:12.4034454Z    |
2019-10-03T01:59:12.4035533Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.4035686Z error: lifetime may not live long enough
2019-10-03T01:59:12.4035980Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:32:9
2019-10-03T01:59:12.4036074Z    |
2019-10-03T01:59:12.4036074Z    |
2019-10-03T01:59:12.4036357Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4036745Z    |                                             |
2019-10-03T01:59:12.4037055Z    |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4037368Z    |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4037456Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4037456Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4037804Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4037886Z 
2019-10-03T01:59:12.4037982Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4038378Z    |
2019-10-03T01:59:12.4038378Z    |
2019-10-03T01:59:12.4038655Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4038857Z    |
2019-10-03T01:59:12.4038857Z    |
2019-10-03T01:59:12.4039144Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-10-03T01:59:12.4039284Z error: lifetime may not live long enough
2019-10-03T01:59:12.4039827Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:36:9
2019-10-03T01:59:12.4039921Z    |
2019-10-03T01:59:12.4039921Z    |
2019-10-03T01:59:12.4040229Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4040610Z    |                                         |
2019-10-03T01:59:12.4040900Z    |                                         lifetime `'_` defined here
2019-10-03T01:59:12.4041207Z    |                                         lifetime `'_` defined here
2019-10-03T01:59:12.4041292Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4041292Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4041642Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4041925Z error: aborting due to 12 previous errors
2019-10-03T01:59:12.4041984Z 
2019-10-03T01:59:12.4042310Z For more information about this error, try `rustc --explain E0700`.
2019-10-03T01:59:12.4042367Z 
2019-10-03T01:59:12.4042367Z 
2019-10-03T01:59:12.4042625Z ------------------------------------------
2019-10-03T01:59:12.4042674Z 
2019-10-03T01:59:12.4042710Z 
2019-10-03T01:59:12.4042989Z ---- [ui (nll)] ui/self/elision/ref-mut-self-async.rs stdout ----
2019-10-03T01:59:12.4043071Z diff of stderr:
2019-10-03T01:59:12.4043137Z 
2019-10-03T01:59:12.4043420Z 7    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.4043761Z 9 error: lifetime may not live long enough
2019-10-03T01:59:12.4044090Z -   --> $DIR/ref-mut-self-async.rs:13:51
2019-10-03T01:59:12.4044351Z +   --> $DIR/ref-mut-self-async.rs:14:9
2019-10-03T01:59:12.4044425Z 11    |
2019-10-03T01:59:12.4044425Z 11    |
2019-10-03T01:59:12.4044705Z - LL |       async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4045495Z -    |  _______________________-___________________________^
2019-10-03T01:59:12.4046113Z -    | |                       lifetime `'_` defined here
2019-10-03T01:59:12.4046403Z -    | |                       lifetime `'_` defined here
2019-10-03T01:59:12.4046631Z - LL | |         f
2019-10-03T01:59:12.4046863Z - LL | |     }
2019-10-03T01:59:12.4046863Z - LL | |     }
2019-10-03T01:59:12.4047203Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4047706Z + LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4048043Z +    |                       |
2019-10-03T01:59:12.4048324Z +    |                       lifetime `'_` defined here
2019-10-03T01:59:12.4048850Z +    |                       lifetime `'_` defined here
2019-10-03T01:59:12.4048958Z + LL |         f
2019-10-03T01:59:12.4048958Z + LL |         f
2019-10-03T01:59:12.4049320Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4049427Z 20 
2019-10-03T01:59:12.4049532Z 21 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4049876Z 
2019-10-03T01:59:12.4049876Z 
2019-10-03T01:59:12.4050160Z 27    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.4050335Z 29 error: lifetime may not live long enough
2019-10-03T01:59:12.4050604Z -   --> $DIR/ref-mut-self-async.rs:19:57
2019-10-03T01:59:12.4050848Z +   --> $DIR/ref-mut-self-async.rs:20:9
2019-10-03T01:59:12.4050936Z 31    |
2019-10-03T01:59:12.4050936Z 31    |
2019-10-03T01:59:12.4051219Z - LL |       async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4051507Z -    |  _____________________________-___________________________^
2019-10-03T01:59:12.4052052Z -    | |                             lifetime `'_` defined here
2019-10-03T01:59:12.4052349Z -    | |                             lifetime `'_` defined here
2019-10-03T01:59:12.4052580Z - LL | |         f
2019-10-03T01:59:12.4052994Z - LL | |     }
2019-10-03T01:59:12.4052994Z - LL | |     }
2019-10-03T01:59:12.4053353Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4053678Z + LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4054029Z +    |                             |
2019-10-03T01:59:12.4054313Z +    |                             lifetime `'_` defined here
2019-10-03T01:59:12.4054589Z +    |                             lifetime `'_` defined here
2019-10-03T01:59:12.4054683Z + LL |         f
2019-10-03T01:59:12.4054683Z + LL |         f
2019-10-03T01:59:12.4055368Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4055496Z 40 
2019-10-03T01:59:12.4055732Z 41 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4056412Z 
2019-10-03T01:59:12.4056412Z 
2019-10-03T01:59:12.4056730Z 47    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.4056905Z 49 error: lifetime may not live long enough
2019-10-03T01:59:12.4057198Z -   --> $DIR/ref-mut-self-async.rs:23:66
2019-10-03T01:59:12.4057481Z +   --> $DIR/ref-mut-self-async.rs:24:9
2019-10-03T01:59:12.4057575Z 51    |
2019-10-03T01:59:12.4057575Z 51    |
2019-10-03T01:59:12.4057876Z - LL |       async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4058214Z -    |  _____________________________________-____________________________^
2019-10-03T01:59:12.4058830Z -    | |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4059169Z -    | |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4059428Z - LL | |         f
2019-10-03T01:59:12.4059679Z - LL | |     }
2019-10-03T01:59:12.4059679Z - LL | |     }
2019-10-03T01:59:12.4060200Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4060598Z + LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4061119Z +    |                                     |
2019-10-03T01:59:12.4061476Z +    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4061792Z +    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4061894Z + LL |         f
2019-10-03T01:59:12.4061894Z + LL |         f
2019-10-03T01:59:12.4062244Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4062359Z 60 
2019-10-03T01:59:12.4062470Z 61 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4062823Z 
2019-10-03T01:59:12.4062823Z 
2019-10-03T01:59:12.4063141Z 67    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.4063316Z 69 error: lifetime may not live long enough
2019-10-03T01:59:12.4063605Z -   --> $DIR/ref-mut-self-async.rs:27:66
2019-10-03T01:59:12.4064046Z +   --> $DIR/ref-mut-self-async.rs:28:9
2019-10-03T01:59:12.4064172Z 71    |
2019-10-03T01:59:12.4064172Z 71    |
2019-10-03T01:59:12.4064521Z - LL |       async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4064862Z -    |  _____________________________________-____________________________^
2019-10-03T01:59:12.4065821Z -    | |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4066147Z -    | |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4066388Z - LL | |         f
2019-10-03T01:59:12.4066626Z - LL | |     }
2019-10-03T01:59:12.4066626Z - LL | |     }
2019-10-03T01:59:12.4066953Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4067287Z + LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4067651Z +    |                                     |
2019-10-03T01:59:12.4067961Z +    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4068252Z +    |                                     lifetime `'_` defined here
2019-10-03T01:59:12.4068352Z + LL |         f
2019-10-03T01:59:12.4068352Z + LL |         f
2019-10-03T01:59:12.4069111Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4069244Z 80 
2019-10-03T01:59:12.4069473Z 81 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4069873Z 
2019-10-03T01:59:12.4069873Z 
2019-10-03T01:59:12.4070178Z 87    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.4070346Z 89 error: lifetime may not live long enough
2019-10-03T01:59:12.4070616Z -   --> $DIR/ref-mut-self-async.rs:31:75
2019-10-03T01:59:12.4070866Z +   --> $DIR/ref-mut-self-async.rs:32:9
2019-10-03T01:59:12.4070966Z 91    |
2019-10-03T01:59:12.4070966Z 91    |
2019-10-03T01:59:12.4071258Z - LL |       async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4071586Z -    |  _____________________________________________-_____________________________^
2019-10-03T01:59:12.4072175Z -    | |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4072501Z -    | |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4072740Z - LL | |         f
2019-10-03T01:59:12.4072973Z - LL | |     }
2019-10-03T01:59:12.4072973Z - LL | |     }
2019-10-03T01:59:12.4073571Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4073930Z + LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4074308Z +    |                                             |
2019-10-03T01:59:12.4074791Z +    |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4075461Z +    |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4075584Z + LL |         f
2019-10-03T01:59:12.4075584Z + LL |         f
2019-10-03T01:59:12.4075924Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4076240Z 100 
2019-10-03T01:59:12.4076343Z 101 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4076743Z 
2019-10-03T01:59:12.4076743Z 
2019-10-03T01:59:12.4077046Z 107    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.4077214Z 109 error: lifetime may not live long enough
2019-10-03T01:59:12.4077484Z -   --> $DIR/ref-mut-self-async.rs:35:75
2019-10-03T01:59:12.4077746Z +   --> $DIR/ref-mut-self-async.rs:36:9
2019-10-03T01:59:12.4077836Z 111    |
2019-10-03T01:59:12.4077836Z 111    |
2019-10-03T01:59:12.4078125Z - LL |       async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4078452Z -    |  _____________________________________________-_____________________________^
2019-10-03T01:59:12.4079039Z -    | |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4079366Z -    | |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4079782Z - LL | |         f
2019-10-03T01:59:12.4080051Z - LL | |     }
2019-10-03T01:59:12.4080051Z - LL | |     }
2019-10-03T01:59:12.4080380Z -    | |_____^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4080718Z + LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-10-03T01:59:12.4081105Z +    |                                             |
2019-10-03T01:59:12.4081416Z +    |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4081716Z +    |                                             lifetime `'_` defined here
2019-10-03T01:59:12.4081815Z + LL |         f
2019-10-03T01:59:12.4081815Z + LL |         f
2019-10-03T01:59:12.4082142Z +    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4082470Z 121 error: aborting due to 12 previous errors
2019-10-03T01:59:12.4082557Z 122 
2019-10-03T01:59:12.4082594Z 
2019-10-03T01:59:12.4082646Z 
2019-10-03T01:59:12.4082646Z 
2019-10-03T01:59:12.4082714Z The actual stderr differed from the expected stderr.
2019-10-03T01:59:12.4083150Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/ref-mut-self-async.nll.stderr
2019-10-03T01:59:12.4083682Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T01:59:12.4084067Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`
2019-10-03T01:59:12.4084217Z error: 1 errors occurred comparing output.
2019-10-03T01:59:12.4084292Z status: exit code: 1
2019-10-03T01:59:12.4084292Z status: exit code: 1
2019-10-03T01:59:12.4085560Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/auxiliary" "-A" "unused"
2019-10-03T01:59:12.4086380Z ------------------------------------------
2019-10-03T01:59:12.4086435Z 
2019-10-03T01:59:12.4086703Z ------------------------------------------
2019-10-03T01:59:12.4086780Z stderr:
2019-10-03T01:59:12.4086780Z stderr:
2019-10-03T01:59:12.4087039Z ------------------------------------------
2019-10-03T01:59:12.4087135Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4087825Z    |
2019-10-03T01:59:12.4087825Z    |
2019-10-03T01:59:12.4088300Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4088523Z    |
2019-10-03T01:59:12.4088523Z    |
2019-10-03T01:59:12.4088837Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.4089022Z error: lifetime may not live long enough
2019-10-03T01:59:12.4089363Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:14:9
2019-10-03T01:59:12.4110847Z    |
2019-10-03T01:59:12.4110847Z    |
2019-10-03T01:59:12.4111400Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4111768Z    |                       |
2019-10-03T01:59:12.4112046Z    |                       lifetime `'_` defined here
2019-10-03T01:59:12.4112330Z    |                       lifetime `'_` defined here
2019-10-03T01:59:12.4112414Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4112414Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4112782Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4112862Z 
2019-10-03T01:59:12.4112959Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4113365Z    |
2019-10-03T01:59:12.4113365Z    |
2019-10-03T01:59:12.4113634Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4113826Z    |
2019-10-03T01:59:12.4113826Z    |
2019-10-03T01:59:12.4114120Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-10-03T01:59:12.4114255Z error: lifetime may not live long enough
2019-10-03T01:59:12.4114537Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:20:9
2019-10-03T01:59:12.4114632Z    |
2019-10-03T01:59:12.4114632Z    |
2019-10-03T01:59:12.4115354Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-10-03T01:59:12.4115859Z    |                             |
2019-10-03T01:59:12.4116138Z    |                             lifetime `'_` defined here
2019-10-03T01:59:12.4116429Z    |                             lifetime `'_` defined here
2019-10-03T01:59:12.4116512Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4116512Z LL |         f //~ ERROR lifetime mismatch
2019-10-03T01:59:12.4116859Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-10-03T01:59:12.4116944Z 
2019-10-03T01:59:12.4117042Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-10-03T01:59:12.4117442Z    |
2019-10-03T01:59:12.4117442Z    |
2019-10-03T01:59:12.4117732Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
---
2019-10-03T01:59:12.4327734Z test result: FAILED. 9022 passed; 6 failed; 67 ignored; 0 measured; 0 filtered out
2019-10-03T01:59:12.4327802Z 
2019-10-03T01:59:12.4327837Z 
2019-10-03T01:59:12.4327872Z 
2019-10-03T01:59:12.4329814Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-10-03T01:59:12.4330605Z 
2019-10-03T01:59:12.4330703Z 
2019-10-03T01:59:12.4331062Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-03T01:59:12.4331187Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T01:59:12.4331187Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T01:59:12.4331285Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-03T01:59:12.4331381Z Build completed unsuccessfully in 1:57:03
2019-10-03T01:59:12.4331454Z == clock drift check ==
2019-10-03T01:59:12.4331538Z   local time: Thu Oct  3 01:59:12 UTC 2019
2019-10-03T01:59:12.5473128Z   network time: Thu, 03 Oct 2019 01:59:12 GMT
2019-10-03T01:59:12.5473314Z == end clock drift check ==
2019-10-03T01:59:13.8556953Z ##[error]Bash exited with code '1'.
2019-10-03T01:59:13.8600539Z ##[section]Starting: Upload CPU usage statistics
2019-10-03T01:59:13.8616996Z ==============================================================================
2019-10-03T01:59:13.8617117Z Task         : Bash
2019-10-03T01:59:13.8617219Z Description  : Run a Bash script on macOS, Linux, or Windows
