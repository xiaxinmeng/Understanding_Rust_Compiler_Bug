plain
........................................................................................ 968/13112
........................................................................................ 1056/13112
........................................................................................ 1144/13112
........................................................................................ 1232/13112
.......................F..i........F.F.................................................. 1320/13112
........................................................................................ 1496/13112
........................................................................................ 1584/13112
........................................................................................ 1672/13112
......................i.....ii.......................................................... 1760/13112
......................i.....ii.......................................................... 1760/13112
........................................................................................ 1848/13112
............................................................................i........... 1936/13112
........................................................................................ 2024/13112
........................................................................................ 2112/13112
........................................................................................ 2200/13112
........................................................................................ 2288/13112
........................................................................................ 2376/13112
...........................................................................F.........F.. 2464/13112
.............F...............................................F.................F...F.... 2552/13112
..F.................F................................................................... 2640/13112
........................................................................................ 2816/13112
........................................................................................ 2904/13112
................F....................i.................................................. 2992/13112
...i.................................................................................... 3080/13112
---

27 error: aborting due to 2 previous errors
28 
29 For more information about this error, try `rustc --explain E0080`.
+ Future incompatibility report: Future breakage diagnostic:
+ error: any use of this value will cause an error
+    |
+    |
+ LL | const _CONST: &[u8] = &f(&[], |_| {});
+    |                       |
+    |                       referenced constant has errors
+    |
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
30 

---
To only update this specific test, also pass `--test-args borrowck/issue-81899.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-81899.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-81899" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-81899/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-81899.rs:12:5
   |
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                        -------------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-81899.rs:4:24
...
LL |     panic!() //~ ERROR: evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-81899.rs:12:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-81899.rs:12:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-81899.rs:4:31: 4:37]>` at /checkout/library/std/src/panic.rs:19:9
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: any use of this value will cause an error
  --> /checkout/src/test/ui/borrowck/issue-81899.rs:4:23
  --> /checkout/src/test/ui/borrowck/issue-81899.rs:4:23
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                       |
   |                       referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                       |
   |                       referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs stdout ----
diff of stderr:

27 error: aborting due to 2 previous errors
28 
29 For more information about this error, try `rustc --explain E0080`.
+ Future incompatibility report: Future breakage diagnostic:
+ error: any use of this value will cause an error
+    |
+    |
+ LL | const _CONST: &[u8] = &f(&[], |_| {});
+    |                       |
+    |                       referenced constant has errors
+    |
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
30 

---
To only update this specific test, also pass `--test-args borrowck/issue-88434-removal-index-should-be-less.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:11:5
   |
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                        -------------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:24
...
LL |     panic!() //~ ERROR evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:11:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:11:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:31: 3:37]>` at /checkout/library/std/src/panic.rs:19:9
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: any use of this value will cause an error
  --> /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:23
  --> /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:23
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                       |
   |                       referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                       |
   |                       referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/borrowck/issue-88434-minimal-example.rs stdout ----
diff of stderr:

27 error: aborting due to 2 previous errors
28 
29 For more information about this error, try `rustc --explain E0080`.
+ Future incompatibility report: Future breakage diagnostic:
+ error: any use of this value will cause an error
+    |
+    |
+ LL | const _CONST: &() = &f(&|_| {});
+    |                     |
+    |                     referenced constant has errors
+    |
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
30 

---
To only update this specific test, also pass `--test-args borrowck/issue-88434-minimal-example.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:11:5
   |
   |
LL | const _CONST: &() = &f(&|_| {});
   |                      ---------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:22
...
LL |     panic!() //~ ERROR evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:11:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:11:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:25: 3:31]>` at /checkout/library/std/src/panic.rs:19:9
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: any use of this value will cause an error
  --> /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:21
  --> /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:21
   |
LL | const _CONST: &() = &f(&|_| {});
   |                     |
   |                     referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL | const _CONST: &() = &f(&|_| {});
   |                     |
   |                     referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs stdout ----
diff of 64bit.stderr:

323 For more information about this error, try `rustc --explain E0080`.
324 Future incompatibility report: Future breakage diagnostic:
325 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:29:43
327    |
327    |
+ LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
+    |                                                 |
+    |                                                 |
+    |                                                 unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/const-pointer-values-in-various-types.rs:30:43
+    |
328 LL |     const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
330    |                                           |

336 
337 Future breakage diagnostic:
337 Future breakage diagnostic:
338 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:33:45
+   --> $DIR/const-pointer-values-in-various-types.rs:34:45
340    |
341 LL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };

349 
350 Future breakage diagnostic:
351 error: any use of this value will cause an error
351 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:37:45
+   --> $DIR/const-pointer-values-in-various-types.rs:38:45
353    |
354 LL |     const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };

362 
363 Future breakage diagnostic:
364 error: any use of this value will cause an error
364 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:47:43
+   --> $DIR/const-pointer-values-in-various-types.rs:42:45
366    |
+ LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
+    |                                             |
+    |                                             |
+    |                                             unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/const-pointer-values-in-various-types.rs:49:43
+    |
367 LL |     const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
369    |                                           |

375 
376 Future breakage diagnostic:
376 Future breakage diagnostic:
377 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:51:45
+   --> $DIR/const-pointer-values-in-various-types.rs:53:45
379    |
380 LL |     const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };

388 
389 Future breakage diagnostic:
390 error: any use of this value will cause an error
390 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:55:45
+   --> $DIR/const-pointer-values-in-various-types.rs:57:45
392    |
393 LL |     const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };

401 
402 Future breakage diagnostic:
403 error: any use of this value will cause an error
403 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:65:45
+   --> $DIR/const-pointer-values-in-various-types.rs:61:45
405    |
+ LL |     const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
+    |                                             |
+    |                                             |
+    |                                             unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/const-pointer-values-in-various-types.rs:68:45
+    |
406 LL |     const I32_REF_F32_UNION: f32 = unsafe { Nonsense { int_32_ref: &3 }.float_32 };
408    |                                             |

414 
415 Future breakage diagnostic:
415 Future breakage diagnostic:
416 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:72:47
+   --> $DIR/const-pointer-values-in-various-types.rs:72:45
418    |
+ LL |     const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
+    |                                             |
+    |                                             |
+    |                                             unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/const-pointer-values-in-various-types.rs:76:47
+    |
419 LL |     const I32_REF_BOOL_UNION: bool = unsafe { Nonsense { int_32_ref: &3 }.truthy_falsey };
421    |                                               |

427 
428 Future breakage diagnostic:
428 Future breakage diagnostic:
429 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:76:47
+   --> $DIR/const-pointer-values-in-various-types.rs:80:47
431    |
432 LL |     const I32_REF_CHAR_UNION: char = unsafe { Nonsense { int_32_ref: &3 }.character };

440 
441 Future breakage diagnostic:
442 error: any use of this value will cause an error
442 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:80:39
+   --> $DIR/const-pointer-values-in-various-types.rs:84:39
444    |
445 LL |     const STR_U8_UNION: u8 = unsafe { Nonsense { stringy: "3" }.uint_8 };

453 
454 Future breakage diagnostic:
455 error: any use of this value will cause an error
455 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:84:41
+   --> $DIR/const-pointer-values-in-various-types.rs:88:41
457    |
458 LL |     const STR_U16_UNION: u16 = unsafe { Nonsense { stringy: "3" }.uint_16 };

466 
467 Future breakage diagnostic:
468 error: any use of this value will cause an error
468 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:88:41
+   --> $DIR/const-pointer-values-in-various-types.rs:92:41
470    |
471 LL |     const STR_U32_UNION: u32 = unsafe { Nonsense { stringy: "3" }.uint_32 };

479 
480 Future breakage diagnostic:
481 error: any use of this value will cause an error
481 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:95:43
+   --> $DIR/const-pointer-values-in-various-types.rs:96:41
483    |
+ LL |     const STR_U64_UNION: u64 = unsafe { Nonsense { stringy: "3" }.uint_64 };
+    |                                         |
+    |                                         |
+    |                                         unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/const-pointer-values-in-various-types.rs:100:43
+    |
484 LL |     const STR_U128_UNION: u128 = unsafe { Nonsense { stringy: "3" }.uint_128 };
486    |                                           |

492 
493 Future breakage diagnostic:
493 Future breakage diagnostic:
494 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:99:39
+   --> $DIR/const-pointer-values-in-various-types.rs:104:39
496    |
497 LL |     const STR_I8_UNION: i8 = unsafe { Nonsense { stringy: "3" }.int_8 };

505 
506 Future breakage diagnostic:
507 error: any use of this value will cause an error
507 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:103:41
+   --> $DIR/const-pointer-values-in-various-types.rs:108:41
509    |
510 LL |     const STR_I16_UNION: i16 = unsafe { Nonsense { stringy: "3" }.int_16 };

518 
519 Future breakage diagnostic:
520 error: any use of this value will cause an error
520 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:107:41
+   --> $DIR/const-pointer-values-in-various-types.rs:112:41
522    |
523 LL |     const STR_I32_UNION: i32 = unsafe { Nonsense { stringy: "3" }.int_32 };

531 
532 Future breakage diagnostic:
533 error: any use of this value will cause an error
533 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:114:43
+   --> $DIR/const-pointer-values-in-various-types.rs:116:41
535    |
+ LL |     const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
+    |                                         |
+    |                                         |
+    |                                         unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/const-pointer-values-in-various-types.rs:120:43
+    |
536 LL |     const STR_I128_UNION: i128 = unsafe { Nonsense { stringy: "3" }.int_128 };
538    |                                           |

544 
545 Future breakage diagnostic:
545 Future breakage diagnostic:
546 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:118:41
+   --> $DIR/const-pointer-values-in-various-types.rs:124:41
548    |
549 LL |     const STR_F32_UNION: f32 = unsafe { Nonsense { stringy: "3" }.float_32 };

557 
558 Future breakage diagnostic:
559 error: any use of this value will cause an error
559 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:125:43
+   --> $DIR/const-pointer-values-in-various-types.rs:128:41
561    |
+ LL |     const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: "3" }.float_64 };
+    |                                         |
+    |                                         |
+    |                                         unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/const-pointer-values-in-various-types.rs:132:43
+    |
562 LL |     const STR_BOOL_UNION: bool = unsafe { Nonsense { stringy: "3" }.truthy_falsey };
564    |                                           |

570 
571 Future breakage diagnostic:
571 Future breakage diagnostic:
572 error: any use of this value will cause an error
-   --> $DIR/const-pointer-values-in-various-types.rs:129:43
+   --> $DIR/const-pointer-values-in-various-types.rs:136:43
574    |
575 LL |     const STR_CHAR_UNION: char = unsafe { Nonsense { stringy: "3" }.character };


The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/const-pointer-values-in-various-types.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const-pointer-values-in-various-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/auxiliary"
stdout: none
--- stderr -------------------------------
error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
   |                                                 |
   |                                                 |
   |                                                 unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:30:43
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:30:43
   |
LL |     const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:34:45
   |
LL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:38:45
   |
LL |     const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:42:45
   |
LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:46:5
   |
LL |     const I32_REF_U128_UNION: u128 = unsafe { Nonsense { int_32_ref: &3 }.uint_128 };
   |
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:49:43
   |
   |
LL |     const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:53:45
   |
LL |     const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:57:45
   |
LL |     const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:61:45
   |
LL |     const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:65:5
   |
LL |     const I32_REF_I128_UNION: i128 = unsafe { Nonsense { int_32_ref: &3 }.int_128 };
   |
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:68:45
   |
   |
LL |     const I32_REF_F32_UNION: f32 = unsafe { Nonsense { int_32_ref: &3 }.float_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:72:45
   |
LL |     const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:76:47
   |
LL |     const I32_REF_BOOL_UNION: bool = unsafe { Nonsense { int_32_ref: &3 }.truthy_falsey };
   |                                               |
   |                                               |
   |                                               unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
---
   |
LL |     let x: &'static i32 = &X;
   |                           ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


thread 'rustc' panicked at 'aborting after 2 errors due to `-Z treat-err-as-bug=2`', compiler/rustc_errors/src/lib.rs:1451:36
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   1: core::panicking::panic_fmt
   2: <rustc_errors::HandlerInner>::panic_if_treat_err_as_bug
   4: <rustc_errors::Handler>::emit_diagnostic
   4: <rustc_errors::Handler>::emit_diagnostic
   5: <() as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
   6: <<rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2} as core::ops::function::FnOnce<(rustc_middle::lint::LintDiagnosticBuilder<()>,)>>::call_once::{shim:vtable#0}
   7: <alloc::boxed::Box<dyn for<'a> core::ops::function::FnOnce<(rustc_middle::lint::LintDiagnosticBuilder<'a, ()>,), Output = ()>> as core::ops::function::FnOnce<(rustc_middle::lint::LintDiagnosticBuilder<()>,)>>::call_once
   8: rustc_middle::lint::struct_lint_level::struct_lint_level_impl
   9: rustc_middle::lint::struct_lint_level::<<rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2}>
  10: <rustc_middle::ty::context::TyCtxt>::struct_span_lint_hir::<rustc_span::span_encoding::Span, <rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2}>
  11: <rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint
  12: <rustc_mir_transform::const_prop_lint::ConstPropagator>::eval_constant
  13: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_constant
  14: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_statement
  15: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_body
  16: <rustc_mir_transform::const_prop_lint::ConstProp as rustc_mir_transform::pass_manager::MirLint>::run_lint
  17: rustc_mir_transform::pass_manager::run_passes
  18: rustc_mir_transform::run_post_borrowck_cleanup_passes
  19: rustc_mir_transform::mir_drops_elaborated_and_const_checked
  20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
  21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  25: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
  26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  27: <rustc_middle::ty::context::TyCtxt>::instance_mir
  27: <rustc_middle::ty::context::TyCtxt>::instance_mir
  28: rustc_monomorphize::collector::collect_neighbours
  29: rustc_monomorphize::collector::collect_items_rec
  30: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
  31: rustc_monomorphize::collector::collect_crate_mono_items
  32: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  33: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  34: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  35: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  36: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
  37: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  38: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  39: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorGuaranteed>>
  40: <rustc_interface::queries::Queries>::ongoing_codegen
  42: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  43: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (0d10cfd3d 2022-06-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=2
query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
Future incompatibility report: Future breakage diagnostic:
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:19:16
   |
   |
LL | const X: i32 = 1 / 0; //~WARN any use of this value will cause an error
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:18:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
Future breakage diagnostic:
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:23:27
   |
LL |     let x: &'static i32 = &X;
   |                           ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/consts/const-eval/format.rs stdout ----
diff of stderr:

76 error: aborting due to 8 previous errors
77 
78 For more information about this error, try `rustc --explain E0015`.
+ Future incompatibility report: Future breakage diagnostic:
+ error: erroneous constant used
+    |
+ LL |     panic!("{:?}", 0);
+    |            ^^^^^^ referenced constant has errors
+    |
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: erroneous constant used
+ error: erroneous constant used
+   --> $DIR/format.rs:2:20
+    |
+ LL |     panic!("{:?}", 0);
+    |                    ^ referenced constant has errors
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ Future breakage diagnostic:
+ Future breakage diagnostic:
+ error: erroneous constant used
+   --> $DIR/format.rs:11:14
+    |
+ LL |     println!("{:?}", 0);
+    |              ^^^^^^ referenced constant has errors
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: erroneous constant used
+ error: erroneous constant used
+   --> $DIR/format.rs:11:22
+    |
+ LL |     println!("{:?}", 0);
+    |                      ^ referenced constant has errors
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
79 
---
To only update this specific test, also pass `--test-args consts/const-eval/format.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/format.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/format" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/format/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const formatting macro in constant functions
   |
LL |     panic!("{:?}", 0);
   |                    ^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error[E0015]: cannot call non-const formatting macro in constant functions
  --> /checkout/src/test/ui/consts/const-eval/format.rs:11:22
   |
   |
LL |     println!("{:?}", 0);
   |                      ^
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `Arguments::<'a>::new_v1` is not yet stable as a const fn
   |
LL |     println!("{:?}", 0);
   |     ^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: add `#![feature(const_fmt_arguments_new)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0015]: cannot call non-const fn `_print` in constant functions
   |
LL |     println!("{:?}", 0);
   |     ^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/format.rs:2:12
   |
LL |     panic!("{:?}", 0);
LL |     panic!("{:?}", 0);
   |            ^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/format.rs:2:20
---

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0015`.
Future incompatibility report: Future breakage diagnostic:
error: erroneous constant used
   |
LL |     panic!("{:?}", 0);
   |            ^^^^^^ referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: erroneous constant used
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/format.rs:2:20
   |
LL |     panic!("{:?}", 0);
   |                    ^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)

Future breakage diagnostic:
Future breakage diagnostic:
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/format.rs:11:14
   |
LL |     println!("{:?}", 0);
   |              ^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: erroneous constant used
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/format.rs:11:22
   |
LL |     println!("{:?}", 0);
   |                      ^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
------------------------------------------



---- [ui] src/test/ui/consts/const-eval/ref_to_int_match.rs stdout ----
diff of 64bit.stderr:

24 
25 error: aborting due to 3 previous errors
26 
+ Future incompatibility report: Future breakage diagnostic:
+ error: any use of this value will cause an error
+    |
+    |
+ LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
+    |                           |
+    |                           |
+    |                           unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
27 



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ref_to_int_match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/auxiliary"
stdout: none
--- stderr -------------------------------
error: any use of this value will cause an error
   |
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
   |                           |
   |                           |
   |                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

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


Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
   |                           |
   |                           |
   |                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/consts/const-eval/ub-ref-ptr.rs stdout ----
diff of 64bit.stderr:

176 error: aborting due to 16 previous errors
177 
178 For more information about this error, try `rustc --explain E0080`.
+ Future incompatibility report: Future breakage diagnostic:
+ error: any use of this value will cause an error
+   --> $DIR/ub-ref-ptr.rs:31:1
+    |
+ LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/ub-ref-ptr.rs:35:39
+    |
+ LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
+    |                                       |
+    |                                       |
+    |                                       unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/ub-ref-ptr.rs:35:38
+    |
+ LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
+    |                                      |
+    |                                      referenced constant has errors
+    |
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/ub-ref-ptr.rs:41:86
+    |
+ LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
+    |                                                                                      |
+    |                                                                                      |
+    |                                                                                      unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/ub-ref-ptr.rs:41:85
+    |
+ LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
+    |                                                                                     |
+    |                                                                                     referenced constant has errors
+    |
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
179 



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/ub-ref-ptr.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-ref-ptr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:31:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:31:1
   |
LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:35:39
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:35:39
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   |                                       |
   |                                       |
   |                                       unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:35:38
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   |                                      |
   |                                      referenced constant has errors
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:41:86
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   |                                                                                      |
   |                                                                                      |
   |                                                                                      unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:41:85
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   |                                                                                     |
   |                                                                                     referenced constant has errors
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:47:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (address 0x539 is unallocated)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:50:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:50:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (address 0x539 is unallocated)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:53:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:53:1
   |
LL | const UNINIT_PTR: *const i32 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:56:1
   |
   |
LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
   |
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:58:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:58:1
   |
LL | const UNINIT_FN_PTR: fn() = unsafe { MaybeUninit { uninit: () }.init };
   |
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:60:1
   |
   |
LL | const DANGLING_FN_PTR: fn() = unsafe { mem::transmute(13usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x000000000000000d, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               0d 00 00 00 00 00 00 00                         │ ........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:62:1
   |
   |
LL | const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc43, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: aborting due to 16 previous errors


For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:35:39
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   |                                       |
   |                                       |
   |                                       unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:35:38
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   |                                      |
   |                                      referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:41:86
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   |                                                                                      |
   |                                                                                      |
   |                                                                                      unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:41:85
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   |                                                                                     |
   |                                                                                     referenced constant has errors
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/consts/const-eval/ub-enum.rs stdout ----
diff of 64bit.stderr:

125 error: aborting due to 13 previous errors
126 
127 For more information about this error, try `rustc --explain E0080`.
+ Future incompatibility report: Future breakage diagnostic:
+ error: any use of this value will cause an error
+   --> $DIR/ub-enum.rs:26:1
+    |
+ LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/ub-enum.rs:30:1
+    |
+ LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/ub-enum.rs:45:1
+    |
+ LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ error: any use of this value will cause an error
+ error: any use of this value will cause an error
+   --> $DIR/ub-enum.rs:49:1
+    |
+ LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
---
To only update this specific test, also pass `--test-args consts/const-float-bits-reject-conv.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-float-bits-reject-conv.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-reject-conv" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-reject-conv/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/library/core/src/num/f32.rs:925:21
   |
   |
LL |                     panic!("const-eval error: cannot use f32::to_bits on a NaN")
   |                     |
   |                     |
   |                     the evaluated program panicked at 'const-eval error: cannot use f32::to_bits on a NaN', /checkout/library/core/src/num/f32.rs:925:21
   |                     inside `core::f32::<impl f32>::to_bits::ct_f32_to_u32` at /checkout/library/core/src/panic.rs:57:9
...
LL |         unsafe { intrinsics::const_eval_select((self,), ct_f32_to_u32, rt_f32_to_u32) }
   |                  -------------------------------------------------------------------- inside `core::f32::<impl f32>::to_bits` at /checkout/library/core/src/num/f32.rs:941:18
  ::: /checkout/library/core/src/ops/function.rs:248:5
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |     ------------------------------------------------------------------ inside `<fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32} as FnOnce<(f32,)>>::call_once - shim(fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32})` at /checkout/library/core/src/ops/function.rs:248:5
  ::: /checkout/library/core/src/intrinsics.rs:2395:5
   |
LL |     called_in_const.call_once(arg)
LL |     called_in_const.call_once(arg)
   |     ------------------------------ inside `const_eval_select::<(f32,), fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32}, [closure@core::f32::<impl f32>::to_bits::{closure#0}], u32>` at /checkout/library/core/src/intrinsics.rs:2395:5
  ::: /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:27:30
   |
   |
LL |     const MASKED_NAN1: u32 = f32::NAN.to_bits() ^ 0x002A_AAAA;
   |                              ------------------ inside `f32::MASKED_NAN1` at /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:27:30
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/num/f32.rs:925:21
  --> /checkout/library/core/src/num/f32.rs:925:21
   |
LL |                     panic!("const-eval error: cannot use f32::to_bits on a NaN")
   |                     |
   |                     |
   |                     the evaluated program panicked at 'const-eval error: cannot use f32::to_bits on a NaN', /checkout/library/core/src/num/f32.rs:925:21
   |                     inside `core::f32::<impl f32>::to_bits::ct_f32_to_u32` at /checkout/library/core/src/panic.rs:57:9
...
LL |         unsafe { intrinsics::const_eval_select((self,), ct_f32_to_u32, rt_f32_to_u32) }
   |                  -------------------------------------------------------------------- inside `core::f32::<impl f32>::to_bits` at /checkout/library/core/src/num/f32.rs:941:18
  ::: /checkout/library/core/src/ops/function.rs:248:5
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |     ------------------------------------------------------------------ inside `<fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32} as FnOnce<(f32,)>>::call_once - shim(fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32})` at /checkout/library/core/src/ops/function.rs:248:5
  ::: /checkout/library/core/src/intrinsics.rs:2395:5
   |
LL |     called_in_const.call_once(arg)
LL |     called_in_const.call_once(arg)
   |     ------------------------------ inside `const_eval_select::<(f32,), fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32}, [closure@core::f32::<impl f32>::to_bits::{closure#0}], u32>` at /checkout/library/core/src/intrinsics.rs:2395:5
  ::: /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:28:30
   |
   |
LL |     const MASKED_NAN2: u32 = f32::NAN.to_bits() ^ 0x0055_5555;
   |                              ------------------ inside `f32::MASKED_NAN2` at /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:28:30
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:30:34
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:30:34
   |
LL |             const _: () = assert!($a);
...
...
LL |     const_assert!(f32::from_bits(MASKED_NAN1).is_nan());
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:33:34
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:33:34
   |
LL |             const _: () = assert!($a);
...
...
LL |     const_assert!(f32::from_bits(MASKED_NAN1).is_nan());
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:41:38
   |
LL |             const _: () = assert!($a == $b);
...
...
LL |         const_assert!(f32::from_bits(MASKED_NAN1).to_bits(), MASKED_NAN1);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:44:38
   |
LL |             const _: () = assert!($a == $b);
...
...
LL |         const_assert!(f32::from_bits(MASKED_NAN2).to_bits(), MASKED_NAN2);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/num/f64.rs:918:21
   |
LL |                     panic!("const-eval error: cannot use f64::to_bits on a NaN")
   |                     |
   |                     |
   |                     the evaluated program panicked at 'const-eval error: cannot use f64::to_bits on a NaN', /checkout/library/core/src/num/f64.rs:918:21
   |                     inside `core::f64::<impl f64>::to_bits::ct_f64_to_u64` at /checkout/library/core/src/panic.rs:57:9
...
LL |         unsafe { intrinsics::const_eval_select((self,), ct_f64_to_u64, rt_f64_to_u64) }
   |                  -------------------------------------------------------------------- inside `core::f64::<impl f64>::to_bits` at /checkout/library/core/src/num/f64.rs:934:18
  ::: /checkout/library/core/src/ops/function.rs:248:5
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |     ------------------------------------------------------------------ inside `<fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64} as FnOnce<(f64,)>>::call_once - shim(fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64})` at /checkout/library/core/src/ops/function.rs:248:5
  ::: /checkout/library/core/src/intrinsics.rs:2395:5
   |
LL |     called_in_const.call_once(arg)
LL |     called_in_const.call_once(arg)
   |     ------------------------------ inside `const_eval_select::<(f64,), fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64}, [closure@core::f64::<impl f64>::to_bits::{closure#0}], u64>` at /checkout/library/core/src/intrinsics.rs:2395:5
  ::: /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:54:30
   |
   |
LL |     const MASKED_NAN1: u64 = f64::NAN.to_bits() ^ 0x000A_AAAA_AAAA_AAAA;
   |                              ------------------ inside `f64::MASKED_NAN1` at /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:54:30
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/num/f64.rs:918:21
  --> /checkout/library/core/src/num/f64.rs:918:21
   |
LL |                     panic!("const-eval error: cannot use f64::to_bits on a NaN")
   |                     |
   |                     |
   |                     the evaluated program panicked at 'const-eval error: cannot use f64::to_bits on a NaN', /checkout/library/core/src/num/f64.rs:918:21
   |                     inside `core::f64::<impl f64>::to_bits::ct_f64_to_u64` at /checkout/library/core/src/panic.rs:57:9
...
LL |         unsafe { intrinsics::const_eval_select((self,), ct_f64_to_u64, rt_f64_to_u64) }
   |                  -------------------------------------------------------------------- inside `core::f64::<impl f64>::to_bits` at /checkout/library/core/src/num/f64.rs:934:18
  ::: /checkout/library/core/src/ops/function.rs:248:5
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |     ------------------------------------------------------------------ inside `<fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64} as FnOnce<(f64,)>>::call_once - shim(fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64})` at /checkout/library/core/src/ops/function.rs:248:5
  ::: /checkout/library/core/src/intrinsics.rs:2395:5
   |
LL |     called_in_const.call_once(arg)
LL |     called_in_const.call_once(arg)
   |     ------------------------------ inside `const_eval_select::<(f64,), fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64}, [closure@core::f64::<impl f64>::to_bits::{closure#0}], u64>` at /checkout/library/core/src/intrinsics.rs:2395:5
  ::: /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:55:30
   |
   |
LL |     const MASKED_NAN2: u64 = f64::NAN.to_bits() ^ 0x0005_5555_5555_5555;
   |                              ------------------ inside `f64::MASKED_NAN2` at /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:55:30
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:57:34
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:57:34
   |
LL |             const _: () = assert!($a);
...
...
LL |     const_assert!(f64::from_bits(MASKED_NAN1).is_nan());
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:60:34
   |
LL |             const _: () = assert!($a);
...
...
LL |     const_assert!(f64::from_bits(MASKED_NAN1).is_nan());
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:66:38
   |
LL |             const _: () = assert!($a == $b);
...
...
LL |         const_assert!(f64::from_bits(MASKED_NAN1).to_bits(), MASKED_NAN1);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:69:38
   |
LL |             const _: () = assert!($a == $b);
...
...
LL |         const_assert!(f64::from_bits(MASKED_NAN2).to_bits(), MASKED_NAN2);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL |             const _: () = assert!($a);
...
...
LL |     const_assert!(f32::from_bits(MASKED_NAN1).is_nan());
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:33:34
   |
LL |             const _: () = assert!($a);
...
...
LL |     const_assert!(f32::from_bits(MASKED_NAN1).is_nan());
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:41:38
   |
LL |             const _: () = assert!($a == $b);
...
...
LL |         const_assert!(f32::from_bits(MASKED_NAN1).to_bits(), MASKED_NAN1);
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:44:38
   |
LL |             const _: () = assert!($a == $b);
...
...
LL |         const_assert!(f32::from_bits(MASKED_NAN2).to_bits(), MASKED_NAN2);
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:57:34
   |
LL |             const _: () = assert!($a);
...
...
LL |     const_assert!(f64::from_bits(MASKED_NAN1).is_nan());
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:60:34
   |
LL |             const _: () = assert!($a);
...
...
LL |     const_assert!(f64::from_bits(MASKED_NAN1).is_nan());
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:66:38
   |
LL |             const _: () = assert!($a == $b);
...
...
LL |         const_assert!(f64::from_bits(MASKED_NAN1).to_bits(), MASKED_NAN1);
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:69:38
   |
LL |             const _: () = assert!($a == $b);
...
...
LL |         const_assert!(f64::from_bits(MASKED_NAN2).to_bits(), MASKED_NAN2);
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/consts/ptr_comparisons.rs stdout ----
diff of stderr:

62   --> $DIR/ptr_comparisons.rs:70:27
63    |
64 LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
+    | --------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------
66    |                           |
66    |                           |
67    |                           unable to turn pointer into raw bytes


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/ptr_comparisons.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/ptr_comparisons.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/ptr_comparisons.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/ptr_comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/library/core/src/ptr/const_ptr.rs:457:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc3 has size 8, so pointer to 16 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const usize>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:457:18
  ::: /checkout/src/test/ui/consts/ptr_comparisons.rs:58:34
   |
   |
LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };
   |                                  ------------------------------- inside `_` at /checkout/src/test/ui/consts/ptr_comparisons.rs:58:34
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:61:33
   |
   |
LL |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc3 has size 8, so pointer to 1000 bytes starting at offset 0 is out-of-bounds
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:65:27
   |
   |
LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
   |                           |
   |                           |
   |                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:70:27
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:70:27
   |
LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
   |                           |
   |                           |
   |                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 4 previous errors
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
   |                           |
   |                           |
   |                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:70:27
   |
LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
   |                           |
   |                           |
   |                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------


