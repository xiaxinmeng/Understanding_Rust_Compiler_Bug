plain
.................................................................................................... 3400/11392
.................................................................................................... 3500/11392
.................................................................................................... 3600/11392
..........i......................................................................................... 3700/11392
...............................................................................................F..F. 3800/11392
...........................F.F.F..........F......................................................... 3900/11392
.................................................................................................... 4100/11392
.................................................................................................... 4200/11392
.................................................................................................... 4300/11392
...........ii..................................................................................i.... 4400/11392
---
..................................................................................i.i............... 11300/11392
............................................................................................
failures:

---- [ui] ui/generic-associated-types/gat-trait-path-missing-lifetime.rs stdout ----

7    = note: `#[warn(incomplete_features)]` on by default
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
8    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
8    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
9 
- error[E0107]: wrong number of lifetime arguments: expected 1, found 0
-   --> $DIR/gat-trait-path-missing-lifetime.rs:11:20
+ error[E0107]: missing generics for associated type `X::Y`
+   --> $DIR/gat-trait-path-missing-lifetime.rs:5:8
12    |
- LL |   fn foo<'a, T1: X<Y = T1>>(t : T1) -> T1::Y<'a> {
-    |                    ^^^^^^ expected 1 lifetime argument
+ LL |   type Y<'a>;
+    |        ^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+   --> $DIR/gat-trait-path-missing-lifetime.rs:5:8
+    |
+ LL |   type Y<'a>;
+    |        ^ --
+ help: use angle brackets to add missing lifetime argument
+    |
+ LL |   type Y<'a><'a>;
15 
15 
- error[E0107]: wrong number of lifetime arguments: expected 1, found 0
-   --> $DIR/gat-trait-path-missing-lifetime.rs:11:20
+ error[E0107]: missing generics for associated type `X::Y`
+   --> $DIR/gat-trait-path-missing-lifetime.rs:5:8
18    |
- LL |   fn foo<'a, T1: X<Y = T1>>(t : T1) -> T1::Y<'a> {
-    |                    ^^^^^^ expected 1 lifetime argument
+ LL |   type Y<'a>;
+    |        ^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+   --> $DIR/gat-trait-path-missing-lifetime.rs:5:8
+    |
+ LL |   type Y<'a>;
+    |        ^ --
+ help: use angle brackets to add missing lifetime argument
+    |
+ LL |   type Y<'a><'a>;
21 
22 error: aborting due to 2 previous errors; 1 warning emitted
23 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-missing-lifetime/gat-trait-path-missing-lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/gat-trait-path-missing-lifetime.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/gat-trait-path-missing-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-missing-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-missing-lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-missing-lifetime.rs:1:12
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0107]: missing generics for associated type `X::Y`
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-missing-lifetime.rs:5:8
   |
LL |   type Y<'a>;
   |        ^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-missing-lifetime.rs:5:8
   |
LL |   type Y<'a>;
   |        ^ --
help: use angle brackets to add missing lifetime argument
   |
LL |   type Y<'a><'a>;


error[E0107]: missing generics for associated type `X::Y`
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-missing-lifetime.rs:5:8
   |
LL |   type Y<'a>;
   |        ^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-missing-lifetime.rs:5:8
   |
LL |   type Y<'a>;
   |        ^ --
help: use angle brackets to add missing lifetime argument
   |
LL |   type Y<'a><'a>;

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0107`.
For more information about this error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/generic-associated-types/gat-trait-path-parenthesised-args.rs stdout ----

27    |
28    = note: `#[warn(bare_trait_objects)]` on by default
29 
29 
- error[E0107]: wrong number of lifetime arguments: expected 1, found 0
-   --> $DIR/gat-trait-path-parenthesised-args.rs:8:27
+ error[E0107]: this associated type takes 1 lifetime argument but 0 lifetime arguments were supplied
+   --> $DIR/gat-trait-path-parenthesised-args.rs:5:8
32    |
- LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
-    |                           ^^^^^^^^^^^^^^ expected 1 lifetime argument
+ LL |   type Y<'a>;
+    |        ^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+   --> $DIR/gat-trait-path-parenthesised-args.rs:5:8
+    |
+ LL |   type Y<'a>;
+    |        ^ --
+ help: add missing lifetime argument
+    |
+ LL | fn foo<'a>(arg: Box<dyn X<Y('a'a) = &'a ()>>) {}
35 
35 
- error[E0107]: wrong number of type arguments: expected 0, found 1
-   --> $DIR/gat-trait-path-parenthesised-args.rs:8:29
+ error[E0107]: this associated type takes 0 type arguments but 1 type argument was supplied
+   --> $DIR/gat-trait-path-parenthesised-args.rs:5:8
38    |
- LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
-    |                             ^^ unexpected type argument
+ LL |     type Y<'a>;
+    |  ________^-
+    | |        expected 0 type arguments
+ LL | | }
+ LL | |
+ LL | |
+ LL | | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
+    | |_________________________________________- help: remove these generics
+    |
+ note: associated type defined here, with 0 type parameters
+   --> $DIR/gat-trait-path-parenthesised-args.rs:5:8
+    |
+ LL |   type Y<'a>;
41 
42 error: aborting due to 4 previous errors; 2 warnings emitted
43 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-parenthesised-args/gat-trait-path-parenthesised-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/gat-trait-path-parenthesised-args.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-parenthesised-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-parenthesised-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime in trait object type must be followed by `+`
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:8:29
   |
LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}

error: parenthesized generic arguments cannot be used in associated type constraints
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:8:27
   |
   |
LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}


warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:1:12
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:8:29
   |
LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
   |                             ^^ help: use `dyn`: `dyn 'a`
   = note: `#[warn(bare_trait_objects)]` on by default


error[E0107]: this associated type takes 1 lifetime argument but 0 lifetime arguments were supplied
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:5:8
   |
LL |   type Y<'a>;
   |        ^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:5:8
   |
LL |   type Y<'a>;
   |        ^ --
help: add missing lifetime argument
   |
LL | fn foo<'a>(arg: Box<dyn X<Y('a'a) = &'a ()>>) {}


error[E0107]: this associated type takes 0 type arguments but 1 type argument was supplied
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:5:8
   |
LL |     type Y<'a>;
   |  ________^-
   | |        expected 0 type arguments
LL | | }
LL | |
LL | |
LL | | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
   | |_________________________________________- help: remove these generics
   |
note: associated type defined here, with 0 type parameters
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:5:8
   |
LL |   type Y<'a>;

error: aborting due to 4 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0107`.
---

7    = note: `#[warn(incomplete_features)]` on by default
8    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
9 
- error[E0107]: wrong number of lifetime arguments: expected 1, found 0
-   --> $DIR/issue-76535.rs:37:33
+ error[E0107]: missing generics for associated type `SuperTrait::SubType`
12    |
12    |
- LL |     let sub: Box<dyn SuperTrait<SubType = SubStruct>> = Box::new(SuperStruct::new(0));
-    |                                 ^^^^^^^^^^^^^^^^^^^ expected 1 lifetime argument
+ LL |     type SubType<'a>: SubTrait;
+    |          ^^^^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type SubType<'a>: SubTrait;
+    |          ^^^^^^^ --
+ help: use angle brackets to add missing lifetime argument
+    |
+ LL |     type SubType<'a><'a>: SubTrait;
15 
15 
16 error[E0038]: the trait `SuperTrait` cannot be made into an object


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-76535/issue-76535.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-76535/issue-76535.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-76535.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-76535.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-76535" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-76535/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0107]: missing generics for associated type `SuperTrait::SubType`
  --> /checkout/src/test/ui/generic-associated-types/issue-76535.rs:7:10
   |
LL |     type SubType<'a>: SubTrait;
   |          ^^^^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type SubType<'a>: SubTrait;
   |          ^^^^^^^ --
help: use angle brackets to add missing lifetime argument
   |
LL |     type SubType<'a><'a>: SubTrait;


error[E0038]: the trait `SuperTrait` cannot be made into an object
   |
   |
LL |     let sub: Box<dyn SuperTrait<SubType = SubStruct>> = Box::new(SuperStruct::new(0));
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SuperTrait` cannot be made into an object
   |
   = help: consider moving `get_sub` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
LL | pub trait SuperTrait {
LL | pub trait SuperTrait {
   |           ---------- this trait cannot be made into an object...
...
LL |     fn get_sub<'a>(&'a mut self) -> Self::SubType<'a>;
   |                                     ^^^^^^^^^^^^^^^^^ ...because method `get_sub` references the `Self` type in its return type

error[E0038]: the trait `SuperTrait` cannot be made into an object
   |
   |
LL |     let sub: Box<dyn SuperTrait<SubType = SubStruct>> = Box::new(SuperStruct::new(0));
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SuperTrait` cannot be made into an object
   |
   = help: consider moving `get_sub` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
LL | pub trait SuperTrait {
LL | pub trait SuperTrait {
   |           ---------- this trait cannot be made into an object...
...
LL |     fn get_sub<'a>(&'a mut self) -> Self::SubType<'a>;
   |                                     ^^^^^^^^^^^^^^^^^ ...because method `get_sub` references the `Self` type in its return type
   = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn SuperTrait<SubType = SubStruct<'_>>>>` for `Box<SuperStruct>`
   = note: required by cast to type `Box<dyn SuperTrait<SubType = SubStruct<'_>>>`
error: aborting due to 3 previous errors; 1 warning emitted

Some errors have detailed explanations: E0038, E0107.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-79422.rs stdout ----
diff of stderr:

- error[E0107]: wrong number of lifetime arguments: expected 1, found 0
-   --> $DIR/issue-79422.rs:43:36
+ error[E0107]: missing generics for associated type `MapLike::VRefCont`
3    |
3    |
- LL |         as Box<dyn MapLike<u8, u8, VRefCont = dyn RefCont<'_, u8>>>;
-    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 lifetime argument
+ LL |     type VRefCont<'a>: RefCont<'a, V>;
+    |          ^^^^^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type VRefCont<'a>: RefCont<'a, V>;
+    |          ^^^^^^^^ --
+ help: use angle brackets to add missing lifetime argument
+    |
+ LL |     type VRefCont<'a><'a>: RefCont<'a, V>;
6 
6 
7 error[E0038]: the trait `MapLike` cannot be made into an object


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-79422/issue-79422.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-79422/issue-79422.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-79422.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-79422.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-79422" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-79422/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: missing generics for associated type `MapLike::VRefCont`
   |
   |
LL |     type VRefCont<'a>: RefCont<'a, V>;
   |          ^^^^^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type VRefCont<'a>: RefCont<'a, V>;
   |          ^^^^^^^^ --
help: use angle brackets to add missing lifetime argument
   |
LL |     type VRefCont<'a><'a>: RefCont<'a, V>;


error[E0038]: the trait `MapLike` cannot be made into an object
   |
   |
LL |         as Box<dyn MapLike<u8, u8, VRefCont = dyn RefCont<'_, u8>>>;
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MapLike` cannot be made into an object
   |
   = help: consider moving `get` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL | trait MapLike<K, V> {
   |       ------- this trait cannot be made into an object...
LL |     type VRefCont<'a>: RefCont<'a, V>;
LL |     fn get<'a>(&'a self, key: &K) -> Option<Self::VRefCont<'a>>;
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...because method `get` references the `Self` type in its return type

error[E0038]: the trait `MapLike` cannot be made into an object
   |
   |
LL |     let m = Box::new(std::collections::BTreeMap::<u8, u8>::new())
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MapLike` cannot be made into an object
   |
   = help: consider moving `get` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL | trait MapLike<K, V> {
   |       ------- this trait cannot be made into an object...
LL |     type VRefCont<'a>: RefCont<'a, V>;
LL |     fn get<'a>(&'a self, key: &K) -> Option<Self::VRefCont<'a>>;
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...because method `get` references the `Self` type in its return type
   = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>>>` for `Box<BTreeMap<u8, u8>>`
   = note: required by cast to type `Box<dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>>`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0038, E0107.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-80433.rs stdout ----
diff of stderr:

- error[E0107]: wrong number of lifetime arguments: expected 1, found 0
-   --> $DIR/issue-80433.rs:24:47
+ error[E0107]: missing generics for associated type `TestMut::Output`
3    |
3    |
- LL | fn test_simpler<'a>(dst: &'a mut impl TestMut<Output = &'a mut f32>)
-    |                                               ^^^^^^^^^^^^^^^^^^^^ expected 1 lifetime argument
+ LL |     type Output<'a>;
+    |          ^^^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Output<'a>;
+    |          ^^^^^^ --
+ help: use angle brackets to add missing lifetime argument
+    |
+ LL |     type Output<'a><'a>;
6 
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433/issue-80433.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-80433.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-80433.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: missing generics for associated type `TestMut::Output`
  --> /checkout/src/test/ui/generic-associated-types/issue-80433.rs:10:10
   |
LL |     type Output<'a>;
   |          ^^^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Output<'a>;
   |          ^^^^^^ --
help: use angle brackets to add missing lifetime argument
   |
LL |     type Output<'a><'a>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
---

7    = note: `#[warn(incomplete_features)]` on by default
8    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
9 
- error[E0107]: wrong number of lifetime arguments: expected 1, found 0
-   --> $DIR/trait-path-type-error-once-implemented.rs:9:29
+ error[E0107]: this associated type takes 1 lifetime argument but 0 lifetime arguments were supplied
12    |
12    |
- LL |   fn f2<'a>(arg : Box<dyn X<Y<1> = &'a ()>>) {}
-    |                             ^^^^^^^^^^^^^ expected 1 lifetime argument
+ LL |     type Y<'a>;
+    |          ^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Y<'a>;
+    |          ^ --
+ help: add missing lifetime argument
+    |
+ LL |   fn f2<'a>(arg : Box<dyn X<Y<'a1> = &'a ()>>) {}
15 
15 
- error[E0107]: wrong number of const arguments: expected 0, found 1
-   --> $DIR/trait-path-type-error-once-implemented.rs:9:31
+ error[E0107]: this associated type takes 0 const arguments but 1 const argument was supplied
18    |
18    |
- LL |   fn f2<'a>(arg : Box<dyn X<Y<1> = &'a ()>>) {}
-    |                               ^ unexpected const argument
+ LL |       type Y<'a>;
+    |  __________^-
+    | |          expected 0 const arguments
+ LL | | }
+ LL | |
+ LL | |
+ LL | | const _: () = {
+ LL | |   fn f2<'a>(arg : Box<dyn X<Y<1> = &'a ()>>) {}
+    | |________________________________- help: remove these generics
+    |
+ note: associated type defined here, with 0 const parameters
+    |
+    |
+ LL |     type Y<'a>;
21 
22 error: aborting due to 2 previous errors; 1 warning emitted
23 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-type-error-once-implemented/trait-path-type-error-once-implemented.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/parse/trait-path-type-error-once-implemented.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/parse/trait-path-type-error-once-implemented.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-type-error-once-implemented" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-type-error-once-implemented/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0107]: this associated type takes 1 lifetime argument but 0 lifetime arguments were supplied
   |
   |
LL |     type Y<'a>;
   |          ^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Y<'a>;
   |          ^ --
help: add missing lifetime argument
   |
LL |   fn f2<'a>(arg : Box<dyn X<Y<'a1> = &'a ()>>) {}


error[E0107]: this associated type takes 0 const arguments but 1 const argument was supplied
   |
   |
LL |       type Y<'a>;
   |  __________^-
   | |          expected 0 const arguments
LL | | }
LL | |
LL | |
LL | | const _: () = {
LL | |   fn f2<'a>(arg : Box<dyn X<Y<1> = &'a ()>>) {}
   | |________________________________- help: remove these generics
   |
note: associated type defined here, with 0 const parameters
   |
   |
LL |     type Y<'a>;

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0107`.
---
test result: FAILED. 11294 passed; 6 failed; 92 ignored; 0 measured; 0 filtered out; finished in 134.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:04
