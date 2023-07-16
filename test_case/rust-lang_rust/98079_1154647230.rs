plain
........................................................................................ 9680/13068
..................................ii...............i.................................... 9768/13068
........................ii.............................................................. 9856/13068
........................................................................................ 9944/13068
...............................................................F........................ 10032/13068
..........................................F.........F................................... 10120/13068
........................................................................................ 10296/13068
........................................................................................ 10384/13068
.......................................................i..i.i........................... 10472/13068
........................................................................................ 10560/13068
---

---- [ui] src/test/ui/generic-associated-types/unsatified-item-lifetime-bound.rs stdout ----
diff of stderr:

4 LL |     type Y<'a: 'static>;
6    |
+    = note: `#[warn(named_static_lifetimes)]` on by default
+    = note: `#[warn(named_static_lifetimes)]` on by default
7    = help: you can use the `'static` lifetime directly, in place of `'a`
8 
9 error[E0478]: lifetime bound not satisfied

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatified-item-lifetime-bound/unsatified-item-lifetime-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/unsatified-item-lifetime-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/unsatified-item-lifetime-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatified-item-lifetime-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatified-item-lifetime-bound/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL |     type Y<'a: 'static>;
   |
   = note: `#[warn(named_static_lifetimes)]` on by default
   = note: `#[warn(named_static_lifetimes)]` on by default
   = help: you can use the `'static` lifetime directly, in place of `'a`

error[E0478]: lifetime bound not satisfied
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     f: <T as X>::Y<'a>,
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | struct B<'a, T: for<'r> X<Y<'r> = &'r ()>> {
   = note: but lifetime parameter must outlive the static lifetime


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     f: <T as X>::Y<'a>,
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | struct C<'a, T: X> {
   = note: but lifetime parameter must outlive the static lifetime


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     f: <() as X>::Y<'a>,
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | struct D<'a> {
   = note: but lifetime parameter must outlive the static lifetime

error: aborting due to 3 previous errors; 1 warning emitted


For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/impl-trait/equal-hidden-lifetimes.rs stdout ----
diff of stderr:

4 LL | fn equal_regions_static<'a: 'static>(x: &'a i32) -> impl Sized {
6    |
+    = note: `#[warn(named_static_lifetimes)]` on by default
+    = note: `#[warn(named_static_lifetimes)]` on by default
7    = help: you can use the `'static` lifetime directly, in place of `'a`
9 warning: 1 warning emitted


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/equal-hidden-lifetimes/equal-hidden-lifetimes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/equal-hidden-lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/equal-hidden-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/equal-hidden-lifetimes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/equal-hidden-lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL | fn equal_regions_static<'a: 'static>(x: &'a i32) -> impl Sized {
   |
   = note: `#[warn(named_static_lifetimes)]` on by default
   = note: `#[warn(named_static_lifetimes)]` on by default
   = help: you can use the `'static` lifetime directly, in place of `'a`
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/issues/issue-30438-c.rs stdout ----
diff of stderr:

4 LL | fn silly<'y, 'z>(_s: &'y Test<'z>) -> &'y <Test<'z> as Trait>::Out where 'z: 'static {
6    |
+    = note: `#[warn(named_static_lifetimes)]` on by default
+    = note: `#[warn(named_static_lifetimes)]` on by default
7    = help: you can use the `'static` lifetime directly, in place of `'z`
9 error[E0515]: cannot return reference to local variable `x`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30438-c/issue-30438-c.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-30438-c.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-30438-c.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30438-c" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30438-c/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary lifetime parameter `'z`
   |
   |
LL | fn silly<'y, 'z>(_s: &'y Test<'z>) -> &'y <Test<'z> as Trait>::Out where 'z: 'static {
   |
   = note: `#[warn(named_static_lifetimes)]` on by default
   = note: `#[warn(named_static_lifetimes)]` on by default
   = help: you can use the `'static` lifetime directly, in place of `'z`
error[E0515]: cannot return reference to local variable `x`
  --> /checkout/src/test/ui/issues/issue-30438-c.rs:10:5
   |
LL |     &x
LL |     &x
   |     ^^ returns a reference to data owned by the current function
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0515`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/regions-free-region-outlives-static-outlives-free-region.rs stdout ----
diff of stderr:

4 LL |     where 'a: 'static
6    |
+    = note: `#[warn(named_static_lifetimes)]` on by default
+    = note: `#[warn(named_static_lifetimes)]` on by default
7    = help: you can use the `'static` lifetime directly, in place of `'a`
9 warning: 1 warning emitted


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-outlives-static-outlives-free-region/regions-free-region-outlives-static-outlives-free-region.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-free-region-outlives-static-outlives-free-region.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-free-region-outlives-static-outlives-free-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-outlives-static-outlives-free-region/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-outlives-static-outlives-free-region/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL |     where 'a: 'static //~ WARN unnecessary lifetime parameter `'a`
   |
   = note: `#[warn(named_static_lifetimes)]` on by default
   = note: `#[warn(named_static_lifetimes)]` on by default
   = help: you can use the `'static` lifetime directly, in place of `'a`
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/regions/regions-static-bound.rs stdout ----
diff of stderr:

4 LL |     where 'a: 'static { t }
6    |
+    = note: `#[warn(named_static_lifetimes)]` on by default
+    = note: `#[warn(named_static_lifetimes)]` on by default
7    = help: you can use the `'static` lifetime directly, in place of `'a`
9 warning: unnecessary lifetime parameter `'b`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound/regions-static-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-static-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-static-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL |     where 'a: 'static { t }
   |
   = note: `#[warn(named_static_lifetimes)]` on by default
   = note: `#[warn(named_static_lifetimes)]` on by default
   = help: you can use the `'static` lifetime directly, in place of `'a`
warning: unnecessary lifetime parameter `'b`
  --> /checkout/src/test/ui/regions/regions-static-bound.rs:6:19
   |
   |
LL |     where 'a: 'b, 'b: 'static { t }
   |
   |
   = help: you can use the `'static` lifetime directly, in place of `'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-static-bound.rs:10:5
   |
   |
LL | fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {
   |                        -- lifetime `'a` defined here
LL |     t
   |     ^ returning this value requires that `'a` must outlive `'static`

error[E0521]: borrowed data escapes outside of function
   |
   |
LL | fn error(u: &(), v: &()) {
   |          -  - let's call the lifetime of this reference `'1`
   |          |
   |          `u` is a reference that is only valid in the function body
LL |     static_id(&u);
   |     |
   |     |
   |     `u` escapes the function body here
   |     argument requires that `'1` must outlive `'static`

error[E0521]: borrowed data escapes outside of function
   |
   |
LL | fn error(u: &(), v: &()) {
   |                  -  - let's call the lifetime of this reference `'2`
   |                  |
   |                  `v` is a reference that is only valid in the function body
...
LL |     static_id_indirect(&v);
   |     |
   |     |
   |     `v` escapes the function body here
   |     argument requires that `'2` must outlive `'static`
error: aborting due to 3 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0521`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/regions-static-bound-rpass.rs stdout ----
diff of stderr:

4 LL |     where 'a: 'static { t }
6    |
+    = note: `#[warn(named_static_lifetimes)]` on by default
+    = note: `#[warn(named_static_lifetimes)]` on by default
7    = help: you can use the `'static` lifetime directly, in place of `'a`
9 warning: unnecessary lifetime parameter `'a`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound-rpass/regions-static-bound-rpass.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-static-bound-rpass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-static-bound-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound-rpass/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL |     where 'a: 'static { t }
   |
   = note: `#[warn(named_static_lifetimes)]` on by default
   = note: `#[warn(named_static_lifetimes)]` on by default
   = help: you can use the `'static` lifetime directly, in place of `'a`
warning: unnecessary lifetime parameter `'a`
  --> /checkout/src/test/ui/regions/regions-static-bound-rpass.rs:8:11
   |
   |
LL |     where 'a: 'static { t }
   |
   |
   = help: you can use the `'static` lifetime directly, in place of `'a`
warning: unnecessary lifetime parameter `'b`
  --> /checkout/src/test/ui/regions/regions-static-bound-rpass.rs:12:19
   |
   |
LL |     where 'a: 'b, 'b: 'static { t }
   |
   |
   = help: you can use the `'static` lifetime directly, in place of `'b`
warning: 3 warnings emitted
------------------------------------------



---- [ui] src/test/ui/static/static-lifetime-bound.rs stdout ----
diff of stderr:

4 LL | fn f<'a: 'static>(_: &'a i32) {}
6    |
+    = note: `#[warn(named_static_lifetimes)]` on by default
+    = note: `#[warn(named_static_lifetimes)]` on by default
7    = help: you can use the `'static` lifetime directly, in place of `'a`
8 
9 error[E0597]: `x` does not live long enough

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound/static-lifetime-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-lifetime-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-lifetime-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL | fn f<'a: 'static>(_: &'a i32) {} //~WARN unnecessary lifetime parameter `'a`
   |
   = note: `#[warn(named_static_lifetimes)]` on by default
   = note: `#[warn(named_static_lifetimes)]` on by default
   = help: you can use the `'static` lifetime directly, in place of `'a`

error[E0597]: `x` does not live long enough
   |
   |
LL |     f(&x); //~ERROR does not live long enough
   |     --^^-
   |     | borrowed value does not live long enough
   |     | borrowed value does not live long enough
   |     argument requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/bounds-are-checked.rs stdout ----
diff of stderr:

4 LL | fn f<'a: 'static>(t: &'a str) -> X<'a> {
6    |
+    = note: `#[warn(named_static_lifetimes)]` on by default
+    = note: `#[warn(named_static_lifetimes)]` on by default
7    = help: you can use the `'static` lifetime directly, in place of `'a`
9 error: non-defining opaque type use in defining scope


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked/bounds-are-checked.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/bounds-are-checked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/bounds-are-checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL | fn f<'a: 'static>(t: &'a str) -> X<'a> {
   |
   = note: `#[warn(named_static_lifetimes)]` on by default
   = note: `#[warn(named_static_lifetimes)]` on by default
   = help: you can use the `'static` lifetime directly, in place of `'a`
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/bounds-are-checked.rs:10:5
   |
   |
LL | type X<'a> = impl Into<&'static str> + From<&'a str>;
   |        -- cannot use static lifetime; use a bound lifetime instead or remove the lifetime parameter from the opaque type
LL |     t
   |     ^

error: aborting due to previous error; 1 warning emitted
