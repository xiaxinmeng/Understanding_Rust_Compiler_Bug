plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.068 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i..i...ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.26s

 finished in 2.342 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 20 tests
iiiiiiiiiiii........

 finished in 0.579 seconds
Build completed successfully in 0:31:48
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 11330 tests
.....................i.............................................................................. 100/11330
..................................iiiiiiii.iiiiiiiii................................................ 200/11330
.................................................................................................... 400/11330
.................................................................................................... 500/11330
.................................................................................................... 600/11330
.......................................................ii........................................... 700/11330
---
.................................................................................................... 1700/11330
.............................................................................i...................... 1800/11330
.................................................................................................... 1900/11330
.................................................................................................... 2000/11330
..............................................................................F...i................. 2100/11330
......................F.....................................F.........FFFF.F.FF....F................ 2200/11330
.....................................................F..............................ii.i............ 2300/11330
....................................................i.........................................i.i... 2400/11330
........F...........i.................................i.....F....F...................F.............. 2500/11330
....................F............F.........................i..i..................................... 2600/11330
..............................................................iiiii................................. 2800/11330
.................................................................................................... 2900/11330
.................................................................................................... 3000/11330
.................................................................................................... 3100/11330
---

---- [ui] ui/consts/const-err4.rs stdout ----
diff of stderr:

5    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                __ __ __ __ __ __ __ __                         │ ░░░░░░░░
+    = note: the raw bytes of the constant (size: 4, align: 4) {
+                __ __ __ __                                     │ ░░░░
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/const-err4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-err4.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err4.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-err4.rs:8:11
   |
LL |     Boo = [unsafe { Foo { b: () }.a }; 4][3],
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.

------------------------------------------


thread 'main' panicked at 'I/O failure during tests: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/tools/compiletest/src/main.rs:399:13
---- [ui] ui/consts/const-eval/heap/alloc_intrinsic_uninit.rs stdout ----


5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<deref>, but expected initialized plain (non-pointer) bytes
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc1────────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
10            }
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit/alloc_intrinsic_uninit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/heap/alloc_intrinsic_uninit.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit.rs:8:1
   |
LL | const BAR: &i32 = unsafe { &*(intrinsics::const_allocate(4, 4) as *mut i32) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<deref>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to previous error

---

---- [ui] ui/consts/const-eval/ref_to_int_match.rs stdout ----
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc2, but expected initialized plain (non-pointer) bytes
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc2────────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
10            }
11 
12 error: could not evaluate constant pattern



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ref_to_int_match.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:25:1
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f }; //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc2, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
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


---- [ui] ui/consts/const-eval/ub-enum.rs stdout ----
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x00000001 at .<enum-tag>, but expected a valid enum tag
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                01 00 00 00 00 00 00 00                         │ ........
+    = note: the raw bytes of the constant (size: 4, align: 4) {
10            }
11 
12 error[E0080]: it is undefined behavior to use this value


16    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc8 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
17    |
18    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc8────────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
21            }
22 
23 error[E0080]: it is undefined behavior to use this value


27    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc12 at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
28    |
29    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc12───────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
32            }
33 
34 error[E0080]: it is undefined behavior to use this value


38    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x00000000 at .<enum-tag>, but expected a valid enum tag
39    |
40    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                00 00 00 00 00 00 00 00                         │ ........
+    = note: the raw bytes of the constant (size: 4, align: 4) {
43            }
44 
45 error[E0080]: it is undefined behavior to use this value


49    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc18 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
50    |
51    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc18───────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
54            }
55 
56 error[E0080]: it is undefined behavior to use this value


60    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc22 at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
61    |
62    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc22───────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
65            }
66 
67 error[E0080]: it is undefined behavior to use this value


71    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<enum-tag>, but expected initialized plain (non-pointer) bytes
72    |
73    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                __ __ __ __ __ __ __ __                         │ ░░░░░░░░
+    = note: the raw bytes of the constant (size: 4, align: 4) {
+                __ __ __ __                                     │ ░░░░
77 
78 error[E0080]: it is undefined behavior to use this value


82    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc28 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
83    |
84    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc28───────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
87            }
88 
89 error[E0080]: it is undefined behavior to use this value



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:24:1
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x00000001 at .<enum-tag>, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc8 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc12 at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:42:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:42:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x00000000 at .<enum-tag>, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:44:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:44:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc18 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc22 at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:56:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:56:1
   |
LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:60:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc28 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:77:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:77:1
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<enum-variant(B)>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:79:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:79:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Never at .<enum-variant(D)>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:87:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:87:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0xffffffff at .<enum-variant(Some)>.0.1, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:92:1
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Never at .<enum-variant(Ok)>.0.1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:94:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:94:1
   |
LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<enum-variant(Ok)>.0.1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error: aborting due to 13 previous errors

---

---- [ui] ui/consts/const-eval/ub-upvars.rs stdout ----
diff of stderr:

9    | |__^ type validation failed: encountered a NULL reference at .<deref>.<dyn-downcast>.<captured-var(bad_ref)>
10    |
11    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾───────alloc2────────╼ ╾───────alloc3────────╼ │ ╾──────╼╾──────╼
+    = note: the raw bytes of the constant (size: 8, align: 4) {
+                ╾─alloc2──╼ ╾─alloc3──╼                         │ ╾──╼╾──╼
15 
16 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/ub-upvars.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-upvars.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-upvars.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-upvars.rs:5:1
   |
LL | / const BAD_UPVAR: &dyn FnOnce() = &{ //~ ERROR it is undefined behavior to use this value
LL | |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
LL | |     let another_var = 13;
LL | |     move || { let _ = bad_ref; let _ = another_var; }
LL | | };
   | |__^ type validation failed: encountered a NULL reference at .<deref>.<dyn-downcast>.<captured-var(bad_ref)>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─alloc2──╼ ╾─alloc3──╼                         │ ╾──╼╾──╼

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.

------------------------------------------


---- [ui] ui/consts/const-eval/ub-ref.rs stdout ----
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc2────────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
10            }
11 
12 error[E0080]: it is undefined behavior to use this value


16    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
17    |
18    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc6────────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
21            }
22 
23 error[E0080]: it is undefined behavior to use this value


27    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a NULL reference
28    |
29    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                00 00 00 00 00 00 00 00                         │ ........
+    = note: the raw bytes of the constant (size: 4, align: 4) {
32            }
33 
34 error[E0080]: it is undefined behavior to use this value


38    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a NULL box
39    |
40    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                00 00 00 00 00 00 00 00                         │ ........
+    = note: the raw bytes of the constant (size: 4, align: 4) {
43            }
44 
45 error[E0080]: it is undefined behavior to use this value


49    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc14, but expected initialized plain (non-pointer) bytes
50    |
51    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc14───────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
54            }
55 
56 error[E0080]: it is undefined behavior to use this value


60    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
61    |
62    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc20───────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
65            }
66 
67 error[E0080]: it is undefined behavior to use this value


71    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
72    |
73    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc25───────╼                         │ ╾──────╼
+    = note: the raw bytes of the constant (size: 4, align: 4) {
76            }
77 
78 error[E0080]: it is undefined behavior to use this value


82    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (created from integer)
83    |
84    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                39 05 00 00 00 00 00 00                         │ 9.......
+    = note: the raw bytes of the constant (size: 4, align: 4) {
87            }
88 
89 error[E0080]: it is undefined behavior to use this value


93    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (created from integer)
94    |
95    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                39 05 00 00 00 00 00 00                         │ 9.......
+    = note: the raw bytes of the constant (size: 4, align: 4) {
98            }
99 
100 error: aborting due to 9 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/ub-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-ref.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:6:1
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:10:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:10:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:14:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:14:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a NULL reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:17:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:17:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a NULL box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:23:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:23:1
   |
LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc14, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:26:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:26:1
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:29:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:29:1
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:32:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:32:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (created from integer)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:35:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:35:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (created from integer)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to 9 previous errors

---

---- [ui] ui/consts/const-eval/ub-nonnull.rs stdout ----
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                00 00 00 00 00 00 00 00                         │ ........
+    = note: the raw bytes of the constant (size: 4, align: 4) {
10            }
11 
11 
12 error: any use of this value will cause an error

45    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
46    |
47    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                00 00 00 00 00 00 00 00                         │ ........
+    = note: the raw bytes of the constant (size: 4, align: 4) {
50            }
51 
52 error[E0080]: it is undefined behavior to use this value



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:11:1
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }


error: any use of this value will cause an error
   |
   |
LL | / const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
LL | |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
LL | |     // Use address-of-element for pointer arithmetic. This could wrap around to NULL!
LL | |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR any use of this value will cause an error
   | |                              ^^^^^^^^ memory access failed: pointer must be in-bounds at offset 256, but is outside bounds of alloc10 which has size 1
LL | |     mem::transmute(out_of_bounds_ptr)
LL | | } };
   | |____-
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:14:8
   |
   |
LL | #[deny(const_err)] // this triggers a `const_err` so validation does not even happen

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:22:1
   |
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:24:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:24:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:32:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:32:1
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .0, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               __                                              │ ░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:40:1
   |
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:46:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to 7 previous errors

---

---- [ui] ui/consts/const-eval/union-ice.rs stdout ----
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
+    = note: the raw bytes of the constant (size: 8, align: 4) {
9                __ __ __ __ __ __ __ __                         │ ░░░░░░░░
11 


19    | |__^ type validation failed: encountered uninitialized bytes at .b, but expected initialized plain (non-pointer) bytes
20    |
21    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-                __ __ __ __ __ __ __ __ 2a __ __ __ __ __ __ __ │ ░░░░░░░░*░░░░░░░
+    = note: the raw bytes of the constant (size: 12, align: 4) {
+                __ __ __ __ __ __ __ __ 2a __ __ __             │ ░░░░░░░░*░░░
25 
26 error[E0080]: it is undefined behavior to use this value


36    | |__^ type validation failed: encountered uninitialized bytes at .b[1]
37    |
38    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 40, align: 8) {
+    = note: the raw bytes of the constant (size: 36, align: 4) {
40                0x00 │ 15 00 00 00 00 00 00 00 __ __ __ __ __ __ __ __ │ ........░░░░░░░░
41                0x10 │ 17 00 00 00 00 00 00 00 18 00 00 00 00 00 00 00 │ ................
-                0x20 │ 2a __ __ __ __ __ __ __                         │ *░░░░░░░
+                0x20 │ 2a __ __ __                                     │ *░░░
44 
45 error: aborting due to 3 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice/union-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/union-ice.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-ice.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:14:1
   |
LL | const FIELD3: Field3 = unsafe { UNION.field3 }; //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:16:1
   |
   |
LL | / const FIELD_PATH: Struct = Struct { //~ ERROR it is undefined behavior to use this value
LL | |     a: 42,
LL | |     b: unsafe { UNION.field3 },
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at .b, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 12, align: 4) {
               __ __ __ __ __ __ __ __ 2a __ __ __             │ ░░░░░░░░*░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:26:1
   |
   |
LL | / const FIELD_PATH2: Struct2 = Struct2 { //~ ERROR it is undefined behavior to use this value
LL | |     b: [
LL | |         21,
LL | |         unsafe { UNION.field3 },
LL | |     a: 42,
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at .b[1]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 36, align: 4) {
               0x00 │ 15 00 00 00 00 00 00 00 __ __ __ __ __ __ __ __ │ ........░░░░░░░░
               0x10 │ 17 00 00 00 00 00 00 00 18 00 00 00 00 00 00 00 │ ................
               0x20 │ 2a __ __ __                                     │ *░░░

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.

------------------------------------------


---- [ui] ui/consts/const-eval/ub-uninhabit.rs stdout ----
diff of stderr:

14    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar at .<deref>
15    |
16    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                01 00 00 00 00 00 00 00                         │ ........
+    = note: the raw bytes of the constant (size: 4, align: 4) {
19            }
20 
21 error[E0080]: it is undefined behavior to use this value



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit/ub-uninhabit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-uninhabit.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/ui --pass=check --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:16
