plain
2019-12-26T22:19:06.1410527Z 
2019-12-26T22:19:06.1411152Z 6    |              |
2019-12-26T22:19:06.1412071Z 7    |              let's call the lifetime of this reference `'1`
2019-12-26T22:19:06.1412489Z 8    |
2019-12-26T22:19:06.1413130Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-12-26T22:19:06.1413872Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-26T22:19:06.1414318Z 10    |
2019-12-26T22:19:06.1414778Z 11 LL | fn elided(x: &i32) -> impl Copy + '_ { x }
2019-12-26T22:19:06.1415170Z 
2019-12-26T22:19:06.1415577Z 20    |             lifetime `'a` defined here
2019-12-26T22:19:06.1415784Z 21    |
2019-12-26T22:19:06.1416217Z 22    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1416217Z 22    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1416941Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a constraint
2019-12-26T22:19:06.1417743Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a bound
2019-12-26T22:19:06.1417986Z 24    |
2019-12-26T22:19:06.1418378Z 25 LL | fn explicit<'a>(x: &'a i32) -> impl Copy + 'a { x }
2019-12-26T22:19:06.1419087Z 
2019-12-26T22:19:06.1419245Z 
2019-12-26T22:19:06.1419666Z The actual stderr differed from the expected stderr.
2019-12-26T22:19:06.1419666Z The actual stderr differed from the expected stderr.
2019-12-26T22:19:06.1422372Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound.nll/must_outlive_least_region_or_bound.nll.stderr
2019-12-26T22:19:06.1479285Z To update references, rerun the tests and pass the `--bless` flag
2019-12-26T22:19:06.1479929Z To only update this specific test, also pass `--test-args impl-trait/must_outlive_least_region_or_bound.rs`
2019-12-26T22:19:06.1480102Z error: 1 errors occurred comparing output.
2019-12-26T22:19:06.1480176Z status: exit code: 1
2019-12-26T22:19:06.1480176Z status: exit code: 1
2019-12-26T22:19:06.1481351Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound.nll/auxiliary" "-A" "unused"
2019-12-26T22:19:06.1481963Z ------------------------------------------
2019-12-26T22:19:06.1482035Z 
2019-12-26T22:19:06.1482306Z ------------------------------------------
2019-12-26T22:19:06.1482397Z stderr:
2019-12-26T22:19:06.1482397Z stderr:
2019-12-26T22:19:06.1482734Z ------------------------------------------
2019-12-26T22:19:06.1482830Z error: lifetime may not live long enough
2019-12-26T22:19:06.1483148Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:3:23
2019-12-26T22:19:06.1483258Z    |
2019-12-26T22:19:06.1483531Z LL | fn elided(x: &i32) -> impl Copy { x }
2019-12-26T22:19:06.1483863Z    |              -        ^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
2019-12-26T22:19:06.1484266Z    |              let's call the lifetime of this reference `'1`
2019-12-26T22:19:06.1484365Z    |
2019-12-26T22:19:06.1484365Z    |
2019-12-26T22:19:06.1484690Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-26T22:19:06.1484809Z    |
2019-12-26T22:19:06.1485073Z LL | fn elided(x: &i32) -> impl Copy + '_ { x }
2019-12-26T22:19:06.1485229Z 
2019-12-26T22:19:06.1485311Z error: lifetime may not live long enough
2019-12-26T22:19:06.1485632Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:6:32
2019-12-26T22:19:06.1485736Z    |
2019-12-26T22:19:06.1485736Z    |
2019-12-26T22:19:06.1486005Z LL | fn explicit<'a>(x: &'a i32) -> impl Copy { x }
2019-12-26T22:19:06.1486363Z    |             --                 ^^^^^^^^^ opaque type requires that `'a` must outlive `'static`
2019-12-26T22:19:06.1486737Z    |             lifetime `'a` defined here
2019-12-26T22:19:06.1486828Z    |
2019-12-26T22:19:06.1487092Z    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1487092Z    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1487583Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a bound
2019-12-26T22:19:06.1487686Z    |
2019-12-26T22:19:06.1487981Z LL | fn explicit<'a>(x: &'a i32) -> impl Copy + 'a { x }
2019-12-26T22:19:06.1488265Z 
2019-12-26T22:19:06.1488333Z error: lifetime may not live long enough
2019-12-26T22:19:06.1488706Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:12:69
2019-12-26T22:19:06.1488795Z    |
2019-12-26T22:19:06.1488795Z    |
2019-12-26T22:19:06.1489111Z LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }
2019-12-26T22:19:06.1489514Z    |               -- lifetime `'a` defined here                         ^ returning this value requires that `'a` must outlive `'static`
2019-12-26T22:19:06.1489934Z    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1490609Z    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1490673Z 
2019-12-26T22:19:06.1490762Z error: lifetime may not live long enough
2019-12-26T22:19:06.1490762Z error: lifetime may not live long enough
2019-12-26T22:19:06.1497842Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:17:61
2019-12-26T22:19:06.1497964Z    |
2019-12-26T22:19:06.1498379Z LL | fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) {
2019-12-26T22:19:06.1498784Z    |                          --  -- lifetime `'b` defined here  ^^^^^^^^^^^^^^^^ opaque type requires that `'b` must outlive `'a`
2019-12-26T22:19:06.1499195Z    |                          lifetime `'a` defined here
2019-12-26T22:19:06.1499289Z    |
2019-12-26T22:19:06.1499566Z    = help: consider adding the following bound: `'b: 'a`
2019-12-26T22:19:06.1499640Z 
2019-12-26T22:19:06.1499640Z 
2019-12-26T22:19:06.1499715Z error[E0310]: the parameter type `T` may not live long enough
2019-12-26T22:19:06.1500055Z   --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:22:51
2019-12-26T22:19:06.1500144Z    |
2019-12-26T22:19:06.1500461Z LL | fn ty_param_wont_outlive_static<T:Debug>(x: T) -> impl Debug + 'static {
2019-12-26T22:19:06.1511487Z    |
2019-12-26T22:19:06.1512326Z    = help: consider adding an explicit lifetime bound `T: 'static`...
2019-12-26T22:19:06.1512402Z 
2019-12-26T22:19:06.1512469Z error: aborting due to 5 previous errors
---
2019-12-26T22:19:06.1514335Z 
2019-12-26T22:19:06.1514412Z 6    |                         |
2019-12-26T22:19:06.1514755Z 7    |                         let's call the lifetime of this reference `'1`
2019-12-26T22:19:06.1514854Z 8    |
2019-12-26T22:19:06.1515185Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-12-26T22:19:06.1515547Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-26T22:19:06.1515638Z 10    |
2019-12-26T22:19:06.1515931Z 11 LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> + '_ {
2019-12-26T22:19:06.1516095Z 
2019-12-26T22:19:06.1516354Z 20    |                    lifetime `'a` defined here
2019-12-26T22:19:06.1516446Z 21    |
2019-12-26T22:19:06.1516698Z 22    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1516698Z 22    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1517042Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a constraint
2019-12-26T22:19:06.1517393Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a bound
2019-12-26T22:19:06.1517481Z 24    |
2019-12-26T22:19:06.1517987Z 25 LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {
2019-12-26T22:19:06.1518837Z 
2019-12-26T22:19:06.1518870Z 
2019-12-26T22:19:06.1518935Z The actual stderr differed from the expected stderr.
2019-12-26T22:19:06.1518935Z The actual stderr differed from the expected stderr.
2019-12-26T22:19:06.1520233Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll/static-return-lifetime-infered.nll.stderr
2019-12-26T22:19:06.1520590Z To update references, rerun the tests and pass the `--bless` flag
2019-12-26T22:19:06.1520919Z To only update this specific test, also pass `--test-args impl-trait/static-return-lifetime-infered.rs`
2019-12-26T22:19:06.1521068Z error: 1 errors occurred comparing output.
2019-12-26T22:19:06.1521156Z status: exit code: 1
2019-12-26T22:19:06.1521156Z status: exit code: 1
2019-12-26T22:19:06.1522289Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll/auxiliary" "-A" "unused"
2019-12-26T22:19:06.1523182Z ------------------------------------------
2019-12-26T22:19:06.1523236Z 
2019-12-26T22:19:06.1523496Z ------------------------------------------
2019-12-26T22:19:06.1523569Z stderr:
2019-12-26T22:19:06.1523569Z stderr:
2019-12-26T22:19:06.1523818Z ------------------------------------------
2019-12-26T22:19:06.1523895Z error: lifetime may not live long enough
2019-12-26T22:19:06.1524217Z   --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:6:35
2019-12-26T22:19:06.1524302Z    |
2019-12-26T22:19:06.1524597Z LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> {
2019-12-26T22:19:06.1524961Z    |                         -         ^^^^^^^^^^^^^^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
2019-12-26T22:19:06.1525365Z    |                         let's call the lifetime of this reference `'1`
2019-12-26T22:19:06.1525446Z    |
2019-12-26T22:19:06.1525446Z    |
2019-12-26T22:19:06.1525766Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-26T22:19:06.1525855Z    |
2019-12-26T22:19:06.1526142Z LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> + '_ {
2019-12-26T22:19:06.1526306Z 
2019-12-26T22:19:06.1526368Z error: lifetime may not live long enough
2019-12-26T22:19:06.1527006Z   --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:10:37
2019-12-26T22:19:06.1527098Z    |
2019-12-26T22:19:06.1527098Z    |
2019-12-26T22:19:06.1527516Z LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> {
2019-12-26T22:19:06.1528046Z    |                    --               ^^^^^^^^^^^^^^^^^^^^^^^ opaque type requires that `'a` must outlive `'static`
2019-12-26T22:19:06.1528392Z    |                    lifetime `'a` defined here
2019-12-26T22:19:06.1528460Z    |
2019-12-26T22:19:06.1528711Z    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1528711Z    = help: consider replacing `'a` with `'static`
2019-12-26T22:19:06.1529728Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a bound
2019-12-26T22:19:06.1529843Z    |
2019-12-26T22:19:06.1530135Z LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {
2019-12-26T22:19:06.1530309Z 
2019-12-26T22:19:06.1530388Z error: aborting due to 2 previous errors
2019-12-26T22:19:06.1530433Z 
2019-12-26T22:19:06.1530468Z 
---
2019-12-26T22:19:06.1531424Z 
2019-12-26T22:19:06.1531605Z 6    |                          |
2019-12-26T22:19:06.1531900Z 7    |                          let's call the lifetime of this reference `'1`
2019-12-26T22:19:06.1531997Z 8    |
2019-12-26T22:19:06.1532310Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-12-26T22:19:06.1532661Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-26T22:19:06.1532749Z 10    |
2019-12-26T22:19:06.1533918Z 11 LL |     async fn f(self: Pin<&Self>) -> impl Clone + '_ { self }
2019-12-26T22:19:06.1534108Z 
2019-12-26T22:19:06.1534143Z 
2019-12-26T22:19:06.1534227Z The actual stderr differed from the expected stderr.
2019-12-26T22:19:06.1534227Z The actual stderr differed from the expected stderr.
2019-12-26T22:19:06.1534688Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll/arbitrary_self_types_pin_lifetime_impl_trait-async.nll.stderr
2019-12-26T22:19:06.1535038Z To update references, rerun the tests and pass the `--bless` flag
2019-12-26T22:19:06.1535390Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs`
2019-12-26T22:19:06.1535543Z error: 1 errors occurred comparing output.
2019-12-26T22:19:06.1535632Z status: exit code: 1
2019-12-26T22:19:06.1535632Z status: exit code: 1
2019-12-26T22:19:06.1536904Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll/auxiliary" "-A" "unused"
2019-12-26T22:19:06.1537664Z ------------------------------------------
2019-12-26T22:19:06.1537733Z 
2019-12-26T22:19:06.1537968Z ------------------------------------------
2019-12-26T22:19:06.1538054Z stderr:
2019-12-26T22:19:06.1538054Z stderr:
2019-12-26T22:19:06.1538279Z ------------------------------------------
2019-12-26T22:19:06.1538376Z error: lifetime may not live long enough
2019-12-26T22:19:06.1538684Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs:8:48
2019-12-26T22:19:06.1538948Z    |
2019-12-26T22:19:06.1539618Z LL |     async fn f(self: Pin<&Self>) -> impl Clone { self }
2019-12-26T22:19:06.1540145Z    |                          -                     ^^^^^^^^ returning this value requires that `'1` must outlive `'static`
2019-12-26T22:19:06.1540717Z    |                          let's call the lifetime of this reference `'1`
2019-12-26T22:19:06.1540813Z    |
2019-12-26T22:19:06.1540813Z    |
2019-12-26T22:19:06.1541117Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-26T22:19:06.1541220Z    |
2019-12-26T22:19:06.1541483Z LL |     async fn f(self: Pin<&Self>) -> impl Clone + '_ { self }
2019-12-26T22:19:06.1541652Z 
2019-12-26T22:19:06.1541735Z error: aborting due to previous error
2019-12-26T22:19:06.1541780Z 
2019-12-26T22:19:06.1541814Z 
---
2019-12-26T22:19:06.1543221Z 
2019-12-26T22:19:06.1543282Z 6    |                    |
2019-12-26T22:19:06.1543585Z 7    |                    let's call the lifetime of this reference `'1`
2019-12-26T22:19:06.1543682Z 8    |
2019-12-26T22:19:06.1543998Z - help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a constraint
2019-12-26T22:19:06.1544348Z + help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-26T22:19:06.1544437Z 10    |
2019-12-26T22:19:06.1544715Z 11 LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self }
2019-12-26T22:19:06.1544965Z 
2019-12-26T22:19:06.1544999Z 
2019-12-26T22:19:06.1545076Z The actual stderr differed from the expected stderr.
2019-12-26T22:19:06.1545076Z The actual stderr differed from the expected stderr.
2019-12-26T22:19:06.1545557Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll/arbitrary_self_types_pin_lifetime_impl_trait.nll.stderr
2019-12-26T22:19:06.1545903Z To update references, rerun the tests and pass the `--bless` flag
2019-12-26T22:19:06.1546247Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait.rs`
2019-12-26T22:19:06.1546438Z error: 1 errors occurred comparing output.
2019-12-26T22:19:06.1546526Z status: exit code: 1
2019-12-26T22:19:06.1546526Z status: exit code: 1
2019-12-26T22:19:06.1547700Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll/auxiliary" "-A" "unused"
2019-12-26T22:19:06.1549754Z ------------------------------------------
2019-12-26T22:19:06.1549811Z 
2019-12-26T22:19:06.1550078Z ------------------------------------------
2019-12-26T22:19:06.1550167Z stderr:
2019-12-26T22:19:06.1550167Z stderr:
2019-12-26T22:19:06.1550405Z ------------------------------------------
2019-12-26T22:19:06.1550497Z error: lifetime may not live long enough
2019-12-26T22:19:06.1550819Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:31
2019-12-26T22:19:06.1550921Z    |
2019-12-26T22:19:06.1551243Z LL |     fn f(self: Pin<&Self>) -> impl Clone { self } //~ ERROR cannot infer an appropriate lifetime
2019-12-26T22:19:06.1551601Z    |                    -          ^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
2019-12-26T22:19:06.1551987Z    |                    let's call the lifetime of this reference `'1`
2019-12-26T22:19:06.1552083Z    |
2019-12-26T22:19:06.1552083Z    |
2019-12-26T22:19:06.1552387Z help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
2019-12-26T22:19:06.1552490Z    |
2019-12-26T22:19:06.1552803Z LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self } //~ ERROR cannot infer an appropriate lifetime
2019-12-26T22:19:06.1553136Z 
2019-12-26T22:19:06.1553222Z error: aborting due to previous error
2019-12-26T22:19:06.1553278Z 
2019-12-26T22:19:06.1553313Z 
---
2019-12-26T22:19:06.1555912Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-26T22:19:06.1556033Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-26T22:19:06.1556258Z 
2019-12-26T22:19:06.1556296Z 
2019-12-26T22:19:06.1558645Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-12-26T22:19:06.1560006Z 
2019-12-26T22:19:06.1560044Z 
2019-12-26T22:19:06.1562356Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-26T22:19:06.1562609Z Build completed unsuccessfully in 1:47:44
2019-12-26T22:19:06.1562609Z Build completed unsuccessfully in 1:47:44
2019-12-26T22:19:06.1581130Z == clock drift check ==
2019-12-26T22:19:06.1630423Z   local time: Thu Dec 26 22:19:06 UTC 2019
2019-12-26T22:19:06.4470254Z   network time: Thu, 26 Dec 2019 22:19:06 GMT
2019-12-26T22:19:06.4473532Z == end clock drift check ==
2019-12-26T22:19:07.2548779Z 
2019-12-26T22:19:07.2644109Z ##[error]Bash exited with code '1'.
2019-12-26T22:19:07.2713105Z ##[section]Starting: Checkout
2019-12-26T22:19:07.2715579Z ==============================================================================
2019-12-26T22:19:07.2715687Z Task         : Get sources
2019-12-26T22:19:07.2715769Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
