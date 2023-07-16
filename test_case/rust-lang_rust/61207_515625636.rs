plain
2019-07-26T21:24:16.2196980Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T21:24:16.2198292Z 
2019-07-26T21:24:16.2199469Z   git checkout -b <new-branch-name>
2019-07-26T21:24:16.2200387Z 
2019-07-26T21:24:16.2201657Z HEAD is now at 8bb5fa594 Auto merge of #61207 - taiki-e:arbitrary_self_types-lifetime-elision-2, r=Centril
2019-07-26T21:24:16.2328374Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T21:24:16.2331733Z ==============================================================================
2019-07-26T21:24:16.2331840Z Task         : Bash
2019-07-26T21:24:16.2332093Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T23:04:33.8793906Z test [ui] ui/self/elision/alias.rs ... ok
2019-07-26T23:04:33.9688774Z test [ui] ui/self/elision/assoc.rs ... ok
2019-07-26T23:04:34.1955807Z test [ui] ui/self/elision/lt-alias.rs ... ok
2019-07-26T23:04:34.2595123Z test [ui] ui/self/builtin-superkinds-self-type.rs ... ok
2019-07-26T23:04:34.2686927Z test [ui] ui/self/elision/lt-assoc.rs ... ok
2019-07-26T23:04:34.3304017Z test [ui] ui/self/elision/lt-ref-self.rs ... ok
2019-07-26T23:04:34.3345607Z test [ui] ui/self/elision/lt-self.rs ... ok
2019-07-26T23:04:34.3987034Z test [ui] ui/self/elision/lt-struct.rs ... ok
2019-07-26T23:04:34.4088513Z test [ui] ui/self/elision/multiple-ref-self.rs ... ok
2019-07-26T23:04:34.4744302Z test [ui] ui/self/elision/ref-alias.rs ... ok
2019-07-26T23:04:34.4769358Z test [ui] ui/self/elision/ref-assoc.rs ... ok
2019-07-26T23:04:34.5436424Z test [ui] ui/self/elision/ref-mut-self.rs ... ok
2019-07-26T23:04:34.5457308Z test [ui] ui/self/elision/ref-mut-alias.rs ... ok
2019-07-26T23:04:34.6145910Z test [ui] ui/self/elision/ref-mut-struct.rs ... ok
2019-07-26T23:04:34.6184479Z test [ui] ui/self/elision/ref-self.rs ... ok
2019-07-26T23:04:34.6841729Z test [ui] ui/self/elision/self.rs ... ok
2019-07-26T23:04:34.6853195Z test [ui] ui/self/elision/ref-struct.rs ... ok
2019-07-26T23:04:34.7548170Z test [ui] ui/self/elision/struct.rs ... ok
2019-07-26T23:04:35.0306340Z test [ui] ui/self/explicit-self-generic.rs ... ok
2019-07-26T23:04:35.0554201Z test [ui] ui/self/explicit-self-objects-uniq.rs ... ok
2019-07-26T23:04:35.2915343Z test [ui] ui/self/explicit-self.rs ... ok
2019-07-26T23:04:35.4339844Z test [ui] ui/self/explicit_self_xcrate_exe.rs ... ok
---
2019-07-26T23:09:35.2312335Z test [ui (nll)] ui/self/arbitrary_self_types_struct.rs ... ok
2019-07-26T23:09:35.5125215Z test [ui (nll)] ui/self/arbitrary_self_types_unsized_struct.rs ... ok
2019-07-26T23:09:35.5369335Z test [ui (nll)] ui/self/arbitrary_self_types_trait.rs ... ok
2019-07-26T23:09:35.7171225Z test [ui (nll)] ui/self/by-value-self-in-mut-slot.rs ... ok
2019-07-26T23:09:35.8008642Z test [ui (nll)] ui/self/elision/alias.rs ... ok
2019-07-26T23:09:35.9623002Z test [ui (nll)] ui/self/elision/assoc.rs ... ok
2019-07-26T23:09:36.1252243Z test [ui (nll)] ui/self/elision/lt-alias.rs ... ok
2019-07-26T23:09:36.2052782Z test [ui (nll)] ui/self/elision/lt-assoc.rs ... ok
2019-07-26T23:09:36.2065004Z test [ui (nll)] ui/self/builtin-superkinds-self-type.rs ... ok
2019-07-26T23:09:36.2827447Z test [ui (nll)] ui/self/elision/lt-self.rs ... ok
2019-07-26T23:09:36.2985406Z test [ui (nll)] ui/self/elision/lt-ref-self.rs ... FAILED
2019-07-26T23:09:36.3556206Z test [ui (nll)] ui/self/elision/lt-struct.rs ... ok
2019-07-26T23:09:36.3785929Z test [ui (nll)] ui/self/elision/multiple-ref-self.rs ... ok
2019-07-26T23:09:36.4316325Z test [ui (nll)] ui/self/elision/ref-alias.rs ... ok
2019-07-26T23:09:36.4481195Z test [ui (nll)] ui/self/elision/ref-assoc.rs ... ok
2019-07-26T23:09:36.5024702Z test [ui (nll)] ui/self/elision/ref-mut-alias.rs ... ok
2019-07-26T23:09:36.5340020Z test [ui (nll)] ui/self/elision/ref-mut-self.rs ... FAILED
2019-07-26T23:09:36.5814713Z test [ui (nll)] ui/self/elision/ref-mut-struct.rs ... FAILED
2019-07-26T23:09:36.6233684Z test [ui (nll)] ui/self/elision/ref-self.rs ... FAILED
2019-07-26T23:09:36.6693186Z test [ui (nll)] ui/self/elision/ref-struct.rs ... FAILED
2019-07-26T23:09:36.7013803Z test [ui (nll)] ui/self/elision/self.rs ... ok
2019-07-26T23:09:36.7382498Z test [ui (nll)] ui/self/elision/struct.rs ... ok
2019-07-26T23:09:36.9933848Z test [ui (nll)] ui/self/explicit-self-generic.rs ... ok
2019-07-26T23:09:37.0970860Z test [ui (nll)] ui/self/explicit-self-objects-uniq.rs ... ok
2019-07-26T23:09:37.2394328Z test [ui (nll)] ui/self/explicit-self.rs ... ok
2019-07-26T23:09:37.5404375Z test [ui (nll)] ui/self/move-self.rs ... ok
---
2019-07-26T23:10:17.4291271Z 
2019-07-26T23:10:17.4291957Z ---- [ui (nll)] ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs stdout ----
2019-07-26T23:10:17.4292046Z diff of stderr:
2019-07-26T23:10:17.4292085Z 
2019-07-26T23:10:17.4292697Z - error: cannot infer an appropriate lifetime
2019-07-26T23:10:17.4293001Z -   --> $DIR/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:44
2019-07-26T23:10:17.4293097Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4293346Z +   --> $DIR/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:31
2019-07-26T23:10:17.4293637Z 3    |
2019-07-26T23:10:17.4293909Z 4 LL |     fn f(self: Pin<&Self>) -> impl Clone { self }
2019-07-26T23:10:17.4294197Z -    |                               ----------   ^^^^ ...but this borrow...
2019-07-26T23:10:17.4294427Z -    |                               |
2019-07-26T23:10:17.4294714Z -    |                               this return type evaluates to the `'static` lifetime...
2019-07-26T23:10:17.4294932Z -    |
2019-07-26T23:10:17.4295191Z - note: ...can't outlive the anonymous lifetime #1 defined on the method body at 8:5
2019-07-26T23:10:17.4295458Z -   --> $DIR/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:5
2019-07-26T23:10:17.4295653Z -    |
2019-07-26T23:10:17.4296340Z - LL |     fn f(self: Pin<&Self>) -> impl Clone { self }
2019-07-26T23:10:17.4296610Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-26T23:10:17.4297488Z - help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 8:5
2019-07-26T23:10:17.4298133Z +    |                    -          ^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
2019-07-26T23:10:17.4298542Z +    |                    let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4298542Z +    |                    let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4298872Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-07-26T23:10:17.4298957Z 15    |
2019-07-26T23:10:17.4299342Z 16 LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self }
2019-07-26T23:10:17.4299477Z 
2019-07-26T23:10:17.4299509Z 
2019-07-26T23:10:17.4299583Z The actual stderr differed from the expected stderr.
2019-07-26T23:10:17.4299583Z The actual stderr differed from the expected stderr.
2019-07-26T23:10:17.4299960Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll/arbitrary_self_types_pin_lifetime_impl_trait.nll.stderr
2019-07-26T23:10:17.4300265Z To update references, rerun the tests and pass the `--bless` flag
2019-07-26T23:10:17.4300822Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait.rs`
2019-07-26T23:10:17.4300971Z error: 1 errors occurred comparing output.
2019-07-26T23:10:17.4301049Z status: exit code: 1
2019-07-26T23:10:17.4301049Z status: exit code: 1
2019-07-26T23:10:17.4302011Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll/auxiliary" "-A" "unused"
2019-07-26T23:10:17.4302546Z ------------------------------------------
2019-07-26T23:10:17.4302593Z 
2019-07-26T23:10:17.4302833Z ------------------------------------------
2019-07-26T23:10:17.4302899Z stderr:
2019-07-26T23:10:17.4302899Z stderr:
2019-07-26T23:10:17.4303129Z ------------------------------------------
2019-07-26T23:10:17.4303198Z error: lifetime may not live long enough
2019-07-26T23:10:17.4303487Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:31
2019-07-26T23:10:17.4303566Z    |
2019-07-26T23:10:17.4303867Z LL |     fn f(self: Pin<&Self>) -> impl Clone { self } //~ ERROR cannot infer an appropriate lifetime
2019-07-26T23:10:17.4304171Z    |                    -          ^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
2019-07-26T23:10:17.4304520Z    |                    let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4304520Z    |                    let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4304953Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-07-26T23:10:17.4305044Z    |
2019-07-26T23:10:17.4305348Z LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self } //~ ERROR cannot infer an appropriate lifetime
2019-07-26T23:10:17.4305494Z 
2019-07-26T23:10:17.4305550Z error: aborting due to previous error
2019-07-26T23:10:17.4305590Z 
2019-07-26T23:10:17.4305636Z 
2019-07-26T23:10:17.4305636Z 
2019-07-26T23:10:17.4306329Z ------------------------------------------
2019-07-26T23:10:17.4306399Z 
2019-07-26T23:10:17.4306432Z 
2019-07-26T23:10:17.4306710Z ---- [ui (nll)] ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs stdout ----
2019-07-26T23:10:17.4306806Z diff of stderr:
2019-07-26T23:10:17.4306845Z 
2019-07-26T23:10:17.4307084Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4307155Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4307446Z 2   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch.rs:8:46
2019-07-26T23:10:17.4307521Z 3    |
2019-07-26T23:10:17.4307790Z 4 LL |     fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-07-26T23:10:17.4307839Z 
2019-07-26T23:10:17.4308120Z -    |                              ----     ----   ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4308699Z -    |                              this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4308699Z -    |                              this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4309084Z +    |                    -         -               ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4309579Z +    |                    |         let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4309865Z +    |                    let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4309947Z 8 
2019-07-26T23:10:17.4310175Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4310175Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4310528Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch.rs:10:76
2019-07-26T23:10:17.4310628Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4310927Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch.rs:10:69
2019-07-26T23:10:17.4310999Z 11    |
2019-07-26T23:10:17.4311282Z 12 LL |     fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-07-26T23:10:17.4312169Z -    |                               ----              -----------------          ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4312743Z -    |                               this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4312743Z -    |                               this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4313153Z +    |                    -          -                                     ^^^^^^^^^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4313563Z +    |                    |          let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4313834Z +    |                    let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4313920Z 16 
2019-07-26T23:10:17.4314134Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4314134Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4314215Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4314468Z 18   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch.rs:15:58
2019-07-26T23:10:17.4314553Z 19    |
2019-07-26T23:10:17.4314803Z 20 LL |     fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-07-26T23:10:17.4314867Z 
2019-07-26T23:10:17.4315151Z -    |                                         ------     ---   ^^^ ...but data from `arg` is returned here
2019-07-26T23:10:17.4315731Z -    |                                         this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4315731Z -    |                                         this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4316773Z +    |            --  ---- has type `std::pin::Pin<&'1 Foo>`    ^^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'a`
2019-07-26T23:10:17.4317141Z +    |            lifetime `'a` defined here
2019-07-26T23:10:17.4317223Z 24 
2019-07-26T23:10:17.4317283Z 25 error: aborting due to 3 previous errors
2019-07-26T23:10:17.4317362Z 26 
2019-07-26T23:10:17.4317362Z 26 
2019-07-26T23:10:17.4317397Z 
2019-07-26T23:10:17.4317430Z 
2019-07-26T23:10:17.4317509Z The actual stderr differed from the expected stderr.
2019-07-26T23:10:17.4317909Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.nll/arbitrary_self_types_pin_lifetime_mismatch.nll.stderr
2019-07-26T23:10:17.4318225Z To update references, rerun the tests and pass the `--bless` flag
2019-07-26T23:10:17.4318552Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch.rs`
2019-07-26T23:10:17.4318697Z error: 1 errors occurred comparing output.
2019-07-26T23:10:17.4318777Z status: exit code: 1
2019-07-26T23:10:17.4318777Z status: exit code: 1
2019-07-26T23:10:17.4319819Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.nll/auxiliary" "-A" "unused"
2019-07-26T23:10:17.4320338Z ------------------------------------------
2019-07-26T23:10:17.4320384Z 
2019-07-26T23:10:17.4320622Z ------------------------------------------
2019-07-26T23:10:17.4320779Z stderr:
2019-07-26T23:10:17.4320779Z stderr:
2019-07-26T23:10:17.4321166Z ------------------------------------------
2019-07-26T23:10:17.4321233Z error: lifetime may not live long enough
2019-07-26T23:10:17.4321514Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs:8:46
2019-07-26T23:10:17.4321591Z    |
2019-07-26T23:10:17.4321852Z LL |     fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f } //~ ERROR E0623
2019-07-26T23:10:17.4322194Z    |                    -         -               ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4322562Z    |                    |         let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4322835Z    |                    let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4322894Z 
2019-07-26T23:10:17.4322963Z error: lifetime may not live long enough
2019-07-26T23:10:17.4322963Z error: lifetime may not live long enough
2019-07-26T23:10:17.4323238Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs:10:69
2019-07-26T23:10:17.4323324Z    |
2019-07-26T23:10:17.4323600Z LL |     fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) } //~ ERROR E0623
2019-07-26T23:10:17.4323997Z    |                    -          -                                     ^^^^^^^^^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4324367Z    |                    |          let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4324626Z    |                    let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4324689Z 
2019-07-26T23:10:17.4324743Z error: lifetime may not live long enough
2019-07-26T23:10:17.4324743Z error: lifetime may not live long enough
2019-07-26T23:10:17.4325020Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs:15:58
2019-07-26T23:10:17.4325172Z    |
2019-07-26T23:10:17.4325473Z LL |     fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-07-26T23:10:17.4326383Z    |            --  ---- has type `std::pin::Pin<&'1 Foo>`    ^^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'a`
2019-07-26T23:10:17.4326808Z    |            lifetime `'a` defined here
2019-07-26T23:10:17.4326871Z 
2019-07-26T23:10:17.4326930Z error: aborting due to 3 previous errors
2019-07-26T23:10:17.4326986Z 
2019-07-26T23:10:17.4326986Z 
2019-07-26T23:10:17.4327020Z 
2019-07-26T23:10:17.4327247Z ------------------------------------------
2019-07-26T23:10:17.4327305Z 
2019-07-26T23:10:17.4327339Z 
2019-07-26T23:10:17.4327584Z ---- [ui (nll)] ui/self/elision/lt-ref-self.rs stdout ----
2019-07-26T23:10:17.4327672Z diff of stderr:
2019-07-26T23:10:17.4327711Z 
2019-07-26T23:10:17.4327966Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4328036Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4328283Z 2   --> $DIR/lt-ref-self.rs:12:9
2019-07-26T23:10:17.4328359Z 3    |
2019-07-26T23:10:17.4328616Z 4 LL |     fn ref_self(&self, f: &u32) -> &u32 {
2019-07-26T23:10:17.4328896Z -    |                           ----     ----
2019-07-26T23:10:17.4329264Z -    |                           |
2019-07-26T23:10:17.4329560Z -    |                           this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4329839Z +    |                 -         - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4329839Z +    |                 -         - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4329927Z +    |                 |
2019-07-26T23:10:17.4330172Z +    |                 let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4330258Z 8 LL |         f
2019-07-26T23:10:17.4330487Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4330799Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4331212Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4331212Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4331285Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4331544Z 12   --> $DIR/lt-ref-self.rs:18:9
2019-07-26T23:10:17.4331606Z 13    |
2019-07-26T23:10:17.4331852Z 14 LL |     fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-07-26T23:10:17.4332135Z -    |                                 ----     ----
2019-07-26T23:10:17.4332356Z -    |                                 |
2019-07-26T23:10:17.4332660Z -    |                                 this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4332945Z +    |                       -         - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4332945Z +    |                       -         - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4333035Z +    |                       |
2019-07-26T23:10:17.4333285Z +    |                       let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4333379Z 18 LL |         f
2019-07-26T23:10:17.4333617Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4333933Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4334233Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4334233Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4334297Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4334524Z 22   --> $DIR/lt-ref-self.rs:22:9
2019-07-26T23:10:17.4334585Z 23    |
2019-07-26T23:10:17.4334837Z 24 LL |     fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4335128Z -    |                                          ----     ----
2019-07-26T23:10:17.4335361Z -    |                                          |
2019-07-26T23:10:17.4335677Z -    |                                          this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4336597Z +    |                               -          - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4336597Z +    |                               -          - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4336713Z +    |                               |
2019-07-26T23:10:17.4337001Z +    |                               let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4337094Z 28 LL |         f
2019-07-26T23:10:17.4337338Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4337676Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4338002Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4338002Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4338071Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4338313Z 32   --> $DIR/lt-ref-self.rs:26:9
2019-07-26T23:10:17.4338378Z 33    |
2019-07-26T23:10:17.4338647Z 34 LL |     fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4338962Z -    |                                          ----     ----
2019-07-26T23:10:17.4339339Z -    |                                          |
2019-07-26T23:10:17.4339665Z -    |                                          this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4339959Z +    |                               -          - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4339959Z +    |                               -          - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4340054Z +    |                               |
2019-07-26T23:10:17.4340314Z +    |                               let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4340402Z 38 LL |         f
2019-07-26T23:10:17.4340630Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4340945Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4341243Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4341243Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4341321Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4341543Z 42   --> $DIR/lt-ref-self.rs:30:9
2019-07-26T23:10:17.4341616Z 43    |
2019-07-26T23:10:17.4341969Z 44 LL |     fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4342324Z -    |                                                   ----     ----
2019-07-26T23:10:17.4342588Z -    |                                                   |
2019-07-26T23:10:17.4342903Z -    |                                                   this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4343226Z +    |                                       -           - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4343226Z +    |                                       -           - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4343311Z +    |                                       |
2019-07-26T23:10:17.4343597Z +    |                                       let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4343671Z 48 LL |         f
2019-07-26T23:10:17.4343917Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4344251Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4344559Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4344559Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4344623Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4344853Z 52   --> $DIR/lt-ref-self.rs:34:9
2019-07-26T23:10:17.4344915Z 53    |
2019-07-26T23:10:17.4345171Z 54 LL |     fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4346866Z -    |                                               ----     ----
2019-07-26T23:10:17.4347531Z -    |                                               |
2019-07-26T23:10:17.4347875Z -    |                                               this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4348226Z +    |                                   -           - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4348226Z +    |                                   -           - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4348478Z +    |                                   |
2019-07-26T23:10:17.4348806Z +    |                                   let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4348904Z 58 LL |         f
2019-07-26T23:10:17.4349259Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4349586Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4349743Z 61 error: aborting due to 6 previous errors
2019-07-26T23:10:17.4349804Z 62 
2019-07-26T23:10:17.4349839Z 
2019-07-26T23:10:17.4349889Z 
2019-07-26T23:10:17.4349889Z 
2019-07-26T23:10:17.4349950Z The actual stderr differed from the expected stderr.
2019-07-26T23:10:17.4350293Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self.nll/lt-ref-self.nll.stderr
2019-07-26T23:10:17.4350814Z To update references, rerun the tests and pass the `--bless` flag
2019-07-26T23:10:17.4351122Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self.rs`
2019-07-26T23:10:17.4351257Z error: 1 errors occurred comparing output.
2019-07-26T23:10:17.4351322Z status: exit code: 1
2019-07-26T23:10:17.4351322Z status: exit code: 1
2019-07-26T23:10:17.4356003Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self.nll/auxiliary" "-A" "unused"
2019-07-26T23:10:17.4357267Z ------------------------------------------
2019-07-26T23:10:17.4357349Z 
2019-07-26T23:10:17.4357586Z ------------------------------------------
2019-07-26T23:10:17.4357670Z stderr:
2019-07-26T23:10:17.4357670Z stderr:
2019-07-26T23:10:17.4358026Z ------------------------------------------
2019-07-26T23:10:17.4358126Z error: lifetime may not live long enough
2019-07-26T23:10:17.4358426Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:12:9
2019-07-26T23:10:17.4358518Z    |
2019-07-26T23:10:17.4358761Z LL |     fn ref_self(&self, f: &u32) -> &u32 {
2019-07-26T23:10:17.4359136Z    |                 |
2019-07-26T23:10:17.4359415Z    |                 let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4359617Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4359617Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4359929Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4360065Z error: lifetime may not live long enough
2019-07-26T23:10:17.4360320Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:18:9
2019-07-26T23:10:17.4360402Z    |
2019-07-26T23:10:17.4360402Z    |
2019-07-26T23:10:17.4360638Z LL |     fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-07-26T23:10:17.4361002Z    |                       |
2019-07-26T23:10:17.4361270Z    |                       let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4361342Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4361342Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4361649Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4361778Z error: lifetime may not live long enough
2019-07-26T23:10:17.4362020Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:22:9
2019-07-26T23:10:17.4362101Z    |
2019-07-26T23:10:17.4362101Z    |
2019-07-26T23:10:17.4362334Z LL |     fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4362828Z    |                               |
2019-07-26T23:10:17.4363106Z    |                               let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4363181Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4363181Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4363490Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4363618Z error: lifetime may not live long enough
2019-07-26T23:10:17.4363859Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:26:9
2019-07-26T23:10:17.4363940Z    |
2019-07-26T23:10:17.4363940Z    |
2019-07-26T23:10:17.4364172Z LL |     fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4364540Z    |                               |
2019-07-26T23:10:17.4364822Z    |                               let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4364904Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4364904Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4365263Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4365395Z error: lifetime may not live long enough
2019-07-26T23:10:17.4366181Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:30:9
2019-07-26T23:10:17.4366286Z    |
2019-07-26T23:10:17.4366286Z    |
2019-07-26T23:10:17.4366597Z LL |     fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4367014Z    |                                       |
2019-07-26T23:10:17.4367318Z    |                                       let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4367398Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4367398Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4367745Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4368017Z error: lifetime may not live long enough
2019-07-26T23:10:17.4368308Z   --> /checkout/src/test/ui/self/elision/lt-ref-self.rs:34:9
2019-07-26T23:10:17.4368396Z    |
2019-07-26T23:10:17.4368396Z    |
2019-07-26T23:10:17.4368652Z LL |     fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4369060Z    |                                   |
2019-07-26T23:10:17.4369581Z    |                                   let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4369664Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4369664Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4370101Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4370246Z error: aborting due to 6 previous errors
2019-07-26T23:10:17.4370287Z 
2019-07-26T23:10:17.4370318Z 
2019-07-26T23:10:17.4370566Z ------------------------------------------
2019-07-26T23:10:17.4370566Z ------------------------------------------
2019-07-26T23:10:17.4370609Z 
2019-07-26T23:10:17.4370640Z 
2019-07-26T23:10:17.4370893Z ---- [ui (nll)] ui/self/elision/ref-mut-self.rs stdout ----
2019-07-26T23:10:17.4370965Z diff of stderr:
2019-07-26T23:10:17.4371018Z 
2019-07-26T23:10:17.4371232Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4371315Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4371615Z 3    |
2019-07-26T23:10:17.4371615Z 3    |
2019-07-26T23:10:17.4371848Z 4 LL |     fn ref_self(&mut self, f: &u32) -> &u32 {
2019-07-26T23:10:17.4372137Z -    |                               ----     ----
2019-07-26T23:10:17.4372378Z -    |                               |
2019-07-26T23:10:17.4372793Z -    |                               this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4373197Z +    |                 -             - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4373197Z +    |                 -             - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4373282Z +    |                 |
2019-07-26T23:10:17.4373548Z +    |                 let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4373619Z 8 LL |         f
2019-07-26T23:10:17.4373862Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4374164Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4374470Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4374470Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4374550Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4374841Z 13    |
2019-07-26T23:10:17.4374841Z 13    |
2019-07-26T23:10:17.4375075Z 14 LL |     fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-07-26T23:10:17.4375362Z -    |                                     ----     ----
2019-07-26T23:10:17.4375613Z -    |                                     |
2019-07-26T23:10:17.4376312Z -    |                                     this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4376654Z +    |                       -             - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4376654Z +    |                       -             - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4376737Z +    |                       |
2019-07-26T23:10:17.4378674Z +    |                       let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4378769Z 18 LL |         f
2019-07-26T23:10:17.4379192Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4379497Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4379802Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4379802Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4379883Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4380192Z 23    |
2019-07-26T23:10:17.4380192Z 23    |
2019-07-26T23:10:17.4380557Z 24 LL |     fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4380908Z -    |                                              ----     ----
2019-07-26T23:10:17.4381164Z -    |                                              |
2019-07-26T23:10:17.4381476Z -    |                                              this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4381794Z +    |                               -              - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4381794Z +    |                               -              - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4381873Z +    |                               |
2019-07-26T23:10:17.4382150Z +    |                               let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4382224Z 28 LL |         f
2019-07-26T23:10:17.4382469Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4382766Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4383106Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4383106Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4383172Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4383468Z 33    |
2019-07-26T23:10:17.4383468Z 33    |
2019-07-26T23:10:17.4383725Z 34 LL |     fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4384010Z -    |                                              ----     ----
2019-07-26T23:10:17.4387513Z -    |                                              |
2019-07-26T23:10:17.4388264Z -    |                                              this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4388624Z +    |                               -              - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4388624Z +    |                               -              - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4388724Z +    |                               |
2019-07-26T23:10:17.4389169Z +    |                               let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4389266Z 38 LL |         f
2019-07-26T23:10:17.4389522Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4389863Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4390189Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4390189Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4390258Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4390575Z 43    |
2019-07-26T23:10:17.4390575Z 43    |
2019-07-26T23:10:17.4390859Z 44 LL |     fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4391637Z -    |                                                       ----     ----
2019-07-26T23:10:17.4391945Z -    |                                                       |
2019-07-26T23:10:17.4392795Z -    |                                                       this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4393315Z +    |                                       -               - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4393315Z +    |                                       -               - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4393430Z +    |                                       |
2019-07-26T23:10:17.4393744Z +    |                                       let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4393835Z 48 LL |         f
2019-07-26T23:10:17.4394262Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4394598Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4395106Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4395106Z - error[E0623]: lifetime mismatch
2019-07-26T23:10:17.4395171Z + error: lifetime may not live long enough
2019-07-26T23:10:17.4395605Z 53    |
2019-07-26T23:10:17.4395605Z 53    |
2019-07-26T23:10:17.4396471Z 54 LL |     fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4419811Z -    |                                                       ----     ----
2019-07-26T23:10:17.4420157Z -    |                                                       |
2019-07-26T23:10:17.4420494Z -    |                                                       this parameter and the return type are declared with different lifetimes...
2019-07-26T23:10:17.4420822Z +    |                                       -               - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4420822Z +    |                                       -               - let's call the lifetime of this reference `'1`
2019-07-26T23:10:17.4420910Z +    |                                       |
2019-07-26T23:10:17.4421307Z +    |                                       let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4421385Z 58 LL |         f
2019-07-26T23:10:17.4421671Z -    |         ^ ...but data from `f` is returned here
2019-07-26T23:10:17.4421987Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4422161Z 61 error: aborting due to 6 previous errors
2019-07-26T23:10:17.4422221Z 62 
2019-07-26T23:10:17.4422254Z 
2019-07-26T23:10:17.4422292Z 
2019-07-26T23:10:17.4422292Z 
2019-07-26T23:10:17.4422351Z The actual stderr differed from the expected stderr.
2019-07-26T23:10:17.4422687Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self.nll/ref-mut-self.nll.stderr
2019-07-26T23:10:17.4422963Z To update references, rerun the tests and pass the `--bless` flag
2019-07-26T23:10:17.4423247Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self.rs`
2019-07-26T23:10:17.4423365Z error: 1 errors occurred comparing output.
2019-07-26T23:10:17.4423428Z status: exit code: 1
2019-07-26T23:10:17.4423428Z status: exit code: 1
2019-07-26T23:10:17.4424262Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self.nll/auxiliary" "-A" "unused"
2019-07-26T23:10:17.4424836Z ------------------------------------------
2019-07-26T23:10:17.4424881Z 
2019-07-26T23:10:17.4425113Z ------------------------------------------
2019-07-26T23:10:17.4425176Z stderr:
2019-07-26T23:10:17.4425176Z stderr:
2019-07-26T23:10:17.4425401Z ------------------------------------------
2019-07-26T23:10:17.4425466Z error: lifetime may not live long enough
2019-07-26T23:10:17.4425722Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:12:9
2019-07-26T23:10:17.4426171Z    |
2019-07-26T23:10:17.4426517Z LL |     fn ref_self(&mut self, f: &u32) -> &u32 {
2019-07-26T23:10:17.4426946Z    |                 |
2019-07-26T23:10:17.4427252Z    |                 let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4427334Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4427334Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4427691Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4427834Z error: lifetime may not live long enough
2019-07-26T23:10:17.4428128Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:18:9
2019-07-26T23:10:17.4428212Z    |
2019-07-26T23:10:17.4428212Z    |
2019-07-26T23:10:17.4428488Z LL |     fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-07-26T23:10:17.4428902Z    |                       |
2019-07-26T23:10:17.4429326Z    |                       let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4429507Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4429507Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4429858Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4429984Z error: lifetime may not live long enough
2019-07-26T23:10:17.4430232Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:22:9
2019-07-26T23:10:17.4430307Z    |
2019-07-26T23:10:17.4430307Z    |
2019-07-26T23:10:17.4430548Z LL |     fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4430918Z    |                               |
2019-07-26T23:10:17.4431186Z    |                               let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4431262Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4431262Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4431718Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4431856Z error: lifetime may not live long enough
2019-07-26T23:10:17.4432106Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:26:9
2019-07-26T23:10:17.4432182Z    |
2019-07-26T23:10:17.4432182Z    |
2019-07-26T23:10:17.4432422Z LL |     fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4432791Z    |                               |
2019-07-26T23:10:17.4433060Z    |                               let's call the lifetime of this reference `'2`
2019-07-26T23:10:17.4433135Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4433135Z LL |         f //~ ERROR lifetime mismatch
2019-07-26T23:10:17.4433442Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2019-07-26T23:10:17.4433665Z error: lifetime may not live long enough
2019-07-26T23:10:17.4433935Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:30:9
2019-07-26T23:10:17.4434022Z    |
2019-07-26T23:10:17.4434022Z    |
2019-07-26T23:10:17.4434277Z LL |     fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-07-26T23:10:17.4434667Z    |                                       |
---
2019-07-26T23:10:17.4550477Z test result: FAILED. 5822 passed; 7 failed; 61 ignored; 0 measured; 0 filtered out
2019-07-26T23:10:17.4550551Z 
2019-07-26T23:10:17.4550584Z 
2019-07-26T23:10:17.4550614Z 
2019-07-26T23:10:17.4552828Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-07-26T23:10:17.4553439Z 
2019-07-26T23:10:17.4553475Z 
2019-07-26T23:10:17.4553904Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-26T23:10:17.4554000Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-26T23:10:17.4554000Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-26T23:10:17.4554098Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-26T23:10:17.4554171Z Build completed unsuccessfully in 1:40:25
2019-07-26T23:10:18.5628403Z ##[error]Bash exited with code '1'.
2019-07-26T23:10:18.5668451Z ##[section]Starting: Upload CPU usage statistics
2019-07-26T23:10:18.5679400Z ==============================================================================
2019-07-26T23:10:18.5679511Z Task         : Bash
2019-07-26T23:10:18.5679581Z Description  : Run a Bash script on macOS, Linux, or Windows
