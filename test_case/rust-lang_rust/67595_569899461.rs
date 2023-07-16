plain
2019-12-31T08:36:01.2387712Z 
2019-12-31T08:36:01.2387979Z 6    |              |
2019-12-31T08:36:01.2388707Z 7    |              let's call the lifetime of this reference `'1`
2019-12-31T08:36:01.2389061Z 8    |
2019-12-31T08:36:01.2395759Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-12-31T08:36:01.2396353Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-31T08:36:01.2396470Z 10    |
2019-12-31T08:36:01.2396733Z 11 LL | fn elided(x: &i32) -> impl Copy + '_ { x }
2019-12-31T08:36:01.2396876Z 
2019-12-31T08:36:01.2397137Z 20    |             lifetime `'a` defined here
2019-12-31T08:36:01.2397210Z 21    |
2019-12-31T08:36:01.2397475Z 22    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2397475Z 22    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2397797Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a constraint
2019-12-31T08:36:01.2401779Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a bound
2019-12-31T08:36:01.2401892Z 24    |
2019-12-31T08:36:01.2402199Z 25 LL | fn explicit<'a>(x: &'a i32) -> impl Copy + 'a { x }
2019-12-31T08:36:01.2402354Z 
2019-12-31T08:36:01.2402389Z 
2019-12-31T08:36:01.2402473Z The actual stderr differed from the expected stderr.
2019-12-31T08:36:01.2402473Z The actual stderr differed from the expected stderr.
2019-12-31T08:36:01.2402886Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound.nll/must_outlive_least_region_or_bound.nll.stderr
2019-12-31T08:36:01.2403223Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T08:36:01.2406275Z To only update this specific test, also pass `--test-args impl-trait/must_outlive_least_region_or_bound.rs`
2019-12-31T08:36:01.2406437Z error: 1 errors occurred comparing output.
2019-12-31T08:36:01.2406541Z status: exit code: 1
2019-12-31T08:36:01.2406541Z status: exit code: 1
2019-12-31T08:36:01.2407805Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound.nll/auxiliary" "-A" "unused"
2019-12-31T08:36:01.2408418Z ------------------------------------------
2019-12-31T08:36:01.2408488Z 
2019-12-31T08:36:01.2408755Z ------------------------------------------
2019-12-31T08:36:01.2409132Z stderr:
2019-12-31T08:36:01.2409132Z stderr:
2019-12-31T08:36:01.2409467Z ------------------------------------------
2019-12-31T08:36:01.2409563Z error: lifetime may not live long enough
2019-12-31T08:36:01.2409886Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:3:23
2019-12-31T08:36:01.2409993Z    |
2019-12-31T08:36:01.2410258Z LL | fn elided(x: &i32) -> impl Copy { x }
2019-12-31T08:36:01.2410596Z    |              -        ^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
2019-12-31T08:36:01.2418370Z    |              let's call the lifetime of this reference `'1`
2019-12-31T08:36:01.2418477Z    |
2019-12-31T08:36:01.2418477Z    |
2019-12-31T08:36:01.2418796Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-31T08:36:01.2418902Z    |
2019-12-31T08:36:01.2419155Z LL | fn elided(x: &i32) -> impl Copy + '_ { x }
2019-12-31T08:36:01.2419313Z 
2019-12-31T08:36:01.2419551Z error: lifetime may not live long enough
2019-12-31T08:36:01.2419903Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:6:32
2019-12-31T08:36:01.2420004Z    |
2019-12-31T08:36:01.2420004Z    |
2019-12-31T08:36:01.2420254Z LL | fn explicit<'a>(x: &'a i32) -> impl Copy { x }
2019-12-31T08:36:01.2420582Z    |             --                 ^^^^^^^^^ opaque type requires that `'a` must outlive `'static`
2019-12-31T08:36:01.2420931Z    |             lifetime `'a` defined here
2019-12-31T08:36:01.2421002Z    |
2019-12-31T08:36:01.2421261Z    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2421261Z    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2421586Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a bound
2019-12-31T08:36:01.2421672Z    |
2019-12-31T08:36:01.2421942Z LL | fn explicit<'a>(x: &'a i32) -> impl Copy + 'a { x }
2019-12-31T08:36:01.2422567Z 
2019-12-31T08:36:01.2422682Z error: lifetime may not live long enough
2019-12-31T08:36:01.2423312Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:12:69
2019-12-31T08:36:01.2423415Z    |
2019-12-31T08:36:01.2423415Z    |
2019-12-31T08:36:01.2424112Z LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }
2019-12-31T08:36:01.2424548Z    |               -- lifetime `'a` defined here                         ^ returning this value requires that `'a` must outlive `'static`
2019-12-31T08:36:01.2425026Z    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2425368Z    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2425428Z 
2019-12-31T08:36:01.2425519Z error: lifetime may not live long enough
2019-12-31T08:36:01.2425519Z error: lifetime may not live long enough
2019-12-31T08:36:01.2425879Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:17:61
2019-12-31T08:36:01.2425996Z    |
2019-12-31T08:36:01.2426340Z LL | fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) {
2019-12-31T08:36:01.2426823Z    |                          --  -- lifetime `'b` defined here  ^^^^^^^^^^^^^^^^ opaque type requires that `'b` must outlive `'a`
2019-12-31T08:36:01.2427283Z    |                          lifetime `'a` defined here
2019-12-31T08:36:01.2427388Z    |
2019-12-31T08:36:01.2427691Z    = help: consider adding the following bound: `'b: 'a`
2019-12-31T08:36:01.2427770Z 
2019-12-31T08:36:01.2427770Z 
2019-12-31T08:36:01.2427852Z error[E0310]: the parameter type `T` may not live long enough
2019-12-31T08:36:01.2428228Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:22:51
2019-12-31T08:36:01.2428327Z    |
2019-12-31T08:36:01.2428674Z LL | fn ty_param_wont_outlive_static<T:Debug>(x: T) -> impl Debug + 'static {
2019-12-31T08:36:01.2428899Z    |
2019-12-31T08:36:01.2429233Z    = help: consider adding an explicit lifetime bound `T: 'static`...
2019-12-31T08:36:01.2429326Z 
2019-12-31T08:36:01.2429400Z error: aborting due to 5 previous errors
---
2019-12-31T08:36:01.2430756Z 
2019-12-31T08:36:01.2430822Z 6    |                         |
2019-12-31T08:36:01.2431184Z 7    |                         let's call the lifetime of this reference `'1`
2019-12-31T08:36:01.2431296Z 8    |
2019-12-31T08:36:01.2431665Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-12-31T08:36:01.2432074Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-31T08:36:01.2432329Z 10    |
2019-12-31T08:36:01.2432724Z 11 LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> + '_ {
2019-12-31T08:36:01.2432913Z 
2019-12-31T08:36:01.2433221Z 20    |                    lifetime `'a` defined here
2019-12-31T08:36:01.2433324Z 21    |
2019-12-31T08:36:01.2433623Z 22    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2433623Z 22    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2434022Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a constraint
2019-12-31T08:36:01.2434409Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a bound
2019-12-31T08:36:01.2434530Z 24    |
2019-12-31T08:36:01.2434873Z 25 LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {
2019-12-31T08:36:01.2435166Z 
2019-12-31T08:36:01.2435237Z 
2019-12-31T08:36:01.2435316Z The actual stderr differed from the expected stderr.
2019-12-31T08:36:01.2435316Z The actual stderr differed from the expected stderr.
2019-12-31T08:36:01.2435828Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll/static-return-lifetime-infered.nll.stderr
2019-12-31T08:36:01.2436197Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T08:36:01.2436596Z To only update this specific test, also pass `--test-args impl-trait/static-return-lifetime-infered.rs`
2019-12-31T08:36:01.2436763Z error: 1 errors occurred comparing output.
2019-12-31T08:36:01.2436845Z status: exit code: 1
2019-12-31T08:36:01.2436845Z status: exit code: 1
2019-12-31T08:36:01.2438129Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll/auxiliary" "-A" "unused"
2019-12-31T08:36:01.2438798Z ------------------------------------------
2019-12-31T08:36:01.2438857Z 
2019-12-31T08:36:01.2439165Z ------------------------------------------
2019-12-31T08:36:01.2439249Z stderr:
2019-12-31T08:36:01.2439249Z stderr:
2019-12-31T08:36:01.2439545Z ------------------------------------------
2019-12-31T08:36:01.2439632Z error: lifetime may not live long enough
2019-12-31T08:36:01.2439993Z   --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:6:35
2019-12-31T08:36:01.2440090Z    |
2019-12-31T08:36:01.2440439Z LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> {
2019-12-31T08:36:01.2440848Z    |                         -         ^^^^^^^^^^^^^^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
2019-12-31T08:36:01.2441341Z    |                         let's call the lifetime of this reference `'1`
2019-12-31T08:36:01.2441438Z    |
2019-12-31T08:36:01.2441438Z    |
2019-12-31T08:36:01.2441811Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-31T08:36:01.2441913Z    |
2019-12-31T08:36:01.2442254Z LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> + '_ {
2019-12-31T08:36:01.2442440Z 
2019-12-31T08:36:01.2442512Z error: lifetime may not live long enough
2019-12-31T08:36:01.2442870Z   --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:10:37
2019-12-31T08:36:01.2442976Z    |
2019-12-31T08:36:01.2442976Z    |
2019-12-31T08:36:01.2443411Z LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> {
2019-12-31T08:36:01.2443850Z    |                    --               ^^^^^^^^^^^^^^^^^^^^^^^ opaque type requires that `'a` must outlive `'static`
2019-12-31T08:36:01.2444301Z    |                    lifetime `'a` defined here
2019-12-31T08:36:01.2444386Z    |
2019-12-31T08:36:01.2444699Z    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2444699Z    = help: consider replacing `'a` with `'static`
2019-12-31T08:36:01.2445072Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a bound
2019-12-31T08:36:01.2445191Z    |
2019-12-31T08:36:01.2445514Z LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {
2019-12-31T08:36:01.2445700Z 
2019-12-31T08:36:01.2445787Z error: aborting due to 2 previous errors
2019-12-31T08:36:01.2445839Z 
2019-12-31T08:36:01.2445879Z 
---
2019-12-31T08:36:01.2446940Z 
2019-12-31T08:36:01.2447010Z 6    |                          |
2019-12-31T08:36:01.2447370Z 7    |                          let's call the lifetime of this reference `'1`
2019-12-31T08:36:01.2447465Z 8    |
2019-12-31T08:36:01.2447855Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-12-31T08:36:01.2448264Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-31T08:36:01.2448368Z 10    |
2019-12-31T08:36:01.2448706Z 11 LL |     async fn f(self: Pin<&Self>) -> impl Clone + '_ { self }
2019-12-31T08:36:01.2448897Z 
2019-12-31T08:36:01.2448938Z 
2019-12-31T08:36:01.2449026Z The actual stderr differed from the expected stderr.
2019-12-31T08:36:01.2449026Z The actual stderr differed from the expected stderr.
2019-12-31T08:36:01.2449566Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll/arbitrary_self_types_pin_lifetime_impl_trait-async.nll.stderr
2019-12-31T08:36:01.2449968Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T08:36:01.2450381Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs`
2019-12-31T08:36:01.2450559Z error: 1 errors occurred comparing output.
2019-12-31T08:36:01.2450657Z status: exit code: 1
2019-12-31T08:36:01.2450657Z status: exit code: 1
2019-12-31T08:36:01.2452011Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll/auxiliary" "-A" "unused"
2019-12-31T08:36:01.2452708Z ------------------------------------------
2019-12-31T08:36:01.2452767Z 
2019-12-31T08:36:01.2453071Z ------------------------------------------
2019-12-31T08:36:01.2453154Z stderr:
2019-12-31T08:36:01.2453154Z stderr:
2019-12-31T08:36:01.2453450Z ------------------------------------------
2019-12-31T08:36:01.2453553Z error: lifetime may not live long enough
2019-12-31T08:36:01.2453922Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs:8:48
2019-12-31T08:36:01.2454054Z    |
2019-12-31T08:36:01.2454466Z LL |     async fn f(self: Pin<&Self>) -> impl Clone { self }
2019-12-31T08:36:01.2454940Z    |                          -                     ^^^^^^^^ returning this value requires that `'1` must outlive `'static`
2019-12-31T08:36:01.2455427Z    |                          let's call the lifetime of this reference `'1`
2019-12-31T08:36:01.2455538Z    |
2019-12-31T08:36:01.2455538Z    |
2019-12-31T08:36:01.2455895Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-31T08:36:01.2456013Z    |
2019-12-31T08:36:01.2456322Z LL |     async fn f(self: Pin<&Self>) -> impl Clone + '_ { self }
2019-12-31T08:36:01.2456500Z 
2019-12-31T08:36:01.2456585Z error: aborting due to previous error
2019-12-31T08:36:01.2456637Z 
2019-12-31T08:36:01.2456676Z 
---
2019-12-31T08:36:01.2457725Z 
2019-12-31T08:36:01.2457793Z 6    |                    |
2019-12-31T08:36:01.2458145Z 7    |                    let's call the lifetime of this reference `'1`
2019-12-31T08:36:01.2458238Z 8    |
2019-12-31T08:36:01.2458619Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-12-31T08:36:01.2459007Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-31T08:36:01.2459125Z 10    |
2019-12-31T08:36:01.2459453Z 11 LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self }
2019-12-31T08:36:01.2459612Z 
2019-12-31T08:36:01.2459680Z 
2019-12-31T08:36:01.2459758Z The actual stderr differed from the expected stderr.
2019-12-31T08:36:01.2459758Z The actual stderr differed from the expected stderr.
2019-12-31T08:36:01.2460293Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll/arbitrary_self_types_pin_lifetime_impl_trait.nll.stderr
2019-12-31T08:36:01.2460695Z To update references, rerun the tests and pass the `--bless` flag
2019-12-31T08:36:01.2461097Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait.rs`
2019-12-31T08:36:01.2461274Z error: 1 errors occurred comparing output.
2019-12-31T08:36:01.2461373Z status: exit code: 1
2019-12-31T08:36:01.2461373Z status: exit code: 1
2019-12-31T08:36:01.2462671Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll/auxiliary" "-A" "unused"
2019-12-31T08:36:01.2463355Z ------------------------------------------
2019-12-31T08:36:01.2463413Z 
2019-12-31T08:36:01.2463714Z ------------------------------------------
2019-12-31T08:36:01.2463796Z stderr:
2019-12-31T08:36:01.2463796Z stderr:
2019-12-31T08:36:01.2464093Z ------------------------------------------
2019-12-31T08:36:01.2464178Z error: lifetime may not live long enough
2019-12-31T08:36:01.2464549Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:31
2019-12-31T08:36:01.2464666Z    |
2019-12-31T08:36:01.2465031Z LL |     fn f(self: Pin<&Self>) -> impl Clone { self } //~ ERROR cannot infer an appropriate lifetime
2019-12-31T08:36:01.2465543Z    |                    -          ^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
2019-12-31T08:36:01.2466030Z    |                    let's call the lifetime of this reference `'1`
2019-12-31T08:36:01.2466122Z    |
2019-12-31T08:36:01.2466122Z    |
2019-12-31T08:36:01.2466497Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-31T08:36:01.2466613Z    |
2019-12-31T08:36:01.2466982Z LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self } //~ ERROR cannot infer an appropriate lifetime
2019-12-31T08:36:01.2467171Z 
2019-12-31T08:36:01.2467240Z error: aborting due to previous error
2019-12-31T08:36:01.2467311Z 
2019-12-31T08:36:01.2467351Z 
---
2019-12-31T08:36:01.2470098Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-31T08:36:01.2470232Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-31T08:36:01.2470300Z 
2019-12-31T08:36:01.2470341Z 
2019-12-31T08:36:01.2472963Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-12-31T08:36:01.2475264Z 
2019-12-31T08:36:01.2475333Z 
2019-12-31T08:36:01.2491369Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-31T08:36:01.2491500Z Build completed unsuccessfully in 1:45:47
2019-12-31T08:36:01.2491500Z Build completed unsuccessfully in 1:45:47
2019-12-31T08:36:01.2543780Z == clock drift check ==
2019-12-31T08:36:01.2556451Z   local time: Tue Dec 31 08:36:01 UTC 2019
2019-12-31T08:36:01.5241612Z   network time: Tue, 31 Dec 2019 08:36:01 GMT
2019-12-31T08:36:01.5245665Z == end clock drift check ==
2019-12-31T08:36:02.6271752Z 
2019-12-31T08:36:02.6392981Z ##[error]Bash exited with code '1'.
2019-12-31T08:36:02.6439814Z ##[section]Starting: Checkout
2019-12-31T08:36:02.6442035Z ==============================================================================
2019-12-31T08:36:02.6458188Z Task         : Get sources
2019-12-31T08:36:02.6458300Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
