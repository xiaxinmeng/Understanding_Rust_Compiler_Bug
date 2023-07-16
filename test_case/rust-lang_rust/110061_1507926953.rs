plain
....i......i..i.........................................................................  8976/14805
........................................................................................  9064/14805
........................................................................................  9152/14805
........................................................................................  9240/14805
...................................................................................F..FF  9328/14805
FF..F...............F..F.....F............................i..i..........................  9416/14805
........................................................................................  9592/14805
........................................................................................  9680/14805
..i........................................i............................................  9768/14805
...........................i............................................................  9856/14805
---

4 LL |     let x = {
5    |         - borrow later stored here
6 LL |         let bar = 22;
+    |             --- binding `bar` declared here
7 LL |         Foo::new(&bar).await
8    |                  ^^^^ borrowed value does not live long enough


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/feature-self-return-type/feature-self-return-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/feature-self-return-type/feature-self-return-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/feature-self-return-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/feature-self-return-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/feature-self-return-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/feature-self-return-type/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0597]: `bar` does not live long enough
  --> fake-test-src-base/async-await/feature-self-return-type.rs:22:18
LL |     let x = {
   |         - borrow later stored here
LL |         let bar = 22;
LL |         let bar = 22;
   |             --- binding `bar` declared here
LL |         Foo::new(&bar).await
   |                  ^^^^ borrowed value does not live long enough
LL |         //~^ ERROR `bar` does not live long enough
LL |     };
   |     - `bar` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
---

13 LL |     let x = {
14    |         - borrow later stored here
15 LL |         let bar = 22;
+    |             --- binding `bar` declared here
16 LL |         Foo::new(&bar).await
17    |                  ^^^^ borrowed value does not live long enough


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61949-self-return-type/issue-61949-self-return-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61949-self-return-type/issue-61949-self-return-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-61949-self-return-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issue-61949-self-return-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61949-self-return-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61949-self-return-type/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0658]: `async fn` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
  --> fake-test-src-base/async-await/issue-61949-self-return-type.rs:11:40
   |
LL |     pub async fn new(_bar: &'a i32) -> Self {
   |                                        ^^^^ help: consider spelling out the type instead: `Foo<'a>`
   = note: see issue #103532 <https://github.com/rust-lang/rust/issues/103532> for more information
   = help: add `#![feature(impl_trait_projections)]` to the crate attributes to enable


error[E0597]: `bar` does not live long enough
  --> fake-test-src-base/async-await/issue-61949-self-return-type.rs:22:18
LL |     let x = {
   |         - borrow later stored here
LL |         let bar = 22;
LL |         let bar = 22;
   |             --- binding `bar` declared here
LL |         Foo::new(&bar).await
   |                  ^^^^ borrowed value does not live long enough
LL |         //~^ ERROR `bar` does not live long enough
LL |     };
   |     - `bar` dropped here while still borrowed
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0658.
For more information about an error, try `rustc --explain E0597`.
For more information about an error, try `rustc --explain E0597`.
------------------------------------------


---- [ui] tests/ui/consts/const-eval/generic-slice.rs stdout ----
diff of stderr:

4 LL | impl<'a, T: 'static> Generic<'a, T> {
5    |      -- lifetime `'a` defined here
6 ...
+ LL |         let x: &'static [T] = &[];
+    |             - binding `x` declared here
7 LL |         &x
9    |         |


16 error[E0597]: `x` does not live long enough
18    |
18    |
+ LL |     let x: &[_] = &[];
+    |         - binding `x` declared here
19 LL |     &x
21    |     |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice/generic-slice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/generic-slice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/generic-slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
  --> fake-test-src-base/consts/const-eval/generic-slice.rs:15:9
   |
LL | impl<'a, T: 'static> Generic<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |         let x: &'static [T] = &[];
   |             - binding `x` declared here
LL |         &x
   |         |
   |         borrowed value does not live long enough
   |         borrowed value does not live long enough
   |         using this value as a constant requires that `x` is borrowed for `'a`
LL |         //~^ ERROR `x` does not live long enough
LL |     };
   |     - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
  --> fake-test-src-base/consts/const-eval/generic-slice.rs:27:5
   |
LL |     let x: &[_] = &[];
   |         - binding `x` declared here
LL |     &x
   |     |
   |     borrowed value does not live long enough
   |     borrowed value does not live long enough
   |     using this value as a static requires that `x` is borrowed for `'static`
LL |     //~^ ERROR `x` does not live long enough
LL | };
   | - `x` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/auto-trait-regions.rs#drop_tracking stdout ----
diff of stderr:

9 LL |         assert_foo(a);
11    |
-    = note: consider using a `let` binding to create a longer lived value
+ help: consider using a `let` binding to create a longer lived value
+    |
+    |
+ LL ~         let binding = true;
+ LL ~         let a = A(&mut binding, &mut true, No);
13 
14 error[E0716]: temporary value dropped while borrowed
15   --> $DIR/auto-trait-regions.rs:48:35


22 LL |         assert_foo(a);
24    |
-    = note: consider using a `let` binding to create a longer lived value
+ help: consider using a `let` binding to create a longer lived value
+    |
+    |
+ LL ~         let binding = true;
+ LL ~         let a = A(&mut true, &mut binding, No);
26 
27 error: implementation of `Foo` is not general enough
28   --> $DIR/auto-trait-regions.rs:34:5



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.drop_tracking/auto-trait-regions.drop_tracking.stderr
To only update this specific test, also pass `--test-args generator/auto-trait-regions.rs`


error in revision `drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/auto-trait-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.drop_tracking/auxiliary" "-Zdrop-tracking"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/generator/auto-trait-regions.rs:48:24
   |
   |
LL |         let a = A(&mut true, &mut true, No);
   |                        |
   |                        creates a temporary value which is freed while still in use
...
...
LL |         assert_foo(a);
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~         let binding = true;
LL ~         let binding = true;
LL ~         let a = A(&mut binding, &mut true, No);

error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/generator/auto-trait-regions.rs:48:35
   |
   |
LL |         let a = A(&mut true, &mut true, No);
   |                                   |
   |                                   creates a temporary value which is freed while still in use
...
...
LL |         assert_foo(a);
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~         let binding = true;
LL ~         let binding = true;
LL ~         let a = A(&mut true, &mut binding, No);

error: implementation of `Foo` is not general enough
  --> fake-test-src-base/generator/auto-trait-regions.rs:34:5
   |
   |
LL |     assert_foo(gen);
   |     ^^^^^^^^^^^^^^^ implementation of `Foo` is not general enough
   |
   = note: `&'0 OnlyFooIfStaticRef` must implement `Foo`, for any lifetime `'0`...
   = note: ...but `Foo` is actually implemented for the type `&'static OnlyFooIfStaticRef`
error: implementation of `Foo` is not general enough
  --> fake-test-src-base/generator/auto-trait-regions.rs:54:5
   |
   |
LL |     assert_foo(gen);
   |     ^^^^^^^^^^^^^^^ implementation of `Foo` is not general enough
   |
   = note: `Foo` would have to be implemented for the type `A<'0, '1>`, for any two lifetimes `'0` and `'1`...
   = note: ...but `Foo` is actually implemented for the type `A<'_, '2>`, for some specific lifetime `'2`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/auto-trait-regions.rs#no_drop_tracking stdout ----
diff of stderr:

9 LL |         assert_foo(a);
11    |
-    = note: consider using a `let` binding to create a longer lived value
+ help: consider using a `let` binding to create a longer lived value
+    |
+    |
+ LL ~         let binding = true;
+ LL ~         let a = A(&mut binding, &mut true, No);
13 
14 error[E0716]: temporary value dropped while borrowed
15   --> $DIR/auto-trait-regions.rs:48:35


22 LL |         assert_foo(a);
24    |
-    = note: consider using a `let` binding to create a longer lived value
+ help: consider using a `let` binding to create a longer lived value
+    |
+    |
+ LL ~         let binding = true;
+ LL ~         let a = A(&mut true, &mut binding, No);
26 
27 error: implementation of `Foo` is not general enough
28   --> $DIR/auto-trait-regions.rs:34:5

---
To only update this specific test, also pass `--test-args generator/auto-trait-regions.rs`

error in revision `no_drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/auto-trait-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.no_drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.no_drop_tracking/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/generator/auto-trait-regions.rs:48:24
   |
   |
LL |         let a = A(&mut true, &mut true, No);
   |                        |
   |                        creates a temporary value which is freed while still in use
...
...
LL |         assert_foo(a);
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~         let binding = true;
LL ~         let binding = true;
LL ~         let a = A(&mut binding, &mut true, No);

error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/generator/auto-trait-regions.rs:48:35
   |
   |
LL |         let a = A(&mut true, &mut true, No);
   |                                   |
   |                                   creates a temporary value which is freed while still in use
...
...
LL |         assert_foo(a);
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~         let binding = true;
LL ~         let binding = true;
LL ~         let a = A(&mut true, &mut binding, No);

error: implementation of `Foo` is not general enough
  --> fake-test-src-base/generator/auto-trait-regions.rs:34:5
   |
   |
LL |     assert_foo(gen);
   |     ^^^^^^^^^^^^^^^ implementation of `Foo` is not general enough
   |
   = note: `&'0 OnlyFooIfStaticRef` must implement `Foo`, for any lifetime `'0`...
   = note: ...but `Foo` is actually implemented for the type `&'static OnlyFooIfStaticRef`
error: implementation of `Foo` is not general enough
  --> fake-test-src-base/generator/auto-trait-regions.rs:54:5
   |
   |
LL |     assert_foo(gen);
   |     ^^^^^^^^^^^^^^^ implementation of `Foo` is not general enough
   |
   = note: `Foo` would have to be implemented for the type `A<'0, '1>`, for any two lifetimes `'0` and `'1`...
   = note: ...but `Foo` is actually implemented for the type `A<'_, '2>`, for some specific lifetime `'2`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/auto-trait-regions.rs#drop_tracking_mir stdout ----
diff of stderr:

9 LL |         assert_foo(a);
11    |
-    = note: consider using a `let` binding to create a longer lived value
+ help: consider using a `let` binding to create a longer lived value
+    |
+    |
+ LL ~         let binding = true;
+ LL ~         let a = A(&mut binding, &mut true, No);
13 
14 error[E0716]: temporary value dropped while borrowed
15   --> $DIR/auto-trait-regions.rs:48:35


22 LL |         assert_foo(a);
24    |
-    = note: consider using a `let` binding to create a longer lived value
+ help: consider using a `let` binding to create a longer lived value
+    |
+    |
+ LL ~         let binding = true;
+ LL ~         let a = A(&mut true, &mut binding, No);
26 
27 error: implementation of `Foo` is not general enough
28   --> $DIR/auto-trait-regions.rs:34:5



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.drop_tracking_mir/auto-trait-regions.drop_tracking_mir.stderr
To only update this specific test, also pass `--test-args generator/auto-trait-regions.rs`


error in revision `drop_tracking_mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/auto-trait-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking_mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.drop_tracking_mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.drop_tracking_mir/auxiliary" "-Zdrop-tracking-mir"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/generator/auto-trait-regions.rs:48:24
   |
   |
LL |         let a = A(&mut true, &mut true, No);
   |                        |
   |                        creates a temporary value which is freed while still in use
...
...
LL |         assert_foo(a);
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~         let binding = true;
LL ~         let binding = true;
LL ~         let a = A(&mut binding, &mut true, No);

error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/generator/auto-trait-regions.rs:48:35
   |
   |
LL |         let a = A(&mut true, &mut true, No);
   |                                   |
   |                                   creates a temporary value which is freed while still in use
...
...
LL |         assert_foo(a);
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~         let binding = true;
LL ~         let binding = true;
LL ~         let a = A(&mut true, &mut binding, No);

error: implementation of `Foo` is not general enough
  --> fake-test-src-base/generator/auto-trait-regions.rs:34:5
   |
   |
LL |     assert_foo(gen);
   |     ^^^^^^^^^^^^^^^ implementation of `Foo` is not general enough
   |
   = note: `&'0 OnlyFooIfStaticRef` must implement `Foo`, for any lifetime `'0`...
   = note: ...but `Foo` is actually implemented for the type `&'static OnlyFooIfStaticRef`
error: implementation of `Foo` is not general enough
  --> fake-test-src-base/generator/auto-trait-regions.rs:54:5
   |
   |
LL |     assert_foo(gen);
   |     ^^^^^^^^^^^^^^^ implementation of `Foo` is not general enough
   |
   = note: `Foo` would have to be implemented for the type `A<'0, '1>`, for any two lifetimes `'0` and `'1`...
   = note: ...but `Foo` is actually implemented for the type `A<'_, '2>`, for some specific lifetime `'2`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/lifetimes/issue-69314.rs stdout ----
diff of stderr:

12 error[E0597]: `buf` does not live long enough
14    |
14    |
+ LL |         let mut buf = [0; 512];
+    |             ------- binding `buf` declared here
15 LL |         let m2 = &buf[..];
16    |                   ^^^ borrowed value does not live long enough
17 LL |         let m = Self::g(m2).await;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-69314/issue-69314.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-69314.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lifetimes/issue-69314.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-69314" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-69314/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0726]: implicit elided lifetime not allowed here
  --> fake-test-src-base/lifetimes/issue-69314.rs:18:20
   |
LL |     async fn f2(m: Msg) {}
   |                    ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
LL |     async fn f2(m: Msg<'_>) {}


error[E0597]: `buf` does not live long enough
  --> fake-test-src-base/lifetimes/issue-69314.rs:14:19
   |
LL |         let mut buf = [0; 512];
   |             ------- binding `buf` declared here
LL |         let m2 = &buf[..]; //~ ERROR `buf` does not live long enough
   |                   ^^^ borrowed value does not live long enough
LL |         let m = Self::g(m2).await;
   |                 ----------- argument requires that `buf` is borrowed for `'static`
LL |         Self::f2(m).await;
LL |     }
   |     - `buf` dropped here while still borrowed
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0726.
For more information about an error, try `rustc --explain E0597`.
For more information about an error, try `rustc --explain E0597`.
------------------------------------------


---- [ui] tests/ui/nll/user-annotations/adt-brace-enums.rs stdout ----
diff of stderr:

31    |
32 LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
33    |                                              -- lifetime `'a` defined here
- ...
+ LL |     let _closure = || {
+ LL |         let c = 66;
+    |             - binding `c` declared here
35 LL |         SomeEnum::SomeVariant::<&'a u32> { t: &c };
37    |                                               |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/adt-brace-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-brace-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/user-annotations/adt-brace-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-brace-enums.rs:25:48
LL |     let c = 66;
   |         - binding `c` declared here
   |         - binding `c` declared here
LL |     SomeEnum::SomeVariant::<&'static u32> { t: &c }; //~ ERROR
   |                                                |
   |                                                borrowed value does not live long enough
   |                                                borrowed value does not live long enough
   |                                                this usage requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-brace-enums.rs:30:43
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
   |         - binding `c` declared here
LL |     SomeEnum::SomeVariant::<&'a u32> { t: &c }; //~ ERROR
   |                                           |
   |                                           borrowed value does not live long enough
   |                                           borrowed value does not live long enough
   |                                           this usage requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-brace-enums.rs:40:47
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
LL |     let _closure = || {
LL |         let c = 66;
   |             - binding `c` declared here
LL |         SomeEnum::SomeVariant::<&'a u32> { t: &c }; //~ ERROR
   |                                               |
   |                                               borrowed value does not live long enough
   |                                               borrowed value does not live long enough
   |                                               this usage requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/nll/user-annotations/adt-tuple-enums.rs stdout ----
diff of stderr:

31    |
32 LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
33    |                                              -- lifetime `'a` defined here
- ...
+ LL |     let _closure = || {
+ LL |         let c = 66;
+    |             - binding `c` declared here
35 LL |         SomeEnum::SomeVariant::<&'a u32>(&c);
37    |                                          |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/adt-tuple-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/user-annotations/adt-tuple-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-tuple-enums.rs:28:43
LL |     let c = 66;
   |         - binding `c` declared here
   |         - binding `c` declared here
LL |     SomeEnum::SomeVariant::<&'static u32>(&c); //~ ERROR
   |                                           |
   |                                           borrowed value does not live long enough
   |                                           borrowed value does not live long enough
   |                                           this usage requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-tuple-enums.rs:33:38
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
   |         - binding `c` declared here
LL |     SomeEnum::SomeVariant::<&'a u32>(&c); //~ ERROR
   |                                      |
   |                                      borrowed value does not live long enough
   |                                      borrowed value does not live long enough
   |                                      this usage requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-tuple-enums.rs:43:42
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
LL |     let _closure = || {
LL |         let c = 66;
   |             - binding `c` declared here
LL |         SomeEnum::SomeVariant::<&'a u32>(&c); //~ ERROR
   |                                          |
   |                                          borrowed value does not live long enough
   |                                          borrowed value does not live long enough
   |                                          this usage requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/nll/user-annotations/adt-tuple-struct.rs stdout ----
diff of stderr:

31    |
32 LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
33    |                                              -- lifetime `'a` defined here
- ...
+ LL |     let _closure = || {
+ LL |         let c = 66;
+    |             - binding `c` declared here
35 LL |         SomeStruct::<&'a u32>(&c);
37    |                               |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/adt-tuple-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/user-annotations/adt-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-tuple-struct.rs:23:32
LL |     let c = 66;
   |         - binding `c` declared here
   |         - binding `c` declared here
LL |     SomeStruct::<&'static u32>(&c); //~ ERROR
   |                                |
   |                                borrowed value does not live long enough
   |                                borrowed value does not live long enough
   |                                this usage requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-tuple-struct.rs:28:27
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
   |         - binding `c` declared here
LL |     SomeStruct::<&'a u32>(&c); //~ ERROR
   |                           |
   |                           borrowed value does not live long enough
   |                           borrowed value does not live long enough
   |                           this usage requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-tuple-struct.rs:38:31
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
LL |     let _closure = || {
LL |         let c = 66;
   |             - binding `c` declared here
LL |         SomeStruct::<&'a u32>(&c); //~ ERROR
   |                               |
   |                               borrowed value does not live long enough
   |                               borrowed value does not live long enough
   |                               this usage requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/nll/user-annotations/adt-brace-structs.rs stdout ----
diff of stderr:

31    |
32 LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
33    |                                              -- lifetime `'a` defined here
- ...
+ LL |     let _closure = || {
+ LL |         let c = 66;
+    |             - binding `c` declared here
35 LL |         SomeStruct::<&'a u32> { t: &c };
37    |                                    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs/adt-brace-structs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-brace-structs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/user-annotations/adt-brace-structs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-brace-structs.rs:23:37
LL |     let c = 66;
   |         - binding `c` declared here
   |         - binding `c` declared here
LL |     SomeStruct::<&'static u32> { t: &c }; //~ ERROR
   |                                     |
   |                                     borrowed value does not live long enough
   |                                     borrowed value does not live long enough
   |                                     this usage requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-brace-structs.rs:28:32
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
   |         - binding `c` declared here
LL |     SomeStruct::<&'a u32> { t: &c }; //~ ERROR
   |                                |
   |                                borrowed value does not live long enough
   |                                borrowed value does not live long enough
   |                                this usage requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
error[E0597]: `c` does not live long enough
  --> fake-test-src-base/nll/user-annotations/adt-brace-structs.rs:38:36
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
---
To only update this specific test, also pass `--test-args static/issue-18118.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/static/issue-18118.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/issue-18118" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/issue-18118/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `p` does not live long enough
  --> fake-test-src-base/static/issue-18118.rs:4:9
LL |         let p = 3;
   |             - binding `p` declared here
   |             - binding `p` declared here
LL |         &p //~ ERROR `p` does not live long enough
   |         |
   |         borrowed value does not live long enough
   |         borrowed value does not live long enough
   |         using this value as a constant requires that `p` is borrowed for `'static`
LL |     };
   |     - `p` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
