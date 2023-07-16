plain
....................................i................................................... 12584/14897
........................................................................................ 12672/14897
........................................................................................ 12760/14897
........................................................................................ 12848/14897
..............................................F......................................... 12936/14897
................................................F......F................................ 13024/14897
..................i..................................................................... 13200/14897
........................................................................................ 13288/14897
........................................................................................ 13376/14897
........................................................................................ 13464/14897
---
---- [ui] tests/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs stdout ----
diff of stderr:

148    |
149    = note: expected struct `Box<(dyn Trait + 'static)>`
150               found struct `Struct`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
154 help: try storing this on the heap by calling `Box::new`
155    |
156 LL |         return Box::new(Struct);
167    |
167    |
168    = note: expected struct `Box<(dyn Trait + 'static)>`
169                 found type `{integer}`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
173 help: try storing this on the heap by calling `Box::new`
175 LL |     Box::new(42)

186    |
186    |
187    = note: expected struct `Box<(dyn Trait + 'static)>`
188                 found type `{integer}`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
192 help: try storing this on the heap by calling `Box::new`
194 LL |         return Box::new(0);

205    |
205    |
206    = note: expected struct `Box<(dyn Trait + 'static)>`
207                 found type `{integer}`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
211 help: try storing this on the heap by calling `Box::new`
213 LL |     Box::new(42)

224    |
224    |
225    = note: expected struct `Box<(dyn Trait + 'static)>`
226               found struct `Struct`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
230 help: try storing this on the heap by calling `Box::new`
232 LL |         Box::new(Struct)

243    |
243    |
244    = note: expected struct `Box<(dyn Trait + 'static)>`
245                 found type `{integer}`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
Build completed unsuccessfully in 0:13:40
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
249 help: try storing this on the heap by calling `Box::new`
251 LL |         Box::new(42)

262    |
262    |
263    = note: expected struct `Box<(dyn Trait + 'static)>`
264                 found type `{integer}`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
268 help: try storing this on the heap by calling `Box::new`
270 LL |         Box::new(0)

281    |
281    |
282    = note: expected struct `Box<(dyn Trait + 'static)>`
283                 found type `{integer}`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
287 help: try storing this on the heap by calling `Box::new`
289 LL |         Box::new(42)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait/dyn-trait-return-should-be-impl-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/dyn-trait-return-should-be-impl-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait/auxiliary"
stdout: none
error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:7:35
   |
   |
LL | fn fuz() -> (usize, Trait) { (42, Struct) }
   |                                   ^^^^^^ expected `dyn Trait`, found `Struct`
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:7:13
   |
LL | fn fuz() -> (usize, Trait) { (42, Struct) }
   |             ^^^^^^^^^^^^^^   ------------ this returned value is of type `(usize, (dyn Trait + 'static))`
   |             doesn't have a size known at compile-time
   |
   |
   = help: within `(usize, (dyn Trait + 'static))`, the trait `Sized` is not implemented for `(dyn Trait + 'static)`
   = note: required because it appears within the type `(usize, dyn Trait)`

error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:10:39
   |
   |
LL | fn bar() -> (usize, dyn Trait) { (42, Struct) }
   |                                       ^^^^^^ expected `dyn Trait`, found `Struct`
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:10:13
   |
LL | fn bar() -> (usize, dyn Trait) { (42, Struct) }
   |             ^^^^^^^^^^^^^^^^^^   ------------ this returned value is of type `(usize, (dyn Trait + 'static))`
   |             doesn't have a size known at compile-time
   |
   |
   = help: within `(usize, (dyn Trait + 'static))`, the trait `Sized` is not implemented for `(dyn Trait + 'static)`
   = note: required because it appears within the type `(usize, dyn Trait)`

error[E0746]: return type cannot have an unboxed trait object
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:13:13
   |
   |
LL | fn bap() -> Trait { Struct }
   |
   |
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Trait` as the return type, as all return paths are of type `Struct`, which implements `Trait`
   |
LL | fn bap() -> impl Trait { Struct }

error[E0746]: return type cannot have an unboxed trait object
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:15:13
   |
   |
LL | fn ban() -> dyn Trait { Struct }
   |
   |
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Trait` as the return type, as all return paths are of type `Struct`, which implements `Trait`
   |
LL | fn ban() -> impl Trait { Struct }

error[E0746]: return type cannot have an unboxed trait object
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:17:13
   |
   |
LL | fn bak() -> dyn Trait { unimplemented!() } //~ ERROR E0746
   |
   |
help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
   |
LL | fn bak() -> impl Trait { unimplemented!() } //~ ERROR E0746
help: use a boxed trait object if all return paths implement trait `Trait`
   |
   |
LL | fn bak() -> Box<dyn Trait> { unimplemented!() } //~ ERROR E0746

error[E0746]: return type cannot have an unboxed trait object
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:19:13
   |
   |
LL | fn bal() -> dyn Trait { //~ ERROR E0746
   |
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl Trait` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn bal() -> Box<dyn Trait> { //~ ERROR E0746
help: ... and box this value
   |
   |
LL |         return Box::new(Struct);
help: ... and box this value
   |
LL |     Box::new(42)
   |     +++++++++  +
   |     +++++++++  +

error[E0308]: `if` and `else` have incompatible types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:29:9
LL | /     if true {
LL | |         Struct
   | |         ------ expected because of this
LL | |     } else {
LL | |     } else {
LL | |         42 //~ ERROR `if` and `else` have incompatible types
   | |         ^^ expected `Struct`, found integer
LL | |     }
   | |_____- `if` and `else` have incompatible types
error[E0746]: return type cannot have an unboxed trait object
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:25:13
   |
   |
LL | fn bax() -> dyn Trait { //~ ERROR E0746
   |
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl Trait` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn bax() -> Box<dyn Trait> { //~ ERROR E0746
help: ... and box this value
   |
LL |         Box::new(Struct)
   |         +++++++++      +
   |         +++++++++      +
help: ... and box this value
   |
LL |         Box::new(42) //~ ERROR `if` and `else` have incompatible types
   |         +++++++++  +
error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:34:16
   |
   |
LL | fn bam() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
LL |     if true {
LL |         return Struct; //~ ERROR mismatched types
   |                ^^^^^^ expected `Box<dyn Trait>`, found `Struct`
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
              found struct `Struct`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL |         return Box::new(Struct); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:36:5
   |
   |
LL | fn bam() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
...
LL |     42 //~ ERROR mismatched types
   |     ^^ expected `Box<dyn Trait>`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL |     Box::new(42) //~ ERROR mismatched types
   |     +++++++++  +
error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:40:16
   |
   |
LL | fn baq() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
LL |     if true {
LL |         return 0; //~ ERROR mismatched types
   |                ^ expected `Box<dyn Trait>`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL |         return Box::new(0); //~ ERROR mismatched types
   |                +++++++++ +
error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:42:5
   |
   |
LL | fn baq() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
...
LL |     42 //~ ERROR mismatched types
   |     ^^ expected `Box<dyn Trait>`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL |     Box::new(42) //~ ERROR mismatched types
   |     +++++++++  +
error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:46:9
   |
   |
LL | fn baz() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
LL |     if true {
LL |         Struct //~ ERROR mismatched types
   |         ^^^^^^ expected `Box<dyn Trait>`, found `Struct`
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
              found struct `Struct`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL |         Box::new(Struct) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:48:9
   |
   |
LL | fn baz() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
...
LL |         42 //~ ERROR mismatched types
   |         ^^ expected `Box<dyn Trait>`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL |         Box::new(42) //~ ERROR mismatched types
   |         +++++++++  +
error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:53:9
   |
   |
LL | fn baw() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
LL |     if true {
LL |         0 //~ ERROR mismatched types
   |         ^ expected `Box<dyn Trait>`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL |         Box::new(0) //~ ERROR mismatched types
   |         +++++++++ +
error[E0308]: mismatched types
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:55:9
   |
   |
LL | fn baw() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
...
LL |         42 //~ ERROR mismatched types
   |         ^^ expected `Box<dyn Trait>`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL |         Box::new(42) //~ ERROR mismatched types
   |         +++++++++  +
error[E0746]: return type cannot have an unboxed trait object
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:60:13
   |
   |
LL | fn bat() -> dyn Trait { //~ ERROR E0746
   |
   |
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Trait` as the return type, as all return paths are of type `{integer}`, which implements `Trait`
   |
LL | fn bat() -> impl Trait { //~ ERROR E0746

error[E0746]: return type cannot have an unboxed trait object
  --> fake-test-src-base/impl-trait/dyn-trait-return-should-be-impl-trait.rs:66:13
   |
   |
LL | fn bay() -> dyn Trait { //~ ERROR E0746
   |
   |
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Trait` as the return type, as all return paths are of type `{integer}`, which implements `Trait`
   |
LL | fn bay() -> impl Trait { //~ ERROR E0746

error: aborting due to 20 previous errors

Some errors have detailed explanations: E0277, E0308, E0746.
---
---- [ui] tests/ui/suggestions/issue-90213-expected-boxfuture-self-ice.rs stdout ----
diff of stderr:

8    |
9    = note: expected struct `Box<Option<S>>`
10                 found enum `Option<_>`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
15   --> $DIR/issue-90213-expected-boxfuture-self-ice.rs:7:8
16    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-90213-expected-boxfuture-self-ice/issue-90213-expected-boxfuture-self-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-90213-expected-boxfuture-self-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/issue-90213-expected-boxfuture-self-ice.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-90213-expected-boxfuture-self-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-90213-expected-boxfuture-self-ice/auxiliary" "--edition=2021"
stdout: none
error[E0308]: mismatched types
  --> fake-test-src-base/suggestions/issue-90213-expected-boxfuture-self-ice.rs:9:19
   |
   |
LL |         Self::foo(None) //~ ERROR mismatched types
   |         --------- ^^^^ expected `Box<Option<S>>`, found `Option<_>`
   |         arguments to this function are incorrect
   |
   |
   = note: expected struct `Box<Option<S>>`
                found enum `Option<_>`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
  --> fake-test-src-base/suggestions/issue-90213-expected-boxfuture-self-ice.rs:7:8
   |
   |
LL |     fn foo(_: Box<Option<S>>) {}
   |        ^^^ -----------------
help: try storing this on the heap by calling `Box::new`
   |
LL |         Self::foo(Box::new(None)) //~ ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] tests/ui/suggestions/suggest-boxed-empty-block.rs stdout ----

6    |
6    |
7    = note: expected struct `Box<_>`
8            found unit type `()`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
12 help: try storing this on the heap by calling `Box::new`
13    |
14 LL -     foo({});
23    |
23    |
24    = note: expected struct `Box<_>`
25            found unit type `()`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
29 help: try storing this on the heap by calling `Box::new`
30    |
31 LL -     bar(|| {});

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-boxed-empty-block/suggest-boxed-empty-block.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-boxed-empty-block.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/suggest-boxed-empty-block.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-boxed-empty-block" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-boxed-empty-block/auxiliary" "--edition=2021"
stdout: none
error[E0308]: mismatched types
  --> fake-test-src-base/suggestions/suggest-boxed-empty-block.rs:10:9
   |
   |
LL |     foo({}); //~ ERROR mismatched types
   |         ^^ expected `Box<_>`, found `()`
   |
   = note: expected struct `Box<_>`
           found unit type `()`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL -     foo({}); //~ ERROR mismatched types
LL +     foo(Box::new(())); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> fake-test-src-base/suggestions/suggest-boxed-empty-block.rs:11:12
   |
   |
LL |     bar(|| {}); //~ ERROR mismatched types
   |            ^^ expected `Box<_>`, found `()`
   |
   = note: expected struct `Box<_>`
           found unit type `()`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL -     bar(|| {}); //~ ERROR mismatched types
LL +     bar(|| Box::new(())); //~ ERROR mismatched types

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] tests/ui/suggestions/suggest-box.rs stdout ----
diff of stderr:

12    |
13    = note: expected struct `Box<dyn Fn() -> Result<(), ()>>`
14              found closure `[closure@$DIR/suggest-box.rs:4:47: 4:49]`
-    = note: for more on the distinction between the stack and the heap, see:
-            https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and
-            https://doc.rust-lang.org/std/boxed/index.html
+    = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
18 help: try storing this on the heap by calling `Box::new`
19    |
20 LL ~     let _x: Box<dyn Fn() -> Result<(), ()>> = Box::new(|| {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-box/suggest-box.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-box.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/suggest-box.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-box" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-box/auxiliary"
stdout: none
error[E0308]: mismatched types
  --> fake-test-src-base/suggestions/suggest-box.rs:4:47
   |
   |
LL |       let _x: Box<dyn Fn() -> Result<(), ()>> = || { //~ ERROR mismatched types
   | |             |
   | |             expected due to this
LL | |         Err(())?;
LL | |         Ok(())
LL | |         Ok(())
LL | |     };
   | |_____^ expected `Box<dyn Fn() -> Result<(), ()>>`, found closure
   = note: expected struct `Box<dyn Fn() -> Result<(), ()>>`
   = note: expected struct `Box<dyn Fn() -> Result<(), ()>>`
             found closure `[closure@fake-test-src-base/suggestions/suggest-box.rs:4:47: 4:49]`
   = note: for more on the distinction between the stack and the heap, see: https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: try storing this on the heap by calling `Box::new`
   |
LL ~     let _x: Box<dyn Fn() -> Result<(), ()>> = Box::new(|| { //~ ERROR mismatched types
LL |         Err(())?;
LL ~     });
   |

error: aborting due to previous error
