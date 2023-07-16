plain
2019-11-01T03:08:41.3813386Z 
2019-11-01T03:08:41.3814108Z ---- [ui (nll)] ui/self/elision/lt-ref-self-async.rs stdout ----
2019-11-01T03:08:41.3814213Z diff of stderr:
2019-11-01T03:08:41.3814274Z 
2019-11-01T03:08:41.3814352Z 1 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3814641Z -   --> $DIR/lt-ref-self-async.rs:13:42
2019-11-01T03:08:41.3815155Z +   --> $DIR/lt-ref-self-async.rs:12:42
2019-11-01T03:08:41.3815255Z 3    |
2019-11-01T03:08:41.3815502Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3815652Z 
2019-11-01T03:08:41.3815652Z 
2019-11-01T03:08:41.3815929Z 7    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3816089Z 9 error: lifetime may not live long enough
2019-11-01T03:08:41.3816326Z -   --> $DIR/lt-ref-self-async.rs:14:9
2019-11-01T03:08:41.3816718Z +   --> $DIR/lt-ref-self-async.rs:13:9
2019-11-01T03:08:41.3816817Z 11    |
2019-11-01T03:08:41.3816817Z 11    |
2019-11-01T03:08:41.3817088Z 12 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3818625Z 
2019-11-01T03:08:41.3818625Z 
2019-11-01T03:08:41.3819045Z 18    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3819151Z 19 
2019-11-01T03:08:41.3819260Z 20 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3819534Z -   --> $DIR/lt-ref-self-async.rs:19:48
2019-11-01T03:08:41.3821991Z +   --> $DIR/lt-ref-self-async.rs:18:48
2019-11-01T03:08:41.3822106Z 22    |
2019-11-01T03:08:41.3822531Z 23 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3822688Z 
2019-11-01T03:08:41.3822688Z 
2019-11-01T03:08:41.3822963Z 26    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3823139Z 28 error: lifetime may not live long enough
2019-11-01T03:08:41.3823368Z -   --> $DIR/lt-ref-self-async.rs:20:9
2019-11-01T03:08:41.3823619Z +   --> $DIR/lt-ref-self-async.rs:19:9
2019-11-01T03:08:41.3823688Z 30    |
2019-11-01T03:08:41.3823688Z 30    |
2019-11-01T03:08:41.3823946Z 31 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3824237Z 
2019-11-01T03:08:41.3824237Z 
2019-11-01T03:08:41.3824536Z 37    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3824644Z 38 
2019-11-01T03:08:41.3824758Z 39 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3825000Z -   --> $DIR/lt-ref-self-async.rs:23:57
2019-11-01T03:08:41.3825237Z +   --> $DIR/lt-ref-self-async.rs:22:57
2019-11-01T03:08:41.3825306Z 41    |
2019-11-01T03:08:41.3825579Z 42 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3825749Z 
2019-11-01T03:08:41.3825749Z 
2019-11-01T03:08:41.3826190Z 45    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3826354Z 47 error: lifetime may not live long enough
2019-11-01T03:08:41.3826604Z -   --> $DIR/lt-ref-self-async.rs:24:9
2019-11-01T03:08:41.3826831Z +   --> $DIR/lt-ref-self-async.rs:23:9
2019-11-01T03:08:41.3826919Z 49    |
2019-11-01T03:08:41.3826919Z 49    |
2019-11-01T03:08:41.3827194Z 50 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3827674Z 
2019-11-01T03:08:41.3827674Z 
2019-11-01T03:08:41.3828424Z 56    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3828549Z 57 
2019-11-01T03:08:41.3828641Z 58 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3828922Z -   --> $DIR/lt-ref-self-async.rs:27:57
2019-11-01T03:08:41.3829163Z +   --> $DIR/lt-ref-self-async.rs:26:57
2019-11-01T03:08:41.3829262Z 60    |
2019-11-01T03:08:41.3829530Z 61 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3829703Z 
2019-11-01T03:08:41.3829703Z 
2019-11-01T03:08:41.3829996Z 64    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3830166Z 66 error: lifetime may not live long enough
2019-11-01T03:08:41.3830423Z -   --> $DIR/lt-ref-self-async.rs:28:9
2019-11-01T03:08:41.3830660Z +   --> $DIR/lt-ref-self-async.rs:27:9
2019-11-01T03:08:41.3830751Z 68    |
2019-11-01T03:08:41.3830751Z 68    |
2019-11-01T03:08:41.3831017Z 69 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3831576Z 
2019-11-01T03:08:41.3831576Z 
2019-11-01T03:08:41.3832228Z 75    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3832441Z 76 
2019-11-01T03:08:41.3832541Z 77 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3832828Z -   --> $DIR/lt-ref-self-async.rs:31:66
2019-11-01T03:08:41.3833087Z +   --> $DIR/lt-ref-self-async.rs:30:66
2019-11-01T03:08:41.3833161Z 79    |
2019-11-01T03:08:41.3833458Z 80 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3833637Z 
2019-11-01T03:08:41.3833637Z 
2019-11-01T03:08:41.3833909Z 83    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3834094Z 85 error: lifetime may not live long enough
2019-11-01T03:08:41.3834332Z -   --> $DIR/lt-ref-self-async.rs:32:9
2019-11-01T03:08:41.3834594Z +   --> $DIR/lt-ref-self-async.rs:31:9
2019-11-01T03:08:41.3834667Z 87    |
2019-11-01T03:08:41.3834667Z 87    |
2019-11-01T03:08:41.3835122Z 88 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3835644Z 
2019-11-01T03:08:41.3835644Z 
2019-11-01T03:08:41.3835964Z 94    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3836083Z 95 
2019-11-01T03:08:41.3836165Z 96 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3836442Z -   --> $DIR/lt-ref-self-async.rs:35:62
2019-11-01T03:08:41.3836696Z +   --> $DIR/lt-ref-self-async.rs:34:62
2019-11-01T03:08:41.3836769Z 98    |
2019-11-01T03:08:41.3837060Z 99 LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3837238Z 
2019-11-01T03:08:41.3837238Z 
2019-11-01T03:08:41.3837522Z 102    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3837911Z 104 error: lifetime may not live long enough
2019-11-01T03:08:41.3838217Z -   --> $DIR/lt-ref-self-async.rs:36:9
2019-11-01T03:08:41.3838455Z +   --> $DIR/lt-ref-self-async.rs:35:9
2019-11-01T03:08:41.3838545Z 106    |
2019-11-01T03:08:41.3838545Z 106    |
2019-11-01T03:08:41.3838818Z 107 LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3839156Z 
2019-11-01T03:08:41.3839210Z 
2019-11-01T03:08:41.3839280Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.3839280Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.3839670Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/lt-ref-self-async.nll.stderr
2019-11-01T03:08:41.3839972Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.3840305Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`
2019-11-01T03:08:41.3840455Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.3840536Z status: exit code: 1
2019-11-01T03:08:41.3840536Z status: exit code: 1
2019-11-01T03:08:41.3841530Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.3842077Z ------------------------------------------
2019-11-01T03:08:41.3842128Z 
2019-11-01T03:08:41.3842477Z ------------------------------------------
2019-11-01T03:08:41.3842562Z stderr:
2019-11-01T03:08:41.3842562Z stderr:
2019-11-01T03:08:41.3842836Z ------------------------------------------
2019-11-01T03:08:41.3843003Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3843425Z    |
2019-11-01T03:08:41.3843425Z    |
2019-11-01T03:08:41.3843689Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3843863Z    |
2019-11-01T03:08:41.3843863Z    |
2019-11-01T03:08:41.3844152Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3844299Z error: lifetime may not live long enough
2019-11-01T03:08:41.3844571Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:13:9
2019-11-01T03:08:41.3844669Z    |
2019-11-01T03:08:41.3844669Z    |
2019-11-01T03:08:41.3845075Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3845393Z    |                       |
2019-11-01T03:08:41.3845664Z    |                       lifetime `'_` defined here
2019-11-01T03:08:41.3845911Z    |                       lifetime `'_` defined here
2019-11-01T03:08:41.3846006Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3846006Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3846340Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3846413Z 
2019-11-01T03:08:41.3846509Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3846887Z    |
2019-11-01T03:08:41.3846887Z    |
2019-11-01T03:08:41.3847130Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3847305Z    |
2019-11-01T03:08:41.3847305Z    |
2019-11-01T03:08:41.3847589Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3848141Z error: lifetime may not live long enough
2019-11-01T03:08:41.3848473Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:19:9
2019-11-01T03:08:41.3848573Z    |
2019-11-01T03:08:41.3848573Z    |
2019-11-01T03:08:41.3848826Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3849173Z    |                             |
2019-11-01T03:08:41.3849432Z    |                             lifetime `'_` defined here
2019-11-01T03:08:41.3849714Z    |                             lifetime `'_` defined here
2019-11-01T03:08:41.3849797Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3849797Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3850140Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3850217Z 
2019-11-01T03:08:41.3850327Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3850726Z    |
2019-11-01T03:08:41.3850726Z    |
2019-11-01T03:08:41.3850989Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3851197Z    |
2019-11-01T03:08:41.3851197Z    |
2019-11-01T03:08:41.3851466Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3851769Z error: lifetime may not live long enough
2019-11-01T03:08:41.3852037Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:23:9
2019-11-01T03:08:41.3852111Z    |
2019-11-01T03:08:41.3852111Z    |
2019-11-01T03:08:41.3852370Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3852693Z    |                                     |
2019-11-01T03:08:41.3853036Z    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.3853347Z    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.3853508Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3853508Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3853831Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3853900Z 
2019-11-01T03:08:41.3853993Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3854356Z    |
2019-11-01T03:08:41.3854356Z    |
2019-11-01T03:08:41.3854614Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3854791Z    |
2019-11-01T03:08:41.3854791Z    |
2019-11-01T03:08:41.3855040Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3855182Z error: lifetime may not live long enough
2019-11-01T03:08:41.3855457Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:27:9
2019-11-01T03:08:41.3855541Z    |
2019-11-01T03:08:41.3855541Z    |
2019-11-01T03:08:41.3855802Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3856119Z    |                                     |
2019-11-01T03:08:41.3856385Z    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.3856643Z    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.3856739Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3856739Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3857042Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3857127Z 
2019-11-01T03:08:41.3857202Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3858165Z    |
2019-11-01T03:08:41.3858165Z    |
2019-11-01T03:08:41.3858589Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3858799Z    |
2019-11-01T03:08:41.3858799Z    |
2019-11-01T03:08:41.3859088Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3859232Z error: lifetime may not live long enough
2019-11-01T03:08:41.3859500Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:31:9
2019-11-01T03:08:41.3859600Z    |
2019-11-01T03:08:41.3859600Z    |
2019-11-01T03:08:41.3859868Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3860235Z    |                                             |
2019-11-01T03:08:41.3860715Z    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.3861033Z    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.3861131Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3861131Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3861477Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3861712Z 
2019-11-01T03:08:41.3861806Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3862167Z    |
2019-11-01T03:08:41.3862167Z    |
2019-11-01T03:08:41.3862414Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3862592Z    |
2019-11-01T03:08:41.3862592Z    |
2019-11-01T03:08:41.3862860Z    = note: hidden type `impl std::future::Future` captures lifetime '_#23r
2019-11-01T03:08:41.3863091Z error: lifetime may not live long enough
2019-11-01T03:08:41.3863377Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:35:9
2019-11-01T03:08:41.3863540Z    |
2019-11-01T03:08:41.3863540Z    |
2019-11-01T03:08:41.3863825Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3864196Z    |                                         |
2019-11-01T03:08:41.3864451Z    |                                         lifetime `'_` defined here
2019-11-01T03:08:41.3864733Z    |                                         lifetime `'_` defined here
2019-11-01T03:08:41.3864813Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3864813Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3865132Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3865278Z error: aborting due to 12 previous errors
2019-11-01T03:08:41.3865321Z 
2019-11-01T03:08:41.3865589Z For more information about this error, try `rustc --explain E0700`.
2019-11-01T03:08:41.3865644Z 
---
2019-11-01T03:08:41.3866411Z 1 error: lifetime may not live long enough
2019-11-01T03:08:41.3866829Z -   --> $DIR/lt-ref-self.rs:12:9
2019-11-01T03:08:41.3867054Z +   --> $DIR/lt-ref-self.rs:11:9
2019-11-01T03:08:41.3867141Z 3    |
2019-11-01T03:08:41.3867552Z 4 LL |     fn ref_self(&self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3868155Z 
2019-11-01T03:08:41.3868155Z 
2019-11-01T03:08:41.3868503Z 9    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3868709Z 11 error: lifetime may not live long enough
2019-11-01T03:08:41.3869204Z -   --> $DIR/lt-ref-self.rs:18:9
2019-11-01T03:08:41.3869493Z +   --> $DIR/lt-ref-self.rs:17:9
2019-11-01T03:08:41.3869596Z 13    |
2019-11-01T03:08:41.3869596Z 13    |
2019-11-01T03:08:41.3869850Z 14 LL |     fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3870447Z 
2019-11-01T03:08:41.3870447Z 
2019-11-01T03:08:41.3870798Z 19    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3870982Z 21 error: lifetime may not live long enough
2019-11-01T03:08:41.3871214Z -   --> $DIR/lt-ref-self.rs:22:9
2019-11-01T03:08:41.3871682Z +   --> $DIR/lt-ref-self.rs:21:9
2019-11-01T03:08:41.3871768Z 23    |
2019-11-01T03:08:41.3871768Z 23    |
2019-11-01T03:08:41.3872179Z 24 LL |     fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3872768Z 
2019-11-01T03:08:41.3872768Z 
2019-11-01T03:08:41.3875584Z 29    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3875791Z 31 error: lifetime may not live long enough
2019-11-01T03:08:41.3876065Z -   --> $DIR/lt-ref-self.rs:26:9
2019-11-01T03:08:41.3876303Z +   --> $DIR/lt-ref-self.rs:25:9
2019-11-01T03:08:41.3876370Z 33    |
2019-11-01T03:08:41.3876370Z 33    |
2019-11-01T03:08:41.3876632Z 34 LL |     fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3877009Z 
2019-11-01T03:08:41.3877009Z 
2019-11-01T03:08:41.3877493Z 39    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3878259Z 41 error: lifetime may not live long enough
2019-11-01T03:08:41.3879364Z -   --> $DIR/lt-ref-self.rs:30:9
2019-11-01T03:08:41.3879806Z +   --> $DIR/lt-ref-self.rs:29:9
2019-11-01T03:08:41.3879881Z 43    |
2019-11-01T03:08:41.3879881Z 43    |
2019-11-01T03:08:41.3880312Z 44 LL |     fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3880740Z 
2019-11-01T03:08:41.3880740Z 
2019-11-01T03:08:41.3881062Z 49    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3881241Z 51 error: lifetime may not live long enough
2019-11-01T03:08:41.3881490Z -   --> $DIR/lt-ref-self.rs:34:9
2019-11-01T03:08:41.3881922Z +   --> $DIR/lt-ref-self.rs:33:9
2019-11-01T03:08:41.3882196Z 53    |
2019-11-01T03:08:41.3882196Z 53    |
2019-11-01T03:08:41.3882477Z 54 LL |     fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3883176Z 
2019-11-01T03:08:41.3883212Z 
2019-11-01T03:08:41.3883286Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.3883286Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.3886784Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self.nll/lt-ref-self.nll.stderr
2019-11-01T03:08:41.3887183Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.3887493Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self.rs`
2019-11-01T03:08:41.3888235Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.3888329Z status: exit code: 1
2019-11-01T03:08:41.3888329Z status: exit code: 1
2019-11-01T03:08:41.3901611Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.3902388Z ------------------------------------------
2019-11-01T03:08:41.3902447Z 
2019-11-01T03:08:41.3902712Z ------------------------------------------
2019-11-01T03:08:41.3902789Z stderr:
2019-11-01T03:08:41.3902789Z stderr:
2019-11-01T03:08:41.3903037Z ------------------------------------------
2019-11-01T03:08:41.3903269Z error: lifetime may not live long enough
2019-11-01T03:08:41.3903716Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:11:9
2019-11-01T03:08:41.3903823Z    |
2019-11-01T03:08:41.3904074Z LL |     fn ref_self(&self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3904628Z    |                 |
2019-11-01T03:08:41.3904911Z    |                 let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.3904995Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3904995Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3905340Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3905497Z error: lifetime may not live long enough
2019-11-01T03:08:41.3905768Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:17:9
2019-11-01T03:08:41.3905847Z    |
2019-11-01T03:08:41.3905847Z    |
2019-11-01T03:08:41.3906098Z LL |     fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3906490Z    |                       |
2019-11-01T03:08:41.3906755Z    |                       let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.3906858Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3906858Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3907342Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3907552Z error: lifetime may not live long enough
2019-11-01T03:08:41.3908618Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:21:9
2019-11-01T03:08:41.3908728Z    |
2019-11-01T03:08:41.3908728Z    |
2019-11-01T03:08:41.3909239Z LL |     fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3910565Z    |                               |
2019-11-01T03:08:41.3910883Z    |                               let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.3910973Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3910973Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3911318Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3911490Z error: lifetime may not live long enough
2019-11-01T03:08:41.3911761Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:25:9
2019-11-01T03:08:41.3911859Z    |
2019-11-01T03:08:41.3911859Z    |
2019-11-01T03:08:41.3912363Z LL |     fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3912770Z    |                               |
2019-11-01T03:08:41.3913043Z    |                               let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.3913146Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3913146Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3913461Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3913615Z error: lifetime may not live long enough
2019-11-01T03:08:41.3913889Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:29:9
2019-11-01T03:08:41.3913966Z    |
2019-11-01T03:08:41.3913966Z    |
2019-11-01T03:08:41.3914246Z LL |     fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3914730Z    |                                       |
2019-11-01T03:08:41.3915037Z    |                                       let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.3915126Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3915126Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3915458Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3915607Z error: lifetime may not live long enough
2019-11-01T03:08:41.3915860Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:33:9
2019-11-01T03:08:41.3915951Z    |
2019-11-01T03:08:41.3915951Z    |
2019-11-01T03:08:41.3916201Z LL |     fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3916642Z    |                                   |
2019-11-01T03:08:41.3916923Z    |                                   let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.3917036Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3917036Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3917536Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3917688Z error: aborting due to 6 previous errors
2019-11-01T03:08:41.3918284Z 
2019-11-01T03:08:41.3918326Z 
2019-11-01T03:08:41.3918613Z ------------------------------------------
2019-11-01T03:08:41.3918613Z ------------------------------------------
2019-11-01T03:08:41.3918683Z 
2019-11-01T03:08:41.3918719Z 
2019-11-01T03:08:41.3918994Z ---- [ui (nll)] ui/self/elision/ref-mut-self-async.rs stdout ----
2019-11-01T03:08:41.3919078Z diff of stderr:
2019-11-01T03:08:41.3919120Z 
2019-11-01T03:08:41.3919221Z 1 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3919917Z +   --> $DIR/ref-mut-self-async.rs:12:46
2019-11-01T03:08:41.3920009Z 3    |
2019-11-01T03:08:41.3920009Z 3    |
2019-11-01T03:08:41.3920361Z 4 LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3920520Z 
2019-11-01T03:08:41.3920520Z 
2019-11-01T03:08:41.3920811Z 7    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3920985Z 9 error: lifetime may not live long enough
2019-11-01T03:08:41.3921224Z -   --> $DIR/ref-mut-self-async.rs:14:9
2019-11-01T03:08:41.3921480Z +   --> $DIR/ref-mut-self-async.rs:13:9
2019-11-01T03:08:41.3921553Z 11    |
2019-11-01T03:08:41.3921553Z 11    |
2019-11-01T03:08:41.3921823Z 12 LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3922128Z 
2019-11-01T03:08:41.3922128Z 
2019-11-01T03:08:41.3922457Z 18    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3922574Z 19 
2019-11-01T03:08:41.3922676Z 20 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3923203Z +   --> $DIR/ref-mut-self-async.rs:18:52
2019-11-01T03:08:41.3923277Z 22    |
2019-11-01T03:08:41.3923277Z 22    |
2019-11-01T03:08:41.3923555Z 23 LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3923723Z 
2019-11-01T03:08:41.3923723Z 
2019-11-01T03:08:41.3923996Z 26    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3924165Z 28 error: lifetime may not live long enough
2019-11-01T03:08:41.3924421Z -   --> $DIR/ref-mut-self-async.rs:20:9
2019-11-01T03:08:41.3924676Z +   --> $DIR/ref-mut-self-async.rs:19:9
2019-11-01T03:08:41.3924748Z 30    |
2019-11-01T03:08:41.3924748Z 30    |
2019-11-01T03:08:41.3925034Z 31 LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3925348Z 
2019-11-01T03:08:41.3925348Z 
2019-11-01T03:08:41.3925677Z 37    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3925793Z 38 
2019-11-01T03:08:41.3925876Z 39 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3926394Z +   --> $DIR/ref-mut-self-async.rs:22:61
2019-11-01T03:08:41.3926484Z 41    |
2019-11-01T03:08:41.3926484Z 41    |
2019-11-01T03:08:41.3926910Z 42 LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3927084Z 
2019-11-01T03:08:41.3927084Z 
2019-11-01T03:08:41.3927363Z 45    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3927535Z 47 error: lifetime may not live long enough
2019-11-01T03:08:41.3928221Z -   --> $DIR/ref-mut-self-async.rs:24:9
2019-11-01T03:08:41.3928482Z +   --> $DIR/ref-mut-self-async.rs:23:9
2019-11-01T03:08:41.3928571Z 49    |
2019-11-01T03:08:41.3928571Z 49    |
2019-11-01T03:08:41.3928841Z 50 LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3929171Z 
2019-11-01T03:08:41.3929171Z 
2019-11-01T03:08:41.3929508Z 56    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3929607Z 57 
2019-11-01T03:08:41.3929708Z 58 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3930223Z +   --> $DIR/ref-mut-self-async.rs:26:61
2019-11-01T03:08:41.3930313Z 60    |
2019-11-01T03:08:41.3930313Z 60    |
2019-11-01T03:08:41.3930582Z 61 LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3935143Z 
2019-11-01T03:08:41.3935143Z 
2019-11-01T03:08:41.3935936Z 64    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3936108Z 66 error: lifetime may not live long enough
2019-11-01T03:08:41.3936347Z -   --> $DIR/ref-mut-self-async.rs:28:9
2019-11-01T03:08:41.3936594Z +   --> $DIR/ref-mut-self-async.rs:27:9
2019-11-01T03:08:41.3936664Z 68    |
2019-11-01T03:08:41.3936664Z 68    |
2019-11-01T03:08:41.3936945Z 69 LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3937263Z 
2019-11-01T03:08:41.3937263Z 
2019-11-01T03:08:41.3938003Z 75    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3938136Z 76 
2019-11-01T03:08:41.3938236Z 77 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3938829Z +   --> $DIR/ref-mut-self-async.rs:30:70
2019-11-01T03:08:41.3938913Z 79    |
2019-11-01T03:08:41.3938913Z 79    |
2019-11-01T03:08:41.3939213Z 80 LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3939398Z 
2019-11-01T03:08:41.3939398Z 
2019-11-01T03:08:41.3939671Z 83    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3939843Z 85 error: lifetime may not live long enough
2019-11-01T03:08:41.3940101Z -   --> $DIR/ref-mut-self-async.rs:32:9
2019-11-01T03:08:41.3940339Z +   --> $DIR/ref-mut-self-async.rs:31:9
2019-11-01T03:08:41.3940599Z 87    |
2019-11-01T03:08:41.3940599Z 87    |
2019-11-01T03:08:41.3940938Z 88 LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3941276Z 
2019-11-01T03:08:41.3941276Z 
2019-11-01T03:08:41.3941614Z 94    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3941739Z 95 
2019-11-01T03:08:41.3941824Z 96 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3942346Z +   --> $DIR/ref-mut-self-async.rs:34:70
2019-11-01T03:08:41.3942440Z 98    |
2019-11-01T03:08:41.3942440Z 98    |
2019-11-01T03:08:41.3942718Z 99 LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3942897Z 
2019-11-01T03:08:41.3942897Z 
2019-11-01T03:08:41.3943189Z 102    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3943360Z 104 error: lifetime may not live long enough
2019-11-01T03:08:41.3943625Z -   --> $DIR/ref-mut-self-async.rs:36:9
2019-11-01T03:08:41.3943867Z +   --> $DIR/ref-mut-self-async.rs:35:9
2019-11-01T03:08:41.3943958Z 106    |
2019-11-01T03:08:41.3943958Z 106    |
2019-11-01T03:08:41.3944251Z 107 LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3944598Z 
2019-11-01T03:08:41.3944635Z 
2019-11-01T03:08:41.3944723Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.3944723Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.3945113Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/ref-mut-self-async.nll.stderr
2019-11-01T03:08:41.3946203Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.3946555Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`
2019-11-01T03:08:41.3947204Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.3947587Z status: exit code: 1
2019-11-01T03:08:41.3947587Z status: exit code: 1
2019-11-01T03:08:41.3948857Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.3949518Z ------------------------------------------
2019-11-01T03:08:41.3949592Z 
2019-11-01T03:08:41.3956199Z ------------------------------------------
2019-11-01T03:08:41.3956720Z stderr:
2019-11-01T03:08:41.3956720Z stderr:
2019-11-01T03:08:41.3957025Z ------------------------------------------
2019-11-01T03:08:41.3957156Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3957571Z    |
2019-11-01T03:08:41.3957571Z    |
2019-11-01T03:08:41.3958560Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3958751Z    |
2019-11-01T03:08:41.3958751Z    |
2019-11-01T03:08:41.3959027Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3959177Z error: lifetime may not live long enough
2019-11-01T03:08:41.3959466Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:13:9
2019-11-01T03:08:41.3959549Z    |
2019-11-01T03:08:41.3959549Z    |
2019-11-01T03:08:41.3959815Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3960254Z    |                       |
2019-11-01T03:08:41.3960551Z    |                       lifetime `'_` defined here
2019-11-01T03:08:41.3960820Z    |                       lifetime `'_` defined here
2019-11-01T03:08:41.3960921Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3960921Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3961431Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3961533Z 
2019-11-01T03:08:41.3961612Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3961993Z    |
2019-11-01T03:08:41.3961993Z    |
2019-11-01T03:08:41.3962256Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3962437Z    |
2019-11-01T03:08:41.3962437Z    |
2019-11-01T03:08:41.3962711Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3962850Z error: lifetime may not live long enough
2019-11-01T03:08:41.3963121Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:19:9
2019-11-01T03:08:41.3963217Z    |
2019-11-01T03:08:41.3963217Z    |
2019-11-01T03:08:41.3963465Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3963797Z    |                             |
2019-11-01T03:08:41.3964063Z    |                             lifetime `'_` defined here
2019-11-01T03:08:41.3964318Z    |                             lifetime `'_` defined here
2019-11-01T03:08:41.3964417Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3964417Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3965126Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3965217Z 
2019-11-01T03:08:41.3965317Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3966466Z    |
2019-11-01T03:08:41.3966466Z    |
2019-11-01T03:08:41.3968438Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3968811Z    |
2019-11-01T03:08:41.3968811Z    |
2019-11-01T03:08:41.3969233Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3969383Z error: lifetime may not live long enough
2019-11-01T03:08:41.3969658Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:23:9
2019-11-01T03:08:41.3969758Z    |
2019-11-01T03:08:41.3969758Z    |
2019-11-01T03:08:41.3970022Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3970390Z    |                                     |
2019-11-01T03:08:41.3970659Z    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.3970952Z    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.3971204Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3971204Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3971529Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3971606Z 
2019-11-01T03:08:41.3971700Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3972067Z    |
2019-11-01T03:08:41.3972067Z    |
2019-11-01T03:08:41.3972330Z LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3972511Z    |
2019-11-01T03:08:41.3972511Z    |
2019-11-01T03:08:41.3972763Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3972898Z error: lifetime may not live long enough
2019-11-01T03:08:41.3973169Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:27:9
2019-11-01T03:08:41.3973245Z    |
2019-11-01T03:08:41.3973245Z    |
2019-11-01T03:08:41.3973517Z LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3973851Z    |                                     |
2019-11-01T03:08:41.3974101Z    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.3974380Z    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.3974475Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3974475Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3974779Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3974862Z 
2019-11-01T03:08:41.3974939Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3975304Z    |
2019-11-01T03:08:41.3975304Z    |
2019-11-01T03:08:41.3975666Z LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3975862Z    |
2019-11-01T03:08:41.3975862Z    |
2019-11-01T03:08:41.3976132Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3976248Z error: lifetime may not live long enough
2019-11-01T03:08:41.3976518Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:31:9
2019-11-01T03:08:41.3976609Z    |
2019-11-01T03:08:41.3976609Z    |
2019-11-01T03:08:41.3976867Z LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3977209Z    |                                             |
2019-11-01T03:08:41.3977485Z    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.3978527Z    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.3978743Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3978743Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3979148Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3979295Z 
2019-11-01T03:08:41.3979378Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.3979815Z    |
2019-11-01T03:08:41.3979815Z    |
2019-11-01T03:08:41.3980089Z LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3980290Z    |
2019-11-01T03:08:41.3980290Z    |
2019-11-01T03:08:41.3980757Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.3980906Z error: lifetime may not live long enough
2019-11-01T03:08:41.3981183Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:35:9
2019-11-01T03:08:41.3981292Z    |
2019-11-01T03:08:41.3981292Z    |
2019-11-01T03:08:41.3981712Z LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3982080Z    |                                             |
2019-11-01T03:08:41.3982340Z    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.3982629Z    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.3982709Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3982709Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.3983029Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.3983176Z error: aborting due to 12 previous errors
2019-11-01T03:08:41.3983219Z 
2019-11-01T03:08:41.3983480Z For more information about this error, try `rustc --explain E0700`.
2019-11-01T03:08:41.3983534Z 
---
2019-11-01T03:08:41.3984303Z 1 error: lifetime may not live long enough
2019-11-01T03:08:41.3984541Z -   --> $DIR/ref-mut-self.rs:12:9
2019-11-01T03:08:41.3984761Z +   --> $DIR/ref-mut-self.rs:11:9
2019-11-01T03:08:41.3984844Z 3    |
2019-11-01T03:08:41.3985075Z 4 LL |     fn ref_self(&mut self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3985479Z 
2019-11-01T03:08:41.3985479Z 
2019-11-01T03:08:41.3985797Z 9    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3985978Z 11 error: lifetime may not live long enough
2019-11-01T03:08:41.3986216Z -   --> $DIR/ref-mut-self.rs:18:9
2019-11-01T03:08:41.3986440Z +   --> $DIR/ref-mut-self.rs:17:9
2019-11-01T03:08:41.3986527Z 13    |
2019-11-01T03:08:41.3986527Z 13    |
2019-11-01T03:08:41.3986763Z 14 LL |     fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.3987133Z 
2019-11-01T03:08:41.3987133Z 
2019-11-01T03:08:41.3987622Z 19    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3988221Z 21 error: lifetime may not live long enough
2019-11-01T03:08:41.3988505Z -   --> $DIR/ref-mut-self.rs:22:9
2019-11-01T03:08:41.3988760Z +   --> $DIR/ref-mut-self.rs:21:9
2019-11-01T03:08:41.3988832Z 23    |
2019-11-01T03:08:41.3988832Z 23    |
2019-11-01T03:08:41.3989111Z 24 LL |     fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3989517Z 
2019-11-01T03:08:41.3989517Z 
2019-11-01T03:08:41.3989948Z 29    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3990213Z 31 error: lifetime may not live long enough
2019-11-01T03:08:41.3990475Z -   --> $DIR/ref-mut-self.rs:26:9
2019-11-01T03:08:41.3990726Z +   --> $DIR/ref-mut-self.rs:25:9
2019-11-01T03:08:41.3990796Z 33    |
2019-11-01T03:08:41.3990796Z 33    |
2019-11-01T03:08:41.3991081Z 34 LL |     fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3992148Z 
2019-11-01T03:08:41.3992148Z 
2019-11-01T03:08:41.3992563Z 39    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3992750Z 41 error: lifetime may not live long enough
2019-11-01T03:08:41.3993005Z -   --> $DIR/ref-mut-self.rs:30:9
2019-11-01T03:08:41.3993265Z +   --> $DIR/ref-mut-self.rs:29:9
2019-11-01T03:08:41.3993338Z 43    |
2019-11-01T03:08:41.3993338Z 43    |
2019-11-01T03:08:41.3994884Z 44 LL |     fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3995471Z 
2019-11-01T03:08:41.3995471Z 
2019-11-01T03:08:41.3995760Z 49    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.3995925Z 51 error: lifetime may not live long enough
2019-11-01T03:08:41.3996152Z -   --> $DIR/ref-mut-self.rs:34:9
2019-11-01T03:08:41.3996359Z +   --> $DIR/ref-mut-self.rs:33:9
2019-11-01T03:08:41.3996443Z 53    |
2019-11-01T03:08:41.3996443Z 53    |
2019-11-01T03:08:41.3996708Z 54 LL |     fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.3997075Z 
2019-11-01T03:08:41.3997132Z 
2019-11-01T03:08:41.3997195Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.3997195Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.3997538Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self.nll/ref-mut-self.nll.stderr
2019-11-01T03:08:41.3998248Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.3998635Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self.rs`
2019-11-01T03:08:41.3998781Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.3998871Z status: exit code: 1
2019-11-01T03:08:41.3998871Z status: exit code: 1
2019-11-01T03:08:41.3999806Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.4000352Z ------------------------------------------
2019-11-01T03:08:41.4000405Z 
2019-11-01T03:08:41.4000656Z ------------------------------------------
2019-11-01T03:08:41.4000730Z stderr:
2019-11-01T03:08:41.4000730Z stderr:
2019-11-01T03:08:41.4000973Z ------------------------------------------
2019-11-01T03:08:41.4001050Z error: lifetime may not live long enough
2019-11-01T03:08:41.4001335Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:11:9
2019-11-01T03:08:41.4001576Z    |
2019-11-01T03:08:41.4001990Z LL |     fn ref_self(&mut self, f: &u32) -> &u32 {
2019-11-01T03:08:41.4002353Z    |                 |
2019-11-01T03:08:41.4002734Z    |                 let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4002825Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4002825Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4003223Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4003790Z error: lifetime may not live long enough
2019-11-01T03:08:41.4004111Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:17:9
2019-11-01T03:08:41.4004199Z    |
2019-11-01T03:08:41.4004199Z    |
2019-11-01T03:08:41.4004448Z LL |     fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.4008078Z    |                       |
2019-11-01T03:08:41.4008532Z    |                       let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4008623Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4008623Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4008991Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4009152Z error: lifetime may not live long enough
2019-11-01T03:08:41.4009436Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:21:9
2019-11-01T03:08:41.4009534Z    |
2019-11-01T03:08:41.4009534Z    |
2019-11-01T03:08:41.4009794Z LL |     fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4010241Z    |                               |
2019-11-01T03:08:41.4010527Z    |                               let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4010635Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4010635Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4010962Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4011118Z error: lifetime may not live long enough
2019-11-01T03:08:41.4011597Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:25:9
2019-11-01T03:08:41.4011679Z    |
2019-11-01T03:08:41.4011679Z    |
2019-11-01T03:08:41.4012334Z LL |     fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4012881Z    |                               |
2019-11-01T03:08:41.4013147Z    |                               let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4013226Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4013226Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4013526Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4013662Z error: lifetime may not live long enough
2019-11-01T03:08:41.4013894Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:29:9
2019-11-01T03:08:41.4013980Z    |
2019-11-01T03:08:41.4013980Z    |
2019-11-01T03:08:41.4014220Z LL |     fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4014637Z    |                                       |
2019-11-01T03:08:41.4014897Z    |                                       let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4014993Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4014993Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4015276Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4015413Z error: lifetime may not live long enough
2019-11-01T03:08:41.4015661Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:33:9
2019-11-01T03:08:41.4015730Z    |
2019-11-01T03:08:41.4015730Z    |
2019-11-01T03:08:41.4015981Z LL |     fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4016518Z    |                                       |
2019-11-01T03:08:41.4016828Z    |                                       let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4016983Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4016983Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4017307Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4017619Z error: aborting due to 6 previous errors
2019-11-01T03:08:41.4017661Z 
2019-11-01T03:08:41.4017867Z 
2019-11-01T03:08:41.4018486Z ------------------------------------------
2019-11-01T03:08:41.4018486Z ------------------------------------------
2019-11-01T03:08:41.4018537Z 
2019-11-01T03:08:41.4018573Z 
2019-11-01T03:08:41.4018853Z ---- [ui (nll)] ui/self/elision/ref-mut-struct-async.rs stdout ----
2019-11-01T03:08:41.4018954Z diff of stderr:
2019-11-01T03:08:41.4018996Z 
2019-11-01T03:08:41.4019080Z 1 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4019634Z +   --> $DIR/ref-mut-struct-async.rs:12:56
2019-11-01T03:08:41.4019718Z 3    |
2019-11-01T03:08:41.4019718Z 3    |
2019-11-01T03:08:41.4020002Z 4 LL |     async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4020171Z 
2019-11-01T03:08:41.4020171Z 
2019-11-01T03:08:41.4020683Z 7    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4020861Z 9 error: lifetime may not live long enough
2019-11-01T03:08:41.4021152Z -   --> $DIR/ref-mut-struct-async.rs:14:9
2019-11-01T03:08:41.4021394Z +   --> $DIR/ref-mut-struct-async.rs:13:9
2019-11-01T03:08:41.4021486Z 11    |
2019-11-01T03:08:41.4021486Z 11    |
2019-11-01T03:08:41.4021912Z 12 LL |     async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4022522Z 
2019-11-01T03:08:41.4022522Z 
2019-11-01T03:08:41.4022839Z 18    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4022928Z 19 
2019-11-01T03:08:41.4023026Z 20 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4023506Z +   --> $DIR/ref-mut-struct-async.rs:16:65
2019-11-01T03:08:41.4023753Z 22    |
2019-11-01T03:08:41.4023753Z 22    |
2019-11-01T03:08:41.4023994Z 23 LL |     async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4024152Z 
2019-11-01T03:08:41.4024152Z 
2019-11-01T03:08:41.4024408Z 26    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4024557Z 28 error: lifetime may not live long enough
2019-11-01T03:08:41.4024769Z -   --> $DIR/ref-mut-struct-async.rs:18:9
2019-11-01T03:08:41.4025002Z +   --> $DIR/ref-mut-struct-async.rs:17:9
2019-11-01T03:08:41.4025082Z 30    |
2019-11-01T03:08:41.4025082Z 30    |
2019-11-01T03:08:41.4025323Z 31 LL |     async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4026233Z 
2019-11-01T03:08:41.4026233Z 
2019-11-01T03:08:41.4026769Z 37    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4026862Z 38 
2019-11-01T03:08:41.4026955Z 39 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4030916Z +   --> $DIR/ref-mut-struct-async.rs:20:65
2019-11-01T03:08:41.4030991Z 41    |
2019-11-01T03:08:41.4030991Z 41    |
2019-11-01T03:08:41.4031287Z 42 LL |     async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4031470Z 
2019-11-01T03:08:41.4031470Z 
2019-11-01T03:08:41.4032068Z 45    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4032272Z 47 error: lifetime may not live long enough
2019-11-01T03:08:41.4032831Z -   --> $DIR/ref-mut-struct-async.rs:22:9
2019-11-01T03:08:41.4033405Z +   --> $DIR/ref-mut-struct-async.rs:21:9
2019-11-01T03:08:41.4033470Z 49    |
2019-11-01T03:08:41.4033470Z 49    |
2019-11-01T03:08:41.4033727Z 50 LL |     async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4034017Z 
2019-11-01T03:08:41.4034017Z 
2019-11-01T03:08:41.4034296Z 56    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4034397Z 57 
2019-11-01T03:08:41.4034468Z 58 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4034931Z +   --> $DIR/ref-mut-struct-async.rs:24:74
2019-11-01T03:08:41.4035014Z 60    |
2019-11-01T03:08:41.4035014Z 60    |
2019-11-01T03:08:41.4035280Z 61 LL |     async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4035453Z 
2019-11-01T03:08:41.4035453Z 
2019-11-01T03:08:41.4035691Z 64    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4035839Z 66 error: lifetime may not live long enough
2019-11-01T03:08:41.4036067Z -   --> $DIR/ref-mut-struct-async.rs:26:9
2019-11-01T03:08:41.4036463Z +   --> $DIR/ref-mut-struct-async.rs:25:9
2019-11-01T03:08:41.4036546Z 68    |
2019-11-01T03:08:41.4036546Z 68    |
2019-11-01T03:08:41.4036800Z 69 LL |     async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4037630Z 
2019-11-01T03:08:41.4037630Z 
2019-11-01T03:08:41.4039270Z 75    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4039384Z 76 
2019-11-01T03:08:41.4039488Z 77 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4040992Z +   --> $DIR/ref-mut-struct-async.rs:28:74
2019-11-01T03:08:41.4041094Z 79    |
2019-11-01T03:08:41.4041094Z 79    |
2019-11-01T03:08:41.4041385Z 80 LL |     async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4041569Z 
2019-11-01T03:08:41.4041569Z 
2019-11-01T03:08:41.4041923Z 83    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4042092Z 85 error: lifetime may not live long enough
2019-11-01T03:08:41.4042334Z -   --> $DIR/ref-mut-struct-async.rs:30:9
2019-11-01T03:08:41.4042591Z +   --> $DIR/ref-mut-struct-async.rs:29:9
2019-11-01T03:08:41.4042692Z 87    |
2019-11-01T03:08:41.4042692Z 87    |
2019-11-01T03:08:41.4042980Z 88 LL |     async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4043337Z 
2019-11-01T03:08:41.4043374Z 
2019-11-01T03:08:41.4043463Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4043463Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4043998Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async.nll/ref-mut-struct-async.nll.stderr
2019-11-01T03:08:41.4044472Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.4044947Z To only update this specific test, also pass `--test-args self/elision/ref-mut-struct-async.rs`
2019-11-01T03:08:41.4045085Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.4045150Z status: exit code: 1
2019-11-01T03:08:41.4045150Z status: exit code: 1
2019-11-01T03:08:41.4046177Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-struct-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.4046771Z ------------------------------------------
2019-11-01T03:08:41.4046836Z 
2019-11-01T03:08:41.4047047Z ------------------------------------------
2019-11-01T03:08:41.4047131Z stderr:
2019-11-01T03:08:41.4047131Z stderr:
2019-11-01T03:08:41.4047338Z ------------------------------------------
2019-11-01T03:08:41.4047441Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4048234Z    |
2019-11-01T03:08:41.4048234Z    |
2019-11-01T03:08:41.4048543Z LL |     async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4048746Z    |
2019-11-01T03:08:41.4048746Z    |
2019-11-01T03:08:41.4049020Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4049163Z error: lifetime may not live long enough
2019-11-01T03:08:41.4049458Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:13:9
2019-11-01T03:08:41.4049540Z    |
2019-11-01T03:08:41.4049540Z    |
2019-11-01T03:08:41.4049814Z LL |     async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4050151Z    |                               |
2019-11-01T03:08:41.4050431Z    |                               lifetime `'_` defined here
2019-11-01T03:08:41.4050709Z    |                               lifetime `'_` defined here
2019-11-01T03:08:41.4050812Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4050812Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4051150Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4051239Z 
2019-11-01T03:08:41.4051322Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4051865Z    |
2019-11-01T03:08:41.4051865Z    |
2019-11-01T03:08:41.4052123Z LL |     async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4052300Z    |
2019-11-01T03:08:41.4052300Z    |
2019-11-01T03:08:41.4052560Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4052689Z error: lifetime may not live long enough
2019-11-01T03:08:41.4052944Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:17:9
2019-11-01T03:08:41.4053034Z    |
2019-11-01T03:08:41.4053034Z    |
2019-11-01T03:08:41.4053276Z LL |     async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4053604Z    |                                       |
2019-11-01T03:08:41.4053863Z    |                                       lifetime `'_` defined here
2019-11-01T03:08:41.4054128Z    |                                       lifetime `'_` defined here
2019-11-01T03:08:41.4054206Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4054206Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4054515Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4054582Z 
2019-11-01T03:08:41.4054670Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4055110Z    |
2019-11-01T03:08:41.4055110Z    |
2019-11-01T03:08:41.4055384Z LL |     async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4055627Z    |
2019-11-01T03:08:41.4055627Z    |
2019-11-01T03:08:41.4055907Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4056036Z error: lifetime may not live long enough
2019-11-01T03:08:41.4056282Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:21:9
2019-11-01T03:08:41.4056372Z    |
2019-11-01T03:08:41.4056372Z    |
2019-11-01T03:08:41.4056630Z LL |     async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4056947Z    |                                       |
2019-11-01T03:08:41.4057190Z    |                                       lifetime `'_` defined here
2019-11-01T03:08:41.4057469Z    |                                       lifetime `'_` defined here
2019-11-01T03:08:41.4057547Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4057547Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4058293Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4058380Z 
2019-11-01T03:08:41.4058483Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4058882Z    |
2019-11-01T03:08:41.4058882Z    |
2019-11-01T03:08:41.4059180Z LL |     async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4059384Z    |
2019-11-01T03:08:41.4059384Z    |
2019-11-01T03:08:41.4059651Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4059793Z error: lifetime may not live long enough
2019-11-01T03:08:41.4060093Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:25:9
2019-11-01T03:08:41.4060174Z    |
2019-11-01T03:08:41.4060174Z    |
2019-11-01T03:08:41.4060716Z LL |     async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4061158Z    |                                               |
2019-11-01T03:08:41.4061813Z    |                                               lifetime `'_` defined here
2019-11-01T03:08:41.4062077Z    |                                               lifetime `'_` defined here
2019-11-01T03:08:41.4062173Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4062173Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4062634Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4062892Z 
2019-11-01T03:08:41.4062971Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4063360Z    |
2019-11-01T03:08:41.4063360Z    |
2019-11-01T03:08:41.4064030Z LL |     async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4064235Z    |
2019-11-01T03:08:41.4064235Z    |
2019-11-01T03:08:41.4064516Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4064653Z error: lifetime may not live long enough
2019-11-01T03:08:41.4064916Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:29:9
2019-11-01T03:08:41.4065011Z    |
2019-11-01T03:08:41.4065011Z    |
2019-11-01T03:08:41.4065429Z LL |     async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4065963Z    |                                               |
2019-11-01T03:08:41.4066683Z    |                                               lifetime `'_` defined here
2019-11-01T03:08:41.4067238Z    |                                               lifetime `'_` defined here
2019-11-01T03:08:41.4067798Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4067798Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4068210Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4068832Z error: aborting due to 10 previous errors
2019-11-01T03:08:41.4068910Z 
2019-11-01T03:08:41.4069251Z For more information about this error, try `rustc --explain E0700`.
2019-11-01T03:08:41.4069329Z 
---
2019-11-01T03:08:41.4070142Z 1 error: lifetime may not live long enough
2019-11-01T03:08:41.4070393Z -   --> $DIR/ref-mut-struct.rs:12:9
2019-11-01T03:08:41.4070650Z +   --> $DIR/ref-mut-struct.rs:11:9
2019-11-01T03:08:41.4070739Z 3    |
2019-11-01T03:08:41.4071167Z 4 LL |     fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4071546Z 
2019-11-01T03:08:41.4071546Z 
2019-11-01T03:08:41.4072008Z 9    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4072184Z 11 error: lifetime may not live long enough
2019-11-01T03:08:41.4072397Z -   --> $DIR/ref-mut-struct.rs:16:9
2019-11-01T03:08:41.4072623Z +   --> $DIR/ref-mut-struct.rs:15:9
2019-11-01T03:08:41.4072688Z 13    |
2019-11-01T03:08:41.4072688Z 13    |
2019-11-01T03:08:41.4072947Z 14 LL |     fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4073332Z 
2019-11-01T03:08:41.4073332Z 
2019-11-01T03:08:41.4073637Z 19    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4073808Z 21 error: lifetime may not live long enough
2019-11-01T03:08:41.4074024Z -   --> $DIR/ref-mut-struct.rs:20:9
2019-11-01T03:08:41.4074252Z +   --> $DIR/ref-mut-struct.rs:19:9
2019-11-01T03:08:41.4074317Z 23    |
2019-11-01T03:08:41.4074317Z 23    |
2019-11-01T03:08:41.4074577Z 24 LL |     fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4074949Z 
2019-11-01T03:08:41.4074949Z 
2019-11-01T03:08:41.4075235Z 29    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4075412Z 31 error: lifetime may not live long enough
2019-11-01T03:08:41.4075632Z -   --> $DIR/ref-mut-struct.rs:24:9
2019-11-01T03:08:41.4075860Z +   --> $DIR/ref-mut-struct.rs:23:9
2019-11-01T03:08:41.4075924Z 33    |
2019-11-01T03:08:41.4075924Z 33    |
2019-11-01T03:08:41.4076197Z 34 LL |     fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4076581Z 
2019-11-01T03:08:41.4076581Z 
2019-11-01T03:08:41.4076866Z 39    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4077028Z 41 error: lifetime may not live long enough
2019-11-01T03:08:41.4077257Z -   --> $DIR/ref-mut-struct.rs:28:9
2019-11-01T03:08:41.4077643Z +   --> $DIR/ref-mut-struct.rs:27:9
2019-11-01T03:08:41.4077911Z 43    |
2019-11-01T03:08:41.4077911Z 43    |
2019-11-01T03:08:41.4078908Z 44 LL |     fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4081053Z 
2019-11-01T03:08:41.4081090Z 
2019-11-01T03:08:41.4081219Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4081219Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4081884Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct.nll/ref-mut-struct.nll.stderr
2019-11-01T03:08:41.4082183Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.4082468Z To only update this specific test, also pass `--test-args self/elision/ref-mut-struct.rs`
2019-11-01T03:08:41.4082609Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.4082697Z status: exit code: 1
2019-11-01T03:08:41.4082697Z status: exit code: 1
2019-11-01T03:08:41.4083565Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.4084226Z ------------------------------------------
2019-11-01T03:08:41.4084278Z 
2019-11-01T03:08:41.4084539Z ------------------------------------------
2019-11-01T03:08:41.4084613Z stderr:
2019-11-01T03:08:41.4084613Z stderr:
2019-11-01T03:08:41.4084861Z ------------------------------------------
2019-11-01T03:08:41.4084937Z error: lifetime may not live long enough
2019-11-01T03:08:41.4085223Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:11:9
2019-11-01T03:08:41.4085305Z    |
2019-11-01T03:08:41.4085583Z LL |     fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4086048Z    |                         |
2019-11-01T03:08:41.4086365Z    |                         let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4086461Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4086461Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4086804Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4086956Z error: lifetime may not live long enough
2019-11-01T03:08:41.4087224Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:15:9
2019-11-01T03:08:41.4087320Z    |
2019-11-01T03:08:41.4087320Z    |
2019-11-01T03:08:41.4087784Z LL |     fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4088484Z    |                                 |
2019-11-01T03:08:41.4088786Z    |                                 let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4088895Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4088895Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4089222Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4089389Z error: lifetime may not live long enough
2019-11-01T03:08:41.4089678Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:19:9
2019-11-01T03:08:41.4089765Z    |
2019-11-01T03:08:41.4089765Z    |
2019-11-01T03:08:41.4117174Z LL |     fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4118302Z    |                                 |
2019-11-01T03:08:41.4118805Z    |                                 let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4118913Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4118913Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4119420Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4119580Z error: lifetime may not live long enough
2019-11-01T03:08:41.4119966Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:23:9
2019-11-01T03:08:41.4120058Z    |
2019-11-01T03:08:41.4120058Z    |
2019-11-01T03:08:41.4120335Z LL |     fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4120779Z    |                                         |
2019-11-01T03:08:41.4121085Z    |                                         let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4121188Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4121188Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4121513Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4121675Z error: lifetime may not live long enough
2019-11-01T03:08:41.4122115Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:27:9
2019-11-01T03:08:41.4122202Z    |
2019-11-01T03:08:41.4122202Z    |
2019-11-01T03:08:41.4122476Z LL |     fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4123057Z    |                                         |
2019-11-01T03:08:41.4123334Z    |                                         let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4123425Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4123425Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4123732Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4123858Z error: aborting due to 5 previous errors
2019-11-01T03:08:41.4123917Z 
2019-11-01T03:08:41.4123950Z 
2019-11-01T03:08:41.4124182Z ------------------------------------------
2019-11-01T03:08:41.4124182Z ------------------------------------------
2019-11-01T03:08:41.4124229Z 
2019-11-01T03:08:41.4124262Z 
2019-11-01T03:08:41.4124506Z ---- [ui (nll)] ui/self/elision/ref-self-async.rs stdout ----
2019-11-01T03:08:41.4124589Z diff of stderr:
2019-11-01T03:08:41.4124634Z 
2019-11-01T03:08:41.4124908Z - error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4125152Z -   --> $DIR/ref-self-async.rs:22:42
2019-11-01T03:08:41.4125250Z + error[E0658]: `Wrap<&Struct, Struct>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
2019-11-01T03:08:41.4125502Z +   --> $DIR/ref-self-async.rs:47:39
2019-11-01T03:08:41.4125588Z 3    |
2019-11-01T03:08:41.4125824Z - LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-11-01T03:08:41.4126270Z -    |
2019-11-01T03:08:41.4126270Z -    |
2019-11-01T03:08:41.4126535Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4126964Z - error: lifetime may not live long enough
2019-11-01T03:08:41.4127181Z -   --> $DIR/ref-self-async.rs:23:9
2019-11-01T03:08:41.4127398Z -    |
2019-11-01T03:08:41.4127398Z -    |
2019-11-01T03:08:41.4128199Z - LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-11-01T03:08:41.4128730Z -    |                       |
2019-11-01T03:08:41.4128985Z -    |                       lifetime `'_` defined here
2019-11-01T03:08:41.4129250Z -    |                       lifetime `'_` defined here
2019-11-01T03:08:41.4129466Z - LL |         f
2019-11-01T03:08:41.4129466Z - LL |         f
2019-11-01T03:08:41.4129797Z -    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4130029Z - 
2019-11-01T03:08:41.4130321Z - error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4130790Z -    |
2019-11-01T03:08:41.4130790Z -    |
2019-11-01T03:08:41.4131162Z - LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.4131907Z -    |
2019-11-01T03:08:41.4131907Z -    |
2019-11-01T03:08:41.4132152Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4132570Z - error: lifetime may not live long enough
2019-11-01T03:08:41.4132784Z -   --> $DIR/ref-self-async.rs:29:9
2019-11-01T03:08:41.4132978Z -    |
2019-11-01T03:08:41.4132978Z -    |
2019-11-01T03:08:41.4133206Z - LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-11-01T03:08:41.4133639Z -    |                             |
2019-11-01T03:08:41.4133889Z -    |                             lifetime `'_` defined here
2019-11-01T03:08:41.4134823Z -    |                             lifetime `'_` defined here
2019-11-01T03:08:41.4135089Z - LL |         f
2019-11-01T03:08:41.4135089Z - LL |         f
2019-11-01T03:08:41.4135414Z -    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4135821Z - 
2019-11-01T03:08:41.4138260Z - error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4138821Z -    |
2019-11-01T03:08:41.4138821Z -    |
2019-11-01T03:08:41.4139088Z - LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4139595Z -    |
2019-11-01T03:08:41.4139595Z -    |
2019-11-01T03:08:41.4139867Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4140317Z - error: lifetime may not live long enough
2019-11-01T03:08:41.4140837Z -   --> $DIR/ref-self-async.rs:33:9
2019-11-01T03:08:41.4141053Z -    |
2019-11-01T03:08:41.4141053Z -    |
2019-11-01T03:08:41.4141329Z - LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4141861Z -    |                                     |
2019-11-01T03:08:41.4142155Z -    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.4142436Z -    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.4142662Z - LL |         f
2019-11-01T03:08:41.4142662Z - LL |         f
2019-11-01T03:08:41.4142986Z -    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4143224Z - 
2019-11-01T03:08:41.4143515Z - error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4144151Z -    |
2019-11-01T03:08:41.4144151Z -    |
2019-11-01T03:08:41.4144564Z - LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4145224Z -    |
2019-11-01T03:08:41.4145224Z -    |
2019-11-01T03:08:41.4145648Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4146069Z - error: lifetime may not live long enough
2019-11-01T03:08:41.4146473Z -   --> $DIR/ref-self-async.rs:37:9
2019-11-01T03:08:41.4146676Z -    |
2019-11-01T03:08:41.4146676Z -    |
2019-11-01T03:08:41.4146935Z - LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4148063Z -    |                                     |
2019-11-01T03:08:41.4148349Z -    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.4148637Z -    |                                     lifetime `'_` defined here
2019-11-01T03:08:41.4148858Z - LL |         f
2019-11-01T03:08:41.4148858Z - LL |         f
2019-11-01T03:08:41.4149194Z -    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4149432Z - 
2019-11-01T03:08:41.4149878Z - error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4150507Z -    |
2019-11-01T03:08:41.4150507Z -    |
2019-11-01T03:08:41.4150799Z - LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4151315Z -    |
2019-11-01T03:08:41.4151315Z -    |
2019-11-01T03:08:41.4151598Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4152049Z - error: lifetime may not live long enough
2019-11-01T03:08:41.4152286Z -   --> $DIR/ref-self-async.rs:41:9
2019-11-01T03:08:41.4152496Z -    |
2019-11-01T03:08:41.4152496Z -    |
2019-11-01T03:08:41.4152768Z - LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4153302Z -    |                                             |
2019-11-01T03:08:41.4153604Z -    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.4154062Z -    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.4154451Z - LL |         f
2019-11-01T03:08:41.4154451Z - LL |         f
2019-11-01T03:08:41.4154931Z -    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4155151Z - 
2019-11-01T03:08:41.4155599Z - error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4156041Z -    |
2019-11-01T03:08:41.4156041Z -    |
2019-11-01T03:08:41.4156300Z - LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4156786Z -    |
2019-11-01T03:08:41.4156786Z -    |
2019-11-01T03:08:41.4157040Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4157656Z - error: lifetime may not live long enough
2019-11-01T03:08:41.4158318Z -   --> $DIR/ref-self-async.rs:45:9
2019-11-01T03:08:41.4158542Z -    |
2019-11-01T03:08:41.4158542Z -    |
2019-11-01T03:08:41.4158825Z - LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4159355Z -    |                                             |
2019-11-01T03:08:41.4159645Z -    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.4159936Z -    |                                             lifetime `'_` defined here
2019-11-01T03:08:41.4160166Z - LL |         f
2019-11-01T03:08:41.4160166Z - LL |         f
2019-11-01T03:08:41.4160486Z -    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4160718Z - 
2019-11-01T03:08:41.4161014Z - error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4161651Z -    |
2019-11-01T03:08:41.4161651Z -    |
2019-11-01T03:08:41.4161919Z 118 LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-11-01T03:08:41.4162320Z +    |                                       ^^^^^^^^^^^^^^^^^
2019-11-01T03:08:41.4162403Z 120    |
2019-11-01T03:08:41.4162403Z 120    |
2019-11-01T03:08:41.4162678Z -    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4163080Z +    = help: add `#![feature(arbitrary_self_types)]` to the crate attributes to enable
2019-11-01T03:08:41.4163080Z +    = help: add `#![feature(arbitrary_self_types)]` to the crate attributes to enable
2019-11-01T03:08:41.4163220Z +    = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)
2019-11-01T03:08:41.4163703Z - error: lifetime may not live long enough
2019-11-01T03:08:41.4163970Z -   --> $DIR/ref-self-async.rs:49:9
2019-11-01T03:08:41.4164277Z -    |
2019-11-01T03:08:41.4164277Z -    |
2019-11-01T03:08:41.4164545Z - LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-11-01T03:08:41.4165068Z -    |                                            |
2019-11-01T03:08:41.4165342Z -    |                                            lifetime `'_` defined here
2019-11-01T03:08:41.4165625Z -    |                                            lifetime `'_` defined here
2019-11-01T03:08:41.4167806Z - LL |         f
2019-11-01T03:08:41.4167806Z - LL |         f
2019-11-01T03:08:41.4168276Z -    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4168472Z 133 
2019-11-01T03:08:41.4168721Z - error: aborting due to 14 previous errors
2019-11-01T03:08:41.4168940Z - 
2019-11-01T03:08:41.4169648Z - For more information about this error, try `rustc --explain E0700`.
2019-11-01T03:08:41.4169648Z - For more information about this error, try `rustc --explain E0700`.
2019-11-01T03:08:41.4169962Z + For more information about this error, try `rustc --explain E0658`.
2019-11-01T03:08:41.4170048Z 137 
2019-11-01T03:08:41.4170088Z 
2019-11-01T03:08:41.4170124Z 
2019-11-01T03:08:41.4170201Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4170567Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll/ref-self-async.nll.stderr
2019-11-01T03:08:41.4171161Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.4171480Z To only update this specific test, also pass `--test-args self/elision/ref-self-async.rs`
2019-11-01T03:08:41.4171620Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.4171693Z status: exit code: 1
2019-11-01T03:08:41.4171693Z status: exit code: 1
2019-11-01T03:08:41.4172664Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.4173194Z ------------------------------------------
2019-11-01T03:08:41.4173253Z 
2019-11-01T03:08:41.4173485Z ------------------------------------------
2019-11-01T03:08:41.4173565Z stderr:
2019-11-01T03:08:41.4173565Z stderr:
2019-11-01T03:08:41.4173793Z ------------------------------------------
2019-11-01T03:08:41.4173903Z error[E0658]: `Wrap<&Struct, Struct>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
2019-11-01T03:08:41.4174515Z    |
2019-11-01T03:08:41.4174515Z    |
2019-11-01T03:08:41.4174794Z LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-11-01T03:08:41.4174972Z    |
2019-11-01T03:08:41.4175240Z    = note: for more information, see https://github.com/rust-lang/rust/issues/44874
2019-11-01T03:08:41.4175350Z    = help: add `#![feature(arbitrary_self_types)]` to the crate attributes to enable
2019-11-01T03:08:41.4175350Z    = help: add `#![feature(arbitrary_self_types)]` to the crate attributes to enable
2019-11-01T03:08:41.4175493Z    = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)
2019-11-01T03:08:41.4175655Z error: aborting due to previous error
2019-11-01T03:08:41.4175699Z 
2019-11-01T03:08:41.4175961Z For more information about this error, try `rustc --explain E0658`.
2019-11-01T03:08:41.4176196Z 
2019-11-01T03:08:41.4176196Z 
2019-11-01T03:08:41.4176458Z ------------------------------------------
2019-11-01T03:08:41.4176514Z 
2019-11-01T03:08:41.4176620Z 
2019-11-01T03:08:41.4176896Z ---- [ui (nll)] ui/self/elision/ref-struct-async.rs stdout ----
2019-11-01T03:08:41.4176977Z diff of stderr:
2019-11-01T03:08:41.4177022Z 
2019-11-01T03:08:41.4177103Z 1 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4177584Z +   --> $DIR/ref-struct-async.rs:12:52
2019-11-01T03:08:41.4178069Z 3    |
2019-11-01T03:08:41.4178069Z 3    |
2019-11-01T03:08:41.4178384Z 4 LL |     async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4178539Z 
2019-11-01T03:08:41.4178539Z 
2019-11-01T03:08:41.4178819Z 7    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4178985Z 9 error: lifetime may not live long enough
2019-11-01T03:08:41.4179227Z -   --> $DIR/ref-struct-async.rs:14:9
2019-11-01T03:08:41.4179470Z +   --> $DIR/ref-struct-async.rs:13:9
2019-11-01T03:08:41.4179556Z 11    |
2019-11-01T03:08:41.4179556Z 11    |
2019-11-01T03:08:41.4179819Z 12 LL |     async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4180119Z 
2019-11-01T03:08:41.4180119Z 
2019-11-01T03:08:41.4180701Z 18    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4180811Z 19 
2019-11-01T03:08:41.4180901Z 20 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4181611Z +   --> $DIR/ref-struct-async.rs:16:61
2019-11-01T03:08:41.4181682Z 22    |
2019-11-01T03:08:41.4181682Z 22    |
2019-11-01T03:08:41.4181946Z 23 LL |     async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4182113Z 
2019-11-01T03:08:41.4182113Z 
2019-11-01T03:08:41.4182379Z 26    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4182541Z 28 error: lifetime may not live long enough
2019-11-01T03:08:41.4182959Z -   --> $DIR/ref-struct-async.rs:18:9
2019-11-01T03:08:41.4183253Z +   --> $DIR/ref-struct-async.rs:17:9
2019-11-01T03:08:41.4183327Z 30    |
2019-11-01T03:08:41.4183327Z 30    |
2019-11-01T03:08:41.4183603Z 31 LL |     async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4183918Z 
2019-11-01T03:08:41.4183918Z 
2019-11-01T03:08:41.4184240Z 37    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4184343Z 38 
2019-11-01T03:08:41.4184426Z 39 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4185086Z +   --> $DIR/ref-struct-async.rs:20:61
2019-11-01T03:08:41.4185162Z 41    |
2019-11-01T03:08:41.4185162Z 41    |
2019-11-01T03:08:41.4185435Z 42 LL |     async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4185585Z 
2019-11-01T03:08:41.4185585Z 
2019-11-01T03:08:41.4185852Z 45    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4186009Z 47 error: lifetime may not live long enough
2019-11-01T03:08:41.4186247Z -   --> $DIR/ref-struct-async.rs:22:9
2019-11-01T03:08:41.4186474Z +   --> $DIR/ref-struct-async.rs:21:9
2019-11-01T03:08:41.4186550Z 49    |
2019-11-01T03:08:41.4186550Z 49    |
2019-11-01T03:08:41.4186810Z 50 LL |     async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4187114Z 
2019-11-01T03:08:41.4187114Z 
2019-11-01T03:08:41.4187543Z 56    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4188030Z 57 
2019-11-01T03:08:41.4188126Z 58 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4188798Z +   --> $DIR/ref-struct-async.rs:24:70
2019-11-01T03:08:41.4188876Z 60    |
2019-11-01T03:08:41.4188876Z 60    |
2019-11-01T03:08:41.4189157Z 61 LL |     async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4189330Z 
2019-11-01T03:08:41.4189330Z 
2019-11-01T03:08:41.4189608Z 64    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4189765Z 66 error: lifetime may not live long enough
2019-11-01T03:08:41.4190002Z -   --> $DIR/ref-struct-async.rs:26:9
2019-11-01T03:08:41.4190245Z +   --> $DIR/ref-struct-async.rs:25:9
2019-11-01T03:08:41.4190328Z 68    |
2019-11-01T03:08:41.4190328Z 68    |
2019-11-01T03:08:41.4190617Z 69 LL |     async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4190959Z 
2019-11-01T03:08:41.4190959Z 
2019-11-01T03:08:41.4191277Z 75    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4191383Z 76 
2019-11-01T03:08:41.4191471Z 77 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4192187Z +   --> $DIR/ref-struct-async.rs:28:66
2019-11-01T03:08:41.4192256Z 79    |
2019-11-01T03:08:41.4192256Z 79    |
2019-11-01T03:08:41.4192527Z 80 LL |     async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4192692Z 
2019-11-01T03:08:41.4192692Z 
2019-11-01T03:08:41.4192963Z 83    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4193113Z 85 error: lifetime may not live long enough
2019-11-01T03:08:41.4193358Z -   --> $DIR/ref-struct-async.rs:30:9
2019-11-01T03:08:41.4193589Z +   --> $DIR/ref-struct-async.rs:29:9
2019-11-01T03:08:41.4193658Z 87    |
2019-11-01T03:08:41.4193658Z 87    |
2019-11-01T03:08:41.4193928Z 88 LL |     async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4194240Z 
2019-11-01T03:08:41.4194275Z 
2019-11-01T03:08:41.4194341Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4194341Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4194700Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async.nll/ref-struct-async.nll.stderr
2019-11-01T03:08:41.4194996Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.4195299Z To only update this specific test, also pass `--test-args self/elision/ref-struct-async.rs`
2019-11-01T03:08:41.4195432Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.4195517Z status: exit code: 1
2019-11-01T03:08:41.4195517Z status: exit code: 1
2019-11-01T03:08:41.4196438Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-struct-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.4196948Z ------------------------------------------
2019-11-01T03:08:41.4196996Z 
2019-11-01T03:08:41.4197333Z ------------------------------------------
2019-11-01T03:08:41.4197587Z stderr:
2019-11-01T03:08:41.4197587Z stderr:
2019-11-01T03:08:41.4198069Z ------------------------------------------
2019-11-01T03:08:41.4198400Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4198829Z    |
2019-11-01T03:08:41.4198829Z    |
2019-11-01T03:08:41.4199087Z LL |     async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4199264Z    |
2019-11-01T03:08:41.4199264Z    |
2019-11-01T03:08:41.4199540Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4199671Z error: lifetime may not live long enough
2019-11-01T03:08:41.4199940Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:13:9
2019-11-01T03:08:41.4200026Z    |
2019-11-01T03:08:41.4200026Z    |
2019-11-01T03:08:41.4200279Z LL |     async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4200611Z    |                               |
2019-11-01T03:08:41.4200891Z    |                               lifetime `'_` defined here
2019-11-01T03:08:41.4201163Z    |                               lifetime `'_` defined here
2019-11-01T03:08:41.4201246Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4201246Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4201577Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4201650Z 
2019-11-01T03:08:41.4201740Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4202115Z    |
2019-11-01T03:08:41.4202115Z    |
2019-11-01T03:08:41.4202533Z LL |     async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4202727Z    |
2019-11-01T03:08:41.4202727Z    |
2019-11-01T03:08:41.4202987Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4203124Z error: lifetime may not live long enough
2019-11-01T03:08:41.4203391Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:17:9
2019-11-01T03:08:41.4203470Z    |
2019-11-01T03:08:41.4203470Z    |
2019-11-01T03:08:41.4203728Z LL |     async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4204054Z    |                                       |
2019-11-01T03:08:41.4204314Z    |                                       lifetime `'_` defined here
2019-11-01T03:08:41.4204590Z    |                                       lifetime `'_` defined here
2019-11-01T03:08:41.4204673Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4204673Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4204991Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4205070Z 
2019-11-01T03:08:41.4205156Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4205528Z    |
2019-11-01T03:08:41.4205528Z    |
2019-11-01T03:08:41.4205788Z LL |     async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4205964Z    |
2019-11-01T03:08:41.4205964Z    |
2019-11-01T03:08:41.4206219Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4206345Z error: lifetime may not live long enough
2019-11-01T03:08:41.4206611Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:21:9
2019-11-01T03:08:41.4206687Z    |
2019-11-01T03:08:41.4206687Z    |
2019-11-01T03:08:41.4206945Z LL |     async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4207722Z    |                                       |
2019-11-01T03:08:41.4208092Z    |                                       lifetime `'_` defined here
2019-11-01T03:08:41.4208475Z    |                                       lifetime `'_` defined here
2019-11-01T03:08:41.4208569Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4208569Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4208897Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4208975Z 
2019-11-01T03:08:41.4209057Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4209432Z    |
2019-11-01T03:08:41.4209432Z    |
2019-11-01T03:08:41.4209713Z LL |     async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4209903Z    |
2019-11-01T03:08:41.4209903Z    |
2019-11-01T03:08:41.4210187Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4210327Z error: lifetime may not live long enough
2019-11-01T03:08:41.4210598Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:25:9
2019-11-01T03:08:41.4210685Z    |
2019-11-01T03:08:41.4210685Z    |
2019-11-01T03:08:41.4211119Z LL |     async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4211454Z    |                                               |
2019-11-01T03:08:41.4211730Z    |                                               lifetime `'_` defined here
2019-11-01T03:08:41.4212005Z    |                                               lifetime `'_` defined here
2019-11-01T03:08:41.4212087Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4212087Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4212732Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4212816Z 
2019-11-01T03:08:41.4212899Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2019-11-01T03:08:41.4213301Z    |
2019-11-01T03:08:41.4213301Z    |
2019-11-01T03:08:41.4213554Z LL |     async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4213727Z    |
2019-11-01T03:08:41.4213727Z    |
2019-11-01T03:08:41.4213982Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2019-11-01T03:08:41.4214106Z error: lifetime may not live long enough
2019-11-01T03:08:41.4214753Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:29:9
2019-11-01T03:08:41.4214838Z    |
2019-11-01T03:08:41.4214838Z    |
2019-11-01T03:08:41.4215115Z LL |     async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4215474Z    |                                           |
2019-11-01T03:08:41.4215751Z    |                                           lifetime `'_` defined here
2019-11-01T03:08:41.4216052Z    |                                           lifetime `'_` defined here
2019-11-01T03:08:41.4216139Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4216139Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4216467Z    |         ^ function was supposed to return data with lifetime `'_` but it is returning data with lifetime `'_`
2019-11-01T03:08:41.4216610Z error: aborting due to 10 previous errors
2019-11-01T03:08:41.4216657Z 
2019-11-01T03:08:41.4216923Z For more information about this error, try `rustc --explain E0700`.
2019-11-01T03:08:41.4216980Z 
---
2019-11-01T03:08:41.4218148Z 1 error: lifetime may not live long enough
2019-11-01T03:08:41.4218413Z -   --> $DIR/ref-struct.rs:12:9
2019-11-01T03:08:41.4218739Z +   --> $DIR/ref-struct.rs:11:9
2019-11-01T03:08:41.4218821Z 3    |
2019-11-01T03:08:41.4219073Z 4 LL |     fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4219450Z 
2019-11-01T03:08:41.4219450Z 
2019-11-01T03:08:41.4219774Z 9    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4219953Z 11 error: lifetime may not live long enough
2019-11-01T03:08:41.4220191Z -   --> $DIR/ref-struct.rs:16:9
2019-11-01T03:08:41.4220572Z +   --> $DIR/ref-struct.rs:15:9
2019-11-01T03:08:41.4220820Z 13    |
2019-11-01T03:08:41.4220820Z 13    |
2019-11-01T03:08:41.4221105Z 14 LL |     fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4221509Z 
2019-11-01T03:08:41.4221509Z 
2019-11-01T03:08:41.4221825Z 19    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4221991Z 21 error: lifetime may not live long enough
2019-11-01T03:08:41.4222221Z -   --> $DIR/ref-struct.rs:20:9
2019-11-01T03:08:41.4222443Z +   --> $DIR/ref-struct.rs:19:9
2019-11-01T03:08:41.4222518Z 23    |
2019-11-01T03:08:41.4222518Z 23    |
2019-11-01T03:08:41.4222771Z 24 LL |     fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4223150Z 
2019-11-01T03:08:41.4223150Z 
2019-11-01T03:08:41.4223462Z 29    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4223634Z 31 error: lifetime may not live long enough
2019-11-01T03:08:41.4223859Z -   --> $DIR/ref-struct.rs:24:9
2019-11-01T03:08:41.4224096Z +   --> $DIR/ref-struct.rs:23:9
2019-11-01T03:08:41.4224165Z 33    |
2019-11-01T03:08:41.4224165Z 33    |
2019-11-01T03:08:41.4224433Z 34 LL |     fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4224849Z 
2019-11-01T03:08:41.4224849Z 
2019-11-01T03:08:41.4225159Z 39    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4225327Z 41 error: lifetime may not live long enough
2019-11-01T03:08:41.4225550Z -   --> $DIR/ref-struct.rs:28:9
2019-11-01T03:08:41.4225776Z +   --> $DIR/ref-struct.rs:27:9
2019-11-01T03:08:41.4225843Z 43    |
2019-11-01T03:08:41.4225843Z 43    |
2019-11-01T03:08:41.4226110Z 44 LL |     fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4226708Z 
2019-11-01T03:08:41.4226745Z 
2019-11-01T03:08:41.4226823Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4226823Z The actual stderr differed from the expected stderr.
2019-11-01T03:08:41.4227173Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct.nll/ref-struct.nll.stderr
2019-11-01T03:08:41.4227477Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T03:08:41.4228028Z To only update this specific test, also pass `--test-args self/elision/ref-struct.rs`
2019-11-01T03:08:41.4228173Z error: 1 errors occurred comparing output.
2019-11-01T03:08:41.4228252Z status: exit code: 1
2019-11-01T03:08:41.4228252Z status: exit code: 1
2019-11-01T03:08:41.4229312Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct.nll/auxiliary" "-A" "unused"
2019-11-01T03:08:41.4229915Z ------------------------------------------
2019-11-01T03:08:41.4229967Z 
2019-11-01T03:08:41.4230207Z ------------------------------------------
2019-11-01T03:08:41.4230280Z stderr:
2019-11-01T03:08:41.4230280Z stderr:
2019-11-01T03:08:41.4230514Z ------------------------------------------
2019-11-01T03:08:41.4230601Z error: lifetime may not live long enough
2019-11-01T03:08:41.4230864Z   --> /checkout/src/test/ui/self/elision/ref-struct.rs:11:9
2019-11-01T03:08:41.4230951Z    |
2019-11-01T03:08:41.4231203Z LL |     fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-11-01T03:08:41.4231613Z    |                         |
2019-11-01T03:08:41.4232062Z    |                         let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4232154Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4232154Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4232467Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4232612Z error: lifetime may not live long enough
2019-11-01T03:08:41.4232875Z   --> /checkout/src/test/ui/self/elision/ref-struct.rs:15:9
2019-11-01T03:08:41.4232949Z    |
2019-11-01T03:08:41.4232949Z    |
2019-11-01T03:08:41.4233205Z LL |     fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4233608Z    |                                 |
2019-11-01T03:08:41.4233897Z    |                                 let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4233986Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4233986Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4234479Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4234613Z error: lifetime may not live long enough
2019-11-01T03:08:41.4234857Z   --> /checkout/src/test/ui/self/elision/ref-struct.rs:19:9
2019-11-01T03:08:41.4234937Z    |
2019-11-01T03:08:41.4234937Z    |
2019-11-01T03:08:41.4235176Z LL |     fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4235571Z    |                                 |
2019-11-01T03:08:41.4235836Z    |                                 let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4235928Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4235928Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4236241Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4236376Z error: lifetime may not live long enough
2019-11-01T03:08:41.4236635Z   --> /checkout/src/test/ui/self/elision/ref-struct.rs:23:9
2019-11-01T03:08:41.4236709Z    |
2019-11-01T03:08:41.4236709Z    |
2019-11-01T03:08:41.4236963Z LL |     fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4237369Z    |                                         |
2019-11-01T03:08:41.4238248Z    |                                         let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4238362Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4238362Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4238745Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4238991Z error: lifetime may not live long enough
2019-11-01T03:08:41.4239287Z   --> /checkout/src/test/ui/self/elision/ref-struct.rs:27:9
2019-11-01T03:08:41.4239445Z    |
2019-11-01T03:08:41.4239445Z    |
2019-11-01T03:08:41.4239733Z LL |     fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
2019-11-01T03:08:41.4240160Z    |                                     |
2019-11-01T03:08:41.4240457Z    |                                     let's call the lifetime of this reference `'2`
2019-11-01T03:08:41.4240558Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4240558Z LL |         f //~ ERROR lifetime mismatch
2019-11-01T03:08:41.4240883Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-11-01T03:08:41.4241026Z error: aborting due to 5 previous errors
2019-11-01T03:08:41.4241073Z 
2019-11-01T03:08:41.4241115Z 
2019-11-01T03:08:41.4241355Z ------------------------------------------
---
2019-11-01T03:08:41.4245034Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-01T03:08:41.4245140Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-01T03:08:41.4245194Z 
2019-11-01T03:08:41.4245234Z 
2019-11-01T03:08:41.4247031Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-11-01T03:08:41.4248217Z 
2019-11-01T03:08:41.4248266Z 
2019-11-01T03:08:41.4254348Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-01T03:08:41.4254480Z Build completed unsuccessfully in 1:51:01
2019-11-01T03:08:41.4254480Z Build completed unsuccessfully in 1:51:01
2019-11-01T03:08:41.4254594Z == clock drift check ==
2019-11-01T03:08:41.4254783Z   local time: Fri Nov  1 03:08:41 UTC 2019
2019-11-01T03:08:41.6676537Z   network time: Fri, 01 Nov 2019 03:08:41 GMT
2019-11-01T03:08:41.6677139Z == end clock drift check ==
2019-11-01T03:08:42.7804105Z 
2019-11-01T03:08:42.7971070Z ##[error]Bash exited with code '1'.
2019-11-01T03:08:42.8015099Z ##[section]Starting: Checkout
2019-11-01T03:08:42.8016933Z ==============================================================================
2019-11-01T03:08:42.8017040Z Task         : Get sources
2019-11-01T03:08:42.8017121Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
