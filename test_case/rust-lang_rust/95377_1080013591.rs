plain
............................................................................i....................... 1900/12757
.................................................................................................... 2000/12757
.................................................................................................... 2100/12757
.................................................................................................... 2200/12757
.............................................................F...................................... 2300/12757
.......................................................................F.i.......................... 2400/12757
...............................................................F.F.....ii..FFFFFF.F.F........FF..... 2500/12757
.............................................................i...................................... 2700/12757
.......................i.i....F.....................i..............i...............................i 2800/12757
...............................................................................F.................... 2900/12757
...................i................................................................................ 3000/12757
---

---- [ui] ui/const-generics/min_const_generics/invalid-patterns.rs stdout ----
diff of 32bit.stderr:

22 LL |   get_flag::<42, 0x5ad>();
23    |                  ^^^^^ expected `char`, found `u8`
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/invalid-patterns.rs:38:21
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/invalid-patterns.rs:38:32
+   --> $DIR/invalid-patterns.rs:38:32
27    |
28 LL |   get_flag::<false, { unsafe { char_raw.character } }>();
-    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                __ __ __ __                                     │ ░░░░
-            }
+    |                                ^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
35 
thread 'main' panicked at 'I/O failure during tests: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/tools/compiletest/src/main.rs:432:13
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/invalid-patterns.rs:40:14
+ error[E0080]: evaluation of constant value failed
38    |
38    |
39 LL |   get_flag::<{ unsafe { bool_raw.boolean } }, 'z'>();
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 1, align: 1) {
-            }
-            }
+    |                         ^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/invalid-patterns.rs:42:14
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/invalid-patterns.rs:42:25
+   --> $DIR/invalid-patterns.rs:42:25
49    |
50 LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 1, align: 1) {
-            }
-            }
+    |                         ^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/invalid-patterns.rs:42:47
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/invalid-patterns.rs:42:58
+   --> $DIR/invalid-patterns.rs:42:58
60    |
61 LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
-    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                __ __ __ __                                     │ ░░░░
-            }
+    |                                                          ^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
69 error: aborting due to 8 previous errors
70 



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns/invalid-patterns.32bit.stderr
To only update this specific test, also pass `--test-args const-generics/min_const_generics/invalid-patterns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:29:21
   |
   |
LL |   get_flag::<false, 0xFF>();
   |                     ^^^^ expected `char`, found `u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:31:14
   |
   |
LL |   get_flag::<7, 'c'>();
   |              ^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:33:14
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |              ^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:33:18
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |                  ^^^^^ expected `char`, found `u8`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:38:32
   |
   |
LL |   get_flag::<false, { unsafe { char_raw.character } }>();
   |                                ^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:40:25
   |
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, 'z'>();
   |                         ^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:42:25
   |
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
   |                         ^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:42:58
   |
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
   |                                                          ^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0080, E0308.
For more information about an error, try `rustc --explain E0080`.
---
-   --> $DIR/const-err4.rs:9:11
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/const-err4.rs:9:21
3    |
4 LL |     Boo = [unsafe { Foo { b: () }.a }; 4][3],
-    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                __ __ __ __                                     │ ░░░░
+    |                     ^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
11 
12 error: aborting due to previous error
13 
13 


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/const-err4.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-err4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err4.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL |     Boo = [unsafe { Foo { b: () }.a }; 4][3],
   |                     ^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
-   --> $DIR/ref_to_int_match.rs:25:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ref_to_int_match.rs:25:27
3    |
4 LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc3, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                           ^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc3, but expected initialized plain (non-pointer) bytes
11 
12 error: could not evaluate constant pattern
12 error: could not evaluate constant pattern
13   --> $DIR/ref_to_int_match.rs:7:14


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ref_to_int_match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f }; //~ ERROR evaluation of constant value failed
   |                           ^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc3, but expected initialized plain (non-pointer) bytes
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
   |
   |
LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
   |
   |
LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] ui/consts/const-eval/transmute-const.rs stdout ----
diff of 32bit.stderr:

- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/transmute-const.rs:4:1
+ error[E0080]: could not evaluate static initializer
3    |
3    |
4 LL | static FOO: bool = unsafe { mem::transmute(3u8) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 1, align: 1) {
-            }
-            }
+    |                             ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
12 error: aborting due to previous error
13 



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const/transmute-const.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/transmute-const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/transmute-const.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: could not evaluate static initializer
   |
   |
LL | static FOO: bool = unsafe { mem::transmute(3u8) };
   |                             ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
-   --> $DIR/ub-uninhabit.rs:15:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-uninhabit.rs:15:35
3    |
4 LL | const BAD_BAD_BAD: Bar = unsafe { MaybeUninit { uninit: () }.init };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 0, align: 1) {}
+    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar
10 error[E0080]: it is undefined behavior to use this value
11   --> $DIR/ub-uninhabit.rs:18:1

18                01 00 00 00                                     │ ....
18                01 00 00 00                                     │ ....
19            }
20 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-uninhabit.rs:21:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-uninhabit.rs:21:42
23    |
24 LL | const BAD_BAD_ARRAY: [Bar; 1] = unsafe { MaybeUninit { uninit: () }.init };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at [0]: encountered a value of uninhabited type Bar
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 0, align: 1) {}
+    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at [0]: encountered a value of uninhabited type Bar
30 error: aborting due to 3 previous errors
31 



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit/ub-uninhabit.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-uninhabit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const BAD_BAD_BAD: Bar = unsafe { MaybeUninit { uninit: () }.init };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:18:1
   |
   |
LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>: encountered a value of uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:21:42
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:21:42
   |
LL | const BAD_BAD_ARRAY: [Bar; 1] = unsafe { MaybeUninit { uninit: () }.init };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at [0]: encountered a value of uninhabited type Bar
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
-   --> $DIR/ub-nonnull.rs:12:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-nonnull.rs:12:40
3    |
4 LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                                        ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
11 
12 error[E0080]: evaluation of constant value failed
12 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-nonnull.rs:19:30
+   --> $DIR/ub-nonnull.rs:17:27
14    |
- LL |     let out_of_bounds_ptr = &ptr[255];
-    |                              ^^^^^^^^ dereferencing pointer failed: alloc11 has size 1, so pointer to 256 bytes starting at offset 0 is out-of-bounds
+ LL |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
+    |                           ^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:23:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-nonnull.rs:24:37
+   --> $DIR/ub-nonnull.rs:24:37
20    |
21 LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 1, align: 1) {
-            }
+    |                                     ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
28 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:25:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-nonnull.rs:26:43
31    |
32 LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                                           ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
39 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:33:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-nonnull.rs:34:36
42    |
43 LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 1, align: 1) {
-                __                                              │ ░
+    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
50 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:41:1
-   --> $DIR/ub-nonnull.rs:41:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-nonnull.rs:42:1
53    |
54 LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
55    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
-    |
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                2a 00 00 00                                     │ *...
61 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:47:1
+ error[E0080]: evaluation of constant value failed
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-nonnull.rs:48:1
64    |
65 LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
66    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
-    |
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
72 
73 error: aborting due to 7 previous errors
74 
74 


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   |                                        ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:17:27
   |
   |
LL |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
   |                           ^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:24:37
   |
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   |                                     ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:26:43
   |
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   |                                           ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:34:36
   |
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:42:1
   |
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:48:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
-   --> $DIR/ub-ref-ptr.rs:13:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:13:34
3    |
4 LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
-            }
+    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:17:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:17:42
+   --> $DIR/ub-ref-ptr.rs:17:42
14    |
15 LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
-            }
+    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:21:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:21:29
+   --> $DIR/ub-ref-ptr.rs:21:29
25    |
26 LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null reference
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                             ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null reference
33 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:24:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:24:37
36    |
37 LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null box
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                                     ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null box
44 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:31:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:31:38
47    |
48 LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc15, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                                      ^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc15, but expected initialized plain (non-pointer) bytes
55 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:34:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:34:49
58    |
59 LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                                                 ^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc19, but expected initialized plain (non-pointer) bytes
66 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:37:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:37:87
69    |
70 LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                                                                                       ^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc23, but expected initialized plain (non-pointer) bytes
77 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:40:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:40:44
80    |
81 LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (address 0x539 is unallocated)
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
-            }
+    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (address 0x539 is unallocated)
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:43:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:43:40
+   --> $DIR/ub-ref-ptr.rs:43:40
91    |
92 LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (address 0x539 is unallocated)
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
-            }
+    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (address 0x539 is unallocated)
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:46:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:46:41
+   --> $DIR/ub-ref-ptr.rs:46:41
102    |
103 LL | const UNINIT_PTR: *const i32 = unsafe { MaybeUninit { uninit: () }.init };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                __ __ __ __                                     │ ░░░░
-            }
+    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:49:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:49:36
+   --> $DIR/ub-ref-ptr.rs:49:36
113    |
114 LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null function pointer
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-            }
+    |                                    ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null function pointer
121 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:51:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-ref-ptr.rs:51:38
124    |
125 LL | const UNINIT_FN_PTR: fn() = unsafe { MaybeUninit { uninit: () }.init };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a proper pointer or integer value
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                __ __ __ __                                     │ ░░░░
+    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a proper pointer or integer value
132 
133 error[E0080]: it is undefined behavior to use this value
134   --> $DIR/ub-ref-ptr.rs:53:1
134   --> $DIR/ub-ref-ptr.rs:53:1

145   --> $DIR/ub-ref-ptr.rs:55:1
146    |
147 LL | const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc41, but expected a function pointer
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc39, but expected a function pointer
149    |
150    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
151    = note: the raw bytes of the constant (size: 4, align: 4) {
-                ╾─alloc41─╼                                     │ ╾──╼
+                ╾─alloc39─╼                                     │ ╾──╼
153            }
154 
154 
155 error: aborting due to 14 previous errors


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/ub-ref-ptr.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-ref-ptr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:42
   |
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:29
   |
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null reference
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:37
   |
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   |                                     ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null box
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:31:38
   |
   |
LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
   |                                      ^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc15, but expected initialized plain (non-pointer) bytes
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:34:49
   |
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   |                                                 ^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc19, but expected initialized plain (non-pointer) bytes
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:37:87
   |
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   |                                                                                       ^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc23, but expected initialized plain (non-pointer) bytes
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:40:44
   |
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (address 0x539 is unallocated)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:43:40
   |
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (address 0x539 is unallocated)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:46:41
   |
   |
LL | const UNINIT_PTR: *const i32 = unsafe { MaybeUninit { uninit: () }.init };
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:49:36
   |
   |
LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
   |                                    ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null function pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:51:38
   |
   |
LL | const UNINIT_FN_PTR: fn() = unsafe { MaybeUninit { uninit: () }.init };
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a proper pointer or integer value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:53:1
   |
   |
LL | const DANGLING_FN_PTR: fn() = unsafe { mem::transmute(13usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x0000000d, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               0d 00 00 00                                     │ ....

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:55:1
   |
   |
LL | const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc39, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to 14 previous errors

---
-   --> $DIR/ub-upvars.rs:6:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-upvars.rs:7:42
3    |
- LL | / const BAD_UPVAR: &dyn FnOnce() = &{
- LL | |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
- LL | |     let another_var = 13;
- LL | |     move || { let _ = bad_ref; let _ = another_var; }
- LL | | };
-    | |__^ type validation failed at .<deref>.<dyn-downcast>.<captured-var(bad_ref)>: encountered a null reference
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 4) {
-                ╾─alloc3──╼ ╾─alloc6──╼                         │ ╾──╼╾──╼
-            }
+ LL |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
+    |                                          ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null reference
16 error: aborting due to previous error
17 



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/ub-upvars.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-upvars.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-upvars.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null reference
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
-   --> $DIR/ub-int-array.rs:14:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-int-array.rs:16:9
3    |
- LL | / const UNINIT_INT_0: [u32; 3] = unsafe {
- LL | |
- LL | |
- LL | |     [
- LL | |     ]
- LL | | };
- LL | | };
-    | |__^ type validation failed at [0]: encountered uninitialized bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 12, align: 4) {
-                __ __ __ __ 01 00 00 00 02 00 00 00             │ ░░░░........
-            }
+ LL |         MaybeUninit { uninit: () }.init,
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-int-array.rs:23:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-int-array.rs:29:13
+   --> $DIR/ub-int-array.rs:29:13
20    |
- LL | / const UNINIT_INT_1: [u32; 3] = unsafe {
- LL | |
- LL | |
- LL | |     mem::transmute(
- LL | |     )
- LL | | };
- LL | | };
-    | |__^ type validation failed at [1]: encountered uninitialized bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 12, align: 4) {
-                00 00 00 00 01 __ 01 01 02 02 __ 02             │ .....░....░.
-            }
+ LL |             MaybeUninit { uninit: () }.init,
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-int-array.rs:43:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-int-array.rs:53:13
+   --> $DIR/ub-int-array.rs:53:13
37    |
- LL | / const UNINIT_INT_2: [u32; 3] = unsafe {
- LL | |
- LL | |
- LL | |     mem::transmute(
- LL | |     )
- LL | | };
- LL | | };
-    | |__^ type validation failed at [2]: encountered uninitialized bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 12, align: 4) {
-                00 00 00 00 01 01 01 01 02 02 02 __             │ ...........░
-            }
+ LL |             MaybeUninit { uninit: () }.init,
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
52 error: aborting due to 3 previous errors
53 



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-int-array/ub-int-array.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-int-array.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-int-array.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-int-array" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-int-array/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL |         MaybeUninit { uninit: () }.init, //~ ERROR evaluation of constant value failed
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-int-array.rs:29:13
   |
   |
LL |             MaybeUninit { uninit: () }.init, //~ ERROR evaluation of constant value failed
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-int-array.rs:53:13
   |
   |
LL |             MaybeUninit { uninit: () }.init, //~ ERROR evaluation of constant value failed
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] ui/consts/const-eval/ub-incorrect-vtable.rs stdout ----
diff of 32bit.stderr:

2   --> $DIR/ub-incorrect-vtable.rs:19:14
3    |
4 LL |     unsafe { std::mem::transmute((&92u8, &[0usize, 1usize, 1000usize])) };
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid vtable: alignment `1000` is not a power of 2
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function)
7 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-incorrect-vtable.rs:24:14
+   --> $DIR/ub-incorrect-vtable.rs:23:14
9    |
9    |
10 LL |     unsafe { std::mem::transmute((&92u8, &[1usize, usize::MAX, 1usize])) };
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid vtable: size is bigger than largest supported object
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function)
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-incorrect-vtable.rs:34:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-incorrect-vtable.rs:33:14
+   --> $DIR/ub-incorrect-vtable.rs:33:14
15    |
- LL | / const INVALID_VTABLE_ALIGNMENT_UB: W<&dyn Trait> =
- LL | |     unsafe { std::mem::transmute((&92u8, &(drop_me as fn(*mut usize), 1usize, 1000usize))) };
-    | |_____________________________________________________________________________________________^ type validation failed at .0: encountered invalid vtable: alignment `1000` is not a power of 2
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 4) {
-                ╾─allocN─╼ ╾─allocN─╼                         │ ╾──╼╾──╼
-            }
+ LL |     unsafe { std::mem::transmute((&92u8, &(drop_me as fn(*mut usize), 1usize, 1000usize))) };
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered invalid vtable: alignment `1000` is not a power of 2
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-incorrect-vtable.rs:39:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-incorrect-vtable.rs:37:14
+   --> $DIR/ub-incorrect-vtable.rs:37:14
27    |
- LL | / const INVALID_VTABLE_SIZE_UB: W<&dyn Trait> =
- LL | |     unsafe { std::mem::transmute((&92u8, &(drop_me as fn(*mut usize), usize::MAX, 1usize))) };
-    | |______________________________________________________________________________________________^ type validation failed at .0: encountered invalid vtable: size is bigger than largest supported object
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 4) {
-                ╾─allocN─╼ ╾─allocN─╼                         │ ╾──╼╾──╼
-            }
+ LL |     unsafe { std::mem::transmute((&92u8, &(drop_me as fn(*mut usize), usize::MAX, 1usize))) };
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered invalid vtable: size is bigger than largest supported object
37 error: aborting due to 4 previous errors
38 


