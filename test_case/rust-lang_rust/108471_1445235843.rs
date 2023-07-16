plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 3608/14520
....................iiiii............................................................... 3696/14520
........................................................................................ 3784/14520
........................................................................................ 3872/14520
........................F.F............................................................. 3960/14520
........................................................................................ 4136/14520
........................................................................................ 4224/14520
...........i..........i..........i...................................................... 4312/14520
...........i..........i..........i...................................................... 4312/14520
...........F..F......................................................................... 4400/14520
..............................................................................i......... 4576/14520
........................................................................................ 4664/14520
........................................................................................ 4664/14520
........................................F.FF............................................ 4752/14520
..............................................F......................................... 4840/14520
..................................................................................i..... 5016/14520
........................................................................................ 5104/14520
........................................................................................ 5192/14520
........................................................................................ 5280/14520
---
........................................................................................ 8008/14520
....................................ii..............................................ii.. 8096/14520
...............................................................i........................ 8184/14520
........................................................................................ 8272/14520
...........F......................F............................................ii....... 8360/14520
........................................................................................ 8536/14520
......F................................................................................. 8624/14520
...................................ii.................i......i..ii...................... 8712/14520
........................................................................................ 8800/14520
---

---- [ui] tests/ui/check-static-values-constraints.rs stdout ----
diff of stderr:

- error[E0493]: destructor of `SafeStruct` cannot be evaluated at compile-time
-   --> $DIR/check-static-values-constraints.rs:65:43
-    |
- LL |                                           ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),
- LL | |
- LL | |
- LL | |                                                      field2: SafeEnum::Variant1}};
-    | |                                                                                ^- value is dropped here
-    |                                                                                  the destructor for this type cannot be evaluated in statics
- 
- error[E0010]: allocations are not allowed in statics
+ error: expected expression, found reserved keyword `box`
+ error: expected expression, found reserved keyword `box`
13   --> $DIR/check-static-values-constraints.rs:79:33
14    |
15 LL | static STATIC11: Box<MyOwned> = box MyOwned;
-    |                                 ^^^^^^^^^^^ allocation not allowed in statics
+    |                                 ^^^ expected expression
17 
17 
- error[E0015]: cannot call non-const fn `<str as ToString>::to_string` in statics
-   --> $DIR/check-static-values-constraints.rs:89:38
-    |
- LL |     field2: SafeEnum::Variant4("str".to_string())
-    |
-    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
-    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
-    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
-    = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell
+ error: aborting due to previous error
- error[E0010]: allocations are not allowed in statics
-   --> $DIR/check-static-values-constraints.rs:94:5
-    |
-    |
- LL |     box MyOwned,
-    |     ^^^^^^^^^^^ allocation not allowed in statics
- error[E0010]: allocations are not allowed in statics
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/check-static-values-constraints.rs:95:5
-    |
-    |
- LL |     box MyOwned,
-    |     ^^^^^^^^^^^ allocation not allowed in statics
- error[E0010]: allocations are not allowed in statics
-   --> $DIR/check-static-values-constraints.rs:99:6
-    |
-    |
- LL |     &box MyOwned,
-    |      ^^^^^^^^^^^ allocation not allowed in statics
- error[E0010]: allocations are not allowed in statics
-   --> $DIR/check-static-values-constraints.rs:100:6
-    |
-    |
- LL |     &box MyOwned,
-    |      ^^^^^^^^^^^ allocation not allowed in statics
- error[E0010]: allocations are not allowed in statics
-   --> $DIR/check-static-values-constraints.rs:106:5
-    |
- LL |     box 3;
- LL |     box 3;
-    |     ^^^^^ allocation not allowed in statics
- 
- error[E0507]: cannot move out of static item `x`
-   --> $DIR/check-static-values-constraints.rs:110:45
-    |
- LL |     let y = { static x: Box<isize> = box 3; x };
-    |                                             ^ move occurs because `x` has type `Box<isize>`, which does not implement the `Copy` trait
- help: consider borrowing here
-    |
-    |
- LL |     let y = { static x: Box<isize> = box 3; &x };
- 
- error[E0010]: allocations are not allowed in statics
-   --> $DIR/check-static-values-constraints.rs:110:38
-    |
-    |
- LL |     let y = { static x: Box<isize> = box 3; x };
-    |                                      ^^^^^ allocation not allowed in statics
- error: aborting due to 10 previous errors
- 
- Some errors have detailed explanations: E0010, E0015, E0493, E0507.
- For more information about an error, try `rustc --explain E0010`.
---
To only update this specific test, also pass `--test-args check-static-values-constraints.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/check-static-values-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/check-static-values-constraints.rs:79:33
   |
LL | static STATIC11: Box<MyOwned> = box MyOwned;

error: aborting due to previous error
------------------------------------------

---
- error[E0308]: mismatched types
+ error[E0557]: feature has been removed
+   --> $DIR/coerce-expect-unsized-ascribed.rs:4:12
+    |
+ LL | #![feature(box_syntax, type_ascription)]
+    |
+    |
+    = note: replaced with `#[rustc_box]`
+ error: expected expression, found reserved keyword `box`
2   --> $DIR/coerce-expect-unsized-ascribed.rs:9:27
3    |
3    |
4 LL |     let _ = type_ascribe!(box { [1, 2, 3] }, Box<[i32]>);

-    |                           ^^^^^^^^^^^^^^^^^ expected `Box<[i32]>`, found `Box<[i32; 3]>`
-    |
-    = note: expected struct `Box<[i32]>`
-               found struct `Box<[i32; 3]>`
9 
- error[E0308]: mismatched types
+ error: expected expression, found reserved keyword `box`
11   --> $DIR/coerce-expect-unsized-ascribed.rs:10:27
11   --> $DIR/coerce-expect-unsized-ascribed.rs:10:27
12    |
13 LL |     let _ = type_ascribe!(box if true { [1, 2, 3] } else { [1, 3, 4] }, Box<[i32]>);

-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<[i32]>`, found `Box<[i32; 3]>`
-    |
-    = note: expected struct `Box<[i32]>`
-               found struct `Box<[i32; 3]>`
18 
- error[E0308]: mismatched types
+ error: expected expression, found reserved keyword `box`
20   --> $DIR/coerce-expect-unsized-ascribed.rs:11:27
20   --> $DIR/coerce-expect-unsized-ascribed.rs:11:27
21    |
22 LL |     let _ = type_ascribe!(box match true { true => [1, 2, 3], false => [1, 3, 4] }, Box<[i32]>);

-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<[i32]>`, found `Box<[i32; 3]>`
-    |
-    = note: expected struct `Box<[i32]>`
-               found struct `Box<[i32; 3]>`
27 
- error[E0308]: mismatched types
+ error: expected expression, found reserved keyword `box`
29   --> $DIR/coerce-expect-unsized-ascribed.rs:13:27
29   --> $DIR/coerce-expect-unsized-ascribed.rs:13:27
30    |
31 LL |     let _ = type_ascribe!(box { |x| (x as u8) }, Box<dyn Fn(i32) -> _>);

-    |                           ^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn Fn(i32) -> u8>`, found `Box<[closure@coerce-expect-unsized-ascribed.rs:13:33]>`
-    |
-    = note: expected struct `Box<dyn Fn(i32) -> u8>`
-               found struct `Box<[closure@$DIR/coerce-expect-unsized-ascribed.rs:13:33: 13:36]>`
36 
- error[E0308]: mismatched types
+ error: expected expression, found reserved keyword `box`
38   --> $DIR/coerce-expect-unsized-ascribed.rs:14:27
38   --> $DIR/coerce-expect-unsized-ascribed.rs:14:27
39    |
40 LL |     let _ = type_ascribe!(box if true { false } else { true }, Box<dyn Debug>);

-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn Debug>`, found `Box<bool>`
-    |
-    = note: expected struct `Box<dyn Debug>`
-               found struct `Box<bool>`
45 
- error[E0308]: mismatched types
+ error: expected expression, found reserved keyword `box`
47   --> $DIR/coerce-expect-unsized-ascribed.rs:15:27
47   --> $DIR/coerce-expect-unsized-ascribed.rs:15:27
48    |
49 LL |     let _ = type_ascribe!(box match true { true => 'a', false => 'b' }, Box<dyn Debug>);

-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn Debug>`, found `Box<char>`
+ 
+ error: expected expression, found reserved keyword `box`
+   --> $DIR/coerce-expect-unsized-ascribed.rs:30:9
51    |
51    |
-    = note: expected struct `Box<dyn Debug>`
-               found struct `Box<char>`
+ LL |         box |x| (x as i16 as u8),
+   --> $SRC_DIR/alloc/src/macros.rs:LL:COL
+    |
+    |
+    = note: while parsing argument for this `expr` macro fragment
55 error[E0308]: mismatched types
56   --> $DIR/coerce-expect-unsized-ascribed.rs:17:27


124    = note: expected struct `Box<dyn Fn(i32) -> u8>`
125               found struct `Box<[closure@$DIR/coerce-expect-unsized-ascribed.rs:26:36: 26:39]>`
- error: aborting due to 14 previous errors
+ error: aborting due to 16 previous errors
128 
- For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args coercion/coerce-expect-unsized-ascribed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/coercion/coerce-expect-unsized-ascribed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-expect-unsized-ascribed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-expect-unsized-ascribed/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:4:12
   |
LL | #![feature(box_syntax, type_ascription)]
   |            ^^^^^^^^^^ feature has been removed
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:9:27
   |
   |
LL |     let _ = type_ascribe!(box { [1, 2, 3] }, Box<[i32]>); //~ ERROR mismatched types

error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:10:27
   |
   |
LL |     let _ = type_ascribe!(box if true { [1, 2, 3] } else { [1, 3, 4] }, Box<[i32]>); //~ ERROR mismatched types

error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:11:27
   |
   |
LL |     let _ = type_ascribe!(box match true { true => [1, 2, 3], false => [1, 3, 4] }, Box<[i32]>);

error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:13:27
   |
   |
LL |     let _ = type_ascribe!(box { |x| (x as u8) }, Box<dyn Fn(i32) -> _>); //~ ERROR mismatched types

error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:14:27
   |
   |
LL |     let _ = type_ascribe!(box if true { false } else { true }, Box<dyn Debug>); //~ ERROR mismatched types

error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:15:27
   |
   |
LL |     let _ = type_ascribe!(box match true { true => 'a', false => 'b' }, Box<dyn Debug>); //~ ERROR mismatched types

error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:30:9
   |
   |
LL |         box |x| (x as i16 as u8),
  --> /rustc/FAKE_PREFIX/library/alloc/src/macros.rs:49:8
   |
   |
   = note: while parsing argument for this `expr` macro fragment
error[E0308]: mismatched types
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:17:27
   |
   |
LL |     let _ = type_ascribe!(&{ [1, 2, 3] }, &[i32]); //~ ERROR mismatched types
   |                           ^^^^^^^^^^^^^^ expected `&[i32]`, found `&[i32; 3]`
   = note: expected reference `&[i32]`
              found reference `&[i32; 3]`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:18:27
   |
LL |     let _ = type_ascribe!(&if true { [1, 2, 3] } else { [1, 3, 4] }, &[i32]); //~ ERROR mismatched types
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&[i32]`, found `&[i32; 3]`
   = note: expected reference `&[i32]`
              found reference `&[i32; 3]`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:19:27
   |
LL |     let _ = type_ascribe!(&match true { true => [1, 2, 3], false => [1, 3, 4] }, &[i32]);
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&[i32]`, found `&[i32; 3]`
   = note: expected reference `&[i32]`
              found reference `&[i32; 3]`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:21:27
   |
LL |     let _ = type_ascribe!(&{ |x| (x as u8) }, &dyn Fn(i32) -> _); //~ ERROR mismatched types
   |                           ^^^^^^^^^^^^^^^^^^ expected `&dyn Fn(i32) -> u8`, found `&[closure@coerce-expect-unsized-ascribed.rs:21:30]`
   |
   = note: expected reference `&dyn Fn(i32) -> u8`
              found reference `&[closure@fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:21:30: 21:33]`
error[E0308]: mismatched types
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:22:27
   |
   |
LL |     let _ = type_ascribe!(&if true { false } else { true }, &dyn Debug); //~ ERROR mismatched types
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&dyn Debug`, found `&bool`
   |
   = note: expected reference `&dyn Debug`
              found reference `&bool`
error[E0308]: mismatched types
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:23:27
   |
   |
LL |     let _ = type_ascribe!(&match true { true => 'a', false => 'b' }, &dyn Debug); //~ ERROR mismatched types
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&dyn Debug`, found `&char`
   |
   = note: expected reference `&dyn Debug`
              found reference `&char`
error[E0308]: mismatched types
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:25:27
   |
   |
LL |     let _ = type_ascribe!(Box::new([1, 2, 3]), Box<[i32]>); //~ ERROR mismatched types
   |                           ^^^^^^^^^^^^^^^^^^^ expected `Box<[i32]>`, found `Box<[i32; 3]>`
   |
   = note: expected struct `Box<[i32]>`
              found struct `Box<[i32; 3]>`
error[E0308]: mismatched types
  --> fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:26:27
   |
   |
LL |     let _ = type_ascribe!(Box::new(|x| (x as u8)), Box<dyn Fn(i32) -> _>); //~ ERROR mismatched types
   |                           ^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn Fn(i32) -> u8>`, found `Box<[closure@coerce-expect-unsized-ascribed.rs:26:36]>`
   |
   = note: expected struct `Box<dyn Fn(i32) -> u8>`
              found struct `Box<[closure@fake-test-src-base/coercion/coerce-expect-unsized-ascribed.rs:26:36: 26:39]>`
error: aborting due to 16 previous errors

Some errors have detailed explanations: E0308, E0557.
For more information about an error, try `rustc --explain E0308`.
---
-   --> $DIR/box.rs:9:11
+ error: expected expression, found reserved keyword `box`
+   --> $DIR/box.rs:9:12
3    |
4 LL |     &mut *(box 0)
-    |           ^^^^^^^ calling non-const function `alloc::alloc::exchange_malloc`
6 
- warning: skipping const checks
+ error[E0557]: feature has been removed
+   --> $DIR/box.rs:2:12
+   --> $DIR/box.rs:2:12
8    |
- help: skipping check that does not even have a feature gate
-   --> $DIR/box.rs:9:11
+ LL | #![feature(box_syntax)]
+    |            ^^^^^^^^^^ feature has been removed
11    |
- LL |     &mut *(box 0)
-    |           ^^^^^^^
- help: skipping check for `const_mut_refs` feature
-   --> $DIR/box.rs:9:16
-    |
- LL |     &mut *(box 0)
-    |                ^
- help: skipping check for `const_mut_refs` feature
-   --> $DIR/box.rs:9:5
-    |
- LL |     &mut *(box 0)
- help: skipping check that does not even have a feature gate
-   --> $DIR/box.rs:9:5
-    |
-    |
- LL |     &mut *(box 0)
-    |     ^^^^^^^^^^^^^
+    = note: replaced with `#[rustc_box]`
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to 2 previous errors
31 
- For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/miri_unleashed/box.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/miri_unleashed/box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/box" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/box/auxiliary" "-Zunleash-the-miri-inside-of-you"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/consts/miri_unleashed/box.rs:9:12
   |
LL |     &mut *(box 0)

error[E0557]: feature has been removed
  --> fake-test-src-base/consts/miri_unleashed/box.rs:2:12
   |
   |
LL | #![feature(box_syntax)]
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
---
- error[E0010]: allocations are not allowed in constants
+ error: expected expression, found reserved keyword `box`
2   --> $DIR/E0010-teach.rs:6:24
3    |
4 LL | const CON : Box<i32> = box 0;
-    |                        ^^^^^ allocation not allowed in constants
-    |
-    |
-    = note: The value of statics and constants must be known at compile time, and they live for the entire lifetime of a program. Creating a boxed value allocates memory on the heap at runtime, and therefore cannot be done at compile time.
8 
9 error: aborting due to previous error
10 

---
To only update this specific test, also pass `--test-args error-codes/E0010-teach.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0010-teach.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010-teach" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010-teach/auxiliary" "-Z" "teach"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/error-codes/E0010-teach.rs:6:24
   |
LL | const CON : Box<i32> = box 0; //~ ERROR E0010

error: aborting due to previous error
------------------------------------------

---
- error[E0010]: allocations are not allowed in constants
+ error: expected expression, found reserved keyword `box`
2   --> $DIR/E0010.rs:4:24
3    |
4 LL | const CON : Box<i32> = box 0;
-    |                        ^^^^^ allocation not allowed in constants
+    |                        ^^^ expected expression
6 
7 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args error-codes/E0010.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/error-codes/E0010.rs:4:24
   |
LL | const CON : Box<i32> = box 0; //~ ERROR E0010

error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/feature-gates/feature-gate-box-expr.rs stdout ----
diff of stderr:

- error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
+ error: expected expression, found reserved keyword `box`
3    |
3    |
4 LL |     let x = box 'c';
-    |             ^^^^^^^
-    |
-    = note: see issue #49733 <https://github.com/rust-lang/rust/issues/49733> for more information
-    = help: add `#![feature(box_syntax)]` to the crate attributes to enable
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-box-expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/feature-gate-box-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-box-expr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-box-expr/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/feature-gates/feature-gate-box-expr.rs:12:13
   |
LL |     let x = box 'c'; //~ ERROR box expression syntax is experimental

error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/feature-gates/feature-gate-box_syntax.rs stdout ----
diff of stderr:

- error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
+ error: expected expression, found reserved keyword `box`
3    |
4 LL |     let x = box 3;

-    |             ^^^^^
---
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-box_syntax/feature-gate-box_syntax.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-box_syntax.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/feature-gate-box_syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-box_syntax" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-box_syntax/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/feature-gates/feature-gate-box_syntax.rs:4:13
LL |     let x = box 3;
   |             ^^^ expected expression

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] tests/ui/generator/issue-105084.rs#drop_tracking stdout ----

error in revision `drop_tracking`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/issue-105084.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-105084.drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-105084.drop_tracking/auxiliary" "-Zdrop-tracking"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/generator/issue-105084.rs:28:17
   |
LL |         let t = box (5, yield);

error[E0557]: feature has been removed
  --> fake-test-src-base/generator/issue-105084.rs:12:12
   |
   |
LL | #![feature(box_syntax)]
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
---
-   --> $DIR/issue-105084.rs:44:14
+ error: expected expression, found reserved keyword `box`
+   --> $DIR/issue-105084.rs:28:17
3    |
- LL |     let mut g = || {
-    |         ----- move occurs because `g` has type `[generator@$DIR/issue-105084.rs:22:17: 22:19]`, which does not implement the `Copy` trait
- ...
- LL |     let mut h = copy(g);
- ...
- ...
- LL |     Pin::new(&mut g).resume(());
-    |
-    |
- note: consider changing this parameter type in function `copy` to borrow instead if owning the value isn't necessary
-   --> $DIR/issue-105084.rs:17:21
-    |
- LL | fn copy<T: Copy>(x: T) -> T {
-    |    ----             ^ this parameter takes ownership of the value
-    |    in this function
- help: consider cloning the value if the performance cost is acceptable
-    |
-    |
- LL |     let mut h = copy(g.clone());
-    |                       ++++++++
+ LL |         let t = box (5, yield);
24 
24 
- error[E0277]: the trait bound `Box<(i32, ())>: Copy` is not satisfied in `[generator@$DIR/issue-105084.rs:22:17: 22:19]`
-   --> $DIR/issue-105084.rs:38:17
+ error[E0557]: feature has been removed
27    |
27    |
- LL |     let mut g = || {
-    |                 -- within this `[generator@$DIR/issue-105084.rs:22:17: 22:19]`
- ...
- LL |     let mut h = copy(g);
-    |                 ^^^^ within `[generator@$DIR/issue-105084.rs:22:17: 22:19]`, the trait `Copy` is not implemented for `Box<(i32, ())>`
+ LL | #![feature(box_syntax)]
33    |
33    |
- note: generator does not implement `Copy` as this value is used across a yield
-   --> $DIR/issue-105084.rs:28:25
-    |
- LL |         let t = box (5, yield);
-    |                 |       |
-    |                 |       |
-    |                 |       yield occurs here, with `box (5, yield)` maybe used later
-    |                 has type `Box<(i32, ())>` which does not implement `Copy`
-   --> $DIR/issue-105084.rs:17:12
-    |
-    |
- LL | fn copy<T: Copy>(x: T) -> T {
-    |            ^^^^ required by this bound in `copy`
+    = note: replaced with `#[rustc_box]`
48 error: aborting due to 2 previous errors
49 

- Some errors have detailed explanations: E0277, E0382.
- Some errors have detailed explanations: E0277, E0382.
- For more information about an error, try `rustc --explain E0277`.
+ For more information about this error, try `rustc --explain E0557`.
52 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-105084.drop_tracking_mir/issue-105084.drop_tracking_mir.stderr
To only update this specific test, also pass `--test-args generator/issue-105084.rs`


error in revision `drop_tracking_mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/issue-105084.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking_mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-105084.drop_tracking_mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-105084.drop_tracking_mir/auxiliary" "-Zdrop-tracking-mir"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/generator/issue-105084.rs:28:17
   |
LL |         let t = box (5, yield);

error[E0557]: feature has been removed
  --> fake-test-src-base/generator/issue-105084.rs:12:12
   |
   |
LL | #![feature(box_syntax)]
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/issue-105084.rs#no_drop_tracking stdout ----

error in revision `no_drop_tracking`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/issue-105084.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-105084.no_drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-105084.no_drop_tracking/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/generator/issue-105084.rs:28:17
   |
LL |         let t = box (5, yield);

error[E0557]: feature has been removed
  --> fake-test-src-base/generator/issue-105084.rs:12:12
   |
   |
LL | #![feature(box_syntax)]
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/yield-in-box.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/yield-in-box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-box/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-box/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/generator/yield-in-box.rs:14:22
   |
LL |             let _t = box (&x, yield 0, &y);

error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/generator/yield-in-box.rs:16:15
   |
   |
LL |         match box (&x, yield 0, &y) {
   |         |
   |         while parsing this `match` expression

error: expected expression, found reserved keyword `box`
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/generator/yield-in-box.rs:21:21
   |
LL |     let mut g = |_| box yield;

error[E0557]: feature has been removed
  --> fake-test-src-base/generator/yield-in-box.rs:4:24
   |
   |
LL | #![feature(generators, box_syntax, generator_trait)]
   |
   |
   = note: replaced with `#[rustc_box]`
warning: unused import: `std::pin::Pin`
  --> fake-test-src-base/generator/yield-in-box.rs:5:5
   |
LL | use std::pin::Pin;
---

---- [ui] tests/ui/lang-items/no_owned_box_lang_item.rs stdout ----
diff of stderr:

- error: requires `owned_box` lang_item
+ error: expected expression, found reserved keyword `box`
+    |
+ LL |     let x = box 1i32;
+    |             ^^^ expected expression
2 
2 
- error: aborting due to previous error
+ error[E0557]: feature has been removed
+   --> $DIR/no_owned_box_lang_item.rs:5:24
+    |
+ LL | #![feature(lang_items, box_syntax)]
+    |                        ^^^^^^^^^^ feature has been removed
+    |
+    = note: replaced with `#[rustc_box]`
+ error: aborting due to 2 previous errors
+ 
+ For more information about this error, try `rustc --explain E0557`.
5 
---
To only update this specific test, also pass `--test-args lang-items/no_owned_box_lang_item.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lang-items/no_owned_box_lang_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/no_owned_box_lang_item" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/no_owned_box_lang_item/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/lang-items/no_owned_box_lang_item.rs:11:13
LL |     let x = box 1i32;
   |             ^^^ expected expression

error[E0557]: feature has been removed
error[E0557]: feature has been removed
  --> fake-test-src-base/lang-items/no_owned_box_lang_item.rs:5:24
   |
LL | #![feature(lang_items, box_syntax)]
   |                        ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/macros/rfc-2011-nicer-assert-messages/all-expr-kinds.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/rfc-2011-nicer-assert-messages/all-expr-kinds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/all-expr-kinds/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/all-expr-kinds/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/macros/rfc-2011-nicer-assert-messages/all-expr-kinds.rs:8:12
   |
   |
LL | #![feature(box_syntax, core_intrinsics, generic_assert, generic_assert_internals)]
   |
   |
   = note: replaced with `#[rustc_box]`
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/macros/rfc-2011-nicer-assert-messages/all-expr-kinds.rs:131:7
   |
   |
LL |     [ box elem == box 3 ] => "Assertion failed: box elem == box 3"

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
For more information about this error, try `rustc --explain E0557`.
------------------------------------------


---- [ui] tests/ui/macros/stringify.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/stringify.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/stringify/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/stringify/auxiliary" "--edition=2021" "--test"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/macros/stringify.rs:7:12
   |
LL | #![feature(box_syntax)]
   |            ^^^^^^^^^^ feature has been removed
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/macros/stringify.rs:95:32
   |
   |
LL |     ($expr:expr) => {
   |      ---------- while parsing argument for this `expr` macro fragment
...
LL |     assert_eq!(stringify_expr!(box expr), "box expr");

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
For more information about this error, try `rustc --explain E0557`.
------------------------------------------


---- [ui] tests/ui/mir/mir_boxing.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mir/mir_boxing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_boxing/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_boxing/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/mir/mir_boxing.rs:5:5
LL |     box 42
   |     ^^^ expected expression

error[E0557]: feature has been removed
error[E0557]: feature has been removed
  --> fake-test-src-base/mir/mir_boxing.rs:2:12
   |
LL | #![feature(box_syntax)]
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
---
-   --> $DIR/attr-stmt-expr-attr-bad.rs:3:36
+ error: expected expression, found reserved keyword `box`
+   --> $DIR/attr-stmt-expr-attr-bad.rs:3:32
3    |
4 LL | #[cfg(FALSE)] fn e() { let _ = box #![attr] 0; }
-    |
-    |
-    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
-    = note: outer attributes, like `#[test]`, annotate the item following them
9 
10 error: expected expression, found `]`
11   --> $DIR/attr-stmt-expr-attr-bad.rs:5:40

---
To only update this specific test, also pass `--test-args parser/attr-stmt-expr-attr-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/attr-stmt-expr-attr-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:3:32
   |
LL | #[cfg(FALSE)] fn e() { let _ = box #![attr] 0; }

error: expected expression, found `]`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:5:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = [#[attr]]; }


error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:7:35
   |
LL | #[cfg(FALSE)] fn e() { let _ = foo#[attr](); }
   |                                   ^ expected one of 8 possible tokens
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:9:36
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: expected expression, found `)`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:9:44
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }

error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:12:38
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: expected expression, found `)`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:12:46
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }

error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:15:36
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = 0 + #![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:17:33
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = !#![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:19:33
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = -#![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them

error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:21:34
   |
LL | #[cfg(FALSE)] fn e() { let _ = x #![attr] as Y; }
   |                                  ^ expected one of 8 possible tokens
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:23:35
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = || #![attr] foo; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:25:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = move || #![attr] foo; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:27:35
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = || #![attr] {foo}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:29:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = move || #![attr] {foo}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: expected expression, found `..`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:31:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = #[attr] ..#[attr] 0; }

error: expected expression, found `..`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:33:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = #[attr] ..; }

error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:35:41
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = #[attr] &#![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:37:45
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = #[attr] &mut #![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them

error: outer attributes are not allowed on `if` and `else` branches
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:39:37
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 #[attr] {}; }
   |                                |    |
   |                                |    help: remove the attributes
   |                                the branch belongs to this `if`


error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:41:38
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them

error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:43:40
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} #[attr] else {}; }
   |                                        ^ expected one of `.`, `;`, `?`, `else`, or an operator

error: outer attributes are not allowed on `if` and `else` branches
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:45:45
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] {}; }
   |                                        |    |
   |                                        |    help: remove the attributes
   |                                        |    help: remove the attributes
   |                                        the branch belongs to this `else`
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:47:46
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them

error: outer attributes are not allowed on `if` and `else` branches
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:49:45
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] if 0 {}; }
   |                                        |    |
   |                                        |    help: remove the attributes
   |                                        |    help: remove the attributes
   |                                        the branch belongs to this `else`

error: outer attributes are not allowed on `if` and `else` branches
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:51:50
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 #[attr] {}; }
   |                                             |    |
   |                                             |    help: remove the attributes
   |                                             the branch belongs to this `if`


error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:53:51
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them

error: outer attributes are not allowed on `if` and `else` branches
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:55:45
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 #[attr] {}; }
   |                                |            |
   |                                |            help: remove the attributes
   |                                the branch belongs to this `if`


error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:57:46
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them

error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:59:48
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} #[attr] else {}; }
   |                                                ^ expected one of `.`, `;`, `?`, `else`, or an operator

error: outer attributes are not allowed on `if` and `else` branches
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:61:53
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] {}; }
   |                                                |    |
   |                                                |    help: remove the attributes
   |                                                |    help: remove the attributes
   |                                                the branch belongs to this `else`
error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:63:54
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them

error: outer attributes are not allowed on `if` and `else` branches
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:65:53
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] if let _ = 0 {}; }
   |                                                |    |
   |                                                |    help: remove the attributes
   |                                                |    help: remove the attributes
   |                                                the branch belongs to this `else`

error: outer attributes are not allowed on `if` and `else` branches
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:67:66
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 #[attr] {}; }
   |                                                     |            |
   |                                                     |            help: remove the attributes
   |                                                     the branch belongs to this `if`


error: an inner attribute is not permitted in this context
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:69:67
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted following an outer attribute
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:72:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] let _ = 0; }
   |                        ------- ^^^^^^^^ not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted following an outer attribute
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:74:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] 0; }
   |                        ------- ^^^^^^^^ not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
   = note: outer attributes, like `#[test]`, annotate the item following them
error: an inner attribute is not permitted following an outer attribute
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:76:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!(); }
   |                        ------- ^^^^^^^^ ------- the inner attribute doesn't annotate this item macro invocation
   |                        |       not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
help: to annotate the item macro invocation, change the attribute from inner to outer style
   |
LL - #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!(); }
LL + #[cfg(FALSE)] fn s() { #[attr] #[attr] foo!(); }

error: an inner attribute is not permitted following an outer attribute
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:78:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo![]; }
   |                        ------- ^^^^^^^^ ------- the inner attribute doesn't annotate this item macro invocation
   |                        |       not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
help: to annotate the item macro invocation, change the attribute from inner to outer style
   |
LL - #[cfg(FALSE)] fn s() { #[attr] #![attr] foo![]; }
LL + #[cfg(FALSE)] fn s() { #[attr] #[attr] foo![]; }

error: an inner attribute is not permitted following an outer attribute
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:80:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!{}; }
   |                        ------- ^^^^^^^^ ------ the inner attribute doesn't annotate this item macro invocation
   |                        |       not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
help: to annotate the item macro invocation, change the attribute from inner to outer style
   |
LL - #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!{}; }
LL + #[cfg(FALSE)] fn s() { #[attr] #[attr] foo!{}; }

error[E0586]: inclusive range with no end
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:86:35
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
   |
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)

error: expected one of `=>`, `if`, or `|`, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:86:38
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
   |                                      ^ expected one of `=>`, `if`, or `|`
error[E0586]: inclusive range with no end
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:89:35
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
   |
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)

error: expected one of `=>`, `if`, or `|`, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:89:38
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
   |                                      ^ expected one of `=>`, `if`, or `|`
error: unexpected token: `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:92:39
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=-#[attr] 10 => () } }

error[E0586]: inclusive range with no end
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:94:35
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
   |
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)

error: expected one of `=>`, `if`, or `|`, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:94:38
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
   |                                      ^ expected one of `=>`, `if`, or `|`
error: unexpected token: `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:98:34
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }


error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:98:34
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }
   |                                  ^ expected one of `.`, `;`, `?`, `else`, or an operator
error: unexpected token: `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:101:34
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }


error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:101:34
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }
   |                                  ^ expected one of `.`, `;`, `?`, `else`, or an operator
error: expected statement after outer attribute
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:106:37
   |
   |
LL | #[cfg(FALSE)] fn e() { { fn foo() { #[attr]; } } }

error: expected statement after outer attribute
  --> fake-test-src-base/parser/attr-stmt-expr-attr-bad.rs:108:37
   |
   |
LL | #[cfg(FALSE)] fn e() { { fn foo() { #[attr] } } }

error: aborting due to 53 previous errors

For more information about this error, try `rustc --explain E0586`.
For more information about this error, try `rustc --explain E0586`.
------------------------------------------


---- [ui] tests/ui/parser/issues/issue-65846-rollback-gating-failing-matcher.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/issues/issue-65846-rollback-gating-failing-matcher.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-65846-rollback-gating-failing-matcher/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-65846-rollback-gating-failing-matcher/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/parser/issues/issue-65846-rollback-gating-failing-matcher.rs:14:22
   |
LL |     ($e:expr) => { 0 }; // This fails on the input below due to `, foo`.
   |      ------- while parsing argument for this `expr` macro fragment
...
LL |     assert_eq!(2, m!(box 42, foo));

error: aborting due to previous error
------------------------------------------

---
-   --> $DIR/removed-syntax-uniq-mut-expr.rs:2:21
+ error: expected expression, found reserved keyword `box`
+   --> $DIR/removed-syntax-uniq-mut-expr.rs:2:17
3    |
4 LL |     let a_box = box mut 42;
+    |                 ^^^ expected expression
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args parser/removed-syntax-uniq-mut-expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/removed-syntax-uniq-mut-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-uniq-mut-expr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-uniq-mut-expr/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/parser/removed-syntax-uniq-mut-expr.rs:2:17
   |
LL |     let a_box = box mut 42; //~ ERROR expected expression, found keyword `mut`

error: aborting due to previous error
------------------------------------------

---
+    |            ^^^^^^^^^^ feature has been removed
12    |
- LL | #![deny(unreachable_code)]
-    |         ^^^^^^^^^^^^^^^^
+    = note: replaced with `#[rustc_box]`
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
17 
+ For more information about this error, try `rustc --explain E0557`.
---
To only update this specific test, also pass `--test-args reachable/expr_box.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/reachable/expr_box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_box" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_box/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/reachable/expr_box.rs:6:13
   |
LL |     let x = box return; //~ ERROR unreachable

error[E0557]: feature has been removed
  --> fake-test-src-base/reachable/expr_box.rs:1:12
   |
   |
LL | #![feature(box_syntax)]
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
---
- error[E0010]: allocations are not allowed in statics
+ error: expected expression, found reserved keyword `box`
2   --> $DIR/static-mut-not-constant.rs:3:28
3    |
4 LL | static mut a: Box<isize> = box 3;
-    |                            ^^^^^ allocation not allowed in statics
+    |                            ^^^ expected expression
6 
7 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args static/static-mut-not-constant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/static/static-mut-not-constant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-not-constant" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-not-constant/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/static/static-mut-not-constant.rs:3:28
   |
LL | static mut a: Box<isize> = box 3;

error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/typeck/issue-105946.rs stdout ----
diff of stderr:

- error[E0425]: cannot find value `_y` in this scope
-   --> $DIR/issue-105946.rs:6:10
-    |
- LL |     let [_y..] = [box 1, box 2];
- 
- 
- error[E0658]: `X..` patterns in slices are experimental
-   --> $DIR/issue-105946.rs:6:10
-    |
- LL |     let [_y..] = [box 1, box 2];
-    |
-    = note: see issue #67264 <https://github.com/rust-lang/rust/issues/67264> for more information
-    = note: see issue #67264 <https://github.com/rust-lang/rust/issues/67264> for more information
-    = help: add `#![feature(half_open_range_patterns_in_slices)]` to the crate attributes to enable
- 
- error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
+ error: expected expression, found reserved keyword `box`
18    |
18    |
19 LL |     let [_y..] = [box 1, box 2];
-    |                   ^^^^^
-    |
-    = note: see issue #49733 <https://github.com/rust-lang/rust/issues/49733> for more information
-    = help: add `#![feature(box_syntax)]` to the crate attributes to enable
-    = help: add `#![feature(box_syntax)]` to the crate attributes to enable
+    |                   ^^^ expected expression
24 
- error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
-   --> $DIR/issue-105946.rs:6:26
-    |
- LL |     let [_y..] = [box 1, box 2];
-    |
-    = note: see issue #49733 <https://github.com/rust-lang/rust/issues/49733> for more information
-    = help: add `#![feature(box_syntax)]` to the crate attributes to enable
- 
---
39 
- error[E0527]: pattern requires 1 element but array has 2
-   --> $DIR/issue-105946.rs:6:9
-    |
- LL |     let [_y..] = [box 1, box 2];
-    |         ^^^^^^ expected 2 elements
+ error: aborting due to 2 previous errors
- error: aborting due to 6 previous errors
- 
- Some errors have detailed explanations: E0308, E0425, E0527, E0658.
- For more information about an error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args typeck/issue-105946.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-105946.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-105946" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-105946/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/typeck/issue-105946.rs:6:19
   |
LL |     let [_y..] = [box 1, box 2];

error[E0308]: mismatched types
  --> fake-test-src-base/typeck/issue-105946.rs:2:10
   |
---
-   --> $DIR/issue-87935-unsized-box-expr.rs:4:30
+ error: expected expression, found reserved keyword `box`
+   --> $DIR/issue-87935-unsized-box-expr.rs:4:26
3    |
4 LL |     let _x: Box<[u32]> = box { loop {} };
+    |                          ^^^ expected expression
+ 
+ error[E0557]: feature has been removed
+   --> $DIR/issue-87935-unsized-box-expr.rs:1:12
+   --> $DIR/issue-87935-unsized-box-expr.rs:1:12
6    |
-    = help: the trait `Sized` is not implemented for `[u32]`
-    = note: the type of a box expression must have a statically known size
+ LL | #![feature(box_syntax)]
+    |            ^^^^^^^^^^ feature has been removed
+    |
+    = note: replaced with `#[rustc_box]`
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
11 
- For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args typeck/issue-87935-unsized-box-expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-87935-unsized-box-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87935-unsized-box-expr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87935-unsized-box-expr/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found reserved keyword `box`
  --> fake-test-src-base/typeck/issue-87935-unsized-box-expr.rs:4:26
   |
LL |     let _x: Box<[u32]> = box { loop {} };

error[E0557]: feature has been removed
  --> fake-test-src-base/typeck/issue-87935-unsized-box-expr.rs:1:12
   |
   |
LL | #![feature(box_syntax)]
   |            ^^^^^^^^^^ feature has been removed
   |
   = note: replaced with `#[rustc_box]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
