plain
.................................................................................................... 9200/11534
.................................................................................................... 9300/11534
.................................................................................................... 9400/11534
..........................................................................i......i.................. 9500/11534
............................F....................................................................... 9600/11534
.............iiiiiii..iiiiii.i...................................................................... 9700/11534
.................................................................................................... 9900/11534
.............................F...................................................................... 10000/11534
.................................................................................................... 10100/11534
.................................................................................................... 10200/11534
---
failures:

---- [ui] ui/anon-params/anon-params-edition-hygiene.rs stdout ----
normalized stderr:
warning: anonymous parameters are deprecated and will be removed in the next edition.
  --> $DIR/anon-params-edition-hygiene.rs:11:1
   |
LL | generate_trait_2015!(u8);
   |
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-edition-hygiene/anon-params-edition-hygiene.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args anon-params/anon-params-edition-hygiene.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/anon-params/anon-params-edition-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-edition-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-edition-hygiene/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL | generate_trait_2015!(u8);
   |
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
---

---- [ui] ui/feature-gates/feature-gate-object_safe_for_dispatch.rs stdout ----
diff of stderr:

+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+   --> $DIR/feature-gate-object_safe_for_dispatch.rs:15:19
+    |
+ LL |     fn foo(&self, &Self);
+    |                   ^^^^^ help: try naming the parameter or explicitly ignoring it: `_: &Self`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
1 error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
2   --> $DIR/feature-gate-object_safe_for_dispatch.rs:18:38

78    |       |
78    |       |
79    |       this trait cannot be made into an object...
- error: aborting due to 5 previous errors
+ error: aborting due to 5 previous errors; 1 warning emitted
82 
83 For more information about this error, try `rustc --explain E0038`.
83 For more information about this error, try `rustc --explain E0038`.
84 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch/feature-gate-object_safe_for_dispatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-object_safe_for_dispatch.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition.
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:15:19
   |
LL |     fn foo(&self, &Self);
   |                   ^^^^^ help: try naming the parameter or explicitly ignoring it: `_: &Self`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:18:38
   |
LL | fn takes_non_object_safe_ref<T>(obj: &dyn NonObjectSafe1) {
   |                                      ^^^^^^^^^^^^^^^^^^^ `NonObjectSafe1` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:4:23
   |
LL | trait NonObjectSafe1: Sized {}
   |       --------------  ^^^^^ ...because it requires `Self: Sized`
   |       |
   |       this trait cannot be made into an object...

error[E0038]: the trait `NonObjectSafe2` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:22:36
   |
LL | fn return_non_object_safe_ref() -> &'static dyn NonObjectSafe2 {
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe2` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:7:8
   |
LL | trait NonObjectSafe2 {
   |       -------------- this trait cannot be made into an object...
LL |     fn static_fn() {}
   |        ^^^^^^^^^ ...because associated function `static_fn` has no `self` parameter
help: consider turning `static_fn` into a method by giving it a `&self` argument
   |
LL |     fn static_fn(&self) {}
   |                  ^^^^^
help: alternatively, consider constraining `static_fn` so it does not apply to trait objects
   |
LL |     fn static_fn() where Self: Sized {}


error[E0038]: the trait `NonObjectSafe3` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:27:35
   |
LL | fn takes_non_object_safe_box(obj: Box<dyn NonObjectSafe3>) {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe3` cannot be made into an object
   |
   = help: consider moving `foo` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:11:8
   |
LL | trait NonObjectSafe3 {
   |       -------------- this trait cannot be made into an object...
LL |     fn foo<T>(&self);
   |        ^^^ ...because method `foo` has generic type parameters

error[E0038]: the trait `NonObjectSafe4` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:31:35
   |
LL | fn return_non_object_safe_rc() -> std::rc::Rc<dyn NonObjectSafe4> {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe4` cannot be made into an object
   |
   = help: consider moving `foo` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:15:19
   |
LL | trait NonObjectSafe4 {
   |       -------------- this trait cannot be made into an object...
LL |     fn foo(&self, &Self);
   |                   ^^^^^ ...because method `foo` references the `Self` type in this parameter

error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:38:16
   |
LL | impl Trait for dyn NonObjectSafe1 {}
   |                ^^^^^^^^^^^^^^^^^^ `NonObjectSafe1` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:4:23
   |
LL | trait NonObjectSafe1: Sized {}
   |       --------------  ^^^^^ ...because it requires `Self: Sized`
   |       |
   |       this trait cannot be made into an object...
error: aborting due to 5 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0038`.


------------------------------------------


---- [ui] ui/issues/issue-65611.rs stdout ----
diff of stderr:

+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn from(usize) -> Self;
+    |             ^^^^^ help: try naming the parameter or explicitly ignoring it: `_: usize`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
1 error[E0282]: type annotations needed
2   --> $DIR/issue-65611.rs:59:20
3    |

15 LL |     let x = buffer.last().unwrap().0.clone();
17 
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 1 warning emitted
19 
19 
20 Some errors have detailed explanations: E0282, E0609.
21 For more information about an error, try `rustc --explain E0282`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611/issue-65611.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-65611.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65611.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn from(usize) -> Self;
   |             ^^^^^ help: try naming the parameter or explicitly ignoring it: `_: usize`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-65611.rs:59:20
   |
LL |     let x = buffer.last().unwrap().0.clone();
   |             |      |
   |             |      cannot infer type for type parameter `T`
   |             |      cannot infer type for type parameter `T`
   |             this method call resolves to `Option<&T>`
   |
   = note: type must be known at this point

error[E0609]: no field `0` on type `&_`
   |
   |
LL |     let x = buffer.last().unwrap().0.clone();

error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0282, E0609.
---

---- [ui] ui/issues/issue-78720.rs stdout ----
diff of stderr:

24 LL | struct Map2<Segment2, F> {
26 
26 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn map2<F>(self, F) -> Map2<F> {}
+    |                      ^ help: try naming the parameter or explicitly ignoring it: `_: F`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
27 error[E0308]: mismatched types
28   --> $DIR/issue-78720.rs:7:36
29    |

49 LL |     fn map2<F>(&self, F) -> Map2<F> {}
51 
- error: aborting due to 4 previous errors
+ error: aborting due to 4 previous errors; 1 warning emitted
53 
53 
54 Some errors have detailed explanations: E0277, E0308, E0412.
55 For more information about an error, try `rustc --explain E0277`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720/issue-78720.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-78720.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-78720.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: at least one trait must be specified
  --> /checkout/src/test/ui/issues/issue-78720.rs:1:16
   |
LL | fn server() -> impl {

error[E0412]: cannot find type `F` in this scope
  --> /checkout/src/test/ui/issues/issue-78720.rs:13:12
   |
   |
LL |     _func: F,
   | 
  ::: /checkout/library/core/src/ops/function.rs:67:1
   |
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here
help: a trait with a similar name exists
   |
   |
LL |     _func: Fn,
help: you might be missing a type parameter
   |
   |
LL | struct Map2<Segment2, F> {


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn map2<F>(self, F) -> Map2<F> {}
   |                      ^ help: try naming the parameter or explicitly ignoring it: `_: F`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-78720.rs:7:36
   |
LL |     fn map2<F>(self, F) -> Map2<F> {}
   |                                    ^^ expected struct `Map2`, found `()`
   |
   = note: expected struct `Map2<F>`


error[E0277]: the size for values of type `Self` cannot be known at compilation time
   |
   |
LL |     fn map2<F>(self, F) -> Map2<F> {}
   |                ^^^^ doesn't have a size known at compile-time
   |
   = help: unsized fn params are gated as an unstable feature
help: consider further restricting `Self`
   |
LL |     fn map2<F>(self, F) -> Map2<F> where Self: Sized {}
   |                                    ^^^^^^^^^^^^^^^^^
help: function arguments must have a statically known size, borrowed types always have a known size
   |
LL |     fn map2<F>(&self, F) -> Map2<F> {}

error: aborting due to 4 previous errors; 1 warning emitted

Some errors have detailed explanations: E0277, E0308, E0412.
---

---- [ui] ui/parser/variadic-ffi-semantic-restrictions.rs stdout ----
diff of stderr:

202 LL |     fn t_f6(..., x: isize);
204 
- error: aborting due to 34 previous errors
- error: aborting due to 34 previous errors
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn t_f1(x: isize, ...) {}
+    |                       ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn t_f2(x: isize, ...);
+    |                       ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn t_f3(...) {}
+    |             ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn t_f4(...);
+    |             ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn t_f5(..., x: isize) {}
+    |             ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn t_f6(..., x: isize);
+    |             ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ error: aborting due to 34 previous errors; 6 warnings emitted
+ error: aborting due to 34 previous errors; 6 warnings emitted
206 
207 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-semantic-restrictions/variadic-ffi-semantic-restrictions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/variadic-ffi-semantic-restrictions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-semantic-restrictions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-semantic-restrictions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | fn f1_1(x: isize, ...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:8:9
   |
   |
LL | fn f1_2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | fn f1_2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f2_1(x: isize, ...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:15:20
   |
   |
LL | extern "C" fn f2_2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f2_2(...) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL | extern "C" fn f2_3(..., x: isize) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f2_3(..., x: isize) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f3_1(x: isize, ...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:26:20
   |
   |
LL | extern "C" fn f3_2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f3_2(...) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL | extern "C" fn f3_3(..., x: isize) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f3_3(..., x: isize) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:35:13
   |
   |
LL |     fn e_f1(...);


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn e_f2(..., x: isize);


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f1(x: isize, ...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:46:13
   |
   |
LL |     fn i_f2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f2(...) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn i_f3(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f3(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f3(..., x: isize, ...) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn i_f4(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f4(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f4(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f1(x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f2(x: isize, ...);

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:64:13
   |
   |
LL |     fn t_f3(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f3(...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:67:13
   |
   |
LL |     fn t_f4(...);


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f4(...);


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn t_f5(..., x: isize) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f5(..., x: isize) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn t_f6(..., x: isize);


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f6(..., x: isize);


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn t_f1(x: isize, ...) {}
   |                       ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn t_f2(x: isize, ...);
   |                       ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn t_f3(...) {}
   |             ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn t_f4(...);
   |             ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn t_f5(..., x: isize) {}
   |             ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn t_f6(..., x: isize);
   |             ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

error: aborting due to 34 previous errors; 6 warnings emitted
error: aborting due to 34 previous errors; 6 warnings emitted


------------------------------------------


---- [ui] ui/proc-macro/trait-fn-args-2015.rs stdout ----
normalized stderr:
warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
LL |     fn method(u8);
LL |     fn method(u8);
   |               ^^ help: try naming the parameter or explicitly ignoring it: `_: u8`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/trait-fn-args-2015/trait-fn-args-2015.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/trait-fn-args-2015.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/trait-fn-args-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/trait-fn-args-2015" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/trait-fn-args-2015/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
LL |     fn method(u8);
LL |     fn method(u8);
   |               ^^ help: try naming the parameter or explicitly ignoring it: `_: u8`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

---

---- [ui] ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs stdout ----
diff of stderr:

172 LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
173    |                                                        ^^ not a non-macro attribute
- error: aborting due to 29 previous errors
- error: aborting due to 29 previous errors
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
+    |                                                                ^^^^^^^ help: try naming the parameter or explicitly ignoring it: `_: Vec<u8>`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ error: aborting due to 29 previous errors; 1 warning emitted
176 
177 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/proc-macro-cannot-be-used.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2565-param-attrs/proc-macro-cannot-be-used.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
   |                       ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | unsafe extern "C" fn cvar(arg1: i32, #[id] mut args: ...) {}
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | fn free(#[id] arg1: u8) {
   |           ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     let lam = |#[id] W(x), #[id] y: usize| ();
   |                  ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     let lam = |#[id] W(x), #[id] y: usize| ();
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
   |                    ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
   |                                ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
   |                    ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
   |                                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
   |                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
   |                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
   |                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
   |                                               ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
   |                                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait1(#[id] self, #[id] arg1: u8);
   |                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait1(#[id] self, #[id] arg1: u8);
   |                             ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait2(#[id] &self, #[id] arg1: u8);
   |                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait2(#[id] &self, #[id] arg1: u8);
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
   |                     ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
   |                                         ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                     ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
   |                                                        ^^ not a non-macro attribute

warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                                                                ^^^^^^^ help: try naming the parameter or explicitly ignoring it: `_: Vec<u8>`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

---
---- [ui] ui/specialization/issue-39448.rs stdout ----
diff of stderr:

8    = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
9    = help: consider using `min_specialization` instead, which is more stable and complete
10 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn from(T) -> Self;
+    |             ^ help: try naming the parameter or explicitly ignoring it: `_: T`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
11 error[E0275]: overflow evaluating the requirement `T: FromA<U>`
13    |


17    = note: required because of the requirements on the impl of `FromA<U>` for `T`
18    = note: required because of the requirements on the impl of `ToA<T>` for `U`
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to previous error; 2 warnings emitted
21 
22 For more information about this error, try `rustc --explain E0275`.
22 For more information about this error, try `rustc --explain E0275`.
23 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448/issue-39448.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-39448.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-39448.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete

warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn from(T) -> Self;
   |             ^ help: try naming the parameter or explicitly ignoring it: `_: T`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


error[E0275]: overflow evaluating the requirement `T: FromA<U>`
   |
   |
LL |     x.foo(y.to()).to() //~ ERROR overflow evaluating the requirement
   |
   |
   = note: required because of the requirements on the impl of `FromA<U>` for `T`
   = note: required because of the requirements on the impl of `ToA<T>` for `U`
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0275`.

---
test result: FAILED. 11433 passed; 8 failed; 93 ignored; 0 measured; 0 filtered out; finished in 131.88s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:48
