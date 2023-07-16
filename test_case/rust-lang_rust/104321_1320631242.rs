plain
---- [ui] src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/default-body-with-rpit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/default-body-with-rpit/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: concrete type differs from previous defining opaque type use
   |
LL |         ""
LL |         ""
   |         ^^ expected `impl Debug`, got `&'static str`
note: previous use here
  --> /checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs:10:39
   |
   |
LL |       async fn baz(&self) -> impl Debug {
LL | |         ""
LL | |     }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   | |_____^
   | |_____^

error[E0720]: cannot resolve opaque type
  --> /checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs:10:28
   |
LL |     async fn baz(&self) -> impl Debug {
   |                            ^^^^^^^^^^ cannot resolve opaque type
   |
   = note: these returned values have a concrete "never" type
   = help: this error will resolve once the item's body returns a concrete type
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0720`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/issue-72051-member-region-hang.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-72051-member-region-hang.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-72051-member-region-hang" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-72051-member-region-hang/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | pub async fn query<'a>(_: &(), _: &(), _: (&(dyn std::any::Any + 'a),) ) {}
   |                    |
   |                    |
   |                    hidden type `impl Future<Output = ()>` captures the lifetime `'a` as defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/self_lifetime-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_lifetime-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_lifetime-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_lifetime-async/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &'a ()>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn bar<'a>(self: &Alias, arg: &'a ()) -> &() { arg }
   |
   |
   = note: hidden type `impl Future<Output = &'a ()>` captures lifetime '_#17r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
