plain
.................................................................................................... 3400/12541
.................................................................................................... 3500/12541
.................................................................................................... 3600/12541
.i.........i.........i.............................................................................. 3700/12541
..................................F.........................................ii...................... 3800/12541
........F..F........................................................................................ 3900/12541
.................................................................................................... 4100/12541
.................................................................................................... 4200/12541
.................................................................................................... 4300/12541
.................................................................................................... 4400/12541
---

- error[E0261]: use of undeclared lifetime name `'a`
-   --> $DIR/feature-gate-in_band_lifetimes.rs:50:14
-    |
- LL | impl MyTrait<'a> for Y<&'a u8> {
-    |     -        ^^ undeclared lifetime
-    |     |
-    |     help: consider introducing lifetime `'a` here: `<'a>`
-    |
-    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- error[E0261]: use of undeclared lifetime name `'a`
-   --> $DIR/feature-gate-in_band_lifetimes.rs:50:25
-    |
-    |
- LL | impl MyTrait<'a> for Y<&'a u8> {
-    |     -                   ^^ undeclared lifetime
-    |     |
-    |     help: consider introducing lifetime `'a` here: `<'a>`
-    |
-    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
- error[E0261]: use of undeclared lifetime name `'a`
-   --> $DIR/feature-gate-in_band_lifetimes.rs:53:31
-    |
-    |
- LL |     fn my_lifetime(&self) -> &'a u8 { self.0 }
-    |                               ^^ undeclared lifetime
-    |
-    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
- help: consider introducing lifetime `'a` here
-    |
- LL | impl<'a> MyTrait<'a> for Y<&'a u8> {
-    |     ++++
- help: consider introducing lifetime `'a` here
-    |
- LL |     fn my_lifetime<'a>(&self) -> &'a u8 { self.0 }
- 
- error[E0261]: use of undeclared lifetime name `'b`
-   --> $DIR/feature-gate-in_band_lifetimes.rs:55:27
-    |
-    |
- LL |     fn any_lifetime() -> &'b u8 { &0 }
-    |                           ^^ undeclared lifetime
-    |
-    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
- help: consider introducing lifetime `'b` here
-    |
- LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
-    |     ++++
- help: consider introducing lifetime `'b` here
-    |
- LL |     fn any_lifetime<'b>() -> &'b u8 { &0 }
- 
- error[E0261]: use of undeclared lifetime name `'b`
-   --> $DIR/feature-gate-in_band_lifetimes.rs:57:27
-    |
-    |
- LL |     fn borrowed_lifetime(&'b self) -> &'b u8 { &*self.0 }
-    |                           ^^ undeclared lifetime
-    |
-    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
- help: consider introducing lifetime `'b` here
-    |
- LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
-    |     ++++
- help: consider introducing lifetime `'b` here
-    |
- LL |     fn borrowed_lifetime<'b>(&'b self) -> &'b u8 { &*self.0 }
- 
- error[E0261]: use of undeclared lifetime name `'b`
-   --> $DIR/feature-gate-in_band_lifetimes.rs:57:40
-    |
-    |
- LL |     fn borrowed_lifetime(&'b self) -> &'b u8 { &*self.0 }
-    |                                        ^^ undeclared lifetime
-    |
-    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
- help: consider introducing lifetime `'b` here
-    |
- LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
-    |     ++++
- help: consider introducing lifetime `'b` here
-    |
- LL |     fn borrowed_lifetime<'b>(&'b self) -> &'b u8 { &*self.0 }
- 
85 error[E0261]: use of undeclared lifetime name `'x`
86   --> $DIR/feature-gate-in_band_lifetimes.rs:3:12
87    |
87    |

177    |
178 LL |     fn inner<'a>(&self) -> &'a u8 {
+ 
+ 
+ error[E0261]: use of undeclared lifetime name `'a`
+    |
+    |
+ LL | impl MyTrait<'a> for Y<&'a u8> {
+    |     -        ^^ undeclared lifetime
+    |     |
+    |     help: consider introducing lifetime `'a` here: `<'a>`
+    |
+    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
+ 
+ error[E0261]: use of undeclared lifetime name `'a`
+    |
+    |
+ LL | impl MyTrait<'a> for Y<&'a u8> {
+    |     -                   ^^ undeclared lifetime
+    |     |
+    |     help: consider introducing lifetime `'a` here: `<'a>`
+    |
+    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
+ 
+ error[E0261]: use of undeclared lifetime name `'a`
+    |
+    |
+ LL |     fn my_lifetime(&self) -> &'a u8 { self.0 }
+    |                               ^^ undeclared lifetime
+    |
+    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
+ help: consider introducing lifetime `'a` here
+    |
+ LL | impl<'a> MyTrait<'a> for Y<&'a u8> {
+    |     ++++
+ help: consider introducing lifetime `'a` here
+    |
+ LL |     fn my_lifetime<'a>(&self) -> &'a u8 { self.0 }
+ 
+ 
+ error[E0261]: use of undeclared lifetime name `'b`
+    |
+    |
+ LL |     fn any_lifetime() -> &'b u8 { &0 }
+    |                           ^^ undeclared lifetime
+    |
+    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
+ help: consider introducing lifetime `'b` here
+    |
+ LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
+    |     ++++
+ help: consider introducing lifetime `'b` here
+    |
+ LL |     fn any_lifetime<'b>() -> &'b u8 { &0 }
+ 
+ 
+ error[E0261]: use of undeclared lifetime name `'b`
+    |
+    |
+ LL |     fn borrowed_lifetime(&'b self) -> &'b u8 { &*self.0 }
+    |                           ^^ undeclared lifetime
+    |
+    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
+ help: consider introducing lifetime `'b` here
+    |
+ LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
+    |     ++++
+ help: consider introducing lifetime `'b` here
+    |
+ LL |     fn borrowed_lifetime<'b>(&'b self) -> &'b u8 { &*self.0 }
+ 
+ 
+ error[E0261]: use of undeclared lifetime name `'b`
+    |
+    |
+ LL |     fn borrowed_lifetime(&'b self) -> &'b u8 { &*self.0 }
+    |                                        ^^ undeclared lifetime
+    |
+    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
+ help: consider introducing lifetime `'b` here
+    |
+ LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
+    |     ++++
+ help: consider introducing lifetime `'b` here
+    |
+ LL |     fn borrowed_lifetime<'b>(&'b self) -> &'b u8 { &*self.0 }
180 
181 error[E0261]: use of undeclared lifetime name `'b`
182   --> $DIR/feature-gate-in_band_lifetimes.rs:43:27

---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-in_band_lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-in_band_lifetimes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-in_band_lifetimes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0261]: use of undeclared lifetime name `'x`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:3:12
   |
LL | fn foo(x: &'x u8) -> &'x u8 { x }
   |       -    ^^ undeclared lifetime
   |       |
   |       help: consider introducing lifetime `'x` here: `<'x>`
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
error[E0261]: use of undeclared lifetime name `'x`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:3:23
   |
   |
LL | fn foo(x: &'x u8) -> &'x u8 { x }
   |       -               ^^ undeclared lifetime
   |       |
   |       help: consider introducing lifetime `'x` here: `<'x>`
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:15:12
   |
   |
LL | impl<'a> X<'b> {
   |      -     ^^ undeclared lifetime
   |      |
   |      help: consider introducing lifetime `'b` here: `'b,`
error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:17:27
   |
   |
LL |     fn inner_2(&self) -> &'b u8 {
   |                           ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'b` here
   |
LL | impl<'b, 'a> X<'b> {
   |      +++
help: consider introducing lifetime `'b` here
   |
LL |     fn inner_2<'b>(&self) -> &'b u8 {

error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:23:8
   |
   |
LL | impl X<'b> {
   |     -  ^^ undeclared lifetime
   |     |
   |     help: consider introducing lifetime `'b` here: `<'b>`
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:25:27
   |
   |
LL |     fn inner_3(&self) -> &'b u8 {
   |                           ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'b` here
   |
LL | impl<'b> X<'b> {
   |     ++++
help: consider introducing lifetime `'b` here
   |
LL |     fn inner_3<'b>(&self) -> &'b u8 {

error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:33:9
   |
   |
LL | impl Y<&'a u8> {
   |     -   ^^ undeclared lifetime
   |     |
   |     help: consider introducing lifetime `'a` here: `<'a>`
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:35:25
   |
LL |     fn inner(&self) -> &'a u8 {
LL |     fn inner(&self) -> &'a u8 {
   |                         ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'a` here
   |
LL | impl<'a> Y<&'a u8> {
   |     ++++
help: consider introducing lifetime `'a` here
   |
LL |     fn inner<'a>(&self) -> &'a u8 {

error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:50:14
   |
   |
LL | impl MyTrait<'a> for Y<&'a u8> {
   |     -        ^^ undeclared lifetime
   |     |
   |     help: consider introducing lifetime `'a` here: `<'a>`
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:50:25
   |
   |
LL | impl MyTrait<'a> for Y<&'a u8> {
   |     -                   ^^ undeclared lifetime
   |     |
   |     help: consider introducing lifetime `'a` here: `<'a>`
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:53:31
   |
   |
LL |     fn my_lifetime(&self) -> &'a u8 { self.0 }
   |                               ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'a` here
   |
LL | impl<'a> MyTrait<'a> for Y<&'a u8> {
   |     ++++
help: consider introducing lifetime `'a` here
   |
LL |     fn my_lifetime<'a>(&self) -> &'a u8 { self.0 }

error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:55:27
   |
   |
LL |     fn any_lifetime() -> &'b u8 { &0 }
   |                           ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'b` here
   |
LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
   |     ++++
help: consider introducing lifetime `'b` here
   |
LL |     fn any_lifetime<'b>() -> &'b u8 { &0 }

error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:57:27
   |
   |
LL |     fn borrowed_lifetime(&'b self) -> &'b u8 { &*self.0 }
   |                           ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'b` here
   |
LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
   |     ++++
help: consider introducing lifetime `'b` here
   |
LL |     fn borrowed_lifetime<'b>(&'b self) -> &'b u8 { &*self.0 }

error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:57:40
   |
   |
LL |     fn borrowed_lifetime(&'b self) -> &'b u8 { &*self.0 }
   |                                        ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'b` here
   |
LL | impl<'b> MyTrait<'a> for Y<&'a u8> {
   |     ++++
help: consider introducing lifetime `'b` here
   |
LL |     fn borrowed_lifetime<'b>(&'b self) -> &'b u8 { &*self.0 }

error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:43:27
   |
   |
LL |     fn any_lifetime() -> &'b u8;
   |                           ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'b` here
   |
LL | trait MyTrait<'b, 'a> {
   |               +++
help: consider introducing lifetime `'b` here
   |
LL |     fn any_lifetime<'b>() -> &'b u8;

error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:45:27
   |
   |
LL |     fn borrowed_lifetime(&'b self) -> &'b u8;
   |                           ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'b` here
   |
LL | trait MyTrait<'b, 'a> {
   |               +++
help: consider introducing lifetime `'b` here
   |
LL |     fn borrowed_lifetime<'b>(&'b self) -> &'b u8;

error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/feature-gates/feature-gate-in_band_lifetimes.rs:45:40
   |
   |
LL |     fn borrowed_lifetime(&'b self) -> &'b u8;
   |                                        ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'b` here
   |
LL | trait MyTrait<'b, 'a> {
   |               +++
help: consider introducing lifetime `'b` here
   |
LL |     fn borrowed_lifetime<'b>(&'b self) -> &'b u8;

error: aborting due to 17 previous errors

For more information about this error, try `rustc --explain E0261`.
---

67    = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
68    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
69 
- error[E0183]: manual implementations of `Fn` are experimental
-   --> $DIR/feature-gate-unboxed-closures-manual-impls.rs:9:1
-    |
- LL | impl Fn<()> for Foo {
-    | ^^^^^^^^^^^^^^^^^^^ manual implementations of `Fn` are experimental
-    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
+ error: aborting due to 8 previous errors
77 
77 
- error[E0183]: manual implementations of `FnMut` are experimental
-   --> $DIR/feature-gate-unboxed-closures-manual-impls.rs:23:1
-    |
- LL | impl FnMut<()> for Bar {
-    | ^^^^^^^^^^^^^^^^^^^^^^ manual implementations of `FnMut` are experimental
-    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
- 
- 
- error[E0183]: manual implementations of `FnOnce` are experimental
-   --> $DIR/feature-gate-unboxed-closures-manual-impls.rs:16:1
-    |
- LL | impl FnOnce() for Foo1 {
-    | ^^^^^^^^^^^^^^^^^^^^^^ manual implementations of `FnOnce` are experimental
-    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
- 
- 
- error[E0183]: manual implementations of `FnOnce` are experimental
-   --> $DIR/feature-gate-unboxed-closures-manual-impls.rs:30:1
-    |
- LL | impl FnOnce<()> for Baz {
-    | ^^^^^^^^^^^^^^^^^^^^^^^ manual implementations of `FnOnce` are experimental
-    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
- 
- error: aborting due to 12 previous errors
- 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-unboxed-closures-manual-impls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:12:12
   |
LL |     extern "rust-call" fn call(self, args: ()) -> () {}
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:19:12
   |
LL |     extern "rust-call" fn call_once(self, args: ()) -> () {}
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:26:12
   |
LL |     extern "rust-call" fn call_mut(&self, args: ()) -> () {}
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:33:12
   |
LL |     extern "rust-call" fn call_once(&self, args: ()) -> () {}
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
   |
   |
LL | impl Fn<()> for Foo {
   |      ^^^^^^ help: use parenthetical notation instead: `Fn() -> ()`
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error[E0229]: associated type bindings are not allowed here
error[E0229]: associated type bindings are not allowed here
  --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:16:6
   |
LL | impl FnOnce() for Foo1 {
   |      ^^^^^^^^ associated type not allowed here

error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
   |
   |
LL | impl FnMut<()> for Bar {
   |      ^^^^^^^^^ help: use parenthetical notation instead: `FnMut() -> ()`
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
   |
   |
LL | impl FnOnce<()> for Baz {
   |      ^^^^^^^^^^ help: use parenthetical notation instead: `FnOnce() -> ()`
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error: aborting due to 8 previous errors
---

16    = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
17    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
18 
- error[E0183]: manual implementations of `FnOnce` are experimental
-   --> $DIR/feature-gate-unboxed-closures.rs:5:1
-    |
- LL | impl FnOnce<(u32, u32)> for Test {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ manual implementations of `FnOnce` are experimental
-    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
+ error: aborting due to 2 previous errors
26 
- error: aborting due to 3 previous errors
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-unboxed-closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures.rs:10:12
   |
LL |     extern "rust-call" fn call_once(self, (a, b): (u32, u32)) -> u32 {
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
   |
   |
LL | impl FnOnce<(u32, u32)> for Test {
   |      ^^^^^^^^^^^^^^^^^^ help: use parenthetical notation instead: `FnOnce(u32, u32) -> ()`
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error: aborting due to 2 previous errors
---
25 
- error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
-   --> $DIR/issue-3214.rs:6:10
-    |
- LL |     impl<T> Drop for Foo<T> {
+ error: aborting due to 2 previous errors
31 
- error: aborting due to 3 previous errors
- 
---
To only update this specific test, also pass `--test-args issues/issue-3214.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3214.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3214" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3214/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0401]: can't use generic parameters from outer function
  --> /checkout/src/test/ui/issues/issue-3214.rs:3:12
   |
LL | fn foo<T>() {
   |    --- - type parameter from outer function
   |    |
   |    try adding a local generic parameter in this method instead
LL |     struct Foo {
LL |         x: T, //~ ERROR can't use generic parameters from outer function
   |            ^ use of generic parameter from outer function

error[E0107]: this struct takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     impl<T> Drop for Foo<T> {
   |                      ^^^--- help: remove these generics
   |                      expected 0 generic arguments
   |
note: struct defined here, with 0 generic parameters
  --> /checkout/src/test/ui/issues/issue-3214.rs:2:12
---
+    |
+ LL |         fn foo() -> Self;
+    |         ----------------- `foo` from trait
+ ...
+ LL |     impl ::bar::B for f32 { fn foo() -> f32 { 1.0 } }
+    |     ^^^^^^^^^^^^^^^^^^^^^ missing `foo` in implementation
+ 
157 error[E0624]: associated function `bar` is private
159    |


199 LL |         ::bar::baz::A.bar2();
201 
- error: aborting due to 18 previous errors
+ error: aborting due to 19 previous errors
203 
---
To only update this specific test, also pass `--test-args privacy/privacy1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: module `baz` is private
   |
   |
LL |         use bar::baz::{foo, bar};
   |                  ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         use bar::baz::{foo, bar};
   |                  ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         use bar::baz;
   |                  ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `i` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:165:20
   |
LL |     use self::foo::i::A; //~ ERROR: module `i` is private
   |                    ^ private module
note: the module `i` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:170:9
   |
LL |         mod i {
LL |         mod i {
   |         ^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::A::foo();   //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::A::bar();   //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::A.foo2();   //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::A.bar2();   //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: trait `B` is private
   |
   |
LL |         ::bar::B::foo();        //~ ERROR: trait `B` is private
   |                ^ private trait
   |
note: the trait `B` is defined here
   |
LL |     trait B {
   |     ^^^^^^^


error[E0603]: function `epriv` is private
   |
   |
LL |             ::bar::epriv(); //~ ERROR: function `epriv` is private
   |
   |
note: the function `epriv` is defined here
   |
   |
LL |         fn epriv();


error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::foo(); //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::bar(); //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: trait `B` is private
   |
   |
LL |     impl ::bar::B for f32 { fn foo() -> f32 { 1.0 } }
   |                 ^ private trait
   |
note: the trait `B` is defined here
   |
LL |     trait B {
   |     ^^^^^^^


error[E0046]: not all trait items implemented, missing: `foo`
  --> /checkout/src/test/ui/privacy/privacy1.rs:157:5
   |
LL |         fn foo() -> Self;
   |         ----------------- `foo` from trait
...
LL |     impl ::bar::B for f32 { fn foo() -> f32 { 1.0 } }
   |     ^^^^^^^^^^^^^^^^^^^^^ missing `foo` in implementation

error[E0624]: associated function `bar` is private
   |
LL |             fn bar() {}
   |             -------- private associated function defined here
...
...
LL |         self::baz::A::bar(); //~ ERROR: associated function `bar` is private


error[E0624]: associated function `bar` is private
   |
LL |         fn bar() {}
   |         -------- private associated function defined here
...
...
LL |     bar::A::bar(); //~ ERROR: associated function `bar` is private


error[E0624]: associated function `bar` is private
   |
LL |         fn bar() {}
   |         -------- private associated function defined here
...
...
LL |         ::bar::A::bar();        //~ ERROR: associated function `bar` is private


error[E0624]: associated function `bar` is private
   |
LL |             fn bar() {}
   |             -------- private associated function defined here
...
...
LL |         ::bar::baz::A::bar();   //~ ERROR: module `baz` is private


error[E0624]: associated function `bar2` is private
   |
LL |             fn bar2(&self) {}
   |             -------------- private associated function defined here
...
...
LL |         ::bar::baz::A.bar2();   //~ ERROR: module `baz` is private

error: aborting due to 19 previous errors

Some errors have detailed explanations: E0046, E0603, E0624.
---

- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:16:13
-    |
- LL | impl Trait1<usize> for S {
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:20:13
-    |
- LL | impl Trait1<isize> for S {
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:24:13
-    |
- LL | impl Trait2<usize> for S {
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- 
25 warning: use of deprecated struct `unstable_generic_param::Struct4`: test
27    |

241    |            ^^^^^
242 
242 
243 error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:35:20
+   --> $DIR/generics-default-stability.rs:16:13
245    |
- LL |     let _: Struct1<isize> = Struct1 { field: 1 };
-    |                    ^^^^^
+ LL | impl Trait1<usize> for S {
248    |
249    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
250 


251 error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:39:20
+   --> $DIR/generics-default-stability.rs:20:13
253    |
- LL |     let _: Struct1<usize> = STRUCT1;
-    |                    ^^^^^
+ LL | impl Trait1<isize> for S {
256    |
257    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
258 


259 error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:40:20
+   --> $DIR/generics-default-stability.rs:24:13
261    |
- LL |     let _: Struct1<isize> = Struct1 { field: 0 };
-    |                    ^^^^^
+ LL | impl Trait2<usize> for S {
264    |
265    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
266 


- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:69:27
-    |
- LL |     let _: Struct3<isize, usize> = STRUCT3;
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:71:27
-    |
- LL |     let _: Struct3<isize, isize> = Struct3 { field1: 0, field2: 0 };
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:72:27
-    |
- LL |     let _: Struct3<usize, usize> = Struct3 { field1: 0, field2: 0 };
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:96:20
-    |
- LL |     let _: Struct5<isize> = Struct5 { field: 1 };
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:102:20
-    |
- LL |     let _: Struct5<usize> = STRUCT5;
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:104:20
-    |
- LL |     let _: Struct5<isize> = Struct5 { field: 0 };
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:112:19
-    |
- LL |     let _: Alias1<isize> = Alias1::Some(1);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:116:19
-    |
- LL |     let _: Alias1<usize> = ALIAS1;
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:117:19
-    |
- LL |     let _: Alias1<isize> = Alias1::Some(0);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:145:26
-    |
- LL |     let _: Alias3<isize, usize> = ALIAS3;
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:147:26
-    |
- LL |     let _: Alias3<isize, isize> = Alias3::Ok(0);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:148:26
-    |
- LL |     let _: Alias3<usize, usize> = Alias3::Ok(0);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:170:19
-    |
- LL |     let _: Alias5<isize> = Alias5::Some(1);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:175:19
-    |
- LL |     let _: Alias5<usize> = ALIAS5;
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:177:19
-    |
- LL |     let _: Alias5<isize> = Alias5::Some(0);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:184:18
-    |
- LL |     let _: Enum1<isize> = Enum1::Some(1);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:188:18
-    |
- LL |     let _: Enum1<usize> = ENUM1;
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:189:18
-    |
- LL |     let _: Enum1<isize> = Enum1::Some(0);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:217:25
-    |
- LL |     let _: Enum3<isize, usize> = ENUM3;
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:219:25
-    |
- LL |     let _: Enum3<isize, isize> = Enum3::Ok(0);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:220:25
-    |
- LL |     let _: Enum3<usize, usize> = Enum3::Ok(0);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:242:18
-    |
- LL |     let _: Enum5<isize> = Enum5::Some(1);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:247:18
-    |
- LL |     let _: Enum5<usize> = ENUM5;
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_default'
- error[E0658]: use of unstable library feature 'unstable_default'
-   --> $DIR/generics-default-stability.rs:249:18
-    |
- LL |     let _: Enum5<isize> = Enum5::Some(0);
-    |
-    = help: add `#![feature(unstable_default)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'box_alloc_param'
- error[E0658]: use of unstable library feature 'box_alloc_param'
-   --> $DIR/generics-default-stability.rs:256:24
-    |
- LL |     let _: Box1<isize, System> = Box1::new(1);
-    |
-    |
-    = help: add `#![feature(box_alloc_param)]` to the crate attributes to enable
- 
- warning: use of deprecated field `unstable_generic_param::Struct4::field`: test
-   --> $DIR/generics-default-stability.rs:83:39
-    |
- LL |     let _: Struct4<isize> = Struct4 { field: 1 };
- 
- 
- warning: use of deprecated field `unstable_generic_param::Struct4::field`: test
-   --> $DIR/generics-default-stability.rs:90:39
-    |
- LL |     let _: Struct4<isize> = Struct4 { field: 0 };
- 
- 
- warning: use of deprecated field `unstable_generic_param::Struct5::field`: test
-   --> $DIR/generics-default-stability.rs:96:39
-    |
- LL |     let _: Struct5<isize> = Struct5 { field: 1 };
- 
- 
- warning: use of deprecated field `unstable_generic_param::Struct5::field`: test
-   --> $DIR/generics-default-stability.rs:104:39
-    |
- LL |     let _: Struct5<isize> = Struct5 { field: 0 };
- 
- error: aborting due to 31 previous errors; 40 warnings emitted
+ error: aborting due to 3 previous errors; 36 warnings emitted
492 
---
To only update this specific test, also pass `--test-args stability-attribute/generics-default-stability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/generics-default-stability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/generics-default-stability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/generics-default-stability/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated struct `unstable_generic_param::Struct4`: test
   |
   |
LL |     let _: Struct4<isize> = Struct4 { field: 1 };
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated struct `unstable_generic_param::Struct4`: test
   |
   |
LL |     let _: Struct4<isize> = Struct4 { field: 1 };


warning: use of deprecated struct `unstable_generic_param::Struct4`: test
   |
   |
LL |     let _: Struct4 = STRUCT4; //~ use of deprecated struct `unstable_generic_param::Struct4`: test [deprecated]


warning: use of deprecated struct `unstable_generic_param::Struct4`: test
   |
   |
LL |     let _: Struct4<usize> = STRUCT4; //~ use of deprecated struct `unstable_generic_param::Struct4`: test [deprecated]


warning: use of deprecated struct `unstable_generic_param::Struct4`: test
   |
   |
LL |     let _: Struct4<isize> = Struct4 { field: 0 };


warning: use of deprecated struct `unstable_generic_param::Struct4`: test
   |
   |
LL |     let _: Struct4<isize> = Struct4 { field: 0 };


warning: use of deprecated struct `unstable_generic_param::Struct5`: test
   |
   |
LL |     let _: Struct5<isize> = Struct5 { field: 1 }; //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated struct `unstable_generic_param::Struct5`: test
   |
   |
LL |     let _: Struct5<isize> = Struct5 { field: 1 }; //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated struct `unstable_generic_param::Struct5`: test
   |
   |
LL |     let _: Struct5 = STRUCT5; //~ use of deprecated struct `unstable_generic_param::Struct5`: test [deprecated]


warning: use of deprecated struct `unstable_generic_param::Struct5`: test
   |
   |
LL |     let _: Struct5<usize> = STRUCT5; //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated struct `unstable_generic_param::Struct5`: test
   |
   |
LL |     let _: Struct5<isize> = Struct5 { field: 0 }; //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated struct `unstable_generic_param::Struct5`: test
   |
   |
LL |     let _: Struct5<isize> = Struct5 { field: 0 }; //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated type alias `unstable_generic_param::Alias4`: test
   |
   |
LL |     let _: Alias4<isize> = Alias4::Some(1);


warning: use of deprecated type alias `unstable_generic_param::Alias4`: test
   |
   |
LL |     let _: Alias4<isize> = Alias4::Some(1);


warning: use of deprecated type alias `unstable_generic_param::Alias4`: test
   |
   |
LL |     let _: Alias4 = ALIAS4; //~ use of deprecated type alias `unstable_generic_param::Alias4`: test [deprecated]


warning: use of deprecated type alias `unstable_generic_param::Alias4`: test
   |
   |
LL |     let _: Alias4<usize> = ALIAS4; //~ use of deprecated type alias `unstable_generic_param::Alias4`: test [deprecated]


warning: use of deprecated type alias `unstable_generic_param::Alias4`: test
   |
   |
LL |     let _: Alias4<isize> = Alias4::Some(0);


warning: use of deprecated type alias `unstable_generic_param::Alias4`: test
   |
   |
LL |     let _: Alias4<isize> = Alias4::Some(0);


warning: use of deprecated type alias `unstable_generic_param::Alias5`: test
   |
   |
LL |     let _: Alias5<isize> = Alias5::Some(1); //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated type alias `unstable_generic_param::Alias5`: test
   |
   |
LL |     let _: Alias5<isize> = Alias5::Some(1); //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated type alias `unstable_generic_param::Alias5`: test
   |
   |
LL |     let _: Alias5 = ALIAS5; //~ use of deprecated type alias `unstable_generic_param::Alias5`: test [deprecated]


warning: use of deprecated type alias `unstable_generic_param::Alias5`: test
   |
   |
LL |     let _: Alias5<usize> = ALIAS5; //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated type alias `unstable_generic_param::Alias5`: test
   |
   |
LL |     let _: Alias5<isize> = Alias5::Some(0); //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated type alias `unstable_generic_param::Alias5`: test
   |
   |
LL |     let _: Alias5<isize> = Alias5::Some(0); //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated variant `unstable_generic_param::Enum4::Some`: test
   |
   |
LL |     let _: Enum4<isize> = Enum4::Some(1);


warning: use of deprecated enum `unstable_generic_param::Enum4`: test
   |
   |
LL |     let _: Enum4<isize> = Enum4::Some(1);


warning: use of deprecated enum `unstable_generic_param::Enum4`: test
   |
   |
LL |     let _: Enum4 = ENUM4; //~ use of deprecated enum `unstable_generic_param::Enum4`: test [deprecated]


warning: use of deprecated enum `unstable_generic_param::Enum4`: test
   |
   |
LL |     let _: Enum4<usize> = ENUM4; //~ use of deprecated enum `unstable_generic_param::Enum4`: test [deprecated]


warning: use of deprecated variant `unstable_generic_param::Enum4::Some`: test
   |
   |
LL |     let _: Enum4<isize> = Enum4::Some(0);


warning: use of deprecated enum `unstable_generic_param::Enum4`: test
   |
   |
LL |     let _: Enum4<isize> = Enum4::Some(0);


warning: use of deprecated variant `unstable_generic_param::Enum5::Some`: test
   |
   |
LL |     let _: Enum5<isize> = Enum5::Some(1); //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated enum `unstable_generic_param::Enum5`: test
   |
   |
LL |     let _: Enum5<isize> = Enum5::Some(1); //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated enum `unstable_generic_param::Enum5`: test
   |
   |
LL |     let _: Enum5 = ENUM5; //~ use of deprecated enum `unstable_generic_param::Enum5`: test [deprecated]


warning: use of deprecated enum `unstable_generic_param::Enum5`: test
   |
   |
LL |     let _: Enum5<usize> = ENUM5; //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated variant `unstable_generic_param::Enum5::Some`: test
   |
   |
LL |     let _: Enum5<isize> = Enum5::Some(0); //~ ERROR use of unstable library feature 'unstable_default'


warning: use of deprecated enum `unstable_generic_param::Enum5`: test
   |
   |
LL |     let _: Enum5<isize> = Enum5::Some(0); //~ ERROR use of unstable library feature 'unstable_default'

error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:16:13
   |
   |
LL | impl Trait1<usize> for S { //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_default'
error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:20:13
   |
LL | impl Trait1<isize> for S { //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_default'
error[E0658]: use of unstable library feature 'unstable_default'
  --> /checkout/src/test/ui/stability-attribute/generics-default-stability.rs:24:13
   |
LL | impl Trait2<usize> for S { //~ ERROR use of unstable library feature 'unstable_default'
   |
   = help: add `#![feature(unstable_default)]` to the crate attributes to enable

error: aborting due to 3 previous errors; 36 warnings emitted
---
test result: FAILED. 12416 passed; 6 failed; 119 ignored; 0 measured; 0 filtered out; finished in 119.26s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:51
