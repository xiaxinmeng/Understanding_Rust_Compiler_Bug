plain
running 11943 tests
.................................................................................................... 100/11943
..........................................ii..........ii............................................ 200/11943
.................................................................................................... 300/11943
.......................F.......................................................................F.... 400/11943
.................................................................................................... 600/11943
.....................................i.............................................................. 700/11943
...........ii....................................................................................... 800/11943
.................................................................................................... 900/11943
---
.................................................................................................... 5300/11943
..............i..i.................................................................................. 5400/11943
.................................................................................................... 5500/11943
.................................................................................................... 5600/11943
F.......................................F........................................................... 5700/11943
.................................................................................................... 5900/11943
...............................i.................................................................... 6000/11943
.................................................................................................... 6100/11943
.....................................i.............................................................. 6200/11943
---

---- [ui] ui/associated-types/associated-types-outlives.rs stdout ----
diff of stderr:

- error[E0505]: cannot move out of `x` because it is borrowed
-   --> $DIR/associated-types-outlives.rs:22:14
+ error[E0311]: the parameter type `T` may not live long enough
3    |
3    |
- LL |         's: loop { y = denormalise(&x); break }
-    |                                    -- borrow of `x` occurs here
- LL |         drop(x);
-    |              ^ move out of `x` occurs here
- LL |         return f(y);
-    |                  - borrow later used here
+ LL |   fn denormalise<'a, T>(t: &'a T) -> <T as Foo<'a>>::Bar {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |   ^                  - help: consider adding an explicit lifetime bound...: `T: 'a`
+    |  _|
+ LL | |     t
+ LL | | }
+ LL | | }
+    | |_^ ...so that the type `T` will meet its required lifetime bounds
11 error: aborting due to previous error
12 

- For more information about this error, try `rustc --explain E0505`.
---
To only update this specific test, also pass `--test-args associated-types/associated-types-outlives.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-outlives.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-outlives" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-outlives/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0311]: the parameter type `T` may not live long enough
   |
   |
LL |   fn denormalise<'a, T>(t: &'a T) -> <T as Foo<'a>>::Bar {
   |   ^                  - help: consider adding an explicit lifetime bound...: `T: 'a`
   |  _|
LL | |     t
LL | | }
LL | | }
   | |_^ ...so that the type `T` will meet its required lifetime bounds
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/higher-ranked-projection.rs#bad stdout ----
diff of stderr:

4 LL |     foo(());
5    |     ^^^ lifetime mismatch
6    |
-    = note: expected type `&'a ()`
-               found type `&()`
+    = note: expected reference `&'a ()`
+                    found type `&()`
9 note: the lifetime requirement is introduced here
10   --> $DIR/higher-ranked-projection.rs:15:33


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/higher-ranked-projection.bad/higher-ranked-projection.bad.stderr
To only update this specific test, also pass `--test-args associated-types/higher-ranked-projection.rs`


error in revision `bad`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/higher-ranked-projection.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "bad" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/higher-ranked-projection.bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/higher-ranked-projection.bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/higher-ranked-projection.rs:25:5
   |
LL |     foo(());
   |     ^^^ lifetime mismatch
   |
   = note: expected reference `&'a ()`
                   found type `&()`
note: the lifetime requirement is introduced here
   |
   |
LL |     where for<'a> &'a T: Mirror<Image=U>

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
15    |
16    = note: expected associated type `<T as impl_trait::Trait>::Assoc`
-                          found type `()`
+                     found unit type `()`
18 help: consider constraining the associated type `<T as impl_trait::Trait>::Assoc` to `()`
19    |
20 LL |     fn foo_fail<T: Trait<Assoc = ()>>() -> impl FooLike<Output=T::Assoc> {
33    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected associated type, found `()`
34    |
34    |
35    = note: expected associated type `<T as lifetimes::Trait<'static>>::Assoc`
+                     found unit type `()`
+                     found unit type `()`
37 help: consider constraining the associated type `<T as lifetimes::Trait<'static>>::Assoc` to `()`
38    |
39 LL |     fn foo2_fail<'a, T: Trait<'a, Assoc = ()>>() -> impl FooLike<Output=T::Assoc> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/bound-normalization-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bound-normalization-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(impl_trait_in_bindings)]
   |            ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
   |
   |
LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output=T::Assoc> {
   |
   = note: expected associated type `<T as impl_trait::Trait>::Assoc`
                    found unit type `()`
                    found unit type `()`
help: consider constraining the associated type `<T as impl_trait::Trait>::Assoc` to `()`
   |
LL |     fn foo_fail<T: Trait<Assoc = ()>>() -> impl FooLike<Output=T::Assoc> {


error[E0760]: `impl Trait` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
   |
   |
LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {


error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
   |
   |
LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output=T::Assoc> {
   |
   |
   = note: expected associated type `<T as lifetimes::Trait<'static>>::Assoc`
                    found unit type `()`
help: consider constraining the associated type `<T as lifetimes::Trait<'static>>::Assoc` to `()`
   |
LL |     fn foo2_fail<'a, T: Trait<'a, Assoc = ()>>() -> impl FooLike<Output=T::Assoc> {

error: aborting due to 3 previous errors; 1 warning emitted

Some errors have detailed explanations: E0271, E0760.
---

---- [ui] ui/issues/issue-18611.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `isize: HasState` is not satisfied
-   --> $DIR/issue-18611.rs:1:4
3    |
3    |
- LL | fn add_state(op: <isize as HasState>::State) {
-    |    ^^^^^^^^^ the trait `HasState` is not implemented for `isize`
+ LL | / fn add_state(op: <isize as HasState>::State) {
+ LL | |
+ LL | | }
+    | |_^ the trait `HasState` is not implemented for `isize`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18611/issue-18611.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-18611.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18611.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18611" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18611/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `isize: HasState` is not satisfied
   |
   |
LL | / fn add_state(op: <isize as HasState>::State) {
LL | | //~^ ERROR `isize: HasState` is not satisfied
LL | | }
   | |_^ the trait `HasState` is not implemented for `isize`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/issues/issue-22872.rs stdout ----
diff of stderr:

- error[E0277]: `<P as Process<'b>>::Item` is not an iterator
+ error[E0277]: `<P as Process<'_>>::Item` is not an iterator
3    |
3    |
4 LL |     let _: Box<dyn for<'b> Wrap<'b>> = Box::new(Wrapper(process));

-    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ `<P as Process<'b>>::Item` is not an iterator
+    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ `<P as Process<'_>>::Item` is not an iterator
6    |
-    = help: the trait `for<'b> Iterator` is not implemented for `<P as Process<'b>>::Item`
+    = help: the trait `Iterator` is not implemented for `<P as Process<'_>>::Item`
8 note: required because of the requirements on the impl of `for<'b> Wrap<'b>` for `Wrapper<P>`
10    |


13    = note: required for the cast to the object type `dyn for<'b> Wrap<'b>`
15    |
15    |
- LL | fn push_process<P>(process: P) where P: Process<'static>, for<'b> <P as Process<'b>>::Item: Iterator {
-    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | fn push_process<P>(process: P) where P: Process<'static>, <P as Process<'_>>::Item: Iterator {
18 
19 error: aborting due to previous error
20 

---
To only update this specific test, also pass `--test-args issues/issue-22872.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22872.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22872" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22872/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `<P as Process<'_>>::Item` is not an iterator
   |
   |
LL |     let _: Box<dyn for<'b> Wrap<'b>> = Box::new(Wrapper(process));
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ `<P as Process<'_>>::Item` is not an iterator
   |
   = help: the trait `Iterator` is not implemented for `<P as Process<'_>>::Item`
note: required because of the requirements on the impl of `for<'b> Wrap<'b>` for `Wrapper<P>`
   |
   |
LL | impl<'b, P> Wrap<'b> for Wrapper<P>
   |             ^^^^^^^^     ^^^^^^^^^^
   = note: required for the cast to the object type `dyn for<'b> Wrap<'b>`
   |
   |
LL | fn push_process<P>(process: P) where P: Process<'static>, <P as Process<'_>>::Item: Iterator {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/issues/issue-35570.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `for<'a> (): Trait2<'a>` is not satisfied
-   --> $DIR/issue-35570.rs:8:4
3    |
3    |
- LL | fn _ice(param: Box<dyn for <'a> Trait1<<() as Trait2<'a>>::Ty>>) {
-    |    ^^^^ the trait `for<'a> Trait2<'a>` is not implemented for `()`
+ LL | / fn _ice(param: Box<dyn for <'a> Trait1<<() as Trait2<'a>>::Ty>>) {
+ LL | |
+ LL | |     let _e: (usize, usize) = unsafe{mem::transmute(param)};
+ LL | | }
+    | |_^ the trait `for<'a> Trait2<'a>` is not implemented for `()`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35570/issue-35570.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-35570.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35570.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35570" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35570/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `for<'a> (): Trait2<'a>` is not satisfied
   |
   |
LL | / fn _ice(param: Box<dyn for <'a> Trait1<<() as Trait2<'a>>::Ty>>) {
LL | | //~^ the trait bound `for<'a> (): Trait2<'a>` is not satisfied
LL | |     let _e: (usize, usize) = unsafe{mem::transmute(param)};
LL | | }
   | |_^ the trait `for<'a> Trait2<'a>` is not implemented for `()`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/issues/issue-37109.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37109.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37109/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37109/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0311]: the parameter type `T` may not live long enough
   |
   |
LL |   fn example<'a, T>(value: &'a T) -> (<T as ToRef<'a>>::Ref, u32) {
   |   ^              - help: consider adding an explicit lifetime bound...: `T: 'a`
   |  _|
LL | |     (value, 0)
LL | | }
LL | | }
   | |_^ ...so that the type `T` will meet its required lifetime bounds
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-67039-unsound-pin-partialeq.rs stdout ----
diff of stderr:

4 LL |     let _ = Pin::new(Apple) == Rc::pin(Apple);
5    |                             ^^ expected struct `Apple`, found struct `Rc`
6    |
-    = note: expected type `Apple`
-             found struct `Rc<Apple>`
+    = note: expected struct `Apple`
+               found struct `Rc<Apple>`
9    = note: required because of the requirements on the impl of `PartialEq<Pin<Rc<Apple>>>` for `Pin<Apple>`
11 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67039-unsound-pin-partialeq/issue-67039-unsound-pin-partialeq.stderr
To only update this specific test, also pass `--test-args issues/issue-67039-unsound-pin-partialeq.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-67039-unsound-pin-partialeq.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67039-unsound-pin-partialeq" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67039-unsound-pin-partialeq/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Rc<Apple> as Deref>::Target == Rc<Apple>`
  --> /checkout/src/test/ui/issues/issue-67039-unsound-pin-partialeq.rs:25:29
   |
LL |     let _ = Pin::new(Apple) == Rc::pin(Apple);
   |                             ^^ expected struct `Apple`, found struct `Rc`
   |
   = note: expected struct `Apple`
              found struct `Rc<Apple>`
   = note: required because of the requirements on the impl of `PartialEq<Pin<Rc<Apple>>>` for `Pin<Apple>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.

---
diff of stderr:

2   --> $DIR/issue-75361-mismatched-impl.rs:18:3
3    |
4 LL |   fn adjacent_edges(&self) -> Box<dyn MyTrait<Item = &Self::EdgeType>>;
-    |   --------------------------------------------------------------------- expected `fn(&T) -> Box<(dyn MyTrait<Item = &_> + 'static)>`
+    |   --------------------------------------------------------------------- expected `fn(&T) -> Box<(dyn MyTrait<Item = &T> + 'static)>`
6 ...
7 LL |   fn adjacent_edges(&self) -> Box<dyn MyTrait<Item = &Self::EdgeType> + '_> {
-    |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&T) -> Box<dyn MyTrait<Item = &_>>`
+    |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&T) -> Box<dyn MyTrait<Item = &T>>`
9    |
10    = note: expected `fn(&T) -> Box<(dyn MyTrait<Item = &T> + 'static)>`
11               found `fn(&T) -> Box<dyn MyTrait<Item = &T>>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-75361-mismatched-impl/issue-75361-mismatched-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/issue-75361-mismatched-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-75361-mismatched-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-75361-mismatched-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-75361-mismatched-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `impl` item signature doesn't match `trait` item signature
   |
   |
LL |   fn adjacent_edges(&self) -> Box<dyn MyTrait<Item = &Self::EdgeType>>;
   |   --------------------------------------------------------------------- expected `fn(&T) -> Box<(dyn MyTrait<Item = &T> + 'static)>`
...
LL |   fn adjacent_edges(&self) -> Box<dyn MyTrait<Item = &Self::EdgeType> + '_> { //~ ERROR `impl`
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&T) -> Box<dyn MyTrait<Item = &T>>`
   |
   = note: expected `fn(&T) -> Box<(dyn MyTrait<Item = &T> + 'static)>`
              found `fn(&T) -> Box<dyn MyTrait<Item = &T>>`
help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
   |
   |
LL |   fn adjacent_edges(&self) -> Box<dyn MyTrait<Item = &Self::EdgeType>>;
   |                                                       ^^^^ consider borrowing this type parameter in the trait
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/nll/normalization-bounds-error.rs stdout ----
diff of stderr:

1 error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'d` due to conflicting requirements
-   --> $DIR/normalization-bounds-error.rs:12:4
3    |
3    |
4 LL | fn visit_seq<'d, 'a: 'd>() -> <&'a () as Visitor<'d>>::Value {}
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7 note: first, the lifetime cannot outlive the lifetime `'d` as defined on the function body at 12:14...


15 LL | fn visit_seq<'d, 'a: 'd>() -> <&'a () as Visitor<'d>>::Value {}
16    |                  ^^
17 note: ...so that the types are compatible
-   --> $DIR/normalization-bounds-error.rs:12:4
19    |
19    |
20 LL | fn visit_seq<'d, 'a: 'd>() -> <&'a () as Visitor<'d>>::Value {}
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
22    = note: expected `Visitor<'d>`
23               found `Visitor<'_>`
24 
24 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/normalization-bounds-error/normalization-bounds-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/normalization-bounds-error.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/normalization-bounds-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/normalization-bounds-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/normalization-bounds-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'d` due to conflicting requirements
   |
   |
LL | fn visit_seq<'d, 'a: 'd>() -> <&'a () as Visitor<'d>>::Value {}
   |
   |
note: first, the lifetime cannot outlive the lifetime `'d` as defined on the function body at 12:14...
   |
   |
LL | fn visit_seq<'d, 'a: 'd>() -> <&'a () as Visitor<'d>>::Value {}
   |              ^^
note: ...but the lifetime must also be valid for the lifetime `'a` as defined on the function body at 12:18...
   |
   |
LL | fn visit_seq<'d, 'a: 'd>() -> <&'a () as Visitor<'d>>::Value {}
   |                  ^^
note: ...so that the types are compatible
   |
   |
LL | fn visit_seq<'d, 'a: 'd>() -> <&'a () as Visitor<'d>>::Value {}
   = note: expected `Visitor<'d>`
              found `Visitor<'_>`

error: aborting due to previous error
---

---- [ui] ui/regions/regions-implied-bounds-projection-gap-hr-1.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `for<'z> T: Trait2<'y, 'z>` is not satisfied
-   --> $DIR/regions-implied-bounds-projection-gap-hr-1.rs:21:4
+ error[E0277]: the trait bound `for<'y, 'z> T: Trait2<'y, 'z>` is not satisfied
3    |
3    |
- LL | fn callee<'x, 'y, T>(t: &'x dyn for<'z> Trait1< <T as Trait2<'y, 'z>>::Foo >)
-    |    ^^^^^^ the trait `for<'z> Trait2<'y, 'z>` is not implemented for `T`
+ LL | / fn callee<'x, 'y, T>(t: &'x dyn for<'z> Trait1< <T as Trait2<'y, 'z>>::Foo >)
+ LL | |
+ LL | | {
+ LL | | }
+    | |_^ the trait `for<'y, 'z> Trait2<'y, 'z>` is not implemented for `T`
7 help: consider restricting type parameter `T`
8    |


- LL | fn callee<'x, 'y, T: for<'z> Trait2<'y, 'z>>(t: &'x dyn for<'z> Trait1< <T as Trait2<'y, 'z>>::Foo >)
-    |                    ^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | fn callee<'x, 'y, T: for<'y, 'z> Trait2<'y, 'z>>(t: &'x dyn for<'z> Trait1< <T as Trait2<'y, 'z>>::Foo >)
11 
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-implied-bounds-projection-gap-hr-1/regions-implied-bounds-projection-gap-hr-1.stderr
To only update this specific test, also pass `--test-args regions/regions-implied-bounds-projection-gap-hr-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-implied-bounds-projection-gap-hr-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-implied-bounds-projection-gap-hr-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-implied-bounds-projection-gap-hr-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `for<'y, 'z> T: Trait2<'y, 'z>` is not satisfied
   |
   |
LL | / fn callee<'x, 'y, T>(t: &'x dyn for<'z> Trait1< <T as Trait2<'y, 'z>>::Foo >)
LL | |     //~^ the trait bound `for<'z> T: Trait2<'y, 'z>` is not satisfied
LL | | {
LL | | }
   | |_^ the trait `for<'y, 'z> Trait2<'y, 'z>` is not implemented for `T`
help: consider restricting type parameter `T`
   |
   |
LL | fn callee<'x, 'y, T: for<'y, 'z> Trait2<'y, 'z>>(t: &'x dyn for<'z> Trait1< <T as Trait2<'y, 'z>>::Foo >)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/recursion/issue-83150.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>: Iterator`
+ error[E0275]: overflow evaluating the requirement `Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>: Iterator`
2    |
-    = help: consider adding a `#![recursion_limit="80"]` attribute to your crate (`issue_83150`)
-    = note: required because of the requirements on the impl of `Iterator` for `&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>`
+    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_83150`)
+    = note: required because of the requirements on the impl of `Iterator` for `&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>, [closure@$DIR/issue-83150.rs:9:24: 9:33]>`
6 error: aborting due to previous error
7 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/issue-83150.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/issue-83150.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/issue-83150.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>: Iterator`
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_83150`)
   = note: required because of the requirements on the impl of `Iterator` for `&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:9:24: 9:33]>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.


------------------------------------------


---- [ui] ui/traits/inductive-overflow/lifetime.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `Box<X<C<'static>>>: NotAuto`
+ error[E0275]: overflow evaluating the requirement `Box<X<C<'_>>>: NotAuto`
3    |
3    |
4 LL | fn is_send<S: NotAuto>() {}

7 LL |     is_send::<X<C<'static>>>();
9    |
9    |
- note: required because of the requirements on the impl of `NotAuto` for `X<C<'static>>`
+ note: required because of the requirements on the impl of `NotAuto` for `X<C<'_>>`
12    |
12    |
13 LL | impl<T: Y> NotAuto for X<T> where T::P: NotAuto {}
14    |            ^^^^^^^     ^^^^
14    |            ^^^^^^^     ^^^^
+    = note: 2 redundant requirements hidden
+    = note: required because of the requirements on the impl of `NotAuto` for `X<C<'static>>`
16 error: aborting due to previous error
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime/lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `Box<X<C<'_>>>: NotAuto`
   |
   |
LL | fn is_send<S: NotAuto>() {}
   |               ------- required by this bound in `is_send`
...
LL |     is_send::<X<C<'static>>>();
   |
   |
note: required because of the requirements on the impl of `NotAuto` for `X<C<'_>>`
   |
   |
LL | impl<T: Y> NotAuto for X<T> where T::P: NotAuto {} //~ NOTE: required
   |            ^^^^^^^     ^^^^
   = note: 2 redundant requirements hidden
   = note: required because of the requirements on the impl of `NotAuto` for `X<C<'static>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.


------------------------------------------


---- [ui] ui/type-alias-impl-trait/issue-60371.rs stdout ----
diff of stderr:

25    = help: the following implementations were found:
26              <&() as Bug>
27 
- error[E0277]: the trait bound `(): Bug` is not satisfied
+ error: could not find defining uses
30    |
31 LL |     type Item = impl Bug;


-    |                 ^^^^^^^^ the trait `Bug` is not implemented for `()`
-    = help: the following implementations were found:
-    = help: the following implementations were found:
-              <&() as Bug>
36 
37 error: aborting due to 4 previous errors
38 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-60371.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-60371.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60371" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60371/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL |     type Item = impl Bug; //~ ERROR `impl Trait` in type aliases is unstable
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable
error[E0658]: type alias impl trait is not permitted here
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-60371.rs:14:37
   |
   |
LL |     const FUN: fn() -> Self::Item = || ();
   |
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information
   = help: add `#![feature(impl_trait_in_bindings)]` to the crate attributes to enable


error[E0277]: the trait bound `(): Bug` is not satisfied
   |
   |
LL |     type Item = impl Bug; //~ ERROR `impl Trait` in type aliases is unstable
   |                 ^^^^^^^^ the trait `Bug` is not implemented for `()`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <&() as Bug>
error: could not find defining uses
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-60371.rs:10:17
   |
   |
LL |     type Item = impl Bug; //~ ERROR `impl Trait` in type aliases is unstable

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0658.
---

---- [ui] ui/wf/wf-foreign-fn-decl-ret.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `(): Foo` is not satisfied
-   --> $DIR/wf-foreign-fn-decl-ret.rs:11:12
+   --> $DIR/wf-foreign-fn-decl-ret.rs:11:5
3    |
4 LL |     pub fn lint_me() -> <() as Foo>::Assoc;
-    |            ^^^^^^^ the trait `Foo` is not implemented for `()`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `()`
6 
7 error[E0277]: the trait bound `u32: Unsatisfied` is not satisfied
8   --> $DIR/wf-foreign-fn-decl-ret.rs:14:32

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-foreign-fn-decl-ret/wf-foreign-fn-decl-ret.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args wf/wf-foreign-fn-decl-ret.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-foreign-fn-decl-ret.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-foreign-fn-decl-ret" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-foreign-fn-decl-ret/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `(): Foo` is not satisfied
  --> /checkout/src/test/ui/wf/wf-foreign-fn-decl-ret.rs:11:5
   |
LL |     pub fn lint_me() -> <() as Foo>::Assoc;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `()`

error[E0277]: the trait bound `u32: Unsatisfied` is not satisfied
  --> /checkout/src/test/ui/wf/wf-foreign-fn-decl-ret.rs:14:32
   |
LL | pub struct Bar<T: Unsatisfied>(T);
   |                   ----------- required by this bound in `Bar`
...
LL |     pub fn lint_me_aswell() -> Bar<u32>;
   |                                ^^^^^^^^ the trait `Unsatisfied` is not implemented for `u32`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11831 passed; 15 failed; 97 ignored; 0 measured; 0 filtered out; finished in 122.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:24
