plain
.....i.............................................................................................. 4000/12555
.i.................i.......................i............F........................................... 4100/12555
.................................................................................................... 4200/12555
.................................................................................................... 4300/12555
.............................F............F......................................................... 4400/12555
.................................................................................................... 4600/12555
.................................................................................................... 4700/12555
...............................................................................................i.... 4800/12555
..........................................................i......................................... 4900/12555
---
.................................................i....i........................................i.... 6700/12555
............i.............i....................................................i.................... 6800/12555
..................................................................................i................. 6900/12555
.................................................................................................... 7000/12555
...................................................F.F.............................................. 7100/12555
.................................................................................................... 7300/12555
...........................................................i........................................ 7400/12555
.................................................................................................... 7500/12555
.................................................................................................... 7500/12555
...............................................................................F..F................. 7600/12555
.................................................................................................... 7800/12555
.................................................................................................... 7900/12555
.................................................................................................... 8000/12555
..............................................................i..ii................................. 8100/12555
---
.................................................................................................... 9200/12555
.................................................................................................... 9300/12555
iiii.iiiii..................................................................ii...............i...... 9400/12555
.................................................................................................... 9500/12555
..........................................................................................F......... 9600/12555
..............................................................................F..................F.. 9700/12555
.................................................................................................... 9900/12555
.................................................................................................... 10000/12555
..............................................................................................i..i.. 10100/12555
.i.............................................................F.................................... 10200/12555
---
1 error[E0623]: lifetime mismatch
-   --> $DIR/associated-types-project-from-hrtb-in-fn-body.rs:22:40
+   --> $DIR/associated-types-project-from-hrtb-in-fn-body.rs:22:29
3    |
4 LL |     x: <I as Foo<&'a isize>>::A,
-    |                  --------- these two types are declared with different lifetimes...
- LL |     y: <I as Foo<&'b isize>>::A,
7    |                  ---------
+ LL |     y: <I as Foo<&'b isize>>::A,
+    |                  --------- these two types are declared with different lifetimes...
8 ...
9 LL |     let z: I::A = if cond { x } else { y };
-    |                                        ^ ...but data from `x` flows into `y` here
+    |                             ^ ...but data from `y` flows into `x` here
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body/associated-types-project-from-hrtb-in-fn-body.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-project-from-hrtb-in-fn-body.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs:22:29
   |
LL |     x: <I as Foo<&'a isize>>::A,
   |                  ---------
LL |     y: <I as Foo<&'b isize>>::A,
   |                  --------- these two types are declared with different lifetimes...
...
LL |     let z: I::A = if cond { x } else { y };
   |                             ^ ...but data from `y` flows into `x` here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.

---
1 error[E0623]: lifetime mismatch
-   --> $DIR/project-fn-ret-invariant.rs:56:6
+   --> $DIR/project-fn-ret-invariant.rs:54:22
3    |
4 LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |                         |
-    |                         this parameter and the return type are declared with different lifetimes...
- ...
- LL |     (a, b)
- LL |     (a, b)
-    |      ^ ...but data from `y` is returned here
+    |                                      |
+    |                                      this parameter and the return type are declared with different lifetimes...
+    |                                      this parameter and the return type are declared with different lifetimes...
+ LL |     let a = bar(foo, y);
+    |                      ^ ...but data from `x` is returned here
12 error[E0623]: lifetime mismatch
-   --> $DIR/project-fn-ret-invariant.rs:55:22
+   --> $DIR/project-fn-ret-invariant.rs:56:9
14    |
14    |
15 LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |                         |
-    |                         this parameter and the return type are declared with different lifetimes...
-    |                         this parameter and the return type are declared with different lifetimes...
- LL |     let a = bar(foo, y);
- LL |     let b = bar(foo, x);
-    |                      ^ ...but data from `y` is returned here
+    |                                      |
+    |                                      this parameter and the return type are declared with different lifetimes...
+ ...
+ LL |     (a, b)
+ LL |     (a, b)
+    |         ^ ...but data from `x` is returned here
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross/project-fn-ret-invariant.krisskross.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`


error in revision `krisskross`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:54:22
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |                                      |
   |                                      this parameter and the return type are declared with different lifetimes...
   |                                      this parameter and the return type are declared with different lifetimes...
LL |     let a = bar(foo, y); //[krisskross]~ ERROR E0623
   |                      ^ ...but data from `x` is returned here
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:56:9
   |
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |                                      |
   |                                      this parameter and the return type are declared with different lifetimes...
...
...
LL |     (a, b) //[krisskross]~ ERROR E0623
   |         ^ ...but data from `x` is returned here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0623`.

---
1 error[E0623]: lifetime mismatch
-   --> $DIR/project-fn-ret-invariant.rs:39:20
+   --> $DIR/project-fn-ret-invariant.rs:40:20
3    |
4 LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |                   |
-    |                   this parameter and the return type are declared with different lifetimes...
-    |                   this parameter and the return type are declared with different lifetimes...
- LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
- LL |     let a = bar(f, x);
-    |                    ^ ...but data from `y` is returned here
+    |                                |
+    |                                this parameter and the return type are declared with different lifetimes...
+ ...
+ ...
+ LL |     let b = bar(f, y);
+    |                    ^ ...but data from `x` is returned here
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse/project-fn-ret-invariant.oneuse.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`

error in revision `oneuse`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "oneuse" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:40:20
   |
LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |                                |
   |                                this parameter and the return type are declared with different lifetimes...
...
...
LL |     let b = bar(f, y); //[oneuse]~ ERROR lifetime mismatch [E0623]
   |                    ^ ...but data from `x` is returned here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.


------------------------------------------


---- [ui] ui/associated-types/issue-87261.rs stdout ----
diff of stderr:

160 LL | fn returns_opaque_derived() -> impl DerivedTrait<Associated = ()> + 'static {
162 
162 
- error[E0271]: type mismatch resolving `<impl Trait + Foo as Trait>::Associated == ()`
+ error[E0271]: type mismatch resolving `<impl Foo + Trait as Trait>::Associated == ()`
165    |
165    |
166 LL | fn returns_opaque_foo() -> impl Trait + Foo {
170    |     ^^^^^^^^^^^^^ expected `()`, found associated type
171    |
172    = note:    expected unit type `()`
172    = note:    expected unit type `()`
-            found associated type `<impl Trait + Foo as Trait>::Associated`
+            found associated type `<impl Foo + Trait as Trait>::Associated`
174 note: required by a bound in `accepts_trait`
176    |


177 LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
178    |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
- help: consider constraining the associated type `<impl Trait + Foo as Trait>::Associated` to `()`
+ help: consider constraining the associated type `<impl Foo + Trait as Trait>::Associated` to `()`
180    |
181 LL | fn returns_opaque_foo() -> impl Trait<Associated = ()> + Foo {


221 LL | fn returns_opaque_generic() -> impl GenericTrait<(), Associated = ()> + 'static {
223 
223 
- error[E0271]: type mismatch resolving `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated == ()`
+ error[E0271]: type mismatch resolving `<impl GenericTrait<()> + Foo as GenericTrait<()>>::Associated == ()`
226    |
226    |
227 LL | fn returns_opaque_generic_foo() -> impl GenericTrait<()> + Foo {
231    |     ^^^^^^^^^^^^^^^^^^^^^ expected `()`, found associated type
232    |
233    = note:    expected unit type `()`
233    = note:    expected unit type `()`
-            found associated type `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated`
+            found associated type `<impl GenericTrait<()> + Foo as GenericTrait<()>>::Associated`
235 note: required by a bound in `accepts_generic_trait`
237    |


238 LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
239    |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
- help: consider constraining the associated type `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated` to `()`
+ help: consider constraining the associated type `<impl GenericTrait<()> + Foo as GenericTrait<()>>::Associated` to `()`
241    |
242 LL | fn returns_opaque_generic_foo() -> impl GenericTrait<(), Associated = ()> + Foo {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261/issue-87261.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261/issue-87261.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-87261.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-87261.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<A as Trait>::Associated == ()`
   |
LL |     accepts_trait(a);
   |     ^^^^^^^^^^^^^ expected `()`, found associated type
   |
   |
   = note:    expected unit type `()`
           found associated type `<A as Trait>::Associated`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<A as Trait>::Associated` to `()`
   |
LL |     A: Trait<Associated = ()> + 'static,


error[E0271]: type mismatch resolving `<B as Trait>::Associated == ()`
   |
LL |     accepts_trait(b);
   |     ^^^^^^^^^^^^^ expected `()`, found associated type
   |
   |
   = note:    expected unit type `()`
           found associated type `<B as Trait>::Associated`
   = help: consider constraining the associated type `<B as Trait>::Associated` to `()`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`

error[E0271]: type mismatch resolving `<C as Trait>::Associated == ()`
   |
LL |     accepts_trait(c);
   |     ^^^^^^^^^^^^^ expected `()`, found associated type
   |
   |
   = note:    expected unit type `()`
           found associated type `<C as Trait>::Associated`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<C as Trait>::Associated` to `()`
   |
LL |     C: Trait<Associated = ()> + Foo,


error[E0271]: type mismatch resolving `<D as Trait>::Associated == ()`
   |
LL |     accepts_trait(d);
   |     ^^^^^^^^^^^^^ expected `()`, found associated type
   |
   |
   = note:    expected unit type `()`
           found associated type `<D as Trait>::Associated`
   = help: consider constraining the associated type `<D as Trait>::Associated` to `()`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`

error[E0271]: type mismatch resolving `<E as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(e);
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<E as GenericTrait<()>>::Associated`
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
help: consider constraining the associated type `<E as GenericTrait<()>>::Associated` to `()`
   |
LL |     E: GenericTrait<(), Associated = ()> + 'static,


error[E0271]: type mismatch resolving `<F as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(f);
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<F as GenericTrait<()>>::Associated`
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
help: consider constraining the associated type `<F as GenericTrait<()>>::Associated` to `()`
   |
LL |     F: GenericTrait<(), Associated = ()> + Foo,


error[E0271]: type mismatch resolving `<G as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(g);
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<G as GenericTrait<()>>::Associated`
   = help: consider constraining the associated type `<G as GenericTrait<()>>::Associated` to `()`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`

error[E0271]: type mismatch resolving `<impl Trait as Trait>::Associated == ()`
   |
   |
LL | fn returns_opaque() -> impl Trait + 'static {
...
...
LL |     accepts_trait(returns_opaque());
   |
   = note:    expected unit type `()`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
           found associated type `<impl Trait as Trait>::Associated`
           found associated type `<impl Trait as Trait>::Associated`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<impl Trait as Trait>::Associated` to `()`
   |
LL | fn returns_opaque() -> impl Trait<Associated = ()> + 'static {


error[E0271]: type mismatch resolving `<impl DerivedTrait as Trait>::Associated == ()`
   |
   |
LL | fn returns_opaque_derived() -> impl DerivedTrait + 'static {
...
...
LL |     accepts_trait(returns_opaque_derived());
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<impl DerivedTrait as Trait>::Associated`
note: required by a bound in `accepts_trait`
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<impl DerivedTrait as Trait>::Associated` to `()`
   |
LL | fn returns_opaque_derived() -> impl DerivedTrait<Associated = ()> + 'static {


error[E0271]: type mismatch resolving `<impl Foo + Trait as Trait>::Associated == ()`
   |
   |
LL | fn returns_opaque_foo() -> impl Trait + Foo {
...
...
LL |     accepts_trait(returns_opaque_foo());
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<impl Foo + Trait as Trait>::Associated`
note: required by a bound in `accepts_trait`
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<impl Foo + Trait as Trait>::Associated` to `()`
   |
LL | fn returns_opaque_foo() -> impl Trait<Associated = ()> + Foo {


error[E0271]: type mismatch resolving `<impl DerivedTrait + Foo as Trait>::Associated == ()`
   |
   |
LL | fn returns_opaque_derived_foo() -> impl DerivedTrait + Foo {
...
...
LL |     accepts_trait(returns_opaque_derived_foo());
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<impl DerivedTrait + Foo as Trait>::Associated`
   = help: consider constraining the associated type `<impl DerivedTrait + Foo as Trait>::Associated` to `()`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`

error[E0271]: type mismatch resolving `<impl GenericTrait<()> as GenericTrait<()>>::Associated == ()`
   |
   |
LL | fn returns_opaque_generic() -> impl GenericTrait<()> + 'static {
...
...
LL |     accepts_generic_trait(returns_opaque_generic());
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<impl GenericTrait<()> as GenericTrait<()>>::Associated`
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
help: consider constraining the associated type `<impl GenericTrait<()> as GenericTrait<()>>::Associated` to `()`
   |
LL | fn returns_opaque_generic() -> impl GenericTrait<(), Associated = ()> + 'static {


error[E0271]: type mismatch resolving `<impl GenericTrait<()> + Foo as GenericTrait<()>>::Associated == ()`
   |
   |
LL | fn returns_opaque_generic_foo() -> impl GenericTrait<()> + Foo {
...
...
LL |     accepts_generic_trait(returns_opaque_generic_foo());
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<impl GenericTrait<()> + Foo as GenericTrait<()>>::Associated`
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
help: consider constraining the associated type `<impl GenericTrait<()> + Foo as GenericTrait<()>>::Associated` to `()`
   |
LL | fn returns_opaque_generic_foo() -> impl GenericTrait<(), Associated = ()> + Foo {


error[E0271]: type mismatch resolving `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated == ()`
   |
   |
LL | fn returns_opaque_generic_duplicate() -> impl GenericTrait<()> + GenericTrait<u8> {
...
...
LL |     accepts_generic_trait(returns_opaque_generic_duplicate());
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated`
   = help: consider constraining the associated type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated` to `()`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/closures/closure-expected-type/expect-region-supply-region-2.rs stdout ----
diff of stderr:

6    |
7    = note: expected reference `&u32`
8               found reference `&'x u32`
- note: the anonymous lifetime #1 defined here...
+ note: the lifetime `'x` as defined here...
+    |
+    |
+ LL | fn expect_bound_supply_named<'x>() {
+    |                              ^^
+ note: ...does not necessarily outlive the anonymous lifetime #1 defined here
11    |
11    |
12 LL |       closure_expecting_bound(|x: &'x u32| {
18 LL | |         f = Some(x);
19 LL | |     });
20    | |_____^
20    | |_____^
- note: ...does not necessarily outlive the lifetime `'x` as defined here
-   --> $DIR/expect-region-supply-region-2.rs:9:30
-    |
- LL | fn expect_bound_supply_named<'x>() {
26 
27 error[E0308]: mismatched types
28   --> $DIR/expect-region-supply-region-2.rs:14:33


32    |
33    = note: expected reference `&u32`
34               found reference `&'x u32`
- note: the lifetime `'x` as defined here...
-   --> $DIR/expect-region-supply-region-2.rs:9:30
-    |
- LL | fn expect_bound_supply_named<'x>() {
-    |                              ^^
- note: ...does not necessarily outlive the anonymous lifetime #1 defined here
+ note: the anonymous lifetime #1 defined here...
42    |
42    |
43 LL |       closure_expecting_bound(|x: &'x u32| {
49 LL | |         f = Some(x);
50 LL | |     });
51    | |_____^
51    | |_____^
+ note: ...does not necessarily outlive the lifetime `'x` as defined here
+    |
+    |
+ LL | fn expect_bound_supply_named<'x>() {
52 
53 error: aborting due to 2 previous errors
54 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2/expect-region-supply-region-2.stderr
To only update this specific test, also pass `--test-args closures/closure-expected-type/expect-region-supply-region-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region-2.rs:14:33
   |
LL |     closure_expecting_bound(|x: &'x u32| {
   |                                 ^^^^^^^ lifetime mismatch
   = note: expected reference `&u32`
              found reference `&'x u32`
              found reference `&'x u32`
note: the lifetime `'x` as defined here...
   |
   |
LL | fn expect_bound_supply_named<'x>() {
   |                              ^^
note: ...does not necessarily outlive the anonymous lifetime #1 defined here
   |
   |
LL |       closure_expecting_bound(|x: &'x u32| {
   |  _____________________________^
LL | |         //~^ ERROR mismatched types
LL | |         //~| ERROR mismatched types
LL | |
LL | |         f = Some(x);
LL | |     });
   | |_____^


error[E0308]: mismatched types
  --> /checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region-2.rs:14:33
   |
LL |     closure_expecting_bound(|x: &'x u32| {
   |                                 ^^^^^^^ lifetime mismatch
   = note: expected reference `&u32`
              found reference `&'x u32`
              found reference `&'x u32`
note: the anonymous lifetime #1 defined here...
---
---- [ui] ui/generator/resume-arg-late-bound.rs stdout ----
diff of stderr:

6    |
7    = note: expected type `for<'a> Generator<&'a mut bool>`
8               found type `Generator<&mut bool>`
- note: the anonymous lifetime #1 defined here doesn't meet the lifetime requirements
+ note: the required lifetime does not necessarily outlive the anonymous lifetime #1 defined here
11    |
11    |
12 LL |       let gen = |arg: &mut bool| {
29    |
29    |
30    = note: expected type `for<'a> Generator<&'a mut bool>`
31               found type `Generator<&mut bool>`
- note: the required lifetime does not necessarily outlive the anonymous lifetime #1 defined here
+ note: the anonymous lifetime #1 defined here doesn't meet the lifetime requirements
34    |
34    |
35 LL |       let gen = |arg: &mut bool| {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-arg-late-bound/resume-arg-late-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/resume-arg-late-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/resume-arg-late-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-arg-late-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-arg-late-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/generator/resume-arg-late-bound.rs:15:5
   |
LL |     test(gen);
   |     ^^^^ lifetime mismatch
   |
   = note: expected type `for<'a> Generator<&'a mut bool>`
              found type `Generator<&mut bool>`
note: the required lifetime does not necessarily outlive the anonymous lifetime #1 defined here
   |
   |
LL |       let gen = |arg: &mut bool| {
LL | |         yield ();
LL | |         *arg = true;
LL | |     };
   | |_____^
   | |_____^
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/generator/resume-arg-late-bound.rs:8:17
   |
LL | fn test(a: impl for<'a> Generator<&'a mut bool>) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/generator/resume-arg-late-bound.rs:15:5
   |
   |
LL |     test(gen);
   |     ^^^^ lifetime mismatch
   |
   = note: expected type `for<'a> Generator<&'a mut bool>`
              found type `Generator<&mut bool>`
note: the anonymous lifetime #1 defined here doesn't meet the lifetime requirements
   |
   |
LL |       let gen = |arg: &mut bool| {
LL | |         yield ();
LL | |         *arg = true;
LL | |     };
   | |_____^
   | |_____^
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/generator/resume-arg-late-bound.rs:8:17
   |
LL | fn test(a: impl for<'a> Generator<&'a mut bool>) {}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/hr-subtype/hr-subtype.rs#bound_a_b_ret_a_vs_bound_a_ret_a stdout ----


8 LL | | for<'a>    fn(&'a u32, &'a u32) -> &'a u32) }
10    |
10    |
-    = note: expected enum `Option<for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32>`
+    = note: expected enum `Option<for<'b, 'a> fn(&'a u32, &'b u32) -> &'a u32>`
12               found enum `Option<for<'a> fn(&'a u32, &'a u32) -> &'a u32>`
13    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_a_b_ret_a_vs_bound_a_ret_a/hr-subtype.bound_a_b_ret_a_vs_bound_a_ret_a.stderr
To only update this specific test, also pass `--test-args hr-subtype/hr-subtype.rs`


error in revision `bound_a_b_ret_a_vs_bound_a_ret_a`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/hr-subtype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "bound_a_b_ret_a_vs_bound_a_ret_a" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_a_b_ret_a_vs_bound_a_ret_a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_a_b_ret_a_vs_bound_a_ret_a/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/hr-subtype/hr-subtype.rs:45:26
   |
LL |               gimme::<$t1>(None::<$t2>);
   |                            ^^^^^^^^^^^ one type is more general than the other
...
LL | / check! { bound_a_b_ret_a_vs_bound_a_ret_a: (for<'a,'b> fn(&'a u32, &'b u32) -> &'a u32,
LL | | for<'a>    fn(&'a u32, &'a u32) -> &'a u32) }
   |
   |
   = note: expected enum `Option<for<'b, 'a> fn(&'a u32, &'b u32) -> &'a u32>`
              found enum `Option<for<'a> fn(&'a u32, &'a u32) -> &'a u32>`
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/hr-subtype/hr-subtype.rs#bound_inv_a_b_vs_bound_inv_a stdout ----


8 LL | | for<'a>    fn(Inv<'a>, Inv<'a>)) }
10    |
10    |
-    = note: expected enum `Option<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>`
+    = note: expected enum `Option<for<'b, 'a> fn(Inv<'a>, Inv<'b>)>`
12               found enum `Option<for<'a> fn(Inv<'a>, Inv<'a>)>`
13    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_inv_a_b_vs_bound_inv_a/hr-subtype.bound_inv_a_b_vs_bound_inv_a.stderr
To only update this specific test, also pass `--test-args hr-subtype/hr-subtype.rs`


error in revision `bound_inv_a_b_vs_bound_inv_a`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/hr-subtype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "bound_inv_a_b_vs_bound_inv_a" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_inv_a_b_vs_bound_inv_a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_inv_a_b_vs_bound_inv_a/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/hr-subtype/hr-subtype.rs:45:26
   |
LL |               gimme::<$t1>(None::<$t2>);
   |                            ^^^^^^^^^^^ one type is more general than the other
...
LL | / check! { bound_inv_a_b_vs_bound_inv_a: (for<'a,'b> fn(Inv<'a>, Inv<'b>),
LL | | for<'a>    fn(Inv<'a>, Inv<'a>)) }
   |
   |
   = note: expected enum `Option<for<'b, 'a> fn(Inv<'a>, Inv<'b>)>`
              found enum `Option<for<'a> fn(Inv<'a>, Inv<'a>)>`
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/issues/issue-17740.rs stdout ----
diff of stderr:

6    |
7    = note: expected struct `Foo<'a>`
8               found struct `Foo<'_>`
- note: the lifetime `'a` as defined here...
-   --> $DIR/issue-17740.rs:5:7
-    |
- LL | impl <'a> Foo<'a>{
-    |       ^^
- note: ...does not necessarily outlive the anonymous lifetime defined here
+ note: the anonymous lifetime defined here...
16    |
16    |
17 LL |     fn bar(self: &mut Foo) {
18    |                       ^^^
18    |                       ^^^
+ note: ...does not necessarily outlive the lifetime `'a` as defined here
+    |
+    |
+ LL | impl <'a> Foo<'a>{
19 
19 
20 error[E0308]: mismatched `self` parameter type

25    |
26    = note: expected struct `Foo<'a>`
27               found struct `Foo<'_>`
27               found struct `Foo<'_>`
- note: the anonymous lifetime defined here...
-   --> $DIR/issue-17740.rs:6:23
-    |
- LL |     fn bar(self: &mut Foo) {
-    |                       ^^^
- note: ...does not necessarily outlive the lifetime `'a` as defined here
+ note: the lifetime `'a` as defined here...
35    |
35    |
36 LL | impl <'a> Foo<'a>{
37    |       ^^
37    |       ^^
+ note: ...does not necessarily outlive the anonymous lifetime defined here
+    |
+    |
+ LL |     fn bar(self: &mut Foo) {
38 
39 error: aborting due to 2 previous errors
40 

---
To only update this specific test, also pass `--test-args issues/issue-17740.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17740.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17740" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17740/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn bar(self: &mut Foo) {
   |                  ^^^^^^^^ lifetime mismatch
   = note: expected struct `Foo<'a>`
              found struct `Foo<'_>`
note: the anonymous lifetime defined here...
  --> /checkout/src/test/ui/issues/issue-17740.rs:6:23
  --> /checkout/src/test/ui/issues/issue-17740.rs:6:23
   |
LL |     fn bar(self: &mut Foo) {
   |                       ^^^
note: ...does not necessarily outlive the lifetime `'a` as defined here
   |
   |
LL | impl <'a> Foo<'a>{


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn bar(self: &mut Foo) {
   |                  ^^^^^^^^ lifetime mismatch
   = note: expected struct `Foo<'a>`
              found struct `Foo<'_>`
              found struct `Foo<'_>`
note: the lifetime `'a` as defined here...
   |
   |
LL | impl <'a> Foo<'a>{
   |       ^^
note: ...does not necessarily outlive the anonymous lifetime defined here
   |
   |
LL |     fn bar(self: &mut Foo) {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
---
---- [ui] ui/issues/issue-27942.rs stdout ----
diff of stderr:

6    |
7    = note: expected type `Resources<'_>`
8               found type `Resources<'a>`
- note: the anonymous lifetime defined here...
-   --> $DIR/issue-27942.rs:5:15
-    |
- LL |     fn select(&self) -> BufferViewHandle<R>;
-    |               ^^^^^
- note: ...does not necessarily outlive the lifetime `'a` as defined here
+ note: the lifetime `'a` as defined here...
16    |
16    |
17 LL | pub trait Buffer<'a, R: Resources<'a>> {
18    |                  ^^
18    |                  ^^
+ note: ...does not necessarily outlive the anonymous lifetime defined here
+    |
+    |
+ LL |     fn select(&self) -> BufferViewHandle<R>;
19 
20 error[E0308]: mismatched types
21   --> $DIR/issue-27942.rs:5:25


25    |
26    = note: expected type `Resources<'_>`
27               found type `Resources<'a>`
- note: the lifetime `'a` as defined here...
-   --> $DIR/issue-27942.rs:3:18
-    |
- LL | pub trait Buffer<'a, R: Resources<'a>> {
-    |                  ^^
- note: ...does not necessarily outlive the anonymous lifetime defined here
+ note: the anonymous lifetime defined here...
35    |
35    |
36 LL |     fn select(&self) -> BufferViewHandle<R>;
37    |               ^^^^^
37    |               ^^^^^
+ note: ...does not necessarily outlive the lifetime `'a` as defined here
+    |
+    |
+ LL | pub trait Buffer<'a, R: Resources<'a>> {
38 
39 error: aborting due to 2 previous errors
40 

---
To only update this specific test, also pass `--test-args issues/issue-27942.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27942.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27942" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27942/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-27942.rs:5:25
   |
LL |     fn select(&self) -> BufferViewHandle<R>;
   |                         ^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected type `Resources<'_>`
              found type `Resources<'a>`
note: the lifetime `'a` as defined here...
   |
   |
LL | pub trait Buffer<'a, R: Resources<'a>> {
   |                  ^^
note: ...does not necessarily outlive the anonymous lifetime defined here
   |
   |
LL |     fn select(&self) -> BufferViewHandle<R>;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-27942.rs:5:25
   |
   |
LL |     fn select(&self) -> BufferViewHandle<R>;
   |                         ^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected type `Resources<'_>`
              found type `Resources<'a>`
note: the anonymous lifetime defined here...
   |
   |
LL |     fn select(&self) -> BufferViewHandle<R>;
   |               ^^^^^
note: ...does not necessarily outlive the lifetime `'a` as defined here
   |
   |
LL | pub trait Buffer<'a, R: Resources<'a>> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/lifetimes/issue-79187-2.rs stdout ----

error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:8: unexpected error: '8:5: 8:13: implementation of `FnOnce` is not general enough'
error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:8: expected error not found: mismatched types

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-79187-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:13: implementation of `FnOnce` is not general enough",
]

not found errors (from test file): [
    Error {
---

---- [ui] ui/lub-glb/old-lub-glb-hr-noteq1.rs stdout ----
diff of stderr:

10 LL | |     };
11    | |_____- `match` arms have incompatible types
12    |
-    = note: expected fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
+    = note: expected fn pointer `for<'b, 'a> fn(&'a u8, &'b u8) -> &'a u8`
14               found fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
16 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1/old-lub-glb-hr-noteq1.stderr
To only update this specific test, also pass `--test-args lub-glb/old-lub-glb-hr-noteq1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: `match` arms have incompatible types
  --> /checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs:11:14
   |
LL |       let z = match 22 {
LL | |         0 => x,
LL | |         0 => x,
   | |              - this is found to be of type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
LL | |         _ => y, //~ ERROR `match` arms have incompatible types
   | |              ^ one type is more general than the other
LL | |     };
   | |_____- `match` arms have incompatible types
   |
   = note: expected fn pointer `for<'b, 'a> fn(&'a u8, &'b u8) -> &'a u8`
              found fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/lub-glb/old-lub-glb-hr-noteq2.rs stdout ----
diff of stderr:

11    | |_____- `match` arms have incompatible types
12    |
13    = note: expected fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
-               found fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
+               found fn pointer `for<'b, 'a> fn(&'a u8, &'b u8) -> &'a u8`
16 error: aborting due to previous error
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2/old-lub-glb-hr-noteq2.stderr
To only update this specific test, also pass `--test-args lub-glb/old-lub-glb-hr-noteq2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: `match` arms have incompatible types
  --> /checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs:21:14
   |
LL |       let z = match 22 {
LL | |         0 => y,
LL | |         0 => y,
   | |              - this is found to be of type `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
LL | |         _ => x, //~ ERROR `match` arms have incompatible types
   | |              ^ one type is more general than the other
LL | |     };
   | |_____- `match` arms have incompatible types
   |
   = note: expected fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
              found fn pointer `for<'b, 'a> fn(&'a u8, &'b u8) -> &'a u8`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/mismatched_types/closure-mismatch.rs stdout ----

error: /checkout/src/test/ui/mismatched_types/closure-mismatch.rs:8: unexpected error: '8:5: 8:8: implementation of `FnOnce` is not general enough'
error: /checkout/src/test/ui/mismatched_types/closure-mismatch.rs:8: expected error not found: mismatched types

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/closure-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-mismatch/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:8: implementation of `FnOnce` is not general enough",
]

not found errors (from test file): [
    Error {
---

---- [ui] ui/mismatched_types/closure-arg-type-mismatch.rs stdout ----
diff of stderr:

40 LL |         F: FnMut(Self::Item) -> B,
41    |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
42 
- error: implementation of `FnOnce` is not general enough
-   --> $DIR/closure-arg-type-mismatch.rs:10:5
-    |
- LL |     baz(f);
-    |     ^^^ implementation of `FnOnce` is not general enough
-    |
-    = note: `fn(*mut &'a u32)` must implement `FnOnce<(*mut &'0 u32,)>`, for any lifetime `'0`...
-    = note: ...but it actually implements `FnOnce<(*mut &'a u32,)>`
52 error[E0308]: mismatched types
53   --> $DIR/closure-arg-type-mismatch.rs:10:5
54    |


57    |
58    = note: expected type `for<'r> Fn<(*mut &'r u32,)>`
59               found type `Fn<(*mut &'a u32,)>`
- note: the lifetime `'a` as defined here doesn't meet the lifetime requirements
+ note: the required lifetime does not necessarily outlive the lifetime `'a` as defined here
62    |
62    |
63 LL | fn _test<'a>(f: fn(*mut &'a u32)) {
76    |
76    |
77    = note: expected type `for<'r> Fn<(*mut &'r u32,)>`
78               found type `Fn<(*mut &'a u32,)>`
- note: the required lifetime does not necessarily outlive the lifetime `'a` as defined here
+ note: the lifetime `'a` as defined here doesn't meet the lifetime requirements
81    |
81    |
82 LL | fn _test<'a>(f: fn(*mut &'a u32)) {
86    |
86    |
87 LL | fn baz<F: Fn(*mut &u32)>(_: F) {}
+ 
+ 
+ error: implementation of `FnOnce` is not general enough
+    |
+    |
+ LL |     baz(f);
+    |     ^^^ implementation of `FnOnce` is not general enough
+    |
+    = note: `fn(*mut &'a u32)` must implement `FnOnce<(*mut &'0 u32,)>`, for any lifetime `'0`...
+    = note: ...but it actually implements `FnOnce<(*mut &'a u32,)>`
89 
90 error: implementation of `FnOnce` is not general enough


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch/closure-arg-type-mismatch.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch/closure-arg-type-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/closure-arg-type-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:3:14
   |
LL |     a.iter().map(|_: (u32, u32)| 45); //~ ERROR type mismatch
   |              ^^^ ------------------ found signature of `fn((u32, u32)) -> _`
   |              |
   |              expected signature of `fn(&(u32, u32)) -> _`
note: required by a bound in `map`
  --> /checkout/library/core/src/iter/traits/iterator.rs:684:12
   |
   |
LL |         F: FnMut(Self::Item) -> B,
   |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:4:14
   |
   |
LL |     a.iter().map(|_: &(u16, u16)| 45); //~ ERROR type mismatch
   |              ^^^ ------------------- found signature of `for<'r> fn(&'r (u16, u16)) -> _`
   |              |
   |              expected signature of `fn(&(u32, u32)) -> _`
note: required by a bound in `map`
  --> /checkout/library/core/src/iter/traits/iterator.rs:684:12
   |
   |
LL |         F: FnMut(Self::Item) -> B,
   |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:5:14
   |
   |
LL |     a.iter().map(|_: (u16, u16)| 45); //~ ERROR type mismatch
   |              ^^^ ------------------ found signature of `fn((u16, u16)) -> _`
   |              |
   |              expected signature of `fn(&(u32, u32)) -> _`
note: required by a bound in `map`
  --> /checkout/library/core/src/iter/traits/iterator.rs:684:12
   |
   |
LL |         F: FnMut(Self::Item) -> B,
   |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:10:5
   |
   |
LL |     baz(f);
   |     ^^^ lifetime mismatch
   |
   = note: expected type `for<'r> Fn<(*mut &'r u32,)>`
              found type `Fn<(*mut &'a u32,)>`
note: the required lifetime does not necessarily outlive the lifetime `'a` as defined here
   |
   |
LL | fn _test<'a>(f: fn(*mut &'a u32)) {
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:8:11
   |
   |
LL | fn baz<F: Fn(*mut &u32)>(_: F) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:10:5
   |
   |
LL |     baz(f);
   |     ^^^ lifetime mismatch
   |
   = note: expected type `for<'r> Fn<(*mut &'r u32,)>`
              found type `Fn<(*mut &'a u32,)>`
note: the lifetime `'a` as defined here doesn't meet the lifetime requirements
   |
   |
LL | fn _test<'a>(f: fn(*mut &'a u32)) {
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:8:11
   |
   |
LL | fn baz<F: Fn(*mut &u32)>(_: F) {}


error: implementation of `FnOnce` is not general enough
   |
   |
LL |     baz(f);
   |     ^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(*mut &'a u32)` must implement `FnOnce<(*mut &'0 u32,)>`, for any lifetime `'0`...
   = note: ...but it actually implements `FnOnce<(*mut &'a u32,)>`

error: implementation of `FnOnce` is not general enough
   |
   |
LL |     baz(f);
   |     ^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(*mut &'a u32)` must implement `FnOnce<(*mut &'0 u32,)>`, for any lifetime `'0`...
   = note: ...but it actually implements `FnOnce<(*mut &'a u32,)>`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0308, E0631.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/regions/regions-addr-of-upvar-self.rs stdout ----

error: /checkout/src/test/ui/regions/regions-addr-of-upvar-self.rs:8: unexpected error: '8:41: 8:55: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement [E0759]'
error: /checkout/src/test/ui/regions/regions-addr-of-upvar-self.rs:8: expected error not found: cannot infer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-addr-of-upvar-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-upvar-self" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-upvar-self/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:41: 8:55: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement [E0759]",
]

not found errors (from test file): [
    Error {
---
---- [ui] ui/regions/regions-infer-not-param.rs stdout ----
diff of stderr:

25    |
26    = note: expected struct `Indirect2<'b>`
27               found struct `Indirect2<'a>`
- note: the lifetime `'a` as defined here...
-   --> $DIR/regions-infer-not-param.rs:19:19
-    |
- LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
-    |                   ^^
- note: ...does not necessarily outlive the lifetime `'b` as defined here
+ note: the lifetime `'b` as defined here...
35    |
35    |
36 LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
37    |                      ^^
37    |                      ^^
+ note: ...does not necessarily outlive the lifetime `'a` as defined here
+    |
+    |
+ LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
38 
39 error[E0308]: mismatched types
40   --> $DIR/regions-infer-not-param.rs:19:63


44    |
45    = note: expected struct `Indirect2<'b>`
46               found struct `Indirect2<'a>`
- note: the lifetime `'b` as defined here...
-   --> $DIR/regions-infer-not-param.rs:19:22
-    |
- LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
-    |                      ^^
- note: ...does not necessarily outlive the lifetime `'a` as defined here
+ note: the lifetime `'a` as defined here...
54    |
54    |
55 LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
56    |                   ^^
56    |                   ^^
+ note: ...does not necessarily outlive the lifetime `'b` as defined here
+    |
+    |
+ LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
57 
58 error: aborting due to 3 previous errors
59 

---
To only update this specific test, also pass `--test-args regions/regions-infer-not-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-not-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-not-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-not-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/regions/regions-infer-not-param.rs:15:54
   |
LL | fn take_direct<'a,'b>(p: Direct<'a>) -> Direct<'b> { p } //~ ERROR mismatched types
   |                                                      ^ lifetime mismatch
   |
   = note: expected struct `Direct<'b>`
              found struct `Direct<'a>`
note: the lifetime `'a` as defined here...
   |
   |
LL | fn take_direct<'a,'b>(p: Direct<'a>) -> Direct<'b> { p } //~ ERROR mismatched types
   |                ^^
note: ...does not necessarily outlive the lifetime `'b` as defined here
   |
   |
LL | fn take_direct<'a,'b>(p: Direct<'a>) -> Direct<'b> { p } //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/regions/regions-infer-not-param.rs:19:63
   |
   |
LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p } //~ ERROR mismatched types
   |                                                               ^ lifetime mismatch
   |
   = note: expected struct `Indirect2<'b>`
              found struct `Indirect2<'a>`
note: the lifetime `'b` as defined here...
   |
   |
LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p } //~ ERROR mismatched types
   |                      ^^
note: ...does not necessarily outlive the lifetime `'a` as defined here
   |
   |
LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p } //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/regions/regions-infer-not-param.rs:19:63
   |
   |
LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p } //~ ERROR mismatched types
   |                                                               ^ lifetime mismatch
   |
   = note: expected struct `Indirect2<'b>`
              found struct `Indirect2<'a>`
note: the lifetime `'a` as defined here...
   |
   |
LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p } //~ ERROR mismatched types
   |                   ^^
note: ...does not necessarily outlive the lifetime `'b` as defined here
   |
   |
LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p } //~ ERROR mismatched types

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/regions/regions-nested-fns.rs stdout ----
diff of stderr:

4 LL |     let mut ay = &y;
6    |
6    |
- note: first, the lifetime cannot outlive the anonymous lifetime #1 defined here...
-   --> $DIR/regions-nested-fns.rs:7:58
+ note: first, the lifetime cannot outlive the lifetime `'x` as defined here...
9    |
9    |
- LL |       ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
-    |  __________________________________________________________^
- LL | |         ay = x;
- LL | |         ay = &y;
- LL | |         ay = z;
- LL | |     }));
-    | |_____^
+ LL | fn nested<'x>(x: &'x isize) {
+    |           ^^
17 note: ...so that reference does not outlive borrowed content
-   --> $DIR/regions-nested-fns.rs:10:14
19    |
19    |
- LL |         ay = z;
+ LL |         ay = x;
21    |              ^
22 note: but, the lifetime must be valid for the anonymous lifetime #1 defined here...


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/regions-nested-fns.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/regions-nested-fns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-nested-fns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-nested-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> /checkout/src/test/ui/regions/regions-nested-fns.rs:5:18
   |
LL |     let mut ay = &y; //~ ERROR E0495
   |
   |
note: first, the lifetime cannot outlive the lifetime `'x` as defined here...
   |
   |
LL | fn nested<'x>(x: &'x isize) {
   |           ^^
note: ...so that reference does not outlive borrowed content
   |
LL |         ay = x;
   |              ^
   |              ^
note: but, the lifetime must be valid for the anonymous lifetime #1 defined here...
   |
   |
LL |       ignore::< Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
   |  ________________________________________________________________________^
LL | |         if false { return x; } //~ ERROR E0312
LL | |         if false { return ay; }
LL | |         return z;
LL | |     }));
   | |_____^
note: ...so that the types are compatible
   |
   |
LL |       ignore::< Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
   |  ____________________________________________________________________________^
LL | |         if false { return x; } //~ ERROR E0312
LL | |         if false { return ay; }
LL | |         return z;
LL | |     }));
   | |_____^
   = note: expected `&isize`
              found `&isize`

error[E0312]: lifetime of reference outlives lifetime of borrowed content...
   |
   |
LL |         if false { return x; } //~ ERROR E0312
   |
   |
note: ...the reference is valid for the anonymous lifetime #1 defined here...
   |
   |
LL |       ignore::< Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
   |  ________________________________________________________________________^
LL | |         if false { return x; } //~ ERROR E0312
LL | |         if false { return ay; }
LL | |         return z;
LL | |     }));
   | |_____^
note: ...but the borrowed content is only valid for the lifetime `'x` as defined here
   |
   |
LL | fn nested<'x>(x: &'x isize) {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0312, E0495.
---
---- [ui] ui/rfc1623.rs stdout ----

error: /checkout/src/test/ui/rfc1623.rs:28: unexpected error: '28:8: 28:11: mismatched types [E0308]'

error: /checkout/src/test/ui/rfc1623.rs:28: expected error not found: implementation of `FnOnce` is not general enough
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1623.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623/auxiliary"
    Error {
        line_num: 28,
        kind: Some(
            Error,
---
        line_num: 28,
        kind: Some(
            Error,
        ),
        msg: "implementation of `FnOnce` is not general enough",
]

thread '[ui] ui/rfc1623.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13


---- [ui] ui/type-alias-impl-trait/issue-57611-trait-alias.rs stdout ----

error: /checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs:19: unexpected error: '19:22: 19:31: mismatched types [E0308]'

error: /checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs:19: expected error not found: implementation of `FnOnce` is not general enough
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias/auxiliary"
    Error {
        line_num: 19,
        kind: Some(
            Error,
---
        line_num: 19,
        kind: Some(
            Error,
        ),
        msg: "implementation of `FnOnce` is not general enough",
]

thread '[ui] ui/type-alias-impl-trait/issue-57611-trait-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13


---- [ui] ui/ufcs/ufcs-explicit-self-bad.rs stdout ----
diff of stderr:

71    |
72    = note: expected reference `&'a Bar<T>`
73               found reference `&Bar<T>`
- note: the anonymous lifetime defined here...
-   --> $DIR/ufcs-explicit-self-bad.rs:39:22
-    |
- LL |     fn dummy3(self: &&Bar<T>) {}
-    |                      ^^^^^^^
- note: ...does not necessarily outlive the lifetime `'a` as defined here
+ note: the lifetime `'a` as defined here...
80   --> $DIR/ufcs-explicit-self-bad.rs:35:6
81    |
82 LL | impl<'a, T> SomeTrait for &'a Bar<T> {
83    |      ^^
83    |      ^^
+ note: ...does not necessarily outlive the anonymous lifetime defined here
+   --> $DIR/ufcs-explicit-self-bad.rs:39:22
+    |
+ LL |     fn dummy3(self: &&Bar<T>) {}
84 
84 
85 error[E0308]: mismatched `self` parameter type
86   --> $DIR/ufcs-explicit-self-bad.rs:39:21
90    |
90    |
91    = note: expected reference `&'a Bar<T>`
92               found reference `&Bar<T>`
- note: the lifetime `'a` as defined here...
-   --> $DIR/ufcs-explicit-self-bad.rs:35:6
-    |
- LL | impl<'a, T> SomeTrait for &'a Bar<T> {
-    |      ^^
- note: ...does not necessarily outlive the anonymous lifetime defined here
+ note: the anonymous lifetime defined here...
99   --> $DIR/ufcs-explicit-self-bad.rs:39:22
100    |
101 LL |     fn dummy3(self: &&Bar<T>) {}
102    |                      ^^^^^^^
102    |                      ^^^^^^^
+ note: ...does not necessarily outlive the lifetime `'a` as defined here
+   --> $DIR/ufcs-explicit-self-bad.rs:35:6
+    |
+ LL | impl<'a, T> SomeTrait for &'a Bar<T> {
103 
104 error: aborting due to 7 previous errors
105 

---
To only update this specific test, also pass `--test-args ufcs/ufcs-explicit-self-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/ufcs/ufcs-explicit-self-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ufcs/ufcs-explicit-self-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ufcs/ufcs-explicit-self-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0307]: invalid `self` parameter type: isize
   |
   |
LL |     fn foo(self: isize, x: isize) -> isize {
   |
   |
   = note: type of `self` must be `Self` or a type that dereferences to it
   = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0307]: invalid `self` parameter type: Bar<isize>
   |
   |
LL |     fn foo(self: Bar<isize>, x: isize) -> isize {
   |
   |
   = note: type of `self` must be `Self` or a type that dereferences to it
   = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0307]: invalid `self` parameter type: &Bar<usize>
   |
   |
LL |     fn bar(self: &Bar<usize>, x: isize) -> isize {
   |
   |
   = note: type of `self` must be `Self` or a type that dereferences to it
   = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn dummy2(self: &Bar<T>) {} //~ ERROR mismatched `self` parameter type
   |                     ^^^^^^^ lifetime mismatch
   |
   = note: expected reference `&'a Bar<T>`
              found reference `&Bar<T>`
note: the anonymous lifetime defined here...
   |
   |
LL |     fn dummy2(self: &Bar<T>) {} //~ ERROR mismatched `self` parameter type
   |                     ^^^^^^^
note: ...does not necessarily outlive the lifetime `'a` as defined here
   |
   |
LL | impl<'a, T> SomeTrait for &'a Bar<T> {


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn dummy2(self: &Bar<T>) {} //~ ERROR mismatched `self` parameter type
   |                     ^^^^^^^ lifetime mismatch
   |
   = note: expected reference `&'a Bar<T>`
              found reference `&Bar<T>`
note: the lifetime `'a` as defined here...
   |
   |
LL | impl<'a, T> SomeTrait for &'a Bar<T> {
   |      ^^
note: ...does not necessarily outlive the anonymous lifetime defined here
   |
   |
LL |     fn dummy2(self: &Bar<T>) {} //~ ERROR mismatched `self` parameter type


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn dummy3(self: &&Bar<T>) {}
   |                     ^^^^^^^^ lifetime mismatch
   |
   = note: expected reference `&'a Bar<T>`
              found reference `&Bar<T>`
note: the lifetime `'a` as defined here...
   |
   |
LL | impl<'a, T> SomeTrait for &'a Bar<T> {
   |      ^^
note: ...does not necessarily outlive the anonymous lifetime defined here
   |
   |
LL |     fn dummy3(self: &&Bar<T>) {}


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn dummy3(self: &&Bar<T>) {}
   |                     ^^^^^^^^ lifetime mismatch
   |
   = note: expected reference `&'a Bar<T>`
              found reference `&Bar<T>`
note: the anonymous lifetime defined here...
   |
   |
LL |     fn dummy3(self: &&Bar<T>) {}
   |                      ^^^^^^^
note: ...does not necessarily outlive the lifetime `'a` as defined here
   |
   |
LL | impl<'a, T> SomeTrait for &'a Bar<T> {

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0307, E0308.
---
test result: FAILED. 12412 passed; 22 failed; 121 ignored; 0 measured; 0 filtered out; finished in 117.81s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:31
