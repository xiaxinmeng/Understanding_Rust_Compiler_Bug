plain
2019-09-03T12:20:28.3751464Z 12 
2019-09-03T12:20:28.3751995Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3752551Z -   --> $DIR/issue-8460-const.rs:7:36
2019-09-03T12:20:28.3753088Z -    |
2019-09-03T12:20:28.3753684Z - LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3754915Z -    |                                    ^^^^^^^^^^^^^^^ attempt to divide with overflow
2019-09-03T12:20:28.3755833Z 19 error: attempt to divide with overflow
2019-09-03T12:20:28.3756284Z 20   --> $DIR/issue-8460-const.rs:10:36
2019-09-03T12:20:28.3756519Z 21    |
2019-09-03T12:20:28.3756664Z 
2019-09-03T12:20:28.3756664Z 
2019-09-03T12:20:28.3757101Z 22 LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3757772Z 24 
2019-09-03T12:20:28.3758173Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3758716Z -   --> $DIR/issue-8460-const.rs:10:36
2019-09-03T12:20:28.3759049Z -    |
2019-09-03T12:20:28.3759049Z -    |
2019-09-03T12:20:28.3759463Z - LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3760486Z -    |                                    ^^^^^^^^^^^^ attempt to divide with overflow
2019-09-03T12:20:28.3761279Z 31 error: attempt to divide with overflow
2019-09-03T12:20:28.3761657Z 32   --> $DIR/issue-8460-const.rs:13:36
2019-09-03T12:20:28.3761878Z 33    |
2019-09-03T12:20:28.3762014Z 
2019-09-03T12:20:28.3762014Z 
2019-09-03T12:20:28.3762438Z 34 LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3762855Z 36 
2019-09-03T12:20:28.3763218Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3763628Z -   --> $DIR/issue-8460-const.rs:13:36
2019-09-03T12:20:28.3763980Z -    |
2019-09-03T12:20:28.3763980Z -    |
2019-09-03T12:20:28.3764422Z - LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3764864Z -    |                                    ^^^^^^^^^^^^^ attempt to divide with overflow
2019-09-03T12:20:28.3765659Z 43 error: attempt to divide with overflow
2019-09-03T12:20:28.3766081Z 44   --> $DIR/issue-8460-const.rs:16:36
2019-09-03T12:20:28.3766313Z 45    |
2019-09-03T12:20:28.3766450Z 
2019-09-03T12:20:28.3766450Z 
2019-09-03T12:20:28.3766872Z 46 LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3768405Z 48 
2019-09-03T12:20:28.3771850Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3772397Z -   --> $DIR/issue-8460-const.rs:16:36
2019-09-03T12:20:28.3772772Z -    |
2019-09-03T12:20:28.3772772Z -    |
2019-09-03T12:20:28.3773221Z - LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3773699Z -    |                                    ^^^^^^^^^^^^^ attempt to divide with overflow
2019-09-03T12:20:28.3774288Z 55 error: attempt to divide with overflow
2019-09-03T12:20:28.3774662Z 56   --> $DIR/issue-8460-const.rs:19:36
2019-09-03T12:20:28.3774908Z 57    |
2019-09-03T12:20:28.3775045Z 
2019-09-03T12:20:28.3775045Z 
2019-09-03T12:20:28.3775826Z 58 LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3776254Z 60 
2019-09-03T12:20:28.3776626Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3777041Z -   --> $DIR/issue-8460-const.rs:19:36
2019-09-03T12:20:28.3777393Z -    |
2019-09-03T12:20:28.3777393Z -    |
2019-09-03T12:20:28.3777843Z - LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3778309Z -    |                                    ^^^^^^^^^^^^^ attempt to divide with overflow
2019-09-03T12:20:28.3778882Z 67 error: attempt to divide by zero
2019-09-03T12:20:28.3779254Z 68   --> $DIR/issue-8460-const.rs:22:36
2019-09-03T12:20:28.3779469Z 69    |
2019-09-03T12:20:28.3779628Z 
2019-09-03T12:20:28.3779628Z 
2019-09-03T12:20:28.3780962Z 130 LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3781507Z 132 
2019-09-03T12:20:28.3781953Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3782430Z -   --> $DIR/issue-8460-const.rs:37:36
2019-09-03T12:20:28.3783336Z -    |
2019-09-03T12:20:28.3783336Z -    |
2019-09-03T12:20:28.3784433Z - LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3785282Z -    |                                    ^^^^^^^^^^^^^^^ attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3785703Z 139 error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3785966Z 140   --> $DIR/issue-8460-const.rs:40:36
2019-09-03T12:20:28.3786055Z 141    |
2019-09-03T12:20:28.3786094Z 
2019-09-03T12:20:28.3786094Z 
2019-09-03T12:20:28.3786375Z 142 LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3786576Z 144 
2019-09-03T12:20:28.3786840Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3787341Z -   --> $DIR/issue-8460-const.rs:40:36
2019-09-03T12:20:28.3787613Z -    |
2019-09-03T12:20:28.3787613Z -    |
2019-09-03T12:20:28.3787903Z - LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3788222Z -    |                                    ^^^^^^^^^^^^ attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3788525Z 151 error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3788791Z 152   --> $DIR/issue-8460-const.rs:43:36
2019-09-03T12:20:28.3788859Z 153    |
2019-09-03T12:20:28.3788896Z 
2019-09-03T12:20:28.3788896Z 
2019-09-03T12:20:28.3789183Z 154 LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3789359Z 156 
2019-09-03T12:20:28.3789594Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3790000Z -   --> $DIR/issue-8460-const.rs:43:36
2019-09-03T12:20:28.3790447Z -    |
2019-09-03T12:20:28.3790447Z -    |
2019-09-03T12:20:28.3790758Z - LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3791096Z -    |                                    ^^^^^^^^^^^^^ attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3791398Z 163 error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3792039Z 164   --> $DIR/issue-8460-const.rs:46:36
2019-09-03T12:20:28.3792133Z 165    |
2019-09-03T12:20:28.3792170Z 
2019-09-03T12:20:28.3792170Z 
2019-09-03T12:20:28.3792811Z 166 LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3793000Z 168 
2019-09-03T12:20:28.3806089Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3806499Z -   --> $DIR/issue-8460-const.rs:46:36
2019-09-03T12:20:28.3806950Z -    |
2019-09-03T12:20:28.3806950Z -    |
2019-09-03T12:20:28.3807277Z - LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3808099Z -    |                                    ^^^^^^^^^^^^^ attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3808663Z 175 error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3808982Z 176   --> $DIR/issue-8460-const.rs:49:36
2019-09-03T12:20:28.3809073Z 177    |
2019-09-03T12:20:28.3809113Z 
2019-09-03T12:20:28.3809113Z 
2019-09-03T12:20:28.3809611Z 178 LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3809823Z 180 
2019-09-03T12:20:28.3810545Z - error: this expression will panic at runtime
2019-09-03T12:20:28.3810840Z -   --> $DIR/issue-8460-const.rs:49:36
2019-09-03T12:20:28.3811293Z -    |
2019-09-03T12:20:28.3811293Z -    |
2019-09-03T12:20:28.3811603Z - LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3812136Z -    |                                    ^^^^^^^^^^^^^ attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3812420Z - 
2019-09-03T12:20:28.3812494Z 187 error: attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3813085Z 189    |
2019-09-03T12:20:28.3813123Z 
2019-09-03T12:20:28.3813123Z 
2019-09-03T12:20:28.3813214Z 244 LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
2019-09-03T12:20:28.3813312Z 245    |                                    ^^^^^^^^ attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3813899Z - error: aborting due to 40 previous errors
2019-09-03T12:20:28.3813974Z + error: aborting due to 30 previous errors
2019-09-03T12:20:28.3814058Z 248 
2019-09-03T12:20:28.3814115Z 249 
2019-09-03T12:20:28.3814115Z 249 
2019-09-03T12:20:28.3814314Z 
2019-09-03T12:20:28.3814371Z 
2019-09-03T12:20:28.3814441Z The actual stderr differed from the expected stderr.
2019-09-03T12:20:28.3815207Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-8460-const/issue-8460-const.stderr
2019-09-03T12:20:28.3816017Z To update references, rerun the tests and pass the `--bless` flag
2019-09-03T12:20:28.3816386Z To only update this specific test, also pass `--test-args issues/issue-8460-const.rs`
2019-09-03T12:20:28.3816522Z error: 1 errors occurred comparing output.
2019-09-03T12:20:28.3816743Z status: exit code: 1
2019-09-03T12:20:28.3816743Z status: exit code: 1
2019-09-03T12:20:28.3818019Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-8460-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-8460-const" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-8460-const/auxiliary" "-A" "unused"
2019-09-03T12:20:28.3819458Z ------------------------------------------
2019-09-03T12:20:28.3819519Z 
2019-09-03T12:20:28.3819785Z ------------------------------------------
2019-09-03T12:20:28.3819853Z stderr:
2019-09-03T12:20:28.3819853Z stderr:
2019-09-03T12:20:28.3820542Z ------------------------------------------
2019-09-03T12:20:28.3820633Z error: attempt to divide with overflow
2019-09-03T12:20:28.3821189Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:7:36
2019-09-03T12:20:28.3821277Z    |
2019-09-03T12:20:28.3821773Z LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3821980Z    |
2019-09-03T12:20:28.3822061Z note: lint level defined here
2019-09-03T12:20:28.3822594Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:1:9
2019-09-03T12:20:28.3822706Z    |
2019-09-03T12:20:28.3822706Z    |
2019-09-03T12:20:28.3822770Z LL | #![deny(const_err)]
2019-09-03T12:20:28.3822871Z    |         ^^^^^^^^^
2019-09-03T12:20:28.3822916Z 
2019-09-03T12:20:28.3822987Z error: attempt to divide with overflow
2019-09-03T12:20:28.3823551Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:10:36
2019-09-03T12:20:28.3823642Z    |
2019-09-03T12:20:28.3824136Z LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3824469Z 
2019-09-03T12:20:28.3824527Z error: attempt to divide with overflow
2019-09-03T12:20:28.3825069Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:13:36
2019-09-03T12:20:28.3825153Z    |
2019-09-03T12:20:28.3825153Z    |
2019-09-03T12:20:28.3825800Z LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3825965Z 
2019-09-03T12:20:28.3826025Z error: attempt to divide with overflow
2019-09-03T12:20:28.3826499Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:16:36
2019-09-03T12:20:28.3826603Z    |
2019-09-03T12:20:28.3826603Z    |
2019-09-03T12:20:28.3826959Z LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3827282Z 
2019-09-03T12:20:28.3827380Z error: attempt to divide with overflow
2019-09-03T12:20:28.3827711Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:19:36
2019-09-03T12:20:28.3827801Z    |
2019-09-03T12:20:28.3827801Z    |
2019-09-03T12:20:28.3828298Z LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
2019-09-03T12:20:28.3828464Z 
2019-09-03T12:20:28.3828541Z error: attempt to divide by zero
2019-09-03T12:20:28.3829026Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:22:36
2019-09-03T12:20:28.3829132Z    |
2019-09-03T12:20:28.3829132Z    |
2019-09-03T12:20:28.3829203Z LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
2019-09-03T12:20:28.3829364Z 
2019-09-03T12:20:28.3829724Z error: this expression will panic at runtime
2019-09-03T12:20:28.3830584Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:22:36
2019-09-03T12:20:28.3830700Z    |
2019-09-03T12:20:28.3830700Z    |
2019-09-03T12:20:28.3830770Z LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
2019-09-03T12:20:28.3830875Z    |                                    ^^^^^^^^^^ attempt to divide by zero
2019-09-03T12:20:28.3830990Z error: attempt to divide by zero
2019-09-03T12:20:28.3831507Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:25:36
2019-09-03T12:20:28.3831613Z    |
2019-09-03T12:20:28.3831613Z    |
2019-09-03T12:20:28.3831684Z LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
2019-09-03T12:20:28.3831831Z 
2019-09-03T12:20:28.3832037Z error: this expression will panic at runtime
2019-09-03T12:20:28.3832397Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:25:36
2019-09-03T12:20:28.3832620Z    |
2019-09-03T12:20:28.3832620Z    |
2019-09-03T12:20:28.3832919Z LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
2019-09-03T12:20:28.3833011Z    |                                    ^^^^^^^ attempt to divide by zero
2019-09-03T12:20:28.3833144Z error: attempt to divide by zero
2019-09-03T12:20:28.3833697Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:28:36
2019-09-03T12:20:28.3833789Z    |
2019-09-03T12:20:28.3833789Z    |
2019-09-03T12:20:28.3833876Z LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
2019-09-03T12:20:28.3834022Z 
2019-09-03T12:20:28.3834083Z error: this expression will panic at runtime
2019-09-03T12:20:28.3834624Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:28:36
2019-09-03T12:20:28.3834870Z    |
2019-09-03T12:20:28.3834870Z    |
2019-09-03T12:20:28.3834958Z LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
2019-09-03T12:20:28.3835053Z    |                                    ^^^^^^^^ attempt to divide by zero
2019-09-03T12:20:28.3835501Z error: attempt to divide by zero
2019-09-03T12:20:28.3835954Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:31:36
2019-09-03T12:20:28.3836188Z    |
2019-09-03T12:20:28.3836188Z    |
2019-09-03T12:20:28.3836259Z LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
2019-09-03T12:20:28.3836396Z 
2019-09-03T12:20:28.3836468Z error: this expression will panic at runtime
2019-09-03T12:20:28.3836760Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:31:36
2019-09-03T12:20:28.3837005Z    |
2019-09-03T12:20:28.3837005Z    |
2019-09-03T12:20:28.3837072Z LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
2019-09-03T12:20:28.3837166Z    |                                    ^^^^^^^^ attempt to divide by zero
2019-09-03T12:20:28.3837283Z error: attempt to divide by zero
2019-09-03T12:20:28.3837571Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:34:36
2019-09-03T12:20:28.3838002Z    |
2019-09-03T12:20:28.3838002Z    |
2019-09-03T12:20:28.3838083Z LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
2019-09-03T12:20:28.3838217Z 
2019-09-03T12:20:28.3838273Z error: this expression will panic at runtime
2019-09-03T12:20:28.3838595Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:34:36
2019-09-03T12:20:28.3838680Z    |
2019-09-03T12:20:28.3838680Z    |
2019-09-03T12:20:28.3838745Z LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
2019-09-03T12:20:28.3838844Z    |                                    ^^^^^^^^ attempt to divide by zero
2019-09-03T12:20:28.3838954Z error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3839756Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:37:36
2019-09-03T12:20:28.3839842Z    |
2019-09-03T12:20:28.3839842Z    |
2019-09-03T12:20:28.3840503Z LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3840837Z 
2019-09-03T12:20:28.3840908Z error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3841259Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:40:36
2019-09-03T12:20:28.3841337Z    |
2019-09-03T12:20:28.3841337Z    |
2019-09-03T12:20:28.3841907Z LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3842088Z 
2019-09-03T12:20:28.3842157Z error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3842505Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:43:36
2019-09-03T12:20:28.3842585Z    |
2019-09-03T12:20:28.3842585Z    |
2019-09-03T12:20:28.3842888Z LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3843029Z 
2019-09-03T12:20:28.3843116Z error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3843591Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:46:36
2019-09-03T12:20:28.3843672Z    |
2019-09-03T12:20:28.3843672Z    |
2019-09-03T12:20:28.3843978Z LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3844119Z 
2019-09-03T12:20:28.3844209Z error: attempt to calculate the remainder with overflow
2019-09-03T12:20:28.3844493Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:49:36
2019-09-03T12:20:28.3844588Z    |
2019-09-03T12:20:28.3844588Z    |
2019-09-03T12:20:28.3844871Z LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
2019-09-03T12:20:28.3845033Z 
2019-09-03T12:20:28.3845033Z 
2019-09-03T12:20:28.3845121Z error: attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3846588Z    |
2019-09-03T12:20:28.3846588Z    |
2019-09-03T12:20:28.3846660Z LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
2019-09-03T12:20:28.3846793Z 
2019-09-03T12:20:28.3846864Z error: this expression will panic at runtime
2019-09-03T12:20:28.3847116Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:52:36
2019-09-03T12:20:28.3847197Z    |
2019-09-03T12:20:28.3847197Z    |
2019-09-03T12:20:28.3847260Z LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
2019-09-03T12:20:28.3847367Z    |                                    ^^^^^^^^^^ attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3847701Z 
2019-09-03T12:20:28.3847787Z error: attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3848146Z    |
2019-09-03T12:20:28.3848146Z    |
2019-09-03T12:20:28.3848209Z LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
2019-09-03T12:20:28.3848525Z 
2019-09-03T12:20:28.3848604Z error: this expression will panic at runtime
2019-09-03T12:20:28.3848853Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:55:36
2019-09-03T12:20:28.3848936Z    |
2019-09-03T12:20:28.3848936Z    |
2019-09-03T12:20:28.3849000Z LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
2019-09-03T12:20:28.3849105Z    |                                    ^^^^^^^ attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3849163Z 
2019-09-03T12:20:28.3849229Z error: attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3850352Z    |
2019-09-03T12:20:28.3850352Z    |
2019-09-03T12:20:28.3850424Z LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
2019-09-03T12:20:28.3850565Z 
2019-09-03T12:20:28.3850627Z error: this expression will panic at runtime
2019-09-03T12:20:28.3851091Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:58:36
2019-09-03T12:20:28.3851180Z    |
2019-09-03T12:20:28.3851180Z    |
2019-09-03T12:20:28.3851268Z LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
2019-09-03T12:20:28.3851362Z    |                                    ^^^^^^^^ attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3851440Z 
2019-09-03T12:20:28.3851508Z error: attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3851901Z    |
2019-09-03T12:20:28.3851901Z    |
2019-09-03T12:20:28.3851989Z LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
2019-09-03T12:20:28.3852129Z 
2019-09-03T12:20:28.3852191Z error: this expression will panic at runtime
2019-09-03T12:20:28.3852471Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:61:36
2019-09-03T12:20:28.3859435Z    |
2019-09-03T12:20:28.3859435Z    |
2019-09-03T12:20:28.3862117Z LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
2019-09-03T12:20:28.3862236Z    |                                    ^^^^^^^^ attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3862316Z 
2019-09-03T12:20:28.3862386Z error: attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3862971Z    |
2019-09-03T12:20:28.3862971Z    |
2019-09-03T12:20:28.3863061Z LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
2019-09-03T12:20:28.3863188Z 
2019-09-03T12:20:28.3863266Z error: this expression will panic at runtime
2019-09-03T12:20:28.3864372Z   --> /checkout/src/test/ui/issues/issue-8460-const.rs:64:36
2019-09-03T12:20:28.3864480Z    |
2019-09-03T12:20:28.3864480Z    |
2019-09-03T12:20:28.3864551Z LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
2019-09-03T12:20:28.3864784Z    |                                    ^^^^^^^^ attempt to calculate the remainder with a divisor of zero
2019-09-03T12:20:28.3865784Z error: aborting due to 30 previous errors
2019-09-03T12:20:28.3865827Z 
2019-09-03T12:20:28.3865862Z 
2019-09-03T12:20:28.3866367Z ------------------------------------------
---
2019-09-03T12:20:28.3867516Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-03T12:20:28.3867629Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-03T12:20:28.3867682Z 
2019-09-03T12:20:28.3867731Z 
2019-09-03T12:20:28.3871278Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-03T12:20:28.3871956Z 
2019-09-03T12:20:28.3871996Z 
2019-09-03T12:20:28.3872131Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-03T12:20:28.3872210Z Build completed unsuccessfully in 1:50:11
2019-09-03T12:20:28.3872210Z Build completed unsuccessfully in 1:50:11
2019-09-03T12:20:28.3872297Z == clock drift check ==
2019-09-03T12:20:28.3893206Z   local time: Tue Sep  3 12:20:28 UTC 2019
2019-09-03T12:20:28.5365643Z   network time: Tue, 03 Sep 2019 12:20:28 GMT
2019-09-03T12:20:28.5370712Z == end clock drift check ==
2019-09-03T12:20:29.4231111Z ##[error]Bash exited with code '1'.
2019-09-03T12:20:29.4283727Z ##[section]Starting: Upload CPU usage statistics
2019-09-03T12:20:29.4292110Z ==============================================================================
2019-09-03T12:20:29.4292206Z Task         : Bash
2019-09-03T12:20:29.4292293Z Description  : Run a Bash script on macOS, Linux, or Windows
