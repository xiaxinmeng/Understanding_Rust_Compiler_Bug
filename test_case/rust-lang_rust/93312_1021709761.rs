plain
.............i...................................................................................... 1300/12554
..............................................i..................................................... 1400/12554
.................................................................................................... 1500/12554
.................................................................................................... 1600/12554
.......i....................................................................F.F.......F..........F.. 1700/12554
.......F............................................................................................ 1800/12554
.................................................................................................... 2000/12554
.............................................................................................i...... 2100/12554
.................................................................................................... 2200/12554
.................................................................................................... 2300/12554
---
................i................................................................................... 10300/12554
............iiiiii.i..iiiiii.i...................................................................... 10400/12554
.................................................................................................... 10500/12554
.................................................................................................... 10600/12554
.......................................................................................F...F........ 10700/12554
.................................................................................................... 10900/12554
.................................................................................................... 11000/12554
.................................................................................................... 11100/12554
................................................................................ii.................. 11200/12554
................................................................................ii.................. 11200/12554
..........i......................................................................................... 11300/12554
.................................................................................................... 11400/12554
..........................................................................F....F.................... 11500/12554
.................................................................................................... 11700/12554
...............................................................................i.................... 11800/12554
.................................................................................................... 11900/12554
.................................................................................................... 12000/12554
---

---- [ui] ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs stdout ----
diff of stderr:

21 LL | impl !Send for dyn Object + Marker2 {}
22    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type
23 
- error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker1`
-   --> $DIR/coherence-impl-trait-for-marker-trait-negative.rs:15:1
-    |
- LL | impl !Marker1 for dyn Object + Marker2 { }
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker1`
- 
30 error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker2`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
31   --> $DIR/coherence-impl-trait-for-marker-trait-negative.rs:17:1


33 LL | impl !Marker2 for dyn Object + Marker2 { }
34    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker2`
+ 
+ error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker1`
+   --> $DIR/coherence-impl-trait-for-marker-trait-negative.rs:15:1
+    |
+ LL | impl !Marker1 for dyn Object + Marker2 { }
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker1`
36 error: aborting due to 5 previous errors
37 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative/coherence-impl-trait-for-marker-trait-negative.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-impl-trait-for-marker-trait-negative.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
   |
   |
LL | impl !Send for dyn Marker2 {} //~ ERROR E0117
   | |              |
   | |              |
   | |              `dyn Marker2` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead


error[E0321]: cross-crate traits with a default impl, like `Send`, can only be implemented for a struct/enum type, not `(dyn Object + 'static)`
   |
   |
LL | impl !Send for dyn Object {} //~ ERROR E0321
   | ^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type

error[E0321]: cross-crate traits with a default impl, like `Send`, can only be implemented for a struct/enum type, not `(dyn Object + Marker2 + 'static)`
   |
   |
LL | impl !Send for dyn Object + Marker2 {} //~ ERROR E0321
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type

error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker2`
   |
   |
LL | impl !Marker2 for dyn Object + Marker2 { }   //~ ERROR E0371
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker2`

error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker1`
   |
   |
LL | impl !Marker1 for dyn Object + Marker2 { }   //~ ERROR E0371
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker1`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0117, E0321, E0371.
For more information about an error, try `rustc --explain E0117`.
For more information about an error, try `rustc --explain E0117`.

------------------------------------------


---- [ui] ui/coherence/coherence-impl-trait-for-marker-trait-positive.rs stdout ----
diff of stderr:

21 LL | unsafe impl Send for dyn Object + Marker2 {}
22    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type
23 
- error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker1`
-   --> $DIR/coherence-impl-trait-for-marker-trait-positive.rs:15:1
-    |
- LL | impl Marker1 for dyn Object + Marker2 { }
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker1`
- 
30 error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker2`
31   --> $DIR/coherence-impl-trait-for-marker-trait-positive.rs:17:1


33 LL | impl Marker2 for dyn Object + Marker2 { }
34    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker2`
+ 
+ error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker1`
+   --> $DIR/coherence-impl-trait-for-marker-trait-positive.rs:15:1
+    |
+ LL | impl Marker1 for dyn Object + Marker2 { }
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker1`
36 error: aborting due to 5 previous errors
37 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive/coherence-impl-trait-for-marker-trait-positive.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-impl-trait-for-marker-trait-positive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
   |
   |
LL | unsafe impl Send for dyn Marker2 {} //~ ERROR E0117
   | |                    |
   | |                    |
   | |                    `dyn Marker2` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead


error[E0321]: cross-crate traits with a default impl, like `Send`, can only be implemented for a struct/enum type, not `(dyn Object + 'static)`
   |
   |
LL | unsafe impl Send for dyn Object {} //~ ERROR E0321
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type

error[E0321]: cross-crate traits with a default impl, like `Send`, can only be implemented for a struct/enum type, not `(dyn Object + Marker2 + 'static)`
   |
   |
LL | unsafe impl Send for dyn Object + Marker2 {} //~ ERROR E0321
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type

error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker2`
   |
   |
LL | impl Marker2 for dyn Object + Marker2 { }   //~ ERROR E0371
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker2`

error[E0371]: the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker1`
   |
   |
LL | impl Marker1 for dyn Object + Marker2 { }   //~ ERROR E0371
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Object + Marker2 + 'static)` automatically implements trait `Marker1`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0117, E0321, E0371.
For more information about an error, try `rustc --explain E0117`.
For more information about an error, try `rustc --explain E0117`.

------------------------------------------


---- [ui] ui/coherence/coherence-impl-trait-for-trait.rs stdout ----
diff of stderr:

- error[E0371]: the object type `(dyn Baz + 'static)` automatically implements the trait `Foo`
-   --> $DIR/coherence-impl-trait-for-trait.rs:9:1
-    |
- LL | impl Foo for dyn Baz { }
-    | ^^^^^^^^^^^^^^^^^^^^ `(dyn Baz + 'static)` automatically implements trait `Foo`
- 
7 error[E0371]: the object type `(dyn Baz + 'static)` automatically implements the trait `Bar`
8   --> $DIR/coherence-impl-trait-for-trait.rs:11:1

15    |
15    |
16 LL | impl Baz for dyn Baz { }
17    | ^^^^^^^^^^^^^^^^^^^^ `(dyn Baz + 'static)` automatically implements trait `Baz`
+ 
+ error[E0371]: the object type `(dyn Baz + 'static)` automatically implements the trait `Foo`
+   --> $DIR/coherence-impl-trait-for-trait.rs:9:1
+    |
+ LL | impl Foo for dyn Baz { }
+    | ^^^^^^^^^^^^^^^^^^^^ `(dyn Baz + 'static)` automatically implements trait `Foo`
19 error: aborting due to 3 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-trait/coherence-impl-trait-for-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-impl-trait-for-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-impl-trait-for-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0371]: the object type `(dyn Baz + 'static)` automatically implements the trait `Bar`
   |
   |
LL | impl Bar for dyn Baz { }
   | ^^^^^^^^^^^^^^^^^^^^ `(dyn Baz + 'static)` automatically implements trait `Bar`

error[E0371]: the object type `(dyn Baz + 'static)` automatically implements the trait `Baz`
   |
   |
LL | impl Baz for dyn Baz { }
   | ^^^^^^^^^^^^^^^^^^^^ `(dyn Baz + 'static)` automatically implements trait `Baz`

error[E0371]: the object type `(dyn Baz + 'static)` automatically implements the trait `Foo`
   |
   |
LL | impl Foo for dyn Baz { }
   | ^^^^^^^^^^^^^^^^^^^^ `(dyn Baz + 'static)` automatically implements trait `Foo`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0371`.


------------------------------------------


---- [ui] ui/coherence/coherence-overlap-messages.rs stdout ----
diff of stderr:

- error[E0119]: conflicting implementations of trait `Foo`
-   --> $DIR/coherence-overlap-messages.rs:4:1
+ error[E0119]: conflicting implementations of trait `Baz<u8>` for type `u8`
+   --> $DIR/coherence-overlap-messages.rs:17:1
3    |
- LL | impl<T> Foo for T {}
-    | ----------------- first implementation here
- LL | impl<U> Foo for U {}
-    | ^^^^^^^^^^^^^^^^^ conflicting implementation
+ LL | impl<T> Baz<u8> for T {}
+    | --------------------- first implementation here
+ LL | impl<T> Baz<T> for u8 {}
+    | ^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u8`
8 
9 error[E0119]: conflicting implementations of trait `Bar` for type `(u8, u8)`
10   --> $DIR/coherence-overlap-messages.rs:11:1

14 LL | impl<T> Bar for (u8, T) {}
15    | ^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(u8, u8)`
16 
- error[E0119]: conflicting implementations of trait `Baz<u8>` for type `u8`
-   --> $DIR/coherence-overlap-messages.rs:17:1
-    |
- LL | impl<T> Baz<u8> for T {}
-    | --------------------- first implementation here
- LL | impl<T> Baz<T> for u8 {}
-    | ^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u8`
- 
25 error[E0119]: conflicting implementations of trait `Quux<_, _>`
26   --> $DIR/coherence-overlap-messages.rs:23:1

38 ...
38 ...
39 LL | impl<T, V> Quux<T, V> for T {}
+ 
+ error[E0119]: conflicting implementations of trait `Foo`
+   --> $DIR/coherence-overlap-messages.rs:4:1
+    |
+    |
+ LL | impl<T> Foo for T {}
+    | ----------------- first implementation here
+ LL | impl<U> Foo for U {}
41 
42 error: aborting due to 5 previous errors
43 

---
To only update this specific test, also pass `--test-args coherence/coherence-overlap-messages.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-overlap-messages.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-messages" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-messages/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0119]: conflicting implementations of trait `Baz<u8>` for type `u8`
   |
   |
LL | impl<T> Baz<u8> for T {}
   | --------------------- first implementation here
LL | impl<T> Baz<T> for u8 {}
   | ^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u8`

error[E0119]: conflicting implementations of trait `Bar` for type `(u8, u8)`
   |
   |
LL | impl<T> Bar for (T, u8) {}
   | ----------------------- first implementation here
LL | impl<T> Bar for (u8, T) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(u8, u8)`

error[E0119]: conflicting implementations of trait `Quux<_, _>`
   |
   |
LL | impl<T, U, V> Quux<U, V> for T {}
   | ------------------------------ first implementation here
LL | impl<T, U> Quux<U, U> for T {}


error[E0119]: conflicting implementations of trait `Quux<_, _>`
   |
   |
LL | impl<T, U, V> Quux<U, V> for T {}
   | ------------------------------ first implementation here
...
LL | impl<T, V> Quux<T, V> for T {}

error[E0119]: conflicting implementations of trait `Foo`
  --> /checkout/src/test/ui/coherence/coherence-overlap-messages.rs:4:1
   |
   |
LL | impl<T> Foo for T {}
   | ----------------- first implementation here
LL | impl<U> Foo for U {}

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0119`.
For more information about this error, try `rustc --explain E0119`.

------------------------------------------


---- [ui] ui/coherence/coherence-orphan.rs stdout ----
diff of stderr:

1 error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
-   --> $DIR/coherence-orphan.rs:10:1
+   --> $DIR/coherence-orphan.rs:17:1
3    |
- LL | impl TheTrait<usize> for isize { }
-    | |    |                   |
-    | |    |                   |
-    | |    |                   `isize` is not defined in the current crate
-    | |    `usize` is not defined in the current crate
+ LL | impl !Send for Vec<isize> { }
+    | |              |
+    | |              `Vec` is not defined in the current crate
9    | impl doesn't use only types from inside the current crate
10    |
10    |
11    = note: define and implement a trait or new type instead

12 
13 error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
-   --> $DIR/coherence-orphan.rs:17:1
+   --> $DIR/coherence-orphan.rs:10:1
15    |
- LL | impl !Send for Vec<isize> { }
-    | |              |
-    | |              `Vec` is not defined in the current crate
-    | |              `Vec` is not defined in the current crate
+ LL | impl TheTrait<usize> for isize { }
+    | |    |                   |
+    | |    |                   |
+    | |    |                   `isize` is not defined in the current crate
+    | |    `usize` is not defined in the current crate
20    | impl doesn't use only types from inside the current crate
22    = note: define and implement a trait or new type instead


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-orphan/coherence-orphan.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-orphan.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-orphan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-orphan" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-orphan/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
   |
   |
LL | impl !Send for Vec<isize> { }
   | |              |
   | |              `Vec` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   |
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
   |
   |
LL | impl TheTrait<usize> for isize { }
   | |    |                   |
   | |    |                   |
   | |    |                   `isize` is not defined in the current crate
   | |    `usize` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead

error: aborting due to 2 previous errors

---

8    = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
9    = help: consider using `min_specialization` instead, which is more stable and complete
10 
- error[E0119]: conflicting implementations of trait `Foo` for type `std::vec::Vec<_>`
-   --> $DIR/specialization-overlap.rs:5:1
+ error[E0119]: conflicting implementations of trait `Baz<u8>` for type `u8`
13    |
13    |
- LL | impl<T: Clone> Foo for T {}
-    | ------------------------ first implementation here
- LL | impl<T> Foo for Vec<T> {}
-    | ^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `std::vec::Vec<_>`
+ LL | impl<T> Baz<T> for u8 {}
+    | --------------------- first implementation here
+ LL | impl<T> Baz<u8> for T {}
+    | ^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u8`
18 
19 error[E0119]: conflicting implementations of trait `Bar` for type `(u8, u8)`


24 LL | impl<T> Bar for (u8, T) {}
25    | ^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(u8, u8)`
26 
- error[E0119]: conflicting implementations of trait `Baz<u8>` for type `u8`
-   --> $DIR/specialization-overlap.rs:13:1
-    |
- LL | impl<T> Baz<T> for u8 {}
-    | --------------------- first implementation here
- LL | impl<T> Baz<u8> for T {}
-    | ^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u8`
- 
35 error[E0119]: conflicting implementations of trait `Qux`
37    |

39    | ------------------------ first implementation here
39    | ------------------------ first implementation here
40 LL | impl<T: Eq> Qux for T {}
+ 
+ error[E0119]: conflicting implementations of trait `Foo` for type `std::vec::Vec<_>`
+   --> $DIR/specialization-overlap.rs:5:1
+    |
+    |
+ LL | impl<T: Clone> Foo for T {}
+    | ------------------------ first implementation here
+ LL | impl<T> Foo for Vec<T> {}
42 
43 error: aborting due to 4 previous errors; 1 warning emitted
44 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-overlap/specialization-overlap.stderr
To only update this specific test, also pass `--test-args specialization/specialization-overlap.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/specialization-overlap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-overlap" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-overlap/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/specialization-overlap.rs:1:12
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0119]: conflicting implementations of trait `Baz<u8>` for type `u8`
   |
   |
LL | impl<T> Baz<T> for u8 {}
   | --------------------- first implementation here
LL | impl<T> Baz<u8> for T {} //~ ERROR E0119
   | ^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u8`

error[E0119]: conflicting implementations of trait `Bar` for type `(u8, u8)`
   |
   |
LL | impl<T> Bar for (T, u8) {}
   | ----------------------- first implementation here
LL | impl<T> Bar for (u8, T) {} //~ ERROR E0119
   | ^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(u8, u8)`

error[E0119]: conflicting implementations of trait `Qux`
   |
   |
LL | impl<T: Clone> Qux for T {}
   | ------------------------ first implementation here
LL | impl<T: Eq> Qux for T {} //~ ERROR E0119

error[E0119]: conflicting implementations of trait `Foo` for type `std::vec::Vec<_>`
  --> /checkout/src/test/ui/specialization/specialization-overlap.rs:5:1
   |
   |
LL | impl<T: Clone> Foo for T {}
   | ------------------------ first implementation here
LL | impl<T> Foo for Vec<T> {} //~ ERROR E0119

error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0119`.
---

8    = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
9    = help: consider using `min_specialization` instead, which is more stable and complete
10 
- error[E0751]: found both positive and negative implementation of trait `Foo` for type `u8`:
-   --> $DIR/specialization-polarity.rs:10:1
-    |
- LL | impl<T> Foo for T {}
-    | ----------------- positive implementation here
- LL | impl !Foo for u8 {}
-    | ^^^^^^^^^^^^^^^^ negative implementation here
- 
19 error[E0751]: found both positive and negative implementation of trait `Bar` for type `u8`:
21    |

23    | ------------------ negative implementation here
23    | ------------------ negative implementation here
24 LL | impl Bar for u8 {}
25    | ^^^^^^^^^^^^^^^ positive implementation here
+ 
+ error[E0751]: found both positive and negative implementation of trait `Foo` for type `u8`:
+    |
+    |
+ LL | impl<T> Foo for T {}
+    | ----------------- positive implementation here
+ LL | impl !Foo for u8 {}
+    | ^^^^^^^^^^^^^^^^ negative implementation here
27 error: aborting due to 2 previous errors; 1 warning emitted
28 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-polarity/specialization-polarity.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/specialization-polarity.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/specialization-polarity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-polarity" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-polarity/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/specialization-polarity.rs:5:12
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0751]: found both positive and negative implementation of trait `Bar` for type `u8`:
   |
   |
LL | impl<T> !Bar for T {}
   | ------------------ negative implementation here
LL | impl Bar for u8 {} //~ ERROR E0751
   | ^^^^^^^^^^^^^^^ positive implementation here

error[E0751]: found both positive and negative implementation of trait `Foo` for type `u8`:
   |
   |
LL | impl<T> Foo for T {}
   | ----------------- positive implementation here
LL | impl !Foo for u8 {} //~ ERROR E0751
   | ^^^^^^^^^^^^^^^^ negative implementation here
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0751`.


------------------------------------------


---- [ui] ui/traits/issue-33140-hack-boundaries.rs stdout ----
diff of stderr:

- error[E0119]: conflicting implementations of trait `Trait1` for type `(dyn std::marker::Send + 'static)`
-   --> $DIR/issue-33140-hack-boundaries.rs:18:1
+ error[E0119]: conflicting implementations of trait `Trait5` for type `(dyn std::marker::Send + 'static)`
3    |
3    |
- LL | impl Trait1 for dyn Send {}
+ LL | impl Trait5 for dyn Send {}
5    | ------------------------ first implementation here
- LL | impl Trait1 for dyn Send {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`
+ LL | impl Trait5 for dyn Send where u32: Copy {}
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`
8 
- error[E0751]: found both positive and negative implementation of trait `Trait2` for type `(dyn std::marker::Send + 'static)`:
-   --> $DIR/issue-33140-hack-boundaries.rs:25:1
-    |
- LL | impl Trait2 for dyn Send {}
-    | ------------------------ positive implementation here
- LL | impl !Trait2 for dyn Send {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^ negative implementation here
- 
17 error[E0119]: conflicting implementations of trait `Trait3<(dyn std::marker::Sync + 'static)>` for type `(dyn std::marker::Send + 'static)`
19    |


22 LL | impl Trait3<dyn Sync> for dyn Send {}
23    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`
24 
+ error[E0751]: found both positive and negative implementation of trait `Trait2` for type `(dyn std::marker::Send + 'static)`:
+    |
+    |
+ LL | impl Trait2 for dyn Send {}
+    | ------------------------ positive implementation here
+ LL | impl !Trait2 for dyn Send {}
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^ negative implementation here
+ 
25 error[E0119]: conflicting implementations of trait `Trait4a` for type `(dyn std::marker::Send + 'static)`
27    |


30 LL | impl Trait4a for dyn Send {}
31    | ^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`
32 
- error[E0119]: conflicting implementations of trait `Trait4b` for type `()`
-   --> $DIR/issue-33140-hack-boundaries.rs:46:1
+ error[E0119]: conflicting implementations of trait `Trait4d` for type `dyn std::marker::Send`
35    |
35    |
- LL | impl Trait4b for () {}
-    | ------------------- first implementation here
- LL | impl Trait4b for () {}
-    | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `()`
+ LL | impl<'a> Trait4d for dyn Send + 'a {}
+    | ---------------------------------- first implementation here
+ LL | impl<'a> Trait4d for dyn Send + 'a {}
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `dyn std::marker::Send`
40 
41 error[E0119]: conflicting implementations of trait `Trait4c` for type `(dyn Trait1 + std::marker::Send + 'static)`


46 LL | impl Trait4c for dyn Trait1 + Send {}
47    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn Trait1 + std::marker::Send + 'static)`
48 
- error[E0119]: conflicting implementations of trait `Trait4d` for type `dyn std::marker::Send`
-   --> $DIR/issue-33140-hack-boundaries.rs:60:1
+ error[E0119]: conflicting implementations of trait `Trait4b` for type `()`
51    |
51    |
- LL | impl<'a> Trait4d for dyn Send + 'a {}
-    | ---------------------------------- first implementation here
- LL | impl<'a> Trait4d for dyn Send + 'a {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `dyn std::marker::Send`
+ LL | impl Trait4b for () {}
+    | ------------------- first implementation here
+ LL | impl Trait4b for () {}
+    | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `()`
56 
- error[E0119]: conflicting implementations of trait `Trait5` for type `(dyn std::marker::Send + 'static)`
-   --> $DIR/issue-33140-hack-boundaries.rs:67:1
+ error[E0119]: conflicting implementations of trait `Trait1` for type `(dyn std::marker::Send + 'static)`
59    |
59    |
- LL | impl Trait5 for dyn Send {}
+ LL | impl Trait1 for dyn Send {}
61    | ------------------------ first implementation here
- LL | impl Trait5 for dyn Send where u32: Copy {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`
+ LL | impl Trait1 for dyn Send {}
+    | ^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`
65 error: aborting due to 8 previous errors
66 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-33140-hack-boundaries/issue-33140-hack-boundaries.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-33140-hack-boundaries.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-33140-hack-boundaries.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-33140-hack-boundaries" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-33140-hack-boundaries/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0119]: conflicting implementations of trait `Trait5` for type `(dyn std::marker::Send + 'static)`
   |
   |
LL | impl Trait5 for dyn Send {}
   | ------------------------ first implementation here
LL | impl Trait5 for dyn Send where u32: Copy {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`

error[E0119]: conflicting implementations of trait `Trait3<(dyn std::marker::Sync + 'static)>` for type `(dyn std::marker::Send + 'static)`
   |
   |
LL | impl Trait3<dyn Sync> for dyn Send {}
   | ---------------------------------- first implementation here
LL | impl Trait3<dyn Sync> for dyn Send {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`

error[E0751]: found both positive and negative implementation of trait `Trait2` for type `(dyn std::marker::Send + 'static)`:
   |
   |
LL | impl Trait2 for dyn Send {}
   | ------------------------ positive implementation here
LL | impl !Trait2 for dyn Send {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^ negative implementation here

error[E0119]: conflicting implementations of trait `Trait4a` for type `(dyn std::marker::Send + 'static)`
   |
   |
LL | impl<T: ?Sized> Trait4a for T {}
   | ----------------------------- first implementation here
LL | impl Trait4a for dyn Send {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`

error[E0119]: conflicting implementations of trait `Trait4d` for type `dyn std::marker::Send`
   |
   |
LL | impl<'a> Trait4d for dyn Send + 'a {}
   | ---------------------------------- first implementation here
LL | impl<'a> Trait4d for dyn Send + 'a {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `dyn std::marker::Send`

error[E0119]: conflicting implementations of trait `Trait4c` for type `(dyn Trait1 + std::marker::Send + 'static)`
   |
   |
LL | impl Trait4c for dyn Trait1 + Send {}
   | ---------------------------------- first implementation here
LL | impl Trait4c for dyn Trait1 + Send {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn Trait1 + std::marker::Send + 'static)`

error[E0119]: conflicting implementations of trait `Trait4b` for type `()`
   |
   |
LL | impl Trait4b for () {}
   | ------------------- first implementation here
LL | impl Trait4b for () {}
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `()`

error[E0119]: conflicting implementations of trait `Trait1` for type `(dyn std::marker::Send + 'static)`
   |
   |
LL | impl Trait1 for dyn Send {}
   | ------------------------ first implementation here
LL | impl Trait1 for dyn Send {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + 'static)`
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0119, E0751.
For more information about an error, try `rustc --explain E0119`.
For more information about an error, try `rustc --explain E0119`.

------------------------------------------


---- [ui] ui/traits/issue-33140.rs stdout ----
diff of stderr:

- error[E0119]: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`
-   --> $DIR/issue-33140.rs:9:1
-    |
- LL | impl Trait for dyn Send + Sync {
-    | ------------------------------ first implementation here
- ...
- LL | impl Trait for dyn Sync + Send {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
- 
10 error[E0119]: conflicting implementations of trait `Trait2` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`
12    |

15 ...
15 ...
16 LL | impl Trait2 for dyn Sync + Send + Sync {
17    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
+ 
+ error[E0119]: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`
+    |
+    |
+ LL | impl Trait for dyn Send + Sync {
+    | ------------------------------ first implementation here
+ ...
+ LL | impl Trait for dyn Sync + Send {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
18 
19 error[E0592]: duplicate definitions with name `abc`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-33140/issue-33140.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-33140/issue-33140.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-33140.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-33140.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-33140" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-33140/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0119]: conflicting implementations of trait `Trait2` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`
   |
   |
LL | impl Trait2 for dyn Send + Sync {
   | ------------------------------- first implementation here
...
LL | impl Trait2 for dyn Sync + Send + Sync {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`

error[E0119]: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`
   |
   |
LL | impl Trait for dyn Send + Sync {
   | ------------------------------ first implementation here
...
LL | impl Trait for dyn Sync + Send {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`

error[E0592]: duplicate definitions with name `abc`
   |
   |
LL |     fn abc() -> bool { //~ ERROR duplicate definitions with name `abc`
   |     ^^^^^^^^^^^^^^^^ duplicate definitions for `abc`
LL |     fn abc() -> bool {
LL |     fn abc() -> bool {
   |     ---------------- other definition for `abc`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0119, E0592.
For more information about an error, try `rustc --explain E0119`.
---
test result: FAILED. 12424 passed; 9 failed; 121 ignored; 0 measured; 0 filtered out; finished in 100.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:43
