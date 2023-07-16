plain

---- [ui] src/test/ui/associated-types/issue-87261.rs stdout ----
diff of stderr:

160 LL | fn returns_opaque_derived() -> impl DerivedTrait<Associated = ()> + 'static {
162 
162 
- error[E0271]: type mismatch resolving `<impl Foo + Trait as Trait>::Associated == ()`
+ error[E0271]: type mismatch resolving `<impl Trait + Foo as Trait>::Associated == ()`
165    |
165    |
166 LL | fn returns_opaque_foo() -> impl Trait + Foo {
170    |     ^^^^^^^^^^^^^ expected `()`, found associated type
171    |
172    = note:    expected unit type `()`
172    = note:    expected unit type `()`
-            found associated type `<impl Foo + Trait as Trait>::Associated`
+            found associated type `<impl Trait + Foo as Trait>::Associated`
174 note: required by a bound in `accepts_trait`
176    |


177 LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
178    |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
- help: consider constraining the associated type `<impl Foo + Trait as Trait>::Associated` to `()`
+ help: consider constraining the associated type `<impl Trait + Foo as Trait>::Associated` to `()`
180    |
181 LL | fn returns_opaque_foo() -> impl Trait<Associated = ()> + Foo {

183 
183 
- error[E0271]: type mismatch resolving `<impl Foo + DerivedTrait as Trait>::Associated == ()`
+ error[E0271]: type mismatch resolving `<impl DerivedTrait + Foo as Trait>::Associated == ()`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
186    |
186    |
187 LL | fn returns_opaque_derived_foo() -> impl DerivedTrait + Foo {
191    |     ^^^^^^^^^^^^^ expected `()`, found associated type
192    |
193    = note:    expected unit type `()`
193    = note:    expected unit type `()`
-            found associated type `<impl Foo + DerivedTrait as Trait>::Associated`
-    = help: consider constraining the associated type `<impl Foo + DerivedTrait as Trait>::Associated` to `()`
+            found associated type `<impl DerivedTrait + Foo as Trait>::Associated`
+    = help: consider constraining the associated type `<impl DerivedTrait + Foo as Trait>::Associated` to `()`
197 note: required by a bound in `accepts_trait`
198   --> $DIR/issue-87261.rs:43:27


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

244 
244 
- error[E0271]: type mismatch resolving `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated == ()`
+ error[E0271]: type mismatch resolving `<impl GenericTrait<()> + GenericTrait<u8> as GenericTrait<()>>::Associated == ()`
247    |
247    |
248 LL | fn returns_opaque_generic_duplicate() -> impl GenericTrait<()> + GenericTrait<u8> {
252    |     ^^^^^^^^^^^^^^^^^^^^^ expected `()`, found associated type
253    |
254    = note:    expected unit type `()`
254    = note:    expected unit type `()`
-            found associated type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated`
-    = help: consider constraining the associated type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated` to `()`
+            found associated type `<impl GenericTrait<()> + GenericTrait<u8> as GenericTrait<()>>::Associated`
+    = help: consider constraining the associated type `<impl GenericTrait<()> + GenericTrait<u8> as GenericTrait<()>>::Associated` to `()`
257    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
258 note: required by a bound in `accepts_generic_trait`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261/issue-87261.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261/issue-87261.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-87261.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-87261.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261/auxiliary"
stdout: none
--- stderr -------------------------------
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
           found associated type `<impl Trait as Trait>::Associated`
note: required by a bound in `accepts_trait`
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


error[E0271]: type mismatch resolving `<impl Trait + Foo as Trait>::Associated == ()`
   |
   |
LL | fn returns_opaque_foo() -> impl Trait + Foo {
...
...
LL |     accepts_trait(returns_opaque_foo());
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<impl Trait + Foo as Trait>::Associated`
note: required by a bound in `accepts_trait`
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<impl Trait + Foo as Trait>::Associated` to `()`
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


error[E0271]: type mismatch resolving `<impl GenericTrait<()> + GenericTrait<u8> as GenericTrait<()>>::Associated == ()`
   |
   |
LL | fn returns_opaque_generic_duplicate() -> impl GenericTrait<()> + GenericTrait<u8> {
...
...
LL |     accepts_generic_trait(returns_opaque_generic_duplicate());
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<impl GenericTrait<()> + GenericTrait<u8> as GenericTrait<()>>::Associated`
   = help: consider constraining the associated type `<impl GenericTrait<()> + GenericTrait<u8> as GenericTrait<()>>::Associated` to `()`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
