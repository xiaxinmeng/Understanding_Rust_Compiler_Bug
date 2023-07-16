plain
........................................................................................ 11352/13398
........................................................................................ 11440/13398
........................................................................................ 11528/13398
........................................................................................ 11616/13398
............................................F...F....................................... 11704/13398
..................i.......i........i.....i.....................i........................ 11880/13398
........................................................................................ 11968/13398
........................................................................................ 12056/13398
........................................................................................ 12144/13398
---
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0311]: the parameter type `T` may not live long enough
   |
   |
LL | /     foo.bar(move |_| {
LL | |     //~^ ERROR the parameter type `T` may not live long enough
LL | |         t.test();
LL | |     });
   |
   |
note: the parameter type `T` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn func<T: Test>(foo: &Foo, t: T) {
   |                        ^^^
note: ...so that the type `T` will meet its required lifetime bounds
   |
   |
LL | /     foo.bar(move |_| {
LL | |     //~^ ERROR the parameter type `T` may not live long enough
LL | |         t.test();
LL | |     });
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn func<T: Test + 'a>(foo: &Foo, t: T) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0311`.
---
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0261]: use of undeclared lifetime name `'a`
   |
   |
LL | fn baz<G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |        -  ^^ undeclared lifetime
   |        |
   |        help: consider introducing lifetime `'a` here: `'a,`

error[E0700]: hidden type for `impl FnOnce()` captures lifetime that does not appear in bounds
   |
   |
LL |   fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce()
   |                            ------ hidden type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:19:5: 19:12]` captures the anonymous lifetime defined here
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR hidden type for `impl FnOnce()` captures lifetime
LL | |         *dest = g.get();
LL | |     }
   |
   |
help: to declare that the `impl Trait` captures `'_`, you can add an explicit `'_` lifetime bound
   |
LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


error[E0311]: the parameter type `G` may not live long enough
   |
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         *dest = g.get();
LL | |     }
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |                          ^^^^^^
note: ...so that the type `G` will meet its required lifetime bounds
   |
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         *dest = g.get();
LL | |     }
help: consider adding an explicit lifetime bound...
   |
   |
LL |     G: Get<T> + 'a,


error[E0311]: the parameter type `G` may not live long enough
   |
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         *dest = g.get();
LL | |     }
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |                                  ^^^^^^
note: ...so that the type `G` will meet its required lifetime bounds
   |
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         *dest = g.get();
LL | |     }
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn qux<'a, G: 'a + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


error[E0311]: the parameter type `G` may not live long enough
   |
LL | /         move || {
LL | /         move || {
LL | |             //~^ ERROR the parameter type `G` may not live long enough
LL | |             *dest = g.get();
LL | |         }
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
   |                                               ^^^^^^
note: ...so that the type `G` will meet its required lifetime bounds
   |
LL | /         move || {
LL | /         move || {
LL | |             //~^ ERROR the parameter type `G` may not live long enough
LL | |             *dest = g.get();
LL | |         }
help: consider adding an explicit lifetime bound...
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b + 'c, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {


error[E0311]: the parameter type `G` may not live long enough
   |
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         //~| ERROR explicit lifetime required
LL | |         *dest = g.get();
LL | |     }
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
   |                                  ^^^^^^
note: ...so that the type `G` will meet its required lifetime bounds
   |
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         //~| ERROR explicit lifetime required
LL | |         *dest = g.get();
LL | |     }
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn bat<'a, G: 'a + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a


error[E0621]: explicit lifetime required in the type of `dest`
   |
   |
LL |   fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
   |                                    ------ help: add explicit lifetime `'a` to the type of `dest`: `&'a mut T`
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         //~| ERROR explicit lifetime required
LL | |         *dest = g.get();
LL | |     }
   | |_____^ lifetime `'a` required

error[E0309]: the parameter type `G` may not live long enough
   |
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         *dest = g.get();
LL | |     }
   | |_____^ ...so that the type `G` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     G: Get<T> + 'a,

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0261, E0309, E0311, E0621, E0700.
